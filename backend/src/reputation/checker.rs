use super::dns_records::check_dns_records;
use super::dnsbl::lookup_dnsbl;
use super::models::{DomainInfo, ReputationCheckResponse};
use super::scorer::compute_score;
use crate::http::ReacherResponseError;
use sqlx::{PgPool, Row};
use warp::http::StatusCode;

pub async fn check_domain(
	pg_pool: Option<&PgPool>,
	domain: &str,
	force_refresh: bool,
) -> Result<ReputationCheckResponse, ReacherResponseError> {
	let normalized = domain.trim().to_lowercase();
	if normalized.is_empty() || !normalized.contains('.') {
		return Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"Invalid domain",
		));
	}

	if !force_refresh {
		if let Some(pool) = pg_pool {
			if let Some(cached) = sqlx::query(
				"SELECT response FROM reputation_cache WHERE domain = $1 AND expires_at > NOW()",
			)
			.bind(&normalized)
			.fetch_optional(pool)
			.await
			.map_err(ReacherResponseError::from)?
			{
				let value: serde_json::Value = cached.get("response");
				let mut response: ReputationCheckResponse =
					serde_json::from_value(value).map_err(ReacherResponseError::from)?;
				response.cached = true;
				return Ok(response);
			}
		}
	}

	let (blacklist_results, dns_records, domain_info) = tokio::join!(
		async { lookup_dnsbl(&normalized).await.unwrap_or_default() },
		async { check_dns_records(&normalized).await.unwrap_or_default() },
		async { fetch_domain_info(&normalized).await.unwrap_or_default() }
	);

	let (score, risk_level) = compute_score(&blacklist_results, &dns_records, &domain_info);
	let response = ReputationCheckResponse {
		domain: normalized.clone(),
		score,
		risk_level,
		blacklist_results,
		dns_records,
		domain_info,
		cached: false,
	};

	if let Some(pool) = pg_pool {
		sqlx::query(
			r#"
			INSERT INTO reputation_cache (domain, response, score, risk_level, expires_at)
			VALUES ($1, $2, $3, $4, NOW() + INTERVAL '24 hours')
			ON CONFLICT (domain)
			DO UPDATE SET
				response = EXCLUDED.response,
				score = EXCLUDED.score,
				risk_level = EXCLUDED.risk_level,
				expires_at = EXCLUDED.expires_at,
				updated_at = NOW()
			"#,
		)
		.bind(&normalized)
		.bind(serde_json::to_value(&response).map_err(ReacherResponseError::from)?)
		.bind(i32::from(response.score))
		.bind(&response.risk_level)
		.execute(pool)
		.await
		.map_err(ReacherResponseError::from)?;
	}

	Ok(response)
}

async fn fetch_domain_info(domain: &str) -> Result<DomainInfo, ReacherResponseError> {
	let url = format!("https://rdap.org/domain/{}", domain);
	let response = reqwest::Client::new()
		.get(url)
		.send()
		.await
		.map_err(ReacherResponseError::from)?;
	let value: serde_json::Value = response.json().await.map_err(ReacherResponseError::from)?;
	let events = value.get("events").and_then(|value| value.as_array());
	let created = events.and_then(|events| {
		events.iter().find_map(|event| {
			let action = event.get("eventAction").and_then(|value| value.as_str())?;
			if matches!(action, "registration" | "creation") {
				event
					.get("eventDate")
					.and_then(|value| value.as_str())
					.map(ToOwned::to_owned)
			} else {
				None
			}
		})
	});
	let registrar = value
		.get("entities")
		.and_then(|value| value.as_array())
		.and_then(|entities| {
			entities.iter().find_map(|entity| {
				entity
					.get("vcardArray")
					.and_then(|value| value.as_array())
					.and_then(|items| items.get(1))
					.and_then(|value| value.as_array())
					.and_then(|items| {
						items.iter().find_map(|item| {
							let item = item.as_array()?;
							if item.first()?.as_str()? == "fn" {
								item.get(3)?.as_str().map(ToOwned::to_owned)
							} else {
								None
							}
						})
					})
			})
		});
	let domain_age_days = created.as_deref().and_then(|created_at| {
		chrono::DateTime::parse_from_rfc3339(created_at)
			.ok()
			.map(|date| (chrono::Utc::now() - date.with_timezone(&chrono::Utc)).num_days())
	});

	Ok(DomainInfo {
		domain_age_days,
		registrar,
		created_at: created,
	})
}
