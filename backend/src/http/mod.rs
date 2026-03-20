// Reacher - Email Verification
// Copyright (C) 2018-2023 Reacher

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

pub mod csv_shared;
pub mod deprecation;
mod error;
mod health;
pub mod idempotency;
pub mod openapi;
pub mod shared;
mod v0;
pub mod v1;
mod version;

use crate::config::BackendConfig;
use crate::tenant::auth::resolve_from_api_key;
use crate::tenant::context::TenantContext;
use check_if_email_exists::LOG_TARGET;
use error::handle_rejection;
pub use error::ReacherResponseError;
use sqlxmq::JobRunnerHandle;
use std::env;
use std::net::IpAddr;
use std::sync::Arc;
use tracing::{debug, info};
pub use v0::check_email::post::CheckEmailRequest;
use warp::http::StatusCode;
use warp::Filter;

pub fn create_routes(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	let pg_pool = config.get_pg_pool();

	let health_routes = health::liveness::healthz()
		.boxed()
		.or(health::readiness::readyz(Arc::clone(&config)).boxed());
	let health_routes = health_routes
		.or(version::get::get_version().boxed())
		.boxed();
	let openapi_route = openapi::openapi_spec().boxed();

	let v0_routes = v0::check_email::post::post_check_email(Arc::clone(&config))
		.boxed()
		.or(v0::bulk::post::create_bulk_job(Arc::clone(&config), pg_pool.clone()).boxed())
		.or(v0::bulk::get::get_bulk_job_status(Arc::clone(&config), pg_pool.clone()).boxed())
		.or(v0::bulk::results::get_bulk_job_result(Arc::clone(&config), pg_pool).boxed())
		.boxed();

	let v1_routes = v1::onboard::v1_check_email_with_onboard(Arc::clone(&config))
		.boxed()
		.or(v1::check_email::post::v1_check_email(Arc::clone(&config)).boxed())
		.or(v1::find_email::post::v1_find_email(Arc::clone(&config)).boxed())
		.or(v1::find_email::get::v1_get_find_email(Arc::clone(&config)).boxed())
		.or(v1::lists::post::v1_create_list(Arc::clone(&config)).boxed())
		.or(v1::lists::get_list::v1_list_lists(Arc::clone(&config)).boxed())
		.or(v1::lists::get_detail::v1_get_list(Arc::clone(&config)).boxed())
		.or(v1::lists::download::v1_download_list(Arc::clone(&config)).boxed())
		.or(v1::lists::delete::v1_delete_list(Arc::clone(&config)).boxed())
		.or(v1::reputation::check::v1_check_reputation(Arc::clone(&config)).boxed())
		.or(v1::suppressions::add::v1_add_suppressions(Arc::clone(&config)).boxed())
		.or(v1::suppressions::check::v1_check_suppression(Arc::clone(&config)).boxed())
		.or(v1::suppressions::list::v1_list_suppressions(Arc::clone(&config)).boxed())
		.or(v1::suppressions::delete::v1_delete_suppression(Arc::clone(&config)).boxed())
		.or(v1::bulk::post::v1_create_bulk_job(Arc::clone(&config)).boxed())
		.or(v1::bulk::get_progress::v1_get_bulk_job_progress(Arc::clone(&config)).boxed())
		.or(v1::bulk::get_results::v1_get_bulk_job_results(Arc::clone(&config)).boxed())
		.or(v1::reverification::status::v1_reverification_status(Arc::clone(&config)).boxed())
		.boxed();

	let v1_job_routes = v1::jobs::get_status::v1_get_job_status(Arc::clone(&config))
		.boxed()
		.or(v1::jobs::cancel::v1_cancel_job(Arc::clone(&config)).boxed())
		.or(v1::jobs::get_events::v1_get_job_events(Arc::clone(&config)).boxed())
		.or(v1::jobs::get_results::v1_get_job_results(Arc::clone(&config)).boxed())
		.or(v1::jobs::download::v1_download_job_results(Arc::clone(&config)).boxed())
		.or(v1::jobs::retry::v1_retry_job(Arc::clone(&config)).boxed())
		.boxed();

	let v1_me_routes = v1::me::v1_me(Arc::clone(&config)).boxed();

	let v1_tenant_routes = v1::tenant_settings::v1_get_tenant_settings(Arc::clone(&config))
		.boxed()
		.or(v1::tenant_settings::v1_update_tenant_settings(Arc::clone(&config)).boxed())
		.or(v1::tenant_settings::v1_get_tenant_webhook(Arc::clone(&config)).boxed())
		.or(v1::tenant_settings::v1_update_tenant_webhook(Arc::clone(&config)).boxed())
		.or(v1::tenant_settings::v1_clear_tenant_webhook(Arc::clone(&config)).boxed())
		.or(v1::tenant_settings::v1_get_tenant_usage(Arc::clone(&config)).boxed())
		.or(v1::tenant_domains::v1_list_tenant_domains(Arc::clone(&config)).boxed())
		.or(v1::tenant_domains::v1_create_tenant_domain(Arc::clone(&config)).boxed())
		.or(v1::tenant_domains::v1_get_tenant_domain(Arc::clone(&config)).boxed())
		.or(v1::tenant_domains::v1_update_tenant_domain(Arc::clone(&config)).boxed())
		.or(v1::tenant_domains::v1_delete_tenant_domain(Arc::clone(&config)).boxed())
		.boxed();

	let v1_account_routes = v1::account_api_keys::list_api_keys(Arc::clone(&config))
		.boxed()
		.or(v1::account_api_keys::get_api_key(Arc::clone(&config)).boxed())
		.or(v1::account_api_keys::create_api_key(Arc::clone(&config)).boxed())
		.or(v1::account_api_keys::update_api_key(Arc::clone(&config)).boxed())
		.or(v1::account_api_keys::revoke_api_key(Arc::clone(&config)).boxed())
		.boxed();

	let v1_admin_routes = v1::admin::tenants::create_tenant(Arc::clone(&config))
		.boxed()
		.or(v1::admin::tenants::list_tenants(Arc::clone(&config)).boxed())
		.or(v1::admin::tenants::get_tenant(Arc::clone(&config)).boxed())
		.or(v1::admin::tenants::update_tenant(Arc::clone(&config)).boxed())
		.or(v1::admin::tenants::delete_tenant(Arc::clone(&config)).boxed())
		.or(v1::admin::quota::get_tenant_quota(Arc::clone(&config)).boxed())
		.or(v1::admin::quota::update_tenant_quota(Arc::clone(&config)).boxed())
		.or(v1::admin::quota::reset_tenant_quota(Arc::clone(&config)).boxed())
		.or(v1::admin::jobs::list_jobs(Arc::clone(&config)).boxed())
		.or(v1::admin::jobs::get_job(Arc::clone(&config)).boxed())
		.or(v1::admin::jobs::get_job_events(Arc::clone(&config)).boxed())
		.or(v1::admin::jobs::get_job_results(Arc::clone(&config)).boxed())
		.or(v1::admin::jobs::list_tenant_jobs(Arc::clone(&config)).boxed())
		.or(v1::admin::api_keys::list_all_api_keys(Arc::clone(&config)).boxed())
		.or(v1::admin::api_keys::create_api_key(Arc::clone(&config)).boxed())
		.or(v1::admin::api_keys::list_api_keys(Arc::clone(&config)).boxed())
		.or(v1::admin::api_keys::get_api_key(Arc::clone(&config)).boxed())
		.or(v1::admin::api_keys::update_api_key(Arc::clone(&config)).boxed())
		.or(v1::admin::api_keys::revoke_api_key(Arc::clone(&config)).boxed())
		.or(v1::admin::api_keys::reactivate_api_key(config).boxed())
		.boxed();

	health_routes
		.or(openapi_route)
		.or(v0_routes)
		.or(v1_routes)
		.or(v1_job_routes)
		.or(v1_me_routes)
		.or(v1_tenant_routes)
		.or(v1_account_routes)
		.or(v1_admin_routes)
		.recover(handle_rejection)
}

