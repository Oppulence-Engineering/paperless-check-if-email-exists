//! E2E tests for tenant/account visibility endpoints.

mod test_helpers;

#[cfg(test)]
mod me_endpoint {
	use crate::test_helpers::{insert_api_key, insert_tenant, TestDb};
	use reacher_backend::config::{BackendConfig, PostgresConfig, StorageConfig};
	use reacher_backend::http::{create_routes, REACHER_SECRET_HEADER};
	use serial_test::serial;
	use std::sync::Arc;
	use warp::http::StatusCode;
	use warp::test::request;

	async fn config_with_db() -> Arc<BackendConfig> {
		let mut config = BackendConfig::empty();
		config.header_secret = Some("test-secret".into());
		let db_url = crate::test_helpers::ensure_test_db_url().await;
		config.storage = Some(StorageConfig::Postgres(PostgresConfig {
			read_replica_url: None,
			db_url,
			extra: None,
		}));
		config.connect().await.expect("Failed to connect");
		Arc::new(config)
	}

	#[tokio::test]
	#[serial]
	async fn test_me_returns_tenant_context() {
		let db = TestDb::start().await;
		let config = config_with_db().await;
		let tenant_id = insert_tenant(db.pool(), "me-test", Some(50), 3).await;
		let (full_key, _) = insert_api_key(db.pool(), tenant_id).await;

		let resp = request()
			.path("/v1/me")
			.method("GET")
			.header("Authorization", format!("Bearer {}", full_key))
			.reply(&create_routes(config))
			.await;

		assert_eq!(resp.status(), StatusCode::OK);
		let tenant_id = tenant_id.to_string();
		let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
		assert_eq!(body["tenant_id"].as_str(), Some(tenant_id.as_str()));
		assert_eq!(body["tenant_name"].as_str(), Some("Tenant me-test"));
		assert_eq!(body["monthly_email_limit"], 50);
		assert_eq!(body["used_this_period"], 3);
		assert_eq!(body["quota_remaining"], 47);
		assert_eq!(body["quota_unlimited"], false);
	}

	#[tokio::test]
	async fn test_me_with_secret_returns_legacy() {
		let mut config = BackendConfig::empty();
		config.header_secret = Some("test-secret".into());
		let config = Arc::new(config);

		let resp = request()
			.path("/v1/me")
			.method("GET")
			.header(REACHER_SECRET_HEADER, "test-secret")
			.reply(&create_routes(config))
			.await;

		assert_eq!(resp.status(), StatusCode::OK);
		let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
		assert_eq!(body["tenant_name"], "legacy");
		assert!(body["tenant_id"].is_null());
		assert_eq!(body["quota_unlimited"], true);
	}

	#[tokio::test]
	#[serial]
	async fn test_me_with_zero_quota_is_unlimited() {
		let db = TestDb::start().await;
		let config = config_with_db().await;
		let routes = create_routes(Arc::clone(&config));
		let tenant_id = insert_tenant(db.pool(), "me-unlimited", Some(0), 3).await;
		let (full_key, _) = insert_api_key(db.pool(), tenant_id).await;

		let resp = request()
			.path("/v1/me")
			.method("GET")
			.header("Authorization", format!("Bearer {}", full_key))
			.reply(&routes)
			.await;

		assert_eq!(resp.status(), StatusCode::OK);
		let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
		let tenant_id = tenant_id.to_string();
		assert_eq!(body["tenant_id"].as_str(), Some(tenant_id.as_str()));
		assert_eq!(body["quota_unlimited"], true);
		assert!(body["quota_remaining"].is_null());
	}

