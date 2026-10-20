//! Short database leases fence redelivery without holding a connection during SMTP.

use super::do_work::{
	check_email_and_prepare_result_with_config, send_task_webhook_json, sync_related_entities,
	CheckEmailJobId, CheckEmailTask,
};
use crate::config::BackendConfig;
use anyhow::{anyhow, ensure};
use lapin::{message::Delivery, options::*, Channel};
use sqlx::{PgPool, Row};
use std::{sync::Arc, time::Duration};
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub enum Claim {
	Acquired(Uuid),
	Busy,
	Finished,
	Obsolete,
}

pub async fn claim_task(pool: &PgPool, task: &CheckEmailTask) -> Result<Claim, anyhow::Error> {
	let metadata = task
		.metadata
		.as_ref()
		.ok_or_else(|| anyhow!("Missing metadata"))?;
	let id = metadata
		.task_db_id
		.ok_or_else(|| anyhow!("Missing task identity"))?;
	let mut tx = pool.begin().await?;
	let mut cancelled = false;
	if let CheckEmailJobId::Bulk(job_id) = task.job_id {
		let job: Option<String> =
			sqlx::query_scalar("SELECT status::TEXT FROM v1_bulk_job WHERE id = $1 FOR UPDATE")
				.bind(job_id)
				.fetch_optional(&mut *tx)
				.await?;
		if job.is_none() {
			return Ok(Claim::Obsolete);
		}
		cancelled = matches!(job.as_deref(), Some("cancelling" | "cancelled"));
	}
	let row = sqlx::query(
		"SELECT dispatch_id, completion_done, verification_stored, task_state::TEXT,
         lease_expires_at > NOW() AS busy FROM v1_task_result WHERE id = $1 FOR UPDATE",
	)
	.bind(id)
	.fetch_optional(&mut *tx)
	.await?;
	let Some(row) = row else {
		return Ok(Claim::Obsolete);
	};
	if row.get::<Option<Uuid>, _>("dispatch_id") != metadata.dispatch_id {
		return Ok(Claim::Obsolete);
	}
	if row.get::<bool, _>("completion_done") {
		return Ok(Claim::Finished);
	}
	if row.get::<Option<bool>, _>("busy").unwrap_or(false) {
		return Ok(Claim::Busy);
	}
	if cancelled && !row.get::<bool, _>("verification_stored") {
		sqlx::query("UPDATE v1_task_result SET task_state = CASE WHEN result IS NULL THEN 'cancelled'::task_state ELSE 'completed'::task_state END, completion_done = true, completed_at = COALESCE(completed_at, NOW()) WHERE id = $1")
            .bind(id).execute(&mut *tx).await?;
		tx.commit().await?;
		return Ok(Claim::Finished);
	}
	let state: String = row.get("task_state");
	if matches!(
		state.as_str(),
		"completed" | "failed" | "dead_lettered" | "cancelled"
	) && !row.get::<bool, _>("verification_stored")
	{
		return Ok(Claim::Finished);
	}
	let lease_id = Uuid::new_v4();
	sqlx::query(
		"UPDATE v1_task_result SET lease_id = $2, lease_expires_at = NOW() + INTERVAL '60 seconds',
         task_state = CASE WHEN verification_stored THEN task_state ELSE 'running'::task_state END,
         started_at = COALESCE(started_at, NOW()), updated_at = NOW() WHERE id = $1",
	)
	.bind(id)
	.bind(lease_id)
	.execute(&mut *tx)
	.await?;
	tx.commit().await?;
	Ok(Claim::Acquired(lease_id))
}

pub async fn renew_lease(pool: &PgPool, id: i32, lease: Uuid) -> Result<(), anyhow::Error> {
	let result = sqlx::query(
		"UPDATE v1_task_result SET lease_expires_at = NOW() + INTERVAL '60 seconds'
         WHERE id = $1 AND lease_id = $2 AND lease_expires_at > NOW()",
	)
	.bind(id)
	.bind(lease)
	.execute(pool)
	.await?;
	ensure!(
		result.rows_affected() == 1,
		"Task lease expired or was superseded"
	);
	Ok(())
}