/// Runs the Warp server.
pub async fn run_warp_server(
	config: Arc<BackendConfig>,
) -> Result<Option<JobRunnerHandle>, anyhow::Error> {
	let host = config
		.http_host
		.parse::<IpAddr>()
		.unwrap_or_else(|_| panic!("Invalid host: {}", config.http_host));
	let port = env::var("PORT")
		.map(|port: String| {
			port.parse::<u16>()
				.unwrap_or_else(|_| panic!("Invalid port: {}", port))
		})
		.unwrap_or(config.http_port);

	let routes = create_routes(Arc::clone(&config));

	// Spawn idempotency key cleanup if Postgres is configured
	if let Some(pool) = config.get_pg_pool() {
		idempotency::spawn_idempotency_cleanup(pool.clone());
		crate::reputation::spawn_cache_cleanup(pool.clone());

		if config.reverification.enable && config.worker.enable {
			crate::reverification::spawn_reverification_scheduler(Arc::clone(&config), pool);
		} else if config.reverification.enable {
			tracing::warn!(
				target: check_if_email_exists::LOG_TARGET,
				"Reverification is enabled but worker mode is disabled. Scheduler will not start."
			);
		}
	} else if config.reverification.enable {
		tracing::error!(
			target: check_if_email_exists::LOG_TARGET,
			"Reverification is enabled but no Postgres pool is configured. Reverification will not run."
		);
	}

	// Run v0 bulk job listener.
	let is_bulk_enabled = env::var("RCH_ENABLE_BULK").unwrap_or_else(|_| "0".into()) == "1";
	let runner = if is_bulk_enabled {
		let pg_pool = config.get_pg_pool().expect(
			"Please set the RCH__STORAGE__POSTGRES__DB_URL environment when RCH_ENABLE_BULK is set",
		);
		let runner = v0::bulk::create_job_registry(&pg_pool).await?;
		Some(runner)
	} else {
		None
	};

	info!(target: LOG_TARGET, host=?host,port=?port, "Server is listening");
	warp::serve(routes).run((host, port)).await;

	Ok(runner)
}

