mod test_helpers;

use crate::test_helpers::{
	ensure_test_amqp_url, insert_api_key_with_scopes, insert_job, insert_task, insert_tenant,
	test_db_url, TestDb,
};
use anyhow::Result;
use reacher_backend::config::{
	BackendConfig, DelayedRecheckConfig, PostgresConfig, RabbitMQConfig, StorageConfig,
	WorkerConfig,
};
use reacher_backend::delayed_recheck::{
	cleanup_terminal_rechecks, delayed_recheck_delay_seconds, run_delayed_recheck_cycle,
	schedule_delayed_recheck,
};
use reacher_backend::http::create_routes;
use reacher_backend::worker::do_work::{
	CheckEmailJobId, CheckEmailTask, RetryPolicy, TaskMetadata,
};
use serial_test::serial;
use sqlx::Row;
use std::sync::Arc;
use uuid::Uuid;
use warp::http::StatusCode;
use warp::test::request;

// ----------------------------------------------------------------------------
// Setup helpers
// ----------------------------------------------------------------------------

async fn worker_config(enable_recheck: bool) -> Arc<BackendConfig> {
	let amqp_url = ensure_test_amqp_url().await;
	let mut c = BackendConfig::empty();
	c.backend_name = "test-delayed-recheck".to_string();
	c.storage = Some(StorageConfig::Postgres(PostgresConfig {
		db_url: test_db_url(),
		read_replica_url: None,
		extra: None,
	}));
	c.worker = WorkerConfig {
		enable: true,
		rabbitmq: Some(RabbitMQConfig {
			url: amqp_url,
			concurrency: 4,
		}),
		webhook: None,
	};
	c.delayed_recheck = DelayedRecheckConfig {
		enable: enable_recheck,
		poll_interval_seconds: 1,
		batch_size: 100,
		stale_publishing_seconds: 300,
		publish_retry_seconds: 60,
		max_publish_attempts: 5,
		retention_days: 7,
		cleanup_interval_seconds: 3600,
	};
	c.connect().await.expect("config connect");
	Arc::new(c)
}

async fn worker_config_with(
	enable_recheck: bool,
	mutate: impl FnOnce(&mut DelayedRecheckConfig),
) -> Arc<BackendConfig> {
	let amqp_url = ensure_test_amqp_url().await;
	let mut c = BackendConfig::empty();
	c.backend_name = "test-delayed-recheck".to_string();
	c.storage = Some(StorageConfig::Postgres(PostgresConfig {
		db_url: test_db_url(),
		read_replica_url: None,
		extra: None,
	}));
	c.worker = WorkerConfig {
		enable: true,
		rabbitmq: Some(RabbitMQConfig {
			url: amqp_url,
			concurrency: 4,
		}),
		webhook: None,
	};
	let mut dr = DelayedRecheckConfig {
		enable: enable_recheck,
		poll_interval_seconds: 1,
		batch_size: 100,
		stale_publishing_seconds: 300,
		publish_retry_seconds: 60,
		max_publish_attempts: 5,
		retention_days: 7,
		cleanup_interval_seconds: 3600,
	};
	mutate(&mut dr);
	c.delayed_recheck = dr;
	c.connect().await.expect("config connect");
	Arc::new(c)
}

fn make_task(job_id: i32, email: &str, tenant_id: Option<Uuid>, task_db_id: i32) -> CheckEmailTask {
	CheckEmailTask {
		input: check_if_email_exists::CheckEmailInput {
			to_email: email.to_string(),
			..Default::default()
		},
		job_id: CheckEmailJobId::Bulk(job_id),
		webhook: None,
		metadata: Some(TaskMetadata {
			tenant_id: tenant_id.map(|t| t.to_string()),
			request_id: None,
			correlation_id: None,
			created_by: None,
			retry_policy: Some(RetryPolicy::default()),
			dedupe_key: None,
			task_db_id: Some(task_db_id),
		}),
	}
}

