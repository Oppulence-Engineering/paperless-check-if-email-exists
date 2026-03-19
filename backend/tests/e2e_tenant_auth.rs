mod test_helpers;

#[cfg(test)]
mod tests {
	use crate::test_helpers::{
		insert_api_key, insert_api_key_with_status, insert_tenant, insert_tenant_with_status,
		TestDb,
	};
	use reacher_backend::config::ThrottleConfig;
	use reacher_backend::tenant::auth::{generate_api_key, hash_api_key, resolve_from_api_key};
	use reacher_backend::tenant::models::{PlanTier, TenantStatus};
	use serial_test::serial;
	use sqlx::Row;

	#[tokio::test]
	#[serial]
	async fn test_resolve_valid_api_key() {
		let db = TestDb::start().await;
		let pool = db.pool();

		let tenant_id = insert_tenant(pool, "valid-auth", Some(10_000), 0).await;
		let (full_key, api_key_id) = insert_api_key(pool, tenant_id).await;

		let global_throttle = ThrottleConfig::new_without_throttle();
		let ctx = resolve_from_api_key(pool, &full_key, &global_throttle)
			.await
			.expect("resolve should succeed");

		assert_eq!(ctx.tenant_id, Some(tenant_id));
		assert_eq!(ctx.api_key_id, Some(api_key_id));
		assert_eq!(ctx.tenant_name, "Tenant valid-auth");
		assert_eq!(ctx.plan_tier, PlanTier::Starter);
		assert_eq!(ctx.status, TenantStatus::Active);
		assert!(!ctx.is_legacy);
		assert_eq!(ctx.monthly_email_limit, Some(10_000));
		// Default result_retention_days from the migration
		assert!(ctx.result_retention_days >= 0);
		// Throttle should fall back to global (all None)
		assert_eq!(ctx.throttle.max_requests_per_second, None);
		assert_eq!(ctx.throttle.max_requests_per_minute, None);
		assert_eq!(ctx.throttle.max_requests_per_hour, None);
		assert_eq!(ctx.throttle.max_requests_per_day, None);
	}

	#[tokio::test]
	#[serial]
	async fn test_resolve_revoked_key() {
		let db = TestDb::start().await;
		let pool = db.pool();

		let tenant_id = insert_tenant(pool, "revoked-key", None, 0).await;
		let (full_key, _) = insert_api_key_with_status(pool, tenant_id, "revoked").await;

		let global_throttle = ThrottleConfig::new_without_throttle();
		let err = resolve_from_api_key(pool, &full_key, &global_throttle)
			.await
			.expect_err("should fail for revoked key");

		let msg = err.to_string();
		assert!(
			msg.contains("revoked"),
			"error should mention 'revoked', got: {msg}"
		);
	}

	#[tokio::test]
	#[serial]
	async fn test_resolve_expired_key() {
		let db = TestDb::start().await;
		let pool = db.pool();

		let tenant_id = insert_tenant(pool, "expired-key", None, 0).await;
		let (full_key, _) = insert_api_key_with_status(pool, tenant_id, "expired").await;

		let global_throttle = ThrottleConfig::new_without_throttle();
		let err = resolve_from_api_key(pool, &full_key, &global_throttle)
			.await
			.expect_err("should fail for expired key");

		let msg = err.to_string();
		assert!(
			msg.contains("expired"),
			"error should mention 'expired', got: {msg}"
		);
	}

	#[tokio::test]
	#[serial]
	async fn test_resolve_expired_by_date() {
		let db = TestDb::start().await;
		let pool = db.pool();

		let tenant_id = insert_tenant(pool, "expired-date", None, 0).await;
		let (full_key, api_key_id) = insert_api_key(pool, tenant_id).await;

		// Set expires_at to yesterday so the key is expired by date
		sqlx::query("UPDATE api_keys SET expires_at = NOW() - INTERVAL '1 day' WHERE id = $1")
			.bind(api_key_id)
			.execute(pool)
			.await
			.expect("failed to update expires_at");

		let global_throttle = ThrottleConfig::new_without_throttle();
		let err = resolve_from_api_key(pool, &full_key, &global_throttle)
			.await
			.expect_err("should fail for date-expired key");

		let msg = err.to_string();
		assert!(
			msg.contains("expired"),
			"error should mention 'expired', got: {msg}"
		);
	}

	#[tokio::test]
	#[serial]
	async fn test_resolve_suspended_tenant() {
		let db = TestDb::start().await;
		let pool = db.pool();

		let tenant_id = insert_tenant_with_status(pool, "suspended-tenant", "suspended").await;
		let (full_key, _) = insert_api_key(pool, tenant_id).await;

		let global_throttle = ThrottleConfig::new_without_throttle();
		let err = resolve_from_api_key(pool, &full_key, &global_throttle)
			.await
			.expect_err("should fail for suspended tenant");

		let msg = err.to_string();
		assert!(
			msg.contains("suspended"),
			"error should mention 'suspended', got: {msg}"
		);
	}

