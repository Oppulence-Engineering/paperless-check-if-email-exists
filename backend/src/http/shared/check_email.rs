use crate::http::idempotency::{
	check_idempotency_key, complete_idempotency_key, fail_idempotency_key, hash_request_body,
	IdempotencyCheck,
};
use check_if_email_exists::{check_email, LOG_TARGET};
use futures::StreamExt;
use lapin::options::{
	BasicAckOptions, BasicConsumeOptions, BasicRejectOptions, QueueDeclareOptions,
};
use lapin::types::FieldTable;
use lapin::BasicProperties;
use std::sync::Arc;
use tracing::{info, warn};
use warp::http::StatusCode;

use crate::config::BackendConfig;
use crate::http::v0::check_email::post::CheckEmailRequest;
use crate::http::v1::bulk::post::publish_task;
use crate::http::ReacherResponseError;
use crate::scoring::response::{prepare_check_email_success, prepare_verification_response};
use crate::storage::commercial_license_trial::send_to_reacher;
use crate::tenant::context::TenantContext;
use crate::tenant::quota::{check_and_increment_quota, QuotaCheckResult};
use crate::worker::consume::MAX_QUEUE_PRIORITY;
use crate::worker::do_work::{CheckEmailJobId, CheckEmailTask};
use crate::worker::single_shot::SingleShotReply;
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

