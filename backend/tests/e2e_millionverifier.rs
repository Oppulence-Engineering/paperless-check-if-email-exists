mod test_helpers;

use reacher_backend::{
	http::{
		shared::verification_operation::{admit_operation, read_operation},
		CheckEmailRequest,
	},
	tenant::context::TenantContext,
	verification::VerificationBackend,
	worker::{do_work::CheckEmailTask, lifecycle::execute_task},
};
use serde_json::json;
use serial_test::serial;
use std::sync::Arc;
use test_helpers::{build_test_config, insert_tenant, test_db_url, ConfigProfile, TestDb};

#[tokio::test]
#[serial]
#[ignore = "Uses live provider credits; requires an account key and explicitly supplied test addresses"]
async fn live_mailboxes_complete_through_the_durable_worker() {
	let key = std::env::var("RCH__MILLIONVERIFIER__API_KEY").expect("live key required");
	let input = std::env::var("MILLIONVERIFIER_LIVE_EMAILS").expect("test addresses required");
	let emails: Vec<_> = input.split(',').map(str::trim).collect();
	assert!((1..=2).contains(&emails.len()), "at most two live checks");
	assert!(emails.iter().all(|email| !email.is_empty()));
	assert!(
		std::env::var_os("TEST_DATABASE_URL").is_none()
			&& std::env::var_os("USE_LOCAL_TEST_INFRA").is_none(),
		"live smoke test must use an isolated test database"
	);
	let report_path =
		std::env::var("MILLIONVERIFIER_LIVE_REPORT").expect("local result report path required");
	assert!(
		!std::path::Path::new(&report_path).exists(),
		"preserve the previous run and avoid accidental paid reruns"
	);
	let db = TestDb::start().await;
	let tenant = insert_tenant(db.pool(), "millionverifier-live", Some(2), 0).await;
	let config = build_test_config(ConfigProfile::DbOnly, Some(&test_db_url()), None).await;
	let mut config = Arc::try_unwrap(config).unwrap();
	config.verification_backend = VerificationBackend::Millionverifier;
	config.millionverifier.api_key = Some(key);
	config
		.millionverifier
		.validate(config.verification_backend)
		.unwrap();
	let config = Arc::new(config);
	let mut ctx = TenantContext::legacy(config.throttle.clone());
	ctx.tenant_id = Some(tenant);
	ctx.is_legacy = false;
	let mut results = Vec::new();
	for email in &emails {
		let (operation, dispatch) = admit_operation(
			Arc::clone(&config),
			db.pool(),
			&CheckEmailRequest {
				to_email: (*email).into(),
				..Default::default()
			},
			&ctx,
			None,
			None,
		)
		.await
		.unwrap();
		let payload: serde_json::Value =
			sqlx::query_scalar("SELECT payload FROM verification_dispatch WHERE id = $1")
				.bind(dispatch)
				.fetch_one(db.pool())
				.await
				.unwrap();
		let task: CheckEmailTask = serde_json::from_value(payload).unwrap();
		execute_task(&task, Arc::clone(&config)).await.unwrap();
		execute_task(&task, Arc::clone(&config)).await.unwrap();
		let response = read_operation(db.pool(), operation, &ctx).await.unwrap();
		let body: serde_json::Value = serde_json::from_slice(&response.body).unwrap();
		let provider: Option<serde_json::Value> = sqlx::query_scalar(
			"SELECT response FROM verification_provider_attempts WHERE dispatch_id = $1",
		)
		.bind(dispatch)
		.fetch_one(db.pool())
		.await
		.unwrap();
		let result = json!({
			"email": email, "http_status": response.status_code.as_u16(),
			"provider": provider, "application": body
		});
		println!("{}", serde_json::to_string(&result).unwrap());
		results.push(result);
		std::fs::write(&report_path, serde_json::to_vec_pretty(&results).unwrap()).unwrap();
		assert_eq!(response.status_code, 200, "application result: {body}");
		assert_eq!(body["external_verification"]["service"], "millionverifier");
		assert_eq!(results.last().unwrap()["provider"]["livemode"], true);
	}
	let used: i32 = sqlx::query_scalar("SELECT used_this_period FROM tenants WHERE id = $1")
		.bind(tenant)
		.fetch_one(db.pool())
		.await
		.unwrap();
	assert_eq!(used, emails.len() as i32);
	let attempts: i64 = sqlx::query_scalar(
		"SELECT COUNT(*) FROM verification_provider_attempts a
		 JOIN v1_task_result t ON t.id = a.task_id WHERE t.tenant_id = $1",
	)
	.bind(tenant)
	.fetch_one(db.pool())
	.await
	.unwrap();
	assert_eq!(attempts, emails.len() as i64);
}

