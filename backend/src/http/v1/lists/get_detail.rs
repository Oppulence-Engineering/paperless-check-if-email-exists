use crate::config::BackendConfig;
use crate::finder::require_tenant_id;
use crate::http::v1::bulk::with_worker_db;
use crate::http::{resolve_tenant, ReacherResponseError};
use crate::tenant::context::TenantContext;
use check_if_email_exists::LOG_TARGET;
use serde::Serialize;
use sqlx::{PgPool, Row};
use std::sync::Arc;
use warp::http::StatusCode;
use warp::Filter;

#[derive(Debug, Serialize)]
pub struct Summary {
	pub total_valid: i64,
	pub total_risky: i64,
	pub total_unknown: i64,
	pub total_invalid: i64,
	pub total_processed: i64,
}

#[derive(Debug, Serialize)]
struct Response {
	id: i32,
	job_id: i32,
	name: String,
	status: String,
	total_rows: i32,
	email_column: String,
	summary: Summary,
}

async fn http_handler(
	list_id: i32,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	let row = sqlx::query(
		r#"
		SELECT id, job_id, name, status::TEXT AS status, total_rows, email_column
		FROM v1_lists
		WHERE id = $1 AND tenant_id = $2
		"#,
	)
	.bind(list_id)
	.bind(tenant_id)
	.fetch_optional(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;
	let row = row.ok_or_else(|| {
		warp::reject::custom(ReacherResponseError::new(StatusCode::NOT_FOUND, "List not found"))
	})?;

	let summary = list_summary(&pg_pool, list_id).await.map_err(warp::reject::custom)?;

	if summary.total_processed >= i64::from(row.get::<i32, _>("total_rows"))
		&& row.get::<String, _>("status") != "completed"
	{
		sqlx::query(
			"UPDATE v1_lists SET status = 'completed'::list_status, completed_at = COALESCE(completed_at, NOW()), updated_at = NOW() WHERE id = $1",
		)
		.bind(list_id)
		.execute(&pg_pool)
		.await
		.map_err(ReacherResponseError::from)
		.map_err(warp::reject::custom)?;
	}

	Ok(warp::reply::json(&Response {
		id: row.get("id"),
		job_id: row.get("job_id"),
		name: row.get("name"),
		status: if summary.total_processed >= i64::from(row.get::<i32, _>("total_rows")) {
			"completed".to_string()
		} else {
			row.get("status")
		},
		total_rows: row.get("total_rows"),
		email_column: row.get("email_column"),
		summary,
	}))
}

pub async fn list_summary(pg_pool: &PgPool, list_id: i32) -> Result<Summary, ReacherResponseError> {
	let row = sqlx::query(
		r#"
		SELECT
			COUNT(*) FILTER (WHERE score_category = 'valid') AS total_valid,
			COUNT(*) FILTER (WHERE score_category = 'risky') AS total_risky,
			COUNT(*) FILTER (WHERE score_category = 'unknown') AS total_unknown,
			COUNT(*) FILTER (WHERE score_category = 'invalid') AS total_invalid,
			COUNT(*) FILTER (WHERE task_state NOT IN ('queued', 'running', 'retrying')) AS total_processed
		FROM v1_task_result
		WHERE (extra->>'list_id')::INTEGER = $1
		"#,
	)
	.bind(list_id)
	.fetch_one(pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	Ok(Summary {
		total_valid: row.get::<Option<i64>, _>("total_valid").unwrap_or(0),
		total_risky: row.get::<Option<i64>, _>("total_risky").unwrap_or(0),
		total_unknown: row.get::<Option<i64>, _>("total_unknown").unwrap_or(0),
		total_invalid: row.get::<Option<i64>, _>("total_invalid").unwrap_or(0),
		total_processed: row.get::<Option<i64>, _>("total_processed").unwrap_or(0),
	})
}

/// GET /v1/lists/{list_id}
#[utoipa::path(
	get,
	path = "/v1/lists/{list_id}",
	tag = "v1",
	params(("list_id" = i32, Path, description = "List identifier")),
	responses((status = 200, description = "List detail"))
)]
pub fn v1_get_list(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "lists" / i32)
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and_then(http_handler)
		.with(warp::log(LOG_TARGET))
}
