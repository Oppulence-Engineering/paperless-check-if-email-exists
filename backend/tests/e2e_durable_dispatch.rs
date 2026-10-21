#[path = "../src/main.rs"]
#[allow(dead_code)]
mod retention;
mod test_helpers;

use reacher_backend::{
	http::create_routes,
	worker::{
		dispatch::{enqueue, publish_next, recover_pending_completion},
		do_work::CheckEmailTask,
		lifecycle::{claim_task, finish_stored_task, Claim},
	},
};
use serial_test::serial;
use std::sync::Arc;
use test_helpers::{
	build_test_config, insert_api_key, insert_tenant, test_db_url, ConfigProfile, TestDb,
};

const CSV_BODY: &str = concat!(
    "--Audit\r\nContent-Disposition: form-data; name=\"file\"; filename=\"contacts.csv\"\r\n",
    "Content-Type: text/csv\r\n\r\nemail\r\nSales@company.example\r\nsales@company.example\r\n--Audit--\r\n"
);

#[tokio::test]
#[serial]
async fn retention_preserves_pending_effects_leases_and_unpublished_work() {
	let db = TestDb::start().await;
	let tenant = insert_tenant(db.pool(), "retention-pending-work", Some(100), 0).await;
	for (state, complete, lease) in [
		("running", false, false),
		("retrying", true, false),
		("completed", false, false),
		("completed", true, true),
		("completed", true, false),
	] {
		sqlx::query("INSERT INTO v1_task_result (tenant_id, payload, result, task_state, dispatch_id, verification_stored, completion_done, lease_expires_at, completed_at) VALUES ($1, '{}', '{}', $2::task_state, gen_random_uuid(), true, $3, CASE WHEN $4 THEN NOW() + INTERVAL '60 seconds' ELSE NULL END, NOW() - INTERVAL '1000 days')")
			.bind(tenant).bind(state).bind(complete).bind(lease).execute(db.pool()).await.unwrap();
	}
	let queued_id: i32 = sqlx::query_scalar("INSERT INTO v1_task_result (tenant_id, payload, result, task_state, dispatch_id, verification_stored, completion_done, completed_at) VALUES ($1, '{}', '{}', 'completed', gen_random_uuid(), true, true, NOW() - INTERVAL '1000 days') RETURNING id")
		.bind(tenant).fetch_one(db.pool()).await.unwrap();
	sqlx::query("INSERT INTO verification_dispatch (id, task_id, payload, priority) SELECT dispatch_id, id, '{}', 1 FROM v1_task_result WHERE id = $1")
		.bind(queued_id).execute(db.pool()).await.unwrap();
	retention::prune_expired_results(db.pool()).await.unwrap();
	let rows: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM v1_task_result")
		.fetch_one(db.pool())
		.await
		.unwrap();
	assert_eq!(
		rows, 5,
		"only the completed task without outstanding work may expire"
	);
}