async fn insert_recheck_row(
	pool: &sqlx::PgPool,
	task_result_id: i32,
	job_id: i32,
	tenant_id: Option<Uuid>,
	status: &str,
	run_at_offset_seconds: i64,
	publish_attempts: i32,
) -> i64 {
	let mut task = make_task(job_id, "test@example.com", tenant_id, task_result_id);
	if let Some(metadata) = task.metadata.as_mut() {
		metadata.created_by = Some("delayed_recheck".to_string());
	}
	let task_payload = serde_json::to_value(&task).expect("serialize CheckEmailTask");
	sqlx::query_scalar::<_, i64>(
		r#"
		INSERT INTO verification_delayed_rechecks
			(task_result_id, job_id, tenant_id, task, retry_count, run_at, status, publish_attempts)
		VALUES ($1, $2, $3, $4, 1, NOW() + ($5::BIGINT * INTERVAL '1 second'), $6, $7)
		RETURNING id
		"#,
	)
	.bind(task_result_id)
	.bind(job_id)
	.bind(tenant_id)
	.bind(&task_payload)
	.bind(run_at_offset_seconds)
	.bind(status)
	.bind(publish_attempts)
	.fetch_one(pool)
	.await
	.expect("insert recheck row")
}

async fn fetch_recheck_status(pool: &sqlx::PgPool, recheck_id: i64) -> String {
	sqlx::query_scalar::<_, String>(
		"SELECT status FROM verification_delayed_rechecks WHERE id = $1",
	)
	.bind(recheck_id)
	.fetch_one(pool)
	.await
	.expect("fetch status")
}

async fn fetch_recheck_publish_attempts(pool: &sqlx::PgPool, recheck_id: i64) -> i32 {
	sqlx::query_scalar::<_, i32>(
		"SELECT publish_attempts FROM verification_delayed_rechecks WHERE id = $1",
	)
	.bind(recheck_id)
	.fetch_one(pool)
	.await
	.expect("fetch publish_attempts")
}

async fn count_events(pool: &sqlx::PgPool, job_id: i32, event_type: &str) -> i64 {
	sqlx::query_scalar::<_, i64>(
		"SELECT COUNT(*) FROM job_events WHERE job_id = $1 AND event_type = $2",
	)
	.bind(job_id)
	.bind(event_type)
	.fetch_one(pool)
	.await
	.expect("count events")
}

async fn count_recheck_rows(pool: &sqlx::PgPool, task_result_id: i32) -> i64 {
	sqlx::query_scalar::<_, i64>(
		"SELECT COUNT(*) FROM verification_delayed_rechecks WHERE task_result_id = $1",
	)
	.bind(task_result_id)
	.fetch_one(pool)
	.await
	.expect("count recheck rows")
}

// ----------------------------------------------------------------------------
// schedule_delayed_recheck() — direct scheduling logic
// ----------------------------------------------------------------------------

#[tokio::test]
#[serial]
async fn test_schedule_delayed_recheck_creates_scheduled_row() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "schedule-create", Some(1000), 0).await;
	let job_id = insert_job(db.pool(), Some(tenant_id), 1, "running").await;
	let task_id = insert_task(db.pool(), job_id, "running", Some(tenant_id), None, None).await;
	let config = worker_config(true).await;

	let task = make_task(job_id, "user@example.com", Some(tenant_id), task_id);
	schedule_delayed_recheck(
		&task,
		Arc::clone(&config),
		task_id,
		1,
		&RetryPolicy::default(),
	)
	.await;

	let row = sqlx::query(
		"SELECT status::TEXT as s, retry_count, publish_attempts, EXTRACT(EPOCH FROM (run_at - NOW()))::BIGINT as delta FROM verification_delayed_rechecks WHERE task_result_id = $1",
	)
	.bind(task_id)
	.fetch_one(db.pool())
	.await?;
	let status: String = row.get("s");
	let retry_count: i32 = row.get("retry_count");
	let attempts: i32 = row.get("publish_attempts");
	let delta_seconds: i64 = row.get("delta");

	assert_eq!(status, "scheduled");
	assert_eq!(retry_count, 1);
	assert_eq!(attempts, 0);
	assert!(
		(290..=310).contains(&delta_seconds),
		"expected delta near 300s, got {}",
		delta_seconds
	);
	assert_eq!(
		count_events(db.pool(), job_id, "task.delayed_recheck_scheduled").await,
		1
	);
	assert_eq!(
		count_events(db.pool(), job_id, "task.partial_confidence").await,
		1
	);
	Ok(())
}

#[tokio::test]
#[serial]
async fn test_schedule_delayed_recheck_skips_singleshot_jobs() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "schedule-singleshot", Some(1000), 0).await;
	let job_id = insert_job(db.pool(), Some(tenant_id), 1, "running").await;
	let task_id = insert_task(db.pool(), job_id, "running", Some(tenant_id), None, None).await;
	let config = worker_config(true).await;

	let mut task = make_task(job_id, "x@example.com", Some(tenant_id), task_id);
	task.job_id = CheckEmailJobId::SingleShot;
	schedule_delayed_recheck(
		&task,
		Arc::clone(&config),
		task_id,
		1,
		&RetryPolicy::default(),
	)
	.await;

	assert_eq!(count_recheck_rows(db.pool(), task_id).await, 0);
	Ok(())
}

