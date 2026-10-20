mod test_helpers;

use reacher_backend::{
	http::{create_routes, idempotency::cleanup_idempotency_keys},
	worker::{
		dispatch::execute_local,
		do_work::{CheckEmailTask, TaskWebhook, Webhook},
	},
};
use serial_test::serial;
use std::{
	sync::Arc,
	time::{Duration, Instant},
};
use test_helpers::{build_test_config, insert_api_key, insert_tenant, ConfigProfile, TestDb};
use uuid::Uuid;

#[tokio::test]
#[serial]
async fn unavailable_smtp_capability_does_not_reserve_quota_or_dispatch() {
	let db = TestDb::start().await;
	let tenant = insert_tenant(db.pool(), "blocked-smtp", Some(100), 0).await;
	let (key, _) = insert_api_key(db.pool(), tenant).await;
	let config = build_test_config(ConfigProfile::PseudoWorker, Some(db.db_url()), None).await;
	let mut config = Arc::try_unwrap(config).ok().expect("unshared config");
	config.smtp_health_enabled = true;
	let routes = create_routes(Arc::new(config));
	for (path, body) in [
		(
			"/v1/check_email",
			serde_json::json!({"to_email":"user@example.com"}),
		),
		(
			"/v1/bulk",
			serde_json::json!({"input":["user@example.com"]}),
		),
		("/v1/pipelines/1/trigger", serde_json::json!({})),
		(
			"/v1/pipelines/1/push",
			serde_json::json!({"rows":[{"email":"user@example.com"}]}),
		),
	] {
		let response = warp::test::request()
			.method("POST")
			.path(path)
			.header("authorization", format!("Bearer {key}"))
			.json(&body)
			.reply(&routes)
			.await;
		assert_eq!(response.status(), 503, "{path}: {:?}", response.body());
	}
	let counts: (i32, i64, i64) = sqlx::query_as("SELECT used_this_period, (SELECT COUNT(*) FROM v1_task_result), (SELECT COUNT(*) FROM verification_dispatch) FROM tenants WHERE id = $1")
		.bind(tenant).fetch_one(db.pool()).await.unwrap();
	assert_eq!(counts, (0, 0, 0));
}

