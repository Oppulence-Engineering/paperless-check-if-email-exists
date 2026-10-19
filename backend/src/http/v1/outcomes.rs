use crate::config::BackendConfig;
use crate::finder::require_tenant_id;
use crate::http::v1::bulk::with_worker_db;
use crate::http::{check_scope, resolve_tenant, ReacherResponseError};
use crate::tenant::context::{scope, TenantContext};
use check_if_email_exists::LOG_TARGET;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use sqlx::{PgPool, Row};
use std::sync::Arc;
use warp::http::StatusCode;
use warp::Filter;

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct OutcomeIngestRequest {
	pub(crate) provider: String,
	pub(crate) outcomes: Vec<OutcomeInput>,
	pub(crate) source_key: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, utoipa::ToSchema)]
pub struct OutcomeInput {
	pub(crate) email: String,
	pub(crate) event_type: String,
	pub(crate) source_key: Option<String>,
	pub(crate) campaign_id: Option<String>,
	pub(crate) occurred_at: Option<DateTime<Utc>>,
	pub(crate) metadata: Option<Value>,
	#[serde(skip)]
	#[schema(ignore)]
	pub(crate) endpoint_id: Option<uuid::Uuid>,
	#[serde(skip)]
	#[schema(ignore)]
	pub(crate) receipt_id: Option<uuid::Uuid>,
	pub(crate) provider_event_id: Option<String>,
	pub(crate) provider_message_id: Option<String>,
	#[serde(skip)]
	#[schema(ignore)]
	pub(crate) event_family: Option<String>,
	#[serde(skip)]
	#[schema(ignore)]
	pub(crate) correlation_status: Option<String>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct OutcomeIngestResponse {
	pub ingested: i64,
	pub auto_suppressed: i64,
	pub ignored: i64,
}

#[derive(Debug, Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
struct OutcomeListQuery {
	limit: Option<i64>,
	offset: Option<i64>,
	email: Option<String>,
	event_type: Option<String>,
	source_key: Option<String>,
	since: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct OutcomeView {
	pub id: i64,
	pub email: String,
	pub canonical_email: String,
	pub provider: String,
	pub event_type: String,
	pub source_key: Option<String>,
	pub campaign_id: Option<String>,
	pub occurred_at: DateTime<Utc>,
	pub metadata: Value,
	pub endpoint_id: Option<uuid::Uuid>,
	pub receipt_id: Option<uuid::Uuid>,
	pub provider_event_id: Option<String>,
	pub provider_message_id: Option<String>,
	pub event_family: Option<String>,
	pub correlation_status: String,
	pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct OutcomeListResponse {
	pub outcomes: Vec<OutcomeView>,
	pub limit: i64,
	pub offset: i64,
}

async fn http_handler(
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	body: OutcomeIngestRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::SUPPRESSIONS)?;
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	let response = ingest_outcomes(&pg_pool, tenant_id, body).await?;
	Ok(warp::reply::json(&response))
}

pub(crate) async fn ingest_outcomes(
	pg_pool: &PgPool,
	tenant_id: uuid::Uuid,
	body: OutcomeIngestRequest,
) -> Result<OutcomeIngestResponse, warp::Rejection> {
	if body.provider.trim().is_empty() {
		return Err(
			ReacherResponseError::new(StatusCode::BAD_REQUEST, "provider is required").into(),
		);
	}
	if body.outcomes.is_empty() {
		return Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"outcomes array is required and must not be empty",
		)
		.into());
	}
	if body.outcomes.len() > 10_000 {
		return Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"Maximum 10,000 outcomes per request",
		)
		.into());
	}

	let provider = body.provider.trim().to_ascii_lowercase();
	let default_source_key = normalize_source_key(body.source_key.as_deref());
	let mut ingested = 0;
	let mut auto_suppressed = 0;
	let mut ignored = 0;
	let mut tx = pg_pool.begin().await.map_err(ReacherResponseError::from)?;

	for outcome in &body.outcomes {
		let canonical_email = normalize_email(&outcome.email);
		if canonical_email.is_empty() {
			ignored += 1;
			continue;
		}
		validate_event_type(&outcome.event_type)?;
		let metadata = metadata_for_outcome(outcome).map_err(warp::reject::custom)?;
		let source_key = normalize_source_key(outcome.source_key.as_deref())
			.or_else(|| default_source_key.clone());
		let occurred_at = outcome.occurred_at.unwrap_or_else(Utc::now);
		let event_type = outcome.event_type.trim().to_ascii_lowercase();

		let outcome_id: Option<i64> = sqlx::query_scalar(
			r#"
			INSERT INTO v1_contact_outcomes (
				tenant_id, email, canonical_email, provider, event_type, source_key,
				campaign_id, occurred_at, metadata, endpoint_id, receipt_id,
				provider_event_id, provider_message_id, event_family, correlation_status
			)
			VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
			ON CONFLICT (tenant_id, provider, provider_event_id)
				WHERE provider_event_id IS NOT NULL
			DO NOTHING
			RETURNING id
			"#,
		)
		.bind(tenant_id)
		.bind(outcome.email.trim())
		.bind(&canonical_email)
		.bind(&provider)
		.bind(&event_type)
		.bind(&source_key)
		.bind(&outcome.campaign_id)
		.bind(occurred_at)
		.bind(&metadata)
		.bind(outcome.endpoint_id)
		.bind(outcome.receipt_id)
		.bind(&outcome.provider_event_id)
		.bind(&outcome.provider_message_id)
		.bind(&outcome.event_family)
		.bind(outcome.correlation_status.as_deref().unwrap_or("unmatched"))
		.fetch_optional(&mut *tx)
		.await
		.map_err(ReacherResponseError::from)?;
		let Some(outcome_id) = outcome_id else {
			ignored += 1;
			continue;
		};
		ingested += 1;

		if let Some(reason) = suppression_reason_for_event(&event_type) {
			upsert_outcome_suppression(
				&mut tx,
				tenant_id,
				&canonical_email,
				reason,
				&provider,
				outcome.campaign_id.as_deref(),
				outcome_id,
				&metadata,
			)
			.await
			.map_err(warp::reject::custom)?;
			auto_suppressed += 1;
		}
	}

	tx.commit().await.map_err(ReacherResponseError::from)?;
	Ok(OutcomeIngestResponse {
		ingested,
		auto_suppressed,
		ignored,
	})
}

