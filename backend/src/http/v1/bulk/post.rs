//! This file implements the `POST /v1/bulk` endpoint.

use std::{convert::TryFrom, sync::Arc};

use bytes::Bytes;
use check_if_email_exists::LOG_TARGET;
use lapin::Channel;
use lapin::{options::*, BasicProperties};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::{debug, info, warn};
use warp::http::header::CONTENT_TYPE;
use warp::http::StatusCode;
use warp::{Filter, Reply};

use super::with_worker_db;
use crate::config::BackendConfig;
use crate::http::idempotency::{
	check_idempotency_key, complete_idempotency_key, fail_idempotency_key, hash_request_body,
	IdempotencyCheck,
};
use crate::http::v0::check_email::post::with_config;
use crate::http::CheckEmailRequest;
use crate::http::ReacherResponseError;
use crate::http::{check_scope, resolve_tenant};
use crate::tenant::context::{scope, TenantContext};
use crate::tenant::quota::{reserve_quota_in_tx, QuotaCheckResult};
use crate::worker::consume::CHECK_EMAIL_QUEUE;
use crate::worker::do_work::CheckEmailJobId;
use crate::worker::do_work::CheckEmailTask;
use crate::worker::do_work::TaskMetadata;
use crate::worker::do_work::TaskWebhook;

const BULK_REQUEST_PATH: &str = "/v1/bulk";

/// AMQP delivery mode 2 marks a message persistent, so the durable
/// `check_email` queue keeps it across a broker restart. Left unset, messages
/// are transient and a restart drops every queued task.
const PERSISTENT_DELIVERY_MODE: u8 = 2;

/// POST v1/bulk endpoint request body.
#[derive(Debug, Deserialize)]
struct Request {
	input: Vec<String>,
	webhook: Option<TaskWebhook>,
	#[serde(default, alias = "source")]
	source_key: Option<String>,
}

/// POST v1/bulk endpoint response body.
#[derive(Clone, Debug, Deserialize, Serialize)]
struct Response {
	job_id: i32,
	#[serde(skip_serializing_if = "Option::is_none")]
	source_key: Option<String>,
}

async fn http_handler(
	tenant_ctx: TenantContext,
	config: Arc<BackendConfig>,
	pg_pool: PgPool,
	idempotency_key: Option<String>,
	raw_body: Bytes,
) -> Result<warp::reply::Response, warp::Rejection> {
	check_scope(&tenant_ctx, scope::BULK)?;

	let body: Request = serde_json::from_slice(&raw_body).map_err(|err| {
		ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			format!("Invalid request body: {}", err),
		)
	})?;
	if body.input.is_empty() {
		return Err(ReacherResponseError::new(StatusCode::BAD_REQUEST, "Empty input").into());
	}

	// A client retrying a request whose response it never received must get
	// the original job back, not a second job charged against its quota.
	let idempotency_record_id = match idempotency_key {
		Some(key) => match check_idempotency_key(
			&pg_pool,
			&tenant_ctx.tenant_id_str(),
			&key,
			BULK_REQUEST_PATH,
			&hash_request_body(&raw_body),
			&tenant_ctx.tenant_name,
		)
		.await?
		{
			IdempotencyCheck::New { record_id } => Some(record_id),
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
				let status =
					StatusCode::from_u16(cached.status_code).map_err(ReacherResponseError::from)?;
				return Ok(json_response(status, cached.body));
			}
		},
		None => None,
	};

	let result = create_bulk_job(&tenant_ctx, config, &pg_pool, body, idempotency_record_id).await;
	if result.is_err() {
		if let Some(record_id) = idempotency_record_id {
			// A committed response is never reopened after an uncertain commit result.
			if let Err(error) = fail_idempotency_key(&pg_pool, record_id).await {
				warn!(target: LOG_TARGET, record_id, error = ?error, "Failed to settle bulk idempotency key");
			}
		}
	}

	Ok(json_response(StatusCode::OK, result?))
}

