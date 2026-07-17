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
	status: String,
}

async fn http_handler(
	id: i32,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	let mut tx = pg_pool.begin().await.map_err(ReacherResponseError::from)?;

	let row = sqlx::query(
		r#"
		UPDATE v1_suppression_entries
		SET status = 'inactive',
		    updated_at = NOW()
		WHERE id = $1 AND tenant_id = $2 AND status = 'active'
		RETURNING id, canonical_email, reason_code, reason_detail, source_type, source_ref, status
		"#,
	)
	.bind(id)
	.bind(tenant_id)
	.fetch_optional(&mut *tx)
	.await
	.map_err(ReacherResponseError::from)?;
	let row = row.ok_or_else(|| {
		ReacherResponseError::new(StatusCode::NOT_FOUND, "Suppression entry not found")
	})?;

	sqlx::query(
		r#"
		INSERT INTO v1_suppression_events (
			tenant_id, entry_id, canonical_email, event_type, from_status, to_status,
			reason_code, reason_detail, source_type, source_ref, actor_type
		)
		VALUES ($1, $2, $3, 'deactivated', 'active', 'inactive', $4, $5, $6, $7, 'api')
		"#,
	)
	.bind(tenant_id)
	.bind(row.get::<i32, _>("id"))
	.bind(row.get::<String, _>("canonical_email"))
	.bind(row.get::<Option<String>, _>("reason_code"))
	.bind(row.get::<Option<String>, _>("reason_detail"))
	.bind(row.get::<Option<String>, _>("source_type"))
	.bind(row.get::<Option<String>, _>("source_ref"))
	.execute(&mut *tx)
	.await
	.map_err(ReacherResponseError::from)?;
	tx.commit().await.map_err(ReacherResponseError::from)?;

	Ok(warp::reply::json(&Response {
		deleted: true,
		status: row.get("status"),
	}))
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
