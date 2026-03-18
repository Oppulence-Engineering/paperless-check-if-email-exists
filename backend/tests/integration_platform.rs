// Integration tests for Phase 1 platform features.
// These test the full HTTP layer through warp::test::request().

#[cfg(test)]
mod health_integration {
	use reacher_backend::config::BackendConfig;
	use reacher_backend::http::create_routes;
	use std::sync::Arc;
	use warp::http::StatusCode;
	use warp::test::request;

	fn config() -> Arc<BackendConfig> {
		Arc::new(BackendConfig::empty())
	}

	#[tokio::test]
	async fn test_healthz_returns_200() {
		let resp = request()
			.path("/healthz")
			.method("GET")
			.reply(&create_routes(config()))
			.await;

		assert_eq!(resp.status(), StatusCode::OK);
		let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
		assert_eq!(body["status"], "ok");
	}

	#[tokio::test]
	async fn test_healthz_no_auth_needed() {
		let mut c = BackendConfig::empty();
		c.header_secret = Some("super-secret".into());
		let resp = request()
			.path("/healthz")
			.method("GET")
			// No auth headers
			.reply(&create_routes(Arc::new(c)))
			.await;

		assert_eq!(resp.status(), StatusCode::OK);
	}

	#[tokio::test]
	async fn test_readyz_returns_status_structure() {
		let resp = request()
			.path("/readyz")
			.method("GET")
			.reply(&create_routes(config()))
			.await;

		// Without Postgres, may return 200 or 503
		let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
		assert!(body["status"].is_string());
		assert!(body["checks"]["postgres"]["status"].is_string());
		assert!(body["checks"]["rabbitmq"]["status"].is_string());
	}

	#[tokio::test]
	async fn test_readyz_no_auth_needed() {
		let mut c = BackendConfig::empty();
		c.header_secret = Some("secret-123".into());
		let resp = request()
			.path("/readyz")
			.method("GET")
			.reply(&create_routes(Arc::new(c)))
			.await;

		// Should respond regardless of auth
		let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
		assert!(body["status"].is_string());
	}
}

#[cfg(test)]
mod auth_integration {
	use reacher_backend::config::BackendConfig;
	use reacher_backend::http::{create_routes, CheckEmailRequest, REACHER_SECRET_HEADER};
	use std::sync::Arc;
	use warp::http::StatusCode;
	use warp::test::request;

	fn config_with_secret(secret: &str) -> Arc<BackendConfig> {
		let mut c = BackendConfig::empty();
		c.header_secret = Some(secret.to_string());
		Arc::new(c)
	}

	fn config_open() -> Arc<BackendConfig> {
		let mut c = BackendConfig::empty();
		c.header_secret = None;
		Arc::new(c)
	}

	fn config_empty_secret() -> Arc<BackendConfig> {
		let mut c = BackendConfig::empty();
		c.header_secret = Some(String::new());
		Arc::new(c)
	}

