use crate::decision::custom_policy::{resolve_requested_policy, DecisionPolicy};
use crate::http::idempotency::{
	check_idempotency_key, fail_idempotency_key, hash_request_body, IdempotencyCheck,
};
use check_if_email_exists::{check_email, LOG_TARGET};
use std::sync::Arc;
use tracing::{info, warn};
use warp::http::StatusCode;

use super::verification_operation::{
	admit_operation, read_operation, wait_for_operation, PendingVerification,
};
use crate::config::BackendConfig;
use crate::http::v0::check_email::post::CheckEmailRequest;
use crate::http::ReacherResponseError;
use crate::scoring::response::{
	inject_freshness_into_result_at, prepare_check_email_success,
	prepare_verification_response_with_policy,
};
use crate::tenant::context::TenantContext;
use crate::tenant::quota::{check_and_increment_quota, release_reserved_usage, QuotaCheckResult};
use crate::worker::do_work::{CheckEmailJobId, CheckEmailTask};
use chrono::{TimeZone, Utc};

#[derive(Debug)]
pub struct CheckEmailResponse {
	pub status_code: StatusCode,
	pub body: Vec<u8>,
}

async fn mark_idempotency_failed(pool: &sqlx::PgPool, record_id: i64) {
	if let Err(e) = fail_idempotency_key(pool, record_id).await {
		warn!(target: LOG_TARGET, record_id = ?record_id, error = ?e, "Failed to mark idempotency key as failed");
	}
}

