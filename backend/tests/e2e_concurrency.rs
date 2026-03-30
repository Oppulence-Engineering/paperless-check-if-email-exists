mod resilience_helpers;
mod test_helpers;

use crate::resilience_helpers::OwnedResilienceEnv;
use crate::test_helpers::{
	build_test_config, insert_comment, insert_job, insert_keys_for_existing_tenant, insert_list,
	insert_tenant, ConfigProfile, ADMIN_SECRET,
};
use futures::future::join_all;
use reacher_backend::http::idempotency::hash_request_body;
use reacher_backend::http::{create_routes, REACHER_SECRET_HEADER};
use serial_test::serial;
use sqlx::Row;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::Barrier;
use uuid::Uuid;
use warp::http::StatusCode;
use warp::test::request;

async fn seed_source_list(pool: &sqlx::PgPool, tenant_id: Uuid) -> i32 {
	let job_id = insert_job(pool, Some(tenant_id), 1, "completed").await;
	insert_list(
		pool,
		tenant_id,
		job_id,
		"Concurrent Source",
		"completed",
		1,
		&["email"],
		serde_json::json!({
			"0": { "email": "seed@example.com" }
		}),
	)
	.await
}

#[tokio::test]
#[serial]
async fn same_idempotency_key_same_body_creates_one_record_and_no_duplicate_side_effects() {
	let env = OwnedResilienceEnv::start().await;
	let pool = env.postgres.pool().await;
	let tenant_id = insert_tenant(&pool, "idem-same", Some(1000), 0).await;
	let keys = insert_keys_for_existing_tenant(&pool, tenant_id).await;
	pool.close().await;

	let config =
		build_test_config(ConfigProfile::BearerTenant, Some(env.postgres.db_url()), None).await;
	let barrier = Arc::new(Barrier::new(10));
	let requests = (0..10).map(|_| {
		let barrier = Arc::clone(&barrier);
		let config = Arc::clone(&config);
		let api_key = keys.full_access_key.clone();
		async move {
			barrier.wait().await;
			request()
				.method("POST")
				.path("/v1/check_email")
				.header("Authorization", format!("Bearer {}", api_key))
				.header("Idempotency-Key", "idem-same-body")
				.json(&serde_json::json!({
					"to_email": "bad-syntax"
				}))
				.reply(&create_routes(config))
				.await
		}
	});
	let responses = join_all(requests).await;
	assert!(responses
		.iter()
		.all(|response| matches!(response.status(), StatusCode::OK | StatusCode::CONFLICT)));
	assert!(responses
		.iter()
		.any(|response| response.status() == StatusCode::OK));

	let pool = env.postgres.pool().await;
	let idempotency_rows: i64 = sqlx::query_scalar(
		"SELECT COUNT(*) FROM idempotency_keys WHERE tenant_id = $1 AND idempotency_key = $2",
	)
	.bind(tenant_id.to_string())
	.bind("idem-same-body")
	.fetch_one(&pool)
	.await
	.expect("count idempotency rows");
	assert_eq!(idempotency_rows, 1);

	let bulk_jobs: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM v1_bulk_job")
		.fetch_one(&pool)
		.await
		.expect("count bulk jobs");
	assert_eq!(bulk_jobs, 0);
	pool.close().await;
}

