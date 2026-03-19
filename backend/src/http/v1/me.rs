use crate::config::BackendConfig;
use crate::http::resolve_tenant;
use crate::tenant::context::TenantContext;
use serde::Serialize;
use std::sync::Arc;
use warp::Filter;

#[derive(Debug, Serialize)]
struct MeResponse {
	pub tenant_id: Option<String>,
	pub tenant_name: String,
	pub plan_tier: String,
	pub status: String,
	pub is_legacy: bool,
	pub monthly_email_limit: Option<i32>,
	pub used_this_period: i32,
	pub period_reset_at: String,
	pub quota_unlimited: bool,
	pub quota_remaining: Option<i32>,
	pub default_webhook_url: Option<String>,
	pub result_retention_days: i32,
}

async fn http_handler(tenant_ctx: TenantContext) -> Result<impl warp::Reply, warp::Rejection> {
	let limit = tenant_ctx.monthly_email_limit;
	let quota_unlimited = limit.is_none() || limit.unwrap_or(0) <= 0;
	let quota_remaining = match limit {
		Some(lim) if lim > 0 => Some((lim - tenant_ctx.used_this_period).max(0)),
		_ => None,
	};

	let response = MeResponse {
		tenant_id: tenant_ctx.tenant_id.map(|id| id.to_string()),
		tenant_name: tenant_ctx.tenant_name,
		plan_tier: format!("{:?}", tenant_ctx.plan_tier).to_lowercase(),
		status: format!("{:?}", tenant_ctx.status).to_lowercase(),
		is_legacy: tenant_ctx.is_legacy,
		monthly_email_limit: limit,
		used_this_period: tenant_ctx.used_this_period,
		period_reset_at: tenant_ctx.period_reset_at.to_rfc3339(),
		quota_unlimited,
		quota_remaining,
		default_webhook_url: tenant_ctx.default_webhook_url,
		result_retention_days: tenant_ctx.result_retention_days,
	};

	Ok(warp::reply::json(&response))
}

/// GET /v1/me
///
/// Returns the authenticated tenant context and quota metadata.
#[utoipa::path(
	get,
	path = "/v1/me",
	tag = "Account",
	responses((status = 200, description = "Current tenant profile"))
)]
pub fn v1_me(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "me")
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and_then(http_handler)
		.with(warp::log("reacher_backend::v1::me"))
}
