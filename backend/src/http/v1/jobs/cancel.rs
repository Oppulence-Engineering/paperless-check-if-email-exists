use crate::config::BackendConfig;
use crate::http::v1::bulk::with_worker_db;
use crate::http::{resolve_tenant, ReacherResponseError};
use crate::tenant::context::TenantContext;
use check_if_email_exists::LOG_TARGET;
use serde::Serialize;
use sqlx::PgPool;
use std::sync::Arc;
use warp::http::StatusCode;
use warp::Filter;

#[derive(Debug, Serialize)]
struct Response {
	job_id: i32,
	status: String,
	tasks_cancelled: i64,
}

async fn http_handler(
	job_id: i32,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	// Use a transaction with SELECT FOR UPDATE to prevent TOCTOU races
	let mut tx = pg_pool.begin().await.map_err(ReacherResponseError::from)?;

	// Check current job status (tenant-scoped, locked)
	let job = sqlx::query(
		"SELECT status::TEXT as status FROM v1_bulk_job WHERE id = $1 AND (tenant_id = $2 OR $2 IS NULL) FOR UPDATE"
	)
	.bind(job_id)
	.bind(tenant_ctx.tenant_id)
	.fetch_optional(&mut *tx)
	.await
	.map_err(ReacherResponseError::from)?;

	let job = match job {
		Some(j) => j,
		None => {
			return Err(ReacherResponseError::new(StatusCode::NOT_FOUND, "Job not found").into())
		}
	};

	let status: String = sqlx::Row::get(&job, "status");

	// Only pending or running jobs can be cancelled
	match status.as_str() {
		"pending" | "running" => {}
		"cancelling" | "cancelled" => {
			return Err(ReacherResponseError::new(
				StatusCode::CONFLICT,
				"Job is already being cancelled or was cancelled",
			)
			.into())
		}
		_ => {
			return Err(ReacherResponseError::new(
				StatusCode::CONFLICT,
				format!("Cannot cancel job with status: {}", status),
			)
			.into())
		}
	}

	// Set job to cancelling
	sqlx::query(
		"UPDATE v1_bulk_job SET status = 'cancelling'::job_state, updated_at = NOW() WHERE id = $1",
	)
	.bind(job_id)
	.execute(&mut *tx)
	.await
	.map_err(ReacherResponseError::from)?;

	// Cancel all queued/retrying tasks
	let result = sqlx::query(
		"UPDATE v1_task_result SET task_state = 'cancelled'::task_state, updated_at = NOW(), completed_at = NOW() WHERE job_id = $1 AND task_state IN ('queued', 'retrying')"
	)
	.bind(job_id)
	.execute(&mut *tx)
	.await
	.map_err(ReacherResponseError::from)?;

	let tasks_cancelled = result.rows_affected() as i64;

	// Check if all tasks are now terminal. If so, finalize to cancelled.
	let non_terminal: i64 = sqlx::query_scalar(
		"SELECT COUNT(*) FROM v1_task_result WHERE job_id = $1 AND task_state::TEXT IN ('queued', 'running', 'retrying')"
	)
	.bind(job_id)
	.fetch_one(&mut *tx)
	.await
	.map_err(ReacherResponseError::from)?;

	let final_status = if non_terminal == 0 {
		sqlx::query("UPDATE v1_bulk_job SET status = 'cancelled'::job_state, cancelled_at = NOW(), updated_at = NOW() WHERE id = $1")
			.bind(job_id)
			.execute(&mut *tx)
			.await
			.map_err(ReacherResponseError::from)?;
		"cancelled"
	} else {
		"cancelling"
	};

	tx.commit().await.map_err(ReacherResponseError::from)?;

	// Record event
	let _ = sqlx::query!(
		r#"
		INSERT INTO job_events (job_id, event_type, event_data, actor)
		VALUES ($1, 'job.cancellation_requested', $2, 'api')
		"#,
		job_id,
		serde_json::json!({ "tasks_cancelled": tasks_cancelled }),
	)
	.execute(&pg_pool)
	.await;

	Ok(warp::reply::json(&Response {
		job_id,
		status: final_status.to_string(),
		tasks_cancelled,
	}))
}

/// POST /v1/jobs/{job_id}/cancel
///
/// Requests cancellation for a tenant-scoped bulk job.
#[utoipa::path(
	post,
	path = "/v1/jobs/{job_id}/cancel",
	tag = "Jobs",
	params(("job_id" = i32, Path, description = "Bulk job identifier")),
	responses((status = 200, description = "Job cancellation accepted"))
)]
pub fn v1_cancel_job(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "jobs" / i32 / "cancel")
		.and(warp::post())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and_then(http_handler)
		.with(warp::log(LOG_TARGET))
}
