use crate::config::BackendConfig;
use crate::finder::require_tenant_id;
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
struct Response {
	deleted: bool,
}

async fn http_handler(
	list_id: i32,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::LISTS)?;

	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	delete_list_records(&pg_pool, tenant_id, list_id).await?;
	Ok(warp::reply::json(&Response { deleted: true }))
}

pub async fn delete_list_records(
	pool: &PgPool,
	tenant_id: uuid::Uuid,
	list_id: i32,
) -> Result<(), ReacherResponseError> {
	let mut tx = pool.begin().await?;
	// Retry locks the job first; use the same order before inspecting the list.
	let job_status: Option<String> = sqlx::query_scalar(
		"SELECT status::TEXT FROM v1_bulk_job WHERE id = (SELECT job_id FROM v1_lists WHERE id = $1 AND tenant_id = $2) FOR UPDATE",
	).bind(list_id).bind(tenant_id).fetch_optional(&mut *tx).await?;
	let row = sqlx::query(
		"SELECT job_id, status::TEXT AS status FROM v1_lists WHERE id = $1 AND tenant_id = $2 FOR UPDATE",
	).bind(list_id).bind(tenant_id).fetch_optional(&mut *tx).await?
		.ok_or_else(|| ReacherResponseError::new(StatusCode::NOT_FOUND, "List not found"))?;
	let job_id: Option<i32> = row.get("job_id");
	let active: bool = sqlx::query_scalar(
		"SELECT EXISTS(SELECT 1 FROM v1_task_result t WHERE (job_id = $1 OR extra->>'list_id' = $2) AND (task_state IN ('queued', 'running', 'retrying') OR (dispatch_id IS NOT NULL AND NOT completion_done) OR EXISTS(SELECT 1 FROM verification_dispatch d WHERE d.task_id = t.id)))",
	).bind(job_id).bind(list_id.to_string()).fetch_one(&mut *tx).await?;
	if !matches!(
		row.get::<String, _>("status").as_str(),
		"completed" | "failed" | "deleted"
	) || job_status
		.as_deref()
		.is_some_and(|status| !matches!(status, "completed" | "failed" | "cancelled"))
		|| active
	{
		return Err(ReacherResponseError::new(
			StatusCode::CONFLICT,
			"List has active or unpublished work",
		));
	}
	sqlx::query("DELETE FROM v1_task_result WHERE job_id = $1 OR extra->>'list_id' = $2")
		.bind(job_id)
		.bind(list_id.to_string())
		.execute(&mut *tx)
		.await?;
	sqlx::query("DELETE FROM v1_lists WHERE id = $1")
		.bind(list_id)
		.execute(&mut *tx)
		.await?;
	if let Some(job_id) = job_id {
		sqlx::query("DELETE FROM v1_bulk_job WHERE id = $1")
			.bind(job_id)
			.execute(&mut *tx)
			.await?;
	}
	tx.commit().await?;
	Ok(())
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