async fn list_handler(
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	query: OutcomeListQuery,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::SUPPRESSIONS)?;
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	let limit = query.limit.unwrap_or(100).clamp(1, 500);
	let offset = query.offset.unwrap_or(0).max(0);
	let email = query.email.map(|value| normalize_email(&value));
	let event_type = query
		.event_type
		.map(|value| value.trim().to_ascii_lowercase());
	let source_key = normalize_source_key(query.source_key.as_deref());
	let rows = sqlx::query(
		r#"
		SELECT id, email, canonical_email, provider, event_type, source_key, campaign_id,
		       occurred_at, metadata, endpoint_id, receipt_id, provider_event_id,
		       provider_message_id, event_family, correlation_status, created_at
		FROM v1_contact_outcomes
		WHERE tenant_id = $1
		  AND ($2::TEXT IS NULL OR canonical_email = $2)
		  AND ($3::TEXT IS NULL OR event_type = $3)
		  AND ($4::TEXT IS NULL OR source_key = $4)
		  AND ($5::TIMESTAMPTZ IS NULL OR occurred_at >= $5)
		ORDER BY occurred_at DESC, id DESC
		LIMIT $6 OFFSET $7
		"#,
	)
	.bind(tenant_id)
	.bind(email)
	.bind(event_type)
	.bind(source_key)
	.bind(query.since)
	.bind(limit)
	.bind(offset)
	.fetch_all(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;
	let outcomes = rows
		.into_iter()
		.map(|row| OutcomeView {
			id: row.get("id"),
			email: row.get("email"),
			canonical_email: row.get("canonical_email"),
			provider: row.get("provider"),
			event_type: row.get("event_type"),
			source_key: row.get("source_key"),
			campaign_id: row.get("campaign_id"),
			occurred_at: row.get("occurred_at"),
			metadata: row.get("metadata"),
			endpoint_id: row.get("endpoint_id"),
			receipt_id: row.get("receipt_id"),
			provider_event_id: row.get("provider_event_id"),
			provider_message_id: row.get("provider_message_id"),
			event_family: row.get("event_family"),
			correlation_status: row.get("correlation_status"),
			created_at: row.get("created_at"),
		})
		.collect::<Vec<_>>();
	Ok(warp::reply::json(&OutcomeListResponse {
		outcomes,
		limit,
		offset,
	}))
}

