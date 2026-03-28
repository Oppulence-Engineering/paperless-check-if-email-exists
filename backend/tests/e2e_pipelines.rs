mod test_helpers;

use crate::test_helpers::{
	insert_api_key_with_scopes, insert_tenant, test_amqp_url, test_db_url, TestDb,
};
use reacher_backend::config::{
	BackendConfig, PipelinesConfig, PostgresConfig, RabbitMQConfig, StorageConfig, WorkerConfig,
};
use reacher_backend::http::create_routes;
use reacher_backend::pipelines::{
	create_pipeline, run_pipeline_scheduler_cycle, CreatePipelineInput, PipelineDeliveryConfig,
	PipelineSchedule, PipelineSource, PipelineStatus, PipelineVerificationSettings,
};
use serial_test::serial;
use sqlx::Row;
use std::sync::Arc;
use warp::http::StatusCode;
use warp::test::request;

async fn worker_pipeline_config(enable_scheduler: bool) -> Arc<BackendConfig> {
	let mut c = BackendConfig::empty();
	c.storage = Some(StorageConfig::Postgres(PostgresConfig {
		db_url: test_db_url(),
		read_replica_url: None,
		extra: None,
	}));
	c.worker = WorkerConfig {
		enable: true,
		rabbitmq: Some(RabbitMQConfig {
			url: test_amqp_url(),
			concurrency: 4,
		}),
		webhook: None,
	};
	c.pipelines = PipelinesConfig {
		enable: enable_scheduler,
		tick_seconds: 60,
		max_due_per_tick: 10,
		max_missed_run_age_hours: 24,
		min_interval_seconds: 3600,
	};
	c.connect().await.unwrap();
	Arc::new(c)
}

async fn insert_source_list(pool: &sqlx::PgPool, tenant_id: uuid::Uuid) -> i32 {
	insert_source_list_with_data(
		pool,
		tenant_id,
		serde_json::json!({
			"0": { "email": "Alice@example.com", "name": "Alice" },
			"1": { "email": "alice@example.com", "name": "Alice Dup" },
			"2": { "email": "", "name": "Blank" }
		}),
	)
	.await
}

async fn insert_source_list_with_data(
	pool: &sqlx::PgPool,
	tenant_id: uuid::Uuid,
	original_data: serde_json::Value,
) -> i32 {
	sqlx::query_scalar(
		r#"
		INSERT INTO v1_lists (
			tenant_id, name, original_filename, file_size_bytes, total_rows,
			email_column, original_headers, original_data, status, unique_emails, deduplicated_count
		)
		VALUES ($1, 'Seed List', 'seed.csv', 123, 3, 'email', ARRAY['email', 'name'], $2, 'completed'::list_status, 1, 1)
		RETURNING id
		"#,
	)
	.bind(tenant_id)
	.bind(original_data)
	.fetch_one(pool)
	.await
	.unwrap()
}