#[tokio::test]
#[serial]
async fn timed_out_wait_and_expired_key_retrieve_one_committed_operation() {
	let db = TestDb::start().await;
	let tenant = insert_tenant(db.pool(), "durable-single", Some(100), 0).await;
	let other = insert_tenant(db.pool(), "other-single", Some(100), 0).await;
	let (key, _) = insert_api_key(db.pool(), tenant).await;
	let (other_key, _) = insert_api_key(db.pool(), other).await;
	let config = build_test_config(ConfigProfile::PseudoWorker, Some(db.db_url()), None).await;
	let routes = create_routes(Arc::clone(&config));
	let started = Instant::now();
	let response = warp::test::request()
		.method("POST")
		.path("/v1/check_email")
		.header("authorization", format!("Bearer {key}"))
		.header("idempotency-key", "one-check")
		.json(&serde_json::json!({"to_email":"invalid-syntax"}))
		.reply(&routes)
		.await;
	assert_eq!(response.status(), 202, "{:?}", response.body());
	assert!(started.elapsed() >= Duration::from_secs(19));
	let pending: serde_json::Value = serde_json::from_slice(response.body()).unwrap();
	let url = pending["status_url"].as_str().unwrap();
	let id: Uuid = pending["operation_id"].as_str().unwrap().parse().unwrap();

	sqlx::query("UPDATE idempotency_keys SET expires_at = NOW() - INTERVAL '2 days', locked_at = NOW() - INTERVAL '2 days'").execute(db.pool()).await.unwrap();
	cleanup_idempotency_keys(db.pool()).await;
	for _ in 0..2 {
		let replay = warp::test::request()
			.method("POST")
			.path("/v1/check_email")
			.header("authorization", format!("Bearer {key}"))
			.header("idempotency-key", "one-check")
			.json(&serde_json::json!({"to_email":"invalid-syntax"}))
			.reply(&routes)
			.await;
		assert_eq!(replay.status(), 202);
		assert_eq!(replay.body(), response.body());
	}
	let foreign = warp::test::request()
		.method("GET")
		.path(url)
		.header("authorization", format!("Bearer {other_key}"))
		.reply(&routes)
		.await;
	assert_eq!(foreign.status(), 404);
	let counts: (i32, i64, i64) = sqlx::query_as("SELECT used_this_period, (SELECT COUNT(*) FROM v1_task_result), (SELECT COUNT(*) FROM verification_dispatch) FROM tenants WHERE id = $1")
		.bind(tenant).fetch_one(db.pool()).await.unwrap();
	assert_eq!(counts, (1, 1, 1));

	let dispatch: Uuid =
		sqlx::query_scalar("SELECT dispatch_id FROM v1_task_result WHERE operation_id = $1")
			.bind(id)
			.fetch_one(db.pool())
			.await
			.unwrap();
	// A fresh executor finishes after the original HTTP connection has gone.
	let restarted = build_test_config(ConfigProfile::BearerTenant, Some(db.db_url()), None).await;
	execute_local(db.pool(), restarted, dispatch).await.unwrap();
	cleanup_idempotency_keys(db.pool()).await;
	let result = warp::test::request()
		.method("GET")
		.path(url)
		.header("authorization", format!("Bearer {key}"))
		.reply(&routes)
		.await;
	assert_eq!(result.status(), 200, "{:?}", result.body());
	let replay = warp::test::request()
		.method("POST")
		.path("/v1/check_email")
		.header("authorization", format!("Bearer {key}"))
		.header("idempotency-key", "one-check")
		.json(&serde_json::json!({"to_email":"invalid-syntax"}))
		.reply(&routes)
		.await;
	assert_eq!(replay.status(), 200);
	assert_eq!(replay.body(), result.body());
	let used: i32 = sqlx::query_scalar("SELECT used_this_period FROM tenants WHERE id = $1")
		.bind(tenant)
		.fetch_one(db.pool())
		.await
		.unwrap();
	assert_eq!(used, 1);
}