#[tokio::test]
#[serial]
async fn every_list_insertion_failure_rolls_back_quota_rows_and_dispatch() {
	let db = TestDb::start().await;
	let tenant = insert_tenant(db.pool(), "atomic-list", Some(100), 0).await;
	let (key, _) = insert_api_key(db.pool(), tenant).await;
	let config = build_test_config(ConfigProfile::PseudoWorker, Some(&test_db_url()), None).await;
	let routes = create_routes(config);
	sqlx::raw_sql("CREATE FUNCTION audit_insert_failure() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN RAISE EXCEPTION 'audit insertion failure'; END $$")
        .execute(db.pool()).await.unwrap();
	for table in [
		"v1_bulk_job",
		"v1_lists",
		"v1_task_result",
		"verification_dispatch",
	] {
		sqlx::raw_sql(&format!("CREATE TRIGGER audit_fail BEFORE INSERT ON {table} FOR EACH ROW EXECUTE FUNCTION audit_insert_failure()"))
            .execute(db.pool()).await.unwrap();
		let response = warp::test::request()
			.method("POST")
			.path("/v1/lists")
			.header("authorization", format!("Bearer {key}"))
			.header("content-type", "multipart/form-data; boundary=Audit")
			.body(CSV_BODY)
			.reply(&routes)
			.await;
		assert_eq!(response.status(), 500, "{table}: {:?}", response.body());
		let counts: (i32, i64, i64, i64, i64) = sqlx::query_as(
			"SELECT used_this_period, (SELECT COUNT(*) FROM v1_bulk_job),
             (SELECT COUNT(*) FROM v1_lists), (SELECT COUNT(*) FROM v1_task_result),
             (SELECT COUNT(*) FROM verification_dispatch) FROM tenants WHERE id = $1",
		)
		.bind(tenant)
		.fetch_one(db.pool())
		.await
		.unwrap();
		assert_eq!(counts, (0, 0, 0, 0, 0), "{table}");
		sqlx::raw_sql(&format!("DROP TRIGGER audit_fail ON {table}"))
			.execute(db.pool())
			.await
			.unwrap();
	}
	sqlx::raw_sql("DROP FUNCTION audit_insert_failure()")
		.execute(db.pool())
		.await
		.unwrap();
	let response = warp::test::request()
		.method("POST")
		.path("/v1/lists")
		.header("authorization", format!("Bearer {key}"))
		.header("content-type", "multipart/form-data; boundary=Audit")
		.body(CSV_BODY)
		.reply(&routes)
		.await;
	assert_eq!(response.status(), 202, "{:?}", response.body());
	let counts: (i32, i64) = sqlx::query_as("SELECT used_this_period, (SELECT COUNT(*) FROM verification_dispatch) FROM tenants WHERE id = $1")
        .bind(tenant).fetch_one(db.pool()).await.unwrap();
	assert_eq!(
		counts,
		(1, 1),
		"case variants share one charge and broker availability is independent of committed admission"
	);
}