pub async fn process_delivery(
	task: &CheckEmailTask,
	delivery: Delivery,
	channel: Arc<Channel>,
	config: Arc<BackendConfig>,
) -> Result<(), anyhow::Error> {
	let result = async {
		execute_task(task, Arc::clone(&config)).await?;
		let pool = config
			.get_pg_pool()
			.ok_or_else(|| anyhow!("Persistent task requires Postgres"))?;
		reply_from_storage(
			&pool,
			task.metadata
				.as_ref()
				.and_then(|m| m.task_db_id)
				.ok_or_else(|| anyhow!("Missing task identity"))?,
			&delivery,
			&channel,
		)
		.await
	}
	.await;
	match result {
		Ok(()) => {
			super::do_work::maybe_pause_before_delivery_settle().await;
			delivery.ack(BasicAckOptions::default()).await?;
		}
		Err(error) => {
			tokio::time::sleep(Duration::from_secs(1)).await;
			delivery
				.reject(BasicRejectOptions { requeue: true })
				.await?;
			return Err(error);
		}
	}
	Ok(())
}

/// Both broker and direct execution use the same durable ownership and completion.
pub async fn execute_task(
	task: &CheckEmailTask,
	config: Arc<BackendConfig>,
) -> Result<(), anyhow::Error> {
	let pool = config
		.get_pg_pool()
		.ok_or_else(|| anyhow!("Persistent task requires Postgres"))?;
	let id = task
		.metadata
		.as_ref()
		.and_then(|m| m.task_db_id)
		.ok_or_else(|| anyhow!("Missing task identity"))?;
	let mut owned_task = task.clone();
	let result = async {
		match claim_task(&pool, task).await? {
			Claim::Obsolete => return Ok(()),
			Claim::Busy => return Err(anyhow!("Another worker owns this task")),
			Claim::Finished => {
				crate::list_intelligence::record_verification_change_event(&pool, id).await?;
				return sync_related_entities(&config, task).await;
			}
			Claim::Acquired(lease_id) => {
				owned_task
					.metadata
					.as_mut()
					.expect("metadata checked")
					.lease_id = Some(lease_id);
				let work = run_owned_task(&pool, &owned_task, &config);
				let heartbeat = async {
					loop {
						tokio::time::sleep(Duration::from_secs(10)).await;
						renew_lease(&pool, id, lease_id).await?;
					}
					#[allow(unreachable_code)]
					Ok::<(), anyhow::Error>(())
				};
				tokio::select! {
					result = work => result?,
					result = heartbeat => result?,
				}
				Ok(())
			}
		}
	}
	.await;
	if let Some(lease) = owned_task.metadata.as_ref().and_then(|m| m.lease_id) {
		// Expiry also releases a lease when this write fails during an outage.
		let _ = sqlx::query("UPDATE v1_task_result SET lease_id = NULL, lease_expires_at = NULL WHERE id = $1 AND lease_id = $2")
            .bind(id).bind(lease).execute(&pool).await;
	}
	result
}

async fn run_owned_task(
	pool: &PgPool,
	task: &CheckEmailTask,
	config: &Arc<BackendConfig>,
) -> Result<(), anyhow::Error> {
	let metadata = task.metadata.as_ref().expect("claimed task");
	let id = metadata.task_db_id.expect("claimed task id");
	let lease = metadata.lease_id.expect("claimed task lease");
	let row = sqlx::query("SELECT verification_stored, retry_count, result IS NOT NULL AS previous_result FROM v1_task_result WHERE id = $1 AND lease_id = $2")
        .bind(id).bind(lease).fetch_one(pool).await?;
	let retry_count: i32 = row.get("retry_count");
	let policy = metadata.retry_policy.clone().unwrap_or_default();
	if !row.get::<bool, _>("verification_stored") {
		crate::smtp_egress::require_smtp_ready(config)?;
		let output =
			check_email_and_prepare_result_with_config(task, Some(Arc::clone(config))).await;
		if let Err(error) = &output {
			if error.is_retryable() && (retry_count as u32) < policy.max_retries {
				let mut tx = pool.begin().await?;
				lock_owned(&mut tx, task).await?;
				let next_count = retry_count.saturating_add(1);
				sqlx::query("UPDATE v1_task_result SET task_state = 'retrying', retry_count = $2, updated_at = NOW() WHERE id = $1")
                    .bind(id).bind(next_count).execute(&mut *tx).await?;
				let next = super::dispatch::enqueue(&mut tx, task.clone(), 1).await?;
				let seconds =
					crate::delayed_recheck::delayed_recheck_delay_seconds(&policy, next_count);
				sqlx::query("UPDATE verification_dispatch SET next_attempt_at = NOW() + ($2::BIGINT * INTERVAL '1 second') WHERE id = $1")
                    .bind(next).bind(seconds.min(3600) as i64).execute(&mut *tx).await?;
				tx.commit().await?;
				return Ok(());
			}
		}
		let storage = config.get_storage_adapter();
		match output {
			Ok(success) => {
				storage
					.store_prepared(task, &success, storage.get_extra())
					.await?
			}
			Err(error) => {
				if metadata.created_by.as_deref() == Some("delayed_recheck")
					&& row.get::<bool, _>("previous_result")
				{
					let mut tx = pool.begin().await?;
					lock_owned(&mut tx, task).await?;
					sqlx::query(
						"UPDATE v1_task_result SET verification_stored = true WHERE id = $1",
					)
					.bind(id)
					.execute(&mut *tx)
					.await?;
					if let CheckEmailJobId::Bulk(job_id) = task.job_id {
						sqlx::query("INSERT INTO job_events (job_id, task_id, event_type, event_data, actor) VALUES ($1, $2, 'task.delayed_recheck_failed', $3, $4)")
							.bind(job_id).bind(id).bind(serde_json::json!({"reason":error.to_string(),"kept_previous_result":true}))
							.bind(&config.backend_name).execute(&mut *tx).await?;
					}
					tx.commit().await?;
				} else {
					storage
						.store_error(task, &error, storage.get_extra())
						.await?;
				}
			}
		}
	}
	finish_stored_task(pool, task, config).await
}