/// The header which holds the Reacher backend secret.
pub const REACHER_SECRET_HEADER: &str = "x-reacher-secret";

const BEARER_PREFIX: &str = "Bearer ";
const API_KEY_PREFIX: &str = "rch_live_";

/// Warp filter that resolves a TenantContext from the request.
///
/// Resolution order:
/// 1. `Authorization: Bearer rch_live_...` → resolve via API key lookup
/// 2. `x-reacher-secret` header → validate against config → legacy context
/// 3. No auth headers + no `header_secret` configured → legacy context (open mode)
/// 4. Otherwise → 401 Unauthorized
pub fn resolve_tenant(config: Arc<BackendConfig>) -> warp::filters::BoxedFilter<(TenantContext,)> {
	let config_clone = Arc::clone(&config);
	warp::any()
		.and(warp::header::optional::<String>("authorization"))
		.and(warp::header::optional::<String>(REACHER_SECRET_HEADER))
		.and_then(move |auth_header: Option<String>, secret_header: Option<String>| {
			let config = Arc::clone(&config_clone);
			async move {
				// Path 1: Bearer token with rch_live_ prefix
				if let Some(auth) = &auth_header {
					if let Some(token) = auth.strip_prefix(BEARER_PREFIX) {
						if token.starts_with(API_KEY_PREFIX) {
							if let Some(pool) = config.get_pg_pool() {
								match resolve_from_api_key(&pool, token, &config.throttle).await {
									Ok(ctx) => {
										debug!(target: LOG_TARGET, tenant=?ctx.tenant_name, "Resolved tenant from API key");
										return Ok(ctx);
									}
									Err(e) => {
										return Err(warp::reject::custom(
											ReacherResponseError::new(
												StatusCode::UNAUTHORIZED,
												format!("API key authentication failed: {}", e),
											),
										));
									}
								}
							} else {
								return Err(warp::reject::custom(ReacherResponseError::new(
									StatusCode::SERVICE_UNAVAILABLE,
									"API key authentication requires a Postgres database",
								)));
							}
						}
					}
				}

				// Path 2: Legacy x-reacher-secret header
				if let Some(secret) = &secret_header {
					if let Some(expected) = &config.header_secret {
						if !expected.is_empty() && secret == expected {
							return Ok(TenantContext::legacy(config.throttle.clone()));
						} else if !expected.is_empty() {
							return Err(warp::reject::custom(ReacherResponseError::new(
								StatusCode::UNAUTHORIZED,
								"Invalid x-reacher-secret header",
							)));
						}
					}
					// If header_secret is not configured, fall through
				}

				// Path 3: No auth + no secret configured → open mode
				match &config.header_secret {
					None => Ok(TenantContext::legacy(config.throttle.clone())),
					Some(s) if s.is_empty() => {
						Ok(TenantContext::legacy(config.throttle.clone()))
					}
					Some(_) => {
						// Secret is configured but no valid auth provided
						if auth_header.is_some() || secret_header.is_some() {
							// They tried to auth but failed
							Err(warp::reject::custom(ReacherResponseError::new(
								StatusCode::UNAUTHORIZED,
								"Invalid authentication credentials",
							)))
						} else {
							// No auth headers at all
							Err(warp::reject::custom(ReacherResponseError::new(
								StatusCode::UNAUTHORIZED,
								"Authentication required. Provide Authorization: Bearer <api_key> or x-reacher-secret header.",
							)))
						}
					}
				}
			}
		})
		.boxed()
}

/// Legacy check_header for v0 bulk routes that don't use TenantContext.
/// Kept for backward compatibility with v0 bulk which passes through to sqlxmq.
pub fn check_header(config: Arc<BackendConfig>) -> warp::filters::BoxedFilter<()> {
	if let Some(secret) = config.header_secret.clone() {
		if secret.is_empty() {
			return warp::any().boxed();
		}

		let secret: &'static str = Box::leak(Box::new(secret));

		warp::header::exact(REACHER_SECRET_HEADER, secret).boxed()
	} else {
		warp::any().boxed()
	}
}