fn json_response(status: StatusCode, body: Vec<u8>) -> warp::reply::Response {
	let mut response = warp::reply::with_status(body, status).into_response();
	response.headers_mut().insert(
		CONTENT_TYPE,
		"application/json".parse().expect("static header value"),
	);
	response
}

/// Quota, task rows, dispatch intent and the replayable response commit together.
async fn create_bulk_job(
	tenant_ctx: &TenantContext,
	config: Arc<BackendConfig>,
	pg_pool: &PgPool,
	body: Request,
	idempotency_record_id: Option<i64>,
) -> Result<Vec<u8>, ReacherResponseError> {
	crate::smtp_egress::require_smtp_ready(&config)?;
	let source_key = normalize_source_key(body.source_key.as_deref());
	let email_count = i32::try_from(body.input.len())
		.map_err(|_| ReacherResponseError::new(StatusCode::BAD_REQUEST, "Too many emails"))?;
	let mut tx = pg_pool.begin().await.map_err(ReacherResponseError::from)?;
	if let Some(id) = idempotency_record_id {
		let owner: Option<i64> = sqlx::query_scalar(
			"SELECT id FROM idempotency_keys WHERE id = $1 AND status = 'processing' FOR UPDATE",
		)
		.bind(id)
		.fetch_optional(&mut *tx)
		.await
		.map_err(ReacherResponseError::from)?;
		if owner.is_none() {
			return Err(ReacherResponseError::new(
				StatusCode::CONFLICT,
				"Request ownership changed; retry with the same key",
			));
		}
	}
	match reserve_quota_in_tx(&mut tx, tenant_ctx, email_count)
		.await
		.map_err(ReacherResponseError::from)?
	{
		QuotaCheckResult::Allowed => {}
		QuotaCheckResult::Unavailable => {
			return Err(ReacherResponseError::new(
				StatusCode::SERVICE_UNAVAILABLE,
				"Quota service unavailable",
			))
		}
		QuotaCheckResult::ExceededMonthlyLimit {
			limit,
			used,
			resets_at,
		} => {
			return Err(ReacherResponseError::new(
				StatusCode::TOO_MANY_REQUESTS,
				format!(
					"Monthly email limit of {limit} reached ({used} used). Resets at {resets_at}"
				),
			));
		}
	}
	let job_id: i32 = sqlx::query_scalar("INSERT INTO v1_bulk_job (total_records, tenant_id, status, source_key) VALUES ($1, $2, 'pending', $3) RETURNING id")
        .bind(email_count).bind(tenant_ctx.tenant_id).bind(&source_key)
        .fetch_one(&mut *tx).await.map_err(ReacherResponseError::from)?;
	for email in body.input {
		let mut task = CheckEmailTask {
			input: CheckEmailRequest {
				to_email: email.clone(),
				..Default::default()
			}
			.to_check_email_input(Arc::clone(&config)),
			job_id: CheckEmailJobId::Bulk(job_id),
			webhook: body.webhook.clone(),
			metadata: Some(TaskMetadata {
				dispatch_id: None,
				lease_id: None,
				decision_policy: None,
				tenant_id: tenant_ctx.tenant_id.map(|id| id.to_string()),
				request_id: None,
				correlation_id: None,
				created_by: None,
				retry_policy: None,
				dedupe_key: None,
				task_db_id: None,
			}),
		};
		let task_id = sqlx::query_scalar("INSERT INTO v1_task_result (job_id, payload, task_state, tenant_id, source_key) VALUES ($1, $2, 'queued', $3, $4) RETURNING id")
            .bind(job_id).bind(serde_json::json!({"input":{"to_email": email}, "webhook": body.webhook}))
            .bind(tenant_ctx.tenant_id).bind(&source_key).fetch_one(&mut *tx).await.map_err(ReacherResponseError::from)?;
		task.metadata
			.as_mut()
			.expect("metadata set above")
			.task_db_id = Some(task_id);
		crate::worker::dispatch::enqueue(&mut tx, task, 1)
			.await
			.map_err(ReacherResponseError::from)?;
	}
	sqlx::query("UPDATE v1_bulk_job SET status = 'running', updated_at = NOW() WHERE id = $1 AND status = 'pending'")
        .bind(job_id).execute(&mut *tx).await.map_err(ReacherResponseError::from)?;
	sqlx::query("INSERT INTO job_events (job_id, event_type, event_data, actor) VALUES ($1, 'job.created', $2, 'api')")
        .bind(job_id).bind(serde_json::json!({"total_records":email_count}))
        .execute(&mut *tx).await.map_err(ReacherResponseError::from)?;
	let response =
		serde_json::to_vec(&Response { job_id, source_key }).map_err(ReacherResponseError::from)?;
	if let Some(id) = idempotency_record_id {
		complete_idempotency_key(&mut *tx, id, StatusCode::OK.as_u16(), &response).await?;
	}
	tx.commit().await.map_err(ReacherResponseError::from)?;
	info!(target: LOG_TARGET, job_id, email_count, "Committed bulk verification dispatch");
	Ok(response)
}

