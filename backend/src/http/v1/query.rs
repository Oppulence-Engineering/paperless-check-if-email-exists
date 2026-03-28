use crate::config::BackendConfig;
use crate::http::v1::bulk::with_worker_db;
use crate::http::{check_scope, resolve_tenant, ReacherResponseError};
use crate::tenant::context::{scope, TenantContext};
use check_if_email_exists::LOG_TARGET;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use std::sync::Arc;
use warp::http::StatusCode;
use warp::Filter;

#[derive(Debug, Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
struct QueryParams {
	limit: Option<i64>,
	offset: Option<i64>,
	category: Option<String>,
	min_score: Option<i16>,
	max_score: Option<i16>,
	safe_to_send: Option<bool>,
	job_id: Option<i32>,
	since: Option<String>,
	until: Option<String>,
}

#[derive(Debug, Serialize)]
struct ResultRow {
	id: i32,
	job_id: Option<i32>,
	email: Option<String>,
	score: Option<i16>,
	category: Option<String>,
	sub_reason: Option<String>,
	safe_to_send: Option<bool>,
	reason_codes: Option<Vec<String>>,
	task_state: String,
	completed_at: Option<DateTime<Utc>>,
	created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
struct Response {
	results: Vec<ResultRow>,
	total: i64,
}

async fn http_handler(
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	query: QueryParams,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::BULK)?;

	let limit = query.limit.unwrap_or(50).clamp(0, 500);
	let offset = query.offset.unwrap_or(0).max(0);

	let since = match &query.since {
		Some(s) => Some(
			DateTime::parse_from_rfc3339(s)
				.map(|dt| dt.with_timezone(&Utc))
				.map_err(|_| {
					ReacherResponseError::new(
						StatusCode::BAD_REQUEST,
						"Invalid 'since' date format. Expected RFC3339.",
					)
				})?,
		),
		None => None,
	};
	let until = match &query.until {
		Some(s) => Some(
			DateTime::parse_from_rfc3339(s)
				.map(|dt| dt.with_timezone(&Utc))
				.map_err(|_| {
					ReacherResponseError::new(
						StatusCode::BAD_REQUEST,
						"Invalid 'until' date format. Expected RFC3339.",
					)
				})?,
		),
		None => None,
	};

	let total: i64 = sqlx::query_scalar(
		r#"
		SELECT COUNT(*) FROM v1_task_result
		WHERE (tenant_id = $1 OR $1 IS NULL)
		  AND ($2::TEXT IS NULL OR score_category = $2)
		  AND ($3::SMALLINT IS NULL OR score >= $3)
		  AND ($4::SMALLINT IS NULL OR score <= $4)
		  AND ($5::BOOLEAN IS NULL OR safe_to_send = $5)
		  AND ($6::INTEGER IS NULL OR job_id = $6)
		  AND ($7::TIMESTAMPTZ IS NULL OR completed_at >= $7)
		  AND ($8::TIMESTAMPTZ IS NULL OR completed_at <= $8)
		"#,
	)
	.bind(tenant_ctx.tenant_id)
	.bind(&query.category)
	.bind(query.min_score)
	.bind(query.max_score)
	.bind(query.safe_to_send)
	.bind(query.job_id)
	.bind(since)
	.bind(until)
	.fetch_one(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let rows = sqlx::query(
		r#"
		SELECT
			id,
			job_id,
			COALESCE(result->>'input', payload->'input'->>'to_email') AS email,
			score,
			score_category,
			sub_reason,
			safe_to_send,
			reason_codes,
			task_state::TEXT AS task_state,
			completed_at,
			created_at
		FROM v1_task_result
		WHERE (tenant_id = $1 OR $1 IS NULL)
		  AND ($2::TEXT IS NULL OR score_category = $2)
		  AND ($3::SMALLINT IS NULL OR score >= $3)
		  AND ($4::SMALLINT IS NULL OR score <= $4)
		  AND ($5::BOOLEAN IS NULL OR safe_to_send = $5)
		  AND ($6::INTEGER IS NULL OR job_id = $6)
		  AND ($7::TIMESTAMPTZ IS NULL OR completed_at >= $7)
		  AND ($8::TIMESTAMPTZ IS NULL OR completed_at <= $8)
		ORDER BY completed_at DESC NULLS LAST
		LIMIT $9 OFFSET $10
		"#,
	)
	.bind(tenant_ctx.tenant_id)
	.bind(&query.category)
	.bind(query.min_score)
	.bind(query.max_score)
	.bind(query.safe_to_send)
	.bind(query.job_id)
	.bind(since)
	.bind(until)
	.bind(limit)
	.bind(offset)
	.fetch_all(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let results: Vec<ResultRow> = rows
		.iter()
		.map(|r| ResultRow {
			id: r.get("id"),
			job_id: r.get("job_id"),
			email: r.get("email"),
			score: r.get("score"),
			category: r.get("score_category"),
			sub_reason: r.get("sub_reason"),
			safe_to_send: r.get("safe_to_send"),
			reason_codes: r.get("reason_codes"),
			task_state: r.get("task_state"),
			completed_at: r.get("completed_at"),
			created_at: r.get("created_at"),
		})
		.collect();

	Ok(warp::reply::json(&Response { results, total }))
}

/// GET /v1/query
///
/// Flexible historical query API for verification results across jobs.
#[utoipa::path(
	get,
	path = "/v1/query",
	tag = "Query",
	params(QueryParams),
	responses((status = 200, description = "Filtered verification results"))
)]
pub fn v1_query_results(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "query")
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::query::<QueryParams>())
		.and_then(http_handler)
		.with(warp::log(LOG_TARGET))
}
