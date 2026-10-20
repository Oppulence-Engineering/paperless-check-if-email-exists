mod test_helpers;

use base64::{engine::general_purpose::STANDARD, Engine};
use reacher_backend::http::create_routes;
use serde_json::{json, Value};
use serial_test::serial;
use std::sync::{
	atomic::{AtomicUsize, Ordering},
	Arc,
};
use test_helpers::{
	build_test_config, insert_api_key, insert_job, insert_scored_task, insert_suppression,
	insert_tenant, ConfigProfile, TestDb,
};
use warp::{Filter, Reply};

#[tokio::test]
#[serial]
async fn import_review_export_reuses_creation_and_rechecks_current_tenant_policy() {
	let db = TestDb::start().await;
	let tenant = insert_tenant(db.pool(), "research-owner", None, 0).await;
	let other = insert_tenant(db.pool(), "research-other", None, 0).await;
	let (key, _) = insert_api_key(db.pool(), tenant).await;
	let (other_key, _) = insert_api_key(db.pool(), other).await;
	let job = insert_job(db.pool(), Some(tenant), 1, "completed").await;
	let verification = json!({"input":"jane@acme.com", "external_verification":{"service":"millionverifier","result":"ok"},"recommendation":{"action":"send"},"score":{"score":100,"category":"valid","sub_reason":"deliverable","safe_to_send":true,"reason_codes":[],"signals":{"valid_syntax":true,"reachable":"safe","has_mx_records":true,"smtp_error":false,"smtp_can_connect":false,"smtp_is_deliverable":false,"smtp_is_disabled":false,"smtp_is_catch_all":false,"smtp_has_full_inbox":false,"is_disposable":false,"is_role_account":false,"is_spam_trap_domain":false,"is_free_provider":false,"has_domain_suggestion":false}}});
	let task = insert_scored_task(
		db.pool(),
		job,
		Some(tenant),
		"jane@acme.com",
		None,
		Some(verification),
		"completed",
		Some(100),
		Some("valid"),
		Some("deliverable"),
		Some(true),
		Some(vec![]),
		Some("jane@acme.com"),
		false,
	)
	.await;
	let requests = Arc::new(AtomicUsize::new(0));
	let created = requests.clone();
	let upstream = warp::path::full().and(warp::method()).and(warp::header::<String>("x-api-key")).and(warp::body::bytes()).map(move |path:warp::path::FullPath, method:warp::http::Method, api_key:String, body:bytes::Bytes| {
        let body:Value = serde_json::from_slice(&body).unwrap_or(json!({}));
        let path = path.as_str();
        if path.ends_with("/internal/verifier-workspace") {
            assert_eq!(api_key, "bootstrap-test");
            let workspace = if body["verifierTenantId"] == tenant.to_string() { "44" } else { "99" };
            return warp::reply::json(&json!({"organizationId":"11","tenantId":"22","accountId":"33","workspaceId":workspace,"workspaceApiKey":format!("workspace-{workspace}")})).into_response();
        }
        if path.contains("/workspaces/99/") { return warp::reply::with_status("not found",warp::http::StatusCode::NOT_FOUND).into_response(); }
        assert_eq!(api_key, "workspace-44");
        assert!(path.contains("/workspaces/44/"));
        if method == warp::http::Method::POST {
            created.fetch_add(1, Ordering::SeqCst);
            assert_eq!(body["workspaceId"], "44");
            if path.ends_with("/columns") {
                assert_eq!(body["runImmediately"], true);
                return warp::reply::json(&json!({"column":{"id":"9"}})).into_response();
            }
            assert_eq!(body["requestedSources"], json!(["user_import"]));
            assert_eq!(body["dryRun"], true);
            return warp::reply::json(&json!({"session":{"id":"42","status":"planned"}})).into_response();
        }
        if path.ends_with("/columns") {
            return warp::reply::json(&json!({"columns":[{"id":"9","columnKey":"prospect_contact","executorKind":"prospect_contact","configJson":STANDARD.encode(json!({"target_role":"CEO","max_candidates":2,"max_verifications":3}).to_string())}]})).into_response();
        }
        let evidence = json!({"status":"ready","reason":"published_person_and_matching_title_with_verified_mailbox","domain":"acme.com","candidates":[{"name":"Jane Doe","title":"CEO","email":"jane@acme.com","email_source":"published","person_status":"published","employment_status":"published_on_company_site","role_status":"title_matches","evidence_url":"https://acme.com/team","observed_at":"2026-10-20T12:00:00Z","verification":{"external_verification":{"service":"millionverifier","result":"ok"},"recommendation":{"action":"send"},"is_catch_all":false}}]});
        warp::reply::json(&json!({"session":{"id":"42","prompt":"Pilot","rowsTotal":1},"verificationRequests":1,"rows":[{"rowKey":"row-1","leadId":"7","companyName":"=Acme","cells":[{"columnKey":"prospect_contact","provider":"prospect_contact","evidenceJson":STANDARD.encode(evidence.to_string())}]}]})).into_response()
    });
	let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
	let address = listener.local_addr().unwrap();
	let server = tokio::spawn(warp::serve(upstream).incoming(listener).run());
	let mut config = build_test_config(ConfigProfile::BearerTenant, Some(db.db_url()), None).await;
	Arc::get_mut(&mut config).unwrap().research = serde_json::from_value(
		json!({"url":format!("http://{address}"),"bootstrap_api_key":"bootstrap-test"}),
	)
	.unwrap();
	let routes = create_routes(config);
	let input = json!({"name":"Pilot","target_role":"CEO","import_content":"company_name,company_domain\nAcme,acme.com","import_format":"csv","max_candidates":2,"max_verifications":3,"idempotency_key":uuid::Uuid::new_v4().to_string()});
	for _ in 0..2 {
		let response = warp::test::request()
			.method("POST")
			.path("/v1/research")
			.header("authorization", format!("Bearer {key}"))
			.json(&input)
			.reply(&routes)
			.await;
		assert_eq!(response.status(), 200, "{:?}", response.body());
	}
	assert_eq!(
		requests.load(Ordering::SeqCst),
		2,
		"one import and one column despite a duplicate submission"
	);
	let mut changed = input.clone();
	changed["max_verifications"] = json!(4);
	let response = warp::test::request()
		.method("POST")
		.path("/v1/research")
		.header("authorization", format!("Bearer {key}"))
		.json(&changed)
		.reply(&routes)
		.await;
	assert_eq!(response.status(), 409);
	for path in ["/v1/research/42", "/v1/research/42/export"] {
		let mut request = warp::test::request()
			.path(path)
			.header("authorization", format!("Bearer {other_key}"));
		if path.ends_with("export") {
			request = request
				.method("POST")
				.json(&json!({"row_keys":["row-1"],"mode":"approved"}));
		}
		let response = request.reply(&routes).await;
		assert_eq!(response.status(), 404, "{:?}", response.body());
		assert!(!String::from_utf8_lossy(response.body()).contains("jane@"));
	}
	let response = warp::test::request()
		.path("/v1/research/42")
		.header("authorization", format!("Bearer {key}"))
		.reply(&routes)
		.await;
	assert_eq!(response.status(), 200);
	let detail: Value = serde_json::from_slice(response.body()).unwrap();
	assert_eq!(detail["rows"][0]["status"], "ready");
	assert!(!detail.to_string().contains("workspace-44"));
	let approved = json!({"row_keys":["row-1"],"mode":"approved"});
	for phase in [
		"allowed",
		"suppressed",
		"stale",
		"negative_feedback",
		"new_pending",
	] {
		match phase {
			"suppressed" => {
				insert_suppression(db.pool(), tenant, "jane@acme.com", "manual").await;
			}
			"stale" => {
				sqlx::query(
					"UPDATE v1_suppression_entries SET status='inactive' WHERE tenant_id=$1",
				)
				.bind(tenant)
				.execute(db.pool())
				.await
				.unwrap();
				sqlx::query(
					"UPDATE v1_task_result SET completed_at=NOW()-INTERVAL '400 days' WHERE id=$1",
				)
				.bind(task)
				.execute(db.pool())
				.await
				.unwrap();
			}
			"negative_feedback" => {
				sqlx::query(
					"UPDATE v1_task_result SET completed_at=NOW()-INTERVAL '1 hour' WHERE id=$1",
				)
				.bind(task)
				.execute(db.pool())
				.await
				.unwrap();
				sqlx::query("INSERT INTO verification_outcomes(tenant_id,canonical_email,outcome_type,occurred_at) VALUES($1,'jane@acme.com','complaint',NOW())").bind(tenant).execute(db.pool()).await.unwrap();
			}
			"new_pending" => {
				sqlx::query("DELETE FROM verification_outcomes WHERE tenant_id=$1")
					.bind(tenant)
					.execute(db.pool())
					.await
					.unwrap();
				insert_scored_task(
					db.pool(),
					job,
					Some(tenant),
					"jane@acme.com",
					None,
					None,
					"queued",
					None,
					None,
					None,
					None,
					None,
					Some("jane@acme.com"),
					false,
				)
				.await;
			}
			_ => {}
		}
		let response = warp::test::request()
			.method("POST")
			.path("/v1/research/42/export")
			.header("authorization", format!("Bearer {key}"))
			.json(&approved)
			.reply(&routes)
			.await;
		assert_eq!(response.status(), 200, "{phase}: {:?}", response.body());
		let output: Value = serde_json::from_slice(response.body()).unwrap();
		assert_eq!(
			output["exported_rows"],
			if phase == "allowed" { 1 } else { 0 },
			"{phase}: {output}"
		);
		if phase == "allowed" {
			assert!(output["csv"].as_str().unwrap().contains("'=Acme"));
		} else {
			assert_eq!(output["excluded_row_keys"], json!(["row-1"]));
		}
	}
	let response = warp::test::request()
		.method("POST")
		.path("/v1/research/42/export")
		.header("authorization", format!("Bearer {key}"))
		.json(&json!({"row_keys":["row-1"],"mode":"review"}))
		.reply(&routes)
		.await;
	assert_eq!(response.status(), 200);
	let output: Value = serde_json::from_slice(response.body()).unwrap();
	assert_eq!(output["exported_rows"], 1);
	assert_eq!(
		requests.load(Ordering::SeqCst),
		2,
		"reads and exports do not purchase verification"
	);
	server.abort();
}
