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
struct Response {
	deleted: bool,
}

async fn http_handler(
	list_id: i32,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	let row = sqlx::query(
		"SELECT job_id, status::TEXT AS status FROM v1_lists WHERE id = $1 AND tenant_id = $2",
	)
	.bind(list_id)
	.bind(tenant_id)
	.fetch_optional(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;
	let row = row.ok_or_else(|| {
		warp::reject::custom(ReacherResponseError::new(
			StatusCode::NOT_FOUND,
			"List not found",
		))
	})?;

	if row.get::<String, _>("status") == "processing" {
		return Err(
			ReacherResponseError::new(StatusCode::CONFLICT, "List is still processing").into(),
		);
	}

	let job_id: i32 = row.get("job_id");
	sqlx::query("DELETE FROM v1_task_result WHERE (extra->>'list_id')::INTEGER = $1")
		.bind(list_id)
		.execute(&pg_pool)
		.await
		.map_err(ReacherResponseError::from)?;
	sqlx::query("DELETE FROM v1_lists WHERE id = $1")
		.bind(list_id)
		.execute(&pg_pool)
		.await
		.map_err(ReacherResponseError::from)?;
	sqlx::query("DELETE FROM v1_bulk_job WHERE id = $1")
		.bind(job_id)
		.execute(&pg_pool)
		.await
		.map_err(ReacherResponseError::from)?;

	Ok(warp::reply::json(&Response { deleted: true }))
}

/// DELETE /v1/lists/{list_id}
#[utoipa::path(
	delete,
	path = "/v1/lists/{list_id}",
	tag = "v1",
	params(("list_id" = i32, Path, description = "List identifier")),
	responses((status = 200, description = "List deleted"))
)]
pub fn v1_delete_list(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "lists" / i32)
		.and(warp::delete())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and_then(http_handler)
		.with(warp::log(LOG_TARGET))
}