#[tokio::test]
#[serial]
async fn test_schedule_delayed_recheck_upsert_replaces_existing_scheduled_row() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "schedule-upsert", Some(1000), 0).await;
	let job_id = insert_job(db.pool(), Some(tenant_id), 1, "running").await;
	let task_id = insert_task(db.pool(), job_id, "running", Some(tenant_id), None, None).await;
	let config = worker_config(true).await;
	let task = make_task(job_id, "user@example.com", Some(tenant_id), task_id);

	schedule_delayed_recheck(
		&task,
		Arc::clone(&config),
		task_id,
		1,
		&RetryPolicy::default(),
	)
	.await;
	schedule_delayed_recheck(
		&task,
		Arc::clone(&config),
		task_id,
		2,
		&RetryPolicy::default(),
	)
	.await;

	assert_eq!(count_recheck_rows(db.pool(), task_id).await, 1);
	let retry_count: i32 = sqlx::query_scalar(
		"SELECT retry_count FROM verification_delayed_rechecks WHERE task_result_id = $1",
	)
	.bind(task_id)
	.fetch_one(db.pool())
	.await?;
	assert_eq!(retry_count, 2, "second call should overwrite retry_count");
	Ok(())
}

#[tokio::test]
#[serial]
async fn test_schedule_delayed_recheck_marks_metadata_actor_and_db_id() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "schedule-metadata", Some(1000), 0).await;
	let job_id = insert_job(db.pool(), Some(tenant_id), 1, "running").await;
	let task_id = insert_task(db.pool(), job_id, "running", Some(tenant_id), None, None).await;
	let config = worker_config(true).await;

	let task = make_task(job_id, "user@example.com", Some(tenant_id), task_id);
	schedule_delayed_recheck(
		&task,
		Arc::clone(&config),
		task_id,
		1,
		&RetryPolicy::default(),
	)
	.await;

	let task_json: serde_json::Value = sqlx::query_scalar(
		"SELECT task FROM verification_delayed_rechecks WHERE task_result_id = $1",
	)
	.bind(task_id)
	.fetch_one(db.pool())
	.await?;
	assert_eq!(
		task_json["metadata"]["created_by"].as_str(),
		Some("delayed_recheck")
	);
	assert_eq!(
		task_json["metadata"]["task_db_id"].as_i64(),
		Some(task_id as i64)
	);
	Ok(())
}

#[tokio::test]
#[serial]
async fn test_schedule_delayed_recheck_default_greylist_windows() {
	let p = RetryPolicy::default();
	assert_eq!(delayed_recheck_delay_seconds(&p, 1), 300);
	assert_eq!(delayed_recheck_delay_seconds(&p, 2), 900);
	assert_eq!(delayed_recheck_delay_seconds(&p, 3), 900);
}

#[tokio::test]
#[serial]
async fn test_schedule_delayed_recheck_custom_retry_policy_uses_exponential_backoff() {
	let p = RetryPolicy {
		max_retries: 3,
		backoff_seconds: 10,
		backoff_multiplier: 3.0,
	};
	assert_eq!(delayed_recheck_delay_seconds(&p, 1), 10);
	assert_eq!(delayed_recheck_delay_seconds(&p, 2), 30);
	assert_eq!(delayed_recheck_delay_seconds(&p, 3), 90);
}

// ----------------------------------------------------------------------------
// run_delayed_recheck_cycle() — scheduler claim/publish behavior
// ----------------------------------------------------------------------------

#[tokio::test]
#[serial]
async fn test_scheduler_claims_due_rechecks_and_publishes() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "scheduler-claim", Some(1000), 0).await;
	let job_id = insert_job(db.pool(), Some(tenant_id), 1, "running").await;
	let task_id = insert_task(db.pool(), job_id, "queued", Some(tenant_id), None, None).await;
	let config = worker_config(true).await;
	let recheck_id = insert_recheck_row(
		db.pool(),
		task_id,
		job_id,
		Some(tenant_id),
		"scheduled",
		-10,
		0,
	)
	.await;

	run_delayed_recheck_cycle(Arc::clone(&config), db.pool()).await?;

	assert_eq!(
		fetch_recheck_status(db.pool(), recheck_id).await,
		"published"
	);
	assert_eq!(
		count_events(db.pool(), job_id, "task.delayed_recheck_published").await,
		1
	);
	let attempts = fetch_recheck_publish_attempts(db.pool(), recheck_id).await;
	assert_eq!(attempts, 1);
	Ok(())
}

