//! PostgreSQL owns submission; RabbitMQ delivers committed work at least once.

use super::do_work::CheckEmailTask;
use crate::config::BackendConfig;
use crate::http::v1::bulk::post::publish_payload;
use check_if_email_exists::LOG_TARGET;
use lapin::{BasicProperties, Channel};
use sqlx::{PgPool, Postgres, Row, Transaction};
use std::{
	sync::Arc,
	time::{Duration, Instant},
};
use uuid::Uuid;

/// Call inside the transaction that reserves quota and creates/resets the task.
pub async fn enqueue(
	tx: &mut Transaction<'_, Postgres>,
	mut task: CheckEmailTask,
	priority: u8,
) -> Result<Uuid, anyhow::Error> {
	let metadata = task
		.metadata
		.as_mut()
		.ok_or_else(|| anyhow::anyhow!("Missing task metadata"))?;
	let task_id = metadata
		.task_db_id
		.ok_or_else(|| anyhow::anyhow!("Missing task identity"))?;
	let dispatch_id = Uuid::new_v4();
	metadata.dispatch_id = Some(dispatch_id);
	metadata.lease_id = None;
	let updated = sqlx::query(
		"UPDATE v1_task_result SET dispatch_id = $2, lease_id = NULL, lease_expires_at = NULL, verification_stored = false, webhook_done = false, completion_done = false WHERE id = $1",
    ).bind(task_id).bind(dispatch_id).execute(&mut **tx).await?;
	anyhow::ensure!(updated.rows_affected() == 1, "Task no longer exists");
	sqlx::query(
		"INSERT INTO verification_dispatch (id, task_id, payload, priority) VALUES ($1, $2, $3, $4)
         ON CONFLICT (task_id) DO UPDATE SET id = EXCLUDED.id, payload = EXCLUDED.payload,
         priority = EXCLUDED.priority, attempts = 0, next_attempt_at = NOW()",
	)
	.bind(dispatch_id)
	.bind(task_id)
	.bind(serde_json::to_value(task)?)
	.bind(i16::from(priority))
	.execute(&mut **tx)
	.await?;
	Ok(dispatch_id)
}

/// One bounded publication per transaction; concurrent publishers claim different rows.
pub async fn publish_next(pool: &PgPool, channel: &Channel) -> Result<bool, anyhow::Error> {
	let mut tx = pool.begin().await?;
	let row = sqlx::query(
		"SELECT id, payload, priority FROM verification_dispatch
         WHERE next_attempt_at <= NOW() ORDER BY next_attempt_at, created_at
         LIMIT 1 FOR UPDATE SKIP LOCKED",
	)
	.fetch_optional(&mut *tx)
	.await?;
	let Some(row) = row else { return Ok(false) };
	let id: Uuid = row.get("id");
	let payload: serde_json::Value = row.get("payload");
	let properties = BasicProperties::default()
		.with_content_type("application/json".into())
		.with_priority(row.get::<i16, _>("priority") as u8);
	let payload = serde_json::to_vec(&payload)?;
	match tokio::time::timeout(
		Duration::from_secs(5),
		publish_payload(channel, &payload, properties),
	)
	.await
	{
		Ok(Ok(())) => {
			sqlx::query("DELETE FROM verification_dispatch WHERE id = $1")
				.bind(id)
				.execute(&mut *tx)
				.await?;
		}
		_ => {
			// Keep intent indefinitely; a lost confirm may produce a duplicate delivery.
			sqlx::query(
				"UPDATE verification_dispatch SET attempts = LEAST(attempts, 1000000) + 1,
                 next_attempt_at = NOW() + (LEAST(60, 1 + attempts * 2) * INTERVAL '1 second')
                 WHERE id = $1",
			)
			.bind(id)
			.execute(&mut *tx)
			.await?;
			tracing::warn!(target: LOG_TARGET, dispatch_id = %id, "Verification publication deferred");
		}
	}
	tx.commit().await?;
	Ok(true)
}

/// Repair interrupted completion even if its original broker delivery is unavailable.
pub async fn recover_pending_completion(pool: &PgPool) -> Result<u64, sqlx::Error> {
	let result = sqlx::query(
		"WITH due AS (
		 SELECT id FROM v1_task_result
		 WHERE verification_stored AND NOT completion_done AND NOT is_duplicate
		   AND dispatch_id IS NOT NULL AND (lease_expires_at IS NULL OR lease_expires_at <= NOW())
		   AND updated_at < NOW() - INTERVAL '60 seconds'
		   AND NOT EXISTS (SELECT 1 FROM verification_dispatch d WHERE d.task_id = v1_task_result.id)
		 ORDER BY updated_at LIMIT 100 FOR UPDATE SKIP LOCKED
		), claimed AS (
		 UPDATE v1_task_result t SET updated_at = NOW() FROM due WHERE t.id = due.id
		 RETURNING t.id, t.dispatch_id, t.payload
		)
		INSERT INTO verification_dispatch (id, task_id, payload, priority)
		SELECT dispatch_id, id, jsonb_set(payload, '{metadata,dispatch_id}', to_jsonb(dispatch_id)), 1
		FROM claimed ON CONFLICT DO NOTHING",
	)
	.execute(pool)
	.await?;
	Ok(result.rows_affected())
}

/// A committed dispatch remains available after a direct HTTP executor crashes.
pub async fn execute_local(
	pool: &PgPool,
	config: Arc<BackendConfig>,
	id: Uuid,
) -> Result<(), anyhow::Error> {
	let payload: Option<serde_json::Value> =
		sqlx::query_scalar("SELECT payload FROM verification_dispatch WHERE id = $1")
			.bind(id)
			.fetch_optional(pool)
			.await?;
	if let Some(payload) = payload {
		let task: CheckEmailTask = serde_json::from_value(payload)?;
		super::lifecycle::execute_task(&task, config).await?;
		sqlx::query("DELETE FROM verification_dispatch WHERE id = $1")
			.bind(id)
			.execute(pool)
			.await?;
	}
	Ok(())
}

pub fn spawn_publisher(config: Arc<BackendConfig>, pool: PgPool) {
	tokio::spawn(async move {
		let mut recovery_at = Instant::now();
		loop {
			if Instant::now() >= recovery_at {
				if let Err(error) = recover_pending_completion(&pool).await {
					tracing::warn!(target: LOG_TARGET, error = ?error, "Completion recovery deferred");
				}
				recovery_at = Instant::now() + Duration::from_secs(30);
			}
			let result = if !config.worker.enable {
				async {
					let id: Option<Uuid> = sqlx::query_scalar("SELECT d.id FROM verification_dispatch d JOIN v1_task_result t ON t.id = d.task_id WHERE t.operation_id IS NOT NULL AND d.next_attempt_at <= NOW() AND (t.lease_expires_at IS NULL OR t.lease_expires_at <= NOW()) ORDER BY d.next_attempt_at LIMIT 1")
						.fetch_optional(&pool).await?;
					if let Some(id) = id {
						execute_local(&pool, Arc::clone(&config), id).await?;
						Ok::<bool, anyhow::Error>(true)
					} else { Ok(false) }
				}.await
			} else {
				match config.must_worker_config() {
					Ok(worker) => publish_next(&pool, &worker.channel).await,
					Err(error) => Err(error.into()),
				}
			};
			match result {
				Ok(true) => continue,
				Err(error) => {
					tracing::warn!(target: LOG_TARGET, error = ?error, "Verification publisher unavailable")
				}
				Ok(false) => {}
			}
			tokio::time::sleep(Duration::from_secs(1)).await;
		}
	});
}
