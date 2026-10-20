mod resilience_helpers;
mod test_helpers;

use crate::resilience_helpers::OwnedResilienceEnv;
use crate::test_helpers::{build_test_config, insert_job, insert_task, ConfigProfile, TEST_SECRET};
use futures::StreamExt;
use lapin::options::{BasicConsumeOptions, BasicPublishOptions, QueuePurgeOptions};
use lapin::types::FieldTable;
use lapin::BasicProperties;
use reacher_backend::config::RabbitMQConfig;
use reacher_backend::http::{create_routes, REACHER_SECRET_HEADER};
use reacher_backend::worker::consume::{setup_rabbit_mq, CHECK_EMAIL_QUEUE};
use reacher_backend::worker::do_work::{
	do_check_email_work, CheckEmailJobId, CheckEmailTask, TaskMetadata,
};
use serial_test::serial;
use sqlx::Row;
use std::sync::Arc;
use std::time::Duration;
use warp::http::StatusCode;
use warp::test::request;

fn parse_json(body: &[u8]) -> serde_json::Value {
	serde_json::from_slice(body).expect("response should be valid json")
}

#[tokio::test]
#[serial]
async fn readiness_reports_postgres_unavailable_after_outage() {
	let env = OwnedResilienceEnv::start().await;
	let config = build_test_config(ConfigProfile::DbOnly, Some(env.postgres.db_url()), None).await;

	env.postgres.kill().await;

	let response = request()
		.method("GET")
		.path("/readyz")
		.reply(&create_routes(Arc::clone(&config)))
		.await;

	assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
	let body = parse_json(response.body());
	assert_eq!(body["status"], "unavailable");
	assert_eq!(body["checks"]["postgres"]["status"], "unavailable");
	assert_eq!(body["checks"]["rabbitmq"]["status"], "not_configured");
}

#[tokio::test]
#[serial]
async fn comment_write_failure_returns_structured_error_without_partial_comment_insert() {
	let env = OwnedResilienceEnv::start().await;
	let pool = env.postgres.pool().await;
	let job_id = insert_job(&pool, None, 1, "completed").await;
	pool.close().await;

	let config = build_test_config(
		ConfigProfile::PseudoWorker,
		Some(env.postgres.db_url()),
		None,
	)
	.await;

	std::env::set_var("RCH_TEST_FORCE_COMMENT_DB_ERROR", "1");

	let response = tokio::time::timeout(
		Duration::from_secs(5),
		request()
			.method("POST")
			.path("/v1/comments")
			.header(REACHER_SECRET_HEADER, TEST_SECRET)
			.json(&serde_json::json!({
				"job_id": job_id,
				"body": "should not persist",
				"author": "nightly"
			}))
			.reply(&create_routes(Arc::clone(&config))),
	)
	.await
	.expect("comment request should fail fast");
	std::env::remove_var("RCH_TEST_FORCE_COMMENT_DB_ERROR");

	assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
	let body = parse_json(response.body());
	assert!(
		body["error"]
			.as_str()
			.is_some_and(|message| !message.is_empty()),
		"expected structured error body: {body}",
		body = body
	);

	let pool = env.postgres.pool().await;
	let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM job_comments")
		.fetch_one(&pool)
		.await
		.expect("count comments after postgres restart");
	assert_eq!(count, 0, "comment write should not partially persist");
	pool.close().await;
}

#[tokio::test]
#[serial]
async fn readiness_reports_rabbitmq_unavailable_after_outage() {
	let env = OwnedResilienceEnv::start().await;
	let config = build_test_config(
		ConfigProfile::WorkerRabbit,
		Some(env.postgres.db_url()),
		Some(env.rabbitmq.amqp_url()),
	)
	.await;

	env.rabbitmq.stop().await;

	let response = request()
		.method("GET")
		.path("/readyz")
		.reply(&create_routes(Arc::clone(&config)))
		.await;

	assert_eq!(response.status(), StatusCode::SERVICE_UNAVAILABLE);
	let body = parse_json(response.body());
	assert_eq!(body["status"], "unavailable");
	assert_eq!(body["checks"]["postgres"]["status"], "ok");
	assert_eq!(body["checks"]["rabbitmq"]["status"], "unavailable");
}

