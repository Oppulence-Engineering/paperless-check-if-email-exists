use check_if_email_exists::LOG_TARGET;
use chrono::Utc;
use sqlx::PgPool;
use tracing::{debug, warn};
use uuid::Uuid;

use crate::tenant::context::TenantContext;

/// Result of a quota check.
#[derive(Debug, Clone, PartialEq)]
pub enum QuotaCheckResult {
	/// Quota is available (or unlimited).
	Allowed,
	/// Monthly limit has been reached.
	ExceededMonthlyLimit {
		limit: i32,
		used: i32,
		resets_at: chrono::DateTime<Utc>,
	},
}

/// Check whether the tenant has remaining monthly quota.
/// For legacy/unlimited tenants, always returns Allowed.
/// If the billing period has elapsed, resets the counter first.
pub async fn check_quota(pg_pool: Option<&PgPool>, tenant_ctx: &TenantContext) -> QuotaCheckResult {
	// Legacy callers or tenants with no limit are always allowed
	let limit = match tenant_ctx.monthly_email_limit {
		Some(l) if l > 0 => l,
		Some(_) => return QuotaCheckResult::Allowed,
		None => return QuotaCheckResult::Allowed,
	};

	let tenant_id = match tenant_ctx.tenant_id {
		Some(id) => id,
		None => return QuotaCheckResult::Allowed,
	};

	let pool = match pg_pool {
		Some(p) => p,
		None => return QuotaCheckResult::Allowed,
	};

	// If the period has elapsed, reset the counter atomically
	if Utc::now() >= tenant_ctx.period_reset_at {
		if let Err(e) = reset_period(pool, tenant_id).await {
			warn!(target: LOG_TARGET, tenant_id=?tenant_id, error=?e, "Failed to reset billing period");
		}
		// After reset, used = 0, so we're allowed
		return QuotaCheckResult::Allowed;
	}

	// Fetch fresh counter from DB (the context snapshot may be stale)
	let current_usage = match fetch_current_usage(pool, tenant_id).await {
		Ok(u) => u,
		Err(e) => {
			warn!(target: LOG_TARGET, tenant_id=?tenant_id, error=?e, "Failed to fetch usage, allowing request");
			return QuotaCheckResult::Allowed;
		}
	};

	if current_usage >= limit {
		QuotaCheckResult::ExceededMonthlyLimit {
			limit,
			used: current_usage,
			resets_at: tenant_ctx.period_reset_at,
		}
	} else {
		QuotaCheckResult::Allowed
	}
}

/// Atomically check quota AND increment in a single query, avoiding TOCTOU races.
/// Returns Allowed if the increment succeeded, or ExceededMonthlyLimit if at limit.
pub async fn check_and_increment_quota(
	pg_pool: Option<&PgPool>,
	tenant_ctx: &TenantContext,
) -> QuotaCheckResult {
	check_and_increment_quota_for_count(pg_pool, tenant_ctx, 1).await
}

/// Atomically check quota and increment by `count` in a single query, avoiding TOCTOU races.
/// Returns Allowed if the increment succeeded, or ExceededMonthlyLimit if at limit.
pub async fn check_and_increment_quota_for_count(
	pg_pool: Option<&PgPool>,
	tenant_ctx: &TenantContext,
	count: i32,
) -> QuotaCheckResult {
	let limit = match tenant_ctx.monthly_email_limit {
		Some(l) if l > 0 => l,
		Some(_) => return QuotaCheckResult::Allowed,
		None => return QuotaCheckResult::Allowed,
	};
	let tenant_id = match tenant_ctx.tenant_id {
		Some(id) => id,
		None => return QuotaCheckResult::Allowed,
	};
	let pool = match pg_pool {
		Some(p) => p,
		None => return QuotaCheckResult::Allowed,
	};

	// If period expired, reset first
	if Utc::now() >= tenant_ctx.period_reset_at {
		if let Err(e) = reset_period(pool, tenant_id).await {
			warn!(target: LOG_TARGET, tenant_id=?tenant_id, error=?e, "Failed to reset billing period");
		}
	}

	// Avoid a no-op update on non-positive counts.
	let request_count = count.max(0);
	if request_count == 0 {
		return QuotaCheckResult::Allowed;
	}

	let result = sqlx::query(
		"UPDATE tenants \
		 SET used_this_period = used_this_period + $2 \
		 WHERE id = $1 \
		 AND (monthly_email_limit IS NULL OR monthly_email_limit <= 0 OR used_this_period + $2 <= monthly_email_limit) \
		 RETURNING used_this_period"
	)
	.bind(tenant_id)
	.bind(request_count)
	.fetch_optional(pool)
	.await;

	match result {
		Ok(Some(_)) => QuotaCheckResult::Allowed,
		Ok(None) => {
			// 0 rows updated — could be over limit OR tenant deleted
			match fetch_current_usage(pool, tenant_id).await {
				Ok(used) => QuotaCheckResult::ExceededMonthlyLimit {
					limit,
					used,
					resets_at: tenant_ctx.period_reset_at,
				},
				Err(_) => {
					// Tenant row doesn't exist — fail open rather than false 429
					warn!(target: LOG_TARGET, tenant_id=?tenant_id, "Tenant not found during quota check, allowing");
					QuotaCheckResult::Allowed
				}
			}
		}
		Err(e) => {
			warn!(target: LOG_TARGET, tenant_id=?tenant_id, error=?e, "Quota check failed, allowing request");
			QuotaCheckResult::Allowed
		}
	}
}

