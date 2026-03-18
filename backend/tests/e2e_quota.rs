#[cfg(test)]
mod test_helpers;

#[cfg(test)]
mod tests {
	use chrono::Utc;
	use reacher_backend::config::ThrottleConfig;
	use reacher_backend::tenant::context::TenantContext;
	use reacher_backend::tenant::models::{PlanTier, TenantStatus};
	use reacher_backend::tenant::quota::{
		check_and_increment_quota_for_count, check_quota, increment_usage, QuotaCheckResult,
	};
	use serial_test::serial;
	use sqlx::{PgPool, Row};
	use crate::test_helpers::{insert_tenant, TestDb};
	use uuid::Uuid;

	/// Build a TenantContext by reading the tenant row from the database so that
	/// `period_reset_at` and the other fields match what Postgres actually stored.
	async fn make_ctx_from_db(pool: &PgPool, tenant_id: Uuid) -> TenantContext {
		let row = sqlx::query(
			"SELECT monthly_email_limit, used_this_period, period_reset_at FROM tenants WHERE id = $1",
		)
		.bind(tenant_id)
		.fetch_one(pool)
		.await
		.unwrap();

		TenantContext {
			tenant_id: Some(tenant_id),
			api_key_id: None,
			tenant_name: "test".into(),
			plan_tier: PlanTier::Starter,
			status: TenantStatus::Active,
			throttle: ThrottleConfig::new_without_throttle(),
			monthly_email_limit: row.get("monthly_email_limit"),
			used_this_period: row.get("used_this_period"),
			period_reset_at: row.get("period_reset_at"),
			default_webhook_url: None,
			webhook_signing_secret: None,
			result_retention_days: 30,
			is_legacy: false,
		}
	}

	// ---------------------------------------------------------------
	// 1. Under-limit tenant is allowed
	// ---------------------------------------------------------------
	#[tokio::test]
	#[serial]
	async fn test_under_limit_allowed() {
		let db = TestDb::start().await;
		let pool = db.pool();

		let tenant_id = insert_tenant(pool, "under-limit", Some(100), 50).await;
		let ctx = make_ctx_from_db(pool, tenant_id).await;

		let result = check_quota(Some(pool), &ctx).await;
		assert_eq!(result, QuotaCheckResult::Allowed);
	}

	// ---------------------------------------------------------------
	// 2. At-limit tenant is rejected
	// ---------------------------------------------------------------
	#[tokio::test]
	#[serial]
	async fn test_at_limit_rejected() {
		let db = TestDb::start().await;
		let pool = db.pool();

		let tenant_id = insert_tenant(pool, "at-limit", Some(100), 100).await;
		let ctx = make_ctx_from_db(pool, tenant_id).await;

		let result = check_quota(Some(pool), &ctx).await;
		assert!(
			matches!(result, QuotaCheckResult::ExceededMonthlyLimit { .. }),
			"Expected ExceededMonthlyLimit but got {:?}",
			result
		);
	}

	// ---------------------------------------------------------------
	// 3. Over-limit tenant is rejected with correct fields
	// ---------------------------------------------------------------
	#[tokio::test]
	#[serial]
	async fn test_over_limit_rejected() {
		let db = TestDb::start().await;
		let pool = db.pool();

		let tenant_id = insert_tenant(pool, "over-limit", Some(10), 15).await;
		let ctx = make_ctx_from_db(pool, tenant_id).await;

		let result = check_quota(Some(pool), &ctx).await;
		match result {
			QuotaCheckResult::ExceededMonthlyLimit {
				limit,
				used,
				resets_at,
			} => {
				assert_eq!(limit, 10);
				assert_eq!(used, 15);
				assert_eq!(resets_at, ctx.period_reset_at);
			}
			other => panic!("Expected ExceededMonthlyLimit but got {:?}", other),
		}
	}