#[tokio::test]
#[serial]
async fn test_pipeline_api_create_get_list_and_trigger() {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "pipeline-api", Some(1000), 0).await;
	let list_id = insert_source_list(db.pool(), tenant_id).await;
	let (api_key, _) = insert_api_key_with_scopes(
		db.pool(),
		tenant_id,
		&["pipelines.read", "pipelines.write", "pipelines.trigger"],
	)
	.await;
	let config = worker_pipeline_config(false).await;

	let create_response = request()
		.method("POST")
		.path("/v1/pipelines")
		.header("Authorization", format!("Bearer {}", api_key))
		.json(&serde_json::json!({
			"name": "Weekly Cleanup",
			"source": { "type": "list_snapshot", "list_id": list_id },
			"schedule": { "cron": "0 9 * * 1", "timezone": "America/New_York" },
			"verification": { "delta_mode": false, "freshness_days": 30 },
			"delivery": { "dashboard": true },
			"status": "active"
		}))
		.reply(&create_routes(Arc::clone(&config)))
		.await;
	assert_eq!(create_response.status(), StatusCode::CREATED);
	let created: serde_json::Value = serde_json::from_slice(create_response.body()).unwrap();
	let pipeline_id = created["id"].as_i64().unwrap();

	let get_response = request()
		.method("GET")
		.path(&format!("/v1/pipelines/{}", pipeline_id))
		.header("Authorization", format!("Bearer {}", api_key))
		.reply(&create_routes(Arc::clone(&config)))
		.await;
	assert_eq!(get_response.status(), StatusCode::OK);

	let list_response = request()
		.method("GET")
		.path("/v1/pipelines")
		.header("Authorization", format!("Bearer {}", api_key))
		.reply(&create_routes(Arc::clone(&config)))
		.await;
	assert_eq!(list_response.status(), StatusCode::OK);
	let list_body: serde_json::Value = serde_json::from_slice(list_response.body()).unwrap();
	assert_eq!(list_body["total"], 1);

	let trigger_response = request()
		.method("POST")
		.path(&format!("/v1/pipelines/{}/trigger", pipeline_id))
		.header("Authorization", format!("Bearer {}", api_key))
		.json(&serde_json::json!({ "force": false }))
		.reply(&create_routes(Arc::clone(&config)))
		.await;
	assert_eq!(trigger_response.status(), StatusCode::ACCEPTED);
	let trigger_body: serde_json::Value = serde_json::from_slice(trigger_response.body()).unwrap();
	let run_id = trigger_body["run_id"].as_i64().unwrap();

	let run_response = request()
		.method("GET")
		.path(&format!("/v1/pipelines/{}/runs/{}", pipeline_id, run_id))
		.header("Authorization", format!("Bearer {}", api_key))
		.reply(&create_routes(Arc::clone(&config)))
		.await;
	assert_eq!(run_response.status(), StatusCode::OK);
	let run_body: serde_json::Value = serde_json::from_slice(run_response.body()).unwrap();
	assert_eq!(run_body["pipeline_id"], pipeline_id);
	assert_eq!(run_body["job_id"].as_i64().unwrap() > 0, true);
	assert_eq!(run_body["list_id"].as_i64().unwrap() > 0, true);
}

#[tokio::test]
#[serial]
async fn test_pipeline_list_endpoints_clamp_negative_pagination() {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "pipeline-pagination", Some(1000), 0).await;
	let list_id = insert_source_list(db.pool(), tenant_id).await;
	let (api_key, _) = insert_api_key_with_scopes(
		db.pool(),
		tenant_id,
		&["pipelines.read", "pipelines.write", "pipelines.trigger"],
	)
	.await;
	let config = worker_pipeline_config(false).await;

	let create_response = request()
		.method("POST")
		.path("/v1/pipelines")
		.header("Authorization", format!("Bearer {}", api_key))
		.json(&serde_json::json!({
			"name": "Pagination Clamp",
			"source": { "type": "list_snapshot", "list_id": list_id },
			"schedule": { "cron": "0 9 * * 1", "timezone": "UTC" },
			"verification": { "delta_mode": false, "freshness_days": 30 },
			"delivery": { "dashboard": true },
			"status": "active"
		}))
		.reply(&create_routes(Arc::clone(&config)))
		.await;
	assert_eq!(create_response.status(), StatusCode::CREATED);
	let created: serde_json::Value = serde_json::from_slice(create_response.body()).unwrap();
	let pipeline_id = created["id"].as_i64().unwrap();

	let list_response = request()
		.method("GET")
		.path("/v1/pipelines?limit=-5&offset=-10")
		.header("Authorization", format!("Bearer {}", api_key))
		.reply(&create_routes(Arc::clone(&config)))
		.await;
	assert_eq!(list_response.status(), StatusCode::OK);
	let list_body: serde_json::Value = serde_json::from_slice(list_response.body()).unwrap();
	assert_eq!(list_body["total"], 1);

	let runs_response = request()
		.method("GET")
		.path(&format!(
			"/v1/pipelines/{pipeline_id}/runs?limit=-1&offset=-9"
		))
		.header("Authorization", format!("Bearer {}", api_key))
		.reply(&create_routes(Arc::clone(&config)))
		.await;
	assert_eq!(runs_response.status(), StatusCode::OK);
	let runs_body: serde_json::Value = serde_json::from_slice(runs_response.body()).unwrap();
	assert_eq!(runs_body["total"], 0);
}

