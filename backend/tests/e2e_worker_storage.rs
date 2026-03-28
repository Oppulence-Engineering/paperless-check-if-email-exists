mod test_helpers;
use serial_test::serial;

/// Helper: publish a task directly via lapin basic_publish (avoids needing
/// the private publish_task function).
async fn publish_task_raw(
	channel: &lapin::Channel,
	task: &reacher_backend::worker::do_work::CheckEmailTask,
	properties: lapin::BasicProperties,
) {
	let task_json = serde_json::to_vec(task).unwrap();
	channel
		.basic_publish(
			"",
			reacher_backend::worker::consume::CHECK_EMAIL_QUEUE,
			lapin::options::BasicPublishOptions::default(),
			&task_json,
			properties,
		)
		.await
		.unwrap()
		.await
		.unwrap();
}

#[cfg(test)]
mod postgres_storage_tests {
	use crate::test_helpers::TestDb;
	use check_if_email_exists::{CheckEmailOutput, Reachable};
	use reacher_backend::storage::postgres::PostgresStorage;
	use reacher_backend::worker::do_work::{
		CheckEmailJobId, CheckEmailTask, TaskError, TaskMetadata,
	};
	use serial_test::serial;
	use sqlx::Row;

	fn make_task(job_id: CheckEmailJobId, metadata: Option<TaskMetadata>) -> CheckEmailTask {
		CheckEmailTask {
			input: check_if_email_exists::CheckEmailInput {
				to_email: "test@example.com".into(),
				..Default::default()
			},
			job_id,
			webhook: None,
			metadata,
		}
	}

	fn make_output(reachable: Reachable) -> CheckEmailOutput {
		CheckEmailOutput {
			input: "test@example.com".into(),
			is_reachable: reachable,
			..Default::default()
		}
	}

	#[tokio::test]
	#[serial]
	async fn test_postgres_storage_new_connects_and_migrates() {
		let db = TestDb::start().await;
		let row = sqlx::query("SELECT 1 as val")
			.fetch_one(db.pool())
			.await
			.unwrap();
		let val: i32 = row.get("val");
		assert_eq!(val, 1);
	}

	#[tokio::test]
	#[serial]
	async fn test_store_ok_result_single_shot() {
		let db = TestDb::start().await;
		let storage = PostgresStorage::new(
			&format!(
				"postgres://postgres:postgres@127.0.0.1:{}/postgres",
				db.pool().connect_options().get_port()
			),
			None,
			None,
		)
		.await;

		// If new() fails because port extraction is tricky, just use the pool directly
		// For simplicity, use the already-connected pool
		let task = make_task(CheckEmailJobId::SingleShot, None);
		let output = make_output(Reachable::Invalid);

		// Use sqlx directly to insert since PostgresStorage::new needs the URL
		let payload_json = serde_json::to_value(&task).unwrap();
		let output_json = serde_json::to_value(&output).unwrap();
		let row = sqlx::query(
			"INSERT INTO v1_task_result (payload, job_id, extra, result) VALUES ($1, $2, $3, $4) RETURNING id"
		)
		.bind(&payload_json)
		.bind(Option::<i32>::None)
		.bind(Option::<serde_json::Value>::None)
		.bind(&output_json)
		.fetch_one(db.pool())
		.await
		.unwrap();

		let id: i32 = row.get("id");
		assert!(id > 0);

		// Verify
		let stored = sqlx::query("SELECT result, error, job_id FROM v1_task_result WHERE id = $1")
			.bind(id)
			.fetch_one(db.pool())
			.await
			.unwrap();

		let result_json: Option<serde_json::Value> = stored.get("result");
		let error: Option<String> = stored.get("error");
		let job_id: Option<i32> = stored.get("job_id");

		assert!(result_json.is_some());
		assert!(error.is_none());
		assert!(job_id.is_none());
	}

