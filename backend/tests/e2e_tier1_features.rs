mod test_helpers;

use crate::test_helpers::*;
use reacher_backend::config::{
	BackendConfig, PostgresConfig, RabbitMQConfig, StorageConfig, WorkerConfig,
};
use reacher_backend::http::create_routes;
use serial_test::serial;
use sqlx::Row;
use std::sync::Arc;
use warp::http::StatusCode;
use warp::test::request;

async fn worker_config() -> Arc<BackendConfig> {
	let mut c = BackendConfig::empty();
	c.header_secret = Some("s".into());
	let db = test_db_url();
	let rmq = test_amqp_url();
	c.storage = Some(StorageConfig::Postgres(PostgresConfig {
		db_url: db,
		extra: None,
	}));
	c.worker = WorkerConfig {
		enable: true,
		rabbitmq: Some(RabbitMQConfig {
			url: rmq,
			concurrency: 4,
		}),
		webhook: None,
	};
	c.connect().await.unwrap();
	Arc::new(c)
}

async fn setup_tenant_with_key(pool: &sqlx::PgPool) -> (uuid::Uuid, String) {
	let tid = insert_tenant(pool, "tier1-feat", None, 0).await;
	let (key, _) = insert_api_key(pool, tid).await;
	(tid, key)
}

async fn setup_job_with_tasks(pool: &sqlx::PgPool, tid: uuid::Uuid) -> i32 {
	let jid = insert_job(pool, Some(tid), 3, "completed").await;
	// Insert tasks with scores and timing
	for (email, score, category, safe) in [
		("good@example.com", 95i16, "valid", true),
		("risky@example.com", 60, "risky", false),
		("bad@example.com", 0, "invalid", false),
	] {
		let result = serde_json::json!({
			"input": email,
			"is_reachable": if safe { "safe" } else { "invalid" },
			"misc": {"is_disposable": false, "is_role_account": false, "is_b2c": false},
			"smtp": {"can_connect_smtp": true, "has_full_inbox": false, "is_catch_all": false, "is_deliverable": safe, "is_disabled": false},
			"syntax": {"is_valid_syntax": true}
		});
		let payload = serde_json::json!({"input": {"to_email": email}, "job_id": {"bulk": jid}, "webhook": null});
		sqlx::query(
			r#"INSERT INTO v1_task_result (job_id, payload, task_state, tenant_id, result, score, score_category, sub_reason, safe_to_send, reason_codes, started_at, completed_at)
			   VALUES ($1, $2, 'completed'::task_state, $3, $4, $5, $6, 'deliverable', $7, ARRAY['deliverable'], NOW() - INTERVAL '2 seconds', NOW())"#,
		)
		.bind(jid).bind(&payload).bind(tid).bind(&result).bind(score).bind(category).bind(safe)
		.execute(pool).await.unwrap();
	}
	// Insert events
	insert_event(pool, jid, None, "job.created").await;
	insert_event(pool, jid, None, "job.completed").await;
	jid
}

// ===== #70: API Key Scopes =====

#[cfg(test)]
mod scope_tests {
	use super::*;

	#[tokio::test]
	#[serial]
	async fn test_empty_scopes_grants_full_access() {
		let db = TestDb::start().await;
		let (_, key) = setup_tenant_with_key(db.pool()).await;
		let c = worker_config().await;
		// Empty scopes should allow access to any endpoint
		let r = request()
			.path("/v1/me/usage")
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::OK);
	}

	#[tokio::test]
	#[serial]
	async fn test_scoped_key_allowed() {
		let db = TestDb::start().await;
		let tid = insert_tenant(db.pool(), "scope-allow", None, 0).await;
		let jid = insert_job(db.pool(), Some(tid), 1, "completed").await;
		insert_event(db.pool(), jid, None, "job.created").await;
		let (key, _) = insert_api_key_with_scopes(db.pool(), tid, &["bulk"]).await;
		let c = worker_config().await;
		// "bulk" scope should allow GET /v1/events
		let r = request()
			.path("/v1/events")
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::OK);
	}

	#[tokio::test]
	#[serial]
	async fn test_scoped_key_denied() {
		let db = TestDb::start().await;
		let tid = insert_tenant(db.pool(), "scope-deny", None, 0).await;
		let (key, _) = insert_api_key_with_scopes(db.pool(), tid, &["verify"]).await;
		let c = worker_config().await;
		// "verify" scope should NOT allow GET /v1/events (requires "bulk")
		let r = request()
			.path("/v1/events")
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::FORBIDDEN);
	}

	#[tokio::test]
	#[serial]
	async fn test_wildcard_scope_grants_all() {
		let db = TestDb::start().await;
		let tid = insert_tenant(db.pool(), "scope-wild", None, 0).await;
		let (key, _) = insert_api_key_with_scopes(db.pool(), tid, &["*"]).await;
		let c = worker_config().await;
		let r = request()
			.path("/v1/me/usage")
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::OK);
	}
}