	#[tokio::test]
	#[serial]
	async fn test_resolve_deactivated_tenant() {
		let db = TestDb::start().await;
		let pool = db.pool();

		let tenant_id = insert_tenant_with_status(pool, "deactivated-tenant", "deactivated").await;
		let (full_key, _) = insert_api_key(pool, tenant_id).await;

		let global_throttle = ThrottleConfig::new_without_throttle();
		let err = resolve_from_api_key(pool, &full_key, &global_throttle)
			.await
			.expect_err("should fail for deactivated tenant");

		let msg = err.to_string();
		assert!(
			msg.contains("deactivated"),
			"error should mention 'deactivated', got: {msg}"
		);
	}

	#[tokio::test]
	#[serial]
	async fn test_resolve_nonexistent_key() {
		let db = TestDb::start().await;
		let pool = db.pool();

		let global_throttle = ThrottleConfig::new_without_throttle();
		let err = resolve_from_api_key(pool, "rch_live_doesnotexist000000", &global_throttle)
			.await
			.expect_err("should fail for nonexistent key");

		let msg = err.to_string();
		assert!(
			msg.contains("Invalid API key"),
			"error should mention 'Invalid API key', got: {msg}"
		);
	}

	#[tokio::test]
	#[serial]
	async fn test_resolve_merges_tenant_throttle_overrides() {
		let db = TestDb::start().await;
		let pool = db.pool();

		let tenant_id = insert_tenant(pool, "throttle-merge", None, 0).await;
		let (full_key, _) = insert_api_key(pool, tenant_id).await;

		// Set a per-tenant max_requests_per_second override
		sqlx::query("UPDATE tenants SET max_requests_per_second = 5 WHERE id = $1")
			.bind(tenant_id)
			.execute(pool)
			.await
			.expect("failed to update tenant throttle");

		// Global throttle has max_requests_per_minute = 100
		let global_throttle = ThrottleConfig {
			max_requests_per_second: None,
			max_requests_per_minute: Some(100),
			max_requests_per_hour: None,
			max_requests_per_day: None,
		};

		let ctx = resolve_from_api_key(pool, &full_key, &global_throttle)
			.await
			.expect("resolve should succeed");

		// Tenant override should win for per_second
		assert_eq!(ctx.throttle.max_requests_per_second, Some(5));
		// Global fallback should apply for per_minute
		assert_eq!(ctx.throttle.max_requests_per_minute, Some(100));
		// Unset fields remain None
		assert_eq!(ctx.throttle.max_requests_per_hour, None);
		assert_eq!(ctx.throttle.max_requests_per_day, None);
	}

	#[tokio::test]
	#[serial]
	async fn test_update_last_used_at() {
		let db = TestDb::start().await;
		let pool = db.pool();

		let tenant_id = insert_tenant(pool, "last-used", None, 0).await;
		let (full_key, api_key_id) = insert_api_key(pool, tenant_id).await;

		let global_throttle = ThrottleConfig::new_without_throttle();
		let _ctx = resolve_from_api_key(pool, &full_key, &global_throttle)
			.await
			.expect("resolve should succeed");

		// Give the fire-and-forget spawned task time to complete
		tokio::time::sleep(std::time::Duration::from_millis(100)).await;

		let row = sqlx::query("SELECT last_used_at FROM api_keys WHERE id = $1")
			.bind(api_key_id)
			.fetch_one(pool)
			.await
			.expect("query last_used_at failed");

		let last_used: Option<chrono::DateTime<chrono::Utc>> = row.get("last_used_at");

		assert!(
			last_used.is_some(),
			"last_used_at should be set after resolve"
		);
	}

	#[tokio::test]
	#[serial]
	async fn test_generate_api_key_properties() {
		let (full_key, prefix, hash) = generate_api_key();

		// Full key starts with "rch_live_"
		assert!(
			full_key.starts_with("rch_live_"),
			"key should start with rch_live_, got: {full_key}"
		);

		// Prefix is first 24 chars of the full key
		assert_eq!(prefix.len(), 24);
		assert_eq!(&full_key[..24], prefix);

		// Full key = prefix "rch_live_" (9 chars) + 32 hex chars = 41 total
		assert_eq!(
			full_key.len(),
			41,
			"full key should be 41 chars, got: {}",
			full_key.len()
		);

		// Hash should be consistent
		assert_eq!(hash, hash_api_key(&full_key));

		// Each call produces a different key
		let (full_key2, _, _) = generate_api_key();
		assert_ne!(full_key, full_key2, "two generated keys should differ");
	}

	#[tokio::test]
	#[serial]
	async fn test_hash_api_key_consistency() {
		let input = "rch_live_abcdef1234567890abcdef12";
		let hash1 = hash_api_key(input);
		let hash2 = hash_api_key(input);

		// Same input yields the same hash
		assert_eq!(hash1, hash2, "hashing should be deterministic");

		// Different input yields a different hash
		let different = "rch_live_ffffffffffffffffffffffff";
		let hash3 = hash_api_key(different);
		assert_ne!(
			hash1, hash3,
			"different inputs should produce different hashes"
		);

		// Hash output is a valid hex string of expected length (SHA-256 = 64 hex chars)
		assert_eq!(hash1.len(), 64, "SHA-256 hex digest should be 64 chars");
		assert!(
			hash1.chars().all(|c| c.is_ascii_hexdigit()),
			"hash should be valid hex"
		);
	}
}
