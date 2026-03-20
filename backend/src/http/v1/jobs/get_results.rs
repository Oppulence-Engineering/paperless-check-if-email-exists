use crate::config::BackendConfig;
use crate::http::v1::bulk::with_worker_db;
use crate::http::{resolve_tenant, ReacherResponseError};
use crate::scoring::response::inject_freshness_into_result;
use crate::tenant::context::TenantContext;
use check_if_email_exists::LOG_TARGET;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use warp::http::StatusCode;
use warp::Filter;

#[derive(Debug, Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
struct Query {
	cursor: Option<i64>,
	limit: Option<i64>,
	state: Option<String>,
}

#[derive(Debug, Serialize)]
struct TaskResult {
	id: i64,
	task_state: String,
	result: Option<serde_json::Value>,
	error: Option<String>,
	retry_count: i32,
}

#[derive(Debug, Serialize)]
struct Response {
	results: Vec<TaskResult>,
	next_cursor: Option<i64>,
	has_more: bool,
}

async fn http_handler(
	job_id: i32,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	query: Query,
) -> Result<impl warp::Reply, warp::Rejection> {
	// Verify job belongs to tenant
	let job_exists = sqlx::query_scalar!(
		"SELECT EXISTS(SELECT 1 FROM v1_bulk_job WHERE id = $1 AND (tenant_id = $2 OR $2 IS NULL)) as exists",
		job_id, tenant_ctx.tenant_id,
	).fetch_one(&pg_pool).await.map_err(ReacherResponseError::from)?;
	if !job_exists.unwrap_or(false) {
		return Err(ReacherResponseError::new(StatusCode::NOT_FOUND, "Job not found").into());
	}

	let limit = query.limit.unwrap_or(50).min(200);
	let state_filter = query.state.as_deref().unwrap_or("completed");

	let results = sqlx::query_as::<
		_,
		(
			i32,
			String,
			Option<serde_json::Value>,
			Option<String>,
			i32,
			Option<DateTime<Utc>>,
		),
	>(
		r#"
		SELECT id, task_state::TEXT, result, error, retry_count, completed_at
		FROM v1_task_result
		WHERE job_id = $1
		  AND task_state = $4::task_state
		  AND ($2::BIGINT IS NULL OR id > $2)
		ORDER BY id ASC
		LIMIT $3
		"#,
	)
	.bind(job_id)
	.bind(query.cursor)
	.bind(limit + 1)
	.bind(state_filter)
	.fetch_all(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let has_more = results.len() as i64 > limit;
	let results: Vec<TaskResult> = results
		.into_iter()
		.take(limit as usize)
		.map(|r| {
			let mut result = r.2;
			if let (Some(ref mut res), Some(completed_at)) = (&mut result, r.5) {
				inject_freshness_into_result(res, completed_at);
			}
			TaskResult {
				id: r.0 as i64,
				task_state: r.1,
				result,
				error: r.3,
				retry_count: r.4,
			}
		})
		.collect();

	let next_cursor = if has_more {
		results.last().map(|r| r.id)
	} else {
		None
	};

	Ok(warp::reply::json(&Response {
		results,
		next_cursor,
		has_more,
	}))
}

/// GET /v1/jobs/{job_id}/results
///
/// Returns paginated completed task results for a tenant-scoped bulk job.
#[utoipa::path(
	get,
	path = "/v1/jobs/{job_id}/results",
	tag = "Jobs",
	params(("job_id" = i32, Path, description = "Bulk job identifier"), Query),
	responses((status = 200, description = "Job result page"))
)]
pub fn v1_get_job_results(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "jobs" / i32 / "results")
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::query::<Query>())
		.and_then(http_handler)
		.with(warp::log(LOG_TARGET))
}