// ===== #69: Audit Log Explorer =====

#[cfg(test)]
mod events_tests {
	use super::*;

	#[tokio::test]
	#[serial]
	async fn test_list_events() {
		let db = TestDb::start().await;
		let (tid, key) = setup_tenant_with_key(db.pool()).await;
		let jid = setup_job_with_tasks(db.pool(), tid).await;
		let c = worker_config().await;
		let r = request()
			.path("/v1/events")
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::OK);
		let b: serde_json::Value = serde_json::from_slice(r.body()).unwrap();
		assert!(b["total"].as_i64().unwrap() >= 2);
		assert!(!b["events"].as_array().unwrap().is_empty());
	}

	#[tokio::test]
	#[serial]
	async fn test_events_filter_by_type() {
		let db = TestDb::start().await;
		let (tid, key) = setup_tenant_with_key(db.pool()).await;
		setup_job_with_tasks(db.pool(), tid).await;
		let c = worker_config().await;
		let r = request()
			.path("/v1/events?event_type=job.created")
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::OK);
		let b: serde_json::Value = serde_json::from_slice(r.body()).unwrap();
		for ev in b["events"].as_array().unwrap() {
			assert_eq!(ev["event_type"], "job.created");
		}
	}

	#[tokio::test]
	#[serial]
	async fn test_events_filter_by_job_id() {
		let db = TestDb::start().await;
		let (tid, key) = setup_tenant_with_key(db.pool()).await;
		let jid = setup_job_with_tasks(db.pool(), tid).await;
		let c = worker_config().await;
		let r = request()
			.path(&format!("/v1/events?job_id={}", jid))
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::OK);
		let b: serde_json::Value = serde_json::from_slice(r.body()).unwrap();
		assert!(b["total"].as_i64().unwrap() >= 1);
	}

	#[tokio::test]
	#[serial]
	async fn test_events_pagination() {
		let db = TestDb::start().await;
		let (tid, key) = setup_tenant_with_key(db.pool()).await;
		setup_job_with_tasks(db.pool(), tid).await;
		let c = worker_config().await;
		let r = request()
			.path("/v1/events?limit=1&offset=0")
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::OK);
		let b: serde_json::Value = serde_json::from_slice(r.body()).unwrap();
		assert!(b["events"].as_array().unwrap().len() <= 1);
	}
}

// ===== #28: Historical Verification Timeline =====

#[cfg(test)]
mod email_history_tests {
	use super::*;

	#[tokio::test]
	#[serial]
	async fn test_email_history() {
		let db = TestDb::start().await;
		let (tid, key) = setup_tenant_with_key(db.pool()).await;
		setup_job_with_tasks(db.pool(), tid).await;
		let c = worker_config().await;
		let r = request()
			.path("/v1/emails/good@example.com/history")
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::OK);
		let b: serde_json::Value = serde_json::from_slice(r.body()).unwrap();
		assert_eq!(b["email"], "good@example.com");
		assert!(b["total"].as_i64().unwrap() >= 1);
		let first = &b["history"][0];
		assert_eq!(first["score"], 95);
		assert_eq!(first["category"], "valid");
	}

	#[tokio::test]
	#[serial]
	async fn test_email_history_empty() {
		let db = TestDb::start().await;
		let (_, key) = setup_tenant_with_key(db.pool()).await;
		let c = worker_config().await;
		let r = request()
			.path("/v1/emails/nonexistent@nowhere.com/history")
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::OK);
		let b: serde_json::Value = serde_json::from_slice(r.body()).unwrap();
		assert_eq!(b["total"], 0);
		assert!(b["history"].as_array().unwrap().is_empty());
	}
}

// ===== #91: Latency Analytics =====

#[cfg(test)]
mod latency_tests {
	use super::*;

