use crate::config::BackendConfig;
use crate::finder::require_tenant_id;
use crate::http::v1::bulk::with_worker_db;
use crate::http::v1::lists::canonicalize::canonicalize_email;
use crate::http::v1::lists::csv_parse::parse_csv;
use crate::http::{check_scope, resolve_tenant, ReacherResponseError};
use crate::outcomes::{
	fetch_or_create_default_policy, ingest_outcomes, IngestOutcome, IngestOutcomesRequest,
	IngestOutcomesResponse, IngestRowError, OutcomeType,
};
use crate::tenant::context::{scope, TenantContext};
use check_if_email_exists::LOG_TARGET;
use chrono::{DateTime, Utc};
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use std::sync::Arc;
use warp::http::StatusCode;
use warp::multipart::FormData;
use warp::Filter;
use bytes::Buf;

#[derive(Debug, Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct OutcomeListQuery {
	email: Option<String>,
	source: Option<String>,
	#[serde(rename = "type")]
	outcome_type: Option<String>,
	since: Option<DateTime<Utc>>,
	limit: Option<i64>,
	offset: Option<i64>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct OutcomeView {
	pub id: i64,
	pub email: String,
	#[serde(rename = "type")]
	pub outcome_type: String,
	pub occurred_at: DateTime<Utc>,
	pub source: Option<String>,
	pub campaign_id: Option<String>,
	pub policy_action: Option<String>,
	pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct OutcomeListResponse {
	pub outcomes: Vec<OutcomeView>,
	pub total: i64,
}

// ----------------------------------------------------------------------------
// POST /v1/outcomes
// ----------------------------------------------------------------------------

async fn ingest_handler(
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	body: IngestOutcomesRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::OUTCOMES_WRITE)?;
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;

	if body.outcomes.is_empty() {
		return Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"outcomes array is required and must be non-empty",
		)
		.into());
	}
	if body.outcomes.len() > 5_000 {
		return Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"max 5000 outcomes per request",
		)
		.into());
	}

	let (policy_id, policy) = fetch_or_create_default_policy(&pg_pool, tenant_id).await;
	let summary = ingest_outcomes(&pg_pool, tenant_id, &policy, &body.outcomes).await;

	let response = IngestOutcomesResponse {
		accepted: summary.accepted,
		rejected: summary.rejected,
		suppressed: summary.suppressed,
		policy_id,
		errors: summary.errors,
	};
	Ok(warp::reply::with_status(
		warp::reply::json(&response),
		StatusCode::ACCEPTED,
	))
}

#[utoipa::path(
	post,
	path = "/v1/outcomes",
	tag = "Outcomes",
	request_body = IngestOutcomesRequest,
	responses((status = 202, description = "Outcomes accepted", body = IngestOutcomesResponse))
)]
pub fn v1_post_outcomes(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "outcomes")
		.and(warp::post())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::body::content_length_limit(2_000_000))
		.and(warp::body::json())
		.and_then(ingest_handler)
		.with(warp::log(LOG_TARGET))
}

// ----------------------------------------------------------------------------
// POST /v1/outcomes/upload  (multipart CSV)
// ----------------------------------------------------------------------------

async fn upload_handler(
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	form: FormData,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::OUTCOMES_WRITE)?;
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;

	let upload = read_csv_upload(form).await?;
	let parsed = parse_csv(&upload.file_bytes, Some("email"))?;

	let mut outcomes = Vec::with_capacity(parsed.rows.len());
	let mut errors: Vec<IngestRowError> = Vec::new();

	for (index, row) in parsed.rows.iter().enumerate() {
		match parse_csv_row(index, row) {
			Ok(outcome) => outcomes.push(outcome),
			Err(err) => errors.push(err),
		}
	}

	let (policy_id, policy) = fetch_or_create_default_policy(&pg_pool, tenant_id).await;
	let mut summary = ingest_outcomes(&pg_pool, tenant_id, &policy, &outcomes).await;
	summary.rejected += errors.len();
	summary.errors.extend(errors);

	let response = IngestOutcomesResponse {
		accepted: summary.accepted,
		rejected: summary.rejected,
		suppressed: summary.suppressed,
		policy_id,
		errors: summary.errors,
	};
	Ok(warp::reply::with_status(
		warp::reply::json(&response),
		StatusCode::ACCEPTED,
	))
}

struct CsvUpload {
	file_bytes: Vec<u8>,
}

async fn read_csv_upload(mut form: FormData) -> Result<CsvUpload, ReacherResponseError> {
	let mut file_bytes: Option<Vec<u8>> = None;
	while let Some(part) = form
		.try_next()
		.await
		.map_err(|err| ReacherResponseError::new(StatusCode::BAD_REQUEST, err))?
	{
		let part_name = part.name().to_string();
		let collected = part
			.stream()
			.try_fold(Vec::new(), |mut bytes, mut chunk| async move {
				bytes.extend_from_slice(chunk.copy_to_bytes(chunk.remaining()).as_ref());
				Ok(bytes)
			})
			.await
			.map_err(|err| ReacherResponseError::new(StatusCode::BAD_REQUEST, err))?;
		if part_name == "file" {
			file_bytes = Some(collected);
		}
	}
	let file_bytes = file_bytes
		.ok_or_else(|| ReacherResponseError::new(StatusCode::BAD_REQUEST, "missing 'file' part"))?;
	Ok(CsvUpload { file_bytes })
}