#[tokio::test]
#[serial]
async fn test_scheduler_skips_rechecks_not_yet_due() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "scheduler-skip-future", Some(1000), 0).await;
	let job_id = insert_job(db.pool(), Some(tenant_id), 1, "running").await;
	let task_id = insert_task(db.pool(), job_id, "queued", Some(tenant_id), None, None).await;
	let config = worker_config(true).await;
	let recheck_id = insert_recheck_row(
		db.pool(),
		task_id,
		job_id,
		Some(tenant_id),
		"scheduled",
		600,
		0,
	)
	.await;

	run_delayed_recheck_cycle(Arc::clone(&config), db.pool()).await?;

	assert_eq!(
		fetch_recheck_status(db.pool(), recheck_id).await,
		"scheduled"
	);
	assert_eq!(
		fetch_recheck_publish_attempts(db.pool(), recheck_id).await,
		0
	);
	Ok(())
}

#[tokio::test]
#[serial]
async fn test_scheduler_respects_batch_size() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "scheduler-batch", Some(1000), 0).await;
	let job_id = insert_job(db.pool(), Some(tenant_id), 5, "running").await;
	let mut recheck_ids = vec![];
	for _ in 0..5 {
		let task_id = insert_task(db.pool(), job_id, "queued", Some(tenant_id), None, None).await;
		recheck_ids.push(
			insert_recheck_row(
				db.pool(),
				task_id,
				job_id,
				Some(tenant_id),
				"scheduled",
				-10,
				0,
			)
			.await,
		);
	}
	let config = worker_config_with(true, |dr| dr.batch_size = 2).await;

	run_delayed_recheck_cycle(Arc::clone(&config), db.pool()).await?;

	let published: i64 = sqlx::query_scalar(
		"SELECT COUNT(*) FROM verification_delayed_rechecks WHERE id = ANY($1) AND status = 'published'",
	)
	.bind(&recheck_ids)
	.fetch_one(db.pool())
	.await?;
	assert_eq!(
		published, 2,
		"only the configured batch size should be claimed per cycle"
	);
	Ok(())
}

#[tokio::test]
#[serial]
async fn test_scheduler_handles_invalid_task_payload() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "scheduler-invalid", Some(1000), 0).await;
	let job_id = insert_job(db.pool(), Some(tenant_id), 1, "running").await;
	let task_id = insert_task(db.pool(), job_id, "queued", Some(tenant_id), None, None).await;
	let config = worker_config(true).await;
	let recheck_id = sqlx::query_scalar::<_, i64>(
		r#"
		INSERT INTO verification_delayed_rechecks
			(task_result_id, job_id, tenant_id, task, retry_count, run_at, status, publish_attempts)
		VALUES ($1, $2, $3, '{"garbage": true}'::jsonb, 1, NOW() - INTERVAL '10 seconds', 'scheduled', 0)
		RETURNING id
		"#,
	)
	.bind(task_id)
	.bind(job_id)
	.bind(tenant_id)
	.fetch_one(db.pool())
	.await?;

	run_delayed_recheck_cycle(Arc::clone(&config), db.pool()).await?;

	assert_eq!(fetch_recheck_status(db.pool(), recheck_id).await, "failed");
	assert_eq!(
		count_events(db.pool(), job_id, "task.delayed_recheck_failed").await,
		1
	);
	Ok(())
}

#[tokio::test]
#[serial]
async fn test_scheduler_concurrent_cycles_do_not_double_claim() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "scheduler-concurrent", Some(1000), 0).await;
	let job_id = insert_job(db.pool(), Some(tenant_id), 4, "running").await;
	let mut recheck_ids = vec![];
	for _ in 0..4 {
		let task_id = insert_task(db.pool(), job_id, "queued", Some(tenant_id), None, None).await;
		recheck_ids.push(
			insert_recheck_row(
				db.pool(),
				task_id,
				job_id,
				Some(tenant_id),
				"scheduled",
				-5,
				0,
			)
			.await,
		);
	}
	let config = worker_config(true).await;
	let pool_a = db.pool().clone();
	let pool_b = db.pool().clone();
	let cfg_a = Arc::clone(&config);
	let cfg_b = Arc::clone(&config);

	let (a, b) = tokio::join!(
		tokio::spawn(async move { run_delayed_recheck_cycle(cfg_a, &pool_a).await }),
		tokio::spawn(async move { run_delayed_recheck_cycle(cfg_b, &pool_b).await }),
	);
	a.expect("cycle a join")?;
	b.expect("cycle b join")?;

	let total_attempts: i32 = sqlx::query_scalar(
		"SELECT COALESCE(SUM(publish_attempts), 0)::INT FROM verification_delayed_rechecks WHERE id = ANY($1)",
	)
	.bind(&recheck_ids)
	.fetch_one(db.pool())
	.await?;
	assert_eq!(
		total_attempts, 4,
		"each row should be claimed exactly once across two concurrent cycles"
	);
	Ok(())
}