	#[tokio::test]
	#[serial]
	async fn test_job_latency() {
		let db = TestDb::start().await;
		let (tid, key) = setup_tenant_with_key(db.pool()).await;
		let jid = setup_job_with_tasks(db.pool(), tid).await;
		let c = worker_config().await;
		let r = request()
			.path(&format!("/v1/jobs/{}/latency", jid))
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::OK);
		let b: serde_json::Value = serde_json::from_slice(r.body()).unwrap();
		assert_eq!(b["job_id"], jid);
		assert!(b["total_completed"].as_i64().unwrap() >= 1);
		assert!(b["avg_duration_ms"].as_f64().unwrap() >= 0.0);
		assert!(b["p50_duration_ms"].as_f64().unwrap() >= 0.0);
		assert!(b["p95_duration_ms"].as_f64().unwrap() >= 0.0);
	}

	#[tokio::test]
	#[serial]
	async fn test_latency_not_found() {
		let db = TestDb::start().await;
		let (_, key) = setup_tenant_with_key(db.pool()).await;
		let c = worker_config().await;
		let r = request()
			.path("/v1/jobs/999999/latency")
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::NOT_FOUND);
	}
}

// ===== #86: Historical Query API =====

#[cfg(test)]
mod query_tests {
	use super::*;

	#[tokio::test]
	#[serial]
	async fn test_query_all() {
		let db = TestDb::start().await;
		let (tid, key) = setup_tenant_with_key(db.pool()).await;
		setup_job_with_tasks(db.pool(), tid).await;
		let c = worker_config().await;
		let r = request()
			.path("/v1/query")
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::OK);
		let b: serde_json::Value = serde_json::from_slice(r.body()).unwrap();
		assert!(b["total"].as_i64().unwrap() >= 3);
	}

	#[tokio::test]
	#[serial]
	async fn test_query_by_category() {
		let db = TestDb::start().await;
		let (tid, key) = setup_tenant_with_key(db.pool()).await;
		setup_job_with_tasks(db.pool(), tid).await;
		let c = worker_config().await;
		let r = request()
			.path("/v1/query?category=valid")
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::OK);
		let b: serde_json::Value = serde_json::from_slice(r.body()).unwrap();
		assert_eq!(b["total"].as_i64().unwrap(), 1);
		assert_eq!(b["results"][0]["category"], "valid");
	}

	#[tokio::test]
	#[serial]
	async fn test_query_by_score_range() {
		let db = TestDb::start().await;
		let (tid, key) = setup_tenant_with_key(db.pool()).await;
		setup_job_with_tasks(db.pool(), tid).await;
		let c = worker_config().await;
		let r = request()
			.path("/v1/query?min_score=50&max_score=100")
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::OK);
		let b: serde_json::Value = serde_json::from_slice(r.body()).unwrap();
		// good (95) + risky (60) = 2
		assert_eq!(b["total"].as_i64().unwrap(), 2);
	}

	#[tokio::test]
	#[serial]
	async fn test_query_safe_to_send() {
		let db = TestDb::start().await;
		let (tid, key) = setup_tenant_with_key(db.pool()).await;
		setup_job_with_tasks(db.pool(), tid).await;
		let c = worker_config().await;
		let r = request()
			.path("/v1/query?safe_to_send=true")
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::OK);
		let b: serde_json::Value = serde_json::from_slice(r.body()).unwrap();
		assert_eq!(b["total"].as_i64().unwrap(), 1);
	}
}

// ===== #88: Comments =====

#[cfg(test)]
mod comments_tests {
	use super::*;

	#[tokio::test]
	#[serial]
	async fn test_comments_crud() {
		let db = TestDb::start().await;
		let (tid, key) = setup_tenant_with_key(db.pool()).await;
		let jid = insert_job(db.pool(), Some(tid), 1, "completed").await;
		let c = worker_config().await;
		let routes = create_routes(Arc::clone(&c));

		// Create
		let r = request()
			.path("/v1/comments")
			.method("POST")
			.header("Authorization", format!("Bearer {}", key))
			.json(&serde_json::json!({"job_id": jid, "body": "Looks good!", "author": "tester"}))
			.reply(&routes)
			.await;
		assert_eq!(r.status(), StatusCode::CREATED);
		let b: serde_json::Value = serde_json::from_slice(r.body()).unwrap();
		assert_eq!(b["body"], "Looks good!");
		assert_eq!(b["author"], "tester");
		let comment_id = b["id"].as_i64().unwrap();

		// List
		let r = request()
			.path(&format!("/v1/comments?job_id={}", jid))
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&routes)
			.await;
		assert_eq!(r.status(), StatusCode::OK);
		let b: serde_json::Value = serde_json::from_slice(r.body()).unwrap();
		assert_eq!(b["total"].as_i64().unwrap(), 1);