/// Replays required effects from saved evidence; it never invokes SMTP.
pub async fn finish_stored_task(
	pool: &PgPool,
	task: &CheckEmailTask,
	config: &Arc<BackendConfig>,
) -> Result<(), anyhow::Error> {
	let metadata = task
		.metadata
		.as_ref()
		.ok_or_else(|| anyhow!("Missing metadata"))?;
	let id = metadata
		.task_db_id
		.ok_or_else(|| anyhow!("Missing task identity"))?;
	let row = sqlx::query(
		"SELECT result, error, tenant_id, retry_count, webhook_done FROM v1_task_result
         WHERE id = $1 AND lease_id = $2 AND lease_expires_at > NOW() AND verification_stored AND NOT completion_done",
	)
	.bind(id)
	.bind(metadata.lease_id)
	.fetch_one(pool)
	.await?;
	let result: Option<serde_json::Value> = row.get("result");
	let error: Option<String> = row.get("error");
	let tenant_id: Option<Uuid> = row.get("tenant_id");
	let mut awaiting_recheck = false;
	if let Some(result) = &result {
		if let Some(tenant) = tenant_id {
			let score = result
				.get("score")
				.ok_or_else(|| anyhow!("Stored result lacks score"))?;
			super::actions::evaluate_post_completion_actions(
				pool,
				tenant,
				&task.input.to_email,
				score
					.get("score")
					.and_then(|v| v.as_i64())
					.map(|s| s as i16),
				score.get("category").and_then(|v| v.as_str()),
			)
			.await?;
			crate::outcomes::apply_post_verification_outcome_check(
				pool,
				tenant,
				&task.input.to_email,
			)
			.await?;
		}
		if !row.get::<bool, _>("webhook_done") {
			let secret = match tenant_id {
				Some(tenant) => {
					sqlx::query_scalar::<_, Option<String>>(
						"SELECT webhook_signing_secret FROM tenants WHERE id = $1",
					)
					.bind(tenant)
					.fetch_one(pool)
					.await?
				}
				None => None,
			};
			let mut webhook_error = None;
			for attempt in 0..3 {
				match send_task_webhook_json(task, result, secret.as_deref()).await {
					Ok(()) => {
						webhook_error = None;
						break;
					}
					Err(error) => {
						let retryable = error.is_retryable();
						webhook_error = Some(error.to_string());
						if !retryable || attempt == 2 {
							break;
						}
						tokio::time::sleep(Duration::from_secs(attempt + 1)).await;
					}
				}
			}
			let mut tx = pool.begin().await?;
			lock_owned(&mut tx, task).await?;
			if let (Some(error), CheckEmailJobId::Bulk(job_id)) = (webhook_error, task.job_id) {
				sqlx::query("INSERT INTO job_events (job_id, task_id, event_type, event_data, actor) VALUES ($1, $2, 'task.webhook_failed', $3, $4)")
                    .bind(job_id).bind(id).bind(serde_json::json!({"error":error,"attempts":3}))
                    .bind(&config.backend_name).execute(&mut *tx).await?;
			}
			sqlx::query("UPDATE v1_task_result SET webhook_done = true WHERE id = $1")
				.bind(id)
				.execute(&mut *tx)
				.await?;
			tx.commit().await?;
		}
		let retry_count: i32 = row.get("retry_count");
		let policy = metadata.retry_policy.clone().unwrap_or_default();
		if result.get("is_reachable").and_then(|v| v.as_str()) == Some("unknown")
			&& config.delayed_recheck.enable
			&& (retry_count as u32) < policy.max_retries
		{
			awaiting_recheck = crate::delayed_recheck::schedule_delayed_recheck(
				task,
				Arc::clone(config),
				id,
				retry_count + 1,
				&policy,
			)
			.await?;
		}
		// A crash after scheduling must retain that already committed recheck.
		if let Some(dispatch_id) = metadata.dispatch_id {
			awaiting_recheck |= sqlx::query_scalar::<_, bool>(
                "SELECT EXISTS(SELECT 1 FROM verification_delayed_rechecks WHERE task_result_id = $1
                 AND task->'metadata'->>'dispatch_id' = $2 AND status IN ('scheduled', 'publishing'))",
            ).bind(id).bind(dispatch_id.to_string()).fetch_one(pool).await?;
		}
	}
	ensure!(
		result.is_some() || error.is_some(),
		"Stored task has no result or error"
	);
	let mut tx = pool.begin().await?;
	lock_owned(&mut tx, task).await?;
	// Refund a failed single check once, in its original quota period.
	if error.is_some() {
		sqlx::query("UPDATE tenants SET used_this_period = GREATEST(used_this_period - 1, 0) FROM v1_task_result t WHERE t.id = $1 AND t.operation_id IS NOT NULL AND NOT t.completion_done AND tenants.id = t.tenant_id AND tenants.period_reset_at = t.quota_period_end")
			.bind(id).execute(&mut *tx).await?;
	}
	let state = if awaiting_recheck {
		"retrying"
	} else if error.is_some() {
		"dead_lettered"
	} else {
		"completed"
	};
	sqlx::query(
		"UPDATE v1_task_result SET task_state = $2::task_state, completion_done = true,
         completed_at = COALESCE(completed_at, NOW()), updated_at = NOW() WHERE id = $1",
	)
	.bind(id)
	.bind(state)
	.execute(&mut *tx)
	.await?;
	if let CheckEmailJobId::Bulk(job_id) = task.job_id {
		sqlx::query("INSERT INTO job_events (job_id, task_id, event_type, event_data, actor) VALUES ($1, $2, $3, $4, $5)")
            .bind(job_id).bind(id).bind(if error.is_some() { "task.dead_lettered" } else { "task.completed" })
            .bind(serde_json::json!({"dispatch_id":metadata.dispatch_id}))
            .bind(&config.backend_name).execute(&mut *tx).await?;
	}
	tx.commit().await?;
	crate::list_intelligence::record_verification_change_event(pool, id).await?;
	sync_related_entities(config, task).await?;
	Ok(())
}

