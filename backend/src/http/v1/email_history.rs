use crate::config::BackendConfig;
use crate::http::v1::bulk::with_worker_db;
use crate::http::{check_scope, resolve_tenant, ReacherResponseError};
use crate::tenant::context::{scope, TenantContext};
use check_if_email_exists::LOG_TARGET;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use std::sync::Arc;
use warp::Filter;

#[derive(Debug, Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
struct Query {
	limit: Option<i64>,
}

#[derive(Debug, Serialize)]
struct HistoryEntry {
	job_id: Option<i32>,
	score: Option<i16>,
	category: Option<String>,
	sub_reason: Option<String>,
	safe_to_send: Option<bool>,
	reason_codes: Option<Vec<String>>,
	is_reachable: Option<String>,
	completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
struct Response {
	email: String,
	history: Vec<HistoryEntry>,
	total: i64,
}

async fn http_handler(
	email: String,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	query: Query,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::VERIFY)?;

	let limit = query.limit.unwrap_or(50).clamp(0, 200);
	let email_lower = email.to_lowercase();

	let total: i64 = sqlx::query_scalar(
		r#"
		SELECT COUNT(*) FROM v1_task_result
		WHERE (tenant_id = $1 OR $1 IS NULL)
		  AND LOWER(COALESCE(result->>'input', payload->'input'->>'to_email')) = $2
		  AND task_state IN ('completed', 'failed', 'dead_lettered')
		"#,
	)
	.bind(tenant_ctx.tenant_id)
	.bind(&email_lower)
	.fetch_one(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let rows = sqlx::query(
		r#"
		SELECT
			job_id,
			score,
			score_category,
			sub_reason,
			safe_to_send,
			reason_codes,
			result->>'is_reachable' AS is_reachable,
			completed_at
		FROM v1_task_result
		WHERE (tenant_id = $1 OR $1 IS NULL)
		  AND LOWER(COALESCE(result->>'input', payload->'input'->>'to_email')) = $2
		  AND task_state IN ('completed', 'failed', 'dead_lettered')
		ORDER BY completed_at DESC NULLS LAST
		LIMIT $3
		"#,
	)
	.bind(tenant_ctx.tenant_id)
	.bind(&email_lower)
	.bind(limit)
	.fetch_all(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let history: Vec<HistoryEntry> = rows
		.iter()
		.map(|r| HistoryEntry {
			job_id: r.get("job_id"),
			score: r.get("score"),
			category: r.get("score_category"),
			sub_reason: r.get("sub_reason"),
			safe_to_send: r.get("safe_to_send"),
			reason_codes: r.get("reason_codes"),
			is_reachable: r.get("is_reachable"),
			completed_at: r.get("completed_at"),
		})
		.collect();

	Ok(warp::reply::json(&Response {
		email: email_lower,
		history,
		total,
	}))
}

/// GET /v1/emails/{email}/history
///
/// Returns the verification history timeline for a specific email address.
#[utoipa::path(
	get,
	path = "/v1/emails/{email}/history",
	tag = "Verification",
	params(
		("email" = String, Path, description = "Email address to look up"),
		Query
	),
	responses((status = 200, description = "Verification history for the email"))
)]
pub fn v1_email_history(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "emails" / String / "history")
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::query::<Query>())
		.and_then(http_handler)
		.with(warp::log(LOG_TARGET))
}
