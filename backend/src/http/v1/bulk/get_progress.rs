//! This file implements the `GET /v1/bulk/{id}` endpoint.

use std::sync::Arc;

use check_if_email_exists::LOG_TARGET;
use serde::Serialize;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use warp::http::StatusCode;
use warp::Filter;

use super::with_worker_read_db;
use crate::config::BackendConfig;
use crate::http::ReacherResponseError;
use crate::http::{check_scope, resolve_tenant};
use crate::tenant::context::{scope, TenantContext};

#[derive(Debug, Serialize, PartialEq, Eq)]
enum ValidStatus {
	Running,
	Completed,
}

#[derive(sqlx::FromRow, Debug, Serialize)]
struct JobRecord {
	id: i32,
	created_at: DateTime<Utc>,
	total_records: i32,
}

#[derive(Debug, Serialize)]
struct ResponseSummary {
	total_safe: i32,
	total_risky: i32,
	total_invalid: i32,
	total_unknown: i32,
}

#[derive(Debug, Serialize)]
struct Response {
	job_id: i32,
	created_at: DateTime<Utc>,
	finished_at: Option<DateTime<Utc>>,
	total_records: i32,
	total_processed: i32,
	summary: ResponseSummary,
	job_status: ValidStatus,
}

async fn http_handler(
	job_id: i32,
	tenant_ctx: TenantContext,
	conn_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::BULK)?;

	let job_rec = sqlx::query_as!(
		JobRecord,
		r#"
		SELECT id, created_at, total_records FROM v1_bulk_job
		WHERE id = $1 AND (tenant_id = $2 OR $2 IS NULL)
		LIMIT 1
		"#,
		job_id,
		tenant_ctx.tenant_id,
	)
	.fetch_optional(&conn_pool)
	.await
	.map_err(ReacherResponseError::from)?
	.ok_or_else(|| ReacherResponseError::new(StatusCode::NOT_FOUND, "Job not found"))?;

	// Every terminal state counts as processed. Cancelled tasks carry neither a
	// result nor an error, so counting by those columns left cancelled jobs
	// running forever.
	let agg_info = sqlx::query(
		r#"
		SELECT
			COUNT(*) AS total_processed,
			COUNT(CASE WHEN result ->> 'is_reachable' LIKE 'safe' THEN 1 END) AS safe_count,
			COUNT(CASE WHEN result ->> 'is_reachable' LIKE 'risky' THEN 1 END) AS risky_count,
			COUNT(CASE WHEN result ->> 'is_reachable' LIKE 'invalid' THEN 1 END) AS invalid_count,
			COUNT(CASE WHEN result ->> 'is_reachable' LIKE 'unknown' THEN 1 END) AS unknown_count,
			-- Rows written before completed_at existed only have created_at.
			MAX(COALESCE(completed_at, created_at)) AS finished_at
		FROM v1_task_result
		WHERE job_id = $1 AND task_state NOT IN ('queued', 'running', 'retrying')
		"#,
	)
	.bind(job_id)
	.fetch_one(&conn_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let total_processed = agg_info.get::<i64, _>("total_processed") as i32;
	let (job_status, finished_at) = if total_processed < job_rec.total_records {
		(ValidStatus::Running, None)
	} else {
		// NULL for a job with no tasks.
		(
			ValidStatus::Completed,
			agg_info.get::<Option<DateTime<Utc>>, _>("finished_at"),
		)
	};

	Ok(warp::reply::json(&Response {
		job_id: job_rec.id,
		created_at: job_rec.created_at,
		finished_at,
		total_records: job_rec.total_records,
		total_processed,
		summary: ResponseSummary {
			total_safe: agg_info.get::<i64, _>("safe_count") as i32,
			total_risky: agg_info.get::<i64, _>("risky_count") as i32,
			total_invalid: agg_info.get::<i64, _>("invalid_count") as i32,
			total_unknown: agg_info.get::<i64, _>("unknown_count") as i32,
		},
		job_status,
	}))
}

/// GET /v1/bulk/{job_id}
///
/// Returns status and current progress for a tenant-scoped v1 bulk job.
#[utoipa::path(
	get,
	path = "/v1/bulk/{job_id}",
	tag = "Jobs",
	params(("job_id" = i32, Path, description = "V1 bulk job identifier")),
	responses((status = 200, description = "Bulk job progress"))
)]
pub fn v1_get_bulk_job_progress(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "bulk" / i32)
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_read_db(config))
		.and_then(http_handler)
		.with(warp::log(LOG_TARGET))
}