#[tokio::test]
#[serial]
async fn same_idempotency_key_different_bodies_pick_one_winner_without_extra_rows() {
	let env = OwnedResilienceEnv::start().await;
	let pool = env.postgres.pool().await;
	let tenant_id = insert_tenant(&pool, "idem-mismatch", Some(1000), 0).await;
	let keys = insert_keys_for_existing_tenant(&pool, tenant_id).await;
	pool.close().await;

	let config =
		build_test_config(ConfigProfile::BearerTenant, Some(env.postgres.db_url()), None).await;
	let barrier = Arc::new(Barrier::new(6));
	let requests = (0..6).map(|index| {
		let barrier = Arc::clone(&barrier);
		let config = Arc::clone(&config);
		let api_key = keys.full_access_key.clone();
		let email = if index % 2 == 0 {
			"winner-invalid"
		} else {
			"loser-invalid"
		};
		async move {
			barrier.wait().await;
			request()
				.method("POST")
				.path("/v1/check_email")
				.header("Authorization", format!("Bearer {}", api_key))
				.header("Idempotency-Key", "idem-body-mismatch")
				.json(&serde_json::json!({
					"to_email": email
				}))
				.reply(&create_routes(config))
				.await
		}
	});
	let responses = join_all(requests).await;
	assert!(responses.iter().any(|response| response.status() == StatusCode::OK));
	assert!(responses.iter().all(|response| {
		matches!(
			response.status(),
			StatusCode::OK | StatusCode::BAD_REQUEST | StatusCode::CONFLICT
		)
	}));

	let pool = env.postgres.pool().await;
	let stored_hash: Vec<u8> = sqlx::query_scalar(
		"SELECT request_body_hash FROM idempotency_keys WHERE tenant_id = $1 AND idempotency_key = $2",
	)
	.bind(tenant_id.to_string())
	.bind("idem-body-mismatch")
	.fetch_one(&pool)
	.await
	.expect("fetch stored idempotency hash");
	let winner_body = serde_json::json!({ "to_email": "winner-invalid" });
	let loser_body = serde_json::json!({ "to_email": "loser-invalid" });
	let winner_hash =
		hash_request_body(&serde_json::to_vec(&winner_body).expect("serialize winner body"));
	let loser_hash =
		hash_request_body(&serde_json::to_vec(&loser_body).expect("serialize loser body"));
	assert!(
		stored_hash == winner_hash || stored_hash == loser_hash,
		"stored hash should match exactly one competing request body"
	);

	let follow_up_winner = request()
		.method("POST")
		.path("/v1/check_email")
		.header("Authorization", format!("Bearer {}", keys.full_access_key))
		.header("Idempotency-Key", "idem-body-mismatch")
		.json(if stored_hash == winner_hash {
			&winner_body
		} else {
			&loser_body
		})
		.reply(&create_routes(Arc::clone(&config)))
		.await;
	assert_eq!(follow_up_winner.status(), StatusCode::OK);

	let follow_up_loser = request()
		.method("POST")
		.path("/v1/check_email")
		.header("Authorization", format!("Bearer {}", keys.full_access_key))
		.header("Idempotency-Key", "idem-body-mismatch")
		.json(if stored_hash == winner_hash {
			&loser_body
		} else {
			&winner_body
		})
		.reply(&create_routes(Arc::clone(&config)))
		.await;
	assert_eq!(follow_up_loser.status(), StatusCode::BAD_REQUEST);
	let follow_up_json: serde_json::Value =
		serde_json::from_slice(follow_up_loser.body()).expect("follow up mismatch json");
	assert_eq!(follow_up_json["error"], "Idempotency key body mismatch");

	let idempotency_rows: i64 = sqlx::query_scalar(
		"SELECT COUNT(*) FROM idempotency_keys WHERE tenant_id = $1 AND idempotency_key = $2",
	)
	.bind(tenant_id.to_string())
	.bind("idem-body-mismatch")
	.fetch_one(&pool)
	.await
	.expect("count mismatch idempotency rows");
	assert_eq!(idempotency_rows, 1);
	pool.close().await;
}