		// Delete
		let r = request()
			.path(&format!("/v1/comments/{}", comment_id))
			.method("DELETE")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&routes)
			.await;
		assert_eq!(r.status(), StatusCode::OK);
		let b: serde_json::Value = serde_json::from_slice(r.body()).unwrap();
		assert_eq!(b["deleted"], true);
	}

	#[tokio::test]
	#[serial]
	async fn test_comment_requires_body() {
		let db = TestDb::start().await;
		let (tid, key) = setup_tenant_with_key(db.pool()).await;
		let jid = insert_job(db.pool(), Some(tid), 1, "completed").await;
		let c = worker_config().await;
		let r = request()
			.path("/v1/comments")
			.method("POST")
			.header("Authorization", format!("Bearer {}", key))
			.json(&serde_json::json!({"job_id": jid, "body": "  "}))
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::BAD_REQUEST);
	}

	#[tokio::test]
	#[serial]
	async fn test_comment_requires_target() {
		let db = TestDb::start().await;
		let (_, key) = setup_tenant_with_key(db.pool()).await;
		let c = worker_config().await;
		let r = request()
			.path("/v1/comments")
			.method("POST")
			.header("Authorization", format!("Bearer {}", key))
			.json(&serde_json::json!({"body": "No target"}))
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::BAD_REQUEST);
	}

	#[tokio::test]
	#[serial]
	async fn test_delete_nonexistent_comment() {
		let db = TestDb::start().await;
		let (_, key) = setup_tenant_with_key(db.pool()).await;
		let c = worker_config().await;
		let r = request()
			.path("/v1/comments/999999")
			.method("DELETE")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::NOT_FOUND);
	}
}

// ===== #34: Custom Threshold Policies =====

#[cfg(test)]
mod custom_threshold_tests {
	use super::*;

	#[tokio::test]
	#[serial]
	async fn test_approval_with_custom_thresholds() {
		let db = TestDb::start().await;
		let tid = insert_tenant(db.pool(), "custom-thresh", None, 0).await;
		let (key, _) = insert_api_key(db.pool(), tid).await;
		// Set custom thresholds: excellent=80, good=60
		sqlx::query("UPDATE tenants SET settings = $1 WHERE id = $2")
			.bind(
				serde_json::json!({"approval_excellent_threshold": 80, "approval_good_threshold": 60}),
			)
			.bind(tid)
			.execute(db.pool())
			.await
			.unwrap();
		let jid = setup_job_with_tasks(db.pool(), tid).await;
		let c = worker_config().await;
		let r = request()
			.path(&format!("/v1/jobs/{}/approval", jid))
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::OK);
		let b: serde_json::Value = serde_json::from_slice(r.body()).unwrap();
		// With 1/3 safe_to_send (~33%), quality should be "poor" regardless of thresholds
		assert!(!b["ready_to_send"].as_bool().unwrap());
	}

	#[tokio::test]
	#[serial]
	async fn test_approval_default_thresholds() {
		let db = TestDb::start().await;
		let (tid, key) = setup_tenant_with_key(db.pool()).await;
		let jid = setup_job_with_tasks(db.pool(), tid).await;
		let c = worker_config().await;
		let r = request()
			.path(&format!("/v1/jobs/{}/approval", jid))
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::OK);
		let b: serde_json::Value = serde_json::from_slice(r.body()).unwrap();
		assert_eq!(b["job_id"], jid);
		assert!(b.get("recommendation").is_some());
	}
}

// ===== #95: List Quality Benchmark =====

#[cfg(test)]
mod list_quality_tests {
	use super::*;

