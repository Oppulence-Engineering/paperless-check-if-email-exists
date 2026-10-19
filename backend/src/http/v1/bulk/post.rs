//! This file implements the `POST /v1/bulk` endpoint.

use std::sync::Arc;

use bytes::Bytes;
use check_if_email_exists::LOG_TARGET;
use futures::stream::StreamExt;
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
use crate::tenant::quota::{
	check_and_increment_quota_for_count, release_reserved_usage, QuotaCheckResult,
};
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
			IdempotencyCheck::Cached(cached) => {
				let status =
					StatusCode::from_u16(cached.status_code).map_err(ReacherResponseError::from)?;
				return Ok(json_response(status, cached.body));
			}
		},
		None => None,
	};

	let result = create_bulk_job(&tenant_ctx, config, &pg_pool, body).await;

	if let Some(record_id) = idempotency_record_id {
		let settled = match &result {
			Ok(response_body) => {
				complete_idempotency_key(
					&pg_pool,
					record_id,
					StatusCode::OK.as_u16(),
					response_body,
				)
				.await
			}
			// Failed attempts release the key so the client can retry with it.
			Err(_) => fail_idempotency_key(&pg_pool, record_id).await,
		};
		if let Err(err) = settled {
			warn!(target: LOG_TARGET, record_id = record_id, error = ?err, "Failed to settle bulk idempotency key");
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

/// Reserves quota, creates the job and its task rows, and publishes the
/// tasks. Returns the serialized response body. Any failure after the quota
/// reservation refunds every email that was not published.
async fn create_bulk_job(
	tenant_ctx: &TenantContext,
	config: Arc<BackendConfig>,
	pg_pool: &PgPool,
	body: Request,
) -> Result<Vec<u8>, ReacherResponseError> {
	let source_key = normalize_source_key(body.source_key.as_deref());

	// Atomically check and increment quota for all emails in the bulk request.
	let email_count = body.input.len() as i32;
	match check_and_increment_quota_for_count(Some(pg_pool), tenant_ctx, email_count).await {
		QuotaCheckResult::Allowed => {}
		QuotaCheckResult::ExceededMonthlyLimit {
			limit,
			used,
			resets_at,
		} => {
			return Err(ReacherResponseError::new(
				StatusCode::TOO_MANY_REQUESTS,
				format!(
					"Monthly email limit of {} reached ({} used). Resets at {}",
					limit,
					used,
					resets_at.format("%Y-%m-%d %H:%M:%S UTC")
				),
			));
		}
	}

	let tenant_id = tenant_ctx.tenant_id;

	// Create job entry with status = 'pending'
	let job_id: i32 = match sqlx::query_scalar(
		r#"
		INSERT INTO v1_bulk_job (total_records, tenant_id, status, source_key)
		VALUES ($1, $2, 'pending', $3)
		RETURNING id
		"#,
	)
	.bind(email_count)
	.bind(tenant_id)
	.bind(source_key.as_deref())
	.fetch_one(pg_pool)
	.await
	{
		Ok(job_id) => job_id,
		Err(err) => {
			let _ = release_reserved_usage(Some(pg_pool), tenant_id, email_count).await;
			return Err(ReacherResponseError::from(err));
		}
	};

	let task_ids = match insert_queued_tasks(
		&config,
		pg_pool,
		job_id,
		tenant_id,
		source_key.as_deref(),
		&body,
	)
	.await
	{
		Ok(task_ids) => task_ids,
		Err(err) => {
			cleanup_publish_failure(pg_pool, tenant_id, job_id, email_count, &[]).await;
			return Err(err);
		}
	};

	// Publish tasks to RabbitMQ with metadata.task_db_id
	let n = body.input.len();
	let webhook = body.webhook.clone();
	let emails_with_ids: Vec<(String, i32)> = body.input.into_iter().zip(task_ids).collect();
	let stream = futures::stream::iter(emails_with_ids.into_iter());

	let properties = BasicProperties::default()
		.with_content_type("application/json".into())
		.with_priority(1); // Low priority

	let channel = match config.must_worker_config() {
		Ok(worker_config) => worker_config.channel,
		Err(err) => {
			cleanup_publish_failure(pg_pool, tenant_id, job_id, email_count, &[]).await;
			return Err(ReacherResponseError::from(err));
		}
	};
	let publish_results = stream
		.map(|(to_email, task_db_id)| {
			let config = Arc::clone(&config);
			let webhook = webhook.clone();
			let properties = properties.clone();
			let channel = Arc::clone(&channel);
			async move {
				let input = CheckEmailRequest {
					to_email,
					..Default::default()
				}
				.to_check_email_input(Arc::clone(&config));

				let task = CheckEmailTask {
					input,
					job_id: CheckEmailJobId::Bulk(job_id),
					webhook,
					metadata: Some(TaskMetadata {
						tenant_id: tenant_id.map(|id| id.to_string()),
						request_id: None,
						correlation_id: None,
						created_by: None,
						retry_policy: None,
						dedupe_key: None,
						task_db_id: Some(task_db_id),
					}),
				};

				publish_task(channel, task, properties)
					.await
					.map(|_| task_db_id)
			}
		})
		.buffer_unordered(10)
		.collect::<Vec<_>>()
		.await;

	let mut published_task_ids = Vec::new();
	let mut publish_error = None;
	for result in publish_results {
		match result {
			Ok(task_id) => published_task_ids.push(task_id),
			Err(err) if publish_error.is_none() => publish_error = Some(err),
			Err(_) => {}
		}
	}
	if let Some(err) = publish_error {
		cleanup_publish_failure(pg_pool, tenant_id, job_id, email_count, &published_task_ids).await;
		return Err(err);
	}

	// Update job to 'running'
	sqlx::query!(
		"UPDATE v1_bulk_job SET status = 'running', updated_at = NOW() WHERE id = $1",
		job_id,
	)
	.execute(pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	// Record job.created event
	let _ = sqlx::query!(
		r#"
		INSERT INTO job_events (job_id, event_type, event_data, actor)
		VALUES ($1, 'job.created', $2, 'api')
		"#,
		job_id,
		serde_json::json!({ "total_records": n }),
	)
	.execute(pg_pool)
	.await;

	info!(
		target: LOG_TARGET,
		queue = CHECK_EMAIL_QUEUE,
		"Added {n} emails",
	);
	serde_json::to_vec(&Response { job_id, source_key }).map_err(ReacherResponseError::from)
}

/// Pre-create one task result row per input email, in state 'queued'.
async fn insert_queued_tasks(
	config: &Arc<BackendConfig>,
	pg_pool: &PgPool,
	job_id: i32,
	tenant_id: Option<uuid::Uuid>,
	source_key: Option<&str>,
	body: &Request,
) -> Result<Vec<i32>, ReacherResponseError> {
	let mut task_ids: Vec<i32> = Vec::with_capacity(body.input.len());
	for email in &body.input {
		let input = CheckEmailRequest {
			to_email: email.clone(),
			..Default::default()
		}
		.to_check_email_input(Arc::clone(config));

		let payload_json = serde_json::to_value(&CheckEmailTask {
			input,
			job_id: CheckEmailJobId::Bulk(job_id),
			webhook: body.webhook.clone(),
			metadata: None,
		})
		.map_err(ReacherResponseError::from)?;

		let task_row: i32 = sqlx::query_scalar(
			r#"
			INSERT INTO v1_task_result (job_id, payload, task_state, tenant_id, source_key)
			VALUES ($1, $2, 'queued', $3, $4)
			RETURNING id
			"#,
		)
		.bind(job_id)
		.bind(payload_json)
		.bind(tenant_id)
		.bind(source_key)
		.fetch_one(pg_pool)
		.await
		.map_err(ReacherResponseError::from)?;

		task_ids.push(task_row);
	}
	Ok(task_ids)
}

fn normalize_source_key(source_key: Option<&str>) -> Option<String> {
	let source_key = source_key?.trim().to_ascii_lowercase();
	if source_key.is_empty() {
		None
	} else {
		Some(source_key)
	}
}

async fn cleanup_publish_failure(
	pg_pool: &PgPool,
	tenant_id: Option<uuid::Uuid>,
	job_id: i32,
	reserved_count: i32,
	published_task_ids: &[i32],
) {
	let refund_count = reserved_count.saturating_sub(published_task_ids.len() as i32);
	if refund_count > 0 {
		let _ = release_reserved_usage(Some(pg_pool), tenant_id, refund_count).await;
	}

	if let Err(err) = sqlx::query(
		"UPDATE v1_bulk_job SET status = 'failed'::job_state, completed_at = COALESCE(completed_at, NOW()), updated_at = NOW() WHERE id = $1",
	)
	.bind(job_id)
	.execute(pg_pool)
	.await
	{
		warn!(target: LOG_TARGET, job_id = job_id, error = ?err, "Failed to mark bulk job failed after publish failure");
	}

	if let Err(err) = sqlx::query(
		r#"
		UPDATE v1_task_result
		SET task_state = 'failed'::task_state,
		    error = COALESCE(error, 'publish_failed'),
		    completed_at = COALESCE(completed_at, NOW()),
		    updated_at = NOW()
		WHERE job_id = $1
		  AND task_state = 'queued'::task_state
		  AND NOT (id = ANY($2::INTEGER[]))
		"#,
	)
	.bind(job_id)
	.bind(published_task_ids)
	.execute(pg_pool)
	.await
	{
		warn!(target: LOG_TARGET, job_id = job_id, error = ?err, "Failed to mark queued tasks failed after publish failure");
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
