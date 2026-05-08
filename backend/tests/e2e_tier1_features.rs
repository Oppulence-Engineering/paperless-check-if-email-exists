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
	let db = ensure_test_db_url().await;
	let rmq = ensure_test_amqp_url().await;
	c.storage = Some(StorageConfig::Postgres(PostgresConfig {
		db_url: db,
		read_replica_url: None,
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
		setup_job_with_tasks(db.pool(), tid).await;
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

#[cfg(test)]
mod list_intelligence_api_tests {
	use super::*;

	fn scored_result(email: &str, category: &str, score: i16, safe: bool) -> serde_json::Value {
		serde_json::json!({
			"input": email,
			"is_reachable": if safe { "safe" } else { "invalid" },
			"misc": {"is_disposable": false, "is_role_account": false},
			"smtp": {"is_catch_all": false, "is_deliverable": safe},
			"score": {
				"score": score,
				"category": category,
				"sub_reason": if safe { "deliverable" } else { "invalid_recipient" },
				"safe_to_send": safe,
				"reason_codes": [if safe { "deliverable" } else { "invalid_recipient" }]
			}
		})
	}

	fn original_rows(emails: &[&str]) -> serde_json::Value {
		let rows = emails
			.iter()
			.enumerate()
			.map(|(index, email)| {
				(
					index.to_string(),
					serde_json::json!({
						"email": email,
						"source": format!("row-{index}")
					}),
				)
			})
			.collect::<serde_json::Map<_, _>>();
		serde_json::Value::Object(rows)
	}

	async fn insert_policy(
		pool: &sqlx::PgPool,
		tenant_id: uuid::Uuid,
		name: &str,
		is_default: bool,
		rules: serde_json::Value,
	) -> i64 {
		sqlx::query_scalar(
			"INSERT INTO v1_score_policies (tenant_id, name, is_default, rules) VALUES ($1, $2, $3, $4) RETURNING id",
		)
		.bind(tenant_id)
		.bind(name)
		.bind(is_default)
		.bind(rules)
		.fetch_one(pool)
		.await
		.expect("insert_policy failed")
	}

	async fn insert_segment(
		pool: &sqlx::PgPool,
		tenant_id: uuid::Uuid,
		name: &str,
		filter: serde_json::Value,
	) -> i64 {
		sqlx::query_scalar(
			"INSERT INTO v1_saved_segments (tenant_id, name, scope, filter) VALUES ($1, $2, 'lists', $3) RETURNING id",
		)
		.bind(tenant_id)
		.bind(name)
		.bind(filter)
		.fetch_one(pool)
		.await
		.expect("insert_segment failed")
	}

	async fn set_completed_at(pool: &sqlx::PgPool, task_id: i32, interval: &str) {
		sqlx::query(&format!(
			"UPDATE v1_task_result SET completed_at = NOW() - INTERVAL '{interval}' WHERE id = $1"
		))
		.bind(task_id)
		.execute(pool)
		.await
		.expect("set_completed_at failed");
	}

	async fn insert_list_row(
		pool: &sqlx::PgPool,
		tenant_id: uuid::Uuid,
		job_id: i32,
		list_id: i32,
		row_index: i32,
		email: &str,
		category: &str,
		score: i16,
		safe: bool,
		is_duplicate: bool,
	) -> i32 {
		let reason = if safe {
			"deliverable"
		} else {
			"invalid_recipient"
		};
		insert_scored_task(
			pool,
			job_id,
			Some(tenant_id),
			email,
			Some(serde_json::json!({"list_id": list_id, "row_index": row_index})),
			Some(scored_result(email, category, score, safe)),
			"completed",
			Some(score),
			Some(category),
			Some(reason),
			Some(safe),
			Some(vec![reason.to_string()]),
			Some(&email.to_lowercase()),
			is_duplicate,
		)
		.await
	}

	fn csv_records(body: &[u8]) -> (Vec<String>, Vec<Vec<String>>) {
		let mut reader = csv::Reader::from_reader(body);
		let headers = reader
			.headers()
			.expect("csv headers")
			.iter()
			.map(str::to_string)
			.collect::<Vec<_>>();
		let rows = reader
			.records()
			.map(|record| {
				record
					.expect("csv row")
					.iter()
					.map(str::to_string)
					.collect::<Vec<_>>()
			})
			.collect::<Vec<_>>();
		(headers, rows)
	}

	#[tokio::test]
	#[serial]
	async fn test_score_policy_and_segment_crud() {
		let db = TestDb::start().await;
		let (_, key) = setup_tenant_with_key(db.pool()).await;
		let c = worker_config().await;

		let create_policy = request()
			.path("/v1/score-policies")
			.method("POST")
			.header("Authorization", format!("Bearer {}", key))
			.json(&serde_json::json!({
				"name": "Default policy",
				"is_default": true,
				"rules": {
					"send": {"score_min": 90, "safe_to_send": true},
					"suppress": {"score_max": 30}
				}
			}))
			.reply(&create_routes(Arc::clone(&c)))
			.await;
		assert_eq!(create_policy.status(), StatusCode::CREATED);

		let create_segment = request()
			.path("/v1/segments")
			.method("POST")
			.header("Authorization", format!("Bearer {}", key))
			.json(&serde_json::json!({
				"name": "Newly invalid",
				"filter": {"change_type": "became_invalid", "policy_decision": "suppress"}
			}))
			.reply(&create_routes(c))
			.await;
		assert_eq!(create_segment.status(), StatusCode::CREATED);
	}

	#[tokio::test]
	#[serial]
	async fn test_policy_and_segment_validation_and_default_rotation() {
		let db = TestDb::start().await;
		let (_, key) = setup_tenant_with_key(db.pool()).await;
		let c = worker_config().await;
		let routes = create_routes(c);

		let first = request()
			.path("/v1/score-policies")
			.method("POST")
			.header("Authorization", format!("Bearer {}", key))
			.json(&serde_json::json!({
				"name": "First default",
				"is_default": true,
				"rules": {"send": {"score_min": 85}}
			}))
			.reply(&routes)
			.await;
		assert_eq!(first.status(), StatusCode::CREATED);
		let first_body: serde_json::Value = serde_json::from_slice(first.body()).unwrap();
		let first_policy_id = first_body["id"].as_i64().unwrap();

		let second = request()
			.path("/v1/score-policies")
			.method("POST")
			.header("Authorization", format!("Bearer {}", key))
			.json(&serde_json::json!({
				"name": "Second default",
				"is_default": true,
				"rules": {"suppress": {"score_max": 20}}
			}))
			.reply(&routes)
			.await;
		assert_eq!(second.status(), StatusCode::CREATED);

		let policies = request()
			.path("/v1/score-policies")
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&routes)
			.await;
		assert_eq!(policies.status(), StatusCode::OK);
		let body: serde_json::Value = serde_json::from_slice(policies.body()).unwrap();
		let policies = body["policies"].as_array().unwrap();
		assert_eq!(
			policies.iter().filter(|p| p["is_default"] == true).count(),
			1
		);
		let first_after_rotation = policies
			.iter()
			.find(|p| p["id"].as_i64() == Some(first_policy_id))
			.unwrap();
		assert_eq!(first_after_rotation["is_default"], false);

		let invalid_policy = request()
			.path("/v1/score-policies")
			.method("POST")
			.header("Authorization", format!("Bearer {}", key))
			.json(&serde_json::json!({
				"name": "Invalid policy",
				"rules": {"send": {"score_min": 101}}
			}))
			.reply(&routes)
			.await;
		assert_eq!(invalid_policy.status(), StatusCode::BAD_REQUEST);

		let duplicate_policy = request()
			.path("/v1/score-policies")
			.method("POST")
			.header("Authorization", format!("Bearer {}", key))
			.json(&serde_json::json!({
				"name": "Second default",
				"rules": {}
			}))
			.reply(&routes)
			.await;
		assert_eq!(duplicate_policy.status(), StatusCode::CONFLICT);

		let invalid_scope = request()
			.path("/v1/segments")
			.method("POST")
			.header("Authorization", format!("Bearer {}", key))
			.json(&serde_json::json!({
				"name": "Bad scope",
				"scope": "sql",
				"filter": {}
			}))
			.reply(&routes)
			.await;
		assert_eq!(invalid_scope.status(), StatusCode::BAD_REQUEST);

		let invalid_filter = request()
			.path("/v1/segments")
			.method("POST")
			.header("Authorization", format!("Bearer {}", key))
			.json(&serde_json::json!({
				"name": "Bad filter",
				"filter": {"score_min": 90, "score_max": 10}
			}))
			.reply(&routes)
			.await;
		assert_eq!(invalid_filter.status(), StatusCode::BAD_REQUEST);

		let segment = request()
			.path("/v1/segments")
			.method("POST")
			.header("Authorization", format!("Bearer {}", key))
			.json(&serde_json::json!({
				"name": "Role accounts",
				"scope": "tenant",
				"filter": {"is_role_account": true}
			}))
			.reply(&routes)
			.await;
		assert_eq!(segment.status(), StatusCode::CREATED);
		let segment_body: serde_json::Value = serde_json::from_slice(segment.body()).unwrap();
		let segment_id = segment_body["id"].as_i64().unwrap();

		let patched = request()
			.path(&format!("/v1/segments/{}", segment_id))
			.method("PATCH")
			.header("Authorization", format!("Bearer {}", key))
			.json(&serde_json::json!({
				"filter": {"category": ["valid", "risky"], "score_min": 50}
			}))
			.reply(&routes)
			.await;
		assert_eq!(patched.status(), StatusCode::OK);
		let patched_body: serde_json::Value = serde_json::from_slice(patched.body()).unwrap();
		assert_eq!(patched_body["filter"]["score_min"], 50);

		let deleted = request()
			.path(&format!("/v1/segments/{}", segment_id))
			.method("DELETE")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&routes)
			.await;
		assert_eq!(deleted.status(), StatusCode::OK);

		let missing = request()
			.path(&format!("/v1/segments/{}", segment_id))
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&routes)
			.await;
		assert_eq!(missing.status(), StatusCode::NOT_FOUND);
	}

	#[tokio::test]
	#[serial]
	async fn test_alerts_and_list_diff() {
		let db = TestDb::start().await;
		let (tid, key) = setup_tenant_with_key(db.pool()).await;

		let previous_job = insert_job(db.pool(), Some(tid), 1, "completed").await;
		let previous_task = insert_scored_task(
			db.pool(),
			previous_job,
			Some(tid),
			"user@example.com",
			None,
			Some(scored_result("user@example.com", "valid", 95, true)),
			"completed",
			Some(95),
			Some("valid"),
			Some("deliverable"),
			Some(true),
			Some(vec!["deliverable".to_string()]),
			Some("user@example.com"),
			false,
		)
		.await;
		assert!(previous_task > 0);
		let current_job = insert_job(db.pool(), Some(tid), 1, "completed").await;
		let current_task = insert_scored_task(
			db.pool(),
			current_job,
			Some(tid),
			"user@example.com",
			None,
			Some(scored_result("user@example.com", "invalid", 0, false)),
			"completed",
			Some(0),
			Some("invalid"),
			Some("invalid_recipient"),
			Some(false),
			Some(vec!["invalid_recipient".to_string()]),
			Some("user@example.com"),
			false,
		)
		.await;
		reacher_backend::list_intelligence::record_verification_change_event(
			db.pool(),
			current_task,
		)
		.await
		.unwrap();

		let base_job = insert_job(db.pool(), Some(tid), 2, "completed").await;
		let base_list = insert_list(
			db.pool(),
			tid,
			base_job,
			"Base",
			"completed",
			2,
			&["email"],
			serde_json::json!({
				"0": {"email": "user@example.com"},
				"1": {"email": "removed@example.com"}
			}),
		)
		.await;
		insert_scored_task(
			db.pool(),
			base_job,
			Some(tid),
			"user@example.com",
			Some(serde_json::json!({"list_id": base_list, "row_index": 0})),
			Some(scored_result("user@example.com", "valid", 95, true)),
			"completed",
			Some(95),
			Some("valid"),
			Some("deliverable"),
			Some(true),
			Some(vec!["deliverable".to_string()]),
			Some("user@example.com"),
			false,
		)
		.await;
		insert_scored_task(
			db.pool(),
			base_job,
			Some(tid),
			"removed@example.com",
			Some(serde_json::json!({"list_id": base_list, "row_index": 1})),
			Some(scored_result("removed@example.com", "valid", 90, true)),
			"completed",
			Some(90),
			Some("valid"),
			Some("deliverable"),
			Some(true),
			Some(vec!["deliverable".to_string()]),
			Some("removed@example.com"),
			false,
		)
		.await;

		let compare_job = insert_job(db.pool(), Some(tid), 2, "completed").await;
		let compare_list = insert_list(
			db.pool(),
			tid,
			compare_job,
			"Compare",
			"completed",
			2,
			&["email"],
			serde_json::json!({
				"0": {"email": "user@example.com"},
				"1": {"email": "added@example.com"}
			}),
		)
		.await;
		insert_scored_task(
			db.pool(),
			compare_job,
			Some(tid),
			"user@example.com",
			Some(serde_json::json!({"list_id": compare_list, "row_index": 0})),
			Some(scored_result("user@example.com", "invalid", 0, false)),
			"completed",
			Some(0),
			Some("invalid"),
			Some("invalid_recipient"),
			Some(false),
			Some(vec!["invalid_recipient".to_string()]),
			Some("user@example.com"),
			false,
		)
		.await;
		insert_scored_task(
			db.pool(),
			compare_job,
			Some(tid),
			"added@example.com",
			Some(serde_json::json!({"list_id": compare_list, "row_index": 1})),
			Some(scored_result("added@example.com", "valid", 98, true)),
			"completed",
			Some(98),
			Some("valid"),
			Some("deliverable"),
			Some(true),
			Some(vec!["deliverable".to_string()]),
			Some("added@example.com"),
			false,
		)
		.await;

		let c = worker_config().await;
		let alerts = request()
			.path("/v1/alerts?type=became_invalid")
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&create_routes(Arc::clone(&c)))
			.await;
		assert_eq!(alerts.status(), StatusCode::OK);
		let body: serde_json::Value = serde_json::from_slice(alerts.body()).unwrap();
		assert_eq!(body["total"], 1);

		let diff = request()
			.path(&format!("/v1/lists/{}/diff/{}", base_list, compare_list))
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&create_routes(c))
			.await;
		assert_eq!(diff.status(), StatusCode::OK, "{:?}", diff.body());
		let diff_body: serde_json::Value = serde_json::from_slice(diff.body()).unwrap();
		assert_eq!(diff_body["newly_invalid"]["count"], 1);
		assert_eq!(diff_body["added"]["count"], 1);
		assert_eq!(diff_body["removed"]["count"], 1);
	}

	#[tokio::test]
	#[serial]
	async fn test_email_history_enriched_change_fields_and_tenant_isolation() {
		let db = TestDb::start().await;
		let (tid, key) = setup_tenant_with_key(db.pool()).await;
		let other_tid = insert_tenant(db.pool(), "history-other", None, 0).await;
		let (other_key, _) = insert_api_key(db.pool(), other_tid).await;

		let previous_job = insert_job(db.pool(), Some(tid), 1, "completed").await;
		let previous_list = insert_list(
			db.pool(),
			tid,
			previous_job,
			"Previous history list",
			"completed",
			1,
			&["email", "source"],
			original_rows(&["u.ser@gmail.com"]),
		)
		.await;
		let pipeline_id = insert_pipeline(
			db.pool(),
			tid,
			"History pipeline",
			serde_json::json!({"type": "list_snapshot", "list_id": previous_list}),
		)
		.await;
		let previous_run = insert_pipeline_run(
			db.pool(),
			pipeline_id,
			tid,
			"completed",
			Some(previous_job),
			Some(previous_list),
		)
		.await;
		let previous_task = insert_scored_task(
			db.pool(),
			previous_job,
			Some(tid),
			"u.ser@gmail.com",
			Some(serde_json::json!({
				"list_id": previous_list,
				"row_index": 0,
				"pipeline_run_id": previous_run
			})),
			Some(scored_result("u.ser@gmail.com", "valid", 95, true)),
			"completed",
			Some(95),
			Some("valid"),
			Some("deliverable"),
			Some(true),
			Some(vec!["deliverable".to_string()]),
			Some("user@gmail.com"),
			false,
		)
		.await;
		set_completed_at(db.pool(), previous_task, "2 hours").await;

		let current_job = insert_job(db.pool(), Some(tid), 1, "completed").await;
		let current_list = insert_list(
			db.pool(),
			tid,
			current_job,
			"Current history list",
			"completed",
			1,
			&["email", "source"],
			original_rows(&["user+latest@gmail.com"]),
		)
		.await;
		let current_run = insert_pipeline_run(
			db.pool(),
			pipeline_id,
			tid,
			"completed",
			Some(current_job),
			Some(current_list),
		)
		.await;
		let current_task = insert_scored_task(
			db.pool(),
			current_job,
			Some(tid),
			"user+latest@gmail.com",
			Some(serde_json::json!({
				"list_id": current_list,
				"row_index": 0,
				"pipeline_run_id": current_run
			})),
			Some(scored_result("user+latest@gmail.com", "invalid", 0, false)),
			"completed",
			Some(0),
			Some("invalid"),
			Some("invalid_recipient"),
			Some(false),
			Some(vec!["invalid_recipient".to_string()]),
			Some("user@gmail.com"),
			false,
		)
		.await;
		set_completed_at(db.pool(), current_task, "1 hour").await;
		reacher_backend::list_intelligence::record_verification_change_event(
			db.pool(),
			current_task,
		)
		.await
		.unwrap();

		let other_job = insert_job(db.pool(), Some(other_tid), 1, "completed").await;
		insert_scored_task(
			db.pool(),
			other_job,
			Some(other_tid),
			"user@gmail.com",
			None,
			Some(scored_result("user@gmail.com", "valid", 99, true)),
			"completed",
			Some(99),
			Some("valid"),
			Some("deliverable"),
			Some(true),
			Some(vec!["deliverable".to_string()]),
			Some("user@gmail.com"),
			false,
		)
		.await;

		let c = worker_config().await;
		let routes = create_routes(c);
		let history = request()
			.path("/v1/emails/u.s.e.r+promo@googlemail.com/history?limit=1")
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&routes)
			.await;
		assert_eq!(history.status(), StatusCode::OK);
		let body: serde_json::Value = serde_json::from_slice(history.body()).unwrap();
		assert_eq!(body["total"], 2);
		assert_eq!(body["history"].as_array().unwrap().len(), 1);
		let latest = &body["history"][0];
		assert_eq!(latest["task_id"], current_task);
		assert_eq!(latest["list_id"], current_list);
		assert_eq!(latest["pipeline_run_id"], current_run);
		assert_eq!(latest["previous_task_id"], previous_task);
		assert_eq!(latest["previous_list_id"], previous_list);
		assert_eq!(latest["previous_pipeline_run_id"], previous_run);
		assert_eq!(latest["previous_score"], 95);
		assert_eq!(latest["previous_category"], "valid");
		assert_eq!(latest["change_type"], "became_invalid");

		let other_history = request()
			.path("/v1/emails/user@gmail.com/history")
			.method("GET")
			.header("Authorization", format!("Bearer {}", other_key))
			.reply(&routes)
			.await;
		assert_eq!(other_history.status(), StatusCode::OK);
		let other_body: serde_json::Value = serde_json::from_slice(other_history.body()).unwrap();
		assert_eq!(other_body["total"], 1);
		assert_eq!(other_body["history"][0]["score"], 99);
	}

	#[tokio::test]
	#[serial]
	async fn test_alert_status_updates_no_duplicates_and_tenant_isolation() {
		let db = TestDb::start().await;
		let (tid, key) = setup_tenant_with_key(db.pool()).await;
		let other_tid = insert_tenant(db.pool(), "alerts-other", None, 0).await;
		let (other_key, _) = insert_api_key(db.pool(), other_tid).await;

		let previous_job = insert_job(db.pool(), Some(tid), 1, "completed").await;
		let previous_task = insert_scored_task(
			db.pool(),
			previous_job,
			Some(tid),
			"alert@example.com",
			None,
			Some(scored_result("alert@example.com", "valid", 95, true)),
			"completed",
			Some(95),
			Some("valid"),
			Some("deliverable"),
			Some(true),
			Some(vec!["deliverable".to_string()]),
			Some("alert@example.com"),
			false,
		)
		.await;
		set_completed_at(db.pool(), previous_task, "2 hours").await;

		let current_job = insert_job(db.pool(), Some(tid), 1, "completed").await;
		let current_task = insert_scored_task(
			db.pool(),
			current_job,
			Some(tid),
			"alert@example.com",
			None,
			Some(scored_result("alert@example.com", "invalid", 0, false)),
			"completed",
			Some(0),
			Some("invalid"),
			Some("invalid_recipient"),
			Some(false),
			Some(vec!["invalid_recipient".to_string()]),
			Some("alert@example.com"),
			false,
		)
		.await;
		set_completed_at(db.pool(), current_task, "1 hour").await;
		for _ in 0..2 {
			reacher_backend::list_intelligence::record_verification_change_event(
				db.pool(),
				current_task,
			)
			.await
			.unwrap();
		}

		let c = worker_config().await;
		let routes = create_routes(c);
		let unread = request()
			.path("/v1/alerts?status=unread&type=became_invalid")
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&routes)
			.await;
		assert_eq!(unread.status(), StatusCode::OK);
		let unread_body: serde_json::Value = serde_json::from_slice(unread.body()).unwrap();
		assert_eq!(unread_body["total"], 1);
		assert_eq!(unread_body["alerts"].as_array().unwrap().len(), 1);
		let alert_id = unread_body["alerts"][0]["id"].as_i64().unwrap();

		let bad_filter = request()
			.path("/v1/alerts?status=archived")
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&routes)
			.await;
		assert_eq!(bad_filter.status(), StatusCode::BAD_REQUEST);

		let bad_update = request()
			.path(&format!("/v1/alerts/{}", alert_id))
			.method("PATCH")
			.header("Authorization", format!("Bearer {}", key))
			.json(&serde_json::json!({"status": "unread"}))
			.reply(&routes)
			.await;
		assert_eq!(bad_update.status(), StatusCode::BAD_REQUEST);

		let read = request()
			.path(&format!("/v1/alerts/{}", alert_id))
			.method("PATCH")
			.header("Authorization", format!("Bearer {}", key))
			.json(&serde_json::json!({"status": "read"}))
			.reply(&routes)
			.await;
		assert_eq!(read.status(), StatusCode::OK);
		let read_body: serde_json::Value = serde_json::from_slice(read.body()).unwrap();
		assert_eq!(read_body["status"], "read");

		let unread_after = request()
			.path("/v1/alerts?status=unread")
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&routes)
			.await;
		let unread_after_body: serde_json::Value =
			serde_json::from_slice(unread_after.body()).unwrap();
		assert_eq!(unread_after_body["total"], 0);

		let read_after = request()
			.path("/v1/alerts?status=read")
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&routes)
			.await;
		let read_after_body: serde_json::Value = serde_json::from_slice(read_after.body()).unwrap();
		assert_eq!(read_after_body["total"], 1);

		let other_list = request()
			.path("/v1/alerts")
			.method("GET")
			.header("Authorization", format!("Bearer {}", other_key))
			.reply(&routes)
			.await;
		assert_eq!(other_list.status(), StatusCode::OK);
		let other_body: serde_json::Value = serde_json::from_slice(other_list.body()).unwrap();
		assert_eq!(other_body["total"], 0);

		let other_patch = request()
			.path(&format!("/v1/alerts/{}", alert_id))
			.method("PATCH")
			.header("Authorization", format!("Bearer {}", other_key))
			.json(&serde_json::json!({"status": "dismissed"}))
			.reply(&routes)
			.await;
		assert_eq!(other_patch.status(), StatusCode::NOT_FOUND);
	}

	#[tokio::test]
	#[serial]
	async fn test_list_download_changed_filters_segments_and_policy_labels() {
		let db = TestDb::start().await;
		let (tid, key) = setup_tenant_with_key(db.pool()).await;
		let policy_id = insert_policy(
			db.pool(),
			tid,
			"Export policy",
			true,
			serde_json::json!({
				"send": {"score_min": 80, "safe_to_send": true},
				"review": {"category": ["risky", "unknown"]},
				"suppress": {"category": ["invalid"]}
			}),
		)
		.await;
		let segment_id = insert_segment(
			db.pool(),
			tid,
			"New suppressions",
			serde_json::json!({
				"change_type": "became_invalid",
				"policy_decision": "suppress",
				"reason_codes_any": ["invalid_recipient"]
			}),
		)
		.await;

		let base_job = insert_job(db.pool(), Some(tid), 2, "completed").await;
		let base_list = insert_list(
			db.pool(),
			tid,
			base_job,
			"Base export list",
			"completed",
			2,
			&["email", "source"],
			original_rows(&["change@example.com", "same@example.com"]),
		)
		.await;
		insert_list_row(
			db.pool(),
			tid,
			base_job,
			base_list,
			0,
			"change@example.com",
			"valid",
			95,
			true,
			false,
		)
		.await;
		insert_list_row(
			db.pool(),
			tid,
			base_job,
			base_list,
			1,
			"same@example.com",
			"valid",
			92,
			true,
			false,
		)
		.await;

		let compare_job = insert_job(db.pool(), Some(tid), 3, "completed").await;
		let compare_list = insert_list(
			db.pool(),
			tid,
			compare_job,
			"Compare export list",
			"completed",
			3,
			&["email", "source"],
			original_rows(&[
				"change@example.com",
				"same@example.com",
				"added@example.com",
			]),
		)
		.await;
		sqlx::query("UPDATE v1_lists SET policy_id = $1 WHERE id = $2")
			.bind(policy_id)
			.bind(compare_list)
			.execute(db.pool())
			.await
			.unwrap();
		insert_list_row(
			db.pool(),
			tid,
			compare_job,
			compare_list,
			0,
			"change@example.com",
			"invalid",
			0,
			false,
			false,
		)
		.await;
		insert_list_row(
			db.pool(),
			tid,
			compare_job,
			compare_list,
			1,
			"same@example.com",
			"valid",
			92,
			true,
			false,
		)
		.await;
		insert_list_row(
			db.pool(),
			tid,
			compare_job,
			compare_list,
			2,
			"added@example.com",
			"risky",
			55,
			false,
			false,
		)
		.await;

		let c = worker_config().await;
		let routes = create_routes(c);
		let changed = request()
			.path(&format!(
				"/v1/lists/{}/download?changed_since_list_id={}",
				compare_list, base_list
			))
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&routes)
			.await;
		assert_eq!(changed.status(), StatusCode::OK);
		let (headers, rows) = csv_records(changed.body());
		let email_idx = headers.iter().position(|h| h == "email").unwrap();
		let policy_idx = headers.iter().position(|h| h == "policy_decision").unwrap();
		let change_idx = headers.iter().position(|h| h == "change_type").unwrap();
		assert_eq!(rows.len(), 2);
		assert!(rows.iter().any(|row| {
			row[email_idx] == "change@example.com"
				&& row[policy_idx] == "suppress"
				&& row[change_idx] == "became_invalid"
		}));
		assert!(rows.iter().any(|row| {
			row[email_idx] == "added@example.com"
				&& row[policy_idx] == "review"
				&& row[change_idx] == "new"
		}));

		let newly_invalid = request()
			.path(&format!(
				"/v1/lists/{}/download?changed_since_list_id={}&filter=newly_invalid",
				compare_list, base_list
			))
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&routes)
			.await;
		assert_eq!(newly_invalid.status(), StatusCode::OK);
		let (_, rows) = csv_records(newly_invalid.body());
		assert_eq!(rows.len(), 1);
		assert_eq!(rows[0][email_idx], "change@example.com");

		let segment = request()
			.path(&format!(
				"/v1/lists/{}/download?changed_since_list_id={}&segment_id={}",
				compare_list, base_list, segment_id
			))
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&routes)
			.await;
		assert_eq!(segment.status(), StatusCode::OK);
		let (_, rows) = csv_records(segment.body());
		assert_eq!(rows.len(), 1);
		assert_eq!(rows[0][email_idx], "change@example.com");

		let missing_segment = request()
			.path(&format!(
				"/v1/lists/{}/download?segment_id=999999",
				compare_list
			))
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&routes)
			.await;
		assert_eq!(missing_segment.status(), StatusCode::NOT_FOUND);
	}

	#[tokio::test]
	#[serial]
	async fn test_list_diff_all_change_groups_and_duplicate_primary_rows() {
		let db = TestDb::start().await;
		let (tid, key) = setup_tenant_with_key(db.pool()).await;

		let base_job = insert_job(db.pool(), Some(tid), 7, "completed").await;
		let base_list = insert_list(
			db.pool(),
			tid,
			base_job,
			"Base full diff",
			"completed",
			7,
			&["email", "source"],
			original_rows(&[
				"same@example.com",
				"improved@example.com",
				"degraded@example.com",
				"invalid@example.com",
				"risky@example.com",
				"safe@example.com",
				"removed@example.com",
			]),
		)
		.await;
		for (row_index, email, category, score, safe) in [
			(0, "same@example.com", "valid", 90, true),
			(1, "improved@example.com", "risky", 40, false),
			(2, "degraded@example.com", "valid", 95, true),
			(3, "invalid@example.com", "valid", 90, true),
			(4, "risky@example.com", "valid", 90, true),
			(5, "safe@example.com", "risky", 50, false),
			(6, "removed@example.com", "valid", 88, true),
		] {
			insert_list_row(
				db.pool(),
				tid,
				base_job,
				base_list,
				row_index,
				email,
				category,
				score,
				safe,
				false,
			)
			.await;
		}

		let compare_job = insert_job(db.pool(), Some(tid), 8, "completed").await;
		let compare_list = insert_list(
			db.pool(),
			tid,
			compare_job,
			"Compare full diff",
			"completed",
			8,
			&["email", "source"],
			original_rows(&[
				"same@example.com",
				"improved@example.com",
				"degraded@example.com",
				"invalid@example.com",
				"risky@example.com",
				"safe@example.com",
				"added@example.com",
				"same@example.com",
			]),
		)
		.await;
		for (row_index, email, category, score, safe) in [
			(0, "same@example.com", "valid", 85, true),
			(1, "improved@example.com", "unknown", 70, false),
			(2, "degraded@example.com", "unknown", 70, false),
			(3, "invalid@example.com", "invalid", 0, false),
			(4, "risky@example.com", "risky", 55, false),
			(5, "safe@example.com", "valid", 94, true),
			(6, "added@example.com", "valid", 88, true),
		] {
			insert_list_row(
				db.pool(),
				tid,
				compare_job,
				compare_list,
				row_index,
				email,
				category,
				score,
				safe,
				false,
			)
			.await;
		}
		insert_list_row(
			db.pool(),
			tid,
			compare_job,
			compare_list,
			7,
			"same@example.com",
			"invalid",
			0,
			false,
			true,
		)
		.await;

		let c = worker_config().await;
		let diff = request()
			.path(&format!("/v1/lists/{}/diff/{}", base_list, compare_list))
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&create_routes(c))
			.await;
		assert_eq!(diff.status(), StatusCode::OK, "{:?}", diff.body());
		let body: serde_json::Value = serde_json::from_slice(diff.body()).unwrap();
		for group in [
			"added",
			"removed",
			"unchanged",
			"improved",
			"degraded",
			"newly_invalid",
			"newly_risky",
			"newly_safe",
		] {
			assert_eq!(body[group]["count"], 1, "unexpected count for {group}");
			assert_eq!(body[group]["rows"].as_array().unwrap().len(), 1);
		}
		assert_eq!(body["unchanged"]["rows"][0]["compare_row_index"], 0);
		assert_eq!(
			body["newly_safe"]["rows"][0]["canonical_email"],
			"safe@example.com"
		);
		assert_eq!(body["improved"]["rows"][0]["change_type"], "improved");
		assert_eq!(body["degraded"]["rows"][0]["change_type"], "degraded");
	}

	#[tokio::test]
	#[serial]
	async fn test_list_diff_and_changed_exports_enforce_completion_and_tenant() {
		let db = TestDb::start().await;
		let (tid, key) = setup_tenant_with_key(db.pool()).await;
		let other_tid = insert_tenant(db.pool(), "diff-other", None, 0).await;

		let complete_job = insert_job(db.pool(), Some(tid), 1, "completed").await;
		let complete_list = insert_list(
			db.pool(),
			tid,
			complete_job,
			"Complete list",
			"completed",
			1,
			&["email", "source"],
			original_rows(&["complete@example.com"]),
		)
		.await;
		insert_list_row(
			db.pool(),
			tid,
			complete_job,
			complete_list,
			0,
			"complete@example.com",
			"valid",
			90,
			true,
			false,
		)
		.await;

		let incomplete_job = insert_job(db.pool(), Some(tid), 2, "running").await;
		let incomplete_list = insert_list(
			db.pool(),
			tid,
			incomplete_job,
			"Incomplete list",
			"processing",
			2,
			&["email", "source"],
			original_rows(&["one@example.com", "two@example.com"]),
		)
		.await;
		insert_list_row(
			db.pool(),
			tid,
			incomplete_job,
			incomplete_list,
			0,
			"one@example.com",
			"valid",
			90,
			true,
			false,
		)
		.await;

		let other_job = insert_job(db.pool(), Some(other_tid), 1, "completed").await;
		let other_list = insert_list(
			db.pool(),
			other_tid,
			other_job,
			"Other tenant list",
			"completed",
			1,
			&["email", "source"],
			original_rows(&["other@example.com"]),
		)
		.await;
		insert_list_row(
			db.pool(),
			other_tid,
			other_job,
			other_list,
			0,
			"other@example.com",
			"valid",
			90,
			true,
			false,
		)
		.await;

		let c = worker_config().await;
		let routes = create_routes(c);
		let incomplete_diff = request()
			.path(&format!(
				"/v1/lists/{}/diff/{}",
				complete_list, incomplete_list
			))
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&routes)
			.await;
		assert_eq!(incomplete_diff.status(), StatusCode::BAD_REQUEST);

		let cross_tenant_diff = request()
			.path(&format!("/v1/lists/{}/diff/{}", complete_list, other_list))
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&routes)
			.await;
		assert_eq!(cross_tenant_diff.status(), StatusCode::NOT_FOUND);

		let incomplete_download = request()
			.path(&format!("/v1/lists/{}/download", incomplete_list))
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&routes)
			.await;
		assert_eq!(incomplete_download.status(), StatusCode::BAD_REQUEST);

		let cross_tenant_export = request()
			.path(&format!(
				"/v1/lists/{}/download?changed_since_list_id={}",
				complete_list, other_list
			))
			.method("GET")
			.header("Authorization", format!("Bearer {}", key))
			.reply(&routes)
			.await;
		assert_eq!(cross_tenant_export.status(), StatusCode::NOT_FOUND);
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

// ===== #34: Approval Threshold Policies =====

#[cfg(test)]
mod custom_threshold_tests {
	use super::*;

	#[tokio::test]
	#[serial]
	async fn test_approval_default_threshold_behavior() {
		let db = TestDb::start().await;
		let tid = insert_tenant(db.pool(), "custom-thresh", None, 0).await;
		let (key, _) = insert_api_key(db.pool(), tid).await;
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
		let list_id: i32 = sqlx::query("INSERT INTO v1_lists (tenant_id, job_id, name, original_filename, file_size_bytes, total_rows, email_column, status) VALUES ($1, $2, 'test', 'test.csv', 128, 3, 'email', 'completed'::list_status) RETURNING id")
			.bind(tid)
			.bind(jid)
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
