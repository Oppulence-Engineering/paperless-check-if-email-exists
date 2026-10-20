use crate::config::{BackendConfig, ReverificationConfig};
use crate::http::v1::bulk::post::publish_task;
use crate::http::CheckEmailRequest;
use crate::tenant::quota::release_reserved_usage;
use crate::worker::do_work::{CheckEmailJobId, CheckEmailTask, TaskMetadata};
use check_if_email_exists::LOG_TARGET;
use lapin::{BasicProperties, Channel};
use sqlx::PgPool;
use std::sync::Arc;
use std::time::Duration;
use tracing::{info, warn};

pub fn spawn_reverification_scheduler(config: Arc<BackendConfig>, pg_pool: PgPool) {
	let reverification_config = config.reverification.clone();
	tokio::spawn(async move {
		let interval = Duration::from_secs(reverification_config.interval_seconds);
		// Run first cycle immediately, then sleep between subsequent cycles
		loop {
			if let Err(err) =
				run_reverification_cycle(&reverification_config, &config, &pg_pool).await
			{
				warn!(target: LOG_TARGET, error = ?err, "Reverification cycle failed");
			}
			tokio::time::sleep(interval).await;
		}
	});
}

/// Claim every due reverification schedule and queue a reverification job
/// for each. Public so the scheduler cycle can be driven from tests.
pub async fn run_reverification_cycle(
	reverification_config: &ReverificationConfig,
	config: &Arc<BackendConfig>,
	pg_pool: &PgPool,
) -> Result<(), anyhow::Error> {
	crate::smtp_egress::require_smtp_ready(config)?;
	// Atomically claim due schedules by advancing next_run_at in a single
	// UPDATE ... RETURNING, preventing concurrent instances from processing
	// the same schedule (no transaction needed, the UPDATE itself is atomic).
	let schedules = sqlx::query_as::<_, ScheduleRow>(
		r#"
		UPDATE reverification_schedules rs
		SET next_run_at = NOW() + make_interval(secs => $1),
			updated_at = NOW()
		FROM tenants t
		WHERE t.id = rs.tenant_id
		  AND rs.enabled = true
		  AND t.status = 'active'
		  AND (rs.next_run_at IS NULL OR rs.next_run_at <= NOW())
		RETURNING rs.id, rs.tenant_id, rs.staleness_days, rs.batch_size
		"#,
	)
	.bind(reverification_config.interval_seconds as f64)
	.fetch_all(pg_pool)
	.await?;

	if schedules.is_empty() {
		return Ok(());
	}

	info!(target: LOG_TARGET, count = schedules.len(), "Running reverification cycle");

	for schedule in schedules {
		if let Err(err) = process_schedule(reverification_config, config, pg_pool, &schedule).await
		{
			warn!(
				target: LOG_TARGET,
				tenant_id = %schedule.tenant_id,
				schedule_id = schedule.id,
				error = ?err,
				"Failed to process reverification schedule"
			);
		}
	}

	Ok(())
}