	// ---------------------------------------------------------------
	// 4. Unlimited (NULL limit) tenant is always allowed
	// ---------------------------------------------------------------
	#[tokio::test]
	#[serial]
	async fn test_unlimited_always_allowed() {
		let db = TestDb::start().await;
		let pool = db.pool();

		let tenant_id = insert_tenant(pool, "unlimited", None, 999).await;
		let ctx = make_ctx_from_db(pool, tenant_id).await;

		let result = check_quota(Some(pool), &ctx).await;
		assert_eq!(result, QuotaCheckResult::Allowed);
	}

	#[tokio::test]
	#[serial]
	async fn test_zero_limit_is_unlimited() {
		let db = TestDb::start().await;
		let pool = db.pool();

		let tenant_id = insert_tenant(pool, "zero-limit", Some(0), 12).await;
		let ctx = make_ctx_from_db(pool, tenant_id).await;

		let result = check_quota(Some(pool), &ctx).await;
		assert_eq!(result, QuotaCheckResult::Allowed);

		let result = check_and_increment_quota_for_count(Some(pool), &ctx, 100).await;
		assert_eq!(result, QuotaCheckResult::Allowed);

		// Zero limit = unlimited: the function returns Allowed early without
		// incrementing the usage counter, so the value stays at the original 12.
		let used: i32 = sqlx::query("SELECT used_this_period FROM tenants WHERE id = $1")
			.bind(tenant_id)
			.fetch_one(pool)
			.await
			.unwrap()
			.get("used_this_period");
		assert_eq!(used, 12);
	}

	// ---------------------------------------------------------------
	// 5. increment_usage updates the database counter
	// ---------------------------------------------------------------
	#[tokio::test]
	#[serial]
	async fn test_increment_usage_updates_db() {
		let db = TestDb::start().await;
		let pool = db.pool();

		let tenant_id = insert_tenant(pool, "inc-usage", Some(100), 0).await;

		for _ in 0..3 {
			increment_usage(Some(pool), Some(tenant_id)).await;
		}

		let row = sqlx::query("SELECT used_this_period FROM tenants WHERE id = $1")
			.bind(tenant_id)
			.fetch_one(pool)
			.await
			.unwrap();

		let used: i32 = row.get("used_this_period");

		assert_eq!(used, 3);
	}

	// ---------------------------------------------------------------
	// 6. increment_usage with None tenant_id is a no-op
	// ---------------------------------------------------------------
	#[tokio::test]
	#[serial]
	async fn test_increment_no_tenant_is_noop() {
		let db = TestDb::start().await;
		let pool = db.pool();

		// Should not panic or error
		increment_usage(Some(pool), None).await;
		// Also test with no pool
		increment_usage(None, Some(Uuid::new_v4())).await;
		// Both None
		increment_usage(None, None).await;
	}

	// ---------------------------------------------------------------
	// 7. Expired period resets the counter and returns Allowed
	// ---------------------------------------------------------------
	#[tokio::test]
	#[serial]
	async fn test_expired_period_resets_counter() {
		let db = TestDb::start().await;
		let pool = db.pool();

		let tenant_id = insert_tenant(pool, "expired-period", Some(10), 10).await;

		// Move period_reset_at to the past so the period is expired
		sqlx::query("UPDATE tenants SET period_reset_at = NOW() - INTERVAL '1 hour' WHERE id = $1")
			.bind(tenant_id)
			.execute(pool)
			.await
			.unwrap();

		let ctx = make_ctx_from_db(pool, tenant_id).await;
		let result = check_quota(Some(pool), &ctx).await;
		assert_eq!(result, QuotaCheckResult::Allowed);

		// Verify that the DB counter was reset to 0
		let row = sqlx::query("SELECT used_this_period FROM tenants WHERE id = $1")
			.bind(tenant_id)
			.fetch_one(pool)
			.await
			.unwrap();

		let used: i32 = row.get("used_this_period");

		assert_eq!(used, 0, "Counter should have been reset to 0 after period expiry");
	}