#[tokio::test]
#[serial]
async fn bulk_response_and_retry_intent_commit_with_their_quota() {
	let db = TestDb::start().await;
	let tenant = insert_tenant(db.pool(), "atomic-bulk-retry", Some(100), 0).await;
	let (key, _) = insert_api_key(db.pool(), tenant).await;
	let config = build_test_config(ConfigProfile::PseudoWorker, Some(&test_db_url()), None).await;
	let routes = create_routes(config);
	sqlx::raw_sql("CREATE FUNCTION audit_replay_failure() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN IF NEW.status = 'completed' THEN RAISE EXCEPTION 'audit response failure'; END IF; RETURN NEW; END $$; CREATE TRIGGER audit_fail BEFORE UPDATE ON idempotency_keys FOR EACH ROW EXECUTE FUNCTION audit_replay_failure()")
        .execute(db.pool()).await.unwrap();
	let response = warp::test::request()
		.method("POST")
		.path("/v1/bulk")
		.header("authorization", format!("Bearer {key}"))
		.header("idempotency-key", "one-job")
		.json(&serde_json::json!({"input":["Sales@company.example"]}))
		.reply(&routes)
		.await;
	assert_eq!(response.status(), 500);
	let rows: (i32, i64, i64) = sqlx::query_as("SELECT used_this_period, (SELECT COUNT(*) FROM v1_bulk_job), (SELECT COUNT(*) FROM verification_dispatch) FROM tenants WHERE id = $1")
        .bind(tenant).fetch_one(db.pool()).await.unwrap();
	assert_eq!(rows, (0, 0, 0));
	sqlx::raw_sql(
		"DROP TRIGGER audit_fail ON idempotency_keys; DROP FUNCTION audit_replay_failure()",
	)
	.execute(db.pool())
	.await
	.unwrap();
	let mut original = None;
	for _ in 0..2 {
		let response = warp::test::request()
			.method("POST")
			.path("/v1/bulk")
			.header("authorization", format!("Bearer {key}"))
			.header("idempotency-key", "one-job")
			.json(&serde_json::json!({"input":["Sales@company.example"]}))
			.reply(&routes)
			.await;
		assert_eq!(response.status(), 200, "{:?}", response.body());
		let body: serde_json::Value = serde_json::from_slice(response.body()).unwrap();
		if let Some(first) = &original {
			assert_eq!(first, &body);
		} else {
			original = Some(body);
		}
	}
	let job_id = original.unwrap()["job_id"].as_i64().unwrap() as i32;
	sqlx::raw_sql("DELETE FROM verification_dispatch; UPDATE v1_task_result SET task_state = 'failed', completion_done = true; UPDATE v1_bulk_job SET status = 'failed'; UPDATE tenants SET used_this_period = 0")
        .execute(db.pool()).await.unwrap();
	sqlx::raw_sql("CREATE FUNCTION audit_retry_failure() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN IF NEW.event_type = 'job.retry_requested' THEN RAISE EXCEPTION 'audit retry commit failure'; END IF; RETURN NEW; END $$; CREATE TRIGGER audit_fail BEFORE INSERT ON job_events FOR EACH ROW EXECUTE FUNCTION audit_retry_failure()")
        .execute(db.pool()).await.unwrap();
	let response = warp::test::request()
		.method("POST")
		.path(&format!("/v1/jobs/{job_id}/retry"))
		.header("authorization", format!("Bearer {key}"))
		.reply(&routes)
		.await;
	assert_eq!(response.status(), 500);
	let rollback: (i32, String, i64) = sqlx::query_as("SELECT used_this_period, (SELECT task_state::TEXT FROM v1_task_result LIMIT 1), (SELECT COUNT(*) FROM verification_dispatch) FROM tenants WHERE id = $1")
        .bind(tenant).fetch_one(db.pool()).await.unwrap();
	assert_eq!(rollback, (0, "failed".into(), 0));
	sqlx::raw_sql("DROP TRIGGER audit_fail ON job_events; DROP FUNCTION audit_retry_failure()")
		.execute(db.pool())
		.await
		.unwrap();
	for expected in [1, 0] {
		let response = warp::test::request()
			.method("POST")
			.path(&format!("/v1/jobs/{job_id}/retry"))
			.header("authorization", format!("Bearer {key}"))
			.reply(&routes)
			.await;
		assert_eq!(response.status(), 200, "{:?}", response.body());
		let body: serde_json::Value = serde_json::from_slice(response.body()).unwrap();
		assert_eq!(body["tasks_retried"], expected);
	}
	let committed: (i32, i64) = sqlx::query_as("SELECT used_this_period, (SELECT COUNT(*) FROM verification_dispatch) FROM tenants WHERE id = $1")
        .bind(tenant).fetch_one(db.pool()).await.unwrap();
	assert_eq!(committed, (1, 1));
}