#[tokio::test]
#[serial]
async fn worker_backed_check_email_retains_pending_work_when_rabbitmq_is_down() {
	let env = OwnedResilienceEnv::start().await;
	let config = build_test_config(
		ConfigProfile::WorkerRabbit,
		Some(env.postgres.db_url()),
		Some(env.rabbitmq.amqp_url()),
	)
	.await;

	env.rabbitmq.stop().await;

	let response = tokio::time::timeout(
		Duration::from_secs(25),
		request()
			.method("POST")
			.path("/v1/check_email")
			.header(REACHER_SECRET_HEADER, TEST_SECRET)
			.json(&serde_json::json!({
				"to_email": "worker-path@example.com"
			}))
			.reply(&create_routes(Arc::clone(&config))),
	)
	.await
	.expect("worker-backed request must return its pending operation");

	assert_eq!(response.status(), StatusCode::ACCEPTED);
	let body = parse_json(response.body());
	assert_eq!(body["status"], "pending");
	assert!(body["operation_id"].is_string());
	let pool = env.postgres.pool().await;
	let pending: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM verification_dispatch")
		.fetch_one(&pool)
		.await
		.unwrap();
	assert_eq!(pending, 1);
	pool.close().await;
}

#[tokio::test]
#[serial]
async fn killed_worker_before_ack_does_not_duplicate_completion_side_effects() {
	let env = OwnedResilienceEnv::start().await;
	let pool = env.postgres.pool().await;
	let job_id = insert_job(&pool, None, 1, "running").await;
	let task_db_id = insert_task(&pool, job_id, "queued", None, None, None).await;
	pool.close().await;

	let rabbitmq_config = RabbitMQConfig {
		url: env.rabbitmq.amqp_url().to_string(),
		concurrency: 4,
	};
	let publisher = setup_rabbit_mq("mid-job-publisher", &rabbitmq_config)
		.await
		.expect("publisher channel");
	let _ = publisher
		.queue_purge(CHECK_EMAIL_QUEUE.into(), QueuePurgeOptions::default())
		.await;

	let task = CheckEmailTask {
		verification_backend: Default::default(),
		input: check_if_email_exists::CheckEmailInput {
			to_email: "bad-syntax".into(),
			..Default::default()
		},
		job_id: CheckEmailJobId::Bulk(job_id),
		webhook: None,
		metadata: Some(TaskMetadata {
			dispatch_id: None,
			lease_id: None,
			decision_policy: None,
			tenant_id: None,
			request_id: None,
			correlation_id: None,
			created_by: None,
			retry_policy: None,
			dedupe_key: None,
			task_db_id: Some(task_db_id),
		}),
	};

	let task_json = serde_json::to_vec(&task).expect("serialize task");
	publisher
		.basic_publish(
			"".into(),
			CHECK_EMAIL_QUEUE.into(),
			BasicPublishOptions::default(),
			&task_json,
			BasicProperties::default()
				.with_content_type("application/json".into())
				.with_priority(1),
		)
		.await
		.expect("publish task")
		.await
		.expect("confirm publish");

	let mut child = tokio::process::Command::new(std::env::current_exe().unwrap())
		.args(["--exact", "owned_worker_child", "--nocapture"])
		.env("RCH_TEST_WORKER_CHILD", "1")
		.env("RCH_TEST_CHILD_DATABASE_URL", env.postgres.db_url())
		.env("RCH_TEST_CHILD_AMQP_URL", env.rabbitmq.amqp_url())
		.env("RCH_TEST_PRE_ACK_DELAY_MS", "30000")
		.stdout(std::process::Stdio::null())
		.stderr(std::process::Stdio::null())
		.kill_on_drop(true)
		.spawn()
		.unwrap();
	let pool = env.postgres.pool().await;
	let stored = tokio::time::timeout(Duration::from_secs(15), async {
		loop {
			let done: bool =
				sqlx::query_scalar("SELECT completion_done FROM v1_task_result WHERE id = $1")
					.bind(task_db_id)
					.fetch_one(&pool)
					.await
					.unwrap();
			if done {
				break;
			}
			tokio::time::sleep(Duration::from_millis(50)).await;
		}
	})
	.await;
	child
		.kill()
		.await
		.expect("kill owned worker process before broker ack");
	child.wait().await.unwrap();
	stored.expect("worker must durably complete before being killed");
	pool.close().await;

	let consumer = setup_rabbit_mq("mid-job-consumer", &rabbitmq_config)
		.await
		.unwrap();
	let mut deliveries = consumer
		.basic_consume(
			CHECK_EMAIL_QUEUE.into(),
			"mid-job-consumer".into(),
			BasicConsumeOptions::default(),
			FieldTable::default(),
		)
		.await
		.unwrap();

	let recovery_delivery = tokio::time::timeout(Duration::from_secs(10), deliveries.next())
		.await
		.expect("receive requeued delivery")
		.expect("requeued delivery stream item")
		.expect("lapin requeued delivery");

	let recovery_config = build_test_config(
		ConfigProfile::WorkerRabbit,
		Some(env.postgres.db_url()),
		Some(env.rabbitmq.amqp_url()),
	)
	.await;
	let recovery_channel = recovery_config
		.must_worker_config()
		.expect("worker config after restart")
		.channel;
	do_check_email_work(
		&task,
		recovery_delivery,
		recovery_channel,
		Arc::clone(&recovery_config),
	)
	.await
	.expect("requeued task should complete");

	let pool = env.postgres.pool().await;
	let task_state: String =
		sqlx::query("SELECT task_state::TEXT FROM v1_task_result WHERE id = $1")
			.bind(task_db_id)
			.fetch_one(&pool)
			.await
			.expect("fetch task state")
			.get(0);
	assert_eq!(task_state, "completed");

	let completed_events: i64 = sqlx::query_scalar(
		"SELECT COUNT(*) FROM job_events WHERE job_id = $1 AND task_id = $2 AND event_type = 'task.completed'",
	)
	.bind(job_id)
	.bind(task_db_id)
	.fetch_one(&pool)
	.await
	.expect("count completed events");
	assert_eq!(
		completed_events, 1,
		"task completion should be recorded exactly once after requeue recovery"
	);

	let running_events: i64 = sqlx::query_scalar(
		"SELECT COUNT(*) FROM v1_task_result WHERE id = $1 AND task_state = 'running'",
	)
	.bind(task_db_id)
	.fetch_one(&pool)
	.await
	.expect("count running state");
	assert_eq!(running_events, 0, "task should not remain stuck in running");

	pool.close().await;
	let _ = publisher.close(0, "test cleanup".into()).await;
	let _ = consumer.close(0, "test cleanup".into()).await;
}