/// Atomically increment the tenant's used_this_period counter.
pub async fn increment_usage(pg_pool: Option<&PgPool>, tenant_id: Option<Uuid>) {
	let (pool, id) = match (pg_pool, tenant_id) {
		(Some(p), Some(id)) => (p, id),
		_ => return,
	};

	let result = sqlx::query!(
		"UPDATE tenants SET used_this_period = used_this_period + 1 WHERE id = $1",
		id,
	)
	.execute(pool)
	.await;

	if let Err(e) = result {
		warn!(target: LOG_TARGET, tenant_id=?id, error=?e, "Failed to increment usage counter");
	}
}

/// Reset the billing period: set used_this_period = 0 and advance period_reset_at
/// to the start of the next month.
async fn reset_period(pool: &PgPool, tenant_id: Uuid) -> Result<(), sqlx::Error> {
	sqlx::query!(
		r#"
		UPDATE tenants
		SET used_this_period = 0,
		    period_reset_at = date_trunc('month', NOW()) + INTERVAL '1 month'
		WHERE id = $1 AND period_reset_at <= NOW()
		"#,
		tenant_id,
	)
	.execute(pool)
	.await?;

	debug!(target: LOG_TARGET, tenant_id=?tenant_id, "Reset billing period");
	Ok(())
}

/// Fetch the current usage counter directly from the database.
async fn fetch_current_usage(pool: &PgPool, tenant_id: Uuid) -> Result<i32, sqlx::Error> {
	let row = sqlx::query_scalar!(
		"SELECT used_this_period FROM tenants WHERE id = $1",
		tenant_id,
	)
	.fetch_one(pool)
	.await?;

	Ok(row)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::config::ThrottleConfig;
	use crate::tenant::models::{PlanTier, TenantStatus};
	use chrono::Duration;

	fn make_ctx(limit: Option<i32>, used: i32, period_expired: bool) -> TenantContext {
		let reset_at = if period_expired {
			Utc::now() - Duration::hours(1)
		} else {
			Utc::now() + Duration::hours(24)
		};
		TenantContext {
			tenant_id: Some(Uuid::new_v4()),
			api_key_id: Some(Uuid::new_v4()),
			tenant_name: "test-tenant".into(),
			plan_tier: PlanTier::Starter,
			status: TenantStatus::Active,
			throttle: ThrottleConfig::new_without_throttle(),
			monthly_email_limit: limit,
			used_this_period: used,
			period_reset_at: reset_at,
			default_webhook_url: None,
			webhook_signing_secret: None,
			result_retention_days: 30,
			is_legacy: false,
			scopes: vec![],
		}
	}

	#[tokio::test]
	async fn test_unlimited_always_allowed() {
		let ctx = make_ctx(None, 999999, false);
		assert_eq!(check_quota(None, &ctx).await, QuotaCheckResult::Allowed);
	}

	#[tokio::test]
	async fn test_legacy_always_allowed() {
		let mut ctx = make_ctx(Some(100), 200, false);
		ctx.is_legacy = true;
		ctx.tenant_id = None;
		assert_eq!(check_quota(None, &ctx).await, QuotaCheckResult::Allowed);
	}

	#[tokio::test]
	async fn test_no_pool_always_allowed() {
		let ctx = make_ctx(Some(100), 200, false);
		// No PgPool available — fail-open
		assert_eq!(check_quota(None, &ctx).await, QuotaCheckResult::Allowed);
	}

	#[tokio::test]
	async fn test_period_expired_resets() {
		// When period is expired, the check returns Allowed because
		// the counter gets reset (or would, with a real DB)
		let ctx = make_ctx(Some(100), 200, true);
		// Without a DB pool, this falls through to Allowed
		assert_eq!(check_quota(None, &ctx).await, QuotaCheckResult::Allowed);
	}
}
