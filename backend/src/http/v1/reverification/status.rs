use crate::config::BackendConfig;
use crate::finder::require_tenant_id;
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
	enabled: bool,
	#[serde(skip_serializing_if = "Option::is_none")]
	staleness_days: Option<i32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	batch_size: Option<i32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	last_run_at: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	next_run_at: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	last_job_id: Option<i32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	emails_requeued: Option<i32>,
}

async fn http_handler(
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;

	let row = sqlx::query(
		r#"
		SELECT enabled, staleness_days, batch_size,
			   TO_CHAR(last_run_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS last_run_at,
			   TO_CHAR(next_run_at AT TIME ZONE 'UTC', 'YYYY-MM-DD"T"HH24:MI:SS"Z"') AS next_run_at,
			   last_job_id, emails_requeued
		FROM reverification_schedules
		WHERE tenant_id = $1
		"#,
	)
	.bind(tenant_id)
	.fetch_optional(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	match row {
		Some(row) => Ok(warp::reply::json(&Response {
			enabled: row.get("enabled"),
			staleness_days: Some(row.get("staleness_days")),
			batch_size: Some(row.get("batch_size")),
			last_run_at: row.get("last_run_at"),
			next_run_at: row.get("next_run_at"),
			last_job_id: row.get("last_job_id"),
			emails_requeued: Some(row.get("emails_requeued")),
		})),
		None => Ok(warp::reply::json(&Response {
			enabled: false,
			staleness_days: None,
			batch_size: None,
			last_run_at: None,
			next_run_at: None,
			last_job_id: None,
			emails_requeued: None,
		})),
	}
}

/// Pool-only filter that doesn't require worker mode.
fn with_pg_pool(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (PgPool,), Error = warp::Rejection> + Clone {
	warp::any().and_then(move || {
		let config = Arc::clone(&config);
		async move {
			config.get_pg_pool().ok_or_else(|| {
				warp::reject::custom(ReacherResponseError::new(
					StatusCode::SERVICE_UNAVAILABLE,
					"Database not configured",
				))
			})
		}
	})
}

/// GET /v1/reverification/status
#[utoipa::path(
	get,
	path = "/v1/reverification/status",
	tag = "v1",
	responses((status = 200, description = "Reverification schedule status"))
)]
pub fn v1_reverification_status(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "reverification" / "status")
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_pg_pool(config))
		.and_then(http_handler)
		.with(warp::log(LOG_TARGET))
}