#[tokio::test]
#[serial]
async fn recorded_provider_result_resumes_without_spending_again_and_bills_once() {
	let db = TestDb::start().await;
	let tenant = insert_tenant(db.pool(), "millionverifier-resume", Some(100), 0).await;
	let config = build_test_config(ConfigProfile::DbOnly, Some(&test_db_url()), None).await;
	let mut config = Arc::try_unwrap(config).expect("unshared test configuration");
	config.verification_backend = VerificationBackend::Millionverifier;
	config.millionverifier.api_key = Some("test-placeholder-never-sent".into());
	let config = Arc::new(config);
	let mut ctx = TenantContext::legacy(config.throttle.clone());
	ctx.tenant_id = Some(tenant);
	ctx.is_legacy = false;
	let (operation, dispatch) = admit_operation(
		Arc::clone(&config),
		db.pool(),
		&CheckEmailRequest {
			to_email: "person@example.com".into(),
			..Default::default()
		},
		&ctx,
		None,
		None,
	)
	.await
	.unwrap();
	let payload: serde_json::Value =
		sqlx::query_scalar("SELECT payload FROM verification_dispatch WHERE id = $1")
			.bind(dispatch)
			.fetch_one(db.pool())
			.await
			.unwrap();
	let task: CheckEmailTask = serde_json::from_value(payload.clone()).unwrap();
	assert_eq!(
		task.verification_backend,
		VerificationBackend::Millionverifier
	);
	assert!(!payload.to_string().contains("test-placeholder-never-sent"));
	let id = task.metadata.as_ref().unwrap().task_db_id.unwrap();
	sqlx::query("INSERT INTO verification_provider_attempts (dispatch_id, task_id, response) VALUES ($1, $2, $3)")
		.bind(dispatch).bind(id).bind(json!({
			"email": "person@example.com", "quality": "risky", "result": "catch_all",
			"resultcode": 2, "subresult": "ok", "free": false, "role": false,
			"didyoumean": "", "error": "", "livemode": true
		})).execute(db.pool()).await.unwrap();
	execute_task(&task, Arc::clone(&config)).await.unwrap();
	execute_task(&task, Arc::clone(&config)).await.unwrap();
	let response = read_operation(db.pool(), operation, &ctx).await.unwrap();
	assert_eq!(response.status_code, 200);
	let body: serde_json::Value = serde_json::from_slice(&response.body).unwrap();
	assert_eq!(body["score"]["category"], "risky");
	assert_eq!(body["score"]["safe_to_send"], false);
	assert_eq!(body["external_verification"]["result"], "catch_all");
	let used: i32 = sqlx::query_scalar("SELECT used_this_period FROM tenants WHERE id = $1")
		.bind(tenant)
		.fetch_one(db.pool())
		.await
		.unwrap();
	assert_eq!(used, 1);
	let attempts: i64 = sqlx::query_scalar(
		"SELECT COUNT(*) FROM verification_provider_attempts WHERE task_id = $1",
	)
	.bind(id)
	.fetch_one(db.pool())
	.await
	.unwrap();
	assert_eq!(attempts, 1);
	let rechecks: i64 = sqlx::query_scalar(
		"SELECT COUNT(*) FROM verification_delayed_rechecks WHERE task_result_id = $1",
	)
	.bind(id)
	.fetch_one(db.pool())
	.await
	.unwrap();
	assert_eq!(rechecks, 0);
}

#[tokio::test]
#[serial]
async fn interrupted_paid_attempt_fails_without_retry_and_refunds_customer_quota() {
	let db = TestDb::start().await;
	let tenant = insert_tenant(db.pool(), "millionverifier-ambiguous", Some(100), 0).await;
	let config = build_test_config(ConfigProfile::DbOnly, Some(&test_db_url()), None).await;
	let mut config = Arc::try_unwrap(config).unwrap();
	config.verification_backend = VerificationBackend::Millionverifier;
	config.millionverifier.api_key = Some("test-placeholder-never-sent".into());
	let config = Arc::new(config);
	let mut ctx = TenantContext::legacy(config.throttle.clone());
	ctx.tenant_id = Some(tenant);
	ctx.is_legacy = false;
	let (operation, dispatch) = admit_operation(
		Arc::clone(&config),
		db.pool(),
		&CheckEmailRequest {
			to_email: "person@example.com".into(),
			..Default::default()
		},
		&ctx,
		None,
		None,
	)
	.await
	.unwrap();
	let payload: serde_json::Value =
		sqlx::query_scalar("SELECT payload FROM verification_dispatch WHERE id = $1")
			.bind(dispatch)
			.fetch_one(db.pool())
			.await
			.unwrap();
	let task: CheckEmailTask = serde_json::from_value(payload).unwrap();
	let id = task.metadata.as_ref().unwrap().task_db_id.unwrap();
	sqlx::query(
		"INSERT INTO verification_provider_attempts (dispatch_id, task_id) VALUES ($1, $2)",
	)
	.bind(dispatch)
	.bind(id)
	.execute(db.pool())
	.await
	.unwrap();
	execute_task(&task, Arc::clone(&config)).await.unwrap();
	let response = read_operation(db.pool(), operation, &ctx).await.unwrap();
	assert_eq!(response.status_code, 500);
	let body: serde_json::Value = serde_json::from_slice(&response.body).unwrap();
	assert!(body["error"]
		.as_str()
		.unwrap()
		.contains("no automatic paid retry"));
	let state: (String, i32, bool) = sqlx::query_as(
		"SELECT task_state::TEXT, retry_count, completion_done FROM v1_task_result WHERE id = $1",
	)
	.bind(id)
	.fetch_one(db.pool())
	.await
	.unwrap();
	assert_eq!(state, ("dead_lettered".into(), 0, true));
	let used: i32 = sqlx::query_scalar("SELECT used_this_period FROM tenants WHERE id = $1")
		.bind(tenant)
		.fetch_one(db.pool())
		.await
		.unwrap();
	assert_eq!(used, 0);
}