async fn process_schedule(
	_reverification_config: &ReverificationConfig,
	config: &Arc<BackendConfig>,
	pg_pool: &PgPool,
	schedule: &ScheduleRow,
) -> Result<(), anyhow::Error> {
	let staleness_days = schedule.staleness_days;
	let batch_size = schedule.batch_size;

	// Find emails whose LATEST completion is stale. The subquery ensures we
	// only pick emails where the most recent completed_at is older than the
	// threshold, not emails that happen to have any old result.
	let stale_emails: Vec<StaleEmail> = sqlx::query_as::<_, StaleEmail>(
		r#"
		SELECT email FROM (
			SELECT payload->'input'->>'to_email' AS email,
				   MAX(completed_at) AS latest_completed
			FROM v1_task_result
			WHERE tenant_id = $1
			  AND task_state = 'completed'
			  AND completed_at IS NOT NULL
			  -- Duplicates share their primary's result, and blank or
			  -- malformed rows were never verified; re-verifying them
			  -- would only spend credits.
			  AND is_duplicate IS NOT TRUE
			  AND COALESCE(payload->'input'->>'to_email', '') <> ''
			  AND sub_reason IS DISTINCT FROM 'invalid_syntax'
			GROUP BY payload->'input'->>'to_email'
		) sub
		WHERE sub.latest_completed < NOW() - make_interval(days => $2)
		  AND NOT EXISTS (
			SELECT 1 FROM v1_task_result t2
			WHERE t2.tenant_id = $1
			  AND t2.payload->'input'->>'to_email' = sub.email
			  AND t2.task_state IN ('queued', 'running', 'retrying')
		  )
		LIMIT $3
		"#,
	)
	.bind(schedule.tenant_id)
	.bind(staleness_days)
	.bind(batch_size)
	.fetch_all(pg_pool)
	.await?;

	if stale_emails.is_empty() {
		// next_run_at already advanced by the atomic claim; just update last_run_at
		let _ =
			sqlx::query("UPDATE reverification_schedules SET last_run_at = NOW() WHERE id = $1")
				.bind(schedule.id)
				.execute(pg_pool)
				.await;
		return Ok(());
	}

	// Validate worker config early before consuming quota or creating DB records
	let worker_config = config.must_worker_config()?;

	// Check quota
	let tenant_ctx =
		crate::tenant::auth::resolve_tenant_context_by_id(pg_pool, schedule.tenant_id).await?;
	let email_count = stale_emails.len() as i32;
	match crate::tenant::quota::check_and_increment_quota_for_count(
		Some(pg_pool),
		&tenant_ctx,
		email_count,
	)
	.await
	{
		crate::tenant::quota::QuotaCheckResult::Allowed => {}
		crate::tenant::quota::QuotaCheckResult::Unavailable => {
			anyhow::bail!("Quota service unavailable; reverification was not dispatched");
		}
		crate::tenant::quota::QuotaCheckResult::ExceededMonthlyLimit { .. } => {
			info!(
				target: LOG_TARGET,
				tenant_id = %schedule.tenant_id,
				"Skipping reverification: quota exceeded"
			);
			// next_run_at already advanced by the atomic claim
			return Ok(());
		}
	}

	// From here on the whole batch is reserved: every exit path must give
	// back what was not published.
	let job_id: i32 = match sqlx::query_scalar(
		r#"
		INSERT INTO v1_bulk_job (total_records, tenant_id, status, metadata)
		VALUES ($1, $2, 'pending'::job_state, '{"source": "reverification"}'::jsonb)
		RETURNING id
		"#,
	)
	.bind(email_count)
	.bind(schedule.tenant_id)
	.fetch_one(pg_pool)
	.await
	{
		Ok(job_id) => job_id,
		Err(err) => {
			// release_reserved_usage logs its own failure.
			let _ =
				release_reserved_usage(Some(pg_pool), Some(schedule.tenant_id), email_count).await;
			return Err(err.into());
		}
	};

	let PublishOutcome {
		published,
		insert_error,
	} = publish_stale_emails(
		config,
		pg_pool,
		&worker_config.channel,
		schedule.tenant_id,
		job_id,
		&stale_emails,
	)
	.await;

	let _ = release_reserved_usage(
		Some(pg_pool),
		Some(schedule.tenant_id),
		email_count - published,
	)
	.await;

	// The job stays 'pending' while its rows are created and published, so
	// the worker cannot finish it after the first task. With nothing published
	// no worker will ever finalize it, so it fails here.
	let settled = if published > 0 {
		crate::worker::job_lifecycle::start_bulk_job(pg_pool, job_id).await
	} else {
		sqlx::query(
			"UPDATE v1_bulk_job SET status = 'failed'::job_state, completed_at = COALESCE(completed_at, NOW()), updated_at = NOW() WHERE id = $1 AND status = 'pending'",
		)
		.bind(job_id)
		.execute(pg_pool)
		.await
		.map(|_| ())
	};
	if let Err(err) = settled {
		warn!(target: LOG_TARGET, job_id = job_id, error = ?err, "Failed to settle reverification job");
	}

	// Update schedule (next_run_at already set by the atomic claim)
	sqlx::query(
		r#"
		UPDATE reverification_schedules
		SET last_run_at = NOW(),
			last_job_id = $2,
			emails_requeued = emails_requeued + $3,
			updated_at = NOW()
		WHERE id = $1
		"#,
	)
	.bind(schedule.id)
	.bind(job_id)
	.bind(published)
	.execute(pg_pool)
	.await?;

	info!(
		target: LOG_TARGET,
		tenant_id = %schedule.tenant_id,
		job_id = job_id,
		published = published,
		"Reverification job created"
	);

	match insert_error {
		Some(err) => Err(err.into()),
		None => Ok(()),
	}
}

