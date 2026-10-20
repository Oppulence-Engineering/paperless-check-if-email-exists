pub mod csv_shared;
pub mod deprecation;
mod error;
mod health;
pub mod idempotency;
pub mod openapi;
pub mod routes;
pub mod shared;
mod v0;
pub mod v1;
mod version;

use crate::config::BackendConfig;
use crate::tenant::auth::resolve_from_api_key;
use crate::tenant::context::{scope, TenantContext};
use crate::tenant::jwt::{resolve_from_jwt, JwtAuthError};
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
	routes::build_all_routes(config).recover(handle_rejection)
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
		crate::worker::dispatch::spawn_publisher(Arc::clone(&config), pool.clone());

		if config.delayed_recheck.enable && config.worker.enable {
			crate::delayed_recheck::spawn_delayed_recheck_scheduler(
				Arc::clone(&config),
				pool.clone(),
			);
			crate::delayed_recheck::spawn_delayed_recheck_cleanup(
				Arc::clone(&config),
				pool.clone(),
			);
		} else if config.delayed_recheck.enable {
			tracing::warn!(
				target: check_if_email_exists::LOG_TARGET,
				"Delayed recheck is enabled but worker mode is disabled. Scheduler will not start."
			);
		} else if config.worker.enable {
			let config = Arc::clone(&config);
			let pool = pool.clone();
			tokio::spawn(async move {
				match crate::delayed_recheck::release_pending_rechecks(&config, &pool).await {
					Ok(0) => {}
					Ok(released) => tracing::info!(
						target: check_if_email_exists::LOG_TARGET,
						released,
						"Delayed recheck is disabled; released tasks that were waiting on one"
					),
					Err(error) => tracing::warn!(
						target: check_if_email_exists::LOG_TARGET,
						error = ?error,
						"Failed to release pending delayed rechecks"
					),
				}
			});
		}

		if config.reverification.enable && config.worker.enable {
			crate::reverification::spawn_reverification_scheduler(Arc::clone(&config), pool);
		} else if config.reverification.enable {
			tracing::warn!(
				target: check_if_email_exists::LOG_TARGET,
				"Reverification is enabled but worker mode is disabled. Scheduler will not start."
			);
		}
		if config.pipelines.enable && config.worker.enable {
			crate::pipelines::spawn_pipeline_scheduler(
				Arc::clone(&config),
				config.get_pg_pool().expect("pg pool exists"),
			);
		} else if config.pipelines.enable {
			tracing::warn!(
				target: check_if_email_exists::LOG_TARGET,
				"Pipelines are enabled but worker mode is disabled. Scheduler will not start."
			);
		}
	} else {
		if config.reverification.enable {
			tracing::error!(
				target: check_if_email_exists::LOG_TARGET,
				"Reverification is enabled but no Postgres pool is configured. Reverification will not run."
			);
		}
		if config.pipelines.enable {
			tracing::error!(
				target: check_if_email_exists::LOG_TARGET,
				"Pipelines are enabled but no Postgres pool is configured. Pipeline scheduler will not run."
			);
		}
	}

	// Run v0 bulk job listener.
	let is_bulk_enabled = env::var("RCH_ENABLE_BULK").unwrap_or_else(|_| "0".into()) == "1";
	let runner = if is_bulk_enabled {
		let pg_pool = config.get_pg_pool().expect(
			"Please set the RCH__STORAGE__POSTGRES__DB_URL environment when RCH_ENABLE_BULK is set",
		);
		let runner = v0::bulk::create_job_registry(&pg_pool, Arc::clone(&config)).await?;
		Some(runner)
	} else {
		None
	};

	info!(target: LOG_TARGET, host=?host,port=?port, "Server is listening");
	warp::serve(routes).run((host, port)).await;

	Ok(runner)
}

/// Deserializes a PATCH field that may be absent, `null` or a value.
///
/// With `#[serde(default, deserialize_with = "crate::http::present_nullable")]`
/// on an `Option<Option<T>>`, an absent field stays `None` (leave unchanged)
/// and `null` becomes `Some(None)` (clear the column). Plain serde maps both
/// to `None`, so a client could never clear a nullable column.
pub(crate) fn present_nullable<'de, D, T>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
	D: serde::Deserializer<'de>,
	T: serde::Deserialize<'de>,
{
	<Option<T> as serde::Deserialize>::deserialize(deserializer).map(Some)
}

/// Check that the tenant context has the required scope.
/// Returns 403 Forbidden if the scope is missing.
pub fn check_scope(ctx: &TenantContext, scope: &str) -> Result<(), warp::Rejection> {
	if ctx.has_scope(scope) {
		Ok(())
	} else {
		Err(warp::reject::custom(ReacherResponseError::new(
			StatusCode::FORBIDDEN,
			format!("API key lacks required scope: {}", scope),
		)))
	}
}