/// Shared check_email handler used by both v0 and v1 endpoints.
/// Applies throttle checking, routes to worker or direct execution,
/// stores results, and returns the serialized response.
pub async fn handle_check_email(
	config: Arc<BackendConfig>,
	request_body: &[u8],
	body: &CheckEmailRequest,
	tenant_ctx: &TenantContext,
	request_path: &str,
	idempotency_key: Option<String>,
) -> Result<CheckEmailResponse, warp::Rejection> {
	if body.to_email.is_empty() {
		return Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"to_email field is required.",
		)
		.into());
	}
	body.validate_routing_overrides(&config)
		.map_err(|message| ReacherResponseError::new(StatusCode::BAD_REQUEST, message))?;

	// Sandbox mode: return deterministic mock results without SMTP,
	// throttle checks, or quota consumption. Freshness fields are
	// hardcoded so responses are fully deterministic across time.
	if body.sandbox {
		let pool = config.get_pg_pool();
		let decision_policy = resolve_requested_policy(
			pool.as_ref(),
			tenant_ctx.tenant_id,
			body.policy_mode,
			body.policy_profile_key.as_deref(),
		)
		.await?;
		let mock_result = crate::sandbox::sandbox_check(&body.to_email);
		let completed_at = Utc
			.with_ymd_and_hms(2025, 1, 1, 0, 0, 0)
			.single()
			.expect("static sandbox timestamp");
		let mut prepared = prepare_verification_response_with_policy(
			config.as_ref(),
			&mock_result,
			None,
			completed_at,
			false,
			decision_policy.as_ref(),
		)
		.await
		.map_err(ReacherResponseError::from)?;
		inject_freshness_into_result_at(&mut prepared.json, completed_at, completed_at);
		prepared.body = serde_json::to_vec(&prepared.json).map_err(ReacherResponseError::from)?;
		return Ok(CheckEmailResponse {
			status_code: StatusCode::OK,
			body: prepared.body,
		});
	}

	let mut idempotency_record_id: Option<i64> = None;
	let mut idempotency_pool: Option<sqlx::PgPool> = None;
	if config.get_pg_pool().is_none() && (config.worker.enable || idempotency_key.is_some()) {
		return Err(ReacherResponseError::new(
			StatusCode::SERVICE_UNAVAILABLE,
			"Durable worker execution and idempotency require Postgres; no verification was dispatched.",
		).into());
	}

	if let Some(idempotency_key) = idempotency_key {
		if let Some(pool) = config.get_pg_pool() {
			let tenant_id = tenant_ctx.tenant_id_str();
			let request_body_hash = hash_request_body(request_body);

			let idempotency_result = check_idempotency_key(
				&pool,
				&tenant_id,
				&idempotency_key,
				request_path,
				&request_body_hash,
				&tenant_ctx.tenant_name,
			)
			.await?;

			match idempotency_result {
				IdempotencyCheck::New { record_id } => {
					idempotency_record_id = Some(record_id);
					idempotency_pool = Some(pool);
				}
				IdempotencyCheck::InProgress => {
					return Err(ReacherResponseError::new(
						StatusCode::CONFLICT,
						"Idempotency key is already in use for this path",
					)
					.into());
				}
				IdempotencyCheck::BodyMismatch => {
					return Err(ReacherResponseError::new(
						StatusCode::BAD_REQUEST,
						"Idempotency key body mismatch",
					)
					.into());
				}
				IdempotencyCheck::PathMismatch => {
					return Err(ReacherResponseError::new(
						StatusCode::UNPROCESSABLE_ENTITY,
						"Idempotency key was already used on another endpoint",
					)
					.into());
				}
				IdempotencyCheck::Cached(cached) => {
					if cached.status_code == StatusCode::ACCEPTED.as_u16() {
						let pending: PendingVerification = serde_json::from_slice(&cached.body)
							.map_err(ReacherResponseError::from)?;
						return read_operation(&pool, pending.operation_id, tenant_ctx)
							.await
							.map_err(Into::into);
					}
					let cached_status = StatusCode::from_u16(cached.status_code).map_err(|e| {
						ReacherResponseError::new(
							StatusCode::INTERNAL_SERVER_ERROR,
							format!("Invalid cached status code: {e}"),
						)
					})?;
					return Ok(CheckEmailResponse {
						status_code: cached_status,
						body: cached.body,
					});
				}
			}
		}
	}

	let pg_pool = config.get_pg_pool();
	let decision_policy = match resolve_requested_policy(
		pg_pool.as_ref(),
		tenant_ctx.tenant_id,
		body.policy_mode,
		body.policy_profile_key.as_deref(),
	)
	.await
	{
		Ok(policy) => policy,
		Err(error) => {
			if let (Some(pool), Some(record_id)) = (idempotency_pool, idempotency_record_id) {
				mark_idempotency_failed(&pool, record_id).await;
			}
			return Err(error.into());
		}
	};
	let throttle = match (tenant_ctx.tenant_id, pg_pool.as_ref()) {
		(Some(tenant_id), Some(pool)) => {
			crate::throttle::reserve_tenant_rate(pool, tenant_id, &tenant_ctx.throttle).await
		}
		(Some(_), None)
			if [
				tenant_ctx.throttle.max_requests_per_second,
				tenant_ctx.throttle.max_requests_per_minute,
				tenant_ctx.throttle.max_requests_per_hour,
				tenant_ctx.throttle.max_requests_per_day,
			]
			.iter()
			.any(Option::is_some) =>
		{
			Err(sqlx::Error::PoolClosed)
		}
		(None, _) if !config.worker.enable => Ok(config.get_throttle_manager().try_acquire().await),
		_ => Ok(None),
	};
	let throttle = match throttle {
		Ok(result) => result,
		Err(error) => {
			warn!(target: LOG_TARGET, error=?error, "Tenant rate reservation unavailable");
			if let (Some(pool), Some(record_id)) = (idempotency_pool, idempotency_record_id) {
				mark_idempotency_failed(&pool, record_id).await;
			}
			return Err(ReacherResponseError::new(
				StatusCode::SERVICE_UNAVAILABLE,
				"Rate limit service unavailable; no verification was dispatched.",
			)
			.into());
		}
	};
	if let Some(throttle_result) = throttle {
		if let (Some(pool), Some(record_id)) = (idempotency_pool, idempotency_record_id) {
			mark_idempotency_failed(&pool, record_id).await;
		}
		return Err(ReacherResponseError::new(
			StatusCode::TOO_MANY_REQUESTS,
			format!(
				"Rate limit {} exceeded, please wait {:?}",
				throttle_result.limit_type, throttle_result.delay
			),
		)
		.into());
	}

	if let Some(pool) = pg_pool.as_ref() {
		let admitted = admit_operation(
			Arc::clone(&config),
			pool,
			body,
			tenant_ctx,
			decision_policy,
			idempotency_record_id,
		)
		.await;
		let (operation_id, dispatch_id) = match admitted {
			Ok(ids) => ids,
			Err(error) => {
				if let Some(id) = idempotency_record_id {
					mark_idempotency_failed(pool, id).await;
				}
				return Err(error.into());
			}
		};
		if !config.worker.enable {
			let pool = pool.clone();
			tokio::spawn(async move {
				if let Err(error) =
					crate::worker::dispatch::execute_local(&pool, config, dispatch_id).await
				{
					warn!(target: LOG_TARGET, operation_id = %operation_id, error = ?error, "Direct operation awaits durable recovery");
				}
			});
		}
		return wait_for_operation(pool, operation_id, tenant_ctx)
			.await
			.map_err(Into::into);
	}

	// Atomically check + increment monthly quota (avoids TOCTOU race)
	crate::smtp_egress::require_smtp_ready(&config)?;
	match check_and_increment_quota(pg_pool.as_ref(), tenant_ctx).await {
		QuotaCheckResult::Allowed => {} // Quota already incremented atomically
		QuotaCheckResult::Unavailable => {
			if let (Some(pool), Some(record_id)) = (idempotency_pool, idempotency_record_id) {
				mark_idempotency_failed(&pool, record_id).await;
			}
			return Err(ReacherResponseError::new(
				StatusCode::SERVICE_UNAVAILABLE,
				"Quota service unavailable; no verification was dispatched.",
			)
			.into());
		}
		QuotaCheckResult::ExceededMonthlyLimit {
			limit,
			used,
			resets_at,
		} => {
			if let (Some(pool), Some(record_id)) = (idempotency_pool, idempotency_record_id) {
				mark_idempotency_failed(&pool, record_id).await;
			}
			return Err(ReacherResponseError::new(
				StatusCode::TOO_MANY_REQUESTS,
				format!(
					"Monthly email limit of {} reached ({} used). Resets at {}",
					limit,
					used,
					resets_at.format("%Y-%m-%d %H:%M:%S UTC")
				),
			)
			.into());
		}
	}

	let body = handle_without_worker(config, body, tenant_ctx, decision_policy).await?;
	Ok(CheckEmailResponse {
		status_code: StatusCode::OK,
		body,
	})
}