#[tokio::test]
async fn owned_worker_child() {
	if std::env::var("RCH_TEST_WORKER_CHILD").as_deref() != Ok("1") {
		return;
	}
	let db = std::env::var("RCH_TEST_CHILD_DATABASE_URL").unwrap();
	let amqp = std::env::var("RCH_TEST_CHILD_AMQP_URL").unwrap();
	let config = build_test_config(ConfigProfile::WorkerRabbit, Some(&db), Some(&amqp)).await;
	reacher_backend::worker::run_worker(config).await.unwrap();
}

/// Failures that hit after work was reserved or done: the task, the quota
/// and the job must all end up accounted for.
mod settlement_after_failure {
	use super::*;
	use crate::test_helpers::{insert_keys_for_existing_tenant, insert_tenant};
	use lapin::types::AMQPValue;
	use reacher_backend::reverification::run_reverification_cycle;
	use reacher_backend::worker::do_work::DELIVERY_ATTEMPT_HEADER;

	async fn publish_task_message(channel: &lapin::Channel, payload: &[u8], attempt: Option<u32>) {
		let mut properties = BasicProperties::default().with_priority(1);
		if let Some(attempt) = attempt {
			let mut headers = FieldTable::default();
			headers.insert(DELIVERY_ATTEMPT_HEADER.into(), AMQPValue::LongUInt(attempt));
			properties = properties.with_headers(headers);
		}
		channel
			.basic_publish(
				"".into(),
				CHECK_EMAIL_QUEUE.into(),
				BasicPublishOptions::default(),
				payload,
				properties,
			)
			.await
			.expect("publish task")
			.await
			.expect("confirm publish");
	}

	async fn used_this_period(pool: &sqlx::PgPool, tenant_id: uuid::Uuid) -> i32 {
		sqlx::query_scalar("SELECT used_this_period FROM tenants WHERE id = $1")
			.bind(tenant_id)
			.fetch_one(pool)
			.await
			.expect("read tenant usage")
	}

