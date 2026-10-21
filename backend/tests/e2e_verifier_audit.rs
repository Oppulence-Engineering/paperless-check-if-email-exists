mod test_helpers;
use serde_json::{json, Value};
use serial_test::serial;
use test_helpers::{build_test_config, insert_api_key, insert_tenant, ConfigProfile, TestDb};
use warp::test::request;

#[tokio::test]
#[serial]
async fn suppression_validation_restore_and_read_only_checks() {
	let db = TestDb::start().await;
	let tenant = insert_tenant(db.pool(), "audit-suppression", Some(100), 0).await;
	let (key, _) = insert_api_key(db.pool(), tenant).await;
	let routes = reacher_backend::http::create_routes(
		build_test_config(ConfigProfile::PseudoWorker, Some(db.db_url()), None).await,
	);
	let auth = format!("Bearer {key}");
	let response = request().method("POST").path("/v1/suppressions").header("authorization", &auth)
        .json(&json!({"emails":["Test.User+qa@Example.com","test.user+qa@example.com","not-an-email","qa <script>x</script>@example.com", ""], "created_by":"forged"})).reply(&routes).await;
	assert_eq!(response.status(), 200, "{:?}", response.body());
	let body: Value = serde_json::from_slice(response.body()).unwrap();
	assert_eq!(body["submitted"], 5);
	assert_eq!(body["added"], 1);
	assert_eq!(body["duplicates"], 1);
	assert_eq!(body["rejected"].as_array().unwrap().len(), 3);
	let id = body["entry_ids"][0].as_i64().unwrap();
	let before: chrono::DateTime<chrono::Utc> =
		sqlx::query_scalar("SELECT last_seen_at FROM v1_suppression_entries WHERE id=$1")
			.bind(id as i32)
			.fetch_one(db.pool())
			.await
			.unwrap();
	let checked = request()
		.path("/v1/suppressions/check?email=test.user%2Bqa%40example.com")
		.header("authorization", &auth)
		.reply(&routes)
		.await;
	assert_eq!(
		serde_json::from_slice::<Value>(checked.body()).unwrap()["suppressed"],
		true
	);
	let after: chrono::DateTime<chrono::Utc> =
		sqlx::query_scalar("SELECT last_seen_at FROM v1_suppression_entries WHERE id=$1")
			.bind(id as i32)
			.fetch_one(db.pool())
			.await
			.unwrap();
	assert_eq!(before, after);
	let actor: String =
		sqlx::query_scalar("SELECT actor_id FROM v1_suppression_events WHERE entry_id=$1 LIMIT 1")
			.bind(id as i32)
			.fetch_one(db.pool())
			.await
			.unwrap();
	assert_ne!(actor, "forged");
	let removed = request()
		.method("DELETE")
		.path(&format!("/v1/suppressions/{id}"))
		.header("authorization", &auth)
		.reply(&routes)
		.await;
	assert_eq!(removed.status(), 200);
	let replacement = request().method("POST").path("/v1/suppressions").header("authorization",&auth)
        .json(&json!({"emails":["TEST.USER+QA@EXAMPLE.COM"],"expires_at":(chrono::Utc::now()+chrono::Duration::days(30)).to_rfc3339()})).reply(&routes).await;
	assert_eq!(replacement.status(), 200);
	let restored = request()
		.method("POST")
		.path(&format!("/v1/suppressions/{id}/restore"))
		.header("authorization", &auth)
		.reply(&routes)
		.await;
	assert_eq!(restored.status(), 200, "{:?}", restored.body());
	let permanent: bool=sqlx::query_scalar("SELECT expires_at IS NULL FROM v1_suppression_entries WHERE tenant_id=$1 AND status='active'").bind(tenant).fetch_one(db.pool()).await.unwrap();
	assert!(
		permanent,
		"restoring a permanent suppression must not shorten protection"
	);
	sqlx::query("UPDATE v1_suppression_entries SET expires_at=NOW()-INTERVAL '1 day' WHERE tenant_id=$1 AND status='active'").bind(tenant).execute(db.pool()).await.unwrap();
	let expired_check = request()
		.path("/v1/suppressions/check?email=test.user%2Bqa%40example.com")
		.header("authorization", &auth)
		.reply(&routes)
		.await;
	let expired_check: Value = serde_json::from_slice(expired_check.body()).unwrap();
	assert_eq!(expired_check["suppressed"], false);
	assert_eq!(expired_check["status"], "expired");
	for path in [
		"/v1/suppressions?include_expired=true",
		"/v1/suppressions?status=expired",
	] {
		let r = request()
			.path(path)
			.header("authorization", &auth)
			.reply(&routes)
			.await;
		let body: Value = serde_json::from_slice(r.body()).unwrap();
		assert_eq!(body["total"], 1);
		assert_eq!(body["entries"][0]["status"], "expired");
	}
	let export = request()
		.path("/v1/suppressions/export?include_expired=true")
		.header("authorization", &auth)
		.reply(&routes)
		.await;
	assert!(std::str::from_utf8(export.body())
		.unwrap()
		.contains(",expired,"));

	let expired = request()
		.method("POST")
		.path("/v1/suppressions")
		.header("authorization", &auth)
		.json(&json!({"emails":["old@example.com"],"expires_at":"2000-01-01T00:00:00Z"}))
		.reply(&routes)
		.await;
	assert_eq!(expired.status(), 400);
	let other = insert_tenant(db.pool(), "audit-other", Some(100), 0).await;
	let (other_key, _) = insert_api_key(db.pool(), other).await;
	let forbidden = request()
		.method("POST")
		.path(&format!("/v1/suppressions/{id}/restore"))
		.header("authorization", format!("Bearer {other_key}"))
		.reply(&routes)
		.await;
	assert_eq!(forbidden.status(), 404);
}

