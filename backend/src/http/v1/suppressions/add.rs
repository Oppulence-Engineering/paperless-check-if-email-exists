use crate::config::BackendConfig;
use crate::finder::require_tenant_id;
use crate::http::v1::bulk::with_worker_db;
use crate::http::{resolve_tenant, ReacherResponseError};
use crate::tenant::context::TenantContext;
use check_if_email_exists::LOG_TARGET;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sqlx::{PgPool, Row};
use std::collections::BTreeSet;
use std::sync::Arc;
use warp::http::StatusCode;
use warp::Filter;

#[derive(Debug, Deserialize)]
struct Request {
	emails: Vec<String>,
	#[serde(default = "default_reason")]
	reason: String,
	source: Option<String>,
	notes: Option<String>,
	reason_detail: Option<String>,
	source_type: Option<String>,
	source_ref: Option<String>,
	created_by: Option<String>,
	owner: Option<String>,
	expires_at: Option<DateTime<Utc>>,
	confidence: Option<String>,
	auto_suppressed_by_policy: Option<bool>,
	metadata: Option<Value>,
}

fn default_reason() -> String {
	"manual".to_string()
}

#[derive(Debug, Serialize)]
struct Response {
	added: i64,
	updated: i64,
	duplicates: i64,
	entry_ids: Vec<i32>,
}

async fn http_handler(
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	body: Request,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;

	if body.emails.is_empty() {
		return Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"emails array is required and must not be empty",
		)
		.into());
	}
	if body.emails.len() > 10_000 {
		return Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"Maximum 10,000 emails per request",
		)
		.into());
	}

	validate_reason(&body.reason)?;
	validate_confidence(body.confidence.as_deref())?;

	let mut added: i64 = 0;
	let mut updated: i64 = 0;
	let mut duplicates: i64 = 0;
	let mut unique_emails = Vec::new();
	let mut seen = BTreeSet::new();
	for email in &body.emails {
		let normalized = normalize_suppression_email(email);
		if normalized.is_empty() {
			continue;
		}
		if !seen.insert(normalized.clone()) {
			duplicates += 1;
			continue;
		}
		unique_emails.push(normalized);
	}

	if unique_emails.is_empty() {
		return Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"emails array must include at least one non-empty email",
		)
		.into());
	}

	let metadata = metadata_for_request(&body).map_err(warp::reject::custom)?;
	let source_type = body
		.source_type
		.clone()
		.or_else(|| body.source.clone())
		.unwrap_or_else(|| "manual".to_string());
	let created_by = body
		.created_by
		.clone()
		.or_else(|| body.owner.clone())
		.unwrap_or_else(|| "api".to_string());
	let actor_type = if body.auto_suppressed_by_policy == Some(true) {
		"policy"
	} else {
		"api"
	};
	let mut entry_ids = Vec::new();
	let mut tx = pg_pool.begin().await.map_err(ReacherResponseError::from)?;

	for email in &unique_emails {
		let row = sqlx::query(
			r#"
			INSERT INTO v1_suppression_entries (
				tenant_id, email, canonical_email, status, reason, source, notes,
				reason_code, reason_detail, source_type, source_ref, created_by,
				expires_at, last_seen_at, metadata
			)
			VALUES (
				$1, $2, $2, 'active', $3::suppression_reason, $4, $5,
				$3, $6, $7, $8, $9, $10, NOW(), $11
			)
			ON CONFLICT (tenant_id, canonical_email) WHERE status = 'active'
			DO UPDATE SET
				reason = EXCLUDED.reason,
				source = COALESCE(EXCLUDED.source, v1_suppression_entries.source),
				notes = COALESCE(EXCLUDED.notes, v1_suppression_entries.notes),
				reason_code = EXCLUDED.reason_code,
				reason_detail = COALESCE(EXCLUDED.reason_detail, v1_suppression_entries.reason_detail),
				source_type = EXCLUDED.source_type,
				source_ref = COALESCE(EXCLUDED.source_ref, v1_suppression_entries.source_ref),
				created_by = COALESCE(EXCLUDED.created_by, v1_suppression_entries.created_by),
				expires_at = COALESCE(EXCLUDED.expires_at, v1_suppression_entries.expires_at),
				last_seen_at = NOW(),
				metadata = v1_suppression_entries.metadata || EXCLUDED.metadata,
				updated_at = NOW()
			RETURNING id, canonical_email, (xmax = 0) AS inserted
			"#,
		)
		.bind(tenant_id)
		.bind(email)
		.bind(&body.reason)
		.bind(&body.source)
		.bind(&body.notes)
		.bind(&body.reason_detail)
		.bind(&source_type)
		.bind(&body.source_ref)
		.bind(&created_by)
		.bind(body.expires_at)
		.bind(&metadata)
		.fetch_one(&mut *tx)
		.await
		.map_err(ReacherResponseError::from)?;

		let entry_id: i32 = row.get("id");
		let inserted: bool = row.get("inserted");
		let canonical_email: String = row.get("canonical_email");
		if inserted {
			added += 1;
		} else {
			updated += 1;
		}
		entry_ids.push(entry_id);

		sqlx::query(
			r#"
			INSERT INTO v1_suppression_events (
				tenant_id, entry_id, canonical_email, event_type, from_status, to_status,
				reason_code, reason_detail, source_type, source_ref, actor_type, actor_id, metadata
			)
			VALUES ($1, $2, $3, $4, $5, 'active', $6, $7, $8, $9, $10, $11, $12)
			"#,
		)
		.bind(tenant_id)
		.bind(entry_id)
		.bind(&canonical_email)
		.bind(if inserted { "created" } else { "updated" })
		.bind(if inserted { None } else { Some("active") })
		.bind(&body.reason)
		.bind(&body.reason_detail)
		.bind(&source_type)
		.bind(&body.source_ref)
		.bind(actor_type)
		.bind(&created_by)
		.bind(&metadata)
		.execute(&mut *tx)
		.await
		.map_err(ReacherResponseError::from)?;
	}
	tx.commit().await.map_err(ReacherResponseError::from)?;

	Ok(warp::reply::with_status(
		warp::reply::json(&Response {
			added,
			updated,
			duplicates,
			entry_ids,
		}),
		StatusCode::OK,
	))
}