struct PublishOutcome {
	published: i32,
	/// Set when creating a task row failed; the remaining emails were skipped.
	insert_error: Option<sqlx::Error>,
}

/// Create a task row for each stale email and publish it. Rows whose publish
/// fails are marked failed so they don't stay orphaned as 'queued'. A failed
/// row INSERT stops the batch: it usually means Postgres is unavailable, and
/// trying every remaining email would only wait out the pool timeout each
/// time.
async fn publish_stale_emails(
	config: &Arc<BackendConfig>,
	pg_pool: &PgPool,
	channel: &Arc<Channel>,
	tenant_id: uuid::Uuid,
	job_id: i32,
	stale_emails: &[StaleEmail],
) -> PublishOutcome {
	let properties = BasicProperties::default()
		.with_content_type("application/json".into())
		.with_priority(0); // Lower priority than normal tasks

	let mut published = 0;
	let mut failed_task_ids: Vec<i32> = Vec::new();
	let mut insert_error = None;

	for stale in stale_emails {
		let mut task = reverification_task(config, tenant_id, job_id, &stale.email);
		// Stored as the full task (like every other publisher does) so that
		// jobs/retry.rs can deserialize and republish it.
		let payload = match serde_json::to_value(&task) {
			Ok(payload) => payload,
			Err(err) => {
				warn!(target: LOG_TARGET, email = %stale.email, error = ?err, "Failed to serialize reverification task");
				continue;
			}
		};
		let task_row_id: i32 = match sqlx::query_scalar(
			r#"
			INSERT INTO v1_task_result (job_id, payload, task_state, tenant_id)
			VALUES ($1, $2, 'queued', $3)
			RETURNING id
			"#,
		)
		.bind(job_id)
		.bind(payload)
		.bind(tenant_id)
		.fetch_one(pg_pool)
		.await
		{
			Ok(id) => id,
			Err(err) => {
				insert_error = Some(err);
				break;
			}
		};

		if let Some(metadata) = task.metadata.as_mut() {
			metadata.task_db_id = Some(task_row_id);
		}

		if let Err(err) = publish_task(Arc::clone(channel), task, properties.clone()).await {
			warn!(
				target: LOG_TARGET,
				email = %stale.email,
				error = ?err,
				"Failed to publish reverification task"
			);
			failed_task_ids.push(task_row_id);
			continue;
		}
		published += 1;
	}

	for task_id in &failed_task_ids {
		let _ = sqlx::query(
			"UPDATE v1_task_result SET task_state = 'failed'::task_state, error = 'publish_failed', completed_at = NOW(), updated_at = NOW() WHERE id = $1",
		)
		.bind(task_id)
		.execute(pg_pool)
		.await;
	}

	if published < stale_emails.len() as i32 {
		warn!(
			target: LOG_TARGET,
			tenant_id = %tenant_id,
			job_id = job_id,
			published = published,
			failed = failed_task_ids.len(),
			"Partial reverification publish failure"
		);
	}

	PublishOutcome {
		published,
		insert_error,
	}
}

/// Build the task the same way the API publishers do, so reverification
/// honours the configured verification method (proxy, hello name, from
/// email, SMTP timeout, provider overrides) instead of library defaults.
fn reverification_task(
	config: &Arc<BackendConfig>,
	tenant_id: uuid::Uuid,
	job_id: i32,
	email: &str,
) -> CheckEmailTask {
	CheckEmailTask {
		input: CheckEmailRequest {
			to_email: email.to_string(),
			..Default::default()
		}
		.to_check_email_input(Arc::clone(config)),
		job_id: CheckEmailJobId::Bulk(job_id),
		webhook: None,
		metadata: Some(TaskMetadata {
			dispatch_id: None,
			lease_id: None,
			decision_policy: None,
			tenant_id: Some(tenant_id.to_string()),
			request_id: None,
			correlation_id: None,
			created_by: Some("reverification".to_string()),
			retry_policy: None,
			dedupe_key: Some(format!("reverify:{}:{}", job_id, email)),
			task_db_id: None,
		}),
	}
}

#[derive(Debug, sqlx::FromRow)]
struct ScheduleRow {
	id: i32,
	tenant_id: uuid::Uuid,
	staleness_days: i32,
	batch_size: i32,
}

#[derive(Debug, sqlx::FromRow)]
struct StaleEmail {
	email: String,
}