#[tokio::test]
#[serial]
async fn admission_failure_rolls_back_and_disconnected_http_never_reopens_work() {
	let db = TestDb::start().await;
	let tenant = insert_tenant(db.pool(), "rollback-single", Some(100), 0).await;
	let (key, _) = insert_api_key(db.pool(), tenant).await;
	let config = build_test_config(ConfigProfile::PseudoWorker, Some(db.db_url()), None).await;
	let routes = create_routes(Arc::clone(&config));
	sqlx::raw_sql("CREATE FUNCTION fail_operation_commit() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN IF NEW.status = 'completed' THEN RAISE EXCEPTION 'operation commit failure'; END IF; RETURN NEW; END $$; CREATE TRIGGER fail_operation BEFORE UPDATE ON idempotency_keys FOR EACH ROW EXECUTE FUNCTION fail_operation_commit()").execute(db.pool()).await.unwrap();
	let response = warp::test::request()
		.method("POST")
		.path("/v1/check_email")
		.header("authorization", format!("Bearer {key}"))
		.header("idempotency-key", "one-failure")
		.json(&serde_json::json!({"to_email":"invalid-syntax"}))
		.reply(&routes)
		.await;
	assert_eq!(response.status(), 500);
	let counts: (i32, i64, i64) = sqlx::query_as("SELECT used_this_period, (SELECT COUNT(*) FROM v1_task_result), (SELECT COUNT(*) FROM verification_dispatch) FROM tenants WHERE id = $1")
		.bind(tenant).fetch_one(db.pool()).await.unwrap();
	assert_eq!(counts, (0, 0, 0));
	sqlx::raw_sql(
		"DROP TRIGGER fail_operation ON idempotency_keys; DROP FUNCTION fail_operation_commit()",
	)
	.execute(db.pool())
	.await
	.unwrap();

	// Cancel the HTTP future after admission, before its synchronous wait expires.
	let disconnected = tokio::time::timeout(
		Duration::from_secs(1),
		warp::test::request()
			.method("POST")
			.path("/v1/check_email")
			.header("authorization", format!("Bearer {key}"))
			.header("idempotency-key", "one-failure")
			.json(&serde_json::json!({"to_email":"invalid-syntax"}))
			.reply(&routes),
	)
	.await;
	assert!(disconnected.is_err());
	let replay = warp::test::request()
		.method("POST")
		.path("/v1/check_email")
		.header("authorization", format!("Bearer {key}"))
		.header("idempotency-key", "one-failure")
		.json(&serde_json::json!({"to_email":"invalid-syntax"}))
		.reply(&routes)
		.await;
	assert_eq!(replay.status(), 202);
	let pending: serde_json::Value = serde_json::from_slice(replay.body()).unwrap();
	let (dispatch, payload): (Uuid, serde_json::Value) =
		sqlx::query_as("SELECT id, payload FROM verification_dispatch")
			.fetch_one(db.pool())
			.await
			.unwrap();
	let mut task: CheckEmailTask = serde_json::from_value(payload).unwrap();
	task.webhook = Some(TaskWebhook {
		on_each_email: Some(Webhook {
			url: "http://example.com/invalid-hook".into(),
			headers: Default::default(),
			extra: None,
		}),
	});
	sqlx::query("UPDATE verification_dispatch SET payload = $1 WHERE id = $2")
		.bind(serde_json::to_value(&task).unwrap())
		.bind(dispatch)
		.execute(db.pool())
		.await
		.unwrap();
	execute_local(db.pool(), Arc::clone(&config), dispatch)
		.await
		.unwrap();
	execute_local(db.pool(), config, dispatch).await.unwrap();
	let failed = warp::test::request()
		.method("POST")
		.path("/v1/check_email")
		.header("authorization", format!("Bearer {key}"))
		.header("idempotency-key", "one-failure")
		.json(&serde_json::json!({"to_email":"invalid-syntax"}))
		.reply(&routes)
		.await;
	assert_eq!(failed.status(), 500);
	let failed_body: serde_json::Value = serde_json::from_slice(failed.body()).unwrap();
	assert_eq!(failed_body["operation_id"], pending["operation_id"]);
	let counts: (i32, i64, i64) = sqlx::query_as("SELECT used_this_period, (SELECT COUNT(*) FROM v1_task_result), (SELECT COUNT(*) FROM verification_dispatch) FROM tenants WHERE id = $1")
		.bind(tenant).fetch_one(db.pool()).await.unwrap();
	assert_eq!(
		counts,
		(0, 1, 0),
		"one failed operation, one refund, no redispatch"
	);
}

#[tokio::test]
#[serial]
async fn direct_execution_replays_a_saved_result() {
	let db = TestDb::start().await;
	let tenant = insert_tenant(db.pool(), "direct-single", Some(100), 0).await;
	let (key, _) = insert_api_key(db.pool(), tenant).await;
	let config = build_test_config(ConfigProfile::BearerTenant, Some(db.db_url()), None).await;
	let routes = create_routes(config);
	let mut first = None;
	for _ in 0..2 {
		let response = warp::test::request()
			.method("POST")
			.path("/v0/check_email")
			.header("authorization", format!("Bearer {key}"))
			.header("idempotency-key", "direct-check")
			.json(&serde_json::json!({"to_email":"invalid-syntax"}))
			.reply(&routes)
			.await;
		assert_eq!(response.status(), 200, "{:?}", response.body());
		if let Some(first) = &first {
			assert_eq!(first, response.body());
		} else {
			first = Some(response.body().clone());
		}
	}
	let counts: (i32, i64) = sqlx::query_as(
		"SELECT used_this_period, (SELECT COUNT(*) FROM v1_task_result) FROM tenants WHERE id = $1",
	)
	.bind(tenant)
	.fetch_one(db.pool())
	.await
	.unwrap();
	assert_eq!(counts, (1, 1));
}