async fn upsert_outcome_suppression(
	tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
	tenant_id: uuid::Uuid,
	canonical_email: &str,
	reason: &str,
	provider: &str,
	campaign_id: Option<&str>,
	outcome_id: i64,
	metadata: &Value,
) -> Result<(), ReacherResponseError> {
	let source_ref = campaign_id
		.map(ToOwned::to_owned)
		.unwrap_or_else(|| provider.to_string());
	let suppression_metadata = outcome_suppression_metadata(provider, outcome_id, metadata);
	let row = sqlx::query(
		r#"
		INSERT INTO v1_suppression_entries (
			tenant_id, email, canonical_email, status, reason, source, notes,
			reason_code, reason_detail, source_type, source_ref, created_by,
			last_seen_at, metadata
		)
		VALUES (
			$1, $2, $2, 'active', $3::suppression_reason, $4, NULL,
			$3, $5, 'outcome', $6, 'outcome_ingest', NOW(), $7
		)
		ON CONFLICT (tenant_id, canonical_email) WHERE status = 'active'
		DO UPDATE SET
			reason = EXCLUDED.reason,
			source = EXCLUDED.source,
			reason_code = EXCLUDED.reason_code,
			reason_detail = EXCLUDED.reason_detail,
			source_type = EXCLUDED.source_type,
			source_ref = EXCLUDED.source_ref,
			last_seen_at = NOW(),
			metadata = v1_suppression_entries.metadata || EXCLUDED.metadata,
			updated_at = NOW()
		RETURNING id, (xmax = 0) AS inserted
		"#,
	)
	.bind(tenant_id)
	.bind(canonical_email)
	.bind(reason)
	.bind(provider)
	.bind(format!("Auto-suppressed from {} outcome.", provider))
	.bind(&source_ref)
	.bind(&suppression_metadata)
	.fetch_one(&mut **tx)
	.await
	.map_err(ReacherResponseError::from)?;

	let entry_id: i32 = row.get("id");
	let inserted: bool = row.get("inserted");
	sqlx::query(
		r#"
		INSERT INTO v1_suppression_events (
			tenant_id, entry_id, canonical_email, event_type, from_status, to_status,
			reason_code, reason_detail, source_type, source_ref, actor_type, actor_id, metadata
		)
		VALUES ($1, $2, $3, $4, $5, 'active', $6, $7, 'outcome', $8, 'outcome', $9, $10)
		"#,
	)
	.bind(tenant_id)
	.bind(entry_id)
	.bind(canonical_email)
	.bind(if inserted { "created" } else { "updated" })
	.bind(if inserted { None } else { Some("active") })
	.bind(reason)
	.bind(format!("Auto-suppressed from {} outcome.", provider))
	.bind(&source_ref)
	.bind(provider)
	.bind(&suppression_metadata)
	.execute(&mut **tx)
	.await
	.map_err(ReacherResponseError::from)?;

	Ok(())
}