	#[tokio::test]
	#[serial]
	async fn test_list_quality() {
		let db = TestDb::start().await;
		let tid = insert_tenant(db.pool(), "list-qual", None, 0).await;
		let (key, _) = insert_api_key(db.pool(), tid).await;
		let jid = insert_job(db.pool(), Some(tid), 3, "completed").await;
		// Create list
		let list_id: i32 = sqlx::query("INSERT INTO v1_lists (tenant_id, job_id, name, original_filename, total_rows, email_column, status) VALUES ($1, $2, 'test', 'test.csv', 3, 'email', 'completed'::list_status) RETURNING id")
			.bind(tid).bind(jid)
			.fetch_one(db.pool()).await.unwrap().get("id");
		// Insert tasks linked to list via extra
		for (email, score, category, safe) in [
			("a@t.com", 95i16, "valid", true),
			("b@t.com", 60, "risky", false),
			("c@t.com", 0, "invalid", false),
		] {
			let result = serde_json::json!({"input": email, "is_reachable": "safe", "misc": {"is_disposable": false, "is_role_account": false}, "smtp": {"is_catch_all": false, "has_full_inbox": false}});
			let payload =
				serde_json::json!({"input": {"to_email": email}, "job_id": {"bulk": jid}});
			let extra = serde_json::json!({"list_id": list_id});
			sqlx::query(
				"INSERT INTO v1_task_result (job_id, payload, extra, task_state, tenant_id, result, score, score_category, safe_to_send) VALUES ($1, $2, $3, 'completed'::task_state, $4, $5, $6, $7, $8)",
			)
			.bind(jid).bind(&payload).bind(&extra).bind(tid).bind(&result).bind(score).bind(category).bind(safe)
			.execute(db.pool()).await.unwrap();
		}
		let c = worker_config().await;
		let r = request()
			.path(&format!("/v1/lists/{}/quality", list_id))
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::OK);
		let b: serde_json::Value = serde_json::from_slice(r.body()).unwrap();
		assert_eq!(b["list_id"], list_id);
		assert_eq!(b["processed"], 3);
		assert!(b["avg_score"].as_f64().unwrap() > 0.0);
		assert_eq!(b["categories"]["valid"], 1);
		assert_eq!(b["categories"]["risky"], 1);
		assert_eq!(b["categories"]["invalid"], 1);
		assert_eq!(b["safe_to_send_count"], 1);
		// 1/3 safe = ~33%, should be grade "D" or "F"
		let grade = b["quality_grade"].as_str().unwrap();
		assert!(grade == "D" || grade == "F", "got grade: {}", grade);
	}

	#[tokio::test]
	#[serial]
	async fn test_list_quality_not_found() {
		let db = TestDb::start().await;
		let (_, key) = setup_tenant_with_key(db.pool()).await;
		let c = worker_config().await;
		let r = request()
			.path("/v1/lists/999999/quality")
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::NOT_FOUND);
	}
}

// ===== #51: Alternative Contact Suggestions =====

#[cfg(test)]
mod alternatives_tests {
	use super::*;

	#[tokio::test]
	#[serial]
	async fn test_finder_includes_alternatives() {
		let db = TestDb::start().await;
		let tid = insert_tenant(db.pool(), "finder-alt", None, 0).await;
		let (key, _) = insert_api_key(db.pool(), tid).await;
		let jid = insert_job(db.pool(), Some(tid), 3, "completed").await;
		// Create finder job
		let fid: i32 = sqlx::query(
			"INSERT INTO v1_finder_job (tenant_id, bulk_job_id, first_name, last_name, domain, normalized_first_name, normalized_last_name, status, domain_has_mx, domain_is_catch_all, candidates_checked) VALUES ($1, $2, 'John', 'Doe', 'example.com', 'john', 'doe', 'completed'::job_state, true, false, 3) RETURNING id",
		)
		.bind(tid).bind(jid)
		.fetch_one(db.pool()).await.unwrap().get("id");
		// Create task results + finder results
		for (email, pattern, score) in [
			("john.doe@example.com", "first.last", 95i16),
			("jdoe@example.com", "flast", 85),
			("johndoe@example.com", "firstlast", 70),
		] {
			let result = serde_json::json!({"input": email, "is_reachable": "safe", "score": {"score": score, "category": "valid"}});
			let payload =
				serde_json::json!({"input": {"to_email": email}, "job_id": {"bulk": jid}});
			let task_id: i32 = sqlx::query(
				"INSERT INTO v1_task_result (job_id, payload, task_state, tenant_id, result, score, score_category, sub_reason) VALUES ($1, $2, 'completed'::task_state, $3, $4, $5, 'valid', 'deliverable') RETURNING id",
			)
			.bind(jid).bind(&payload).bind(tid).bind(&result).bind(score)
			.fetch_one(db.pool()).await.unwrap().get("id");
			sqlx::query(
				"INSERT INTO v1_finder_result (finder_job_id, task_result_id, candidate_email, pattern) VALUES ($1, $2, $3, $4)",
			)
			.bind(fid).bind(task_id).bind(email).bind(pattern)
			.execute(db.pool()).await.unwrap();
		}
		let c = worker_config().await;
		let r = request()
			.path(&format!("/v1/find_email/{}", fid))
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&create_routes(c))
			.await;
		assert_eq!(r.status(), StatusCode::OK);
		let b: serde_json::Value = serde_json::from_slice(r.body()).unwrap();
		assert!(
			b.get("alternatives").is_some(),
			"response missing 'alternatives' field"
		);
		let alts = b["alternatives"].as_array().unwrap();
		// Best match is john.doe (95), alternatives are jdoe (85) and johndoe (70)
		assert_eq!(alts.len(), 2);
		assert_eq!(alts[0]["email"], "jdoe@example.com");
	}
}
