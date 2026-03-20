use crate::config::BackendConfig;
use crate::finder::require_tenant_id;
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
	deleted: bool,
}

async fn http_handler(
	id: i32,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;

	let result = sqlx::query("DELETE FROM v1_suppression_entries WHERE id = $1 AND tenant_id = $2")
		.bind(id)
		.bind(tenant_id)
		.execute(&pg_pool)
		.await
		.map_err(ReacherResponseError::from)?;

	if result.rows_affected() == 0 {
		return Err(ReacherResponseError::new(
			StatusCode::NOT_FOUND,
			"Suppression entry not found",
		)
		.into());
	}

	Ok(warp::reply::json(&Response { deleted: true }))
}

/// DELETE /v1/suppressions/{id}
#[utoipa::path(
	delete,
	path = "/v1/suppressions/{id}",
	tag = "v1",
	params(("id" = i32, Path, description = "Suppression entry identifier")),
	responses((status = 200, description = "Suppression entry deleted"))
)]
pub fn v1_delete_suppression(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "suppressions" / i32)
		.and(warp::delete())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and_then(http_handler)
		.with(warp::log(LOG_TARGET))
}