async fn handle_without_worker(
	config: Arc<BackendConfig>,
	body: &CheckEmailRequest,
	tenant_ctx: &TenantContext,
	decision_policy: Option<DecisionPolicy>,
) -> Result<Vec<u8>, warp::Rejection> {
	info!(target: LOG_TARGET, email=body.to_email, "Starting verification");
	let input = body.to_check_email_input(Arc::clone(&config));
	let result = check_email(&input).await;
	// The caller gets an error on either failure below, so the check reserved
	// from the monthly quota is given back.
	let prepared = match prepare_check_email_success(
		config.as_ref(),
		result,
		tenant_ctx.tenant_id,
		Utc::now(),
		true,
		decision_policy.as_ref(),
	)
	.await
	{
		Ok(prepared) => prepared,
		Err(err) => {
			release_single_reserved_usage(&config, tenant_ctx).await;
			return Err(ReacherResponseError::from(err).into());
		}
	};

	let storage = Arc::clone(&config).get_storage_adapter();
	if let Err(err) = storage
		.store_prepared(
			&CheckEmailTask {
				input: body.to_check_email_input(Arc::clone(&config)),
				job_id: CheckEmailJobId::SingleShot,
				webhook: None,
				metadata: Some(crate::worker::do_work::TaskMetadata {
					dispatch_id: None,
					lease_id: None,
					decision_policy,
					tenant_id: tenant_ctx.tenant_id.map(|id| id.to_string()),
					request_id: None,
					correlation_id: None,
					created_by: Some("api".to_string()),
					retry_policy: None,
					dedupe_key: None,
					task_db_id: None,
				}),
			},
			&prepared,
			storage.get_extra(),
		)
		.await
	{
		release_single_reserved_usage(&config, tenant_ctx).await;
		return Err(ReacherResponseError::from(err).into());
	}

	// Evaluate conditional actions (auto-suppression) for direct (non-worker) path
	if let Some(pool) = config.get_pg_pool() {
		if let Some(tenant_id) = tenant_ctx.tenant_id {
			crate::worker::actions::evaluate_post_completion_actions(
				&pool,
				tenant_id,
				&body.to_email,
				Some(prepared.response.score.score),
				Some(
					&serde_json::to_value(&prepared.response.score.category)
						.ok()
						.and_then(|v| v.as_str().map(ToOwned::to_owned))
						.unwrap_or_else(|| "unknown".to_string()),
				),
			)
			.await
			.map_err(ReacherResponseError::from)?;
		}
	}

	info!(target: LOG_TARGET, email=body.to_email, is_reachable=?prepared.output.is_reachable, "Done verification");
	Ok(prepared.response.body)
}

async fn release_single_reserved_usage(config: &BackendConfig, tenant_ctx: &TenantContext) {
	let pg_pool = config.get_pg_pool();
	let _ = release_reserved_usage(pg_pool.as_ref(), tenant_ctx.tenant_id, 1).await;
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::config::ThrottleConfig;

	#[tokio::test]
	async fn worker_mode_tenant_throttle_requires_the_shared_database() {
		let mut config = BackendConfig::empty();
		config.worker.enable = true;
		let config = Arc::new(config);
		let mut tenant_ctx = TenantContext::legacy(ThrottleConfig {
			max_requests_per_second: Some(1),
			..ThrottleConfig::new_without_throttle()
		});
		tenant_ctx.is_legacy = false;
		tenant_ctx.tenant_id = Some(uuid::Uuid::new_v4());
		let body = CheckEmailRequest {
			to_email: "someone@example.com".into(),
			..Default::default()
		};
		let status = || async {
			handle_check_email(
				Arc::clone(&config),
				b"{}",
				&body,
				&tenant_ctx,
				"/v1/check_email",
				None,
			)
			.await
			.err()
			.and_then(|rejection| {
				rejection
					.find::<ReacherResponseError>()
					.map(|error| error.code)
			})
		};

		assert_eq!(status().await, Some(StatusCode::SERVICE_UNAVAILABLE));
		assert_eq!(status().await, Some(StatusCode::SERVICE_UNAVAILABLE));
	}
}