fn validate_reason(reason: &str) -> Result<(), warp::Rejection> {
	let valid_reasons = [
		"manual",
		"bounce",
		"invalid",
		"spam_trap",
		"unsubscribe",
		"complaint",
		"auto_invalid",
	];
	if valid_reasons.contains(&reason) {
		return Ok(());
	}
	Err(ReacherResponseError::new(
		StatusCode::BAD_REQUEST,
		format!(
			"Invalid reason '{}'. Must be one of: {}",
			reason,
			valid_reasons.join(", ")
		),
	)
	.into())
}

fn validate_confidence(confidence: Option<&str>) -> Result<(), warp::Rejection> {
	let Some(confidence) = confidence else {
		return Ok(());
	};
	if matches!(confidence, "low" | "medium" | "high") {
		return Ok(());
	}
	Err(ReacherResponseError::new(
		StatusCode::BAD_REQUEST,
		"Invalid confidence. Must be one of: low, medium, high",
	)
	.into())
}

fn normalize_suppression_email(email: &str) -> String {
	email.trim().to_lowercase()
}

fn metadata_for_request(body: &Request) -> Result<Value, ReacherResponseError> {
	let mut metadata = match body
		.metadata
		.clone()
		.unwrap_or_else(|| Value::Object(Map::new()))
	{
		Value::Object(object) => object,
		_ => {
			return Err(ReacherResponseError::new(
				StatusCode::BAD_REQUEST,
				"metadata must be a JSON object",
			))
		}
	};
	if let Some(confidence) = &body.confidence {
		metadata.insert("confidence".to_string(), Value::String(confidence.clone()));
	}
	if let Some(owner) = &body.owner {
		metadata.insert("owner".to_string(), Value::String(owner.clone()));
	}
	if let Some(auto) = body.auto_suppressed_by_policy {
		metadata.insert("auto_suppressed_by_policy".to_string(), Value::Bool(auto));
	}
	Ok(Value::Object(metadata))
}

/// POST /v1/suppressions
#[utoipa::path(
	post,
	path = "/v1/suppressions",
	tag = "v1",
	responses((status = 200, description = "Suppression entries added"))
)]
pub fn v1_add_suppressions(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "suppressions")
		.and(warp::post())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::body::json())
		.and_then(http_handler)
		.with(warp::log(LOG_TARGET))
}

#[cfg(test)]
mod tests {
	use super::*;
	use serde_json::json;

	fn request() -> Request {
		Request {
			emails: vec!["User@Example.COM".to_string()],
			reason: "manual".to_string(),
			source: None,
			notes: None,
			reason_detail: None,
			source_type: None,
			source_ref: None,
			created_by: None,
			owner: Some("revops".to_string()),
			expires_at: None,
			confidence: Some("high".to_string()),
			auto_suppressed_by_policy: Some(true),
			metadata: Some(json!({"batch_id": "csv-1"})),
		}
	}

	#[test]
	fn normalizes_suppression_email() {
		assert_eq!(
			normalize_suppression_email(" User@Example.COM "),
			"user@example.com"
		);
	}

	#[test]
	fn metadata_includes_intelligence_fields() {
		let metadata = metadata_for_request(&request()).expect("metadata should build");

		assert_eq!(metadata["batch_id"], "csv-1");
		assert_eq!(metadata["confidence"], "high");
		assert_eq!(metadata["owner"], "revops");
		assert_eq!(metadata["auto_suppressed_by_policy"], true);
	}

	#[test]
	fn metadata_must_be_object() {
		let mut request = request();
		request.metadata = Some(json!(["invalid"]));

		assert!(metadata_for_request(&request).is_err());
	}

	#[test]
	fn validates_confidence_values() {
		assert!(validate_confidence(Some("high")).is_ok());
		assert!(validate_confidence(Some("certain")).is_err());
	}
}