fn required_route_scope(path: &str) -> Option<&'static str> {
	// v0 check_email runs the same verification as v1 and accepts API keys.
	if path == "/v0/check_email" {
		return Some(scope::VERIFY);
	}
	let route = path.strip_prefix("/v1/")?;
	match route.split('/').next()? {
		"check_email" | "operations" | "emails" => Some(scope::VERIFY),
		"bulk" | "jobs" | "events" | "query" | "sources" => Some(scope::BULK),
		"find_email" | "research" => Some(scope::FIND),
		"lists" | "segments" => Some(scope::LISTS),
		"alerts" => Some(scope::VERIFY),
		"suppressions" | "outcomes" => Some(scope::SUPPRESSIONS),
		"reputation" => Some(scope::REPUTATION),
		"provider-endpoints" | "reverification" | "score-policies" | "outcome-policies" => {
			Some(scope::SETTINGS)
		}
		// Campaign outcome read and write scopes differ by method and are
		// enforced in the handlers.
		"me" => match route.split('/').nth(1) {
			Some("settings" | "domains" | "webhook") => Some(scope::SETTINGS),
			Some("api-keys") => Some(scope::ADMIN),
			_ => None,
		},
		// Pipelines and comments choose scopes by method and resource in their handlers.
		_ => None,
	}
}

fn check_route_scope(ctx: &TenantContext, path: &str) -> Result<(), warp::Rejection> {
	if let Some(scope) = required_route_scope(path) {
		check_scope(ctx, scope)?;
	}
	Ok(())
}

/// The header which holds the Reacher backend secret.
pub const REACHER_SECRET_HEADER: &str = "x-reacher-secret";

const BEARER_PREFIX: &str = "Bearer ";
const API_KEY_PREFIX: &str = "rch_live_";