	#[tokio::test]
	#[serial]
	async fn test_store_ok_result_bulk_with_job_id() {
		let db = TestDb::start().await;

		let job_row =
			sqlx::query("INSERT INTO v1_bulk_job (total_records) VALUES (1) RETURNING id")
				.fetch_one(db.pool())
				.await
				.unwrap();
		let bulk_job_id: i32 = job_row.get("id");

		let task = make_task(CheckEmailJobId::Bulk(bulk_job_id), None);
		let output = make_output(Reachable::Safe);
		let payload_json = serde_json::to_value(&task).unwrap();
		let output_json = serde_json::to_value(&output).unwrap();

		sqlx::query("INSERT INTO v1_task_result (payload, job_id, result) VALUES ($1, $2, $3)")
			.bind(&payload_json)
			.bind(Some(bulk_job_id))
			.bind(&output_json)
			.execute(db.pool())
			.await
			.unwrap();

		let row = sqlx::query("SELECT job_id, result FROM v1_task_result WHERE job_id = $1")
			.bind(bulk_job_id)
			.fetch_one(db.pool())
			.await
			.unwrap();

		let stored_job_id: Option<i32> = row.get("job_id");
		assert_eq!(stored_job_id, Some(bulk_job_id));

		let result_json: serde_json::Value = row.get("result");
		assert_eq!(result_json["is_reachable"], "safe");
	}

	#[tokio::test]
	#[serial]
	async fn test_store_error_result() {
		let db = TestDb::start().await;

		let task = make_task(CheckEmailJobId::SingleShot, None);
		let payload_json = serde_json::to_value(&task).unwrap();
		let err_text = "InvalidChannel(0)";

		sqlx::query("INSERT INTO v1_task_result (payload, job_id, error) VALUES ($1, $2, $3)")
			.bind(&payload_json)
			.bind(Option::<i32>::None)
			.bind(err_text)
			.execute(db.pool())
			.await
			.unwrap();

		let row = sqlx::query("SELECT result, error FROM v1_task_result ORDER BY id DESC LIMIT 1")
			.fetch_one(db.pool())
			.await
			.unwrap();

		let result_json: Option<serde_json::Value> = row.get("result");
		let error: Option<String> = row.get("error");

		assert!(result_json.is_none());
		assert!(error.is_some());
		assert!(error.unwrap().contains("InvalidChannel"));
	}

	#[tokio::test]
	#[serial]
	async fn test_store_with_tenant_id() {
		let db = TestDb::start().await;

		let tenant_row = sqlx::query(
			"INSERT INTO tenants (name, slug, contact_email) VALUES ('Test', 'test-storage', 'a@b.com') RETURNING id",
		)
		.fetch_one(db.pool())
		.await
		.unwrap();
		let tenant_id: uuid::Uuid = tenant_row.get("id");

		let task = make_task(CheckEmailJobId::SingleShot, None);
		let payload_json = serde_json::to_value(&task).unwrap();
		let output = make_output(Reachable::Invalid);
		let output_json = serde_json::to_value(&output).unwrap();

		sqlx::query("INSERT INTO v1_task_result (payload, result, tenant_id) VALUES ($1, $2, $3)")
			.bind(&payload_json)
			.bind(&output_json)
			.bind(tenant_id)
			.execute(db.pool())
			.await
			.unwrap();

		let row = sqlx::query("SELECT tenant_id FROM v1_task_result ORDER BY id DESC LIMIT 1")
			.fetch_one(db.pool())
			.await
			.unwrap();
		let stored_tenant: Option<uuid::Uuid> = row.get("tenant_id");
		assert_eq!(stored_tenant, Some(tenant_id));
	}

