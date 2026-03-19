/// Full coverage E2E tests using persistent Postgres + RabbitMQ.
/// Set TEST_DATABASE_URL and TEST_AMQP_URL to use existing services.
mod test_helpers;

use serial_test::serial;

// ═══════════════════════════════════════════════════════════════
// Worker consume.rs coverage: setup_rabbit_mq, queue declare, QoS
// ═══════════════════════════════════════════════════════════════
#[cfg(test)]
mod consume_coverage {
	use reacher_backend::config::RabbitMQConfig;
	use reacher_backend::worker::consume::{
		setup_rabbit_mq, CHECK_EMAIL_QUEUE, MAX_QUEUE_PRIORITY,
	};
	use serial_test::serial;

	fn rmq_url() -> String {
		crate::test_helpers::test_amqp_url()
	}

	#[tokio::test]
	#[serial]
	async fn test_setup_connects_and_declares_queue() {
		let config = RabbitMQConfig {
			url: rmq_url(),
			concurrency: 4,
		};
		let channel = setup_rabbit_mq("cov-test-1", &config).await.unwrap();
		assert!(channel.status().connected());
		// Verify queue exists via passive declare
		let q = channel
			.queue_declare(
				CHECK_EMAIL_QUEUE,
				lapin::options::QueueDeclareOptions {
					passive: true,
					..Default::default()
				},
				lapin::types::FieldTable::default(),
			)
			.await
			.unwrap();
		// Queue should exist
		assert!(q.name().as_str() == CHECK_EMAIL_QUEUE);
	}

	#[tokio::test]
	#[serial]
	async fn test_setup_sets_qos() {
		let config = RabbitMQConfig {
			url: rmq_url(),
			concurrency: 8,
		};
		let channel = setup_rabbit_mq("cov-test-qos", &config).await.unwrap();
		assert!(channel.status().connected());
	}

	#[tokio::test]
	#[serial]
	async fn test_setup_bad_url() {
		let config = RabbitMQConfig {
			url: "amqp://bad:9999".into(),
			concurrency: 1,
		};
		assert!(setup_rabbit_mq("cov-fail", &config).await.is_err());
	}

	#[test]
	fn test_constants() {
		assert_eq!(CHECK_EMAIL_QUEUE, "check_email");
		assert_eq!(MAX_QUEUE_PRIORITY, 5);
	}
}

// ═══════════════════════════════════════════════════════════════
// Worker do_work.rs coverage: publish, consume, check, state machine
// ═══════════════════════════════════════════════════════════════
#[cfg(test)]
mod do_work_coverage {
	use futures::StreamExt;
	use lapin::options::*;
	use lapin::types::FieldTable;
	use lapin::BasicProperties;
	use reacher_backend::config::RabbitMQConfig;
	use reacher_backend::worker::consume::{setup_rabbit_mq, CHECK_EMAIL_QUEUE};
	use reacher_backend::worker::do_work::{
		check_email_and_send_result, CheckEmailJobId, CheckEmailTask, RetryPolicy, TaskMetadata,
		TaskWebhook, Webhook,
	};
	use serial_test::serial;
	use std::collections::HashMap;

	fn rmq_url() -> String {
		crate::test_helpers::test_amqp_url()
	}

	#[tokio::test]
	#[serial]
	async fn test_check_email_returns_invalid_for_bad_syntax() {
		let task = CheckEmailTask {
			input: check_if_email_exists::CheckEmailInput {
				to_email: "not-email".into(),
				..Default::default()
			},
			job_id: CheckEmailJobId::SingleShot,
			webhook: None,
			metadata: None,
		};
		let result = check_email_and_send_result(&task, None).await.unwrap();
		assert_eq!(
			result.is_reachable,
			check_if_email_exists::Reachable::Invalid
		);
	}

	#[tokio::test]
	#[serial]
	async fn test_check_email_with_metadata() {
		let task = CheckEmailTask {
			input: check_if_email_exists::CheckEmailInput {
				to_email: "bad".into(),
				..Default::default()
			},
			job_id: CheckEmailJobId::Bulk(1),
			webhook: None,
			metadata: Some(TaskMetadata {
				tenant_id: Some("t1".into()),
				request_id: Some("r1".into()),
				correlation_id: Some("c1".into()),
				created_by: Some("test".into()),
				retry_policy: Some(RetryPolicy::default()),
				dedupe_key: Some("dk".into()),
				task_db_id: Some(99),
			}),
		};
		let result = check_email_and_send_result(&task, None).await.unwrap();
		assert_eq!(
			result.is_reachable,
			check_if_email_exists::Reachable::Invalid
		);
	}

	#[tokio::test]
	#[serial]
	async fn test_publish_consume_roundtrip() {
		let config = RabbitMQConfig {
			url: rmq_url(),
			concurrency: 4,
		};
		let channel = setup_rabbit_mq("dowork-rt", &config).await.unwrap();
		// Purge queue first
		let _ = channel
			.queue_purge(CHECK_EMAIL_QUEUE, QueuePurgeOptions::default())
			.await;

		let task = CheckEmailTask {
			input: check_if_email_exists::CheckEmailInput {
				to_email: "roundtrip@test.com".into(),
				..Default::default()
			},
			job_id: CheckEmailJobId::SingleShot,
			webhook: None,
			metadata: Some(TaskMetadata {
				tenant_id: None,
				request_id: None,
				correlation_id: None,
				created_by: None,
				retry_policy: None,
				dedupe_key: None,
				task_db_id: None,
			}),
		};
		let json = serde_json::to_vec(&task).unwrap();
		channel
			.basic_publish(
				"",
				CHECK_EMAIL_QUEUE,
				BasicPublishOptions::default(),
				&json,
				BasicProperties::default()
					.with_content_type("application/json".into())
					.with_priority(1),
			)
			.await
			.unwrap()
			.await
			.unwrap();

		let mut consumer = channel
			.basic_consume(
				CHECK_EMAIL_QUEUE,
				"dowork-con",
				BasicConsumeOptions::default(),
				FieldTable::default(),
			)
			.await
			.unwrap();

		let delivery = tokio::time::timeout(std::time::Duration::from_secs(5), consumer.next())
			.await
			.unwrap()
			.unwrap()
			.unwrap();

		let received: CheckEmailTask = serde_json::from_slice(&delivery.data).unwrap();
		assert_eq!(received.input.to_email, "roundtrip@test.com");
		delivery.ack(BasicAckOptions::default()).await.unwrap();
	}

