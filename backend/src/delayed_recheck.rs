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
use crate::http::v1::bulk::post::publish_task;
use crate::worker::do_work::{CheckEmailJobId, CheckEmailTask, RetryPolicy, TaskMetadata};
use check_if_email_exists::LOG_TARGET;
use chrono::{Duration as ChronoDuration, Utc};
use lapin::{BasicProperties, Channel};
use serde_json::Value;
use sqlx::{PgPool, Row};
use std::sync::Arc;
use tokio::time::{self, Duration};
use tracing::{debug, info, warn};

const DELAYED_RECHECK_ACTOR: &str = "delayed_recheck";

#[derive(Debug)]
struct DelayedRecheckRow {
	id: i64,
	task_result_id: i32,
	job_id: i32,
	retry_count: i32,
	publish_attempts: i32,
	task: Value,
}

pub fn spawn_delayed_recheck_scheduler(config: Arc<BackendConfig>, pg_pool: PgPool) {
	let poll_interval_seconds = config.delayed_recheck.poll_interval_seconds;
	tokio::spawn(async move {
		let mut interval = time::interval(Duration::from_secs(poll_interval_seconds));
		interval.set_missed_tick_behavior(time::MissedTickBehavior::Delay);

		loop {
			interval.tick().await;
			if let Err(error) = run_delayed_recheck_cycle(Arc::clone(&config), &pg_pool).await {
				warn!(
					target: LOG_TARGET,
					error = ?error,
					"Delayed recheck scheduler cycle failed"
				);
			}
		}
	});
}

pub fn spawn_delayed_recheck_cleanup(config: Arc<BackendConfig>, pg_pool: PgPool) {
	let cleanup_interval_seconds = config.delayed_recheck.cleanup_interval_seconds;
	let retention_days = config.delayed_recheck.retention_days;
	tokio::spawn(async move {
		let mut interval = time::interval(Duration::from_secs(cleanup_interval_seconds));
		interval.set_missed_tick_behavior(time::MissedTickBehavior::Delay);

		loop {
			interval.tick().await;
			if let Err(error) = cleanup_terminal_rechecks(&pg_pool, retention_days).await {
				warn!(
					target: LOG_TARGET,
					error = ?error,
					"Delayed recheck cleanup cycle failed"
				);
			}
		}
	});
}

pub async fn cleanup_terminal_rechecks(
	pg_pool: &PgPool,
	retention_days: i32,
) -> Result<u64, anyhow::Error> {
	let result = sqlx::query(
		r#"
		DELETE FROM verification_delayed_rechecks
		WHERE status IN ('published', 'failed', 'cancelled')
			AND updated_at < NOW() - ($1::INT * INTERVAL '1 day')
		"#,
	)
	.bind(retention_days)
	.execute(pg_pool)
	.await?;

	let deleted = result.rows_affected();
	if deleted > 0 {
		debug!(
			target: LOG_TARGET,
			deleted = deleted,
			retention_days = retention_days,
			"Cleaned up terminal delayed rechecks"
		);
	}
	Ok(deleted)
}