#[tokio::test]
#[serial]
async fn preview_submission_idempotency_quality_and_original_rows() {
	let db = TestDb::start().await;
	let tenant = insert_tenant(db.pool(), "audit-list", Some(100), 0).await;
	let (key, _) = insert_api_key(db.pool(), tenant).await;
	let routes = reacher_backend::http::create_routes(
		build_test_config(ConfigProfile::PseudoWorker, Some(db.db_url()), None).await,
	);
	let auth = format!("Bearer {key}");
	let csv="--Audit\r\nContent-Disposition: form-data; name=\"file\"; filename=\"audit.csv\"\r\nContent-Type: text/csv\r\n\r\nemail,name\r\nOne@Example.com,A\r\none@example.com,B\r\nnot-an-email,C\r\n,D\r\ntwo@example.com,E\r\n--Audit\r\nContent-Disposition: form-data; name=\"source_key\"\r\n\r\naudit\r\n--Audit--\r\n";
	let preview = request()
		.method("POST")
		.path("/v1/lists/preview")
		.header("authorization", &auth)
		.header("content-type", "multipart/form-data; boundary=Audit")
		.body(csv)
		.reply(&routes)
		.await;
	assert_eq!(preview.status(), 200, "{:?}", preview.body());
	let p: Value = serde_json::from_slice(preview.body()).unwrap();
	assert_eq!(p["total_rows"], 5);
	assert_eq!(p["unique_emails"], 2);
	assert_eq!(p["duplicates"], 1);
	assert_eq!(p["estimated_credits"], 2);
	assert_eq!(p["rejected"].as_array().unwrap().len(), 2);
	let used: i32 = sqlx::query_scalar("SELECT used_this_period FROM tenants WHERE id=$1")
		.bind(tenant)
		.fetch_one(db.pool())
		.await
		.unwrap();
	assert_eq!(used, 0);
	sqlx::query("UPDATE tenants SET used_this_period=100 WHERE id=$1")
		.bind(tenant)
		.execute(db.pool())
		.await
		.unwrap();
	let over_quota = request()
		.method("POST")
		.path("/v1/lists")
		.header("authorization", &auth)
		.header("content-type", "multipart/form-data; boundary=Audit")
		.body(csv)
		.reply(&routes)
		.await;
	assert_eq!(over_quota.status(), 429);
	sqlx::query("UPDATE tenants SET used_this_period=0 WHERE id=$1")
		.bind(tenant)
		.execute(db.pool())
		.await
		.unwrap();
	let submit = || async {
		let r = request()
			.method("POST")
			.path("/v1/lists")
			.header("authorization", &auth)
			.header("idempotency-key", "audit-retry")
			.header("content-type", "multipart/form-data; boundary=Audit")
			.body(csv)
			.reply(&routes)
			.await;
		assert_eq!(r.status(), 202, "{:?}", r.body());
		serde_json::from_slice::<Value>(r.body()).unwrap()
	};
	let (first, second) = tokio::join!(submit(), submit());
	let responses = [first, second];
	assert_eq!(responses[0], responses[1]);
	let used: i32 = sqlx::query_scalar("SELECT used_this_period FROM tenants WHERE id=$1")
		.bind(tenant)
		.fetch_one(db.pool())
		.await
		.unwrap();
	assert_eq!(used, 2);
	let job = responses[0]["job_id"].as_i64().unwrap() as i32;
	let list = responses[0]["list_id"].as_i64().unwrap();
	let comment = request()
		.method("POST")
		.path("/v1/comments")
		.header("authorization", &auth)
		.json(&json!({"list_id":list,"body":"Release validation","author":"forged user"}))
		.reply(&routes)
		.await;
	assert_eq!(comment.status(), 201);
	let comment: Value = serde_json::from_slice(comment.body()).unwrap();
	assert!(comment["author"].as_str().unwrap().starts_with("API key: "));
	assert!(!comment["author"].as_str().unwrap().contains("forged"));

	sqlx::query("UPDATE v1_task_result SET task_state='completed',completed_at=now(),safe_to_send=(canonical_email='one@example.com'),score_category=CASE WHEN canonical_email='one@example.com' THEN 'valid' ELSE 'risky' END,result=jsonb_build_object('input',payload->'input'->>'to_email','score',jsonb_build_object('signals',jsonb_build_object('smtp_is_catch_all',canonical_email='two@example.com'))) WHERE job_id=$1 AND canonical_email IS NOT NULL").bind(job).execute(db.pool()).await.unwrap();
	let q = request()
		.path(&format!("/v1/lists/{list}/quality"))
		.header("authorization", &auth)
		.reply(&routes)
		.await;
	assert_eq!(q.status(), 200, "{:?}", q.body());
	let q: Value = serde_json::from_slice(q.body()).unwrap();
	assert_eq!(q["processed"], 2);
	assert_eq!(q["safe_to_send_count"], 1);
	assert_eq!(q["safe_to_send_pct"], 50.0);
	assert_eq!(q["quality_grade"], "C");
	assert_eq!(q["risk_breakdown"]["catch_all"], 1);
	let approval = request()
		.path(&format!("/v1/jobs/{job}/approval"))
		.header("authorization", &auth)
		.reply(&routes)
		.await;
	let approval: Value = serde_json::from_slice(approval.body()).unwrap();
	assert_eq!(approval["safe_to_send_pct"], 50.0);
	assert_eq!(approval["safe_to_send_count"], 1);
	assert_eq!(approval["risk_flags"]["catch_all_count"], 1);

	let source = request()
		.path("/v1/sources/quality?source_key=audit")
		.header("authorization", &auth)
		.reply(&routes)
		.await;
	let source: Value = serde_json::from_slice(source.body()).unwrap();
	assert_eq!(source["sources"][0]["quality_grade"], "C");
	assert_eq!(source["sources"][0]["processed_records"], 2);
	let rows = request()
		.path(&format!("/v1/jobs/{job}/results"))
		.header("authorization", &auth)
		.reply(&routes)
		.await;
	let rows: Value = serde_json::from_slice(rows.body()).unwrap();
	let bad = rows["results"]
		.as_array()
		.unwrap()
		.iter()
		.find(|row| row["row_number"] == 3)
		.unwrap();
	assert_eq!(bad["original_email"], "not-an-email");
	let history = request()
		.method("POST")
		.path("/v1/emails/history")
		.header("authorization", &auth)
		.json(&json!({"email":"ONE@EXAMPLE.COM"}))
		.reply(&routes)
		.await;
	assert_eq!(history.status(), 200);
	assert_eq!(
		serde_json::from_slice::<Value>(history.body()).unwrap()["total"],
		1
	);

	sqlx::query("INSERT INTO v1_task_result (tenant_id,payload,result,canonical_email,task_state,completed_at,safe_to_send,score_category,source_key) SELECT tenant_id,payload,result,canonical_email,task_state,completed_at,safe_to_send,score_category,source_key FROM v1_task_result WHERE job_id=$1 AND canonical_email='one@example.com' AND NOT is_duplicate LIMIT 1").bind(job).execute(db.pool()).await.unwrap();
	let repeated = request()
		.path("/v1/sources/quality?source_key=audit")
		.header("authorization", &auth)
		.reply(&routes)
		.await;
	let repeated: Value = serde_json::from_slice(repeated.body()).unwrap();
	assert_eq!(repeated["sources"][0]["processed_records"], 2);
	assert_eq!(repeated["sources"][0]["quality_grade"], "C");
	sqlx::query("UPDATE v1_task_result SET task_state='failed' WHERE job_id=$1 AND canonical_email='two@example.com'").bind(job).execute(db.pool()).await.unwrap();
	let incomplete = request()
		.path(&format!("/v1/jobs/{job}/approval"))
		.header("authorization", &auth)
		.reply(&routes)
		.await;
	let incomplete: Value = serde_json::from_slice(incomplete.body()).unwrap();
	assert_eq!(incomplete["ready_to_send"], false);
	assert!(incomplete["recommendation"]
		.as_str()
		.unwrap()
		.contains("could not be verified"));
	let suppressed = request()
		.method("POST")
		.path("/v1/suppressions")
		.header("authorization", &auth)
		.json(&json!({"emails":["ONE@EXAMPLE.COM"]}))
		.reply(&routes)
		.await;
	assert_eq!(suppressed.status(), 200);
	sqlx::query("UPDATE v1_lists SET status='completed' WHERE id=$1")
		.bind(list as i32)
		.execute(db.pool())
		.await
		.unwrap();
	sqlx::query("UPDATE v1_bulk_job SET status='completed' WHERE id=$1")
		.bind(job)
		.execute(db.pool())
		.await
		.unwrap();
	for path in [
		format!("/v1/jobs/{job}/download"),
		format!("/v1/lists/{list}/download"),
	] {
		let export = request()
			.path(&path)
			.header("authorization", &auth)
			.reply(&routes)
			.await;
		assert_eq!(export.status(), 200, "{:?}", export.body());
		assert!(
			!String::from_utf8_lossy(export.body())
				.to_ascii_lowercase()
				.contains("one@example.com"),
			"new suppressions must not remain eligible for export"
		);
	}
}

