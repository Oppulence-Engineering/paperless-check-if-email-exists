use crate::config::BackendConfig;
use crate::http::v0::check_email::post::with_config;
use crate::http::v1::bulk::post::publish_task;
use crate::http::v1::bulk::with_worker_db;
use crate::http::{check_scope, resolve_tenant, ReacherResponseError};
use crate::tenant::context::{scope, TenantContext};
use crate::tenant::quota::{
	check_and_increment_quota_for_count, release_reserved_usage, QuotaCheckResult,
};
use crate::worker::do_work::CheckEmailTask;
use check_if_email_exists::LOG_TARGET;
use lapin::BasicProperties;
use serde::Serialize;
use sqlx::{PgPool, Row};
use std::sync::Arc;
use tracing::info;
use warp::http::StatusCode;
use warp::Filter;

#[derive(Debug, Serialize)]
struct Response {
	job_id: i32,
	status: String,
	tasks_retried: i64,
}

async fn http_handler(
	job_id: i32,
	tenant_ctx: TenantContext,
	config: Arc<BackendConfig>,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::BULK)?;

	let mut tx = pg_pool.begin().await.map_err(ReacherResponseError::from)?;

	// Lock the job row (tenant-scoped)
	let job = sqlx::query(
		"SELECT status::TEXT as status FROM v1_bulk_job WHERE id = $1 AND (tenant_id = $2 OR $2 IS NULL) FOR UPDATE",
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

	// Only completed, failed, or running jobs can be retried
	match status.as_str() {
		"completed" | "failed" | "running" => {}
		"pending" => {
			return Err(ReacherResponseError::new(
				StatusCode::CONFLICT,
				"Job is still pending and has not been processed yet",
			)
			.into())
		}
		"cancelling" | "cancelled" => {
			return Err(ReacherResponseError::new(
				StatusCode::CONFLICT,
				"Cannot retry a cancelled job",
			)
			.into())
		}
		_ => {
			return Err(ReacherResponseError::new(
				StatusCode::CONFLICT,
				format!("Cannot retry job with status: {}", status),
			)
			.into())
		}
	}

	// Fetch and lock retryable task rows atomically (FOR UPDATE prevents
	// concurrent retries from selecting the same rows and ensures the count
	// matches the actual rows we'll publish).
	let retryable_tasks = sqlx::query(
		r#"
		SELECT id, payload
		FROM v1_task_result
		WHERE job_id = $1 AND task_state IN ('failed', 'dead_lettered')
		FOR UPDATE
		"#,
	)
	.bind(job_id)
	.fetch_all(&mut *tx)
	.await
	.map_err(ReacherResponseError::from)?;

	let retryable_count = retryable_tasks.len() as i64;
	let reserved_count = retryable_count as i32;

	if retryable_count == 0 {
		tx.commit().await.map_err(ReacherResponseError::from)?;
		return Ok(warp::reply::json(&Response {
			job_id,
			status,
			tasks_retried: 0,
		}));
	}

	// Check quota before publishing (reject early if over limit).
	// Quota is charged atomically here, using the same pattern as bulk/post.rs.
	match check_and_increment_quota_for_count(Some(&pg_pool), &tenant_ctx, reserved_count).await {
		QuotaCheckResult::Allowed => {}
		QuotaCheckResult::ExceededMonthlyLimit {
			limit,
			used,
			resets_at,
		} => {
			return Err(ReacherResponseError::new(
				StatusCode::TOO_MANY_REQUESTS,
				format!(
					"Monthly email limit of {} reached ({} used). Resets at {}. Cannot retry {} tasks.",
					limit,
					used,
					resets_at.format("%Y-%m-%d %H:%M:%S UTC"),
					retryable_count,
				),
			)
			.into());
		}
	}

	// Collect task IDs for the targeted UPDATE after publish
	let task_ids: Vec<i32> = retryable_tasks
		.iter()
		.map(|r| r.get::<i32, _>("id"))
		.collect();

	// Publish tasks to RabbitMQ BEFORE committing DB changes.
	// If publish fails, the transaction rolls back and tasks stay in
	// failed/dead_lettered, which is safe to retry again.
	let mut published_count = 0_i32;
	let channel = match config.must_worker_config() {
		Ok(worker_config) => worker_config.channel,
		Err(err) => {
			release_unpublished_retry_quota(
				&pg_pool,
				tenant_ctx.tenant_id,
				reserved_count,
				published_count,
			)
			.await;
			return Err(ReacherResponseError::from(err).into());
		}
	};

	let properties = BasicProperties::default()
		.with_content_type("application/json".into())
		.with_priority(1);

	for row in &retryable_tasks {
		let payload: serde_json::Value = row.get("payload");
		let task_db_id: i32 = row.get("id");

		let mut task: CheckEmailTask = match serde_json::from_value(payload) {
			Ok(task) => task,
			Err(err) => {
				release_unpublished_retry_quota(
					&pg_pool,
					tenant_ctx.tenant_id,
					reserved_count,
					published_count,
				)
				.await;
				return Err(ReacherResponseError::from(err).into());
			}
		};

		if let Some(ref mut metadata) = task.metadata {
			metadata.task_db_id = Some(task_db_id);
		} else {
			task.metadata = Some(crate::worker::do_work::TaskMetadata {
				tenant_id: tenant_ctx.tenant_id.map(|id| id.to_string()),
				request_id: None,
				correlation_id: None,
				created_by: None,
				retry_policy: None,
				dedupe_key: None,
				task_db_id: Some(task_db_id),
			});
		}

		if let Err(err) = publish_task(channel.clone(), task, properties.clone()).await {
			release_unpublished_retry_quota(
				&pg_pool,
				tenant_ctx.tenant_id,
				reserved_count,
				published_count,
			)
			.await;
			return Err(err.into());
		}
		published_count += 1;
	}

	// All tasks published: reset only the exact rows we locked and published.
	sqlx::query(
		r#"
		UPDATE v1_task_result
		SET task_state = 'queued'::task_state,
		    result = NULL,
		    error = NULL,
		    score = NULL,
		    score_category = NULL,
		    sub_reason = NULL,
		    safe_to_send = NULL,
		    reason_codes = NULL,
		    bounce_risk_score = NULL,
		    bounce_risk_category = NULL,
		    bounce_risk_confidence = NULL,
		    bounce_risk_action = NULL,
		    bounce_risk_model_version = NULL,
		    bounce_risk_signals = NULL,
		    retry_count = 0,
		    completed_at = NULL,
		    updated_at = NOW()
		WHERE id = ANY($1)
		"#,
	)
	.bind(&task_ids)
	.execute(&mut *tx)
	.await
	.map_err(ReacherResponseError::from)?;

	// If the job was completed or failed, set it back to running
	let new_status = match status.as_str() {
		"completed" | "failed" => {
			sqlx::query(
				"UPDATE v1_bulk_job SET status = 'running'::job_state, completed_at = NULL, updated_at = NOW() WHERE id = $1",
			)
			.bind(job_id)
			.execute(&mut *tx)
			.await
			.map_err(ReacherResponseError::from)?;
			"running"
		}
		_ => status.as_str(),
	};

	tx.commit().await.map_err(ReacherResponseError::from)?;

	info!(
		target: LOG_TARGET,
		job_id = job_id,
		tasks_retried = retryable_count,
		"Retried failed tasks"
	);

	// Record event (fire-and-forget)
	let _ = sqlx::query(
		"INSERT INTO job_events (job_id, event_type, event_data, actor) VALUES ($1, 'job.retry_requested', $2, 'api')",
	)
	.bind(job_id)
	.bind(serde_json::json!({ "tasks_retried": retryable_count }))
	.execute(&pg_pool)
	.await;

	Ok(warp::reply::json(&Response {
		job_id,
		status: new_status.to_string(),
		tasks_retried: retryable_count,
	}))
}

async fn release_unpublished_retry_quota(
	pg_pool: &PgPool,
	tenant_id: Option<uuid::Uuid>,
	reserved_count: i32,
	published_count: i32,
) {
	let refund_count = reserved_count.saturating_sub(published_count);
	if refund_count > 0 {
		let _ = release_reserved_usage(Some(pg_pool), tenant_id, refund_count).await;
	}
}

/// POST /v1/jobs/{job_id}/retry
///
/// Retries all failed or dead-lettered tasks in a tenant-scoped bulk job.
#[utoipa::path(
	post,
	path = "/v1/jobs/{job_id}/retry",
	tag = "Jobs",
	params(("job_id" = i32, Path, description = "Bulk job identifier")),
	responses((status = 200, description = "Retry initiated"))
)]
pub fn v1_retry_job(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "jobs" / i32 / "retry")
		.and(warp::post())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_config(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and_then(http_handler)
		.with(warp::log(LOG_TARGET))
}