fn validate_event_type(event_type: &str) -> Result<(), warp::Rejection> {
	let event_type = event_type.trim().to_ascii_lowercase();
	if matches!(
		event_type.as_str(),
		"bounced" | "delivered" | "opened" | "clicked" | "complained" | "unsubscribed"
	) {
		return Ok(());
	}
	Err(ReacherResponseError::new(
		StatusCode::BAD_REQUEST,
		format!("Invalid outcome event_type: {}", event_type),
	)
	.into())
}

fn suppression_reason_for_event(event_type: &str) -> Option<&'static str> {
	match event_type {
		"bounced" => Some("bounce"),
		"complained" => Some("complaint"),
		"unsubscribed" => Some("unsubscribe"),
		_ => None,
	}
}

fn normalize_email(email: &str) -> String {
	email.trim().to_ascii_lowercase()
}

fn normalize_source_key(source_key: Option<&str>) -> Option<String> {
	let source_key = source_key?.trim().to_ascii_lowercase();
	if source_key.is_empty() {
		None
	} else {
		Some(source_key)
	}
}

fn metadata_for_outcome(outcome: &OutcomeInput) -> Result<Value, ReacherResponseError> {
	let metadata = match outcome
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
	Ok(Value::Object(metadata))
}

fn outcome_suppression_metadata(provider: &str, outcome_id: i64, metadata: &Value) -> Value {
	let mut object = metadata.as_object().cloned().unwrap_or_default();
	object.insert(
		"outcome_provider".to_string(),
		Value::String(provider.to_string()),
	);
	object.insert("outcome_id".to_string(), json!(outcome_id));
	object.insert("auto_suppressed_by_policy".to_string(), Value::Bool(true));
	Value::Object(object)
}

/// POST /v1/outcomes
#[utoipa::path(
	post,
	path = "/v1/outcomes",
	tag = "Outcomes",
	request_body = OutcomeIngestRequest,
	responses(
		(status = 200, description = "Provider outcomes ingested", body = OutcomeIngestResponse),
		(status = 400, description = "Invalid outcome payload")
	)
)]
pub fn v1_ingest_outcomes(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "outcomes")
		.and(warp::post())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::body::json())
		.and_then(http_handler)
		.with(warp::log(LOG_TARGET))
}

/// GET /v1/outcomes
#[utoipa::path(
	get,
	path = "/v1/outcomes",
	tag = "Outcomes",
	params(OutcomeListQuery),
	responses((status = 200, description = "Paginated provider outcomes", body = OutcomeListResponse))
)]
pub fn v1_list_outcomes(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "outcomes")
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::query::<OutcomeListQuery>())
		.and_then(list_handler)
		.with(warp::log(LOG_TARGET))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn validates_known_outcome_events() {
		assert!(validate_event_type("bounced").is_ok());
		assert!(validate_event_type("complained").is_ok());
		assert!(validate_event_type("unknown").is_err());
	}

	#[test]
	fn maps_risky_outcomes_to_suppression_reasons() {
		assert_eq!(suppression_reason_for_event("bounced"), Some("bounce"));
		assert_eq!(
			suppression_reason_for_event("complained"),
			Some("complaint")
		);
		assert_eq!(
			suppression_reason_for_event("unsubscribed"),
			Some("unsubscribe")
		);
		assert_eq!(suppression_reason_for_event("opened"), None);
	}

	#[test]
	fn outcome_suppression_metadata_marks_policy_source() {
		let metadata = outcome_suppression_metadata("hubspot", 12, &json!({"campaign": "q3"}));

		assert_eq!(metadata["outcome_provider"], "hubspot");
		assert_eq!(metadata["outcome_id"], 12);
		assert_eq!(metadata["auto_suppressed_by_policy"], true);
		assert_eq!(metadata["campaign"], "q3");
	}
}