#[tokio::test]
#[serial]
async fn test_pipeline_scheduler_cycle_creates_run_and_usage() {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "pipeline-scheduler", Some(1000), 0).await;
	let list_id = insert_source_list(db.pool(), tenant_id).await;
	let config = worker_pipeline_config(true).await;

	let pipeline = create_pipeline(
		db.pool(),
		tenant_id,
		CreatePipelineInput {
			name: "Scheduled Cleanup".to_string(),
			source: PipelineSource::ListSnapshot { list_id },
			schedule: PipelineSchedule {
				cron: "0 9 * * 1".to_string(),
				timezone: "UTC".to_string(),
			},
			verification: PipelineVerificationSettings::default(),
			policy: Default::default(),
			delivery: PipelineDeliveryConfig::default(),
			status: PipelineStatus::Active,
		},
		&config.pipelines,
	)
	.await
	.unwrap();

	sqlx::query("UPDATE v1_pipelines SET next_run_at = NOW() - INTERVAL '1 minute' WHERE id = $1")
		.bind(pipeline.id)
		.execute(db.pool())
		.await
		.unwrap();

	run_pipeline_scheduler_cycle(Arc::clone(&config), db.pool())
		.await
		.unwrap();

	let run_count: i64 =
		sqlx::query_scalar("SELECT COUNT(*) FROM v1_pipeline_runs WHERE pipeline_id = $1")
			.bind(pipeline.id)
			.fetch_one(db.pool())
			.await
			.unwrap();
	assert_eq!(run_count, 1);

	let usage_count: i64 =
		sqlx::query_scalar("SELECT COUNT(*) FROM v1_usage_events WHERE pipeline_id = $1")
			.bind(pipeline.id)
			.fetch_one(db.pool())
			.await
			.unwrap();
	assert_eq!(usage_count, 1);

	let row = sqlx::query(
		"SELECT job_id, list_id, status::TEXT FROM v1_pipeline_runs WHERE pipeline_id = $1 ORDER BY id DESC LIMIT 1",
	)
	.bind(pipeline.id)
	.fetch_one(db.pool())
	.await
	.unwrap();
	let job_id: Option<i32> = row.get("job_id");
	let run_status: String = row.get("status");
	assert!(job_id.is_some());
	assert!(matches!(
		run_status.as_str(),
		"running" | "completed" | "failed"
	));
}

#[tokio::test]
#[serial]
async fn test_pipeline_scheduler_recovers_stranded_queued_runs() {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "pipeline-recover-queued", Some(1000), 0).await;
	let list_id = insert_source_list(db.pool(), tenant_id).await;
	let config = worker_pipeline_config(true).await;

	let pipeline = create_pipeline(
		db.pool(),
		tenant_id,
		CreatePipelineInput {
			name: "Recover Queued".to_string(),
			source: PipelineSource::ListSnapshot { list_id },
			schedule: PipelineSchedule {
				cron: "0 9 * * 1".to_string(),
				timezone: "UTC".to_string(),
			},
			verification: PipelineVerificationSettings::default(),
			policy: Default::default(),
			delivery: PipelineDeliveryConfig::default(),
			status: PipelineStatus::Active,
		},
		&config.pipelines,
	)
	.await
	.unwrap();

	let run_id: i64 = sqlx::query_scalar(
		r#"
		INSERT INTO v1_pipeline_runs (pipeline_id, tenant_id, trigger_type, status, source_snapshot, stats, updated_at)
		VALUES ($1, $2, 'schedule', 'queued', $3, '{}'::jsonb, NOW() - INTERVAL '10 minutes')
		RETURNING id
		"#,
	)
	.bind(pipeline.id)
	.bind(tenant_id)
	.bind(serde_json::json!({ "type": "list_snapshot", "list_id": list_id }))
	.fetch_one(db.pool())
	.await
	.unwrap();

	run_pipeline_scheduler_cycle(Arc::clone(&config), db.pool())
		.await
		.unwrap();

	let row = sqlx::query(
		"SELECT status::TEXT, job_id, list_id, started_at FROM v1_pipeline_runs WHERE id = $1",
	)
	.bind(run_id)
	.fetch_one(db.pool())
	.await
	.unwrap();
	let status: String = row.get("status");
	let job_id: Option<i32> = row.get("job_id");
	let list_id: Option<i32> = row.get("list_id");
	let started_at: Option<chrono::DateTime<chrono::Utc>> = row.get("started_at");
	assert!(job_id.is_some());
	assert!(list_id.is_some());
	assert!(started_at.is_some());
	assert!(matches!(
		status.as_str(),
		"running" | "completed" | "failed" | "cancelled" | "delivering"
	));
}