	fn email_body() -> CheckEmailRequest {
		serde_json::from_str(r#"{"to_email": "foo@bar"}"#).unwrap()
	}

	// --- Auth resolution order tests ---

	#[tokio::test]
	async fn test_legacy_secret_accepted_on_v0() {
		let resp = request()
			.path("/v0/check_email")
			.method("POST")
			.header(REACHER_SECRET_HEADER, "my-secret")
			.json(&email_body())
			.reply(&create_routes(config_with_secret("my-secret")))
			.await;

		assert_eq!(resp.status(), StatusCode::OK);
	}

	#[tokio::test]
	async fn test_legacy_secret_accepted_on_v1() {
		let resp = request()
			.path("/v1/check_email")
			.method("POST")
			.header(REACHER_SECRET_HEADER, "my-secret")
			.json(&email_body())
			.reply(&create_routes(config_with_secret("my-secret")))
			.await;

		assert_eq!(resp.status(), StatusCode::OK);
	}

	#[tokio::test]
	async fn test_wrong_legacy_secret_rejected() {
		let resp = request()
			.path("/v1/check_email")
			.method("POST")
			.header(REACHER_SECRET_HEADER, "wrong-secret")
			.json(&email_body())
			.reply(&create_routes(config_with_secret("correct-secret")))
			.await;

		assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
	}

	#[tokio::test]
	async fn test_no_auth_with_secret_configured_rejected() {
		let resp = request()
			.path("/v1/check_email")
			.method("POST")
			.json(&email_body())
			.reply(&create_routes(config_with_secret("my-secret")))
			.await;

		assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
	}

	#[tokio::test]
	async fn test_open_mode_no_auth_allowed() {
		let resp = request()
			.path("/v1/check_email")
			.method("POST")
			.json(&email_body())
			.reply(&create_routes(config_open()))
			.await;

		assert_eq!(resp.status(), StatusCode::OK);
	}

	#[tokio::test]
	async fn test_empty_secret_is_open_mode() {
		let resp = request()
			.path("/v1/check_email")
			.method("POST")
			.json(&email_body())
			.reply(&create_routes(config_empty_secret()))
			.await;

		assert_eq!(resp.status(), StatusCode::OK);
	}

	#[tokio::test]
	async fn test_bearer_without_db_returns_503() {
		let resp = request()
			.path("/v1/check_email")
			.method("POST")
			.header(
				"Authorization",
				"Bearer rch_live_0123456789abcdef0123456789ab",
			)
			.json(&email_body())
			.reply(&create_routes(config_with_secret("unused")))
			.await;

		// API key auth requires Postgres
		assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
	}

	#[tokio::test]
	async fn test_non_rch_bearer_falls_through_to_secret() {
		// A Bearer token that doesn't start with rch_live_ should fall through
		// to the x-reacher-secret check
		let resp = request()
			.path("/v1/check_email")
			.method("POST")
			.header("Authorization", "Bearer some-other-token")
			.json(&email_body())
			.reply(&create_routes(config_with_secret("my-secret")))
			.await;

		// No x-reacher-secret header, so should be rejected
		assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
	}
}

#[cfg(test)]
mod v0_deprecation_integration {
	use reacher_backend::config::BackendConfig;
	use reacher_backend::http::{create_routes, CheckEmailRequest, REACHER_SECRET_HEADER};
	use std::sync::Arc;
	use warp::http::StatusCode;
	use warp::test::request;

	fn config() -> Arc<BackendConfig> {
		let mut c = BackendConfig::empty();
		c.header_secret = Some("test-secret".into());
		Arc::new(c)
	}