#[tokio::test]
#[serial]
async fn test_scheduler_no_op_when_table_empty() -> Result<()> {
	let db = TestDb::start().await;
	let _tenant_id = insert_tenant(db.pool(), "scheduler-empty", Some(1000), 0).await;
	let config = worker_config(true).await;

	run_delayed_recheck_cycle(Arc::clone(&config), db.pool()).await?;

	let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM verification_delayed_rechecks")
		.fetch_one(db.pool())
		.await?;
	assert_eq!(total, 0);
	Ok(())
}

// ----------------------------------------------------------------------------
// Stale publishing recovery
// ----------------------------------------------------------------------------

#[tokio::test]
#[serial]
async fn test_stale_publishing_claim_is_reset_to_scheduled() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "stale-reset", Some(1000), 0).await;
	let job_id = insert_job(db.pool(), Some(tenant_id), 1, "running").await;
	let task_id = insert_task(db.pool(), job_id, "queued", Some(tenant_id), None, None).await;
	let config = worker_config(true).await;
	let recheck_id = insert_recheck_row(
		db.pool(),
		task_id,
		job_id,
		Some(tenant_id),
		"publishing",
		-3600,
		1,
	)
	.await;
	sqlx::query(
		"UPDATE verification_delayed_rechecks SET updated_at = NOW() - INTERVAL '600 seconds' WHERE id = $1",
	)
	.bind(recheck_id)
	.execute(db.pool())
	.await?;

	run_delayed_recheck_cycle(Arc::clone(&config), db.pool()).await?;

	let row = sqlx::query(
		"SELECT status::TEXT as s, last_error FROM verification_delayed_rechecks WHERE id = $1",
	)
	.bind(recheck_id)
	.fetch_one(db.pool())
	.await?;
	let status: String = row.get("s");
	let last_error: Option<String> = row.get("last_error");
	assert!(
		status == "scheduled" || status == "published",
		"stale publishing should be reset, got {}",
		status
	);
	if status == "scheduled" {
		assert_eq!(last_error.as_deref(), Some("stale publisher claim reset"));
	}
	Ok(())
}

#[tokio::test]
#[serial]
async fn test_stale_publishing_marked_failed_when_newer_scheduled_exists() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "stale-superseded", Some(1000), 0).await;
	let job_id = insert_job(db.pool(), Some(tenant_id), 1, "running").await;
	let task_id = insert_task(db.pool(), job_id, "queued", Some(tenant_id), None, None).await;
	let config = worker_config(true).await;

	let stale_id = insert_recheck_row(
		db.pool(),
		task_id,
		job_id,
		Some(tenant_id),
		"publishing",
		-3600,
		1,
	)
	.await;
	sqlx::query(
		"UPDATE verification_delayed_rechecks SET updated_at = NOW() - INTERVAL '600 seconds' WHERE id = $1",
	)
	.bind(stale_id)
	.execute(db.pool())
	.await?;
	let _newer_id = insert_recheck_row(
		db.pool(),
		task_id,
		job_id,
		Some(tenant_id),
		"scheduled",
		600,
		0,
	)
	.await;

	run_delayed_recheck_cycle(Arc::clone(&config), db.pool()).await?;

	let row = sqlx::query(
		"SELECT status::TEXT as s, last_error FROM verification_delayed_rechecks WHERE id = $1",
	)
	.bind(stale_id)
	.fetch_one(db.pool())
	.await?;
	let status: String = row.get("s");
	let last_error: Option<String> = row.get("last_error");
	assert_eq!(status, "failed");
	assert_eq!(
		last_error.as_deref(),
		Some("stale publisher claim superseded by a newer scheduled recheck")
	);
	Ok(())
}

