mod test_helpers;

#[cfg(test)]
mod tests {
	use crate::test_helpers::TestDb;
	use reacher_backend::config::{BackendConfig, PostgresConfig, StorageConfig};
	use reacher_backend::http::create_routes;
	use serial_test::serial;
	use std::sync::Arc;
	use warp::http::StatusCode;
	use warp::test::request;

	async fn cfg() -> Arc<BackendConfig> {
		let mut c = BackendConfig::empty();
		c.header_secret = Some("s".into());
		let db = crate::test_helpers::ensure_test_db_url().await;
		c.storage = Some(StorageConfig::Postgres(PostgresConfig {
			read_replica_url: None,
			db_url: db,
			extra: None,
		}));
		c.connect().await.unwrap();
		Arc::new(c)
	}

	#[tokio::test]
	#[serial]
	async fn test_onboard_creates_tenant_key_and_verifies() {
		let _db = TestDb::start().await;
		let config = cfg().await;

		let resp = request()
			.path("/v1/check-email-with-onboard")
			.method("POST")
			.json(&serde_json::json!({
				"email_to_verify": "foo@bar",
				"tenant_name": "Onboard Test Corp",
				"contact_email": "admin@onboard.test"
			}))
			.reply(&create_routes(config))
			.await;

		assert_eq!(
			resp.status(),
			StatusCode::CREATED,
			"body: {:?}",
			resp.body()
		);
		let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();

		// Tenant created
		assert!(body["tenant"]["id"].is_string());
		assert_eq!(body["tenant"]["name"], "Onboard Test Corp");
		assert_eq!(body["tenant"]["slug"], "onboard-test-corp");
		assert_eq!(body["tenant"]["plan_tier"], "free");
		assert_eq!(body["tenant"]["status"], "active");

		// API key created
		assert!(body["api_key"]["key"]
			.as_str()
			.unwrap()
			.starts_with("rch_live_"));
		assert!(body["api_key"]["id"].is_string());
		assert_eq!(body["api_key"]["name"], "Default");

		// Verification ran
		assert!(body["verification_result"]["is_reachable"].is_string());
		assert_eq!(body["verification_result"]["input"], "foo@bar");
	}

	#[tokio::test]
	#[serial]
	async fn test_onboard_returned_key_works_for_auth() {
		let _db = TestDb::start().await;
		let config = cfg().await;
		let routes = create_routes(Arc::clone(&config));

		// Onboard
		let resp = request()
			.path("/v1/check-email-with-onboard")
			.method("POST")
			.json(&serde_json::json!({
				"email_to_verify": "test@bar",
				"tenant_name": "Auth Test Corp",
				"contact_email": "auth@test.com"
			}))
			.reply(&routes)
			.await;

		assert_eq!(resp.status(), StatusCode::CREATED);
		let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
		let api_key = body["api_key"]["key"].as_str().unwrap().to_string();

		// Use the returned key to call /v1/check_email
		let check_resp = request()
			.path("/v1/check_email")
			.method("POST")
			.header("Authorization", format!("Bearer {}", api_key))
			.json(&serde_json::json!({"to_email": "x@y"}))
			.reply(&routes)
			.await;

		assert_eq!(
			check_resp.status(),
			StatusCode::OK,
			"API key from onboard should work"
		);
	}

	#[tokio::test]
	#[serial]
	async fn test_onboard_custom_slug() {
		let _db = TestDb::start().await;
		let config = cfg().await;

		let resp = request()
			.path("/v1/check-email-with-onboard")
			.method("POST")
			.json(&serde_json::json!({
				"email_to_verify": "a@b",
				"tenant_name": "Custom Slug Co",
				"contact_email": "c@d.com",
				"slug": "my-custom-slug"
			}))
			.reply(&create_routes(config))
			.await;

		assert_eq!(resp.status(), StatusCode::CREATED);
		let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
		assert_eq!(body["tenant"]["slug"], "my-custom-slug");
	}