	#[tokio::test]
	#[serial]
	async fn test_me_api_keys_crud() {
		let db = TestDb::start().await;
		let config = config_with_db().await;
		let routes = create_routes(Arc::clone(&config));
		let tenant_id = insert_tenant(db.pool(), "me-api-keys", Some(100), 0).await;
		let (full_key, _) = insert_api_key(db.pool(), tenant_id).await;
		let auth = format!("Bearer {}", full_key);

		let list_resp = request()
			.path("/v1/me/api-keys")
			.method("GET")
			.header("Authorization", &auth)
			.reply(&routes)
			.await;
		assert_eq!(list_resp.status(), StatusCode::OK);
		let list_body: serde_json::Value = serde_json::from_slice(list_resp.body()).unwrap();
		let list = list_body["api_keys"].as_array().unwrap();
		assert!(list.len() >= 1);

		let create_resp = request()
			.path("/v1/me/api-keys")
			.method("POST")
			.header("Authorization", &auth)
			.json(&serde_json::json!({
				"name": "created-by-me",
				"scopes": ["check"]
			}))
			.reply(&routes)
			.await;
		assert_eq!(create_resp.status(), StatusCode::CREATED);
		let created: serde_json::Value = serde_json::from_slice(create_resp.body()).unwrap();
		let created_id = created["id"].as_str().unwrap().to_string();
		assert_eq!(created["name"], "created-by-me");
		assert!(created["key_prefix"].as_str().is_some());
		assert!(created["key"].as_str().is_some());
		assert_eq!(created["scopes"][0], "check");

		let list_after_resp = request()
			.path("/v1/me/api-keys")
			.method("GET")
			.header("Authorization", &auth)
			.reply(&routes)
			.await;
		assert_eq!(list_after_resp.status(), StatusCode::OK);
		let list_after: serde_json::Value = serde_json::from_slice(list_after_resp.body()).unwrap();
		let list_after_keys = list_after["api_keys"]
			.as_array()
			.unwrap()
			.iter()
			.map(|x| x["id"].as_str().unwrap().to_string())
			.collect::<std::collections::HashSet<_>>();
		assert!(list_after_keys.contains(&created_id));

		let delete_resp = request()
			.path(&format!("/v1/me/api-keys/{}", created_id))
			.method("DELETE")
			.header("Authorization", &auth)
			.reply(&routes)
			.await;
		assert_eq!(delete_resp.status(), StatusCode::OK);

		let revoked_body: serde_json::Value = serde_json::from_slice(delete_resp.body()).unwrap();
		assert_eq!(revoked_body["revoked"], true);
	}

	#[tokio::test]
	#[serial]
	async fn test_me_api_key_get_and_update() {
		let db = TestDb::start().await;
		let config = config_with_db().await;
		let routes = create_routes(Arc::clone(&config));
		let tenant_id = insert_tenant(db.pool(), "me-api-key-single", Some(100), 0).await;
		let (full_key, key_id) = insert_api_key(db.pool(), tenant_id).await;
		let auth = format!("Bearer {}", full_key);
		let key_id = key_id.to_string();

		let get_resp = request()
			.path(&format!("/v1/me/api-keys/{}", key_id))
			.method("GET")
			.header("Authorization", &auth)
			.reply(&routes)
			.await;
		assert_eq!(get_resp.status(), StatusCode::OK);
		let got: serde_json::Value = serde_json::from_slice(get_resp.body()).unwrap();
		assert_eq!(got["id"], key_id);
		assert_eq!(got["name"], "test-key");

		let patch_resp = request()
			.path(&format!("/v1/me/api-keys/{}", key_id))
			.method("PATCH")
			.header("Authorization", &auth)
			.json(&serde_json::json!({
				"name": "renamed-key",
				"scopes": ["check", "admin"]
			}))
			.reply(&routes)
			.await;
		assert_eq!(patch_resp.status(), StatusCode::OK);
		let patched: serde_json::Value = serde_json::from_slice(patch_resp.body()).unwrap();
		assert_eq!(patched["id"], key_id);
		assert_eq!(patched["name"], "renamed-key");
		assert_eq!(patched["scopes"][0], "check");
		assert_eq!(patched["scopes"][1], "admin");
	}

	#[tokio::test]
	#[serial]
	async fn test_me_api_key_update_rejects_empty_payload() {
		let db = TestDb::start().await;
		let config = config_with_db().await;
		let routes = create_routes(Arc::clone(&config));
		let tenant_id = insert_tenant(db.pool(), "me-api-key-update-empty", Some(100), 0).await;
		let (full_key, key_id) = insert_api_key(db.pool(), tenant_id).await;
		let auth = format!("Bearer {}", full_key);
		let key_id = key_id.to_string();

		let patch_resp = request()
			.path(&format!("/v1/me/api-keys/{}", key_id))
			.method("PATCH")
			.header("Authorization", &auth)
			.json(&serde_json::json!({}))
			.reply(&routes)
			.await;
		assert_eq!(patch_resp.status(), StatusCode::BAD_REQUEST);
	}

