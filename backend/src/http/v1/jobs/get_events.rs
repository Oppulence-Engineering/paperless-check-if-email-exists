use crate::config::BackendConfig;
use crate::http::v1::bulk::with_worker_db;
use crate::http::{resolve_tenant, ReacherResponseError};
use crate::tenant::context::TenantContext;
use check_if_email_exists::LOG_TARGET;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::PgPool;
use std::sync::Arc;
use warp::http::StatusCode;
use warp::Filter;

#[derive(Debug, Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
struct Query {
	limit: Option<i64>,
	offset: Option<i64>,
}

#[derive(Debug, Serialize)]
struct Event {
	id: i64,
	job_id: i32,
	task_id: Option<i32>,
	event_type: String,
	event_data: Option<serde_json::Value>,
	actor: Option<String>,
	created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
struct Response {
	events: Vec<Event>,
	total: i64,
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
	let offset = query.offset.unwrap_or(0);

	let total = sqlx::query_scalar!("SELECT COUNT(*) FROM job_events WHERE job_id = $1", job_id,)
		.fetch_one(&pg_pool)
		.await
		.map_err(ReacherResponseError::from)?
		.unwrap_or(0);

	let events = sqlx::query_as!(
		Event,
		r#"
		SELECT id, job_id, task_id, event_type, event_data, actor, created_at
		FROM job_events
		WHERE job_id = $1
		ORDER BY created_at ASC
		LIMIT $2 OFFSET $3
		"#,
		job_id,
		limit,
		offset,
	)
	.fetch_all(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	Ok(warp::reply::json(&Response { events, total }))
}

/// GET /v1/jobs/{job_id}/events
///
/// Returns paginated event history for a tenant-scoped bulk job.
#[utoipa::path(
	get,
	path = "/v1/jobs/{job_id}/events",
	tag = "Jobs",
	params(("job_id" = i32, Path, description = "Bulk job identifier"), Query),
	responses((status = 200, description = "Job events"))
)]
pub fn v1_get_job_events(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "jobs" / i32 / "events")
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::query::<Query>())
		.and_then(http_handler)
		.with(warp::log(LOG_TARGET))
}