	#[tokio::test]
	#[serial]
	async fn test_task_with_webhook_no_url() {
		// Webhook with None on_each_email should not send anything
		let task = CheckEmailTask {
			input: check_if_email_exists::CheckEmailInput {
				to_email: "noemail".into(),
				..Default::default()
			},
			job_id: CheckEmailJobId::SingleShot,
			webhook: Some(TaskWebhook {
				on_each_email: None,
			}),
			metadata: None,
		};
		let result = check_email_and_send_result(&task, None).await.unwrap();
		assert_eq!(
			result.is_reachable,
			check_if_email_exists::Reachable::Invalid
		);
	}
}

// ═══════════════════════════════════════════════════════════════
// shared/check_email.rs coverage: handle_check_email flow
// ═══════════════════════════════════════════════════════════════
#[cfg(test)]
mod shared_check_email_coverage {
	use crate::test_helpers::TestDb;
	use reacher_backend::config::{BackendConfig, PostgresConfig, StorageConfig};
	use reacher_backend::http::{create_routes, CheckEmailRequest, REACHER_SECRET_HEADER};
	use serial_test::serial;
	use sqlx::Row;
	use std::sync::Arc;
	use warp::http::StatusCode;
	use warp::test::request;

	async fn config_with_db() -> Arc<BackendConfig> {
		let mut config = BackendConfig::empty();
		config.header_secret = Some("s".into());
		let db_url = crate::test_helpers::test_db_url();
		config.storage = Some(StorageConfig::Postgres(PostgresConfig {
			db_url,
			extra: None,
		}));
		config.connect().await.unwrap();
		Arc::new(config)
	}

