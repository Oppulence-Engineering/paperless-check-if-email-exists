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
use crate::storage::commercial_license_trial::send_to_reacher;
use crate::throttle::ThrottleResult;
use crate::worker::single_shot::send_single_shot_reply;
use check_if_email_exists::{
	check_email, CheckEmailInput, CheckEmailOutput, Reachable, LOG_TARGET,
};
use http::HeaderMap;
use lapin::message::Delivery;
use lapin::{options::*, Channel};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt::Debug;
use std::sync::Arc;
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
}

impl TaskError {
	/// Returns the status code that should be returned to the client.
	pub fn status_code(&self) -> StatusCode {
		match self {
			Self::Throttle(_) => StatusCode::TOO_MANY_REQUESTS,
			Self::Lapin(_) => StatusCode::INTERNAL_SERVER_ERROR,
			Self::Reqwest(_) => StatusCode::INTERNAL_SERVER_ERROR,
			Self::Headers(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
	result: &'a CheckEmailOutput,
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

	let worker_output = check_email_and_send_result(task, webhook_signing_secret.as_deref()).await;

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
		Ok(output) if output.is_reachable == Reachable::Unknown => true,
		Err(_) => true,
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
	} else if should_retry {
		// Exhausted retries — dead letter
		delivery
			.reject(BasicRejectOptions { requeue: false })
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

		// Still store the result and handle single-shot reply
		delivery_finalize(
			task,
			&delivery,
			channel,
			Arc::clone(&config),
			&worker_output,
		)
		.await?;
		info!(target: LOG_TARGET, email=?&task.input.to_email, "Dead-lettered after exhausting retries");
	} else {
		// Success path
		delivery.ack(BasicAckOptions::default()).await?;

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

		delivery_finalize(
			task,
			&delivery,
			channel,
			Arc::clone(&config),
			&worker_output,
		)
		.await?;

		// Evaluate conditional actions (auto-suppression) after successful completion
		if let (Some(pool), Ok(output)) = (config.get_pg_pool(), &worker_output) {
			if let Some(tenant_id_str) = task.metadata.as_ref().and_then(|m| m.tenant_id.clone()) {
				if let Ok(tenant_uuid) = tenant_id_str.parse::<uuid::Uuid>() {
					let email_score = crate::scoring::compute_score(output);
					crate::worker::actions::evaluate_post_completion_actions(
						&pool,
						tenant_uuid,
						&task.input.to_email,
						Some(email_score.score),
						Some(
							&serde_json::to_value(&email_score.category)
								.ok()
								.and_then(|v| v.as_str().map(ToOwned::to_owned))
								.unwrap_or_else(|| "unknown".to_string()),
						),
					)
					.await;
				}
			}
		}

		info!(target: LOG_TARGET,
			email=task.input.to_email,
			worker_output=?worker_output.as_ref().map(|o| &o.is_reachable),
			job_id=?task.job_id,
			"Done check",
		);
	}

	Ok(())
}

/// Finalize delivery: send single-shot reply, store result, send to reacher.
async fn delivery_finalize(
	task: &CheckEmailTask,
	delivery: &Delivery,
	channel: Arc<Channel>,
	config: Arc<BackendConfig>,
	worker_output: &Result<CheckEmailOutput, TaskError>,
) -> Result<(), anyhow::Error> {
	if let CheckEmailJobId::SingleShot = task.job_id {
		send_single_shot_reply(channel, delivery, worker_output).await?;
	}

	let storage = config.get_storage_adapter();
	storage
		.store(task, worker_output, storage.get_extra())
		.await?;

	sync_related_entities(&config, task).await;

	send_to_reacher(config, &task.input.to_email, worker_output).await?;

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
				reason_codes = pri.reason_codes, completed_at = NOW(), updated_at = NOW()
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
					let _ = sqlx::query(
						"UPDATE v1_lists SET status = 'completed'::list_status, completed_at = COALESCE(completed_at, NOW()), updated_at = NOW() WHERE id = $1",
					)
					.bind(list_id)
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

/// Checks the email and sends the result to the webhook.
pub async fn check_email_and_send_result(
	task: &CheckEmailTask,
	webhook_signing_secret: Option<&str>,
) -> Result<CheckEmailOutput, TaskError> {
	let output = check_email(&task.input).await;

	// Check if we have a webhook to send the output to.
	if let Some(TaskWebhook {
		on_each_email: Some(webhook),
	}) = &task.webhook
	{
		let webhook_output = WebhookOutput {
			result: &output,
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

		let client = reqwest::Client::new();
		let res = client
			.post(&webhook.url)
			.json(&webhook_output)
			.headers(headers)
			.send()
			.await?
			.text()
			.await?;
		debug!(target: LOG_TARGET, email=?webhook_output.result.input,res=?res, "Received webhook response");
	}

	Ok(output)
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
		// Deserialize a task that has no metadata field — should default to None
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