#[tokio::test]
#[serial]
async fn broker_failure_and_lost_publication_commit_retain_one_dispatch_identity() {
	let db = TestDb::start().await;
	let broker = test_helpers::TestRabbitMq::start().await;
	let tenant = insert_tenant(db.pool(), "publication-recovery", Some(100), 0).await;
	let (key, _) = insert_api_key(db.pool(), tenant).await;
	let config = build_test_config(
		ConfigProfile::WorkerRabbit,
		Some(&test_db_url()),
		Some(&broker.amqp_url),
	)
	.await;
	let channel = config.must_worker_config().unwrap().channel;
	let routes = create_routes(config);
	let response = warp::test::request()
		.method("POST")
		.path("/v1/bulk")
		.header("authorization", format!("Bearer {key}"))
		.json(&serde_json::json!({"input":["invalid"]}))
		.reply(&routes)
		.await;
	assert_eq!(response.status(), 200, "{:?}", response.body());
	let id: uuid::Uuid = sqlx::query_scalar("SELECT id FROM verification_dispatch")
		.fetch_one(db.pool())
		.await
		.unwrap();
	sqlx::raw_sql("CREATE FUNCTION audit_publication_failure() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN RAISE EXCEPTION 'audit lost publication commit'; END $$; CREATE TRIGGER audit_fail BEFORE DELETE ON verification_dispatch FOR EACH ROW EXECUTE FUNCTION audit_publication_failure()")
        .execute(db.pool()).await.unwrap();
	assert!(publish_next(db.pool(), &channel, "check_email")
		.await
		.is_err());
	sqlx::raw_sql("DROP TRIGGER audit_fail ON verification_dispatch; DROP FUNCTION audit_publication_failure()").execute(db.pool()).await.unwrap();
	assert!(publish_next(db.pool(), &channel, "check_email")
		.await
		.unwrap());
	for _ in 0..2 {
		let delivery = channel
			.basic_get(
				reacher_backend::worker::consume::CHECK_EMAIL_QUEUE.into(),
				Default::default(),
			)
			.await
			.unwrap()
			.unwrap();
		let task: CheckEmailTask = serde_json::from_slice(&delivery.data).unwrap();
		assert_eq!(task.metadata.unwrap().dispatch_id, Some(id));
		delivery.ack(Default::default()).await.unwrap();
	}
	let response = warp::test::request()
		.method("POST")
		.path("/v1/bulk")
		.header("authorization", format!("Bearer {key}"))
		.json(&serde_json::json!({"input":["another-invalid"]}))
		.reply(&routes)
		.await;
	assert_eq!(response.status(), 200, "{:?}", response.body());
	channel
		.close(200, "audit channel failure".into())
		.await
		.unwrap();
	assert!(publish_next(db.pool(), &channel, "check_email")
		.await
		.unwrap());
	let retained: (i64, i32, bool) = sqlx::query_as(
		"SELECT COUNT(*) OVER(), attempts, next_attempt_at > NOW() FROM verification_dispatch",
	)
	.fetch_one(db.pool())
	.await
	.unwrap();
	assert_eq!(retained, (1, 1, true));
}