#[tokio::test]
#[serial]
async fn parallel_bulk_job_creation_creates_exact_job_and_task_counts() {
	let env = OwnedResilienceEnv::start().await;
	let pool = env.postgres.pool().await;
	let tenant_id = insert_tenant(&pool, "bulk-concurrency", Some(1000), 0).await;
	let keys = insert_keys_for_existing_tenant(&pool, tenant_id).await;
	pool.close().await;

	let config = build_test_config(
		ConfigProfile::WorkerRabbit,
		Some(env.postgres.db_url()),
		Some(env.rabbitmq.amqp_url()),
	)
	.await;
	let barrier = Arc::new(Barrier::new(6));
	let requests = (0..6).map(|index| {
		let barrier = Arc::clone(&barrier);
		let config = Arc::clone(&config);
		let api_key = keys.full_access_key.clone();
		async move {
			barrier.wait().await;
			request()
				.method("POST")
				.path("/v1/bulk")
				.header("Authorization", format!("Bearer {}", api_key))
				.json(&serde_json::json!({
					"input": [
						format!("bulk-{}a@example.com", index),
						format!("bulk-{}b@example.com", index)
					]
				}))
				.reply(&create_routes(config))
				.await
		}
	});
	let responses = join_all(requests).await;
	assert!(responses
		.iter()
		.all(|response| response.status() == StatusCode::OK));

	let pool = env.postgres.pool().await;
	let job_count: i64 = sqlx::query_scalar(
		"SELECT COUNT(*) FROM v1_bulk_job WHERE tenant_id = $1",
	)
	.bind(tenant_id)
	.fetch_one(&pool)
	.await
	.expect("count bulk jobs");
	assert_eq!(job_count, 6);
	let task_count: i64 = sqlx::query_scalar(
		"SELECT COUNT(*) FROM v1_task_result WHERE tenant_id = $1",
	)
	.bind(tenant_id)
	.fetch_one(&pool)
	.await
	.expect("count bulk tasks");
	assert_eq!(task_count, 12);
	pool.close().await;
}

#[tokio::test]
#[serial]
async fn parallel_pipeline_trigger_force_false_creates_one_open_run() {
	let env = OwnedResilienceEnv::start().await;
	let pool = env.postgres.pool().await;
	let tenant_id = insert_tenant(&pool, "pipeline-trigger", Some(1000), 0).await;
	let keys = insert_keys_for_existing_tenant(&pool, tenant_id).await;
	let list_id = seed_source_list(&pool, tenant_id).await;
	let pipeline_id: i64 = sqlx::query_scalar(
		r#"
		INSERT INTO v1_pipelines (
			tenant_id, name, status, source_type, source_config, schedule_cron,
			schedule_timezone, verification_settings, delivery_config, next_run_at
		)
		VALUES (
			$1, 'Trigger Concurrency', 'active'::pipeline_status, 'list_snapshot'::pipeline_source_type,
			$2, '0 * * * *', 'UTC', '{}'::jsonb, '{}'::jsonb, NOW() + INTERVAL '1 hour'
		)
		RETURNING id
		"#,
	)
	.bind(tenant_id)
	.bind(serde_json::json!({"type": "list_snapshot", "list_id": list_id}))
	.fetch_one(&pool)
	.await
	.expect("insert pipeline");
	pool.close().await;

	let config = build_test_config(
		ConfigProfile::PipelineEnabled,
		Some(env.postgres.db_url()),
		Some(env.rabbitmq.amqp_url()),
	)
	.await;
	let barrier = Arc::new(Barrier::new(8));
	let requests = (0..8).map(|_| {
		let barrier = Arc::clone(&barrier);
		let config = Arc::clone(&config);
		let api_key = keys.pipelines_key.clone();
		async move {
			barrier.wait().await;
			request()
				.method("POST")
				.path(&format!("/v1/pipelines/{}/trigger", pipeline_id))
				.header("Authorization", format!("Bearer {}", api_key))
				.json(&serde_json::json!({
					"force": false,
					"reason": "concurrency"
				}))
				.reply(&create_routes(config))
				.await
		}
	});
	let responses = join_all(requests).await;
	assert!(responses.iter().any(|response| response.status() == StatusCode::ACCEPTED));
	assert!(responses.iter().all(|response| {
		matches!(response.status(), StatusCode::ACCEPTED | StatusCode::CONFLICT)
	}));

	let pool = env.postgres.pool().await;
	let open_runs: i64 = sqlx::query_scalar(
		r#"
		SELECT COUNT(*)
		FROM v1_pipeline_runs
		WHERE pipeline_id = $1
		  AND status IN (
			'queued'::pipeline_run_status,
			'preparing'::pipeline_run_status,
			'fetching_source'::pipeline_run_status,
			'publishing'::pipeline_run_status,
			'running'::pipeline_run_status,
			'delivering'::pipeline_run_status
		  )
		"#,
	)
	.bind(pipeline_id)
	.fetch_one(&pool)
	.await
	.expect("count open pipeline runs");
	assert_eq!(open_runs, 1);
	pool.close().await;
}

