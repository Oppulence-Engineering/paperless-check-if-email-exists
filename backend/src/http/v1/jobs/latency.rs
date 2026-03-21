use crate::config::BackendConfig;
use crate::http::v1::bulk::with_worker_db;
use crate::http::{check_scope, resolve_tenant, ReacherResponseError};
use crate::tenant::context::{scope, TenantContext};
use check_if_email_exists::LOG_TARGET;
use serde::Serialize;
use sqlx::{PgPool, Row};
use std::sync::Arc;
use warp::http::StatusCode;
use warp::Filter;

#[derive(Debug, Serialize)]
struct LatencyReport {
	job_id: i32,
	total_completed: i64,
	avg_duration_ms: f64,
	min_duration_ms: f64,
	max_duration_ms: f64,
	p50_duration_ms: f64,
	p95_duration_ms: f64,
	p99_duration_ms: f64,
}

async fn http_handler(
	job_id: i32,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::BULK)?;

	let job_exists = sqlx::query_scalar!(
		"SELECT EXISTS(SELECT 1 FROM v1_bulk_job WHERE id = $1 AND (tenant_id = $2 OR $2 IS NULL)) as exists",
		job_id,
		tenant_ctx.tenant_id,
	)
	.fetch_one(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;
	if !job_exists.unwrap_or(false) {
		return Err(ReacherResponseError::new(StatusCode::NOT_FOUND, "Job not found").into());
	}

	let row = sqlx::query(
		r#"
		SELECT
			COUNT(*) AS total_completed,
			COALESCE(AVG(EXTRACT(EPOCH FROM (completed_at - started_at)) * 1000), 0) AS avg_ms,
			COALESCE(MIN(EXTRACT(EPOCH FROM (completed_at - started_at)) * 1000), 0) AS min_ms,
			COALESCE(MAX(EXTRACT(EPOCH FROM (completed_at - started_at)) * 1000), 0) AS max_ms,
			COALESCE(PERCENTILE_CONT(0.50) WITHIN GROUP (ORDER BY EXTRACT(EPOCH FROM (completed_at - started_at)) * 1000), 0) AS p50_ms,
			COALESCE(PERCENTILE_CONT(0.95) WITHIN GROUP (ORDER BY EXTRACT(EPOCH FROM (completed_at - started_at)) * 1000), 0) AS p95_ms,
			COALESCE(PERCENTILE_CONT(0.99) WITHIN GROUP (ORDER BY EXTRACT(EPOCH FROM (completed_at - started_at)) * 1000), 0) AS p99_ms
		FROM v1_task_result
		WHERE job_id = $1
		  AND started_at IS NOT NULL
		  AND completed_at IS NOT NULL
		  AND completed_at > started_at
		"#,
	)
	.bind(job_id)
	.fetch_one(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	Ok(warp::reply::json(&LatencyReport {
		job_id,
		total_completed: row.get::<Option<i64>, _>("total_completed").unwrap_or(0),
		avg_duration_ms: row.get::<Option<f64>, _>("avg_ms").unwrap_or(0.0).round(),
		min_duration_ms: row.get::<Option<f64>, _>("min_ms").unwrap_or(0.0).round(),
		max_duration_ms: row.get::<Option<f64>, _>("max_ms").unwrap_or(0.0).round(),
		p50_duration_ms: row.get::<Option<f64>, _>("p50_ms").unwrap_or(0.0).round(),
		p95_duration_ms: row.get::<Option<f64>, _>("p95_ms").unwrap_or(0.0).round(),
		p99_duration_ms: row.get::<Option<f64>, _>("p99_ms").unwrap_or(0.0).round(),
	}))
}

/// GET /v1/jobs/{job_id}/latency
///
/// Returns verification latency analytics for a job (p50, p95, p99, avg, min, max).
#[utoipa::path(
	get,
	path = "/v1/jobs/{job_id}/latency",
	tag = "Jobs",
	params(("job_id" = i32, Path, description = "Bulk job identifier")),
	responses((status = 200, description = "Latency analytics"))
)]
pub fn v1_job_latency(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "jobs" / i32 / "latency")
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and_then(http_handler)
		.with(warp::log(LOG_TARGET))
}
