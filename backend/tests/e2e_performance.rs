mod test_helpers;
use serde_json::{json, Value};
use serial_test::serial;
use test_helpers::*;
use warp::test::request;

#[tokio::test]
#[serial]
async fn overview_is_scoped_exact_and_matches_live_readiness() {
	let db = TestDb::start().await;
	let tenant = insert_tenant(db.pool(), "performance", Some(100), 0).await;
	let other = insert_tenant(db.pool(), "performance-other", Some(100), 0).await;
	let (key, _) = insert_api_key(db.pool(), tenant).await;
	let config = build_test_config(ConfigProfile::PseudoWorker, Some(db.db_url()), None).await;
	let routes = reacher_backend::http::create_routes(config);
	let job = insert_job(db.pool(), Some(tenant), 2, "running").await;
	let list = insert_list(
		db.pool(),
		tenant,
		job,
		"Performance list",
		"processing",
		2,
		&["email"],
		json!([{"email":"safe@example.com"},{"email":"risky@example.com"}]),
	)
	.await;
	for (email, safe) in [("safe@example.com", true), ("risky@example.com", false)] {
		insert_scored_task(
			db.pool(),
			job,
			Some(tenant),
			email,
			Some(json!({"list_id":list})),
			Some(json!({"input":email})),
			"completed",
			Some(90),
			Some(if safe { "valid" } else { "risky" }),
			None,
			Some(safe),
			None,
			Some(email),
			false,
		)
		.await;
	}
	// No detail read is needed for worker-owned terminal state.
	reacher_backend::worker::job_lifecycle::finalize_list_if_done(db.pool(), list)
		.await
		.unwrap();
	reacher_backend::worker::job_lifecycle::finalize_bulk_job_if_done(db.pool(), job)
		.await
		.unwrap();
	let other_job = insert_job(db.pool(), Some(other), 0, "completed").await;
	insert_list(
		db.pool(),
		other,
		other_job,
		"Private list",
		"completed",
		0,
		&["email"],
		json!([]),
	)
	.await;
	sqlx::query("INSERT INTO v1_contact_outcomes (tenant_id,email,canonical_email,provider,event_type,occurred_at) SELECT $1,'safe@example.com','safe@example.com','test','bounced',NOW() FROM generate_series(1,501)").bind(tenant).execute(db.pool()).await.unwrap();
	let pipeline = insert_pipeline(
		db.pool(),
		tenant,
		"Performance pipeline",
		json!({"type":"list_snapshot","list_id":list}),
	)
	.await;
	let run = insert_pipeline_run(
		db.pool(),
		pipeline,
		tenant,
		"completed",
		Some(job),
		Some(list),
	)
	.await;
	sqlx::query("UPDATE v1_pipelines SET last_run_id=$1 WHERE id=$2")
		.bind(run)
		.bind(pipeline)
		.execute(db.pool())
		.await
		.unwrap();
	let legacy_list: i32 = sqlx::query_scalar("INSERT INTO v1_lists (tenant_id,name,original_filename,file_size_bytes,total_rows,email_column,status) VALUES ($1,'Imported snapshot','snapshot.csv',1,0,'email','completed') RETURNING id").bind(tenant).fetch_one(db.pool()).await.unwrap();
	let auth = format!("Bearer {key}");
	let r = request()
		.path("/v1/overview")
		.header("authorization", &auth)
		.reply(&routes)
		.await;
	assert_eq!(r.status(), 200, "{:?}", r.body());
	let body: Value = serde_json::from_slice(r.body()).unwrap();
	assert_eq!(body["pipelines"]["status"], "ready");
	assert_eq!(body["pipelines"]["data"][0]["id"], pipeline);
	assert_eq!(body["pipelines"]["data"][0]["last_run_id"], run);
	assert_eq!(body["pipelines"]["data"][0]["last_run_status"], "completed");
	assert_eq!(body["lists"]["status"], "ready");
	assert_eq!(body["lists"]["data"].as_array().unwrap().len(), 2);
	let summary = body["lists"]["data"]
		.as_array()
		.unwrap()
		.iter()
		.find(|row| row["id"] == list)
		.unwrap();
	let legacy = body["lists"]["data"]
		.as_array()
		.unwrap()
		.iter()
		.find(|row| row["id"] == legacy_list)
		.unwrap();
	assert!(legacy["job_id"].is_null());
	assert!(legacy["ready_to_send"].is_null());
	let index = request()
		.path("/v1/lists")
		.header("authorization", &auth)
		.reply(&routes)
		.await;
	assert_eq!(index.status(), 200);
	assert_eq!(summary["status"], "completed");
	assert_eq!(summary["total_processed"], 2);
	assert_eq!(summary["quality_grade"], "C");
	assert_eq!(body["outcomes"]["data"]["bounced"], 501);
	let approval = request()
		.path(&format!("/v1/jobs/{job}/approval"))
		.header("authorization", &auth)
		.reply(&routes)
		.await;
	let approval: Value = serde_json::from_slice(approval.body()).unwrap();
	assert_eq!(summary["ready_to_send"], approval["ready_to_send"]);
	assert_eq!(
		summary["safe_to_send_count"],
		approval["safe_to_send_count"]
	);
	let (scoped, _) = insert_api_key_with_scopes(db.pool(), tenant, &["lists"]).await;
	let r = request()
		.path("/v1/overview")
		.header("authorization", format!("Bearer {scoped}"))
		.reply(&routes)
		.await;
	let body: Value = serde_json::from_slice(r.body()).unwrap();
	assert_eq!(body["lists"]["status"], "ready");
	assert!(body["lists"]["data"][0]["ready_to_send"].is_null());
	for section in [
		"alerts",
		"outcomes",
		"suppressions",
		"domains",
		"pipelines",
		"reverification",
	] {
		assert_eq!(body[section]["status"], "forbidden");
		assert!(body[section]["data"].is_null());
	}
	// Reading a retry-in-progress must neither complete it nor write the list.
	sqlx::query("UPDATE v1_lists SET status='processing' WHERE id=$1")
		.bind(list)
		.execute(db.pool())
		.await
		.unwrap();
	sqlx::query("UPDATE v1_task_result SET task_state='retrying' WHERE job_id=$1")
		.bind(job)
		.execute(db.pool())
		.await
		.unwrap();
	let before: String = sqlx::query_scalar("SELECT xmin::text FROM v1_lists WHERE id=$1")
		.bind(list)
		.fetch_one(db.pool())
		.await
		.unwrap();
	let r = request()
		.path(&format!("/v1/lists/{list}"))
		.header("authorization", &auth)
		.reply(&routes)
		.await;
	let detail: Value = serde_json::from_slice(r.body()).unwrap();
	assert_eq!(detail["status"], "processing");
	assert_eq!(detail["summary"]["total_processed"], 0);
	assert_eq!(detail["original_filename"], "fixture.csv");
	let after: String = sqlx::query_scalar("SELECT xmin::text FROM v1_lists WHERE id=$1")
		.bind(list)
		.fetch_one(db.pool())
		.await
		.unwrap();
	assert_eq!(before, after);
}