fn parse_csv_row(
	index: usize,
	row: &serde_json::Map<String, serde_json::Value>,
) -> Result<IngestOutcome, IngestRowError> {
	let email = row
		.get("email")
		.and_then(|v| v.as_str())
		.unwrap_or("")
		.to_string();
	let outcome_type_str = row
		.get("outcome_type")
		.or_else(|| row.get("type"))
		.and_then(|v| v.as_str())
		.unwrap_or("")
		.to_string();
	let occurred_at_str = row
		.get("occurred_at")
		.and_then(|v| v.as_str())
		.unwrap_or("")
		.to_string();

	if email.is_empty() {
		return Err(IngestRowError {
			index,
			email,
			message: "missing email".to_string(),
		});
	}
	let outcome_type: OutcomeType = serde_json::from_value(serde_json::Value::String(
		outcome_type_str.clone(),
	))
	.map_err(|_| IngestRowError {
		index,
		email: email.clone(),
		message: format!("invalid outcome_type '{}'", outcome_type_str),
	})?;
	let occurred_at = DateTime::parse_from_rfc3339(&occurred_at_str)
		.map(|dt| dt.with_timezone(&Utc))
		.map_err(|_| IngestRowError {
			index,
			email: email.clone(),
			message: format!("invalid occurred_at '{}' (expected RFC3339)", occurred_at_str),
		})?;
	let source = row
		.get("source")
		.and_then(|v| v.as_str())
		.filter(|s| !s.is_empty())
		.map(ToOwned::to_owned);
	let campaign_id = row
		.get("campaign_id")
		.and_then(|v| v.as_str())
		.filter(|s| !s.is_empty())
		.map(ToOwned::to_owned);

	Ok(IngestOutcome {
		email,
		outcome_type,
		occurred_at,
		source,
		campaign_id,
		metadata: None,
	})
}

#[utoipa::path(
	post,
	path = "/v1/outcomes/upload",
	tag = "Outcomes",
	responses((status = 202, description = "Outcomes ingested via CSV", body = IngestOutcomesResponse))
)]
pub fn v1_upload_outcomes(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "outcomes" / "upload")
		.and(warp::post())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::multipart::form().max_length(50_000_000))
		.and_then(upload_handler)
		.with(warp::log(LOG_TARGET))
}

// ----------------------------------------------------------------------------
// GET /v1/outcomes
// ----------------------------------------------------------------------------

async fn list_handler(
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	query: OutcomeListQuery,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::OUTCOMES_READ)?;
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	let limit = query.limit.unwrap_or(100).clamp(0, 500);
	let offset = query.offset.unwrap_or(0).max(0);
	let canonical_email = query.email.as_deref().and_then(canonicalize_email);
	let outcome_type: Option<OutcomeType> = match query.outcome_type.as_deref() {
		None => None,
		Some(t) => Some(
			serde_json::from_value(serde_json::Value::String(t.to_string())).map_err(|_| {
				ReacherResponseError::new(
					StatusCode::BAD_REQUEST,
					format!("invalid outcome type '{}'", t),
				)
			})?,
		),
	};

	let total: i64 = sqlx::query_scalar(
		r#"
		SELECT COUNT(*) FROM verification_outcomes
		WHERE tenant_id = $1
			AND ($2::TEXT IS NULL OR canonical_email = $2)
			AND ($3::outcome_type IS NULL OR outcome_type = $3)
			AND ($4::TEXT IS NULL OR source = $4)
			AND ($5::TIMESTAMPTZ IS NULL OR occurred_at >= $5)
		"#,
	)
	.bind(tenant_id)
	.bind(canonical_email.as_deref())
	.bind(outcome_type)
	.bind(query.source.as_deref())
	.bind(query.since)
	.fetch_one(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let rows = sqlx::query(
		r#"
		SELECT id, canonical_email, outcome_type::TEXT as outcome_type, occurred_at,
			NULLIF(source, '') as source, campaign_id, policy_action, created_at
		FROM verification_outcomes
		WHERE tenant_id = $1
			AND ($2::TEXT IS NULL OR canonical_email = $2)
			AND ($3::outcome_type IS NULL OR outcome_type = $3)
			AND ($4::TEXT IS NULL OR source = $4)
			AND ($5::TIMESTAMPTZ IS NULL OR occurred_at >= $5)
		ORDER BY occurred_at DESC, id DESC
		LIMIT $6 OFFSET $7
		"#,
	)
	.bind(tenant_id)
	.bind(canonical_email.as_deref())
	.bind(outcome_type)
	.bind(query.source.as_deref())
	.bind(query.since)
	.bind(limit)
	.bind(offset)
	.fetch_all(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let outcomes: Vec<OutcomeView> = rows
		.iter()
		.map(|row| OutcomeView {
			id: row.get("id"),
			email: row.get("canonical_email"),
			outcome_type: row.get("outcome_type"),
			occurred_at: row.get("occurred_at"),
			source: row.try_get("source").ok().flatten(),
			campaign_id: row.try_get("campaign_id").ok().flatten(),
			policy_action: row.try_get("policy_action").ok().flatten(),
			created_at: row.get("created_at"),
		})
		.collect();

	Ok(warp::reply::json(&OutcomeListResponse { outcomes, total }))
}

#[utoipa::path(
	get,
	path = "/v1/outcomes",
	tag = "Outcomes",
	params(OutcomeListQuery),
	responses((status = 200, description = "List of outcomes", body = OutcomeListResponse))
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