async fn lock_owned(
	tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
	task: &CheckEmailTask,
) -> Result<(), sqlx::Error> {
	if let CheckEmailJobId::Bulk(job_id) = task.job_id {
		sqlx::query("SELECT id FROM v1_bulk_job WHERE id = $1 FOR UPDATE")
			.bind(job_id)
			.fetch_one(&mut **tx)
			.await?;
	}
	let metadata = task.metadata.as_ref().expect("claimed task metadata");
	sqlx::query("SELECT id FROM v1_task_result WHERE id = $1 AND lease_id = $2 AND lease_expires_at > NOW() FOR UPDATE")
        .bind(metadata.task_db_id).bind(metadata.lease_id).fetch_one(&mut **tx).await?;
	Ok(())
}

async fn reply_from_storage(
	pool: &PgPool,
	id: i32,
	delivery: &Delivery,
	channel: &Channel,
) -> Result<(), anyhow::Error> {
	let (Some(reply_to), Some(correlation)) = (
		delivery.properties.reply_to(),
		delivery.properties.correlation_id(),
	) else {
		return Ok(());
	};
	let row = sqlx::query(
		"SELECT result, error FROM v1_task_result WHERE id = $1 AND verification_stored",
	)
	.bind(id)
	.fetch_optional(pool)
	.await?;
	let Some(row) = row else { return Ok(()) };
	let result: Option<serde_json::Value> = row.get("result");
	let reply = match result {
		Some(result) => super::single_shot::SingleShotReply::Ok(serde_json::to_vec(&result)?),
		None => super::single_shot::SingleShotReply::Err((row.get("error"), 500)),
	};
	channel
		.basic_publish(
			"".into(),
			reply_to.as_str().into(),
			BasicPublishOptions::default(),
			&serde_json::to_vec(&reply)?,
			lapin::BasicProperties::default().with_correlation_id(correlation.clone()),
		)
		.await?
		.await?;
	Ok(())
}