#[tokio::test]
#[serial]
async fn unchanged_tenant_claims_do_not_write_or_weaken_billing() {
	let db = TestDb::start().await;
	let tenant = insert_tenant(db.pool(), "claims", Some(100), 0).await;
	use reacher_backend::tenant::jwt::sync_tenant_from_claims;
	sync_tenant_from_claims(
		db.pool(),
		tenant,
		"Claims",
		"owner@example.com",
		Some("professional"),
	)
	.await
	.unwrap();
	let before: String = sqlx::query_scalar("SELECT xmin::text FROM tenants WHERE id=$1")
		.bind(tenant)
		.fetch_one(db.pool())
		.await
		.unwrap();
	sync_tenant_from_claims(
		db.pool(),
		tenant,
		"Claims",
		"owner@example.com",
		Some("professional"),
	)
	.await
	.unwrap();
	let after: String = sqlx::query_scalar("SELECT xmin::text FROM tenants WHERE id=$1")
		.bind(tenant)
		.fetch_one(db.pool())
		.await
		.unwrap();
	assert_eq!(before, after);
	sync_tenant_from_claims(
		db.pool(),
		tenant,
		"Claims",
		"owner@example.com",
		Some("free"),
	)
	.await
	.unwrap();
	let limit: i32 = sqlx::query_scalar("SELECT monthly_email_limit FROM tenants WHERE id=$1")
		.bind(tenant)
		.fetch_one(db.pool())
		.await
		.unwrap();
	assert_eq!(limit, 100);
}