	/// Deleting a list deletes its job and, by cascade, its task rows, so a
	/// task already in flight can never store its result (the fallback INSERT
	/// violates the job foreign key). The result must not be acked away, and
	/// the requeueing must stop at the cap.
	#[tokio::test]
	#[serial]
	async fn deleted_job_delivery_cannot_recreate_its_results() {
		let env = OwnedResilienceEnv::start().await;
		let pool = env.postgres.pool().await;
		let job_id = insert_job(&pool, None, 1, "running").await;
		let task_db_id = insert_task(&pool, job_id, "running", None, None, None).await;
		sqlx::query("DELETE FROM v1_bulk_job WHERE id = $1")
			.bind(job_id)
			.execute(&pool)
			.await
			.expect("delete job mid-flight");
		pool.close().await;

		let config = build_test_config(
			ConfigProfile::WorkerRabbit,
			Some(env.postgres.db_url()),
			Some(env.rabbitmq.amqp_url()),
		)
		.await;
		let channel = config.must_worker_config().expect("worker config").channel;

		let task = CheckEmailTask {
			verification_backend: Default::default(),
			input: check_if_email_exists::CheckEmailInput {
				to_email: "bad-syntax".into(),
				..Default::default()
			},
			job_id: CheckEmailJobId::Bulk(job_id),
			webhook: None,
			metadata: Some(TaskMetadata {
				dispatch_id: None,
				lease_id: None,
				decision_policy: None,
				tenant_id: None,
				request_id: None,
				correlation_id: None,
				created_by: None,
				retry_policy: None,
				dedupe_key: None,
				task_db_id: Some(task_db_id),
			}),
		};
		let payload = serde_json::to_vec(&task).expect("serialize task");
		publish_task_message(&channel, &payload, None).await;

		let mut deliveries = channel
			.basic_consume(
				CHECK_EMAIL_QUEUE.into(),
				"unstorable-result-consumer".into(),
				BasicConsumeOptions::default(),
				FieldTable::default(),
			)
			.await
			.expect("basic consume");
		let first = tokio::time::timeout(Duration::from_secs(5), deliveries.next())
			.await
			.expect("receive first delivery")
			.expect("delivery stream item")
			.expect("lapin delivery");
		do_check_email_work(&task, first, Arc::clone(&channel), Arc::clone(&config))
			.await
			.expect("deleted work is obsolete and must be acknowledged");
		assert!(
			tokio::time::timeout(Duration::from_secs(2), deliveries.next())
				.await
				.is_err()
		);
		let pool = env.postgres.pool().await;
		let rows: i64 =
			sqlx::query_scalar("SELECT COUNT(*) FROM v1_task_result WHERE id = $1 OR job_id = $2")
				.bind(task_db_id)
				.bind(job_id)
				.fetch_one(&pool)
				.await
				.unwrap();
		assert_eq!(rows, 0, "redelivery must never resurrect deleted results");
		pool.close().await;
	}

	#[tokio::test]
	#[serial]
	async fn cancelling_job_is_finalized_when_the_worker_skips_its_last_task() {
		let env = OwnedResilienceEnv::start().await;
		let pool = env.postgres.pool().await;
		let job_id = insert_job(&pool, None, 1, "cancelling").await;
		// The task was running when the cancel came in, so the cancel endpoint
		// left it alone; then its worker died and the message was redelivered.
		let task_db_id = insert_task(&pool, job_id, "running", None, None, None).await;

		let config = build_test_config(
			ConfigProfile::WorkerRabbit,
			Some(env.postgres.db_url()),
			Some(env.rabbitmq.amqp_url()),
		)
		.await;
		let task = CheckEmailTask {
			verification_backend: Default::default(),
			input: check_if_email_exists::CheckEmailInput {
				to_email: "cancelled@example.com".into(),
				..Default::default()
			},
			job_id: CheckEmailJobId::Bulk(job_id),
			webhook: None,
			metadata: Some(TaskMetadata {
				dispatch_id: None,
				lease_id: None,
				decision_policy: None,
				tenant_id: None,
				request_id: None,
				correlation_id: None,
				created_by: None,
				retry_policy: None,
				dedupe_key: None,
				task_db_id: Some(task_db_id),
			}),
		};
		let channel = config.must_worker_config().expect("worker config").channel;
		publish_task_message(
			&channel,
			&serde_json::to_vec(&task).expect("serialize task"),
			None,
		)
		.await;

		let worker = tokio::spawn(reacher_backend::worker::run_worker(Arc::clone(&config)));
		let mut job_status = String::new();
		for _ in 0..40 {
			job_status = sqlx::query_scalar("SELECT status::TEXT FROM v1_bulk_job WHERE id = $1")
				.bind(job_id)
				.fetch_one(&pool)
				.await
				.expect("read job status");
			if job_status == "cancelled" {
				break;
			}
			tokio::time::sleep(Duration::from_millis(250)).await;
		}
		worker.abort();

		let task_state: String =
			sqlx::query_scalar("SELECT task_state::TEXT FROM v1_task_result WHERE id = $1")
				.bind(task_db_id)
				.fetch_one(&pool)
				.await
				.expect("read task state");
		assert_eq!(task_state, "cancelled");
		assert_eq!(
			job_status, "cancelled",
			"the job must not stay 'cancelling'"
		);
		pool.close().await;
	}