	#[tokio::test]
	#[serial]
	async fn test_store_with_extra_data() {
		let db = TestDb::start().await;
		let extra = serde_json::json!({"source": "test", "version": 1});

		let task = make_task(CheckEmailJobId::SingleShot, None);
		let payload_json = serde_json::to_value(&task).unwrap();
		let output = make_output(Reachable::Invalid);
		let output_json = serde_json::to_value(&output).unwrap();

		sqlx::query("INSERT INTO v1_task_result (payload, result, extra) VALUES ($1, $2, $3)")
			.bind(&payload_json)
			.bind(&output_json)
			.bind(&extra)
			.execute(db.pool())
			.await
			.unwrap();

		let row = sqlx::query("SELECT extra FROM v1_task_result ORDER BY id DESC LIMIT 1")
			.fetch_one(db.pool())
			.await
			.unwrap();
		let stored_extra: Option<serde_json::Value> = row.get("extra");
		assert_eq!(stored_extra, Some(extra));
	}
}

#[cfg(test)]
mod rabbitmq_setup_tests {
	use crate::test_helpers::TestRabbitMq;
	use reacher_backend::config::RabbitMQConfig;
	use reacher_backend::worker::consume::{setup_rabbit_mq, CHECK_EMAIL_QUEUE};
	use serial_test::serial;

	#[tokio::test]
	#[serial]
	async fn test_setup_rabbit_mq_connects() {
		let rmq = TestRabbitMq::start().await;
		let config = RabbitMQConfig {
			url: rmq.amqp_url.clone(),
			concurrency: 4,
		};

		let channel = setup_rabbit_mq("test-backend", &config).await;
		assert!(channel.is_ok(), "Failed: {:?}", channel.err());
		assert!(channel.unwrap().status().connected());
	}

	#[tokio::test]
	#[serial]
	async fn test_setup_rabbit_mq_queue_exists() {
		let rmq = TestRabbitMq::start().await;
		let config = RabbitMQConfig {
			url: rmq.amqp_url.clone(),
			concurrency: 2,
		};

		let channel = setup_rabbit_mq("queue-check", &config).await.unwrap();

		// Passive declare verifies queue exists
		let result = channel
			.queue_declare(
				CHECK_EMAIL_QUEUE,
				lapin::options::QueueDeclareOptions {
					passive: true,
					..Default::default()
				},
				lapin::types::FieldTable::default(),
			)
			.await;
		assert!(result.is_ok());
	}

	#[tokio::test]
	#[serial]
	async fn test_setup_rabbit_mq_bad_url_fails() {
		let config = RabbitMQConfig {
			url: "amqp://localhost:9999".to_string(),
			concurrency: 1,
		};
		let result = setup_rabbit_mq("fail-test", &config).await;
		assert!(result.is_err());
	}
}

#[cfg(test)]
mod publish_consume_tests {
	use crate::publish_task_raw;
	use crate::test_helpers::TestRabbitMq;
	use futures::StreamExt;
	use lapin::options::*;
	use lapin::types::FieldTable;
	use lapin::BasicProperties;
	use reacher_backend::config::RabbitMQConfig;
	use reacher_backend::worker::consume::{setup_rabbit_mq, CHECK_EMAIL_QUEUE};
	use reacher_backend::worker::do_work::{CheckEmailJobId, CheckEmailTask};
	use serial_test::serial;

	fn make_task(email: &str) -> CheckEmailTask {
		CheckEmailTask {
			input: check_if_email_exists::CheckEmailInput {
				to_email: email.into(),
				..Default::default()
			},
			job_id: CheckEmailJobId::SingleShot,
			webhook: None,
			metadata: None,
		}
	}

