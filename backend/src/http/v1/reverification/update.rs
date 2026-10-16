use crate::config::BackendConfig;
use crate::finder::require_tenant_id;
use crate::http::{check_scope, resolve_tenant, ReacherResponseError};
use crate::tenant::context::{scope, TenantContext};
use check_if_email_exists::LOG_TARGET;
use serde::Deserialize;
use sqlx::PgPool;
use std::sync::Arc;
use warp::http::StatusCode;
use warp::Filter;

use super::status::{http_handler as status_handler, with_pg_pool};

fn default_batch_size() -> i32 {
	100
}

/// Turns scheduled re-checks on or off for the workspace and sets how old a
/// result may get before it is checked again.
#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct UpdateReverificationRequest {
	pub enabled: bool,
	/// Re-check results older than this many days (1 to 365).
	pub staleness_days: i32,
	/// Addresses re-checked in each run (1 to 1,000).
	#[serde(default = "default_batch_size")]
	pub batch_size: i32,
}

fn validate(body: &UpdateReverificationRequest) -> Result<(), ReacherResponseError> {
	if !(1..=365).contains(&body.staleness_days) {
		return Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"staleness_days must be between 1 and 365",
		));
	}
	if !(1..=1_000).contains(&body.batch_size) {
		return Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"batch_size must be between 1 and 1000",
		));
	}
	Ok(())
}

async fn http_handler(
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	body: UpdateReverificationRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::SETTINGS)?;
	validate(&body)?;
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	// A new schedule is due now, so the next scheduler cycle picks it up.
	sqlx::query(
		"INSERT INTO reverification_schedules (tenant_id, enabled, staleness_days, batch_size, next_run_at) \
		 VALUES ($1, $2, $3, $4, NOW()) \
		 ON CONFLICT (tenant_id) DO UPDATE SET enabled = EXCLUDED.enabled, \
		 staleness_days = EXCLUDED.staleness_days, batch_size = EXCLUDED.batch_size, updated_at = NOW()",
	)
	.bind(tenant_id)
	.bind(body.enabled)
	.bind(body.staleness_days)
	.bind(body.batch_size)
	.execute(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;
	status_handler(tenant_ctx, pg_pool).await
}

/// PUT /v1/reverification
#[utoipa::path(
	put,
	path = "/v1/reverification",
	tag = "v1",
	request_body = UpdateReverificationRequest,
	responses(
		(status = 200, description = "Reverification schedule status"),
		(status = 400, description = "Invalid schedule"),
	)
)]
pub fn v1_update_reverification(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "reverification")
		.and(warp::put())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_pg_pool(config))
		.and(warp::body::content_length_limit(4 * 1024))
		.and(warp::body::json())
		.and_then(http_handler)
		.with(warp::log(LOG_TARGET))
}

#[cfg(test)]
mod tests {
	use super::{validate, UpdateReverificationRequest};

	fn body(staleness_days: i32, batch_size: i32) -> UpdateReverificationRequest {
		UpdateReverificationRequest {
			enabled: true,
			staleness_days,
			batch_size,
		}
	}

	#[test]
	fn accepts_the_published_range_and_rejects_the_rest() {
		assert!(validate(&body(30, 100)).is_ok());
		assert!(validate(&body(365, 1_000)).is_ok());
		assert!(validate(&body(0, 100)).is_err());
		assert!(validate(&body(30, 1_001)).is_err());
	}
}