/// Warp filter that resolves a TenantContext from the request.
///
/// Resolution order:
/// 1. `Authorization: Bearer rch_live_...` → resolve via API key lookup
/// 2. `Authorization: Bearer <Better Auth JWT>` → verify signature, tenant and role
/// 3. `x-reacher-secret` header → validate against config → legacy context
/// 4. No auth headers, no `header_secret` and no JWT `auth` configured → legacy context (open mode)
/// 5. Otherwise → 401 Unauthorized
pub fn resolve_tenant(config: Arc<BackendConfig>) -> warp::filters::BoxedFilter<(TenantContext,)> {
	let config_clone = Arc::clone(&config);
	warp::any()
		.and(warp::method())
		.and(warp::path::full())
		.and(warp::header::optional::<String>("authorization"))
		.and(warp::header::optional::<String>(REACHER_SECRET_HEADER))
		.and_then(move |method: warp::http::Method, path: warp::path::FullPath, auth_header: Option<String>, secret_header: Option<String>| {
			let config = Arc::clone(&config_clone);
			async move {
				// Authentication headers always take precedence over open/legacy mode.
				if let Some(auth) = &auth_header {
					if let Some(token) = auth.strip_prefix(BEARER_PREFIX) {
						if token.starts_with(API_KEY_PREFIX) {
							if let Some(pool) = config.get_pg_pool() {
								match resolve_from_api_key(&pool, token, &config.throttle).await {
									Ok(ctx) => {
										if !ctx.plan_tier.includes_api() {
											return Err(warp::reject::custom(ReacherResponseError::new(
												StatusCode::FORBIDDEN,
												crate::tenant::models::API_PLAN_REQUIRED,
											)));
										}
										check_route_scope(&ctx, path.as_str())?;
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
						if let (Some(auth_config), Some(pool)) = (&config.auth, config.get_pg_pool()) {
								let ctx = resolve_from_jwt(&pool, token, auth_config, &config.throttle, method.as_str(), path.as_str())
									.await
									.map_err(|error| {
										let (status, message) = match error {
											JwtAuthError::Invalid(_) => (StatusCode::UNAUTHORIZED, "Invalid or unauthorized backend token"),
											JwtAuthError::Forbidden(_) => (StatusCode::FORBIDDEN, "This workspace role or account cannot perform this operation"),
											JwtAuthError::Unavailable(_) => (StatusCode::SERVICE_UNAVAILABLE, "Authentication is temporarily unavailable"),
										};
										debug!(target: LOG_TARGET, error = %error, "Backend token refused");
										warp::reject::custom(ReacherResponseError::new(status, message))
									})?;
								check_route_scope(&ctx, path.as_str())?;
								return Ok(ctx);
						}
					}
					return Err(warp::reject::custom(ReacherResponseError::new(
						StatusCode::UNAUTHORIZED,
						"Invalid Authorization header",
					)));
				}

				// Path 2: Legacy x-reacher-secret header
				if let Some(secret) = &secret_header {
					if let Some(expected) = &config.header_secret {
						if !expected.is_empty()
							&& crate::tenant::webhook::constant_time_eq(
								secret.as_bytes(),
								expected.as_bytes(),
							) {
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

				// Path 3: open mode. The legacy context has no tenant id, so it
				// reads every tenant's rows. A backend that verifies workspace
				// tokens serves many tenants and must not answer anonymously.
				if config.auth.is_some() {
					return Err(warp::reject::custom(ReacherResponseError::new(
						StatusCode::UNAUTHORIZED,
						"Authentication required. Provide Authorization: Bearer <api_key> or x-reacher-secret header.",
					)));
				}
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
/// A missing or wrong secret is a 401, and the comparison takes the same time
/// for every wrong secret of the right length.
pub fn check_header(config: Arc<BackendConfig>) -> warp::filters::BoxedFilter<()> {
	let Some(secret) = config
		.header_secret
		.clone()
		.filter(|secret| !secret.is_empty())
	else {
		return warp::any().boxed();
	};
	let secret = Arc::new(secret);
	warp::header::optional::<String>(REACHER_SECRET_HEADER)
		.and_then(move |provided: Option<String>| {
			let secret = Arc::clone(&secret);
			async move {
				match provided {
					Some(provided)
						if crate::tenant::webhook::constant_time_eq(
							provided.as_bytes(),
							secret.as_bytes(),
						) =>
					{
						Ok(())
					}
					_ => Err(warp::reject::custom(ReacherResponseError::new(
						StatusCode::UNAUTHORIZED,
						"Missing or invalid x-reacher-secret header",
					))),
				}
			}
		})
		.untuple_one()
		.boxed()
}

/// Control-plane routes must not inherit the legacy open-mode behavior.
pub fn check_admin_header(config: Arc<BackendConfig>) -> warp::filters::BoxedFilter<()> {
	if !matches!(config.header_secret.as_deref(), Some(secret) if !secret.is_empty()) {
		return warp::any()
			.and_then(|| async {
				Err::<(), _>(warp::reject::custom(ReacherResponseError::new(
					StatusCode::SERVICE_UNAVAILABLE,
					"Admin API requires a configured header secret",
				)))
			})
			.untuple_one()
			.boxed();
	}
	check_header(config)
}

#[cfg(test)]
mod tests {
	use super::required_route_scope;
	use crate::config::BackendConfig;
	use std::sync::Arc;

	#[tokio::test]
	async fn admin_header_fails_closed_without_secret() {
		let filter = super::check_admin_header(Arc::new(BackendConfig::empty()));
		assert!(warp::test::request()
			.header(super::REACHER_SECRET_HEADER, "anything")
			.filter(&filter)
			.await
			.is_err());

		let mut config = BackendConfig::empty();
		config.header_secret = Some("configured-secret".into());
		let filter = super::check_admin_header(Arc::new(config));
		assert!(warp::test::request().filter(&filter).await.is_err());
		assert!(warp::test::request()
			.header(super::REACHER_SECRET_HEADER, "configured-secret")
			.filter(&filter)
			.await
			.is_ok());
	}

	#[tokio::test]
	async fn missing_credentials_are_refused_when_workspace_tokens_are_configured() {
		let rejected_status = |config: BackendConfig| async move {
			warp::test::request()
				.path("/v1/jobs/1")
				.filter(&super::resolve_tenant(Arc::new(config)))
				.await
				.err()
				.and_then(|rejection| {
					rejection
						.find::<super::ReacherResponseError>()
						.map(|error| error.code)
				})
		};

		// A single-user backend with no credentials configured stays open.
		assert_eq!(rejected_status(BackendConfig::empty()).await, None);

		// Workspace tokens mean many tenants share this backend, and the open
		// context has no tenant id, so it would read every tenant's rows.
		let mut config = BackendConfig::empty();
		config.auth = Some(crate::config::JwtAuthConfig {
			jwks_url: "http://localhost/jwks".into(),
			issuer: "http://localhost".into(),
			audience: "check-if-email-exists-api".into(),
		});
		assert_eq!(
			rejected_status(config).await,
			Some(warp::http::StatusCode::UNAUTHORIZED)
		);
	}

	#[test]
	fn every_tenant_api_group_has_a_scope() {
		let spec = super::openapi::build_spec().expect("OpenAPI spec");
		for path in spec["paths"].as_object().expect("OpenAPI paths").keys() {
			// v0 bulk only accepts the backend secret; v0 check_email accepts API keys.
			if path.starts_with("/v0/") && path != "/v0/check_email" {
				continue;
			}
			if !(path.starts_with("/v1/") || path.starts_with("/v0/"))
				|| path.starts_with("/v1/admin/")
				|| path.starts_with("/v1/inbound/")
				|| path.starts_with("/v1/pipelines")
				|| path.starts_with("/v1/comments")
				|| path.starts_with("/v1/campaign-outcomes")
				|| matches!(
					path.as_str(),
					"/v1/check-email-with-onboard" | "/v1/me" | "/v1/me/usage"
				) {
				continue;
			}
			assert!(
				required_route_scope(path).is_some(),
				"missing scope: {}",
				path
			);
		}
	}
}
