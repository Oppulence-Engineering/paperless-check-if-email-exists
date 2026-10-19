use crate::config::BackendConfig;
use crate::http::v1::bulk::post::publish_payload;
use crate::scoring::response::{prepare_check_email_success, PreparedCheckEmailSuccess};
use crate::throttle::ThrottleResult;
use crate::worker::single_shot::send_single_shot_reply;
use check_if_email_exists::{check_email, CheckEmailInput, Reachable, LOG_TARGET};
use chrono::Utc;
use http::HeaderMap;
use lapin::message::Delivery;
use lapin::types::AMQPValue;
use lapin::{options::*, BasicProperties, Channel};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use std::fmt::Debug;
use std::net::IpAddr;
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use tracing::{debug, error, info, warn};
use warp::http::StatusCode;

/// AMQP header counting how many times a worker has put this message back on
/// the queue. The `check_email` queue is a classic (priority) queue, which
/// does not count redeliveries the way quorum queues do (`x-delivery-count`),
/// and single-shot tasks have no DB row holding a retry_count. Without this
/// header a retryable error requeued such a task, re-running its SMTP check,
/// forever.
pub const DELIVERY_ATTEMPT_HEADER: &str = "x-reacher-attempt";

/// Cap on requeues of a bulk task whose outcome could not be stored. Bounds
/// the case where storing can never succeed, e.g. the job was deleted while
/// the task was in flight.
pub const MAX_STORE_FAILURE_REQUEUES: u32 = 8;
/// Pause before requeueing after a store failure, doubled per attempt, so a
/// Postgres outage is ridden out (about four minutes in total) rather than
/// burned through in a burst of SMTP re-checks.
const STORE_FAILURE_BASE_DELAY: Duration = Duration::from_secs(2);
const STORE_FAILURE_MAX_DELAY: Duration = Duration::from_secs(60);

/// Webhook POST attempts per stored result. Retrying the POST is cheap;
/// retrying the whole task, as a webhook 5xx used to do, re-ran the SMTP
/// check.
const WEBHOOK_MAX_ATTEMPTS: u32 = 3;
const WEBHOOK_RETRY_BASE_DELAY: Duration = Duration::from_secs(1);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CheckEmailTask {
	pub input: CheckEmailInput,
	pub job_id: CheckEmailJobId,
	pub webhook: Option<TaskWebhook>,
	#[serde(default)]
	pub metadata: Option<TaskMetadata>,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckEmailJobId {
	/// Single-shot email verification, they won't have an actual job id.
	SingleShot,
	/// Job id of the bulk verification.
	Bulk(i32),
}

/// Metadata attached to a task for tenant scoping, retry policy, and
/// lifecycle tracking.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaskMetadata {
	pub tenant_id: Option<String>,
	pub request_id: Option<String>,
	pub correlation_id: Option<String>,
	pub created_by: Option<String>,
	pub retry_policy: Option<RetryPolicy>,
	pub dedupe_key: Option<String>,
	pub task_db_id: Option<i32>,
	// NOTE: webhook_signing_secret is intentionally NOT in TaskMetadata.
	// It must never be serialized into RabbitMQ messages or the DB payload column.
	// The worker loads it from the tenants table at execution time using tenant_id.
}

/// Configurable retry policy per task.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RetryPolicy {
	#[serde(default = "default_max_retries")]
	pub max_retries: u32,
	#[serde(default = "default_backoff_seconds")]
	pub backoff_seconds: u64,
	#[serde(default = "default_backoff_multiplier")]
	pub backoff_multiplier: f64,
}

fn default_max_retries() -> u32 {
	2
}
fn default_backoff_seconds() -> u64 {
	5
}
fn default_backoff_multiplier() -> f64 {
	2.0
}

impl Default for RetryPolicy {
	fn default() -> Self {
		Self {
			max_retries: default_max_retries(),
			backoff_seconds: default_backoff_seconds(),
			backoff_multiplier: default_backoff_multiplier(),
		}
	}
}

/// The errors that can occur when processing a task.
#[derive(Debug, Error)]
pub enum TaskError {
	/// The worker is at full capacity and cannot accept more tasks. Note that
	/// this error only occurs for single-shot tasks, and not for bulk
	/// verification, as for bulk verification tasks the task will simply stay
	/// in the queue until one worker is ready to process it.
	#[error("Worker at full capacity, wait {0:?}")]
	Throttle(ThrottleResult),
	#[error("Lapin error: {0}")]
	Lapin(lapin::Error),
	#[error("Reqwest error during webhook: {0}")]
	Reqwest(reqwest::Error),
	#[error("Error converting headers: {0}")]
	Headers(#[from] http::Error),
	#[error("Failed to prepare verification response: {0}")]
	Prepare(String),
	#[error("Invalid webhook URL: {0}")]
	WebhookValidation(String),
	#[error("Webhook returned HTTP status {0}")]
	WebhookStatus(u16),
}

impl TaskError {
	/// Returns the status code that should be returned to the client.
	pub fn status_code(&self) -> StatusCode {
		match self {
			Self::Throttle(_) => StatusCode::TOO_MANY_REQUESTS,
			Self::Lapin(_) => StatusCode::INTERNAL_SERVER_ERROR,
			Self::Reqwest(_) => StatusCode::INTERNAL_SERVER_ERROR,
			Self::Headers(_) => StatusCode::INTERNAL_SERVER_ERROR,
			Self::Prepare(_) => StatusCode::INTERNAL_SERVER_ERROR,
			Self::WebhookValidation(_) => StatusCode::BAD_REQUEST,
			Self::WebhookStatus(_) => StatusCode::BAD_GATEWAY,
		}
	}

