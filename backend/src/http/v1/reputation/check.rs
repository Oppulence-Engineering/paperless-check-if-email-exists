use crate::config::BackendConfig;
use crate::http::resolve_tenant;
use crate::http::v0::check_email::post::with_config;
use crate::reputation::checker::check_domain;
use crate::reputation::models::ReputationCheckRequest;
use check_if_email_exists::LOG_TARGET;
use std::sync::Arc;
use warp::Filter;

async fn http_handler(
	_config: Arc<BackendConfig>,
	request: ReputationCheckRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
	let pool = _config.get_pg_pool();
	let response = check_domain(pool.as_ref(), &request.domain, request.force_refresh)
		.await
		.map_err(warp::reject::custom)?;
	Ok(warp::reply::json(&response))
}

/// POST /v1/reputation/check
#[utoipa::path(
	post,
	path = "/v1/reputation/check",
	tag = "v1",
	responses((status = 200, description = "Reputation check response"))
)]
pub fn v1_check_reputation(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "reputation" / "check")
		.and(warp::post())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_config(config))
		.and(warp::body::json::<ReputationCheckRequest>())
		.and_then(|_tenant_ctx, config, request| http_handler(config, request))
		.with(warp::log(LOG_TARGET))
}