pub async fn schedule_delayed_recheck(
	task: &CheckEmailTask,
	config: Arc<BackendConfig>,
	task_db_id: i32,
	retry_count: i32,
	retry_policy: &RetryPolicy,
) {
	let CheckEmailJobId::Bulk(job_id) = task.job_id else {
		return;
	};
	let Some(pg_pool) = config.get_pg_pool() else {
		warn!(
			target: LOG_TARGET,
			task_id = task_db_id,
			"Cannot schedule delayed recheck without Postgres"
		);
		return;
	};

	if let Err(error) =
		sqlx::query("UPDATE v1_task_result SET retry_count = $2, updated_at = NOW() WHERE id = $1")
			.bind(task_db_id)
			.bind(retry_count)
			.execute(&pg_pool)
			.await
	{
		warn!(
			target: LOG_TARGET,
			task_id = task_db_id,
			error = ?error,
			"Failed to update delayed recheck retry count"
		);
	}

	let delay_seconds = delayed_recheck_delay_seconds(retry_policy, retry_count);
	let run_at = Utc::now() + ChronoDuration::seconds(delay_seconds.min(i64::MAX as u64) as i64);
	let mut recheck_task = task.clone();
	let tenant_id_string = recheck_task
		.metadata
		.as_ref()
		.and_then(|metadata| metadata.tenant_id.as_deref())
		.map(ToOwned::to_owned);
	let tenant_id = tenant_id_string
		.as_deref()
		.and_then(|raw| raw.parse::<uuid::Uuid>().ok());

	match recheck_task.metadata.as_mut() {
		Some(metadata) => {
			metadata.created_by = Some(DELAYED_RECHECK_ACTOR.to_string());
			metadata.task_db_id = Some(task_db_id);
		}
		None => {
			recheck_task.metadata = Some(TaskMetadata {
				tenant_id: tenant_id_string,
				request_id: None,
				correlation_id: None,
				created_by: Some(DELAYED_RECHECK_ACTOR.to_string()),
				retry_policy: Some(retry_policy.clone()),
				dedupe_key: None,
				task_db_id: Some(task_db_id),
			});
		}
	}

	let task_json = match serde_json::to_value(&recheck_task) {
		Ok(value) => value,
		Err(error) => {
			warn!(
				target: LOG_TARGET,
				task_id = task_db_id,
				error = ?error,
				"Failed to serialize delayed recheck task"
			);
			return;
		}
	};

	let schedule_result = sqlx::query(
		r#"
		WITH updated AS (
			UPDATE verification_delayed_rechecks
			SET job_id = $2,
				tenant_id = $3,
				task = $4,
				retry_count = $5,
				run_at = $6,
				status = 'scheduled',
				publish_attempts = 0,
				last_error = NULL,
				updated_at = NOW(),
				published_at = NULL
			WHERE task_result_id = $1 AND status = 'scheduled'
			RETURNING id
		)
		INSERT INTO verification_delayed_rechecks (
			task_result_id, job_id, tenant_id, task, retry_count, run_at
		)
		SELECT $1, $2, $3, $4, $5, $6
		WHERE NOT EXISTS (SELECT 1 FROM updated)
		"#,
	)
	.bind(task_db_id)
	.bind(job_id)
	.bind(tenant_id)
	.bind(task_json)
	.bind(retry_count)
	.bind(run_at)
	.execute(&pg_pool)
	.await;

	if let Err(error) = schedule_result {
		warn!(
			target: LOG_TARGET,
			task_id = task_db_id,
			error = ?error,
			"Failed to persist delayed recheck task"
		);
		record_job_event(
			&pg_pool,
			&config,
			job_id,
			Some(task_db_id),
			"task.delayed_recheck_failed",
			Some(serde_json::json!({
				"retry_count": retry_count,
				"reason": "schedule_persistence_failed"
			})),
		)
		.await;
		return;
	}

	record_job_event(
		&pg_pool,
		&config,
		job_id,
		Some(task_db_id),
		"task.partial_confidence",
		Some(serde_json::json!({ "retry_count": retry_count })),
	)
	.await;
	record_job_event(
		&pg_pool,
		&config,
		job_id,
		Some(task_db_id),
		"task.delayed_recheck_scheduled",
		Some(serde_json::json!({
			"retry_count": retry_count,
			"delay_seconds": delay_seconds,
			"run_at": run_at
		})),
	)
	.await;
}

pub async fn run_delayed_recheck_cycle(
	config: Arc<BackendConfig>,
	pg_pool: &PgPool,
) -> Result<(), anyhow::Error> {
	let worker_config = match config.must_worker_config() {
		Ok(worker_config) => worker_config,
		Err(error) => {
			warn!(
				target: LOG_TARGET,
				error = ?error,
				"Delayed recheck scheduler requires worker mode"
			);
			return Ok(());
		}
	};

	reset_stale_publishing_rechecks(pg_pool, config.delayed_recheck.stale_publishing_seconds)
		.await?;
	let due_rechecks = claim_due_rechecks(pg_pool, config.delayed_recheck.batch_size).await?;
	if due_rechecks.is_empty() {
		return Ok(());
	}

	debug!(
		target: LOG_TARGET,
		count = due_rechecks.len(),
		"Publishing due delayed rechecks"
	);
	for recheck in due_rechecks {
		publish_claimed_recheck(
			&config,
			pg_pool,
			Arc::clone(&worker_config.channel),
			recheck,
		)
		.await;
	}

	Ok(())
}

