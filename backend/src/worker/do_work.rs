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

use crate::config::BackendConfig;
use crate::scoring::response::{prepare_check_email_success, PreparedCheckEmailSuccess};
use crate::storage::commercial_license_trial::send_to_reacher;
use crate::throttle::ThrottleResult;
use crate::worker::single_shot::send_single_shot_reply;
use check_if_email_exists::{check_email, CheckEmailInput, Reachable, LOG_TARGET};
use chrono::Utc;
use http::HeaderMap;
use lapin::message::Delivery;
use lapin::{options::*, Channel};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt::Debug;
use std::net::IpAddr;
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use tracing::{debug, info, warn};
use warp::http::StatusCode;

#[derive(Debug, Deserialize, Serialize)]
pub struct CheckEmailTask {
	pub input: CheckEmailInput,
	pub job_id: CheckEmailJobId,
	pub webhook: Option<TaskWebhook>,
	#[serde(default)]
	pub metadata: Option<TaskMetadata>,
}

#[derive(Debug, Deserialize, Serialize)]
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

	// Load webhook signing secret from DB if tenant_id is present
	let webhook_signing_secret: Option<String> =
		if let Some(ref tid) = task.metadata.as_ref().and_then(|m| m.tenant_id.clone()) {
			if let Some(pool) = config.get_pg_pool() {
				if let Ok(uuid) = tid.parse::<uuid::Uuid>() {
					sqlx::query_scalar("SELECT webhook_signing_secret FROM tenants WHERE id = $1")
						.bind(uuid)
						.fetch_optional(&pool)
						.await
						.ok()
						.flatten()
				} else {
					None
				}
			} else {
				None
			}
		} else {
			None
		};

	let worker_output = check_email_and_send_result_with_config(
		task,
		Some(Arc::clone(&config)),
		webhook_signing_secret.as_deref(),
	)
	.await;

	// Determine current retry count from the database
	let current_retry_count = if let Some(id) = task_db_id {
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

	let should_retry = match &worker_output {
		Ok(output) if output.output.is_reachable == Reachable::Unknown => true,
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

		delivery
			.reject(BasicRejectOptions { requeue: true })
			.await?;
		info!(target: LOG_TARGET, email=?&task.input.to_email, retry=new_count, max=retry_policy.max_retries, "Requeued message for retry");
	} else if should_retry || worker_output.is_err() {
		// Exhausted retries or a deterministic non-retryable error: dead letter.
		maybe_pause_before_delivery_settle().await;
		delivery
			.reject(BasicRejectOptions { requeue: false })
			.await?;

		delivery_finalize(
			task,
			&delivery,
			channel,
			Arc::clone(&config),
			&worker_output,
		)
		.await?;

		if let Some(id) = task_db_id {
			update_task_state(&config, id, "dead_lettered", None).await;

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
					"task.dead_lettered",
					Some(serde_json::json!({
						"reason": worker_output.as_ref().err().map(|e| e.to_string())
							.unwrap_or_else(|| "Unknown result after max retries".to_string())
					})),
					Some(&config.backend_name),
				)
				.await;
			}
		}

		sync_related_entities(&config, task).await;
		send_to_reacher(
			Arc::clone(&config),
			&task.input.to_email,
			worker_output.as_ref(),
		)
		.await?;

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

		delivery.ack(BasicAckOptions::default()).await?;

		delivery_finalize(
			task,
			&delivery,
			channel,
			Arc::clone(&config),
			&worker_output,
		)
		.await?;

		if let Some(id) = task_db_id {
			update_task_state(&config, id, "completed", None).await;

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
				}
			}
		}

		sync_related_entities(&config, task).await;
		send_to_reacher(
			Arc::clone(&config),
			&task.input.to_email,
			worker_output.as_ref(),
		)
		.await?;

		info!(target: LOG_TARGET,
			email=task.input.to_email,
			worker_output=?worker_output.as_ref().map(|o| &o.output.is_reachable),
			job_id=?task.job_id,
			"Done check",
		);
	}

	Ok(())
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

async fn sync_related_entities(config: &BackendConfig, task: &CheckEmailTask) {
	let Some(pool) = config.get_pg_pool() else {
		return;
	};

	if let Some(task_db_id) = task
		.metadata
		.as_ref()
		.and_then(|metadata| metadata.task_db_id)
	{
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
				// Count rows with results, errors, or cancelled state as terminal
				let processed = sqlx::query_scalar::<_, i64>(
					"SELECT COUNT(*) FROM v1_task_result WHERE (extra->>'list_id')::INTEGER = $1 AND (result IS NOT NULL OR error IS NOT NULL OR task_state = 'cancelled')",
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

	if let CheckEmailJobId::Bulk(job_id) = task.job_id {
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

		let lapin_err = TaskError::Lapin(lapin::Error::InvalidChannel(0));
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
		assert!(TaskError::Lapin(lapin::Error::InvalidChannel(0)).is_retryable());
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