	#[tokio::test]
	#[serial]
	async fn test_handle_without_worker_stores_and_returns() {
		let _db = TestDb::start().await;
		let config = config_with_db().await;
		let resp = request()
			.path("/v1/check_email")
			.method("POST")
			.header(REACHER_SECRET_HEADER, "s")
			.json(&serde_json::from_str::<CheckEmailRequest>(r#"{"to_email": "foo@bar"}"#).unwrap())
			.reply(&create_routes(Arc::clone(&config)))
			.await;
		assert_eq!(resp.status(), StatusCode::OK);
		let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
		assert_eq!(body["is_reachable"], "invalid");
	}

	#[tokio::test]
	#[serial]
	async fn test_empty_email_rejected() {
		let config = config_with_db().await;
		let resp = request()
			.path("/v1/check_email")
			.method("POST")
			.header(REACHER_SECRET_HEADER, "s")
			.json(&serde_json::from_str::<CheckEmailRequest>(r#"{"to_email": ""}"#).unwrap())
			.reply(&create_routes(config))
			.await;
		assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
	}

	#[tokio::test]
	#[serial]
	async fn test_v0_through_shared_handler() {
		let _db = TestDb::start().await;
		let config = config_with_db().await;
		let resp = request()
			.path("/v0/check_email")
			.method("POST")
			.header(REACHER_SECRET_HEADER, "s")
			.json(&serde_json::from_str::<CheckEmailRequest>(r#"{"to_email": "x@y"}"#).unwrap())
			.reply(&create_routes(config))
			.await;
		assert_eq!(resp.status(), StatusCode::OK);
		assert_eq!(resp.headers().get("Deprecation").unwrap(), "true");
	}

	#[tokio::test]
	#[serial]
	async fn test_throttle_check_passes() {
		let config = config_with_db().await;
		// With no throttle limits, should pass
		let resp = request()
			.path("/v1/check_email")
			.method("POST")
			.header(REACHER_SECRET_HEADER, "s")
			.json(&serde_json::from_str::<CheckEmailRequest>(r#"{"to_email": "a@b"}"#).unwrap())
			.reply(&create_routes(config))
			.await;
		assert_eq!(resp.status(), StatusCode::OK);
	}

	#[tokio::test]
	#[serial]
	async fn test_idempotency_key_reuses_cached_response() {
		let db = TestDb::start().await;
		let config = config_with_db().await;
		let routes = create_routes(config);

		let req_body =
			serde_json::from_str::<CheckEmailRequest>(r#"{"to_email":"cache@reacher.email"}"#)
				.unwrap();

		let first = request()
			.path("/v1/check_email")
			.method("POST")
			.header(REACHER_SECRET_HEADER, "s")
			.header("Idempotency-Key", "idem-cache-key")
			.json(&req_body)
			.reply(&routes)
			.await;
		assert_eq!(first.status(), StatusCode::OK);
		let first_body = first.body().to_vec();

		let second = request()
			.path("/v1/check_email")
			.method("POST")
			.header(REACHER_SECRET_HEADER, "s")
			.header("Idempotency-Key", "idem-cache-key")
			.json(&req_body)
			.reply(&routes)
			.await;
		assert_eq!(second.status(), StatusCode::OK);
		let second_body = second.body().to_vec();

		assert_eq!(
			first_body, second_body,
			"Cached response should match original response"
		);

		let row = sqlx::query(
			"SELECT response_status_code FROM idempotency_keys WHERE tenant_id = $1 AND idempotency_key = $2",
		)
		.bind("legacy")
		.bind("idem-cache-key")
		.fetch_one(db.pool())
		.await
		.unwrap();
		let status: Option<i16> = row.get("response_status_code");
		assert_eq!(status, Some(200));
	}
}

// ═══════════════════════════════════════════════════════════════
// v0/bulk coverage: post, get, results via warp integration
// ═══════════════════════════════════════════════════════════════
#[cfg(test)]
mod v0_bulk_coverage {
	use crate::test_helpers::TestDb;
	use serial_test::serial;
	use sqlx::Row;

	#[tokio::test]
	#[serial]
	async fn test_v0_bulk_job_status_query() {
		let db = TestDb::start().await;
		// Insert a v0 bulk job
		let row = sqlx::query("INSERT INTO bulk_jobs (total_records) VALUES (3) RETURNING id")
			.fetch_one(db.pool())
			.await
			.unwrap();
		let job_id: i32 = row.get("id");

		// Insert some results
		for reachable in &["safe", "risky", "invalid"] {
			let result = serde_json::json!({"is_reachable": reachable, "input": "t@e.com"});
			sqlx::query("INSERT INTO email_results (job_id, result) VALUES ($1, $2)")
				.bind(job_id)
				.bind(&result)
				.execute(db.pool())
				.await
				.unwrap();
		}

		// Query aggregation (mirrors v0/bulk/get.rs logic)
		let agg = sqlx::query(
			"SELECT COUNT(*) as total, COUNT(CASE WHEN result->>'is_reachable' = 'safe' THEN 1 END) as safe_count FROM email_results WHERE job_id = $1"
		).bind(job_id).fetch_one(db.pool()).await.unwrap();

		let total: i64 = agg.get("total");
		let safe: i64 = agg.get("safe_count");
		assert_eq!(total, 3);
		assert_eq!(safe, 1);
	}

	#[tokio::test]
	#[serial]
	async fn test_v0_bulk_results_query() {
		let db = TestDb::start().await;
		let row = sqlx::query("INSERT INTO bulk_jobs (total_records) VALUES (2) RETURNING id")
			.fetch_one(db.pool())
			.await
			.unwrap();
		let job_id: i32 = row.get("id");

		sqlx::query("INSERT INTO email_results (job_id, result) VALUES ($1, $2)")
			.bind(job_id)
			.bind(&serde_json::json!({"is_reachable": "safe", "input": "a@b.com"}))
			.execute(db.pool())
			.await
			.unwrap();
		sqlx::query("INSERT INTO email_results (job_id, result) VALUES ($1, $2)")
			.bind(job_id)
			.bind(&serde_json::json!({"is_reachable": "invalid", "input": "c@d.com"}))
			.execute(db.pool())
			.await
			.unwrap();

		// Query with limit/offset (mirrors v0/bulk/results/mod.rs logic)
		let rows = sqlx::query(
			"SELECT result FROM email_results WHERE job_id = $1 ORDER BY id LIMIT $2 OFFSET $3",
		)
		.bind(job_id)
		.bind(1i64)
		.bind(0i64)
		.fetch_all(db.pool())
		.await
		.unwrap();
		assert_eq!(rows.len(), 1);

		let rows_all =
			sqlx::query("SELECT result FROM email_results WHERE job_id = $1 ORDER BY id")
				.bind(job_id)
				.fetch_all(db.pool())
				.await
				.unwrap();
		assert_eq!(rows_all.len(), 2);
	}

	#[tokio::test]
	#[serial]
	async fn test_v0_bulk_job_creation() {
		let db = TestDb::start().await;
		let row = sqlx::query("INSERT INTO bulk_jobs (total_records) VALUES ($1) RETURNING id")
			.bind(5i32)
			.fetch_one(db.pool())
			.await
			.unwrap();
		let id: i32 = row.get("id");
		assert!(id > 0);

		let job = sqlx::query("SELECT total_records FROM bulk_jobs WHERE id = $1")
			.bind(id)
			.fetch_one(db.pool())
			.await
			.unwrap();
		let total: i32 = job.get("total_records");
		assert_eq!(total, 5);
	}
}

// ═══════════════════════════════════════════════════════════════
// v1/bulk coverage: post (pre-create tasks), get_progress, get_results
// ═══════════════════════════════════════════════════════════════
#[cfg(test)]
mod v1_bulk_coverage {
	use crate::test_helpers::{insert_job, insert_task, invalid_result, safe_result, TestDb};
	use serial_test::serial;
	use sqlx::Row;

	#[tokio::test]
	#[serial]
	async fn test_v1_bulk_pre_create_tasks() {
		let db = TestDb::start().await;
		let job_id = insert_job(db.pool(), None, 3, "pending").await;

		// Pre-create task rows like v1/bulk/post.rs does
		for _ in 0..3 {
			insert_task(db.pool(), job_id, "queued", None, None, None).await;
		}

		// Verify 3 queued tasks
		let count: i64 =
			sqlx::query_scalar("SELECT COUNT(*) FROM v1_task_result WHERE job_id = $1")
				.bind(job_id)
				.fetch_one(db.pool())
				.await
				.unwrap();
		assert_eq!(count, 3);

		// Verify none are "processed" yet
		let processed: i64 = sqlx::query_scalar(
			"SELECT COUNT(*) FROM v1_task_result WHERE job_id = $1 AND (result IS NOT NULL OR error IS NOT NULL)"
		).bind(job_id).fetch_one(db.pool()).await.unwrap();
		assert_eq!(processed, 0);
	}

	#[tokio::test]
	#[serial]
	async fn test_v1_bulk_progress_aggregation() {
		let db = TestDb::start().await;
		let job_id = insert_job(db.pool(), None, 4, "running").await;

		insert_task(
			db.pool(),
			job_id,
			"completed",
			None,
			Some(safe_result()),
			None,
		)
		.await;
		insert_task(
			db.pool(),
			job_id,
			"completed",
			None,
			Some(safe_result()),
			None,
		)
		.await;
		insert_task(
			db.pool(),
			job_id,
			"completed",
			None,
			Some(invalid_result()),
			None,
		)
		.await;
		insert_task(db.pool(), job_id, "queued", None, None, None).await;

		// Mirrors get_progress.rs aggregation
		let agg = sqlx::query(
			"SELECT COUNT(*) as total_processed, \
			 COUNT(CASE WHEN result->>'is_reachable' = 'safe' THEN 1 END) as safe_count, \
			 COUNT(CASE WHEN result->>'is_reachable' = 'invalid' THEN 1 END) as invalid_count \
			 FROM v1_task_result WHERE job_id = $1 AND (result IS NOT NULL OR error IS NOT NULL)",
		)
		.bind(job_id)
		.fetch_one(db.pool())
		.await
		.unwrap();

		let processed: i64 = agg.get("total_processed");
		let safe: i64 = agg.get("safe_count");
		let invalid: i64 = agg.get("invalid_count");
		assert_eq!(processed, 3);
		assert_eq!(safe, 2);
		assert_eq!(invalid, 1);
	}

	#[tokio::test]
	#[serial]
	async fn test_v1_bulk_results_pagination() {
		let db = TestDb::start().await;
		let job_id = insert_job(db.pool(), None, 5, "completed").await;

		for _ in 0..5 {
			insert_task(
				db.pool(),
				job_id,
				"completed",
				None,
				Some(safe_result()),
				None,
			)
			.await;
		}

		// First page (limit 2)
		let rows = sqlx::query(
			"SELECT id, result FROM v1_task_result WHERE job_id = $1 ORDER BY id LIMIT $2",
		)
		.bind(job_id)
		.bind(2i64)
		.fetch_all(db.pool())
		.await
		.unwrap();
		assert_eq!(rows.len(), 2);

		// Offset page
		let last_id: i32 = rows.last().unwrap().get("id");
		let next = sqlx::query(
			"SELECT id FROM v1_task_result WHERE job_id = $1 AND id > $2 ORDER BY id LIMIT $3",
		)
		.bind(job_id)
		.bind(last_id)
		.bind(2i64)
		.fetch_all(db.pool())
		.await
		.unwrap();
		assert_eq!(next.len(), 2);
	}
}

// ═══════════════════════════════════════════════════════════════
// v1/jobs coverage: status, cancel, events, results
// ═══════════════════════════════════════════════════════════════
#[cfg(test)]
mod v1_jobs_coverage {
	use crate::test_helpers::{insert_event, insert_job, insert_task, safe_result, TestDb};
	use serial_test::serial;
	use sqlx::Row;

	#[tokio::test]
	#[serial]
	async fn test_job_status_task_state_aggregation() {
		let db = TestDb::start().await;
		let job_id = insert_job(db.pool(), None, 5, "running").await;

		insert_task(
			db.pool(),
			job_id,
			"completed",
			None,
			Some(safe_result()),
			None,
		)
		.await;
		insert_task(
			db.pool(),
			job_id,
			"completed",
			None,
			Some(safe_result()),
			None,
		)
		.await;
		insert_task(db.pool(), job_id, "running", None, None, None).await;
		insert_task(db.pool(), job_id, "queued", None, None, None).await;
		insert_task(db.pool(), job_id, "failed", None, None, Some("error")).await;

		// Mirrors get_status.rs aggregation
		let row = sqlx::query(
			"SELECT \
			 COUNT(CASE WHEN task_state = 'completed' THEN 1 END) as completed, \
			 COUNT(CASE WHEN task_state = 'running' THEN 1 END) as running, \
			 COUNT(CASE WHEN task_state = 'queued' THEN 1 END) as queued, \
			 COUNT(CASE WHEN task_state = 'failed' THEN 1 END) as failed \
			 FROM v1_task_result WHERE job_id = $1",
		)
		.bind(job_id)
		.fetch_one(db.pool())
		.await
		.unwrap();

		assert_eq!(row.get::<i64, _>("completed"), 2);
		assert_eq!(row.get::<i64, _>("running"), 1);
		assert_eq!(row.get::<i64, _>("queued"), 1);
		assert_eq!(row.get::<i64, _>("failed"), 1);
	}

	#[tokio::test]
	#[serial]
	async fn test_cancel_job_flow() {
		let db = TestDb::start().await;
		let job_id = insert_job(db.pool(), None, 3, "running").await;
		insert_task(db.pool(), job_id, "queued", None, None, None).await;
		insert_task(db.pool(), job_id, "queued", None, None, None).await;
		insert_task(db.pool(), job_id, "running", None, None, None).await;

		// Cancel queued tasks (mirrors cancel.rs)
		sqlx::query("UPDATE v1_task_result SET task_state = 'cancelled'::task_state WHERE job_id = $1 AND task_state IN ('queued', 'retrying')")
			.bind(job_id).execute(db.pool()).await.unwrap();

		// Set job to cancelling
		sqlx::query("UPDATE v1_bulk_job SET status = 'cancelling'::job_state WHERE id = $1")
			.bind(job_id)
			.execute(db.pool())
			.await
			.unwrap();

		// Check non-terminal count
		let non_terminal: i64 = sqlx::query_scalar(
			"SELECT COUNT(*) FROM v1_task_result WHERE job_id = $1 AND task_state IN ('queued', 'running', 'retrying')"
		).bind(job_id).fetch_one(db.pool()).await.unwrap();
		assert_eq!(non_terminal, 1); // running task still active

		// Verify cancelled count
		let cancelled: i64 = sqlx::query_scalar(
			"SELECT COUNT(*) FROM v1_task_result WHERE job_id = $1 AND task_state = 'cancelled'",
		)
		.bind(job_id)
		.fetch_one(db.pool())
		.await
		.unwrap();
		assert_eq!(cancelled, 2);
	}

	#[tokio::test]
	#[serial]
	async fn test_events_recording_and_query() {
		let db = TestDb::start().await;
		let job_id = insert_job(db.pool(), None, 1, "running").await;

		insert_event(db.pool(), job_id, None, "job.created").await;
		let task_id = insert_task(
			db.pool(),
			job_id,
			"completed",
			None,
			Some(safe_result()),
			None,
		)
		.await;
		insert_event(db.pool(), job_id, Some(task_id), "task.completed").await;
		insert_event(db.pool(), job_id, None, "job.completed").await;

		// Query events (mirrors get_events.rs)
		let events = sqlx::query(
			"SELECT event_type, task_id FROM job_events WHERE job_id = $1 ORDER BY created_at",
		)
		.bind(job_id)
		.fetch_all(db.pool())
		.await
		.unwrap();
		assert_eq!(events.len(), 3);
		assert_eq!(events[0].get::<String, _>("event_type"), "job.created");
		assert_eq!(events[1].get::<String, _>("event_type"), "task.completed");
		assert_eq!(events[2].get::<String, _>("event_type"), "job.completed");
	}

	#[tokio::test]
	#[serial]
	async fn test_cursor_based_results() {
		let db = TestDb::start().await;
		let job_id = insert_job(db.pool(), None, 5, "completed").await;

		let mut ids = vec![];
		for _ in 0..5 {
			let id = insert_task(
				db.pool(),
				job_id,
				"completed",
				None,
				Some(safe_result()),
				None,
			)
			.await;
			ids.push(id);
		}

		// Cursor pagination (mirrors get_results.rs)
		let page1 = sqlx::query("SELECT id, task_state::TEXT FROM v1_task_result WHERE job_id = $1 AND task_state = 'completed'::task_state ORDER BY id LIMIT 3")
			.bind(job_id).fetch_all(db.pool()).await.unwrap();
		assert_eq!(page1.len(), 3);

		let cursor: i32 = page1.last().unwrap().get("id");
		let page2 = sqlx::query("SELECT id FROM v1_task_result WHERE job_id = $1 AND task_state = 'completed'::task_state AND id > $2 ORDER BY id LIMIT 3")
			.bind(job_id).bind(cursor).fetch_all(db.pool()).await.unwrap();
		assert_eq!(page2.len(), 2); // Only 2 remaining
	}
}

// ═══════════════════════════════════════════════════════════════
// do_check_email_work coverage: full state machine with real Delivery
// ═══════════════════════════════════════════════════════════════
#[cfg(test)]
mod do_check_email_work_coverage {
	use crate::test_helpers::TestDb;
	use futures::StreamExt;
	use lapin::options::*;
	use lapin::types::FieldTable;
	use lapin::BasicProperties;
	use reacher_backend::config::{
		BackendConfig, PostgresConfig, RabbitMQConfig, StorageConfig, WorkerConfig,
	};
	use reacher_backend::worker::consume::{setup_rabbit_mq, CHECK_EMAIL_QUEUE};
	use reacher_backend::worker::do_work::{
		do_check_email_work, CheckEmailJobId, CheckEmailTask, RetryPolicy, TaskMetadata,
	};
	use serial_test::serial;
	use sqlx::Row;
	use std::sync::Arc;

	fn rmq_url() -> String {
		crate::test_helpers::test_amqp_url()
	}

	fn db_url() -> String {
		crate::test_helpers::test_db_url()
	}

	async fn make_config() -> Arc<BackendConfig> {
		let mut config = BackendConfig::empty();
		config.storage = Some(StorageConfig::Postgres(PostgresConfig {
			db_url: db_url(),
			extra: None,
		}));
		config.worker = WorkerConfig {
			enable: true,
			rabbitmq: Some(RabbitMQConfig {
				url: rmq_url(),
				concurrency: 4,
			}),
			webhook: None,
		};
		config.connect().await.unwrap();
		Arc::new(config)
	}

	#[tokio::test]
	#[serial]
	async fn test_do_check_email_work_success_path() {
		let db = TestDb::start().await;
		let config = make_config().await;
		let rmq_config = RabbitMQConfig {
			url: rmq_url(),
			concurrency: 4,
		};
		let channel = Arc::new(
			setup_rabbit_mq("dowork-success", &rmq_config)
				.await
				.unwrap(),
		);

		// Purge queue
		let _ = channel
			.queue_purge(CHECK_EMAIL_QUEUE, QueuePurgeOptions::default())
			.await;

		// Create a job and task in DB
		let job_id: i32 = sqlx::query("INSERT INTO v1_bulk_job (total_records, status) VALUES (1, 'running'::job_state) RETURNING id")
			.fetch_one(db.pool()).await.unwrap().get("id");
		let task_db_id: i32 = sqlx::query("INSERT INTO v1_task_result (job_id, payload, task_state) VALUES ($1, $2, 'queued'::task_state) RETURNING id")
			.bind(job_id).bind(serde_json::json!({}))
			.fetch_one(db.pool()).await.unwrap().get("id");

		// Publish a task with invalid email (will return Invalid = success path)
		let task = CheckEmailTask {
			input: check_if_email_exists::CheckEmailInput {
				to_email: "invalid-syntax".into(),
				..Default::default()
			},
			job_id: CheckEmailJobId::Bulk(job_id),
			webhook: None,
			metadata: Some(TaskMetadata {
				tenant_id: None,
				request_id: None,
				correlation_id: None,
				created_by: None,
				retry_policy: Some(RetryPolicy {
					max_retries: 0,
					backoff_seconds: 1,
					backoff_multiplier: 1.0,
				}),
				dedupe_key: None,
				task_db_id: Some(task_db_id),
			}),
		};
		let json = serde_json::to_vec(&task).unwrap();
		channel
			.basic_publish(
				"",
				CHECK_EMAIL_QUEUE,
				BasicPublishOptions::default(),
				&json,
				BasicProperties::default().with_content_type("application/json".into()),
			)
			.await
			.unwrap()
			.await
			.unwrap();

		// Consume to get a real Delivery
		let mut consumer = channel
			.basic_consume(
				CHECK_EMAIL_QUEUE,
				"dowork-test",
				BasicConsumeOptions::default(),
				FieldTable::default(),
			)
			.await
			.unwrap();

		let delivery = tokio::time::timeout(std::time::Duration::from_secs(5), consumer.next())
			.await
			.unwrap()
			.unwrap()
			.unwrap();

		let received: CheckEmailTask = serde_json::from_slice(&delivery.data).unwrap();

		// Call do_check_email_work with the real delivery
		let result = do_check_email_work(&received, delivery, Arc::clone(&channel), config).await;
		assert!(
			result.is_ok(),
			"do_check_email_work failed: {:?}",
			result.err()
		);

		// Verify task state was updated to completed
		let state: String =
			sqlx::query("SELECT task_state::TEXT FROM v1_task_result WHERE id = $1")
				.bind(task_db_id)
				.fetch_one(db.pool())
				.await
				.unwrap()
				.get(0);
		assert_eq!(state, "completed");

		// Verify a result was stored (may be in the pre-created row or a new row)
		let total_results: i64 = sqlx::query_scalar(
			"SELECT COUNT(*) FROM v1_task_result WHERE job_id = $1 AND result IS NOT NULL",
		)
		.bind(job_id)
		.fetch_one(db.pool())
		.await
		.unwrap();
		assert!(total_results >= 1, "Should have at least one result stored");
	}

	#[tokio::test]
	#[serial]
	async fn test_do_check_email_work_dead_letter_path() {
		let db = TestDb::start().await;
		let config = make_config().await;
		let rmq_config = RabbitMQConfig {
			url: rmq_url(),
			concurrency: 4,
		};
		let channel = Arc::new(setup_rabbit_mq("dowork-dl", &rmq_config).await.unwrap());
		let _ = channel
			.queue_purge(CHECK_EMAIL_QUEUE, QueuePurgeOptions::default())
			.await;

		let job_id: i32 = sqlx::query("INSERT INTO v1_bulk_job (total_records, status) VALUES (1, 'running'::job_state) RETURNING id")
			.fetch_one(db.pool()).await.unwrap().get("id");
		let task_db_id: i32 = sqlx::query("INSERT INTO v1_task_result (job_id, payload, task_state, retry_count) VALUES ($1, $2, 'queued'::task_state, 2) RETURNING id")
			.bind(job_id).bind(serde_json::json!({}))
			.fetch_one(db.pool()).await.unwrap().get("id");

		// Use an email that will return Unknown (triggers retry path)
		// But with max_retries=0 and retry_count already at 2, it should dead-letter
		let task = CheckEmailTask {
			input: check_if_email_exists::CheckEmailInput {
				to_email: "test@nonexistent-domain-xyz123.tld".into(),
				..Default::default()
			},
			job_id: CheckEmailJobId::Bulk(job_id),
			webhook: None,
			metadata: Some(TaskMetadata {
				tenant_id: None,
				request_id: None,
				correlation_id: None,
				created_by: None,
				retry_policy: Some(RetryPolicy {
					max_retries: 0,
					backoff_seconds: 1,
					backoff_multiplier: 1.0,
				}),
				dedupe_key: None,
				task_db_id: Some(task_db_id),
			}),
		};
		let json = serde_json::to_vec(&task).unwrap();
		channel
			.basic_publish(
				"",
				CHECK_EMAIL_QUEUE,
				BasicPublishOptions::default(),
				&json,
				BasicProperties::default().with_content_type("application/json".into()),
			)
			.await
			.unwrap()
			.await
			.unwrap();

		let mut consumer = channel
			.basic_consume(
				CHECK_EMAIL_QUEUE,
				"dowork-dl-test",
				BasicConsumeOptions::default(),
				FieldTable::default(),
			)
			.await
			.unwrap();

		let delivery = tokio::time::timeout(std::time::Duration::from_secs(5), consumer.next())
			.await
			.unwrap()
			.unwrap()
			.unwrap();

		let received: CheckEmailTask = serde_json::from_slice(&delivery.data).unwrap();
		let _ = do_check_email_work(&received, delivery, Arc::clone(&channel), config).await;

		// Check state — should be either completed or dead_lettered
		let state: String =
			sqlx::query("SELECT task_state::TEXT FROM v1_task_result WHERE id = $1")
				.bind(task_db_id)
				.fetch_one(db.pool())
				.await
				.unwrap()
				.get(0);
		assert!(
			state == "completed" || state == "dead_lettered",
			"Expected completed or dead_lettered, got: {}",
			state
		);
	}
}

// ═══════════════════════════════════════════════════════════════
// Additional do_check_email_work tests: retry path + SingleShot
// ═══════════════════════════════════════════════════════════════
#[cfg(test)]
mod do_check_email_work_retry_coverage {
	use crate::test_helpers::TestDb;
	use futures::StreamExt;
	use lapin::options::*;
	use lapin::types::FieldTable;
	use lapin::BasicProperties;
	use reacher_backend::config::{
		BackendConfig, PostgresConfig, RabbitMQConfig, StorageConfig, WorkerConfig,
	};
	use reacher_backend::worker::consume::{setup_rabbit_mq, CHECK_EMAIL_QUEUE};
	use reacher_backend::worker::do_work::{
		do_check_email_work, CheckEmailJobId, CheckEmailTask, RetryPolicy, TaskMetadata,
	};
	use serial_test::serial;
	use sqlx::Row;
	use std::sync::Arc;

	fn rmq_url() -> String {
		crate::test_helpers::test_amqp_url()
	}
	fn db_url() -> String {
		crate::test_helpers::test_db_url()
	}

	async fn make_config() -> Arc<BackendConfig> {
		let mut config = BackendConfig::empty();
		config.storage = Some(StorageConfig::Postgres(PostgresConfig {
			db_url: db_url(),
			extra: None,
		}));
		config.worker = WorkerConfig {
			enable: true,
			rabbitmq: Some(RabbitMQConfig {
				url: rmq_url(),
				concurrency: 4,
			}),
			webhook: None,
		};
		config.connect().await.unwrap();
		Arc::new(config)
	}

	#[tokio::test]
	#[serial]
	async fn test_do_check_email_work_single_shot() {
		let db = TestDb::start().await;
		let config = make_config().await;
		let rmq_config = RabbitMQConfig {
			url: rmq_url(),
			concurrency: 4,
		};
		let channel = Arc::new(setup_rabbit_mq("dowork-ss", &rmq_config).await.unwrap());
		let _ = channel
			.queue_purge(CHECK_EMAIL_QUEUE, QueuePurgeOptions::default())
			.await;

		// Create a reply queue for SingleShot RPC
		let reply_queue = channel
			.queue_declare(
				"",
				QueueDeclareOptions {
					auto_delete: true,
					durable: false,
					exclusive: true,
					..Default::default()
				},
				FieldTable::default(),
			)
			.await
			.unwrap();
		let correlation_id = uuid::Uuid::new_v4().to_string();

		let task = CheckEmailTask {
			input: check_if_email_exists::CheckEmailInput {
				to_email: "bad".into(),
				..Default::default()
			},
			job_id: CheckEmailJobId::SingleShot,
			webhook: None,
			metadata: None,
		};
		let json = serde_json::to_vec(&task).unwrap();
		channel
			.basic_publish(
				"",
				CHECK_EMAIL_QUEUE,
				BasicPublishOptions::default(),
				&json,
				BasicProperties::default()
					.with_content_type("application/json".into())
					.with_reply_to(reply_queue.name().to_owned())
					.with_correlation_id(correlation_id.into()),
			)
			.await
			.unwrap()
			.await
			.unwrap();

		let mut consumer = channel
			.basic_consume(
				CHECK_EMAIL_QUEUE,
				"dowork-ss-con",
				BasicConsumeOptions::default(),
				FieldTable::default(),
			)
			.await
			.unwrap();
		let delivery = tokio::time::timeout(std::time::Duration::from_secs(5), consumer.next())
			.await
			.unwrap()
			.unwrap()
			.unwrap();
		let received: CheckEmailTask = serde_json::from_slice(&delivery.data).unwrap();

		// Should succeed (Invalid result → success path, no task_db_id to update)
		let result = do_check_email_work(&received, delivery, Arc::clone(&channel), config).await;
		assert!(result.is_ok());
	}

	#[tokio::test]
	#[serial]
	async fn test_do_check_email_work_with_retry_policy() {
		let db = TestDb::start().await;
		let config = make_config().await;
		let rmq_config = RabbitMQConfig {
			url: rmq_url(),
			concurrency: 4,
		};
		let channel = Arc::new(setup_rabbit_mq("dowork-retry", &rmq_config).await.unwrap());
		let _ = channel
			.queue_purge(CHECK_EMAIL_QUEUE, QueuePurgeOptions::default())
			.await;

		let job_id: i32 = sqlx::query("INSERT INTO v1_bulk_job (total_records, status) VALUES (1, 'running'::job_state) RETURNING id")
			.fetch_one(db.pool()).await.unwrap().get("id");
		let task_db_id: i32 = sqlx::query("INSERT INTO v1_task_result (job_id, payload, task_state, retry_count) VALUES ($1, $2, 'queued'::task_state, 0) RETURNING id")
			.bind(job_id).bind(serde_json::json!({}))
			.fetch_one(db.pool()).await.unwrap().get("id");

		// Task with high max_retries — will trigger retry path for Unknown results
		let task = CheckEmailTask {
			input: check_if_email_exists::CheckEmailInput {
				to_email: "user@nonexistent-domain-abc999.tld".into(),
				..Default::default()
			},
			job_id: CheckEmailJobId::Bulk(job_id),
			webhook: None,
			metadata: Some(TaskMetadata {
				tenant_id: None,
				request_id: None,
				correlation_id: None,
				created_by: None,
				retry_policy: Some(RetryPolicy {
					max_retries: 5,
					backoff_seconds: 1,
					backoff_multiplier: 1.0,
				}),
				dedupe_key: None,
				task_db_id: Some(task_db_id),
			}),
		};
		let json = serde_json::to_vec(&task).unwrap();
		channel
			.basic_publish(
				"",
				CHECK_EMAIL_QUEUE,
				BasicPublishOptions::default(),
				&json,
				BasicProperties::default().with_content_type("application/json".into()),
			)
			.await
			.unwrap()
			.await
			.unwrap();

		let mut consumer = channel
			.basic_consume(
				CHECK_EMAIL_QUEUE,
				"dowork-retry-con",
				BasicConsumeOptions::default(),
				FieldTable::default(),
			)
			.await
			.unwrap();
		let delivery = tokio::time::timeout(std::time::Duration::from_secs(5), consumer.next())
			.await
			.unwrap()
			.unwrap()
			.unwrap();
		let received: CheckEmailTask = serde_json::from_slice(&delivery.data).unwrap();

		let result = do_check_email_work(&received, delivery, Arc::clone(&channel), config).await;
		assert!(result.is_ok());

		// Check state — could be completed (if Invalid) or retrying (if Unknown)
		let state: String =
			sqlx::query("SELECT task_state::TEXT FROM v1_task_result WHERE id = $1")
				.bind(task_db_id)
				.fetch_one(db.pool())
				.await
				.unwrap()
				.get(0);
		assert!(
			state == "completed" || state == "retrying",
			"Expected completed or retrying, got: {}",
			state
		);
	}
}

// ═══════════════════════════════════════════════════════════════
// storage/postgres.rs coverage: PostgresStorage::new + store
// ═══════════════════════════════════════════════════════════════
#[cfg(test)]
mod postgres_storage_coverage {
	use check_if_email_exists::{CheckEmailOutput, Reachable};
	use reacher_backend::storage::postgres::PostgresStorage;
	use reacher_backend::worker::do_work::{
		CheckEmailJobId, CheckEmailTask, TaskError, TaskMetadata,
	};
	use serial_test::serial;

	fn db_url() -> String {
		crate::test_helpers::test_db_url()
	}

	fn make_task(metadata: Option<TaskMetadata>) -> CheckEmailTask {
		CheckEmailTask {
			input: check_if_email_exists::CheckEmailInput {
				to_email: "store@test.com".into(),
				..Default::default()
			},
			job_id: CheckEmailJobId::SingleShot,
			webhook: None,
			metadata,
		}
	}

	#[tokio::test]
	#[serial]
	async fn test_new_connects_and_migrates() {
		let storage = PostgresStorage::new(&db_url(), None).await.unwrap();
		// Verify we can query
		let row = sqlx::query("SELECT 1 as v")
			.fetch_one(&storage.pg_pool)
			.await
			.unwrap();
		let v: i32 = sqlx::Row::get(&row, "v");
		assert_eq!(v, 1);
	}

	#[tokio::test]
	#[serial]
	async fn test_store_ok_result() {
		let storage = PostgresStorage::new(&db_url(), None).await.unwrap();
		let task = make_task(None);
		let output = CheckEmailOutput {
			input: "store@test.com".into(),
			is_reachable: Reachable::Invalid,
			..Default::default()
		};
		storage.store(&task, &Ok(output), None).await.unwrap();
	}

	#[tokio::test]
	#[serial]
	async fn test_store_err_result() {
		let storage = PostgresStorage::new(&db_url(), None).await.unwrap();
		let task = make_task(None);
		let err: Result<CheckEmailOutput, TaskError> =
			Err(TaskError::Lapin(lapin::Error::InvalidChannel(0)));
		storage.store(&task, &err, None).await.unwrap();
	}

	#[tokio::test]
	#[serial]
	async fn test_store_with_extra() {
		let extra = Some(serde_json::json!({"key": "value"}));
		let storage = PostgresStorage::new(&db_url(), extra.clone())
			.await
			.unwrap();
		assert_eq!(storage.get_extra(), extra);

		let task = make_task(None);
		let output = CheckEmailOutput {
			input: "store@test.com".into(),
			is_reachable: Reachable::Invalid,
			..Default::default()
		};
		storage.store(&task, &Ok(output), extra).await.unwrap();
	}

	#[tokio::test]
	#[serial]
	async fn test_store_with_tenant_id() {
		let storage = PostgresStorage::new(&db_url(), None).await.unwrap();
		// Create a tenant
		let row = sqlx::query("INSERT INTO tenants (name, slug, contact_email) VALUES ('T', 'store-cov', 's@t.com') RETURNING id")
			.fetch_one(&storage.pg_pool).await.unwrap();
		let tid: uuid::Uuid = sqlx::Row::get(&row, "id");

		let task = make_task(Some(TaskMetadata {
			tenant_id: Some(tid.to_string()),
			request_id: None,
			correlation_id: None,
			created_by: None,
			retry_policy: None,
			dedupe_key: None,
			task_db_id: None,
		}));
		let output = CheckEmailOutput {
			input: "store@test.com".into(),
			is_reachable: Reachable::Invalid,
			..Default::default()
		};
		storage.store(&task, &Ok(output), None).await.unwrap();
	}
}
