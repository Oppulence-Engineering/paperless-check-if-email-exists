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
async fn worker_backed_check_email_fails_fast_when_rabbitmq_is_down() {
	let env = OwnedResilienceEnv::start().await;
	let config = build_test_config(
		ConfigProfile::WorkerRabbit,
		Some(env.postgres.db_url()),
		Some(env.rabbitmq.amqp_url()),
	)
	.await;

	env.rabbitmq.stop().await;

	let response = tokio::time::timeout(
		Duration::from_secs(5),
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
	.expect("worker-backed request should fail fast");

	assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
	let body = parse_json(response.body());
	assert!(
		body["error"]
			.as_str()
			.is_some_and(|message| !message.is_empty()),
		"expected worker error body: {body}",
		body = body
	);
}

#[tokio::test]
#[serial]
async fn pre_ack_requeue_does_not_duplicate_completion_side_effects() {
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
		.queue_purge(CHECK_EMAIL_QUEUE, QueuePurgeOptions::default())
		.await;

	let config = build_test_config(
		ConfigProfile::WorkerRabbit,
		Some(env.postgres.db_url()),
		Some(env.rabbitmq.amqp_url()),
	)
	.await;

	let task = CheckEmailTask {
		input: check_if_email_exists::CheckEmailInput {
			to_email: "bad-syntax".into(),
			..Default::default()
		},
		job_id: CheckEmailJobId::Bulk(job_id),
		webhook: None,
		metadata: Some(TaskMetadata {
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
	let task_for_first_attempt: CheckEmailTask =
		serde_json::from_slice(&task_json).expect("deserialize cloned task");
	publisher
		.basic_publish(
			"",
			CHECK_EMAIL_QUEUE,
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

	let consumer = setup_rabbit_mq("mid-job-consumer", &rabbitmq_config)
		.await
		.expect("consumer channel");
	let mut deliveries = consumer
		.basic_consume(
			CHECK_EMAIL_QUEUE,
			"mid-job-consumer",
			BasicConsumeOptions::default(),
			FieldTable::default(),
		)
		.await
		.expect("basic consume");
	let delivery = tokio::time::timeout(Duration::from_secs(5), deliveries.next())
		.await
		.expect("receive first delivery")
		.expect("delivery stream item")
		.expect("lapin delivery");

	std::env::set_var("RCH_TEST_FORCE_REQUEUE_BEFORE_ACK", "1");
	let first_attempt = tokio::spawn({
		let channel = config.must_worker_config().expect("worker config").channel;
		let config = Arc::clone(&config);
		async move { do_check_email_work(&task_for_first_attempt, delivery, channel, config).await }
	});

	let first_result = tokio::time::timeout(Duration::from_secs(5), first_attempt)
		.await
		.expect("first attempt should fail within timeout")
		.expect("first attempt join");
	std::env::remove_var("RCH_TEST_FORCE_REQUEUE_BEFORE_ACK");
	assert!(
		first_result.is_err(),
		"first attempt should fail before ack and requeue the delivery"
	);

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
	let _ = publisher.close(0, "test cleanup").await;
	let _ = consumer.close(0, "test cleanup").await;
}