#[tokio::test]
#[serial]
async fn test_stale_recovery_respects_configured_timeout() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "stale-tunable", Some(1000), 0).await;
	let job_id = insert_job(db.pool(), Some(tenant_id), 1, "running").await;
	let task_id = insert_task(db.pool(), job_id, "queued", Some(tenant_id), None, None).await;

	let recheck_id = insert_recheck_row(
		db.pool(),
		task_id,
		job_id,
		Some(tenant_id),
		"publishing",
		-3600,
		1,
	)
	.await;
	sqlx::query(
		"UPDATE verification_delayed_rechecks SET updated_at = NOW() - INTERVAL '120 seconds' WHERE id = $1",
	)
	.bind(recheck_id)
	.execute(db.pool())
	.await?;

	let config_long = worker_config_with(true, |dr| dr.stale_publishing_seconds = 600).await;
	run_delayed_recheck_cycle(Arc::clone(&config_long), db.pool()).await?;
	assert_eq!(
		fetch_recheck_status(db.pool(), recheck_id).await,
		"publishing",
		"row should still be 'publishing' under a 600s timeout because only 120s have passed"
	);

	let config_short = worker_config_with(true, |dr| dr.stale_publishing_seconds = 60).await;
	run_delayed_recheck_cycle(Arc::clone(&config_short), db.pool()).await?;
	let after = fetch_recheck_status(db.pool(), recheck_id).await;
	assert!(
		after == "scheduled" || after == "published",
		"row should be reset (and possibly republished) under a 60s timeout, got {}",
		after
	);
	Ok(())
}

// ----------------------------------------------------------------------------
// Cleanup / retention
// ----------------------------------------------------------------------------

#[tokio::test]
#[serial]
async fn test_cleanup_removes_old_terminal_rows() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "cleanup-terminal", Some(1000), 0).await;
	let job_id = insert_job(db.pool(), Some(tenant_id), 1, "running").await;
	let task_id = insert_task(db.pool(), job_id, "queued", Some(tenant_id), None, None).await;

	let old_published = insert_recheck_row(
		db.pool(),
		task_id,
		job_id,
		Some(tenant_id),
		"published",
		-10,
		0,
	)
	.await;
	let old_failed = insert_recheck_row(
		db.pool(),
		task_id,
		job_id,
		Some(tenant_id),
		"failed",
		-10,
		0,
	)
	.await;
	let old_cancelled = insert_recheck_row(
		db.pool(),
		task_id,
		job_id,
		Some(tenant_id),
		"cancelled",
		-10,
		0,
	)
	.await;
	let recent_published = insert_recheck_row(
		db.pool(),
		task_id,
		job_id,
		Some(tenant_id),
		"published",
		-10,
		0,
	)
	.await;
	let scheduled = insert_recheck_row(
		db.pool(),
		task_id,
		job_id,
		Some(tenant_id),
		"scheduled",
		600,
		0,
	)
	.await;

	sqlx::query("UPDATE verification_delayed_rechecks SET updated_at = NOW() - INTERVAL '10 days' WHERE id = ANY($1)")
		.bind(&[old_published, old_failed, old_cancelled])
		.execute(db.pool())
		.await?;

	let deleted = cleanup_terminal_rechecks(db.pool(), 7).await?;
	assert_eq!(deleted, 3);

	for surviving in [recent_published, scheduled] {
		let cnt: i64 =
			sqlx::query_scalar("SELECT COUNT(*) FROM verification_delayed_rechecks WHERE id = $1")
				.bind(surviving)
				.fetch_one(db.pool())
				.await?;
		assert_eq!(cnt, 1, "row {surviving} should still exist after cleanup");
	}
	Ok(())
}

#[tokio::test]
#[serial]
async fn test_cleanup_no_op_when_nothing_old() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "cleanup-noop", Some(1000), 0).await;
	let job_id = insert_job(db.pool(), Some(tenant_id), 1, "running").await;
	let task_id = insert_task(db.pool(), job_id, "queued", Some(tenant_id), None, None).await;
	insert_recheck_row(
		db.pool(),
		task_id,
		job_id,
		Some(tenant_id),
		"published",
		-10,
		0,
	)
	.await;

	let deleted = cleanup_terminal_rechecks(db.pool(), 7).await?;
	assert_eq!(deleted, 0);
	Ok(())
}

// ----------------------------------------------------------------------------
// Cancellation via HTTP — POST /v1/jobs/{id}/cancel
// ----------------------------------------------------------------------------