	#[tokio::test]
	#[serial]
	async fn test_me_api_keys_requires_api_key() {
		let _db = TestDb::start().await;
		let config = config_with_db().await;

		let resp = request()
			.path("/v1/me/api-keys")
			.method("GET")
			.header(REACHER_SECRET_HEADER, "test-secret")
			.reply(&create_routes(config))
			.await;
		assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
	}
}

#[cfg(test)]
mod admin_tenant_quota {
	use crate::test_helpers::TestDb;
	use reacher_backend::config::{BackendConfig, PostgresConfig, StorageConfig};
	use reacher_backend::http::{create_routes, REACHER_SECRET_HEADER};
	use serial_test::serial;
	use std::sync::Arc;
	use warp::http::StatusCode;
	use warp::test::request;

	async fn admin_config() -> Arc<BackendConfig> {
		let mut config = BackendConfig::empty();
		config.header_secret = Some("admin-secret".into());
		let db_url = crate::test_helpers::ensure_test_db_url().await;
		config.storage = Some(StorageConfig::Postgres(PostgresConfig {
			read_replica_url: None,
			db_url,
			extra: None,
		}));
		config.connect().await.expect("Failed to connect");
		Arc::new(config)
	}

	#[tokio::test]
	#[serial]
	async fn test_get_tenant_quota() {
		let db = TestDb::start().await;
		let config = admin_config().await;
		let routes = create_routes(Arc::clone(&config));
		let tenant_id: uuid::Uuid = sqlx::query_scalar(
			"INSERT INTO tenants (name, slug, contact_email, monthly_email_limit, used_this_period, status, result_retention_days) VALUES ($1, $2, $3, $4, $5, 'active', 30) RETURNING id",
		)
		.bind("QuotaTenant")
		.bind("quota-tenant-test")
		.bind("admin@example.com")
		.bind(25i32)
		.bind(9i32)
		.fetch_one(db.pool())
		.await
		.unwrap();

		let resp = request()
			.path(&format!("/v1/admin/tenants/{}/quota", tenant_id))
			.method("GET")
			.header(REACHER_SECRET_HEADER, "admin-secret")
			.reply(&routes)
			.await;

		assert_eq!(resp.status(), StatusCode::OK);
		let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
		assert_eq!(body["monthly_email_limit"], 25);
		assert_eq!(body["used_this_period"], 9);
		assert_eq!(body["remaining_quota"], 16);
		assert_eq!(body["quota_unlimited"], false);
		assert_eq!(body["name"], "QuotaTenant");
	}

	#[tokio::test]
	#[serial]
	async fn test_get_tenant_quota_returns_404_for_missing_tenant() {
		let config = admin_config().await;
		let resp = request()
			.path("/v1/admin/tenants/00000000-0000-0000-0000-000000000000/quota")
			.method("GET")
			.header(REACHER_SECRET_HEADER, "admin-secret")
			.reply(&create_routes(config))
			.await;
		assert_eq!(resp.status(), StatusCode::NOT_FOUND);
	}

	#[tokio::test]
	#[serial]
	async fn test_reset_tenant_quota() {
		let db = TestDb::start().await;
		let config = admin_config().await;
		let routes = create_routes(Arc::clone(&config));
		let tenant_id: uuid::Uuid = sqlx::query_scalar(
			"INSERT INTO tenants (name, slug, contact_email, monthly_email_limit, used_this_period, status, result_retention_days) VALUES ($1, $2, $3, $4, $5, 'active', 30) RETURNING id",
		)
		.bind("ResetQuotaTenant")
		.bind("reset-quota-tenant")
		.bind("admin@example.com")
		.bind(25i32)
		.bind(11i32)
		.fetch_one(db.pool())
		.await
		.unwrap();

		let resp = request()
			.path(&format!("/v1/admin/tenants/{}/quota/reset", tenant_id))
			.method("POST")
			.header(REACHER_SECRET_HEADER, "admin-secret")
			.reply(&routes)
			.await;

		assert_eq!(resp.status(), StatusCode::OK);
		let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
		let expected_tenant_id = tenant_id.to_string();
		assert_eq!(
			body["tenant_id"].as_str(),
			Some(expected_tenant_id.as_str())
		);
		assert_eq!(body["used_this_period"], 0);
		assert_eq!(body["monthly_email_limit"], 25);
		assert_eq!(body["remaining_quota"], 25);
	}

