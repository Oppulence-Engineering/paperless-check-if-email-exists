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
use crate::worker::do_work::{
	sync_task_entities, CheckEmailJobId, CheckEmailTask, RetryPolicy, TaskMetadata,
};
use check_if_email_exists::LOG_TARGET;
use chrono::{Duration as ChronoDuration, Utc};
use lapin::Channel;
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

/// Schedules another verification of a task whose result was Unknown, and
/// moves the task back to 'retrying' so its job, list and pipeline run stay
/// open until the recheck has run. Returns whether the task is now waiting
/// on the recheck.
pub async fn schedule_delayed_recheck(
	task: &CheckEmailTask,
	config: Arc<BackendConfig>,
	task_db_id: i32,
	retry_count: i32,
	retry_policy: &RetryPolicy,
) -> Result<bool, anyhow::Error> {
	if task.verification_backend == crate::verification::VerificationBackend::Millionverifier {
		return Ok(false);
	}
	let CheckEmailJobId::Bulk(job_id) = task.job_id else {
		return Ok(false);
	};
	let pool = config
		.get_pg_pool()
		.ok_or_else(|| anyhow::anyhow!("Delayed recheck requires Postgres"))?;
	let mut tx = pool.begin().await?;
	sqlx::query("SELECT id FROM v1_bulk_job WHERE id = $1 FOR UPDATE")
		.bind(job_id)
		.fetch_one(&mut *tx)
		.await?;
	let dispatch_id = task.metadata.as_ref().and_then(|m| m.dispatch_id);
	let lease_id = task.metadata.as_ref().and_then(|m| m.lease_id);
	sqlx::query("SELECT id FROM v1_task_result WHERE id = $1 AND dispatch_id IS NOT DISTINCT FROM $2::UUID AND ($3::UUID IS NULL OR (lease_id = $3 AND lease_expires_at > NOW())) AND result IS NOT NULL FOR UPDATE")
        .bind(task_db_id).bind(dispatch_id).bind(lease_id).fetch_one(&mut *tx).await?;
	if let Some(id) = dispatch_id {
		let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM verification_delayed_rechecks WHERE task_result_id = $1 AND task->'metadata'->>'dispatch_id' = $2)")
            .bind(task_db_id).bind(id.to_string()).fetch_one(&mut *tx).await?;
		if exists {
			return Ok(true);
		}
	}
	let delay_seconds = delayed_recheck_delay_seconds(retry_policy, retry_count);
	let run_at = Utc::now() + ChronoDuration::seconds(delay_seconds.min(i64::MAX as u64) as i64);
	let mut recheck_task = task.clone();
	let metadata = recheck_task.metadata.get_or_insert_with(|| TaskMetadata {
		dispatch_id: None,
		lease_id: None,
		decision_policy: None,
		tenant_id: None,
		request_id: None,
		correlation_id: None,
		created_by: None,
		retry_policy: Some(retry_policy.clone()),
		dedupe_key: None,
		task_db_id: Some(task_db_id),
	});
	metadata.created_by = Some(DELAYED_RECHECK_ACTOR.to_string());
	metadata.task_db_id = Some(task_db_id);
	let tenant_id = metadata
		.tenant_id
		.as_deref()
		.and_then(|id| id.parse::<uuid::Uuid>().ok());
	let task_json = serde_json::to_value(&recheck_task)?;
	sqlx::query("INSERT INTO verification_delayed_rechecks (task_result_id, job_id, tenant_id, task, retry_count, run_at) VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT (task_result_id) WHERE status = 'scheduled' DO UPDATE SET task = EXCLUDED.task, retry_count = EXCLUDED.retry_count, run_at = EXCLUDED.run_at, updated_at = NOW()")
        .bind(task_db_id).bind(job_id).bind(tenant_id).bind(task_json).bind(retry_count).bind(run_at)
        .execute(&mut *tx).await?;
	sqlx::query("UPDATE v1_task_result SET task_state = 'retrying', retry_count = $2, updated_at = NOW() WHERE id = $1")
        .bind(task_db_id).bind(retry_count).execute(&mut *tx).await?;
	for event_type in ["task.partial_confidence", "task.delayed_recheck_scheduled"] {
		sqlx::query("INSERT INTO job_events (job_id, task_id, event_type, event_data, actor) VALUES ($1, $2, $3, $4, $5)")
            .bind(job_id).bind(task_db_id).bind(event_type)
            .bind(serde_json::json!({"retry_count":retry_count,"delay_seconds":delay_seconds,"run_at":run_at}))
            .bind(&config.backend_name).execute(&mut *tx).await?;
	}
	tx.commit().await?;
	Ok(true)
}