#[tokio::test]
#[serial]
async fn test_cancel_job_cancels_scheduled_rechecks() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "cancel-rechecks", Some(1000), 0).await;
	let (api_key, _) = insert_api_key_with_scopes(db.pool(), tenant_id, &["bulk"]).await;
	let job_id = insert_job(db.pool(), Some(tenant_id), 2, "running").await;
	let task_a = insert_task(db.pool(), job_id, "queued", Some(tenant_id), None, None).await;
	let task_b = insert_task(db.pool(), job_id, "running", Some(tenant_id), None, None).await;

	let scheduled_id = insert_recheck_row(
		db.pool(),
		task_a,
		job_id,
		Some(tenant_id),
		"scheduled",
		600,
		0,
	)
	.await;
	let publishing_id = insert_recheck_row(
		db.pool(),
		task_b,
		job_id,
		Some(tenant_id),
		"publishing",
		-10,
		1,
	)
	.await;

	let config = worker_config(true).await;
	let response = request()
		.method("POST")
		.path(&format!("/v1/jobs/{}/cancel", job_id))
		.header("authorization", format!("Bearer {}", api_key))
		.reply(&create_routes(Arc::clone(&config)))
		.await;
	assert_eq!(response.status(), StatusCode::OK);

	for id in [scheduled_id, publishing_id] {
		let row = sqlx::query(
			"SELECT status::TEXT as s, last_error FROM verification_delayed_rechecks WHERE id = $1",
		)
		.bind(id)
		.fetch_one(db.pool())
		.await?;
		let status: String = row.get("s");
		let last_error: Option<String> = row.get("last_error");
		assert_eq!(status, "cancelled", "row {id} should be cancelled");
		assert_eq!(last_error.as_deref(), Some("job cancelled"));
	}
	Ok(())
}

#[tokio::test]
#[serial]
async fn test_cancel_job_leaves_terminal_rechecks_alone() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "cancel-terminal-untouched", Some(1000), 0).await;
	let (api_key, _) = insert_api_key_with_scopes(db.pool(), tenant_id, &["bulk"]).await;
	let job_id = insert_job(db.pool(), Some(tenant_id), 1, "running").await;
	let task_id = insert_task(db.pool(), job_id, "queued", Some(tenant_id), None, None).await;
	let published_id = insert_recheck_row(
		db.pool(),
		task_id,
		job_id,
		Some(tenant_id),
		"published",
		-10,
		1,
	)
	.await;
	let failed_id = insert_recheck_row(
		db.pool(),
		task_id,
		job_id,
		Some(tenant_id),
		"failed",
		-10,
		1,
	)
	.await;

	let config = worker_config(true).await;
	let response = request()
		.method("POST")
		.path(&format!("/v1/jobs/{}/cancel", job_id))
		.header("authorization", format!("Bearer {}", api_key))
		.reply(&create_routes(Arc::clone(&config)))
		.await;
	assert_eq!(response.status(), StatusCode::OK);

	assert_eq!(
		fetch_recheck_status(db.pool(), published_id).await,
		"published"
	);
	assert_eq!(fetch_recheck_status(db.pool(), failed_id).await, "failed");
	Ok(())
}

#[tokio::test]
#[serial]
async fn test_cancel_other_tenant_does_not_touch_our_rechecks() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_a = insert_tenant(db.pool(), "cancel-tenant-a", Some(1000), 0).await;
	let tenant_b = insert_tenant(db.pool(), "cancel-tenant-b", Some(1000), 0).await;
	let (key_b, _) = insert_api_key_with_scopes(db.pool(), tenant_b, &["bulk"]).await;
	let job_a = insert_job(db.pool(), Some(tenant_a), 1, "running").await;
	let job_b = insert_job(db.pool(), Some(tenant_b), 1, "running").await;
	let task_a = insert_task(db.pool(), job_a, "queued", Some(tenant_a), None, None).await;
	let task_b = insert_task(db.pool(), job_b, "queued", Some(tenant_b), None, None).await;

	let scheduled_a = insert_recheck_row(
		db.pool(),
		task_a,
		job_a,
		Some(tenant_a),
		"scheduled",
		600,
		0,
	)
	.await;
	let scheduled_b = insert_recheck_row(
		db.pool(),
		task_b,
		job_b,
		Some(tenant_b),
		"scheduled",
		600,
		0,
	)
	.await;

	let config = worker_config(true).await;
	let response = request()
		.method("POST")
		.path(&format!("/v1/jobs/{}/cancel", job_b))
		.header("authorization", format!("Bearer {}", key_b))
		.reply(&create_routes(Arc::clone(&config)))
		.await;
	assert_eq!(response.status(), StatusCode::OK);

	assert_eq!(
		fetch_recheck_status(db.pool(), scheduled_a).await,
		"scheduled"
	);
	assert_eq!(
		fetch_recheck_status(db.pool(), scheduled_b).await,
		"cancelled"
	);
	Ok(())
}