fn normalize_source_key(source_key: Option<&str>) -> Option<String> {
	let source_key = source_key?.trim().to_ascii_lowercase();
	if source_key.is_empty() {
		None
	} else {
		Some(source_key)
	}
}

/// Publish a task to the "check_email" queue.
pub async fn publish_task(
	channel: Arc<Channel>,
	task: CheckEmailTask,
	properties: BasicProperties,
) -> Result<(), ReacherResponseError> {
	let payload = serde_json::to_vec(&task).map_err(ReacherResponseError::from)?;

	publish_payload(&channel, &payload, properties).await?;

	debug!(
		target: LOG_TARGET,
		email = ?task.input.to_email,
		queue = CHECK_EMAIL_QUEUE,
		"Published task"
	);

	Ok(())
}

/// Publish an already serialized task to the "check_email" queue.
///
/// The message is always made persistent. On a channel in confirm mode (the
/// channel from `setup_rabbit_mq` is), this returns only after the broker has
/// taken responsibility for the message, and a broker nack is an error, so
/// callers never treat a task the broker refused as queued.
pub async fn publish_payload(
	channel: &Channel,
	payload: &[u8],
	properties: BasicProperties,
) -> Result<(), ReacherResponseError> {
	let confirmation = channel
		.basic_publish(
			"".into(),
			CHECK_EMAIL_QUEUE.into(),
			BasicPublishOptions::default(),
			payload,
			properties.with_delivery_mode(PERSISTENT_DELIVERY_MODE),
		)
		.await?
		.await?;

	if confirmation.is_nack() {
		return Err(ReacherResponseError::new(
			StatusCode::SERVICE_UNAVAILABLE,
			"RabbitMQ did not accept the task (publisher nack)",
		));
	}

	Ok(())
}

/// Create the v1 bulk endpoint.
///
/// Creates a tenant-scoped bulk job for async processing.
#[utoipa::path(
	post,
	path = "/v1/bulk",
	tag = "v1",
	params((
		"Idempotency-Key" = Option<String>, Header, description = "Optional idempotency key")
	),
	responses((status = 200, description = "Bulk job created"))
)]
pub fn v1_create_bulk_job(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "bulk")
		.and(warp::post())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_config(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::header::optional::<String>("Idempotency-Key"))
		.and(warp::body::content_length_limit(1024 * 1024 * 50))
		.and(warp::body::bytes())
		.and_then(http_handler)
		.with(warp::log(LOG_TARGET))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn source_key_is_normalized_for_bulk_jobs() {
		assert_eq!(
			normalize_source_key(Some(" Apollo Import ")),
			Some("apollo import".to_string())
		);
		assert_eq!(normalize_source_key(Some(" ")), None);
		assert_eq!(normalize_source_key(None), None);
	}
}
