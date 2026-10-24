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
	status: String,
}

async fn http_handler(
	id: i32,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::SUPPRESSIONS)?;

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
			reason_code, reason_detail, source_type, source_ref, actor_type, actor_id
		)
		VALUES ($1, $2, $3, 'deactivated', 'active', 'inactive', $4, $5, $6, $7, $8, $9)
		"#,
	)
	.bind(tenant_id)
	.bind(row.get::<i32, _>("id"))
	.bind(row.get::<String, _>("canonical_email"))
	.bind(row.get::<Option<String>, _>("reason_code"))
	.bind(row.get::<Option<String>, _>("reason_detail"))
	.bind(row.get::<Option<String>, _>("source_type"))
	.bind(row.get::<Option<String>, _>("source_ref"))
	.bind(tenant_ctx.actor_type())
	.bind(tenant_ctx.actor_id())
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

async fn restore_handler(
	id: i32,
	ctx: TenantContext,
	pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&ctx, scope::SUPPRESSIONS)?;
	let tenant = require_tenant_id(ctx.tenant_id)?;
	let mut tx = pool.begin().await.map_err(ReacherResponseError::from)?;
	let row = sqlx::query("SELECT canonical_email, expires_at FROM v1_suppression_entries WHERE id=$1 AND tenant_id=$2 AND status='inactive' FOR UPDATE")
        .bind(id).bind(tenant).fetch_optional(&mut *tx).await.map_err(ReacherResponseError::from)?
        .ok_or_else(|| ReacherResponseError::new(StatusCode::NOT_FOUND,"Removed suppression not found"))?;
	let expiry: Option<chrono::DateTime<chrono::Utc>> = row.get("expires_at");
	if expiry.is_some_and(|e| e <= chrono::Utc::now()) {
		return Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"This suppression has expired. Add it again with a future expiry.",
		)
		.into());
	}
	let key: String = row.get("canonical_email");
	// Insert/upsert uses the same uniqueness protection as add, including concurrent restores.
	let restored: i32 = sqlx::query_scalar("INSERT INTO v1_suppression_entries (tenant_id,email,canonical_email,reason,source,notes,reason_code,reason_detail,source_type,source_ref,created_by,expires_at,metadata) SELECT tenant_id,email,canonical_email,reason,source,notes,reason_code,reason_detail,source_type,source_ref,$3,expires_at,metadata FROM v1_suppression_entries WHERE id=$1 AND tenant_id=$2 ON CONFLICT (tenant_id,canonical_email) WHERE status='active' DO UPDATE SET expires_at=CASE WHEN v1_suppression_entries.expires_at IS NULL OR EXCLUDED.expires_at IS NULL THEN NULL ELSE GREATEST(v1_suppression_entries.expires_at,EXCLUDED.expires_at) END, updated_at=NOW() RETURNING id")
        .bind(id).bind(tenant).bind(ctx.actor_id()).fetch_one(&mut *tx).await.map_err(ReacherResponseError::from)?;
	sqlx::query("UPDATE v1_suppression_entries SET status='merged', metadata=metadata || jsonb_build_object('restored_as',$3::integer),updated_at=NOW() WHERE id=$1 AND tenant_id=$2")
        .bind(id).bind(tenant).bind(restored).execute(&mut *tx).await.map_err(ReacherResponseError::from)?;
	sqlx::query("INSERT INTO v1_suppression_events (tenant_id,entry_id,canonical_email,event_type,from_status,to_status,actor_type,actor_id,metadata) VALUES ($1,$2,$3,'restored','inactive','active',$4,$5,jsonb_build_object('previous_entry_id',$6::integer))")
        .bind(tenant).bind(restored).bind(key).bind(ctx.actor_type()).bind(ctx.actor_id()).bind(id).execute(&mut *tx).await.map_err(ReacherResponseError::from)?;
	tx.commit().await.map_err(ReacherResponseError::from)?;
	Ok(warp::reply::json(
		&serde_json::json!({"id":restored,"status":"active"}),
	))
}
#[utoipa::path(post,path="/v1/suppressions/{id}/restore",tag="v1",params(("id"=i32,Path)),responses((status=200,description="Suppression restored")))]
pub fn v1_restore_suppression(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "suppressions" / i32 / "restore")
		.and(warp::post())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and_then(restore_handler)
}