	// ---------------------------------------------------------------
	// 8. Concurrent increments are all counted
	// ---------------------------------------------------------------
	#[tokio::test]
	#[serial]
	async fn test_concurrent_increments() {
		let db = TestDb::start().await;
		let pool = db.pool();

		let tenant_id = insert_tenant(pool, "concurrent", Some(100), 0).await;

		let mut handles = Vec::new();
		for _ in 0..10 {
			let pool_clone = db.pool_owned();
			let tid = tenant_id;
			handles.push(tokio::spawn(async move {
				increment_usage(Some(&pool_clone), Some(tid)).await;
			}));
		}

		for h in handles {
			h.await.unwrap();
		}

		let row = sqlx::query("SELECT used_this_period FROM tenants WHERE id = $1")
			.bind(tenant_id)
			.fetch_one(pool)
			.await
			.unwrap();

		let used: i32 = row.get("used_this_period");

		assert_eq!(used, 10, "All 10 concurrent increments should be reflected");
	}

	// ---------------------------------------------------------------
	// 9. check_quota reads fresh data from DB, not the stale context
	// ---------------------------------------------------------------
	#[tokio::test]
	#[serial]
	async fn test_check_quota_reads_fresh_from_db() {
		let db = TestDb::start().await;
		let pool = db.pool();

		let tenant_id = insert_tenant(pool, "stale-ctx", Some(100), 0).await;

		// Increment 5 times in the DB
		for _ in 0..5 {
			increment_usage(Some(pool), Some(tenant_id)).await;
		}

		// Build a context with stale used=0 (as if cached before the increments)
		let mut ctx = make_ctx_from_db(pool, tenant_id).await;
		// Intentionally override to simulate a stale snapshot
		ctx.used_this_period = 0;

		// check_quota should read fresh DB value (5) which is < limit (100)
		let result = check_quota(Some(pool), &ctx).await;
		assert_eq!(
			result,
			QuotaCheckResult::Allowed,
			"check_quota should read fresh DB usage (5) < limit (100), not stale ctx (0)"
		);
	}

	// ---------------------------------------------------------------
	// 10. check_and_increment_quota_for_count allows bulk-style usage within limit
	// ---------------------------------------------------------------
	#[tokio::test]
	#[serial]
	async fn test_check_and_increment_for_count_within_limit() {
		let db = TestDb::start().await;
		let pool = db.pool();

		let tenant_id = insert_tenant(pool, "within-limit-count", Some(100), 95).await;
		let ctx = make_ctx_from_db(pool, tenant_id).await;

		let result = check_and_increment_quota_for_count(Some(pool), &ctx, 3).await;
		assert_eq!(result, QuotaCheckResult::Allowed);

		let row = sqlx::query("SELECT used_this_period FROM tenants WHERE id = $1")
			.bind(tenant_id)
			.fetch_one(pool)
			.await
			.unwrap();
		let used: i32 = row.get("used_this_period");
		assert_eq!(used, 98);
	}

	// ---------------------------------------------------------------
	// 11. check_and_increment_quota_for_count rejects when count exceeds quota
	// ---------------------------------------------------------------
	#[tokio::test]
	#[serial]
	async fn test_check_and_increment_for_count_exceeds_limit() {
		let db = TestDb::start().await;
		let pool = db.pool();

		let tenant_id = insert_tenant(pool, "exceeds-limit-count", Some(10), 9).await;
		let ctx = make_ctx_from_db(pool, tenant_id).await;

		let result = check_and_increment_quota_for_count(Some(pool), &ctx, 2).await;
		match result {
			QuotaCheckResult::ExceededMonthlyLimit { limit, used, .. } => {
				assert_eq!(limit, 10);
				assert_eq!(used, 9);
			}
			other => panic!("Expected ExceededMonthlyLimit but got {:?}", other),
		}

		let row = sqlx::query("SELECT used_this_period FROM tenants WHERE id = $1")
			.bind(tenant_id)
			.fetch_one(pool)
			.await
			.unwrap();
		let used: i32 = row.get("used_this_period");
		assert_eq!(used, 9, "Count-based quota check should not increment when over limit");
	}
}