async fn reset_stale_publishing_rechecks(
	pg_pool: &PgPool,
	stale_publishing_seconds: i64,
) -> Result<(), anyhow::Error> {
	sqlx::query(
		r#"
		UPDATE verification_delayed_rechecks stale
		SET status = 'failed',
			last_error = 'stale publisher claim superseded by a newer scheduled recheck',
			updated_at = NOW()
		WHERE stale.status = 'publishing'
			AND stale.updated_at < NOW() - ($1::BIGINT * INTERVAL '1 second')
			AND EXISTS (
				SELECT 1
				FROM verification_delayed_rechecks newer
				WHERE newer.task_result_id = stale.task_result_id
					AND newer.status = 'scheduled'
					AND newer.id <> stale.id
			)
		"#,
	)
	.bind(stale_publishing_seconds)
	.execute(pg_pool)
	.await?;

	sqlx::query(
		r#"
		UPDATE verification_delayed_rechecks
		SET status = 'scheduled',
			run_at = NOW(),
			last_error = 'stale publisher claim reset',
			updated_at = NOW()
		WHERE status = 'publishing'
			AND updated_at < NOW() - ($1::BIGINT * INTERVAL '1 second')
		"#,
	)
	.bind(stale_publishing_seconds)
	.execute(pg_pool)
	.await?;

	Ok(())
}

async fn claim_due_rechecks(
	pg_pool: &PgPool,
	batch_size: i64,
) -> Result<Vec<DelayedRecheckRow>, anyhow::Error> {
	let rows = sqlx::query(
		r#"
		WITH due AS (
			SELECT id
			FROM verification_delayed_rechecks
			WHERE status = 'scheduled' AND run_at <= NOW()
			ORDER BY run_at, id
			LIMIT $1
			FOR UPDATE SKIP LOCKED
		)
		UPDATE verification_delayed_rechecks recheck
		SET status = 'publishing',
			publish_attempts = publish_attempts + 1,
			updated_at = NOW()
		FROM due
		WHERE recheck.id = due.id
		RETURNING recheck.id,
			recheck.task_result_id,
			recheck.job_id,
			recheck.retry_count,
			recheck.publish_attempts,
			recheck.task
		"#,
	)
	.bind(batch_size)
	.fetch_all(pg_pool)
	.await?;

	let mut due_rechecks = Vec::with_capacity(rows.len());
	for row in rows {
		due_rechecks.push(DelayedRecheckRow {
			id: row.try_get("id")?,
			task_result_id: row.try_get("task_result_id")?,
			job_id: row.try_get("job_id")?,
			retry_count: row.try_get("retry_count")?,
			publish_attempts: row.try_get("publish_attempts")?,
			task: row.try_get("task")?,
		});
	}

	Ok(due_rechecks)
}

async fn publish_claimed_recheck(
	config: &BackendConfig,
	pg_pool: &PgPool,
	channel: Arc<Channel>,
	recheck: DelayedRecheckRow,
) {
	let task = match serde_json::from_value::<CheckEmailTask>(recheck.task.clone()) {
		Ok(task) => task,
		Err(error) => {
			let reason = format!("invalid delayed recheck task payload: {error}");
			warn!(
				target: LOG_TARGET,
				recheck_id = recheck.id,
				task_id = recheck.task_result_id,
				error = ?error,
				"Failed to deserialize delayed recheck task"
			);
			mark_recheck_failed(pg_pool, recheck.id, &reason).await;
			record_job_event(
				pg_pool,
				config,
				recheck.job_id,
				Some(recheck.task_result_id),
				"task.delayed_recheck_failed",
				Some(serde_json::json!({
					"retry_count": recheck.retry_count,
					"reason": "invalid_task_payload"
				})),
			)
			.await;
			return;
		}
	};

	let properties = BasicProperties::default()
		.with_content_type("application/json".into())
		.with_priority(0);

	match publish_task(channel, task, properties).await {
		Ok(()) => {
			if let Err(error) = sqlx::query(
				r#"
				UPDATE verification_delayed_rechecks
				SET status = 'published',
					published_at = NOW(),
					last_error = NULL,
					updated_at = NOW()
				WHERE id = $1
				"#,
			)
			.bind(recheck.id)
			.execute(pg_pool)
			.await
			{
				warn!(
					target: LOG_TARGET,
					recheck_id = recheck.id,
					task_id = recheck.task_result_id,
					error = ?error,
					"Failed to mark delayed recheck published"
				);
			}

			record_job_event(
				pg_pool,
				config,
				recheck.job_id,
				Some(recheck.task_result_id),
				"task.delayed_recheck_published",
				Some(serde_json::json!({ "retry_count": recheck.retry_count })),
			)
			.await;
			info!(
				target: LOG_TARGET,
				recheck_id = recheck.id,
				task_id = recheck.task_result_id,
				retry_count = recheck.retry_count,
				"Published delayed recheck task"
			);
		}
		Err(error) => {
			let reason = format!("{error:?}");
			warn!(
				target: LOG_TARGET,
				recheck_id = recheck.id,
				task_id = recheck.task_result_id,
				attempts = recheck.publish_attempts,
				error = ?error,
				"Failed to publish delayed recheck task"
			);
			reschedule_or_fail_publish(pg_pool, config, &recheck, &reason).await;
		}
	}
}

