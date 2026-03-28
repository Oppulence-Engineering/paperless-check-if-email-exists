/// E2E HTTP tests that use a real Postgres database via warp::test.
/// These exercise the full handler chain including resolve_tenant with API keys,
/// readiness checks with real DB, and storage writes.
mod test_helpers;

#[cfg(test)]
mod tests {
	use crate::test_helpers::{insert_api_key, insert_tenant, TestDb};
	use reacher_backend::config::{BackendConfig, PostgresConfig, StorageConfig};
	use reacher_backend::http::{create_routes, CheckEmailRequest, REACHER_SECRET_HEADER};
	use serial_test::serial;
	use sqlx::Row;
	use std::sync::Arc;
	use warp::http::StatusCode;
	use warp::test::request;

	/// Create a BackendConfig connected to the test database.
	async fn config_with_db(db_url: &str) -> Arc<BackendConfig> {
		let mut config = BackendConfig::empty();
		config.header_secret = Some("test-secret".to_string());

		config.storage = Some(StorageConfig::Postgres(PostgresConfig {
			read_replica_url: None,
			db_url: db_url.to_string(),
			extra: None,
		}));

		// Connect to populate internal state
		config.connect().await.expect("Failed to connect config");

		Arc::new(config)
	}

	#[tokio::test]
	#[serial]
	async fn test_readyz_with_real_postgres_returns_ok() {
		let db = TestDb::start().await;
		let db_url = db.db_url().to_string();
		db.pool().close().await;
		let config = config_with_db(&db_url).await;

		let resp = request()
			.path("/readyz")
			.method("GET")
			.reply(&create_routes(config))
			.await;

		assert_eq!(resp.status(), StatusCode::OK);
		let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
		assert_eq!(body["status"], "ok");
		assert_eq!(body["checks"]["postgres"]["status"], "ok");
		assert!(body["checks"]["postgres"]["latency_ms"].is_number());
	}

	#[tokio::test]
	#[serial]
	async fn test_v1_check_email_with_bearer_api_key() {
		let db = TestDb::start().await;
		let tenant_id = insert_tenant(db.pool(), "http-bearer-test", None, 0).await;
		let (full_key, _) = insert_api_key(db.pool(), tenant_id).await;
		let db_url = db.db_url().to_string();
		db.pool().close().await;
		let config = config_with_db(&db_url).await;

		let resp = request()
			.path("/v1/check_email")
			.method("POST")
			.header("Authorization", format!("Bearer {}", full_key))
			.json(&serde_json::from_str::<CheckEmailRequest>(r#"{"to_email": "foo@bar"}"#).unwrap())
			.reply(&create_routes(config))
			.await;

		assert_eq!(resp.status(), StatusCode::OK, "{:?}", resp.body());
		let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
		assert!(body["is_reachable"].is_string());
		assert!(body["score"]["score"].is_number());
		assert!(body["score"]["category"].is_string());
	}

	#[tokio::test]
	#[serial]
	async fn test_v0_check_email_with_bearer_api_key() {
		let db = TestDb::start().await;
		let tenant_id = insert_tenant(db.pool(), "http-bearer-v0", None, 0).await;
		let (full_key, _) = insert_api_key(db.pool(), tenant_id).await;
		let db_url = db.db_url().to_string();
		db.pool().close().await;
		let config = config_with_db(&db_url).await;

		let resp = request()
			.path("/v0/check_email")
			.method("POST")
			.header("Authorization", format!("Bearer {}", full_key))
			.json(&serde_json::from_str::<CheckEmailRequest>(r#"{"to_email": "foo@bar"}"#).unwrap())
			.reply(&create_routes(config))
			.await;

		assert_eq!(resp.status(), StatusCode::OK);
		// Should have deprecation headers too
		assert_eq!(resp.headers().get("Deprecation").unwrap(), "true");
	}

	#[tokio::test]
	#[serial]
	async fn test_bearer_invalid_key_returns_401() {
		let db = TestDb::start().await;
		let db_url = db.db_url().to_string();
		db.pool().close().await;
		let config = config_with_db(&db_url).await;

		let resp = request()
			.path("/v1/check_email")
			.method("POST")
			.header(
				"Authorization",
				"Bearer rch_live_0000000000000000000000000000dead",
			)
			.json(&serde_json::from_str::<CheckEmailRequest>(r#"{"to_email": "foo@bar"}"#).unwrap())
			.reply(&create_routes(config))
			.await;

		assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
	}

	#[tokio::test]
	#[serial]
	async fn test_legacy_secret_still_works_with_db() {
		let db = TestDb::start().await;
		let db_url = db.db_url().to_string();
		db.pool().close().await;
		let config = config_with_db(&db_url).await;

		let resp = request()
			.path("/v1/check_email")
			.method("POST")
			.header(REACHER_SECRET_HEADER, "test-secret")
			.json(&serde_json::from_str::<CheckEmailRequest>(r#"{"to_email": "foo@bar"}"#).unwrap())
			.reply(&create_routes(config))
			.await;

		assert_eq!(resp.status(), StatusCode::OK);
	}

	#[tokio::test]
	#[serial]
	async fn test_check_email_stores_result_in_db() {
		let db = TestDb::start().await;
		let before: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM v1_task_result")
			.fetch_one(db.pool())
			.await
			.unwrap();
		let db_url = db.db_url().to_string();
		db.pool().close().await;
		let config = config_with_db(&db_url).await;

		let _resp = request()
			.path("/v1/check_email")
			.method("POST")
			.header(REACHER_SECRET_HEADER, "test-secret")
			.json(&serde_json::from_str::<CheckEmailRequest>(r#"{"to_email": "foo@bar"}"#).unwrap())
			.reply(&create_routes(config))
			.await;

		let verify_pool = sqlx::PgPool::connect(&db_url).await.unwrap();

		// Count rows after — should have one more
		let after: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM v1_task_result")
			.fetch_one(&verify_pool)
			.await
			.unwrap();

		assert_eq!(after, before + 1, "Should have stored one result");

		let row = sqlx::query(
			"SELECT result, score, score_category, sub_reason FROM v1_task_result ORDER BY id DESC LIMIT 1",
		)
		.fetch_one(&verify_pool)
		.await
		.unwrap();

		let result: serde_json::Value = row.get("result");
		let score: Option<i16> = row.get("score");
		let score_category: Option<String> = row.get("score_category");
		let sub_reason: Option<String> = row.get("sub_reason");

		assert!(result["score"].is_object());
		assert_eq!(
			result["score"]["score"].as_i64(),
			score.map(i64::from),
			"stored column score should match stored result payload"
		);
		assert_eq!(
			result["score"]["category"].as_str(),
			score_category.as_deref(),
			"stored score_category should match stored result payload"
		);
		assert_eq!(
			result["score"]["sub_reason"].as_str(),
			sub_reason.as_deref(),
			"stored sub_reason should match stored result payload"
		);
	}

	#[tokio::test]
	#[serial]
	async fn test_healthz_still_works_with_db_config() {
		let db = TestDb::start().await;
		let db_url = db.db_url().to_string();
		db.pool().close().await;
		let config = config_with_db(&db_url).await;

		let resp = request()
			.path("/healthz")
			.method("GET")
			.reply(&create_routes(config))
			.await;

		assert_eq!(resp.status(), StatusCode::OK);
	}
}