	#[tokio::test]
	async fn test_v0_check_email_has_deprecation_header() {
		let resp = request()
			.path("/v0/check_email")
			.method("POST")
			.header(REACHER_SECRET_HEADER, "test-secret")
			.json(&serde_json::from_str::<CheckEmailRequest>(r#"{"to_email": "foo@bar"}"#).unwrap())
			.reply(&create_routes(config()))
			.await;

		assert_eq!(resp.status(), StatusCode::OK);

		// Verify deprecation headers
		let headers = resp.headers();
		assert_eq!(
			headers.get("Deprecation").map(|v| v.to_str().unwrap()),
			Some("true")
		);
		assert_eq!(
			headers.get("Sunset").map(|v| v.to_str().unwrap()),
			Some("2026-09-16")
		);

		let link = headers.get("Link").unwrap().to_str().unwrap();
		assert!(link.contains("/v1/check_email"));
		assert!(link.contains("successor-version"));
	}

	#[tokio::test]
	async fn test_v1_check_email_no_deprecation_headers() {
		let resp = request()
			.path("/v1/check_email")
			.method("POST")
			.header(REACHER_SECRET_HEADER, "test-secret")
			.json(&serde_json::from_str::<CheckEmailRequest>(r#"{"to_email": "foo@bar"}"#).unwrap())
			.reply(&create_routes(config()))
			.await;

		assert_eq!(resp.status(), StatusCode::OK);

		// V1 should NOT have deprecation headers
		assert!(resp.headers().get("Deprecation").is_none());
		assert!(resp.headers().get("Sunset").is_none());
	}
}

#[cfg(test)]
mod v1_worker_mode_integration {
	use reacher_backend::config::BackendConfig;
	use reacher_backend::http::{create_routes, REACHER_SECRET_HEADER};
	use std::sync::Arc;
	use warp::http::StatusCode;
	use warp::test::request;

	#[tokio::test]
	async fn test_v1_bulk_requires_worker_mode() {
		let mut c = BackendConfig::empty();
		c.header_secret = Some("secret".into());
		// worker.enable = false by default

		let resp = request()
			.path("/v1/bulk")
			.method("POST")
			.header(REACHER_SECRET_HEADER, "secret")
			.json(&serde_json::json!({"input": ["test@example.com"]}))
			.reply(&create_routes(Arc::new(c)))
			.await;

		assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
	}

	#[tokio::test]
	async fn test_v1_bulk_progress_requires_worker_mode() {
		let mut c = BackendConfig::empty();
		c.header_secret = Some("secret".into());

		let resp = request()
			.path("/v1/bulk/1")
			.method("GET")
			.header(REACHER_SECRET_HEADER, "secret")
			.reply(&create_routes(Arc::new(c)))
			.await;

		assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
	}

	#[tokio::test]
	async fn test_v1_jobs_status_requires_worker_mode() {
		let mut c = BackendConfig::empty();
		c.header_secret = Some("secret".into());

		let resp = request()
			.path("/v1/jobs/1")
			.method("GET")
			.header(REACHER_SECRET_HEADER, "secret")
			.reply(&create_routes(Arc::new(c)))
			.await;

		assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
	}

	#[tokio::test]
	async fn test_v1_jobs_cancel_requires_worker_mode() {
		let mut c = BackendConfig::empty();
		c.header_secret = Some("secret".into());

		let resp = request()
			.path("/v1/jobs/1/cancel")
			.method("POST")
			.header(REACHER_SECRET_HEADER, "secret")
			.reply(&create_routes(Arc::new(c)))
			.await;

		assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
	}

	#[tokio::test]
	async fn test_v1_jobs_events_requires_worker_mode() {
		let mut c = BackendConfig::empty();
		c.header_secret = Some("secret".into());

		let resp = request()
			.path("/v1/jobs/1/events")
			.method("GET")
			.header(REACHER_SECRET_HEADER, "secret")
			.reply(&create_routes(Arc::new(c)))
			.await;

		assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
	}

	#[tokio::test]
	async fn test_v1_jobs_results_requires_worker_mode() {
		let mut c = BackendConfig::empty();
		c.header_secret = Some("secret".into());

		let resp = request()
			.path("/v1/jobs/1/results")
			.method("GET")
			.header(REACHER_SECRET_HEADER, "secret")
			.reply(&create_routes(Arc::new(c)))
			.await;

		assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
	}
}

#[cfg(test)]
mod content_type_integration {
	use reacher_backend::config::BackendConfig;
	use reacher_backend::http::{create_routes, CheckEmailRequest, REACHER_SECRET_HEADER};
	use std::sync::Arc;
	use warp::http::StatusCode;
	use warp::test::request;

	fn config() -> Arc<BackendConfig> {
		let mut c = BackendConfig::empty();
		c.header_secret = Some("s".into());
		Arc::new(c)
	}

	#[tokio::test]
	async fn test_v0_returns_json_content_type() {
		let resp = request()
			.path("/v0/check_email")
			.method("POST")
			.header(REACHER_SECRET_HEADER, "s")
			.json(&serde_json::from_str::<CheckEmailRequest>(r#"{"to_email": "foo@bar"}"#).unwrap())
			.reply(&create_routes(config()))
			.await;

		assert_eq!(resp.status(), StatusCode::OK);
		// Response should be valid JSON
		let _: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
	}

	#[tokio::test]
	async fn test_v1_returns_json_content_type() {
		let resp = request()
			.path("/v1/check_email")
			.method("POST")
			.header(REACHER_SECRET_HEADER, "s")
			.json(&serde_json::from_str::<CheckEmailRequest>(r#"{"to_email": "foo@bar"}"#).unwrap())
			.reply(&create_routes(config()))
			.await;

		assert_eq!(resp.status(), StatusCode::OK);
		let _: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
	}

	#[tokio::test]
	async fn test_healthz_returns_json() {
		let resp = request()
			.path("/healthz")
			.method("GET")
			.reply(&create_routes(config()))
			.await;

		assert_eq!(resp.status(), StatusCode::OK);
		let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
		assert_eq!(body["status"], "ok");
	}

	#[tokio::test]
	async fn test_error_responses_are_json() {
		let resp = request()
			.path("/v1/check_email")
			.method("POST")
			.header(REACHER_SECRET_HEADER, "s")
			.json(&serde_json::from_str::<CheckEmailRequest>(r#"{"to_email": ""}"#).unwrap())
			.reply(&create_routes(config()))
			.await;

		assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
		let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
		assert!(body["error"].is_string());
	}
}

#[cfg(test)]
mod backward_compat_integration {
	use reacher_backend::config::BackendConfig;
	use reacher_backend::http::{create_routes, CheckEmailRequest, REACHER_SECRET_HEADER};
	use std::sync::Arc;
	use warp::http::StatusCode;
	use warp::test::request;

	fn config() -> Arc<BackendConfig> {
		let mut c = BackendConfig::empty();
		c.header_secret = Some("compat-test".into());
		Arc::new(c)
	}

	#[tokio::test]
	async fn test_v0_still_works_with_legacy_auth() {
		let resp = request()
			.path("/v0/check_email")
			.method("POST")
			.header(REACHER_SECRET_HEADER, "compat-test")
			.json(
				&serde_json::from_str::<CheckEmailRequest>(r#"{"to_email": "user@example.com"}"#)
					.unwrap(),
			)
			.reply(&create_routes(config()))
			.await;

		// Should succeed (email verification returns some result)
		assert_eq!(resp.status(), StatusCode::OK);
		let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
		assert!(body["is_reachable"].is_string());
	}

	#[tokio::test]
	async fn test_v1_still_works_with_legacy_auth() {
		let resp = request()
			.path("/v1/check_email")
			.method("POST")
			.header(REACHER_SECRET_HEADER, "compat-test")
			.json(
				&serde_json::from_str::<CheckEmailRequest>(r#"{"to_email": "user@example.com"}"#)
					.unwrap(),
			)
			.reply(&create_routes(config()))
			.await;

		assert_eq!(resp.status(), StatusCode::OK);
		let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
		assert!(body["is_reachable"].is_string());
	}

	#[tokio::test]
	async fn test_both_versions_return_same_structure() {
		let config = config();

		let v0_resp = request()
			.path("/v0/check_email")
			.method("POST")
			.header(REACHER_SECRET_HEADER, "compat-test")
			.json(&serde_json::from_str::<CheckEmailRequest>(r#"{"to_email": "foo@bar"}"#).unwrap())
			.reply(&create_routes(Arc::clone(&config)))
			.await;

		let v1_resp = request()
			.path("/v1/check_email")
			.method("POST")
			.header(REACHER_SECRET_HEADER, "compat-test")
			.json(&serde_json::from_str::<CheckEmailRequest>(r#"{"to_email": "foo@bar"}"#).unwrap())
			.reply(&create_routes(config))
			.await;

		assert_eq!(v0_resp.status(), StatusCode::OK);
		assert_eq!(v1_resp.status(), StatusCode::OK);

		let v0_body: serde_json::Value = serde_json::from_slice(v0_resp.body()).unwrap();
		let v1_body: serde_json::Value = serde_json::from_slice(v1_resp.body()).unwrap();

		// Both should have the same core structure
		assert_eq!(v0_body["input"], v1_body["input"]);
		assert_eq!(v0_body["is_reachable"], v1_body["is_reachable"]);
		assert!(v0_body["syntax"].is_object());
		assert!(v1_body["syntax"].is_object());
		assert!(v0_body["mx"].is_object());
		assert!(v1_body["mx"].is_object());
		assert!(v0_body["smtp"].is_object());
		assert!(v1_body["smtp"].is_object());
		assert!(v0_body["misc"].is_object());
		assert!(v1_body["misc"].is_object());
	}
}