async fn reschedule_or_fail_publish(
	pg_pool: &PgPool,
	config: &BackendConfig,
	recheck: &DelayedRecheckRow,
	reason: &str,
) {
	if recheck.publish_attempts >= config.delayed_recheck.max_publish_attempts {
		mark_recheck_failed(pg_pool, recheck.id, reason).await;
		record_job_event(
			pg_pool,
			config,
			recheck.job_id,
			Some(recheck.task_result_id),
			"task.delayed_recheck_failed",
			Some(serde_json::json!({
				"retry_count": recheck.retry_count,
				"reason": "publish_failed",
				"publish_attempts": recheck.publish_attempts
			})),
		)
		.await;
		return;
	}

	let next_run_at =
		Utc::now() + ChronoDuration::seconds(config.delayed_recheck.publish_retry_seconds);
	if let Err(error) = sqlx::query(
		r#"
		UPDATE verification_delayed_rechecks
		SET status = 'scheduled',
			run_at = $2,
			last_error = $3,
			updated_at = NOW()
		WHERE id = $1
		"#,
	)
	.bind(recheck.id)
	.bind(next_run_at)
	.bind(reason)
	.execute(pg_pool)
	.await
	{
		warn!(
			target: LOG_TARGET,
			recheck_id = recheck.id,
			task_id = recheck.task_result_id,
			error = ?error,
			"Failed to reschedule delayed recheck publish attempt"
		);
	}
}

async fn mark_recheck_failed(pg_pool: &PgPool, recheck_id: i64, reason: &str) {
	if let Err(error) = sqlx::query(
		r#"
		UPDATE verification_delayed_rechecks
		SET status = 'failed',
			last_error = $2,
			updated_at = NOW()
		WHERE id = $1
		"#,
	)
	.bind(recheck_id)
	.bind(reason)
	.execute(pg_pool)
	.await
	{
		warn!(
			target: LOG_TARGET,
			recheck_id = recheck_id,
			error = ?error,
			"Failed to mark delayed recheck failed"
		);
	}
}

async fn record_job_event(
	pg_pool: &PgPool,
	config: &BackendConfig,
	job_id: i32,
	task_id: Option<i32>,
	event_type: &str,
	event_data: Option<Value>,
) {
	let result = sqlx::query(
		r#"
		INSERT INTO job_events (job_id, task_id, event_type, event_data, actor)
		VALUES ($1, $2, $3, $4, $5)
		"#,
	)
	.bind(job_id)
	.bind(task_id)
	.bind(event_type)
	.bind(event_data)
	.bind(&config.backend_name)
	.execute(pg_pool)
	.await;

	if let Err(error) = result {
		warn!(
			target: LOG_TARGET,
			job_id = job_id,
			event_type = event_type,
			error = ?error,
			"Failed to record delayed recheck event"
		);
	}
}

pub fn delayed_recheck_delay_seconds(retry_policy: &RetryPolicy, retry_count: i32) -> u64 {
	let default_policy = RetryPolicy::default();
	let uses_default_backoff = retry_policy.backoff_seconds == default_policy.backoff_seconds
		&& (retry_policy.backoff_multiplier - default_policy.backoff_multiplier).abs()
			< f64::EPSILON;
	if uses_default_backoff {
		return match retry_count {
			0 | 1 => 300,
			2 => 900,
			_ => 900,
		};
	}

	(retry_policy.backoff_seconds as f64
		* retry_policy
			.backoff_multiplier
			.powf((retry_count - 1).max(0) as f64))
	.max(0.0)
	.round() as u64
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn delayed_recheck_uses_greylist_windows_for_default_policy() {
		let p = RetryPolicy::default();
		assert_eq!(delayed_recheck_delay_seconds(&p, 1), 300);
		assert_eq!(delayed_recheck_delay_seconds(&p, 2), 900);
	}

	#[test]
	fn delayed_recheck_respects_explicit_retry_policy() {
		let p = RetryPolicy {
			max_retries: 2,
			backoff_seconds: 10,
			backoff_multiplier: 3.0,
		};
		assert_eq!(delayed_recheck_delay_seconds(&p, 1), 10);
		assert_eq!(delayed_recheck_delay_seconds(&p, 2), 30);
	}
}