#[tokio::test]
#[serial]
async fn test_pipeline_recovered_run_executes_from_snapshot_source() {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "pipeline-source-snapshot", Some(1000), 0).await;
	let original_list_id = insert_source_list_with_data(
		db.pool(),
		tenant_id,
		serde_json::json!({
			"0": { "email": "original@example.com", "name": "Original" }
		}),
	)
	.await;
	let edited_list_id = insert_source_list_with_data(
		db.pool(),
		tenant_id,
		serde_json::json!({
			"0": { "email": "edited@example.com", "name": "Edited" }
		}),
	)
	.await;
	let config = worker_pipeline_config(true).await;

	let pipeline = create_pipeline(
		db.pool(),
		tenant_id,
		CreatePipelineInput {
			name: "Snapshot Source".to_string(),
			source: PipelineSource::ListSnapshot {
				list_id: original_list_id,
			},
			schedule: PipelineSchedule {
				cron: "0 9 * * 1".to_string(),
				timezone: "UTC".to_string(),
			},
			verification: PipelineVerificationSettings::default(),
			policy: Default::default(),
			delivery: PipelineDeliveryConfig::default(),
			status: PipelineStatus::Active,
		},
		&config.pipelines,
	)
	.await
	.unwrap();

	let run_id: i64 = sqlx::query_scalar(
		r#"
		INSERT INTO v1_pipeline_runs (pipeline_id, tenant_id, trigger_type, status, source_snapshot, stats, updated_at)
		VALUES ($1, $2, 'schedule', 'queued', $3, '{}'::jsonb, NOW() - INTERVAL '10 minutes')
		RETURNING id
		"#,
	)
	.bind(pipeline.id)
	.bind(tenant_id)
	.bind(serde_json::json!({ "type": "list_snapshot", "list_id": original_list_id }))
	.fetch_one(db.pool())
	.await
	.unwrap();

	sqlx::query("UPDATE v1_pipelines SET source_config = $2 WHERE id = $1")
		.bind(pipeline.id)
		.bind(serde_json::json!({ "type": "list_snapshot", "list_id": edited_list_id }))
		.execute(db.pool())
		.await
		.unwrap();

	run_pipeline_scheduler_cycle(Arc::clone(&config), db.pool())
		.await
		.unwrap();

	let created_list_id: i32 =
		sqlx::query_scalar("SELECT list_id FROM v1_pipeline_runs WHERE id = $1")
			.bind(run_id)
			.fetch_one(db.pool())
			.await
			.unwrap();

	let source_list_id: Option<i32> =
		sqlx::query_scalar("SELECT source_list_id FROM v1_lists WHERE id = $1")
			.bind(created_list_id)
			.fetch_one(db.pool())
			.await
			.unwrap();
	assert_eq!(source_list_id, Some(original_list_id));
}