async fn mark_idempotency_complete(
	pool: &sqlx::PgPool,
	record_id: i64,
	status_code: u16,
	response_body: &[u8],
) {
	if let Err(e) = complete_idempotency_key(pool, record_id, status_code, response_body).await {
		warn!(target: LOG_TARGET, record_id = ?record_id, error = ?e, "Failed to mark idempotency key as completed");
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

	// Sandbox mode: return deterministic mock results without SMTP,
	// throttle checks, or quota consumption. Freshness fields are
	// hardcoded so responses are fully deterministic across time.
	if body.sandbox {
		let mock_result = crate::sandbox::sandbox_check(&body.to_email);
		let completed_at = Utc
			.with_ymd_and_hms(2025, 1, 1, 0, 0, 0)
			.single()
			.expect("static sandbox timestamp");
		let prepared =
			prepare_verification_response(config.as_ref(), &mock_result, None, completed_at, false)
				.await
				.map_err(ReacherResponseError::from)?;
		return Ok(CheckEmailResponse {
			status_code: StatusCode::OK,
			body: prepared.body,
		});
	}

	let mut idempotency_record_id: Option<i64> = None;
	let mut idempotency_pool: Option<sqlx::PgPool> = None;

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
				IdempotencyCheck::Cached(cached) => {
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

	// Get per-tenant throttle manager
	let throttle_manager =
		config.get_tenant_throttle_manager(tenant_ctx.tenant_id, &tenant_ctx.throttle);

	// Check throttle
	if let Some(throttle_result) = throttle_manager.check_throttle().await {
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

	// Atomically check + increment monthly quota (avoids TOCTOU race)
	let pg_pool = config.get_pg_pool();
	match check_and_increment_quota(pg_pool.as_ref(), tenant_ctx).await {
		QuotaCheckResult::Allowed => {} // Quota already incremented atomically
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

	if !config.worker.enable {
		let response =
			handle_without_worker(Arc::clone(&config), body, &throttle_manager, tenant_ctx).await;
		match response {
			Ok(body) => {
				let response = CheckEmailResponse {
					status_code: StatusCode::OK,
					body,
				};
				if let (Some(pool), Some(record_id)) = (idempotency_pool, idempotency_record_id) {
					mark_idempotency_complete(
						&pool,
						record_id,
						response.status_code.as_u16(),
						&response.body,
					)
					.await;
				}
				Ok(response)
			}
			Err(error) => {
				if let (Some(pool), Some(record_id)) = (idempotency_pool, idempotency_record_id) {
					mark_idempotency_failed(&pool, record_id).await;
				}
				Err(error)
			}
		}
	} else {
		let response = handle_with_worker(Arc::clone(&config), body, tenant_ctx).await;
		match response {
			Ok(body) => {
				let response = CheckEmailResponse {
					status_code: StatusCode::OK,
					body,
				};
				if let (Some(pool), Some(record_id)) = (idempotency_pool, idempotency_record_id) {
					mark_idempotency_complete(
						&pool,
						record_id,
						response.status_code.as_u16(),
						&response.body,
					)
					.await;
				}
				Ok(response)
			}
			Err(error) => {
				if let (Some(pool), Some(record_id)) = (idempotency_pool, idempotency_record_id) {
					mark_idempotency_failed(&pool, record_id).await;
				}
				Err(error)
			}
		}
	}
}

async fn handle_without_worker(
	config: Arc<BackendConfig>,
	body: &CheckEmailRequest,
	throttle_manager: &crate::throttle::ThrottleManager,
	tenant_ctx: &TenantContext,
) -> Result<Vec<u8>, warp::Rejection> {
	info!(target: LOG_TARGET, email=body.to_email, "Starting verification");
	let input = body.to_check_email_input(Arc::clone(&config));
	let result = check_email(&input).await;
	let prepared = prepare_check_email_success(
		config.as_ref(),
		result,
		tenant_ctx.tenant_id,
		Utc::now(),
		true,
	)
	.await
	.map_err(ReacherResponseError::from)?;

	throttle_manager.increment_counters().await;

	let storage = Arc::clone(&config).get_storage_adapter();
	storage
		.store_prepared(
			&CheckEmailTask {
				input: body.to_check_email_input(Arc::clone(&config)),
				job_id: CheckEmailJobId::SingleShot,
				webhook: None,
				metadata: None,
			},
			&prepared,
			storage.get_extra(),
		)
		.await
		.map_err(ReacherResponseError::from)?;

	send_to_reacher(Arc::clone(&config), &body.to_email, Ok(&prepared))
		.await
		.map_err(ReacherResponseError::from)?;

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
			.await;
		}
	}

	info!(target: LOG_TARGET, email=body.to_email, is_reachable=?prepared.output.is_reachable, "Done verification");
	Ok(prepared.response.body)
}

async fn handle_with_worker(
	config: Arc<BackendConfig>,
	body: &CheckEmailRequest,
	tenant_ctx: &TenantContext,
) -> Result<Vec<u8>, warp::Rejection> {
	let channel = config
		.must_worker_config()
		.map_err(ReacherResponseError::from)?
		.channel;

	let correlation_id = uuid::Uuid::new_v4();
	let reply_queue = channel
		.queue_declare(
			"",
			QueueDeclareOptions {
				auto_delete: true,
				durable: false,
				exclusive: true,
				..Default::default()
			},
			FieldTable::default(),
		)
		.await
		.map_err(ReacherResponseError::from)?;

	let properties = BasicProperties::default()
		.with_content_type("application/json".into())
		.with_priority(MAX_QUEUE_PRIORITY)
		.with_correlation_id(correlation_id.to_string().into())
		.with_reply_to(reply_queue.name().to_owned());

	publish_task(
		channel.clone(),
		CheckEmailTask {
			input: body.to_check_email_input(config.clone()),
			job_id: CheckEmailJobId::SingleShot,
			webhook: None,
			metadata: Some(crate::worker::do_work::TaskMetadata {
				tenant_id: tenant_ctx.tenant_id.map(|id| id.to_string()),
				request_id: None,
				correlation_id: None,
				created_by: Some("api".to_string()),
				retry_policy: None,
				dedupe_key: None,
				task_db_id: None,
			}),
		},
		properties,
	)
	.await?;

	let mut consumer = channel
		.basic_consume(
			reply_queue.name().as_str(),
			format!("rpc.{}", correlation_id).as_str(),
			BasicConsumeOptions::default(),
			FieldTable::default(),
		)
		.await
		.map_err(ReacherResponseError::from)?;

	if let Some(delivery) = consumer.next().await {
		let delivery = delivery.map_err(ReacherResponseError::from)?;

		if delivery
			.properties
			.correlation_id()
			.as_ref()
			.map(|s| s.as_str())
			== Some(correlation_id.to_string().as_str())
		{
			delivery
				.ack(BasicAckOptions::default())
				.await
				.map_err(ReacherResponseError::from)?;

			let single_shot_response = serde_json::from_slice::<SingleShotReply>(&delivery.data)
				.map_err(ReacherResponseError::from)?;

			match single_shot_response {
				SingleShotReply::Ok(body) => {
					return Ok(body);
				}
				SingleShotReply::Err((e, code)) => {
					let status_code =
						StatusCode::from_u16(code).map_err(ReacherResponseError::from)?;
					return Err(ReacherResponseError::new(status_code, e).into());
				}
			}
		} else {
			delivery
				.reject(BasicRejectOptions { requeue: false })
				.await
				.map_err(ReacherResponseError::from)?;
			return Err(ReacherResponseError::new(
				StatusCode::INTERNAL_SERVER_ERROR,
				"Failed to get a reply from the worker.",
			)
			.into());
		}
	}

	Err(ReacherResponseError::new(
		StatusCode::INTERNAL_SERVER_ERROR,
		"Failed to get a reply from the worker.",
	)
	.into())
}