#[tokio::test]
#[serial]
async fn expired_worker_is_fenced_and_saved_completion_recovers_without_verification() {
	let db = TestDb::start().await;
	let tenant = insert_tenant(db.pool(), "fenced-completion", Some(100), 0).await;
	let (key, _) = insert_api_key(db.pool(), tenant).await;
	let config = build_test_config(ConfigProfile::PseudoWorker, Some(&test_db_url()), None).await;
	let routes = create_routes(Arc::clone(&config));
	let response = warp::test::request()
		.method("POST")
		.path("/v1/bulk")
		.header("authorization", format!("Bearer {key}"))
		.json(&serde_json::json!({"input":["Sales@company.example"]}))
		.reply(&routes)
		.await;
	assert_eq!(response.status(), 200, "{:?}", response.body());
	let payload: serde_json::Value =
		sqlx::query_scalar("SELECT payload FROM verification_dispatch")
			.fetch_one(db.pool())
			.await
			.unwrap();
	let mut task: CheckEmailTask = serde_json::from_value(payload).unwrap();
	let id = task.metadata.as_ref().unwrap().task_db_id.unwrap();
	let other_pool = sqlx::PgPool::connect(&test_db_url()).await.unwrap();
	let (one, two) = tokio::join!(claim_task(db.pool(), &task), claim_task(&other_pool, &task));
	let lease = match (one.unwrap(), two.unwrap()) {
		(Claim::Acquired(lease), Claim::Busy) | (Claim::Busy, Claim::Acquired(lease)) => lease,
		pair => panic!("Expected one owner: {:?}", pair),
	};
	task.metadata.as_mut().unwrap().lease_id = Some(lease);
	let stale = task.clone();
	sqlx::query(
		"UPDATE v1_task_result SET lease_expires_at = NOW() - INTERVAL '1 second' WHERE id = $1",
	)
	.bind(id)
	.execute(db.pool())
	.await
	.unwrap();
	let Claim::Acquired(new_lease) = claim_task(&other_pool, &task).await.unwrap() else {
		panic!("Lease must be recoverable")
	};
	task.metadata.as_mut().unwrap().lease_id = Some(new_lease);
	let prepared = reacher_backend::scoring::response::prepare_check_email_success(
		&config,
		reacher_backend::sandbox::sandbox_check(&task.input.to_email),
		Some(tenant),
		chrono::Utc::now(),
		false,
		None,
	)
	.await
	.unwrap();
	let storage = config.get_storage_adapter();
	assert!(storage
		.store_prepared(&stale, &prepared, None)
		.await
		.is_err());
	storage
		.store_prepared(&task, &prepared, None)
		.await
		.unwrap();
	sqlx::query(
		"UPDATE tenants SET settings = '{\"auto_suppress_below_score\":101}' WHERE id = $1",
	)
	.bind(tenant)
	.execute(db.pool())
	.await
	.unwrap();
	sqlx::raw_sql("CREATE FUNCTION audit_effect_failure() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN RAISE EXCEPTION 'audit effect failure'; END $$; CREATE TRIGGER audit_fail BEFORE INSERT ON v1_suppression_entries FOR EACH ROW EXECUTE FUNCTION audit_effect_failure()")
        .execute(db.pool()).await.unwrap();
	assert!(finish_stored_task(db.pool(), &task, &config).await.is_err());
	let pending: (bool, bool, String) = sqlx::query_as("SELECT verification_stored, completion_done, task_state::TEXT FROM v1_task_result WHERE id = $1")
        .bind(id).fetch_one(db.pool()).await.unwrap();
	assert_eq!(pending, (true, false, "running".into()));
	sqlx::raw_sql("DROP TRIGGER audit_fail ON v1_suppression_entries; DROP FUNCTION audit_effect_failure(); DELETE FROM verification_dispatch; UPDATE v1_task_result SET lease_expires_at = NOW() - INTERVAL '1 second', updated_at = NOW() - INTERVAL '61 seconds'")
        .execute(db.pool()).await.unwrap();
	assert_eq!(recover_pending_completion(db.pool()).await.unwrap(), 1);
	assert_eq!(recover_pending_completion(db.pool()).await.unwrap(), 0);
	let Claim::Acquired(recovered) = claim_task(db.pool(), &task).await.unwrap() else {
		panic!("Expected recovery claim")
	};
	task.metadata.as_mut().unwrap().lease_id = Some(recovered);
	// The recovery entry point consumes saved JSON and cannot call check_email.
	finish_stored_task(db.pool(), &task, &config).await.unwrap();
	assert_eq!(claim_task(db.pool(), &task).await.unwrap(), Claim::Finished);
	let effects: (i64, i64, String) = sqlx::query_as(
        "SELECT (SELECT COUNT(*) FROM v1_suppression_entries), (SELECT COUNT(*) FROM job_events WHERE event_type = 'task.completed'), (SELECT status::TEXT FROM v1_bulk_job LIMIT 1)",
    ).fetch_one(db.pool()).await.unwrap();
	assert_eq!(effects, (1, 1, "completed".into()));
	let mut tx = db.pool().begin().await.unwrap();
	enqueue(&mut tx, task.clone(), 1).await.unwrap();
	tx.commit().await.unwrap();
	assert_eq!(claim_task(db.pool(), &task).await.unwrap(), Claim::Obsolete);
	assert!(storage
		.store_prepared(&task, &prepared, None)
		.await
		.is_err());
	sqlx::query("DELETE FROM v1_task_result WHERE id = $1")
		.bind(id)
		.execute(db.pool())
		.await
		.unwrap();
	assert!(storage
		.store_prepared(&task, &prepared, None)
		.await
		.is_err());
	let remaining: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM v1_task_result")
		.fetch_one(db.pool())
		.await
		.unwrap();
	assert_eq!(
		remaining, 0,
		"a stale worker must never recreate a deleted task"
	);
	other_pool.close().await;
}