pub async fn run_delayed_recheck_cycle(
	config: Arc<BackendConfig>,
	pg_pool: &PgPool,
) -> Result<(), anyhow::Error> {
	crate::smtp_egress::require_smtp_ready(&config)?;
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
	_channel: Arc<Channel>,
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
			release_task_awaiting_recheck(config, pg_pool, recheck.task_result_id, recheck.job_id)
				.await;
			return;
		}
	};

	let publication: Result<(), anyhow::Error> = async {
        let mut tx = pg_pool.begin().await?;
        sqlx::query("SELECT id FROM v1_bulk_job WHERE id = $1 FOR UPDATE").bind(recheck.job_id).fetch_one(&mut *tx).await?;
        sqlx::query("SELECT id FROM verification_delayed_rechecks WHERE id = $1 AND status = 'publishing' FOR UPDATE")
            .bind(recheck.id).fetch_one(&mut *tx).await?;
        let ready: bool = sqlx::query_scalar("SELECT completion_done OR dispatch_id IS NULL FROM v1_task_result WHERE id = $1 AND (lease_expires_at IS NULL OR lease_expires_at <= NOW()) FOR UPDATE")
            .bind(recheck.task_result_id).fetch_one(&mut *tx).await?;
        anyhow::ensure!(ready, "Previous verification effects are still pending");
        crate::worker::dispatch::enqueue(&mut tx, task, 0).await?;
        sqlx::query("UPDATE verification_delayed_rechecks SET status = 'published', published_at = NOW(), updated_at = NOW(), last_error = NULL WHERE id = $1")
            .bind(recheck.id).execute(&mut *tx).await?;
        tx.commit().await?;
        Ok(())
    }.await;
	match publication {
		Ok(()) => {
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
		release_task_awaiting_recheck(config, pg_pool, recheck.task_result_id, recheck.job_id)
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

/// A recheck that will never run leaves its task with the earlier result:
/// the task goes back to 'completed' (unless another recheck for it is still
/// pending) and its job, list and pipeline run are finalized if it was the
/// last thing they waited on.
async fn release_task_awaiting_recheck(
	config: &BackendConfig,
	pg_pool: &PgPool,
	task_result_id: i32,
	job_id: i32,
) {
	let released = sqlx::query(
		r#"
		UPDATE v1_task_result t
		SET task_state = 'completed'::task_state, updated_at = NOW()
		WHERE t.id = $1
			AND t.task_state = 'retrying'::task_state
			AND t.result IS NOT NULL
			AND NOT EXISTS (
				SELECT 1 FROM verification_delayed_rechecks r
				WHERE r.task_result_id = t.id AND r.status IN ('scheduled', 'publishing')
			)
		"#,
	)
	.bind(task_result_id)
	.execute(pg_pool)
	.await;
	match released {
		Ok(_) => {
			if let Err(error) = sync_task_entities(config, Some(task_result_id), Some(job_id)).await
			{
				warn!(target: LOG_TARGET, task_id = task_result_id, error = ?error, "Failed to finalize entities after delayed recheck release");
			}
		}
		Err(error) => warn!(
			target: LOG_TARGET,
			task_id = task_result_id,
			error = ?error,
			"Failed to release task after its delayed recheck failed"
		),
	}
}

/// With delayed rechecks turned off nothing will ever publish the pending
/// ones, and their tasks would hold their jobs open forever. Cancels them
/// and releases the tasks with the results they already have.
pub async fn release_pending_rechecks(
	config: &BackendConfig,
	pg_pool: &PgPool,
) -> Result<usize, anyhow::Error> {
	let pending: Vec<(i32, i32)> = sqlx::query_as(
		r#"
		UPDATE verification_delayed_rechecks
		SET status = 'cancelled',
			last_error = 'delayed recheck disabled',
			updated_at = NOW()
		WHERE status IN ('scheduled', 'publishing')
		RETURNING task_result_id, job_id
		"#,
	)
	.fetch_all(pg_pool)
	.await?;
	for (task_result_id, job_id) in &pending {
		release_task_awaiting_recheck(config, pg_pool, *task_result_id, *job_id).await;
	}
	Ok(pending.len())
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