	#[tokio::test]
	#[serial]
	async fn test_onboard_custom_plan_tier() {
		let _db = TestDb::start().await;
		let config = cfg().await;

		let resp = request()
			.path("/v1/check-email-with-onboard")
			.method("POST")
			.json(&serde_json::json!({
				"email_to_verify": "a@b",
				"tenant_name": "Enterprise Co",
				"contact_email": "e@f.com",
				"plan_tier": "enterprise"
			}))
			.reply(&create_routes(config))
			.await;

		assert_eq!(resp.status(), StatusCode::CREATED);
		let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
		assert_eq!(body["tenant"]["plan_tier"], "enterprise");
	}

	#[tokio::test]
	#[serial]
	async fn test_onboard_duplicate_slug_conflict() {
		let _db = TestDb::start().await;
		let config = cfg().await;
		let routes = create_routes(Arc::clone(&config));

		// First onboard
		request()
			.path("/v1/check-email-with-onboard")
			.method("POST")
			.json(&serde_json::json!({
				"email_to_verify": "a@b",
				"tenant_name": "Duplicate Co",
				"contact_email": "dup@test.com",
				"slug": "dup-slug-test"
			}))
			.reply(&routes)
			.await;

		// Second with same slug
		let resp = request()
			.path("/v1/check-email-with-onboard")
			.method("POST")
			.json(&serde_json::json!({
				"email_to_verify": "c@d",
				"tenant_name": "Duplicate Co 2",
				"contact_email": "dup2@test.com",
				"slug": "dup-slug-test"
			}))
			.reply(&routes)
			.await;

		assert_eq!(resp.status(), StatusCode::CONFLICT);
	}

	#[tokio::test]
	#[serial]
	async fn test_onboard_missing_email() {
		let config = cfg().await;
		let resp = request()
			.path("/v1/check-email-with-onboard")
			.method("POST")
			.json(&serde_json::json!({
				"email_to_verify": "",
				"tenant_name": "X",
				"contact_email": "x@y.com"
			}))
			.reply(&create_routes(config))
			.await;

		assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
	}

	#[tokio::test]
	#[serial]
	async fn test_onboard_missing_tenant_name() {
		let config = cfg().await;
		let resp = request()
			.path("/v1/check-email-with-onboard")
			.method("POST")
			.json(&serde_json::json!({
				"email_to_verify": "a@b",
				"tenant_name": "",
				"contact_email": "x@y.com"
			}))
			.reply(&create_routes(config))
			.await;

		assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
	}

	#[tokio::test]
	#[serial]
	async fn test_onboard_missing_contact_email() {
		let config = cfg().await;
		let resp = request()
			.path("/v1/check-email-with-onboard")
			.method("POST")
			.json(&serde_json::json!({
				"email_to_verify": "a@b",
				"tenant_name": "X",
				"contact_email": ""
			}))
			.reply(&create_routes(config))
			.await;

		assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
	}

	#[tokio::test]
	#[serial]
	async fn test_onboard_no_auth_required() {
		let _db = TestDb::start().await;
		let config = cfg().await;

		// Onboard should work WITHOUT any auth header
		let resp = request()
			.path("/v1/check-email-with-onboard")
			.method("POST")
			.json(&serde_json::json!({
				"email_to_verify": "noauth@test.com",
				"tenant_name": "No Auth Co",
				"contact_email": "na@test.com"
			}))
			.reply(&create_routes(config))
			.await;

		assert_eq!(resp.status(), StatusCode::CREATED);
	}

	#[tokio::test]
	#[serial]
	async fn test_onboard_stores_result_in_db() {
		let db = TestDb::start().await;
		let config = cfg().await;

		let resp = request()
			.path("/v1/check-email-with-onboard")
			.method("POST")
			.json(&serde_json::json!({
				"email_to_verify": "stored@test.com",
				"tenant_name": "Store Test Co",
				"contact_email": "st@test.com"
			}))
			.reply(&create_routes(config))
			.await;

		assert_eq!(resp.status(), StatusCode::CREATED);
		let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
		let tenant_id: uuid::Uuid = body["tenant"]["id"].as_str().unwrap().parse().unwrap();

		// Verify result stored in DB
		let count: i64 =
			sqlx::query_scalar("SELECT COUNT(*) FROM v1_task_result WHERE tenant_id = $1")
				.bind(tenant_id)
				.fetch_one(db.pool())
				.await
				.unwrap();
		assert!(count >= 1, "Verification result should be stored in DB");
	}
}