#[tokio::test]
#[serial]
async fn domain_validation_and_verification_fail_closed() {
	let db = TestDb::start().await;
	let tenant = insert_tenant(db.pool(), "audit-domain", Some(100), 0).await;
	let (key, _) = insert_api_key(db.pool(), tenant).await;
	let routes = reacher_backend::http::create_routes(
		build_test_config(ConfigProfile::PseudoWorker, Some(db.db_url()), None).await,
	);
	let auth = format!("Bearer {key}");
	let invalid = request()
		.method("POST")
		.path("/v1/find_email")
		.header("authorization", &auth)
		.json(&json!({"first_name":"Test","last_name":"Person","domain":"not a domain!!"}))
		.reply(&routes)
		.await;
	assert_eq!(invalid.status(), 400);
	let created = request()
		.method("POST")
		.path("/v1/me/domains")
		.header("authorization", &auth)
		.json(&json!({"domain":"audit.example.com"}))
		.reply(&routes)
		.await;
	assert_eq!(created.status(), 201, "{:?}", created.body());
	let body: Value = serde_json::from_slice(created.body()).unwrap();
	assert_eq!(body["verification_record"], "_verifier.audit.example.com");
	assert!(body["verification_value"].as_str().unwrap().len() > 64);
	let forged = request()
		.method("PATCH")
		.path("/v1/me/domains/audit.example.com")
		.header("authorization", &auth)
		.json(&json!({"is_verified":true}))
		.reply(&routes)
		.await;
	assert_eq!(forged.status(), 400);
	let activated = request()
		.method("PATCH")
		.path("/v1/me/domains/audit.example.com")
		.header("authorization", &auth)
		.json(&json!({"is_active":true}))
		.reply(&routes)
		.await;
	assert_eq!(activated.status(), 400);
	let other = insert_tenant(db.pool(), "audit-domain-other", Some(100), 0).await;
	let (other_key, _) = insert_api_key(db.pool(), other).await;
	let foreign = request()
		.method("POST")
		.path("/v1/me/domains/audit.example.com/verify")
		.header("authorization", format!("Bearer {other_key}"))
		.reply(&routes)
		.await;
	assert_eq!(foreign.status(), 404);
	// Reproduce an already verified domain before editing its name.
	sqlx::query("UPDATE tenant_domains SET is_verified=true,is_active=true WHERE tenant_id=$1")
		.bind(tenant)
		.execute(db.pool())
		.await
		.unwrap();
	let changed = request()
		.method("PATCH")
		.path("/v1/me/domains/audit.example.com")
		.header("authorization", &auth)
		.json(&json!({"domain":"changed.example.com"}))
		.reply(&routes)
		.await;
	assert_eq!(changed.status(), 200, "{:?}", changed.body());
	let changed: Value = serde_json::from_slice(changed.body()).unwrap();
	assert_eq!(changed["is_verified"], false);
	assert_eq!(changed["is_active"], false);
	assert_ne!(changed["verification_value"], body["verification_value"]);
	let active:bool=sqlx::query_scalar("INSERT INTO tenant_domains (tenant_id,domain,is_active,is_verified) VALUES ($1,'legacy.example.com',true,false) RETURNING is_active").bind(tenant).fetch_one(db.pool()).await.unwrap();
	assert!(!active, "legacy inserts cannot bypass DNS verification");
}

#[tokio::test]
#[serial]
async fn quality_retains_legacy_pending_rows_without_stored_keys() {
	let db = TestDb::start().await;
	let tenant = insert_tenant(db.pool(), "legacy-quality", Some(100), 0).await;
	sqlx::query("INSERT INTO v1_task_result (tenant_id,payload,task_state,canonical_email) VALUES ($1,'{\"input\":{\"to_email\":\"Waiting@Example.com\"}}','queued',NULL)").bind(tenant).execute(db.pool()).await.unwrap();
	let key: String =
		sqlx::query_scalar("SELECT identity_key FROM verification_quality_rows WHERE tenant_id=$1")
			.bind(tenant)
			.fetch_one(db.pool())
			.await
			.unwrap();
	assert_eq!(key, "waiting@example.com");
}