#[tokio::test]
#[serial]
async fn parallel_pause_and_resume_calls_leave_pipeline_in_valid_terminal_state() {
	let env = OwnedResilienceEnv::start().await;
	let pool = env.postgres.pool().await;
	let tenant_id = insert_tenant(&pool, "pipeline-state", Some(1000), 0).await;
	let keys = insert_keys_for_existing_tenant(&pool, tenant_id).await;
	let list_id = seed_source_list(&pool, tenant_id).await;
	let pipeline_id: i64 = sqlx::query_scalar(
		r#"
		INSERT INTO v1_pipelines (
			tenant_id, name, status, source_type, source_config, schedule_cron,
			schedule_timezone, verification_settings, delivery_config, next_run_at
		)
		VALUES (
			$1, 'Pause Resume', 'active'::pipeline_status, 'list_snapshot'::pipeline_source_type,
			$2, '0 * * * *', 'UTC', '{}'::jsonb, '{}'::jsonb, NOW() + INTERVAL '1 hour'
		)
		RETURNING id
		"#,
	)
	.bind(tenant_id)
	.bind(serde_json::json!({"type": "list_snapshot", "list_id": list_id}))
	.fetch_one(&pool)
	.await
	.expect("insert pipeline for pause/resume");
	pool.close().await;

	let config = build_test_config(
		ConfigProfile::PipelineEnabled,
		Some(env.postgres.db_url()),
		Some(env.rabbitmq.amqp_url()),
	)
	.await;

	let pause_barrier = Arc::new(Barrier::new(6));
	let pause_requests = (0..6).map(|_| {
		let barrier = Arc::clone(&pause_barrier);
		let config = Arc::clone(&config);
		let api_key = keys.pipelines_key.clone();
		async move {
			barrier.wait().await;
			request()
				.method("POST")
				.path(&format!("/v1/pipelines/{}/pause", pipeline_id))
				.header("Authorization", format!("Bearer {}", api_key))
				.reply(&create_routes(config))
				.await
		}
	});
	let pause_responses = join_all(pause_requests).await;
	assert!(pause_responses
		.iter()
		.all(|response| response.status() == StatusCode::OK));

	let pool = env.postgres.pool().await;
	let paused_status: String = sqlx::query("SELECT status::TEXT FROM v1_pipelines WHERE id = $1")
		.bind(pipeline_id)
		.fetch_one(&pool)
		.await
		.expect("fetch paused pipeline status")
		.get(0);
	assert_eq!(paused_status, "paused");
	pool.close().await;

	let resume_barrier = Arc::new(Barrier::new(6));
	let resume_requests = (0..6).map(|_| {
		let barrier = Arc::clone(&resume_barrier);
		let config = Arc::clone(&config);
		let api_key = keys.pipelines_key.clone();
		async move {
			barrier.wait().await;
			request()
				.method("POST")
				.path(&format!("/v1/pipelines/{}/resume", pipeline_id))
				.header("Authorization", format!("Bearer {}", api_key))
				.reply(&create_routes(config))
				.await
		}
	});
	let resume_responses = join_all(resume_requests).await;
	assert!(resume_responses
		.iter()
		.all(|response| response.status() == StatusCode::OK));

	let pool = env.postgres.pool().await;
	let resumed_status: String =
		sqlx::query("SELECT status::TEXT FROM v1_pipelines WHERE id = $1")
			.bind(pipeline_id)
			.fetch_one(&pool)
			.await
			.expect("fetch resumed pipeline status")
			.get(0);
	assert_eq!(resumed_status, "active");
	pool.close().await;
}

