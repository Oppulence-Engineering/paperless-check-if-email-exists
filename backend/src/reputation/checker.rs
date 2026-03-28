use super::dns_records::check_dns_records;
use super::dnsbl::lookup_dnsbl;
use super::models::{DomainInfo, ReputationCheckResponse};
use super::scorer::compute_score;
use crate::http::ReacherResponseError;
use sqlx::{PgPool, Row};
use std::net::IpAddr;
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

pub async fn fetch_domain_info(domain: &str) -> Result<DomainInfo, ReacherResponseError> {
	let client = reqwest::Client::new();
	fetch_domain_info_with_client(&client, domain).await
}

pub async fn fetch_domain_info_with_client(
	client: &reqwest::Client,
	domain: &str,
) -> Result<DomainInfo, ReacherResponseError> {
	validate_public_domain_target(domain).await?;
	let url = format!("https://rdap.org/domain/{}", domain);
	let response = client
		.get(url)
		.send()
		.await
		.map_err(ReacherResponseError::from)?;
	let value: serde_json::Value = response.json().await.map_err(ReacherResponseError::from)?;
	Ok(crate::bounce_risk::parse_domain_info_from_rdap_value(
		&value,
	))
}

pub async fn validate_public_domain_target(domain: &str) -> Result<(), ReacherResponseError> {
	let mut addresses = tokio::net::lookup_host((domain, 80))
		.await
		.map_err(|error| {
			ReacherResponseError::new(
				StatusCode::BAD_REQUEST,
				format!("failed to resolve domain {domain}: {error}"),
			)
		})?;

	let mut saw_address = false;
	for address in addresses.by_ref() {
		saw_address = true;
		if is_disallowed_probe_ip(address.ip()) {
			return Err(ReacherResponseError::new(
				StatusCode::BAD_REQUEST,
				format!("domain {domain} resolved to a private or local address"),
			));
		}
	}

	if !saw_address {
		return Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			format!("domain {domain} did not resolve to any addresses"),
		));
	}

	Ok(())
}

pub(crate) fn is_disallowed_probe_ip(ip: IpAddr) -> bool {
	match ip {
		IpAddr::V4(ip) => {
			ip.is_private()
				|| ip.is_loopback()
				|| ip.is_link_local()
				|| ip.is_broadcast()
				|| ip.is_documentation()
				|| ip.is_multicast()
				|| ip.is_unspecified()
		}
		IpAddr::V6(ip) => {
			ip.is_loopback()
				|| ip.is_unique_local()
				|| ip.is_unicast_link_local()
				|| ip.is_multicast()
				|| ip.is_unspecified()
		}
	}
}