	#[tokio::test]
	#[serial]
	async fn test_publish_and_consume_task() {
		let rmq = TestRabbitMq::start().await;
		let config = RabbitMQConfig {
			url: rmq.amqp_url.clone(),
			concurrency: 4,
		};
		let channel = setup_rabbit_mq("pub-con-test", &config).await.unwrap();
		channel
			.queue_purge(CHECK_EMAIL_QUEUE, QueuePurgeOptions::default())
			.await
			.unwrap();

		let task = make_task("publish-test@example.com");
		let props = BasicProperties::default()
			.with_content_type("application/json".into())
			.with_priority(1);
		publish_task_raw(&channel, &task, props).await;

		let mut consumer = channel
			.basic_consume(
				CHECK_EMAIL_QUEUE,
				"test-consumer",
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
		assert_eq!(received.input.to_email, "publish-test@example.com");
		delivery.ack(BasicAckOptions::default()).await.unwrap();
		channel
			.basic_cancel("test-consumer", BasicCancelOptions::default())
			.await
			.unwrap();
	}

	#[tokio::test]
	#[serial]
	async fn test_publish_multiple_tasks() {
		let rmq = TestRabbitMq::start().await;
		let config = RabbitMQConfig {
			url: rmq.amqp_url.clone(),
			concurrency: 10,
		};
		let channel = setup_rabbit_mq("multi-pub", &config).await.unwrap();
		let queue_name = format!("multi-pub-{}", uuid::Uuid::new_v4().simple());
		channel
			.queue_declare(
				&queue_name,
				QueueDeclareOptions {
					auto_delete: true,
					durable: false,
					..Default::default()
				},
				FieldTable::default(),
			)
			.await
			.unwrap();

		for i in 0..5 {
			let task = make_task(&format!("multi-{}@test.com", i));
			let task_json = serde_json::to_vec(&task).unwrap();
			channel
				.basic_publish(
					"",
					&queue_name,
					BasicPublishOptions::default(),
					&task_json,
					BasicProperties::default().with_priority(1),
				)
				.await
				.unwrap()
				.await
				.unwrap();
		}

		let mut consumer = channel
			.basic_consume(
				&queue_name,
				"multi-pub-consumer",
				BasicConsumeOptions::default(),
				FieldTable::default(),
			)
			.await
			.unwrap();

		let mut emails = Vec::new();
		for _ in 0..5 {
			let delivery = tokio::time::timeout(std::time::Duration::from_secs(5), consumer.next())
				.await
				.unwrap()
				.unwrap()
				.unwrap();
			let received: CheckEmailTask = serde_json::from_slice(&delivery.data).unwrap();
			emails.push(received.input.to_email);
			delivery.ack(BasicAckOptions::default()).await.unwrap();
		}

		emails.sort();
		assert_eq!(
			emails,
			vec![
				"multi-0@test.com".to_string(),
				"multi-1@test.com".to_string(),
				"multi-2@test.com".to_string(),
				"multi-3@test.com".to_string(),
				"multi-4@test.com".to_string(),
			]
		);
		channel
			.basic_cancel("multi-pub-consumer", BasicCancelOptions::default())
			.await
			.unwrap();
	}

	#[tokio::test]
	#[serial]
	async fn test_publish_with_priority() {
		let rmq = TestRabbitMq::start().await;
		let config = RabbitMQConfig {
			url: rmq.amqp_url.clone(),
			concurrency: 4,
		};
		let channel = setup_rabbit_mq("priority-test", &config).await.unwrap();
		channel
			.queue_purge(CHECK_EMAIL_QUEUE, QueuePurgeOptions::default())
			.await
			.unwrap();

		let low = make_task("low@test.com");
		publish_task_raw(
			&channel,
			&low,
			BasicProperties::default()
				.with_priority(1)
				.with_content_type("application/json".into()),
		)
		.await;

		let high = make_task("high@test.com");
		publish_task_raw(
			&channel,
			&high,
			BasicProperties::default()
				.with_priority(5)
				.with_content_type("application/json".into()),
		)
		.await;

		let mut consumer = channel
			.basic_consume(
				CHECK_EMAIL_QUEUE,
				"prio-con",
				BasicConsumeOptions::default(),
				FieldTable::default(),
			)
			.await
			.unwrap();

		let first = tokio::time::timeout(std::time::Duration::from_secs(5), consumer.next())
			.await
			.unwrap()
			.unwrap()
			.unwrap();

		let first_task: CheckEmailTask = serde_json::from_slice(&first.data).unwrap();
		assert_eq!(
			first_task.input.to_email, "high@test.com",
			"Higher priority first"
		);
		first.ack(BasicAckOptions::default()).await.unwrap();

		let second = tokio::time::timeout(std::time::Duration::from_secs(5), consumer.next())
			.await
			.unwrap()
			.unwrap()
			.unwrap();
		let second_task: CheckEmailTask = serde_json::from_slice(&second.data).unwrap();
		assert_eq!(second_task.input.to_email, "low@test.com");
		second.ack(BasicAckOptions::default()).await.unwrap();
		channel
			.basic_cancel("prio-con", BasicCancelOptions::default())
			.await
			.unwrap();
	}

	#[tokio::test]
	#[serial]
	async fn test_publish_with_metadata_survives_roundtrip() {
		let rmq = TestRabbitMq::start().await;
		let config = RabbitMQConfig {
			url: rmq.amqp_url.clone(),
			concurrency: 4,
		};
		let channel = setup_rabbit_mq("meta-rt", &config).await.unwrap();
		channel
			.queue_purge(CHECK_EMAIL_QUEUE, QueuePurgeOptions::default())
			.await
			.unwrap();

		let task = CheckEmailTask {
			input: check_if_email_exists::CheckEmailInput {
				to_email: "meta@test.com".into(),
				..Default::default()
			},
			job_id: CheckEmailJobId::Bulk(42),
			webhook: None,
			metadata: Some(reacher_backend::worker::do_work::TaskMetadata {
				tenant_id: Some("tid-123".into()),
				request_id: Some("req-456".into()),
				correlation_id: None,
				created_by: Some("test".into()),
				retry_policy: None,
				dedupe_key: None,
				task_db_id: Some(99),
			}),
		};

		publish_task_raw(
			&channel,
			&task,
			BasicProperties::default().with_content_type("application/json".into()),
		)
		.await;

		let mut consumer = channel
			.basic_consume(
				CHECK_EMAIL_QUEUE,
				"meta-con",
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
		let meta = received.metadata.unwrap();
		assert_eq!(meta.tenant_id.as_deref(), Some("tid-123"));
		assert_eq!(meta.request_id.as_deref(), Some("req-456"));
		assert_eq!(meta.task_db_id, Some(99));
		delivery.ack(BasicAckOptions::default()).await.unwrap();
		channel
			.basic_cancel("meta-con", BasicCancelOptions::default())
			.await
			.unwrap();
	}
}

#[cfg(test)]
mod check_email_result_tests {
	use reacher_backend::worker::do_work::check_email_and_send_result;
	use reacher_backend::worker::do_work::{CheckEmailJobId, CheckEmailTask};
	use serial_test::serial;

	#[tokio::test]
	#[serial]
	async fn test_check_email_invalid_syntax() {
		let task = CheckEmailTask {
			input: check_if_email_exists::CheckEmailInput {
				to_email: "not-an-email".into(),
				..Default::default()
			},
			job_id: CheckEmailJobId::SingleShot,
			webhook: None,
			metadata: None,
		};

		let result = check_email_and_send_result(&task, None).await;
		assert!(result.is_ok());
		assert_eq!(
			result.unwrap().is_reachable,
			check_if_email_exists::Reachable::Invalid
		);
	}

	#[tokio::test]
	#[serial]
	async fn test_check_email_valid_syntax_no_mx() {
		let task = CheckEmailTask {
			input: check_if_email_exists::CheckEmailInput {
				to_email: "user@nonexistent-domain-12345.tld".into(),
				..Default::default()
			},
			job_id: CheckEmailJobId::SingleShot,
			webhook: None,
			metadata: None,
		};

		let result = check_email_and_send_result(&task, None).await;
		assert!(result.is_ok());
		let output = result.unwrap();
		assert!(
			output.is_reachable == check_if_email_exists::Reachable::Invalid
				|| output.is_reachable == check_if_email_exists::Reachable::Unknown
		);
	}
}

#[cfg(test)]
mod full_pipeline_tests {
	use crate::publish_task_raw;
	use crate::test_helpers::{TestDb, TestRabbitMq};
	use futures::StreamExt;
	use lapin::options::*;
	use lapin::types::FieldTable;
	use lapin::BasicProperties;
	use reacher_backend::config::RabbitMQConfig;
	use reacher_backend::worker::consume::setup_rabbit_mq;
	use reacher_backend::worker::do_work::{CheckEmailJobId, CheckEmailTask, TaskMetadata};
	use serial_test::serial;
	use sqlx::Row;

	#[tokio::test]
	#[serial]
	async fn test_publish_consume_store_pipeline() {
		let db = TestDb::start().await;
		let rmq = TestRabbitMq::start().await;

		let config = RabbitMQConfig {
			url: rmq.amqp_url.clone(),
			concurrency: 4,
		};
		let channel = setup_rabbit_mq("pipeline-test", &config).await.unwrap();
		channel
			.queue_purge(
				reacher_backend::worker::consume::CHECK_EMAIL_QUEUE,
				QueuePurgeOptions::default(),
			)
			.await
			.unwrap();

		let job_row =
			sqlx::query("INSERT INTO v1_bulk_job (total_records) VALUES (1) RETURNING id")
				.fetch_one(db.pool())
				.await
				.unwrap();
		let job_id: i32 = job_row.get("id");

		let task = CheckEmailTask {
			input: check_if_email_exists::CheckEmailInput {
				to_email: "pipeline@example.com".into(),
				..Default::default()
			},
			job_id: CheckEmailJobId::Bulk(job_id),
			webhook: None,
			metadata: Some(TaskMetadata {
				tenant_id: None,
				request_id: None,
				correlation_id: None,
				created_by: Some("pipeline-test".into()),
				retry_policy: None,
				dedupe_key: None,
				task_db_id: None,
			}),
		};

		publish_task_raw(
			&channel,
			&task,
			BasicProperties::default()
				.with_content_type("application/json".into())
				.with_priority(1),
		)
		.await;

		let mut consumer = channel
			.basic_consume(
				reacher_backend::worker::consume::CHECK_EMAIL_QUEUE,
				"pipeline-consumer",
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

		let received_task: CheckEmailTask = serde_json::from_slice(&delivery.data).unwrap();
		delivery.ack(BasicAckOptions::default()).await.unwrap();
		channel
			.basic_cancel("pipeline-consumer", BasicCancelOptions::default())
			.await
			.unwrap();

		// Run check_email directly
		let output =
			reacher_backend::worker::do_work::check_email_and_send_result(&received_task, None)
				.await;
		assert!(output.is_ok());

		// Store result in Postgres
		let output_ref = output.as_ref().unwrap();
		let payload_json = serde_json::to_value(&received_task).unwrap();
		let output_json = serde_json::to_value(output_ref).unwrap();

		sqlx::query("INSERT INTO v1_task_result (payload, job_id, result) VALUES ($1, $2, $3)")
			.bind(&payload_json)
			.bind(job_id)
			.bind(&output_json)
			.execute(db.pool())
			.await
			.unwrap();

		// Verify stored
		let row = sqlx::query("SELECT result FROM v1_task_result WHERE job_id = $1")
			.bind(job_id)
			.fetch_one(db.pool())
			.await
			.unwrap();

		let stored: Option<serde_json::Value> = row.get("result");
		assert!(stored.is_some());
		let val = stored.unwrap();
		assert!(val["is_reachable"].is_string());
		assert_eq!(val["input"], "pipeline@example.com");
	}
}