#[tokio::test]
#[serial]
async fn parallel_comment_creation_and_delete_preserve_row_integrity() {
	let env = OwnedResilienceEnv::start().await;
	let pool = env.postgres.pool().await;
	let tenant_id = insert_tenant(&pool, "comments-concurrency", Some(1000), 0).await;
	let keys = insert_keys_for_existing_tenant(&pool, tenant_id).await;
	let job_id = insert_job(&pool, Some(tenant_id), 1, "completed").await;
	pool.close().await;

	let config = build_test_config(
		ConfigProfile::PseudoWorker,
		Some(env.postgres.db_url()),
		None,
	)
	.await;
	let create_barrier = Arc::new(Barrier::new(10));
	let create_requests = (0..10).map(|index| {
		let barrier = Arc::clone(&create_barrier);
		let config = Arc::clone(&config);
		let api_key = keys.full_access_key.clone();
		async move {
			barrier.wait().await;
			request()
				.method("POST")
				.path("/v1/comments")
				.header("Authorization", format!("Bearer {}", api_key))
				.json(&serde_json::json!({
					"job_id": job_id,
					"body": format!("comment-{}", index),
					"author": "nightly"
				}))
				.reply(&create_routes(config))
				.await
		}
	});
	let create_responses = join_all(create_requests).await;
	assert!(create_responses
		.iter()
		.all(|response| response.status() == StatusCode::CREATED));

	let pool = env.postgres.pool().await;
	let rows = sqlx::query("SELECT id FROM job_comments WHERE job_id = $1")
		.bind(job_id)
		.fetch_all(&pool)
		.await
		.expect("fetch comments");
	let ids: HashSet<i64> = rows.iter().map(|row| row.get("id")).collect();
	assert_eq!(rows.len(), 10);
	assert_eq!(ids.len(), 10);

	let delete_target = insert_comment(&pool, tenant_id, Some(job_id), None, "delete-once").await;
	pool.close().await;

	let delete_barrier = Arc::new(Barrier::new(8));
	let delete_requests = (0..8).map(|_| {
		let barrier = Arc::clone(&delete_barrier);
		let config = Arc::clone(&config);
		let api_key = keys.full_access_key.clone();
		async move {
			barrier.wait().await;
			request()
				.method("DELETE")
				.path(&format!("/v1/comments/{}", delete_target))
				.header("Authorization", format!("Bearer {}", api_key))
				.reply(&create_routes(config))
				.await
		}
	});
	let delete_responses = join_all(delete_requests).await;
	let success_count = delete_responses
		.iter()
		.filter(|response| response.status() == StatusCode::OK)
		.count();
	assert_eq!(success_count, 1);
	assert!(delete_responses.iter().all(|response| {
		matches!(response.status(), StatusCode::OK | StatusCode::NOT_FOUND)
	}));

	let pool = env.postgres.pool().await;
	let remaining: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM job_comments WHERE id = $1")
		.bind(delete_target)
		.fetch_one(&pool)
		.await
		.expect("count deleted comment");
	assert_eq!(remaining, 0);
	pool.close().await;
}

#[tokio::test]
#[serial]
async fn parallel_quota_resets_leave_usage_at_zero() {
	let env = OwnedResilienceEnv::start().await;
	let pool = env.postgres.pool().await;
	let tenant_id = insert_tenant(&pool, "quota-reset", Some(1000), 77).await;
	pool.close().await;

	let config = build_test_config(
		ConfigProfile::AdminSecret,
		Some(env.postgres.db_url()),
		None,
	)
	.await;
	let barrier = Arc::new(Barrier::new(8));
	let requests = (0..8).map(|_| {
		let barrier = Arc::clone(&barrier);
		let config = Arc::clone(&config);
		async move {
			barrier.wait().await;
			request()
				.method("POST")
				.path(&format!("/v1/admin/tenants/{}/quota/reset", tenant_id))
				.header(REACHER_SECRET_HEADER, ADMIN_SECRET)
				.reply(&create_routes(config))
				.await
		}
	});
	let responses = join_all(requests).await;
	assert!(responses
		.iter()
		.all(|response| response.status() == StatusCode::OK));

	let pool = env.postgres.pool().await;
	let row = sqlx::query(
		"SELECT used_this_period, monthly_email_limit FROM tenants WHERE id = $1",
	)
	.bind(tenant_id)
	.fetch_one(&pool)
	.await
	.expect("fetch tenant quota");
	let used: i32 = row.get("used_this_period");
	let limit: Option<i32> = row.get("monthly_email_limit");
	assert_eq!(used, 0);
	assert_eq!(limit, Some(1000));
	pool.close().await;
}
