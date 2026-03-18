use crate::config::BackendConfig;
use crate::http::v1::bulk::with_worker_db;
use crate::http::{resolve_tenant, ReacherResponseError};
use crate::tenant::context::TenantContext;
use check_if_email_exists::LOG_TARGET;
use serde::Serialize;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::sync::Arc;
use warp::http::StatusCode;
use warp::Filter;

#[derive(Debug, Serialize)]
struct TaskStateSummary {
	queued: i64,
	running: i64,
	completed: i64,
	retrying: i64,
	failed: i64,
	cancelled: i64,
	dead_lettered: i64,
}

#[derive(Debug, Serialize)]
struct Response {
	job_id: i32,
	status: String,
	total_records: i32,
	created_at: DateTime<Utc>,
	updated_at: DateTime<Utc>,
	completed_at: Option<DateTime<Utc>>,
	cancelled_at: Option<DateTime<Utc>>,
	task_summary: TaskStateSummary,
}

async fn http_handler(
	job_id: i32,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	let job = sqlx::query!(
		r#"
		SELECT id, total_records, status as "status: String",
		       created_at, updated_at, completed_at, cancelled_at
		FROM v1_bulk_job WHERE id = $1 AND (tenant_id = $2 OR $2 IS NULL)
		"#,
		job_id,
		tenant_ctx.tenant_id,
	)
	.fetch_optional(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let job = match job {
		Some(j) => j,
		None => {
			return Err(
				ReacherResponseError::new(StatusCode::NOT_FOUND, "Job not found").into(),
			)
		}
	};

	let task_summary = sqlx::query!(
		r#"
		SELECT
			COUNT(CASE WHEN task_state = 'queued' THEN 1 END) as queued,
			COUNT(CASE WHEN task_state = 'running' THEN 1 END) as running,
			COUNT(CASE WHEN task_state = 'completed' THEN 1 END) as completed,
			COUNT(CASE WHEN task_state = 'retrying' THEN 1 END) as retrying,
			COUNT(CASE WHEN task_state = 'failed' THEN 1 END) as failed,
			COUNT(CASE WHEN task_state = 'cancelled' THEN 1 END) as cancelled,
			COUNT(CASE WHEN task_state = 'dead_lettered' THEN 1 END) as dead_lettered
		FROM v1_task_result WHERE job_id = $1
		"#,
		job_id
	)
	.fetch_one(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	Ok(warp::reply::json(&Response {
		job_id: job.id,
		status: job.status,
		total_records: job.total_records,
		created_at: job.created_at,
		updated_at: job.updated_at,
		completed_at: job.completed_at,
		cancelled_at: job.cancelled_at,
		task_summary: TaskStateSummary {
			queued: task_summary.queued.unwrap_or(0),
			running: task_summary.running.unwrap_or(0),
			completed: task_summary.completed.unwrap_or(0),
			retrying: task_summary.retrying.unwrap_or(0),
			failed: task_summary.failed.unwrap_or(0),
			cancelled: task_summary.cancelled.unwrap_or(0),
			dead_lettered: task_summary.dead_lettered.unwrap_or(0),
		},
	}))
}

/// GET /v1/jobs/{job_id}
///
/// Returns progress summary and state for a tenant-scoped bulk job.
#[utoipa::path(
	get,
	path = "/v1/jobs/{job_id}",
	tag = "Jobs",
	params(("job_id" = i32, Path, description = "Bulk job identifier")),
	responses((status = 200, description = "Bulk job progress summary"))
)]
pub fn v1_get_job_status(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "jobs" / i32)
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and_then(http_handler)
		.with(warp::log(LOG_TARGET))
}