	#[tokio::test]
	#[serial]
	async fn bulk_job_whose_task_rows_cannot_be_stored_refunds_its_quota() {
		let env = OwnedResilienceEnv::start().await;
		let pool = env.postgres.pool().await;
		let tenant_id = insert_tenant(&pool, "bulk-insert-failure", Some(100), 0).await;
		let keys = insert_keys_for_existing_tenant(&pool, tenant_id).await;
		let config = build_test_config(
			ConfigProfile::WorkerRabbit,
			Some(env.postgres.db_url()),
			Some(env.rabbitmq.amqp_url()),
		)
		.await;

		// Postgres jsonb cannot hold U+0000, so the second task row INSERT
		// fails after quota for the whole request has been reserved.
		let response = request()
			.method("POST")
			.path("/v1/bulk")
			.header("Authorization", format!("Bearer {}", keys.full_access_key))
			.json(&serde_json::json!({
				"input": ["fine@example.com", "nul\u{0000}byte@example.com"]
			}))
			.reply(&create_routes(Arc::clone(&config)))
			.await;
		assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);

		assert_eq!(used_this_period(&pool, tenant_id).await, 0);
		let job_statuses: Vec<String> =
			sqlx::query_scalar("SELECT status::TEXT FROM v1_bulk_job WHERE tenant_id = $1")
				.bind(tenant_id)
				.fetch_all(&pool)
				.await
				.expect("load jobs");
		assert!(
			job_statuses.is_empty(),
			"failed admission must roll back the parent job"
		);
		let queued: i64 = sqlx::query_scalar(
			"SELECT COUNT(*) FROM v1_task_result WHERE tenant_id = $1 AND task_state = 'queued'",
		)
		.bind(tenant_id)
		.fetch_one(&pool)
		.await
		.expect("count queued tasks");
		assert_eq!(
			queued, 0,
			"no task may stay queued for a job that was never published"
		);
		pool.close().await;
	}

	#[tokio::test]
	#[serial]
	async fn reverification_that_cannot_publish_refunds_quota_and_fails_its_job() {
		let env = OwnedResilienceEnv::start().await;
		let pool = env.postgres.pool().await;
		let tenant_id = insert_tenant(&pool, "reverify-publish-failure", Some(100), 0).await;
		let old_job = insert_job(&pool, Some(tenant_id), 1, "completed").await;
		let old_task = insert_task(&pool, old_job, "completed", Some(tenant_id), None, None).await;
		sqlx::query(
			"UPDATE v1_task_result SET completed_at = NOW() - INTERVAL '90 days' WHERE id = $1",
		)
		.bind(old_task)
		.execute(&pool)
		.await
		.expect("age the earlier verification");
		sqlx::query(
			"INSERT INTO reverification_schedules (tenant_id, staleness_days, batch_size) VALUES ($1, 30, 10)",
		)
		.bind(tenant_id)
		.execute(&pool)
		.await
		.expect("insert schedule");

		let config = build_test_config(
			ConfigProfile::WorkerRabbit,
			Some(env.postgres.db_url()),
			Some(env.rabbitmq.amqp_url()),
		)
		.await;
		// The broker channel is gone, so every publish fails.
		config
			.must_worker_config()
			.expect("worker config")
			.channel
			.close(0, "test: broker channel lost".into())
			.await
			.expect("close channel");

		run_reverification_cycle(&config.reverification, &config, &pool)
			.await
			.expect("reverification cycle");

		assert_eq!(used_this_period(&pool, tenant_id).await, 0);
		let job_status: String = sqlx::query_scalar(
			"SELECT status::TEXT FROM v1_bulk_job WHERE tenant_id = $1 AND metadata->>'source' = 'reverification'",
		)
		.bind(tenant_id)
		.fetch_one(&pool)
		.await
		.expect("load reverification job");
		assert_eq!(
			job_status, "failed",
			"a job nothing was published for must not stay running"
		);

		// The row keeps a full task payload, so it can be retried later.
		let payload: serde_json::Value = sqlx::query_scalar(
			"SELECT t.payload FROM v1_task_result t JOIN v1_bulk_job j ON j.id = t.job_id WHERE j.tenant_id = $1 AND j.metadata->>'source' = 'reverification'",
		)
		.bind(tenant_id)
		.fetch_one(&pool)
		.await
		.expect("load reverification task payload");
		let stored: CheckEmailTask =
			serde_json::from_value(payload).expect("payload is a full CheckEmailTask");
		assert_eq!(stored.input.to_email, "t@e.com");
		pool.close().await;
	}
}
