use crate::config::BackendConfig;
use crate::http::v1::bulk::with_worker_db;
use crate::http::{resolve_tenant, ReacherResponseError};
use crate::tenant::context::TenantContext;
use check_if_email_exists::LOG_TARGET;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{PgPool, Row};
use std::sync::Arc;
use warp::http::StatusCode;
use warp::Filter;

#[derive(Debug, Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
struct Query {
	preview_limit: Option<i64>,
}

#[derive(Debug, Serialize)]
struct TaskStateCounts {
	queued: i64,
	running: i64,
	retrying: i64,
	completed: i64,
	failed: i64,
	dead_lettered: i64,
	cancelled: i64,
}

#[derive(Debug, Serialize)]
struct RetrySummary {
	retryable_rows: i64,
	permanent_failures: i64,
	rows_with_retries: i64,
	max_retry_count: i32,
	avg_retry_count: f64,
}

#[derive(Debug, Serialize)]
struct FailureBreakdown {
	error: String,
	count: i64,
}

#[derive(Debug, Serialize)]
struct FailedRowPreview {
	id: i32,
	row_index: Option<i32>,
	input: Option<String>,
	task_state: String,
	error: Option<String>,
	retry_count: i32,
	updated_at: String,
}

#[derive(Debug, Serialize)]
struct Response {
	job_id: i32,
	status: String,
	total_records: i32,
	processed_rows: i64,
	progress_pct: f64,
	task_states: TaskStateCounts,
	failures_total: i64,
	retry_summary: RetrySummary,
	estimated_completion_seconds: Option<i64>,
	failure_breakdown: Vec<FailureBreakdown>,
	failed_rows: Vec<FailedRowPreview>,
	failure_report_url: String,
	created_at: String,
	updated_at: String,
	completed_at: Option<String>,
}