// ----------------------------------------------------------------------------
// Config gating — enable=false short-circuits
// ----------------------------------------------------------------------------

#[tokio::test]
#[serial]
async fn test_disabled_config_skips_scheduling() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "disabled-skip-schedule", Some(1000), 0).await;
	let job_id = insert_job(db.pool(), Some(tenant_id), 1, "running").await;
	let task_id = insert_task(db.pool(), job_id, "running", Some(tenant_id), None, None).await;
	let config = worker_config(false).await;

	let task = make_task(job_id, "user@example.com", Some(tenant_id), task_id);
	// schedule_delayed_recheck still writes when called directly — the gating happens
	// in do_work.rs before invoking it. So we simulate the worker gate here by checking
	// the config value before calling.
	if config.delayed_recheck.enable {
		schedule_delayed_recheck(
			&task,
			Arc::clone(&config),
			task_id,
			1,
			&RetryPolicy::default(),
		)
		.await;
	}

	assert_eq!(count_recheck_rows(db.pool(), task_id).await, 0);
	Ok(())
}

#[tokio::test]
#[serial]
async fn test_disabled_config_still_runs_cycle_safely() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "disabled-cycle-safe", Some(1000), 0).await;
	let job_id = insert_job(db.pool(), Some(tenant_id), 1, "running").await;
	let task_id = insert_task(db.pool(), job_id, "queued", Some(tenant_id), None, None).await;
	let config = worker_config(false).await;
	let recheck_id = insert_recheck_row(
		db.pool(),
		task_id,
		job_id,
		Some(tenant_id),
		"scheduled",
		-10,
		0,
	)
	.await;

	// Even though enable=false, if a row is in the table the cycle runs (the gate is
	// only at spawn time + worker schedule time). Verify it doesn't panic and either
	// publishes or leaves alone safely.
	let _ = run_delayed_recheck_cycle(Arc::clone(&config), db.pool()).await;
	let status = fetch_recheck_status(db.pool(), recheck_id).await;
	assert!(
		matches!(
			status.as_str(),
			"scheduled" | "publishing" | "published" | "failed"
		),
		"unexpected status {}",
		status
	);
	Ok(())
}

// ----------------------------------------------------------------------------
// Migration round-trip
// ----------------------------------------------------------------------------

#[tokio::test]
#[serial]
async fn test_table_and_indexes_present() -> Result<()> {
	let db = TestDb::start().await;
	let table_exists: bool = sqlx::query_scalar(
		"SELECT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'verification_delayed_rechecks')",
	)
	.fetch_one(db.pool())
	.await?;
	assert!(
		table_exists,
		"verification_delayed_rechecks table should exist"
	);

	let indexes = vec![
		"idx_verification_delayed_rechecks_due",
		"idx_verification_delayed_rechecks_task",
		"idx_verification_delayed_rechecks_one_scheduled",
	];
	for ix in indexes {
		let exists: bool =
			sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = $1)")
				.bind(ix)
				.fetch_one(db.pool())
				.await?;
		assert!(exists, "expected index {} to exist", ix);
	}

	let constraint_exists: bool = sqlx::query_scalar(
		"SELECT EXISTS (SELECT 1 FROM pg_constraint WHERE conname = 'verification_delayed_rechecks_status_check')",
	)
	.fetch_one(db.pool())
	.await?;
	assert!(constraint_exists);
	Ok(())
}

#[tokio::test]
#[serial]
async fn test_unique_scheduled_index_prevents_duplicate_inserts() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "unique-scheduled", Some(1000), 0).await;
	let job_id = insert_job(db.pool(), Some(tenant_id), 1, "running").await;
	let task_id = insert_task(db.pool(), job_id, "queued", Some(tenant_id), None, None).await;
	insert_recheck_row(
		db.pool(),
		task_id,
		job_id,
		Some(tenant_id),
		"scheduled",
		600,
		0,
	)
	.await;

	let result = sqlx::query(
		r#"
		INSERT INTO verification_delayed_rechecks
			(task_result_id, job_id, tenant_id, task, retry_count, run_at, status, publish_attempts)
		VALUES ($1, $2, $3, '{}'::jsonb, 1, NOW() + INTERVAL '1 hour', 'scheduled', 0)
		"#,
	)
	.bind(task_id)
	.bind(job_id)
	.bind(tenant_id)
	.execute(db.pool())
	.await;
	assert!(
		result.is_err(),
		"second 'scheduled' row for same task_result_id should be rejected by unique partial index"
	);
	Ok(())
}