	#[tokio::test]
	#[serial]
	async fn test_update_tenant_quota_to_finite_and_zero() {
		let db = TestDb::start().await;
		let config = admin_config().await;
		let routes = create_routes(Arc::clone(&config));
		let tenant_id: uuid::Uuid = sqlx::query_scalar(
			"INSERT INTO tenants (name, slug, contact_email, monthly_email_limit, used_this_period, status, result_retention_days) VALUES ($1, $2, $3, $4, $5, 'active', 30) RETURNING id",
		)
		.bind("UpdateQuotaTenant")
		.bind("update-quota-tenant")
		.bind("admin@example.com")
		.bind(25i32)
		.bind(2i32)
		.fetch_one(db.pool())
		.await
		.unwrap();

		let finite_resp = request()
			.path(&format!("/v1/admin/tenants/{}/quota", tenant_id))
			.method("PATCH")
			.header(REACHER_SECRET_HEADER, "admin-secret")
			.json(&serde_json::json!({"monthly_email_limit": 15}))
			.reply(&routes)
			.await;
		assert_eq!(finite_resp.status(), StatusCode::OK);
		let finite_body: serde_json::Value = serde_json::from_slice(finite_resp.body()).unwrap();
		assert_eq!(finite_body["monthly_email_limit"], 15);
		assert_eq!(finite_body["remaining_quota"], 13);

		let unlimited_resp = request()
			.path(&format!("/v1/admin/tenants/{}/quota", tenant_id))
			.method("PATCH")
			.header(REACHER_SECRET_HEADER, "admin-secret")
			.json(&serde_json::json!({"monthly_email_limit": 0}))
			.reply(&routes)
			.await;
		assert_eq!(unlimited_resp.status(), StatusCode::OK);
		let unlimited_body: serde_json::Value =
			serde_json::from_slice(unlimited_resp.body()).unwrap();
		assert_eq!(unlimited_body["monthly_email_limit"], 0);
		assert!(unlimited_body["quota_unlimited"].as_bool().unwrap());
		assert!(unlimited_body["remaining_quota"].is_null());
	}

	#[tokio::test]
	#[serial]
	async fn test_update_tenant_quota_to_null_is_allowed() {
		let db = TestDb::start().await;
		let config = admin_config().await;
		let routes = create_routes(Arc::clone(&config));
		let tenant_id: uuid::Uuid = sqlx::query_scalar(
			"INSERT INTO tenants (name, slug, contact_email, monthly_email_limit, used_this_period, status, result_retention_days) VALUES ($1, $2, $3, $4, $5, 'active', 30) RETURNING id",
		)
		.bind("UpdateQuotaTenantNull")
		.bind("update-quota-tenant-null")
		.bind("admin@example.com")
		.bind(10i32)
		.bind(3i32)
		.fetch_one(db.pool())
		.await
		.unwrap();

		let resp = request()
			.path(&format!("/v1/admin/tenants/{}/quota", tenant_id))
			.method("PATCH")
			.header(REACHER_SECRET_HEADER, "admin-secret")
			.json(&serde_json::json!({"monthly_email_limit": null}))
			.reply(&routes)
			.await;

		assert_eq!(resp.status(), StatusCode::OK);
		let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
		assert!(body["monthly_email_limit"].is_null());
		assert_eq!(body["remaining_quota"], serde_json::Value::Null);
		assert!(body["quota_unlimited"].as_bool().unwrap());
	}

	#[tokio::test]
	#[serial]
	async fn test_update_tenant_quota_requires_payload() {
		let _db = TestDb::start().await;
		let config = admin_config().await;
		let resp = request()
			.path("/v1/admin/tenants/00000000-0000-0000-0000-000000000000/quota")
			.method("PATCH")
			.header(REACHER_SECRET_HEADER, "admin-secret")
			.json(&serde_json::json!({}))
			.reply(&create_routes(config))
			.await;
		assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
	}
}