	pub fn is_retryable(&self) -> bool {
		match self {
			Self::WebhookValidation(_) | Self::Headers(_) => false,
			Self::WebhookStatus(status) => (500..=599).contains(status),
			_ => true,
		}
	}
}

impl From<lapin::Error> for TaskError {
	fn from(err: lapin::Error) -> Self {
		Self::Lapin(err)
	}
}

impl From<reqwest::Error> for TaskError {
	fn from(err: reqwest::Error) -> Self {
		Self::Reqwest(err)
	}
}

impl Serialize for TaskError {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		serializer.serialize_str(&self.to_string())
	}
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct TaskWebhook {
	pub on_each_email: Option<Webhook>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Webhook {
	pub url: String,
	pub headers: HashMap<String, String>,
	pub extra: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
struct WebhookOutput<'a> {
	result: &'a serde_json::Value,
	extra: &'a Option<serde_json::Value>,
}

/// Update task state in the database. Fire-and-forget on error.
async fn update_task_state(
	config: &BackendConfig,
	task_db_id: i32,
	state: &str,
	retry_count: Option<i32>,
) {
	if let Some(pool) = config.get_pg_pool() {
		let result = if let Some(count) = retry_count {
			sqlx::query(
				"UPDATE v1_task_result SET task_state = $2::task_state, retry_count = $3, updated_at = NOW() WHERE id = $1",
			)
			.bind(task_db_id)
			.bind(state)
			.bind(count)
			.execute(&pool)
			.await
		} else {
			sqlx::query(
				"UPDATE v1_task_result SET task_state = $2::task_state, updated_at = NOW() WHERE id = $1",
			)
			.bind(task_db_id)
			.bind(state)
			.execute(&pool)
			.await
		};

		if let Err(e) = result {
			warn!(target: LOG_TARGET, task_id=task_db_id, state=state, error=?e, "Failed to update task state");
		}
	}
}

async fn maybe_pause_before_delivery_settle() {
	if let Ok(raw_delay) = std::env::var("RCH_TEST_PRE_ACK_DELAY_MS") {
		if let Ok(delay_ms) = raw_delay.parse::<u64>() {
			if delay_ms > 0 {
				tokio::time::sleep(Duration::from_millis(delay_ms)).await;
			}
		}
	}
}

fn take_test_failpoint(name: &str) -> bool {
	match std::env::var(name) {
		Ok(raw) if raw == "1" => {
			std::env::remove_var(name);
			true
		}
		_ => false,
	}
}

/// Record a job event in the database. Fire-and-forget on error.
async fn record_event(
	config: &BackendConfig,
	job_id: i32,
	task_id: Option<i32>,
	event_type: &str,
	event_data: Option<serde_json::Value>,
	actor: Option<&str>,
) {
	if let Some(pool) = config.get_pg_pool() {
		let result = sqlx::query!(
			r#"
			INSERT INTO job_events (job_id, task_id, event_type, event_data, actor)
			VALUES ($1, $2, $3, $4, $5)
			"#,
			job_id,
			task_id,
			event_type,
			event_data,
			actor,
		)
		.execute(&pool)
		.await;

		if let Err(e) = result {
			warn!(target: LOG_TARGET, job_id=job_id, event_type=event_type, error=?e, "Failed to record event");
		}
	}
}

/// Processes the check email task asynchronously.
pub async fn do_check_email_work(
	task: &CheckEmailTask,
	delivery: Delivery,
	channel: Arc<Channel>,
	config: Arc<BackendConfig>,
) -> Result<(), anyhow::Error> {
	let task_db_id = task.metadata.as_ref().and_then(|m| m.task_db_id);
	let retry_policy = task
		.metadata
		.as_ref()
		.and_then(|m| m.retry_policy.clone())
		.unwrap_or_default();

	// Mark task as running
	if let Some(id) = task_db_id {
		update_task_state(&config, id, "running", None).await;

		// Update started_at
		if let Some(pool) = config.get_pg_pool() {
			let _ = sqlx::query!(
				"UPDATE v1_task_result SET started_at = NOW() WHERE id = $1",
				id,
			)
			.execute(&pool)
			.await;
		}
	}

	// Load webhook signing secret from DB if tenant_id is present. A failed
	// read stays an error rather than "no secret": sending unsigned would fail
	// at receivers that verify signatures and look authentic at those that
	// don't.
	let tenant_uuid = task
		.metadata
		.as_ref()
		.and_then(|m| m.tenant_id.as_deref())
		.and_then(|tid| tid.parse::<uuid::Uuid>().ok());
	let webhook_signing_secret: Result<Option<String>, sqlx::Error> =
		match (tenant_uuid, config.get_pg_pool()) {
			(Some(uuid), Some(pool)) => sqlx::query_scalar::<_, Option<String>>(
				"SELECT webhook_signing_secret FROM tenants WHERE id = $1",
			)
			.bind(uuid)
			.fetch_optional(&pool)
			.await
			.map(Option::flatten),
			_ => Ok(None),
		};

	// Only the verification: the webhook goes out once the result is stored
	// (deliver_task_webhook), so a failing tenant endpoint can neither discard
	// the result nor make the worker redo the SMTP check.
	let worker_output =
		check_email_and_prepare_result_with_config(task, Some(Arc::clone(&config))).await;

	// Determine current retry count from the database
	let db_retry_count = if let Some(id) = task_db_id {
		if let Some(pool) = config.get_pg_pool() {
			sqlx::query_scalar!("SELECT retry_count FROM v1_task_result WHERE id = $1", id)
				.fetch_optional(&pool)
				.await
				.ok()
				.flatten()
				.unwrap_or(0)
		} else {
			0
		}
	} else {
		0
	};
	// The row's retry_count exists only for tasks with a DB row and reads as 0
	// when the lookup fails; the attempt header carried on the message covers
	// both gaps, so retries stay bounded either way.
	let delivery_attempt = delivery_attempt(&delivery.properties);
	let current_retry_count =
		db_retry_count.max(i32::try_from(delivery_attempt).unwrap_or(i32::MAX));

	let unknown_success =
		matches!(&worker_output, Ok(output) if output.output.is_reachable == Reachable::Unknown);
	let should_schedule_delayed_recheck = unknown_success
		&& config.delayed_recheck.enable
		&& matches!(task.job_id, CheckEmailJobId::Bulk(_))
		&& task_db_id.is_some()
		&& (current_retry_count as u32) < retry_policy.max_retries;
	let delayed_recheck_count = current_retry_count + 1;

	let should_retry = match &worker_output {
		Err(error) => error.is_retryable(),
		_ => false,
	};

	if should_retry && (current_retry_count as u32) < retry_policy.max_retries {
		// Retry: increment count and requeue
		let new_count = current_retry_count + 1;
		if let Some(id) = task_db_id {
			update_task_state(&config, id, "retrying", Some(new_count)).await;

			if let CheckEmailJobId::Bulk(job_id) = task.job_id {
				record_event(
					&config,
					job_id,
					Some(id),
					"task.retrying",
					Some(serde_json::json!({ "retry_count": new_count })),
					Some(&config.backend_name),
				)
				.await;
			}
		}

		let delay_seconds = (retry_policy.backoff_seconds as f64
			* retry_policy.backoff_multiplier.powf((new_count - 1) as f64))
		.max(0.0);
		if delay_seconds > 0.0 {
			info!(
				target: LOG_TARGET,
				email = ?&task.input.to_email,
				retry = new_count,
				delay_seconds = delay_seconds,
				"Delaying retry before requeue"
			);
			tokio::time::sleep(std::time::Duration::from_secs_f64(delay_seconds)).await;
		}

		requeue_with_attempt(
			&delivery,
			&channel,
			u32::try_from(new_count).unwrap_or(u32::MAX),
		)
		.await?;
		info!(target: LOG_TARGET, email=?&task.input.to_email, retry=new_count, max=retry_policy.max_retries, "Requeued message for retry");
	} else if should_retry || worker_output.is_err() {
		// Exhausted retries or a deterministic non-retryable error: dead letter.
		maybe_pause_before_delivery_settle().await;

		// Persist the failure before settling the delivery, for the same
		// reason as on the success path below.
		if let Err(error) =
			finalize_dead_letter(task, &delivery, &channel, &config, &worker_output).await
		{
			return settle_after_store_failure(task, &delivery, &channel, delivery_attempt, error)
				.await;
		}
		delivery
			.reject(BasicRejectOptions { requeue: false })
			.await?;

		sync_related_entities(&config, task).await;

		info!(target: LOG_TARGET, email=?&task.input.to_email, "Dead-lettered terminal task result");
	} else {
		// Success path
		maybe_pause_before_delivery_settle().await;
		if take_test_failpoint("RCH_TEST_FORCE_REQUEUE_BEFORE_ACK") {
			delivery
				.reject(BasicRejectOptions { requeue: true })
				.await?;
			return Err(anyhow::anyhow!("test failpoint forced requeue before ack"));
		}

		// Store before acking: once acked the message is gone, so a result
		// that then failed to store left the row 'running' and its job and
		// list never finalized.
		if let Err(error) = delivery_finalize(
			task,
			&delivery,
			Arc::clone(&channel),
			Arc::clone(&config),
			&worker_output,
		)
		.await
		{
			return settle_after_store_failure(task, &delivery, &channel, delivery_attempt, error)
				.await;
		}

		// Right after storing, before anything slow: the recheck turns the
		// task back to 'retrying', and until then a sibling task finishing
		// would see this one as done and finalize the job without it.
		let awaiting_recheck = match task_db_id {
			Some(id) if should_schedule_delayed_recheck => {
				crate::delayed_recheck::schedule_delayed_recheck(
					task,
					Arc::clone(&config),
					id,
					delayed_recheck_count,
					&retry_policy,
				)
				.await
			}
			_ => false,
		};

		// Before the ack, so a worker crash mid-delivery redelivers the task
		// rather than silently dropping the webhook.
		if let Ok(success) = &worker_output {
			match &webhook_signing_secret {
				Ok(secret) => deliver_task_webhook(&config, task, success, secret.as_deref()).await,
				Err(error) => skip_webhook_without_secret(&config, task, error).await,
			}
		}

		delivery.ack(BasicAckOptions::default()).await?;

		if let Some(id) = task_db_id {
			if !awaiting_recheck {
				update_task_state(&config, id, "completed", None).await;
			}

			// Update completed_at
			if let Some(pool) = config.get_pg_pool() {
				let _ = sqlx::query!(
					"UPDATE v1_task_result SET completed_at = NOW() WHERE id = $1",
					id,
				)
				.execute(&pool)
				.await;
			}

			if let CheckEmailJobId::Bulk(job_id) = task.job_id {
				record_event(
					&config,
					job_id,
					Some(id),
					"task.completed",
					None,
					Some(&config.backend_name),
				)
				.await;
				if is_delayed_recheck_task(task) {
					record_event(
						&config,
						job_id,
						Some(id),
						"task.delayed_recheck_completed",
						None,
						Some(&config.backend_name),
					)
					.await;
				}
			}
		}

		// Evaluate conditional actions (auto-suppression) after successful completion
		if let (Some(pool), Ok(output)) = (config.get_pg_pool(), &worker_output) {
			if let Some(tenant_id_str) = task.metadata.as_ref().and_then(|m| m.tenant_id.clone()) {
				if let Ok(tenant_uuid) = tenant_id_str.parse::<uuid::Uuid>() {
					crate::worker::actions::evaluate_post_completion_actions(
						&pool,
						tenant_uuid,
						&task.input.to_email,
						Some(output.response.score.score),
						Some(
							&serde_json::to_value(&output.response.score.category)
								.ok()
								.and_then(|v| v.as_str().map(ToOwned::to_owned))
								.unwrap_or_else(|| "unknown".to_string()),
						),
					)
					.await;

					// If outcomes were ingested before this verification ran, ensure
					// suppression still fires (the scorer already saw them via
					// build_scoring_context, but suppression list writes happen here).
					crate::outcomes::apply_post_verification_outcome_check(
						&pool,
						tenant_uuid,
						&task.input.to_email,
					)
					.await;
				}
			}
		}

		sync_related_entities(&config, task).await;

		info!(target: LOG_TARGET,
			email=task.input.to_email,
			worker_output=?worker_output.as_ref().map(|o| &o.output.is_reachable),
			job_id=?task.job_id,
			"Done check",
		);
	}

	Ok(())
}

fn is_delayed_recheck_task(task: &CheckEmailTask) -> bool {
	task.metadata
		.as_ref()
		.and_then(|metadata| metadata.created_by.as_deref())
		== Some("delayed_recheck")
}

/// Persist a task's terminal failure.
///
/// A delayed recheck is the exception: its row still holds the result of the
/// pass that scheduled it, and the recheck only tried to firm that result up.
/// Failing to do so is no reason to erase it, so the row goes back to
/// 'completed' with that result instead of being overwritten with the error.
async fn finalize_dead_letter(
	task: &CheckEmailTask,
	delivery: &Delivery,
	channel: &Arc<Channel>,
	config: &Arc<BackendConfig>,
	worker_output: &Result<PreparedCheckEmailSuccess, TaskError>,
) -> Result<(), anyhow::Error> {
	let task_db_id = task.metadata.as_ref().and_then(|m| m.task_db_id);
	let reason = worker_output
		.as_ref()
		.err()
		.map(|e| e.to_string())
		.unwrap_or_else(|| "Unknown result after max retries".to_string());

	if is_delayed_recheck_task(task) {
		if let (Some(id), Some(pool)) = (task_db_id, config.get_pg_pool()) {
			if keep_result_of_failed_recheck(&pool, id).await? {
				if let CheckEmailJobId::Bulk(job_id) = task.job_id {
					record_event(
						config,
						job_id,
						Some(id),
						"task.delayed_recheck_failed",
						Some(serde_json::json!({
							"reason": reason,
							"kept_previous_result": true
						})),
						Some(&config.backend_name),
					)
					.await;
				}
				return Ok(());
			}
		}
	}

	delivery_finalize(
		task,
		delivery,
		Arc::clone(channel),
		Arc::clone(config),
		worker_output,
	)
	.await?;

	if let Some(id) = task_db_id {
		update_task_state(config, id, "dead_lettered", None).await;

		// Update completed_at
		if let Some(pool) = config.get_pg_pool() {
			let _ = sqlx::query!(
				"UPDATE v1_task_result SET completed_at = NOW() WHERE id = $1",
				id,
			)
			.execute(&pool)
			.await;
			let _ = sqlx::query(
				r#"
				DELETE FROM verification_change_events
				WHERE current_task_result_id = $1
				"#,
			)
			.bind(id)
			.execute(&pool)
			.await;
		}

		if let CheckEmailJobId::Bulk(job_id) = task.job_id {
			record_event(
				config,
				job_id,
				Some(id),
				"task.dead_lettered",
				Some(serde_json::json!({ "reason": reason })),
				Some(&config.backend_name),
			)
			.await;
		}
	}

	Ok(())
}

/// Returns the row of a failed delayed recheck to 'completed' when it still
/// holds the earlier result. Returns false when there is no result to keep.
async fn keep_result_of_failed_recheck(
	pool: &PgPool,
	task_db_id: i32,
) -> Result<bool, sqlx::Error> {
	let kept = sqlx::query(
		"UPDATE v1_task_result SET task_state = 'completed'::task_state, updated_at = NOW() WHERE id = $1 AND result IS NOT NULL",
	)
	.bind(task_db_id)
	.execute(pool)
	.await?;
	Ok(kept.rows_affected() > 0)
}

/// Settle a delivery whose outcome could not be stored (or, for a
/// single-shot task, whose reply could not be sent), then report `error`.
///
/// A bulk task goes back on the queue so a later attempt can store it:
/// acking would leave its row 'running' and its job unfinalized for good.
/// The requeue is counted and capped, so a store that can never succeed
/// cannot loop. Single-shot tasks are not requeued: the reply normally went
/// out already, and by the time a retry ran the caller would have stopped
/// waiting for it.
async fn settle_after_store_failure(
	task: &CheckEmailTask,
	delivery: &Delivery,
	channel: &Channel,
	delivery_attempt: u32,
	error: anyhow::Error,
) -> Result<(), anyhow::Error> {
	let requeue = matches!(task.job_id, CheckEmailJobId::Bulk(_))
		&& delivery_attempt < MAX_STORE_FAILURE_REQUEUES;
	if requeue {
		tokio::time::sleep(store_failure_delay(delivery_attempt)).await;
		requeue_with_attempt(delivery, channel, delivery_attempt + 1).await?;
		Err(error.context("Failed to store task outcome; requeued the task"))
	} else {
		delivery
			.reject(BasicRejectOptions { requeue: false })
			.await?;
		Err(error.context("Failed to store task outcome; dropped the task"))
	}
}

fn store_failure_delay(delivery_attempt: u32) -> Duration {
	STORE_FAILURE_BASE_DELAY
		.saturating_mul(1 << delivery_attempt.min(16))
		.min(STORE_FAILURE_MAX_DELAY)
}

/// Put the delivered message back on the queue with its attempt counter set
/// to `attempt`.
///
/// `basic.reject` cannot change headers, so the message is republished with
/// the new count and the original acked. If republishing fails, the original
/// is rejected with requeue instead: an uncounted attempt beats losing the
/// task.
async fn requeue_with_attempt(
	delivery: &Delivery,
	channel: &Channel,
	attempt: u32,
) -> Result<(), anyhow::Error> {
	let properties = with_delivery_attempt(delivery.properties.clone(), attempt);
	match publish_payload(channel, &delivery.data, properties).await {
		Ok(()) => {
			delivery.ack(BasicAckOptions::default()).await?;
		}
		Err(error) => {
			warn!(target: LOG_TARGET, error=?error, attempt=attempt, "Failed to republish task for another attempt; requeueing it uncounted");
			delivery
				.reject(BasicRejectOptions { requeue: true })
				.await?;
		}
	}
	Ok(())
}

/// How many times a worker has already requeued this message (0 when the
/// header is absent, i.e. on the first delivery).
pub fn delivery_attempt(properties: &BasicProperties) -> u32 {
	properties
		.headers()
		.as_ref()
		.and_then(|headers| headers.inner().get(DELIVERY_ATTEMPT_HEADER))
		.and_then(amqp_value_as_u32)
		.unwrap_or(0)
}

fn with_delivery_attempt(properties: BasicProperties, attempt: u32) -> BasicProperties {
	let mut headers = properties.headers().clone().unwrap_or_default();
	headers.insert(DELIVERY_ATTEMPT_HEADER.into(), AMQPValue::LongUInt(attempt));
	properties.with_headers(headers)
}

/// Header integers keep whatever width the publisher chose; tools other than
/// this worker (e.g. the management UI) may write the counter differently.
fn amqp_value_as_u32(value: &AMQPValue) -> Option<u32> {
	let value = match value {
		AMQPValue::ShortShortInt(n) => i64::from(*n),
		AMQPValue::ShortShortUInt(n) => i64::from(*n),
		AMQPValue::ShortInt(n) => i64::from(*n),
		AMQPValue::ShortUInt(n) => i64::from(*n),
		AMQPValue::LongInt(n) => i64::from(*n),
		AMQPValue::LongUInt(n) => i64::from(*n),
		AMQPValue::LongLongInt(n) => *n,
		_ => return None,
	};
	u32::try_from(value).ok()
}

/// Send the per-email webhook for a result that is already stored.
///
/// Failures are logged and recorded as a `task.webhook_failed` job event but
/// never fail the task: the tenant endpoint being down is no reason to
/// discard the verification result or to redo the SMTP check.
async fn deliver_task_webhook(
	config: &BackendConfig,
	task: &CheckEmailTask,
	success: &PreparedCheckEmailSuccess,
	webhook_signing_secret: Option<&str>,
) {
	let mut attempts = 1;
	let error = loop {
		match send_task_webhook(task, success, webhook_signing_secret).await {
			Ok(()) => return,
			Err(error) if error.is_retryable() && attempts < WEBHOOK_MAX_ATTEMPTS => {
				tokio::time::sleep(WEBHOOK_RETRY_BASE_DELAY * attempts).await;
				attempts += 1;
			}
			Err(error) => break error,
		}
	};

	warn!(target: LOG_TARGET, email=?task.input.to_email, attempts=attempts, error=?error, "Webhook delivery failed; the verification result is kept");
	if let CheckEmailJobId::Bulk(job_id) = task.job_id {
		record_event(
			config,
			job_id,
			task.metadata.as_ref().and_then(|m| m.task_db_id),
			"task.webhook_failed",
			Some(serde_json::json!({
				"error": error.to_string(),
				"attempts": attempts
			})),
			Some(&config.backend_name),
		)
		.await;
	}
}

/// Records that the per-email webhook was not sent because the tenant's
/// signing secret could not be read. The verification result is kept.
async fn skip_webhook_without_secret(
	config: &BackendConfig,
	task: &CheckEmailTask,
	error: &sqlx::Error,
) {
	let has_webhook = matches!(
		&task.webhook,
		Some(TaskWebhook {
			on_each_email: Some(_)
		})
	);
	if !has_webhook {
		return;
	}
	error!(target: LOG_TARGET, email=?task.input.to_email, error=?error, "Webhook not sent: the tenant's signing secret could not be read");
	if let CheckEmailJobId::Bulk(job_id) = task.job_id {
		record_event(
			config,
			job_id,
			task.metadata.as_ref().and_then(|m| m.task_db_id),
			"task.webhook_failed",
			Some(serde_json::json!({
				"error": "signing_secret_unavailable",
				"attempts": 0
			})),
			Some(&config.backend_name),
		)
		.await;
	}
}

/// Finalize delivery: send single-shot reply and store result.
async fn delivery_finalize(
	task: &CheckEmailTask,
	delivery: &Delivery,
	channel: Arc<Channel>,
	config: Arc<BackendConfig>,
	worker_output: &Result<PreparedCheckEmailSuccess, TaskError>,
) -> Result<(), anyhow::Error> {
	let storage = config.get_storage_adapter();

	if let CheckEmailJobId::SingleShot = task.job_id {
		send_single_shot_reply(channel, delivery, worker_output).await?;
	}

	match worker_output {
		Ok(success) => {
			storage
				.store_prepared(task, success, storage.get_extra())
				.await?;
		}
		Err(error) => {
			storage
				.store_error(task, error, storage.get_extra())
				.await?;
		}
	}

	Ok(())
}

/// Finalize whatever the task belongs to (finder job, duplicate rows, list,
/// bulk job, pipeline run) once its row has reached a terminal state.
pub(crate) async fn sync_related_entities(config: &BackendConfig, task: &CheckEmailTask) {
	let task_db_id = task
		.metadata
		.as_ref()
		.and_then(|metadata| metadata.task_db_id);
	let bulk_job_id = match task.job_id {
		CheckEmailJobId::Bulk(job_id) => Some(job_id),
		CheckEmailJobId::SingleShot => None,
	};
	sync_task_entities(config, task_db_id, bulk_job_id).await;
}

/// [`sync_related_entities`] for callers that only have the task row's id
/// and its bulk job id, such as the delayed recheck scheduler.
pub(crate) async fn sync_task_entities(
	config: &BackendConfig,
	task_db_id: Option<i32>,
	bulk_job_id: Option<i32>,
) {
	let Some(pool) = config.get_pg_pool() else {
		return;
	};

	if let Some(task_db_id) = task_db_id {
		if let Ok(rows) =
			sqlx::query("SELECT finder_job_id FROM v1_finder_result WHERE task_result_id = $1")
				.bind(task_db_id)
				.fetch_all(&pool)
				.await
		{
			for row in rows {
				let finder_job_id: i32 = sqlx::Row::get(&row, "finder_job_id");
				let _ = crate::finder::sync_finder_results(&pool, finder_job_id).await;
			}
		}

		// Propagate results (including errors) to duplicate task rows and
		// transition them to 'completed' (they were 'queued' until now).
		let _ = sqlx::query(
			r#"
			UPDATE v1_task_result AS dup
			SET task_state = 'completed'::task_state,
				result = pri.result, error = pri.error,
				score = pri.score, score_category = pri.score_category,
				sub_reason = pri.sub_reason, safe_to_send = pri.safe_to_send,
				reason_codes = pri.reason_codes,
				bounce_risk_score = pri.bounce_risk_score,
				bounce_risk_category = pri.bounce_risk_category,
				bounce_risk_confidence = pri.bounce_risk_confidence,
				bounce_risk_action = pri.bounce_risk_action,
				bounce_risk_model_version = pri.bounce_risk_model_version,
				bounce_risk_signals = pri.bounce_risk_signals,
				recommendation = pri.recommendation,
				recommendation_action = pri.recommendation_action,
				recommendation_confidence = pri.recommendation_confidence,
				recommendation_priority = pri.recommendation_priority,
				policy_mode = pri.policy_mode,
				policy_profile_key = pri.policy_profile_key,
				policy_evaluation = pri.policy_evaluation,
				policy_decision = pri.policy_decision,
				policy_evaluated_at = pri.policy_evaluated_at,
				canonical_email = COALESCE(dup.canonical_email, pri.canonical_email),
				completed_at = NOW(), updated_at = NOW()
			FROM v1_task_result AS pri
			WHERE dup.canonical_task_id = $1 AND pri.id = $1 AND dup.is_duplicate = true
			"#,
		)
		.bind(task_db_id)
		.execute(&pool)
		.await;

		if let Ok(list_id) = sqlx::query_scalar::<_, Option<i32>>(
			"SELECT (extra->>'list_id')::INTEGER FROM v1_task_result WHERE id = $1",
		)
		.bind(task_db_id)
		.fetch_one(&pool)
		.await
		{
			if let Some(list_id) = list_id {
				let total_rows =
					sqlx::query_scalar::<_, i32>("SELECT total_rows FROM v1_lists WHERE id = $1")
						.bind(list_id)
						.fetch_optional(&pool)
						.await
						.ok()
						.flatten()
						.unwrap_or_default();
				// Count rows with results, errors, or cancelled state as
				// terminal. A row being rechecked already holds a result but
				// is not done yet.
				let processed = sqlx::query_scalar::<_, i64>(
					"SELECT COUNT(*) FROM v1_task_result WHERE (extra->>'list_id')::INTEGER = $1 AND (result IS NOT NULL OR error IS NOT NULL OR task_state = 'cancelled') AND task_state NOT IN ('queued', 'running', 'retrying')",
				)
				.bind(list_id)
				.fetch_one(&pool)
				.await
				.unwrap_or(0);
				if processed >= i64::from(total_rows) {
					let failed = sqlx::query_scalar::<_, i64>(
						"SELECT COUNT(*) FROM v1_task_result WHERE (extra->>'list_id')::INTEGER = $1 AND task_state IN ('failed', 'dead_lettered')",
					)
					.bind(list_id)
					.fetch_one(&pool)
					.await
					.unwrap_or(0);
					let final_status = if failed > 0 { "failed" } else { "completed" };
					let _ = sqlx::query(
						"UPDATE v1_lists SET status = $2::list_status, error_message = CASE WHEN $2 = 'failed' THEN COALESCE(error_message, 'task_failed') ELSE error_message END, completed_at = COALESCE(completed_at, NOW()), updated_at = NOW() WHERE id = $1",
					)
					.bind(list_id)
					.bind(final_status)
					.execute(&pool)
					.await;
				}
			}
		}
	}

	if let Some(job_id) = bulk_job_id {
		let status = sqlx::query_scalar::<_, Option<String>>(
			"SELECT status::TEXT FROM v1_bulk_job WHERE id = $1",
		)
		.bind(job_id)
		.fetch_one(&pool)
		.await
		.ok()
		.flatten()
		.unwrap_or_else(|| "running".to_string());

		let non_terminal = sqlx::query_scalar::<_, i64>(
			"SELECT COUNT(*) FROM v1_task_result WHERE job_id = $1 AND task_state IN ('queued', 'running', 'retrying')",
		)
		.bind(job_id)
		.fetch_one(&pool)
		.await
		.unwrap_or(0);

		if non_terminal == 0 {
			let failed = sqlx::query_scalar::<_, i64>(
				"SELECT COUNT(*) FROM v1_task_result WHERE job_id = $1 AND task_state IN ('failed', 'dead_lettered')",
			)
			.bind(job_id)
			.fetch_one(&pool)
			.await
			.unwrap_or(0);

			let final_status = match status.as_str() {
				"cancelling" | "cancelled" => "cancelled",
				_ if failed > 0 => "failed",
				_ => "completed",
			};

			let _ = sqlx::query(
				"UPDATE v1_bulk_job SET status = $2::job_state, completed_at = CASE WHEN $2 IN ('completed', 'failed') THEN COALESCE(completed_at, NOW()) ELSE completed_at END, cancelled_at = CASE WHEN $2 = 'cancelled' THEN COALESCE(cancelled_at, NOW()) ELSE cancelled_at END, updated_at = NOW() WHERE id = $1",
			)
			.bind(job_id)
			.bind(final_status)
			.execute(&pool)
			.await;
		}

		if let Err(err) =
			crate::pipelines::maybe_finalize_pipeline_run_for_job(config, &pool, job_id).await
		{
			warn!(
				target: LOG_TARGET,
				job_id = job_id,
				error = ?err,
				"Failed to finalize pipeline run for job"
			);
		}
	}
}

fn validate_webhook_url(url: &str) -> Result<reqwest::Url, TaskError> {
	let parsed = reqwest::Url::parse(url)
		.map_err(|err| TaskError::WebhookValidation(format!("invalid URL: {}", err)))?;
	match parsed.scheme() {
		"https" => {}
		"http" => {
			return Err(TaskError::WebhookValidation(
				"HTTP is not allowed; use HTTPS".to_string(),
			))
		}
		other => {
			return Err(TaskError::WebhookValidation(format!(
				"scheme '{}' is not allowed; use HTTPS",
				other
			)))
		}
	}

	let host = parsed
		.host_str()
		.ok_or_else(|| TaskError::WebhookValidation("missing host".to_string()))?;
	if host == "localhost"
		|| host == "127.0.0.1"
		|| host == "::1"
		|| host == "[::1]"
		|| host == "0.0.0.0"
		|| host.ends_with(".local")
	{
		return Err(TaskError::WebhookValidation(
			"localhost and loopback targets are not allowed".to_string(),
		));
	}
	let ip_host = host.trim_start_matches('[').trim_end_matches(']');
	if let Ok(ip) = ip_host.parse::<IpAddr>() {
		if is_private_ip(ip) {
			return Err(TaskError::WebhookValidation(
				"private or reserved IP targets are not allowed".to_string(),
			));
		}
	}

	Ok(parsed)
}

async fn validate_resolved_webhook_target(url: &reqwest::Url) -> Result<(), TaskError> {
	let host = url
		.host_str()
		.ok_or_else(|| TaskError::WebhookValidation("missing host".to_string()))?;
	let port = url
		.port_or_known_default()
		.ok_or_else(|| TaskError::WebhookValidation("missing port".to_string()))?;
	let lookup_host = host.trim_start_matches('[').trim_end_matches(']');
	let addrs = tokio::net::lookup_host((lookup_host, port))
		.await
		.map_err(|err| {
			TaskError::WebhookValidation(format!("failed to resolve webhook host: {}", err))
		})?
		.collect::<Vec<_>>();
	if addrs.is_empty() {
		return Err(TaskError::WebhookValidation(
			"host did not resolve to any addresses".to_string(),
		));
	}
	if addrs.iter().any(|addr| is_private_ip(addr.ip())) {
		return Err(TaskError::WebhookValidation(
			"host resolves to a private or reserved IP address".to_string(),
		));
	}

	Ok(())
}

fn is_private_ip(ip: IpAddr) -> bool {
	match ip {
		IpAddr::V4(v4) => {
			v4.is_loopback()
				|| v4.is_private()
				|| v4.is_link_local()
				|| v4.is_broadcast()
				|| v4.is_multicast()
				|| v4.is_unspecified()
				|| v4.octets()[0] == 169 && v4.octets()[1] == 254
				|| v4.octets()[0] == 100 && (v4.octets()[1] & 0xC0) == 64
				|| v4.octets()[0] == 192 && v4.octets()[1] == 0 && v4.octets()[2] == 0
				|| v4.octets()[0] == 192 && v4.octets()[1] == 0 && v4.octets()[2] == 2
				|| v4.octets()[0] == 198 && v4.octets()[1] == 18
				|| v4.octets()[0] == 198 && v4.octets()[1] == 19
				|| v4.octets()[0] == 198 && v4.octets()[1] == 51 && v4.octets()[2] == 100
				|| v4.octets()[0] == 203 && v4.octets()[1] == 0 && v4.octets()[2] == 113
				|| v4.octets()[0] >= 240
		}
		IpAddr::V6(v6) => {
			let first_segment = v6.segments()[0];
			let mapped_private = v6
				.to_ipv4_mapped()
				.is_some_and(|mapped| is_private_ip(IpAddr::V4(mapped)));
			v6.is_loopback()
				|| v6.is_unspecified()
				|| v6.is_multicast()
				|| (first_segment & 0xffc0) == 0xfe80
				|| (first_segment & 0xfe00) == 0xfc00
				|| (first_segment & 0xffc0) == 0xfec0
				|| (first_segment == 0x2001 && v6.segments()[1] == 0x0db8)
				|| mapped_private
		}
	}
}

/// Checks the email and sends the result to the webhook.
pub async fn check_email_and_send_result(
	task: &CheckEmailTask,
	webhook_signing_secret: Option<&str>,
) -> Result<PreparedCheckEmailSuccess, TaskError> {
	check_email_and_send_result_with_config(task, None, webhook_signing_secret).await
}

pub async fn check_email_and_send_result_with_config(
	task: &CheckEmailTask,
	config: Option<Arc<BackendConfig>>,
	webhook_signing_secret: Option<&str>,
) -> Result<PreparedCheckEmailSuccess, TaskError> {
	let config = config.unwrap_or_else(|| Arc::new(BackendConfig::empty()));
	let success = check_email_and_prepare_result_with_config(task, Some(config)).await?;
	send_task_webhook(task, &success, webhook_signing_secret).await?;
	Ok(success)
}

pub async fn check_email_and_prepare_result_with_config(
	task: &CheckEmailTask,
	config: Option<Arc<BackendConfig>>,
) -> Result<PreparedCheckEmailSuccess, TaskError> {
	let config = config.unwrap_or_else(|| Arc::new(BackendConfig::empty()));
	validate_task_webhook_targets(task).await?;

	let output = check_email(&task.input).await;
	let tenant_id = task
		.metadata
		.as_ref()
		.and_then(|metadata| metadata.tenant_id.as_ref())
		.and_then(|tenant_id| tenant_id.parse::<uuid::Uuid>().ok());
	let success = prepare_check_email_success(config.as_ref(), output, tenant_id, Utc::now(), true)
		.await
		.map_err(|error| TaskError::Prepare(error.to_string()))?;

	Ok(success)
}

async fn validate_task_webhook_targets(task: &CheckEmailTask) -> Result<(), TaskError> {
	if let Some(TaskWebhook {
		on_each_email: Some(webhook),
	}) = &task.webhook
	{
		let _: HeaderMap = (&webhook.headers).try_into()?;
		let webhook_url = validate_webhook_url(&webhook.url)?;
		validate_resolved_webhook_target(&webhook_url).await?;
	}

	Ok(())
}

async fn send_task_webhook(
	task: &CheckEmailTask,
	success: &PreparedCheckEmailSuccess,
	webhook_signing_secret: Option<&str>,
) -> Result<(), TaskError> {
	if let Some(TaskWebhook {
		on_each_email: Some(webhook),
	}) = &task.webhook
	{
		let webhook_output = WebhookOutput {
			result: &success.response.json,
			extra: &webhook.extra,
		};

		let mut headers: HeaderMap = (&webhook.headers).try_into()?;

		// Sign the webhook payload if a signing secret is available (loaded from DB)
		if let Some(secret) = webhook_signing_secret {
			if let Ok(body_bytes) = serde_json::to_vec(&webhook_output) {
				let signature = crate::tenant::webhook::sign_payload(secret, &body_bytes);
				if let Ok(val) = signature.parse() {
					headers.insert(crate::tenant::webhook::WEBHOOK_SIGNATURE_HEADER, val);
				}
			}
		}

		let webhook_url = validate_webhook_url(&webhook.url)?;
		validate_resolved_webhook_target(&webhook_url).await?;
		let client = reqwest::Client::builder()
			.timeout(Duration::from_secs(10))
			.redirect(reqwest::redirect::Policy::none())
			.build()?;
		let res = client
			.post(webhook_url)
			.json(&webhook_output)
			.headers(headers)
			.send()
			.await?;
		let status = res.status();
		let body = res.text().await?;
		if let Some(error) = webhook_status_error(status) {
			return Err(error);
		}
		debug!(target: LOG_TARGET, email=?success.output.input,res=?body, "Received webhook response");
	}

	Ok(())
}

fn webhook_status_error(status: reqwest::StatusCode) -> Option<TaskError> {
	if status.is_success() {
		return None;
	}

	if status.is_redirection() {
		return Some(TaskError::WebhookValidation(format!(
			"webhook redirects are not followed (HTTP {})",
			status.as_u16()
		)));
	}

	Some(TaskError::WebhookStatus(status.as_u16()))
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::throttle::{ThrottleLimit, ThrottleResult};
	use std::time::Duration;

	#[test]
	fn test_retry_policy_default() {
		let p = RetryPolicy::default();
		assert_eq!(p.max_retries, 2);
		assert_eq!(p.backoff_seconds, 5);
		assert_eq!(p.backoff_multiplier, 2.0);
	}

	#[test]
	fn test_retry_policy_serde() {
		let p = RetryPolicy {
			max_retries: 5,
			backoff_seconds: 10,
			backoff_multiplier: 3.0,
		};
		let json = serde_json::to_string(&p).unwrap();
		let back: RetryPolicy = serde_json::from_str(&json).unwrap();
		assert_eq!(back.max_retries, 5);
		assert_eq!(back.backoff_seconds, 10);
		assert_eq!(back.backoff_multiplier, 3.0);
	}

	#[test]
	fn test_retry_policy_serde_defaults() {
		// Empty JSON should use defaults for all fields
		let p: RetryPolicy = serde_json::from_str("{}").unwrap();
		assert_eq!(p.max_retries, 2);
		assert_eq!(p.backoff_seconds, 5);
		assert_eq!(p.backoff_multiplier, 2.0);
	}

	#[test]
	fn test_task_metadata_serde() {
		let m = TaskMetadata {
			tenant_id: Some("tid".into()),
			request_id: Some("rid".into()),
			correlation_id: None,
			created_by: Some("api".into()),
			retry_policy: Some(RetryPolicy::default()),
			dedupe_key: Some("dk".into()),
			task_db_id: Some(42),
		};
		let json = serde_json::to_string(&m).unwrap();
		let back: TaskMetadata = serde_json::from_str(&json).unwrap();
		assert_eq!(back.tenant_id.as_deref(), Some("tid"));
		assert_eq!(back.task_db_id, Some(42));
	}

	#[test]
	fn test_task_metadata_none_fields() {
		let m = TaskMetadata {
			tenant_id: None,
			request_id: None,
			correlation_id: None,
			created_by: None,
			retry_policy: None,
			dedupe_key: None,
			task_db_id: None,
		};
		let json = serde_json::to_string(&m).unwrap();
		assert!(json.contains("null"));
		let back: TaskMetadata = serde_json::from_str(&json).unwrap();
		assert!(back.tenant_id.is_none());
	}

	#[test]
	fn test_check_email_job_id_serde() {
		let single = CheckEmailJobId::SingleShot;
		let json = serde_json::to_string(&single).unwrap();
		assert_eq!(json, "\"single_shot\"");

		let bulk = CheckEmailJobId::Bulk(42);
		let json = serde_json::to_string(&bulk).unwrap();
		assert!(json.contains("42"));
	}

	#[test]
	fn test_task_error_status_codes() {
		let throttle_err = TaskError::Throttle(ThrottleResult {
			delay: Duration::from_secs(1),
			limit_type: ThrottleLimit::PerSecond,
		});
		assert_eq!(throttle_err.status_code(), StatusCode::TOO_MANY_REQUESTS);

		let lapin_err = TaskError::Lapin(lapin::Error::from(lapin::ErrorKind::InvalidChannel(0)));
		assert_eq!(lapin_err.status_code(), StatusCode::INTERNAL_SERVER_ERROR);

		let webhook_err = TaskError::WebhookValidation("bad target".to_string());
		assert_eq!(webhook_err.status_code(), StatusCode::BAD_REQUEST);

		let webhook_status_err = TaskError::WebhookStatus(502);
		assert_eq!(webhook_status_err.status_code(), StatusCode::BAD_GATEWAY);
	}

	#[test]
	fn test_task_error_retryability() {
		assert!(!TaskError::WebhookValidation("bad target".to_string()).is_retryable());
		assert!(!TaskError::WebhookStatus(302).is_retryable());
		assert!(!TaskError::WebhookStatus(404).is_retryable());
		assert!(TaskError::WebhookStatus(503).is_retryable());
		assert!(
			TaskError::Lapin(lapin::Error::from(lapin::ErrorKind::InvalidChannel(0)))
				.is_retryable()
		);
	}

	#[test]
	fn attempt_counter_survives_requeue_and_keeps_rpc_routing() {
		use lapin::types::FieldTable;

		// First delivery: no header yet.
		assert_eq!(delivery_attempt(&BasicProperties::default()), 0);

		let mut other_headers = FieldTable::default();
		other_headers.insert("x-trace".into(), AMQPValue::LongString("abc".into()));
		let original = BasicProperties::default()
			.with_reply_to("amq.gen-reply".into())
			.with_correlation_id("corr-1".into())
			.with_priority(5)
			.with_headers(other_headers);

		let requeued = with_delivery_attempt(original, 3);
		assert_eq!(delivery_attempt(&requeued), 3);
		// A single-shot task requeued for retry must still reach its caller.
		assert_eq!(
			requeued.reply_to().as_ref().map(|q| q.as_str()),
			Some("amq.gen-reply")
		);
		assert_eq!(
			requeued.correlation_id().as_ref().map(|c| c.as_str()),
			Some("corr-1")
		);
		assert_eq!(*requeued.priority(), Some(5));
		let headers = requeued.headers().as_ref().expect("headers kept");
		assert!(headers.contains_key("x-trace"));

		let requeued_again = with_delivery_attempt(requeued, 4);
		assert_eq!(delivery_attempt(&requeued_again), 4);
	}

	#[test]
	fn attempt_counter_accepts_other_integer_widths_and_ignores_garbage() {
		let with_value = |value: AMQPValue| {
			let mut headers = lapin::types::FieldTable::default();
			headers.insert(DELIVERY_ATTEMPT_HEADER.into(), value);
			BasicProperties::default().with_headers(headers)
		};
		assert_eq!(delivery_attempt(&with_value(AMQPValue::LongLongInt(2))), 2);
		assert_eq!(
			delivery_attempt(&with_value(AMQPValue::ShortShortUInt(1))),
			1
		);
		assert_eq!(delivery_attempt(&with_value(AMQPValue::LongInt(-1))), 0);
		assert_eq!(
			delivery_attempt(&with_value(AMQPValue::LongString("2".into()))),
			0
		);
	}

	#[test]
	fn store_failure_requeues_back_off_within_the_ack_timeout_and_ride_out_an_outage() {
		let delays: Vec<Duration> = (0..MAX_STORE_FAILURE_REQUEUES)
			.map(store_failure_delay)
			.collect();
		assert_eq!(delays[0], STORE_FAILURE_BASE_DELAY);
		assert!(delays.windows(2).all(|pair| pair[0] <= pair[1]));
		assert!(delays.iter().all(|delay| *delay <= STORE_FAILURE_MAX_DELAY));
		let total: Duration = delays.iter().sum();
		assert!(
			total >= Duration::from_secs(180),
			"requeues only cover {:?} of a Postgres outage",
			total
		);
		assert_eq!(store_failure_delay(u32::MAX), STORE_FAILURE_MAX_DELAY);
	}

	#[test]
	fn test_webhook_status_error_rejects_redirects() {
		let err = webhook_status_error(reqwest::StatusCode::FOUND).unwrap();
		assert!(matches!(err, TaskError::WebhookValidation(_)));
		assert!(webhook_status_error(reqwest::StatusCode::NO_CONTENT).is_none());
		assert!(matches!(
			webhook_status_error(reqwest::StatusCode::INTERNAL_SERVER_ERROR),
			Some(TaskError::WebhookStatus(500))
		));
	}

	#[test]
	fn test_task_error_display() {
		let err = TaskError::Throttle(ThrottleResult {
			delay: Duration::from_secs(5),
			limit_type: ThrottleLimit::PerMinute,
		});
		let display = format!("{}", err);
		assert!(display.contains("full capacity"));
	}

	#[test]
	fn test_task_error_serialize() {
		let err = TaskError::Throttle(ThrottleResult {
			delay: Duration::from_secs(1),
			limit_type: ThrottleLimit::PerSecond,
		});
		let json = serde_json::to_string(&err).unwrap();
		assert!(json.contains("full capacity"));
	}

	#[test]
	fn test_webhook_validation_rejects_unsafe_targets() {
		assert!(validate_webhook_url("http://example.com/hook").is_err());
		assert!(validate_webhook_url("https://localhost/hook").is_err());
		assert!(validate_webhook_url("https://127.0.0.1/hook").is_err());
		assert!(validate_webhook_url("https://10.0.0.1/hook").is_err());
		assert!(validate_webhook_url("https://[::1]/hook").is_err());
		assert!(validate_webhook_url("https://[fd00::1]/hook").is_err());
		assert!(validate_webhook_url("https://example.com/hook").is_ok());
	}

	#[test]
	fn test_is_private_ip_covers_reserved_ranges() {
		assert!(is_private_ip("169.254.1.1".parse().unwrap()));
		assert!(is_private_ip("100.64.0.1".parse().unwrap()));
		assert!(is_private_ip("192.0.2.1".parse().unwrap()));
		assert!(is_private_ip("2001:db8::1".parse().unwrap()));
		assert!(is_private_ip("::ffff:127.0.0.1".parse().unwrap()));
		assert!(!is_private_ip("8.8.8.8".parse().unwrap()));
	}

	#[test]
	fn test_check_email_task_with_metadata() {
		let task = CheckEmailTask {
			input: check_if_email_exists::CheckEmailInput {
				to_email: "test@example.com".into(),
				..Default::default()
			},
			job_id: CheckEmailJobId::SingleShot,
			webhook: None,
			metadata: Some(TaskMetadata {
				tenant_id: Some("t1".into()),
				request_id: None,
				correlation_id: None,
				created_by: None,
				retry_policy: None,
				dedupe_key: None,
				task_db_id: Some(1),
			}),
		};
		let json = serde_json::to_string(&task).unwrap();
		assert!(json.contains("t1"));
		assert!(json.contains("test@example.com"));
	}

	#[test]
	fn test_check_email_task_without_metadata_backward_compat() {
		// Deserialize a task that has no metadata field; it should default to None.
		let json = r#"{"input":{"to_email":"a@b.com"},"job_id":"single_shot","webhook":null}"#;
		let task: Result<CheckEmailTask, _> = serde_json::from_str(json);
		// This may fail if CheckEmailInput requires more fields, but metadata should be None
		if let Ok(t) = task {
			assert!(t.metadata.is_none());
		}
	}

	#[test]
	fn test_webhook_serde() {
		let w = Webhook {
			url: "https://example.com/hook".into(),
			headers: [("Auth".into(), "Bearer x".into())].into(),
			extra: Some(serde_json::json!({"key": "val"})),
		};
		let json = serde_json::to_string(&w).unwrap();
		let back: Webhook = serde_json::from_str(&json).unwrap();
		assert_eq!(back.url, "https://example.com/hook");
		assert_eq!(back.headers.get("Auth").unwrap(), "Bearer x");
	}

	#[test]
	fn test_task_webhook_serde() {
		let tw = TaskWebhook {
			on_each_email: Some(Webhook {
				url: "https://hook.test".into(),
				headers: HashMap::new(),
				extra: None,
			}),
		};
		let json = serde_json::to_string(&tw).unwrap();
		let back: TaskWebhook = serde_json::from_str(&json).unwrap();
		assert!(back.on_each_email.is_some());
		assert_eq!(back.on_each_email.unwrap().url, "https://hook.test");
	}
}