async fn http_handler(
	job_id: i32,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	query: Query,
) -> Result<impl warp::Reply, warp::Rejection> {
	let job = sqlx::query(
		r#"
		SELECT id, total_records, status::TEXT AS status, created_at, updated_at, completed_at
		FROM v1_bulk_job
		WHERE id = $1 AND (tenant_id = $2 OR $2 IS NULL)
		"#,
	)
	.bind(job_id)
	.bind(tenant_ctx.tenant_id)
	.fetch_optional(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;
	let job =
		job.ok_or_else(|| ReacherResponseError::new(StatusCode::NOT_FOUND, "Job not found"))?;

	let counts = load_counts(&pg_pool, job_id).await?;
	let processed_rows = counts.completed + counts.failed + counts.dead_lettered + counts.cancelled;
	let total_records: i32 = job.get("total_records");
	let progress_pct = progress_pct(processed_rows, total_records);
	let retry_summary = load_retry_summary(&pg_pool, job_id, &counts).await?;
	let created_at: DateTime<Utc> = job.get("created_at");
	let updated_at: DateTime<Utc> = job.get("updated_at");
	let estimated_completion_seconds =
		estimate_completion_seconds(total_records, processed_rows, created_at, Utc::now());
	let failure_breakdown = load_failure_breakdown(&pg_pool, job_id).await?;
	let failed_rows =
		load_failed_rows(&pg_pool, job_id, query.preview_limit.unwrap_or(50).min(200)).await?;
	let completed_at = job
		.get::<Option<DateTime<Utc>>, _>("completed_at")
		.map(|ts| ts.to_rfc3339());

	Ok(warp::reply::json(&Response {
		job_id: job.get("id"),
		status: job.get("status"),
		total_records,
		processed_rows,
		progress_pct,
		task_states: counts,
		failures_total: retry_summary.retryable_rows + retry_summary.permanent_failures,
		retry_summary,
		estimated_completion_seconds,
		failure_breakdown,
		failed_rows,
		failure_report_url: format!("/v1/jobs/{}/failure-report?format=csv", job_id),
		created_at: created_at.to_rfc3339(),
		updated_at: updated_at.to_rfc3339(),
		completed_at,
	}))
}

async fn load_counts(
	pg_pool: &PgPool,
	job_id: i32,
) -> Result<TaskStateCounts, ReacherResponseError> {
	let row = sqlx::query(
		r#"
		SELECT
			COUNT(*) FILTER (WHERE task_state = 'queued') AS queued,
			COUNT(*) FILTER (WHERE task_state = 'running') AS running,
			COUNT(*) FILTER (WHERE task_state = 'retrying') AS retrying,
			COUNT(*) FILTER (WHERE task_state = 'completed') AS completed,
			COUNT(*) FILTER (WHERE task_state = 'failed') AS failed,
			COUNT(*) FILTER (WHERE task_state = 'dead_lettered') AS dead_lettered,
			COUNT(*) FILTER (WHERE task_state = 'cancelled') AS cancelled
		FROM v1_task_result
		WHERE job_id = $1
		"#,
	)
	.bind(job_id)
	.fetch_one(pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	Ok(TaskStateCounts {
		queued: row.get::<Option<i64>, _>("queued").unwrap_or(0),
		running: row.get::<Option<i64>, _>("running").unwrap_or(0),
		retrying: row.get::<Option<i64>, _>("retrying").unwrap_or(0),
		completed: row.get::<Option<i64>, _>("completed").unwrap_or(0),
		failed: row.get::<Option<i64>, _>("failed").unwrap_or(0),
		dead_lettered: row.get::<Option<i64>, _>("dead_lettered").unwrap_or(0),
		cancelled: row.get::<Option<i64>, _>("cancelled").unwrap_or(0),
	})
}

async fn load_retry_summary(
	pg_pool: &PgPool,
	job_id: i32,
	counts: &TaskStateCounts,
) -> Result<RetrySummary, ReacherResponseError> {
	let row = sqlx::query(
		r#"
		SELECT
			COUNT(*) FILTER (WHERE retry_count > 0) AS rows_with_retries,
			COALESCE(MAX(retry_count), 0) AS max_retry_count,
			COALESCE(AVG(retry_count::FLOAT), 0)::FLOAT8 AS avg_retry_count
		FROM v1_task_result
		WHERE job_id = $1
		"#,
	)
	.bind(job_id)
	.fetch_one(pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	Ok(RetrySummary {
		retryable_rows: counts.failed + counts.dead_lettered,
		permanent_failures: counts.cancelled,
		rows_with_retries: row.get::<Option<i64>, _>("rows_with_retries").unwrap_or(0),
		max_retry_count: row.get::<Option<i32>, _>("max_retry_count").unwrap_or(0),
		avg_retry_count: round_two(row.get::<Option<f64>, _>("avg_retry_count").unwrap_or(0.0)),
	})
}

async fn load_failure_breakdown(
	pg_pool: &PgPool,
	job_id: i32,
) -> Result<Vec<FailureBreakdown>, ReacherResponseError> {
	let rows = sqlx::query(
		r#"
		SELECT COALESCE(NULLIF(error, ''), task_state::TEXT) AS error, COUNT(*) AS count
		FROM v1_task_result
		WHERE job_id = $1
		  AND task_state IN ('failed', 'dead_lettered', 'cancelled')
		GROUP BY COALESCE(NULLIF(error, ''), task_state::TEXT)
		ORDER BY count DESC, error ASC
		LIMIT 10
		"#,
	)
	.bind(job_id)
	.fetch_all(pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	Ok(rows
		.into_iter()
		.map(|row| FailureBreakdown {
			error: row.get("error"),
			count: row.get("count"),
		})
		.collect())
}

async fn load_failed_rows(
	pg_pool: &PgPool,
	job_id: i32,
	limit: i64,
) -> Result<Vec<FailedRowPreview>, ReacherResponseError> {
	let rows = sqlx::query(
		r#"
		SELECT id, extra, payload, task_state::TEXT AS task_state, error, retry_count, updated_at
		FROM v1_task_result
		WHERE job_id = $1
		  AND task_state IN ('failed', 'dead_lettered', 'cancelled')
		ORDER BY updated_at DESC, id DESC
		LIMIT $2
		"#,
	)
	.bind(job_id)
	.bind(limit)
	.fetch_all(pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	Ok(rows
		.into_iter()
		.map(|row| {
			let extra: Value = row.get("extra");
			let payload: Value = row.get("payload");
			FailedRowPreview {
				id: row.get("id"),
				row_index: extra
					.get("row_index")
					.and_then(Value::as_i64)
					.map(|value| value as i32),
				input: payload_input(&payload),
				task_state: row.get("task_state"),
				error: row.get("error"),
				retry_count: row.get("retry_count"),
				updated_at: row.get::<DateTime<Utc>, _>("updated_at").to_rfc3339(),
			}
		})
		.collect())
}

fn progress_pct(processed_rows: i64, total_records: i32) -> f64 {
	if total_records <= 0 {
		return 100.0;
	}
	round_two((processed_rows as f64 / f64::from(total_records)).min(1.0) * 100.0)
}

fn estimate_completion_seconds(
	total_records: i32,
	processed_rows: i64,
	started_at: DateTime<Utc>,
	now: DateTime<Utc>,
) -> Option<i64> {
	let remaining = i64::from(total_records).saturating_sub(processed_rows);
	if remaining == 0 || processed_rows <= 0 {
		return None;
	}
	let elapsed = (now - started_at).num_seconds().max(1);
	let rows_per_second = processed_rows as f64 / elapsed as f64;
	if rows_per_second <= 0.0 {
		return None;
	}
	Some((remaining as f64 / rows_per_second).ceil() as i64)
}

fn payload_input(payload: &Value) -> Option<String> {
	payload
		.get("input")
		.and_then(|value| {
			value
				.get("to_email")
				.and_then(Value::as_str)
				.or_else(|| value.as_str())
		})
		.or_else(|| payload.get("to_email").and_then(Value::as_str))
		.map(ToOwned::to_owned)
}

fn round_two(value: f64) -> f64 {
	(value * 100.0).round() / 100.0
}

/// GET /v1/jobs/{job_id}/failure-center
pub fn v1_job_failure_center(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "jobs" / i32 / "failure-center")
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::query::<Query>())
		.and_then(http_handler)
		.with(warp::log(LOG_TARGET))
}

#[cfg(test)]
mod tests {
	use super::*;
	use chrono::TimeZone;
	use serde_json::json;

	#[test]
	fn progress_percentage_is_capped() {
		assert_eq!(progress_pct(15, 10), 100.0);
		assert_eq!(progress_pct(3, 10), 30.0);
	}

	#[test]
	fn estimates_completion_from_elapsed_rate() {
		let started = Utc.with_ymd_and_hms(2026, 6, 30, 12, 0, 0).unwrap();
		let now = Utc.with_ymd_and_hms(2026, 6, 30, 12, 1, 0).unwrap();

		assert_eq!(
			estimate_completion_seconds(100, 30, started, now),
			Some(140)
		);
	}

	#[test]
	fn payload_input_handles_nested_task_payload() {
		assert_eq!(
			payload_input(&json!({"input": {"to_email": "user@example.com"}})),
			Some("user@example.com".to_string())
		);
	}
}
