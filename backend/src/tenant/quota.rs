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
	/// Admission could not be established; no work may be dispatched.
	Unavailable,
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
		None if tenant_ctx.is_legacy => return QuotaCheckResult::Allowed,
		None => return QuotaCheckResult::Unavailable,
	};

	let pool = match pg_pool {
		Some(p) => p,
		None => return QuotaCheckResult::Unavailable,
	};

	// If the period has elapsed, reset the counter atomically
	if Utc::now() >= tenant_ctx.period_reset_at {
		if let Err(e) = reset_period(pool, tenant_id).await {
			warn!(target: LOG_TARGET, tenant_id=?tenant_id, error=?e, "Failed to reset billing period");
			return QuotaCheckResult::Unavailable;
		}
	}

	// Fetch fresh counter from DB (the context snapshot may be stale)
	let current_usage = match fetch_current_usage(pool, tenant_id).await {
		Ok(u) => u,
		Err(e) => {
			warn!(target: LOG_TARGET, tenant_id=?tenant_id, error=?e, "Failed to fetch usage");
			return QuotaCheckResult::Unavailable;
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

/// Atomically check quota and increment, avoiding TOCTOU races.
/// Returns Allowed if the increment succeeded, or ExceededMonthlyLimit if at limit.
pub async fn check_and_increment_quota(
	pg_pool: Option<&PgPool>,
	tenant_ctx: &TenantContext,
) -> QuotaCheckResult {
	check_and_increment_quota_for_count(pg_pool, tenant_ctx, 1).await
}

/// Atomically check quota and increment by `count`, avoiding TOCTOU races.
/// Returns Allowed if the increment succeeded, or ExceededMonthlyLimit if at limit.
pub async fn check_and_increment_quota_for_count(
	pg_pool: Option<&PgPool>,
	tenant_ctx: &TenantContext,
	count: i32,
) -> QuotaCheckResult {
	if tenant_ctx
		.monthly_email_limit
		.map_or(true, |limit| limit <= 0)
	{
		return QuotaCheckResult::Allowed;
	}
	if tenant_ctx.is_legacy && tenant_ctx.tenant_id.is_none() {
		return QuotaCheckResult::Allowed;
	}
	let Some(pool) = pg_pool else {
		return QuotaCheckResult::Unavailable;
	};
	let result = async {
		let mut tx = pool.begin().await?;
		let result = reserve_quota_in_tx(&mut tx, tenant_ctx, count).await?;
		if result == QuotaCheckResult::Allowed {
			tx.commit().await?;
		}
		Ok::<_, sqlx::Error>(result)
	}
	.await;
	match result {
		Ok(result) => result,
		Err(error) => {
			warn!(target: LOG_TARGET, tenant_id=?tenant_ctx.tenant_id, error=?error, "Quota reservation unavailable");
			QuotaCheckResult::Unavailable
		}
	}
}

/// Reserve quota in the transaction that creates the work, so both roll back together.
pub async fn reserve_quota_in_tx(
	tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
	tenant_ctx: &TenantContext,
	count: i32,
) -> Result<QuotaCheckResult, sqlx::Error> {
	let Some(tenant_id) = tenant_ctx.tenant_id else {
		return Ok(if tenant_ctx.is_legacy {
			QuotaCheckResult::Allowed
		} else {
			QuotaCheckResult::Unavailable
		});
	};
	let (limit, mut used, mut resets_at): (Option<i32>, i32, chrono::DateTime<Utc>) = sqlx::query_as(
		"SELECT monthly_email_limit, used_this_period, period_reset_at FROM tenants WHERE id = $1 FOR UPDATE",
	).bind(tenant_id).fetch_one(&mut **tx).await?;
	let Some(limit) = limit.filter(|limit| *limit > 0) else {
		return Ok(QuotaCheckResult::Allowed);
	};
	if count <= 0 {
		return Ok(QuotaCheckResult::Allowed);
	}
	if resets_at <= Utc::now() {
		(used, resets_at) = sqlx::query_as(
			"UPDATE tenants SET used_this_period = 0, period_reset_at = date_trunc('month', NOW()) + INTERVAL '1 month' WHERE id = $1 RETURNING used_this_period, period_reset_at",
		).bind(tenant_id).fetch_one(&mut **tx).await?;
	}
	if used.checked_add(count).map_or(true, |next| next > limit) {
		return Ok(QuotaCheckResult::ExceededMonthlyLimit {
			limit,
			used,
			resets_at,
		});
	}
	sqlx::query("UPDATE tenants SET used_this_period = used_this_period + $2 WHERE id = $1")
		.bind(tenant_id)
		.bind(count)
		.execute(&mut **tx)
		.await?;
	Ok(QuotaCheckResult::Allowed)
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

/// Release previously reserved usage back to the tenant counter.
/// This is used when a workflow reserves quota up front but publishes fewer
/// emails than expected.
pub async fn release_reserved_usage(
	pg_pool: Option<&PgPool>,
	tenant_id: Option<Uuid>,
	count: i32,
) -> Result<(), sqlx::Error> {
	let (pool, id) = match (pg_pool, tenant_id) {
		(Some(p), Some(id)) if count > 0 => (p, id),
		_ => return Ok(()),
	};

	// Reservations only count tenants with a positive monthly limit, so the
	// release does too. Otherwise a failed request for an unlimited tenant
	// lowered a counter that its reservation never raised.
	let result = sqlx::query(
		"UPDATE tenants SET used_this_period = GREATEST(used_this_period - $2, 0) \
		 WHERE id = $1 AND monthly_email_limit > 0",
	)
	.bind(id)
	.bind(count)
	.execute(pool)
	.await;

	if let Err(e) = result {
		warn!(target: LOG_TARGET, tenant_id=?id, count=count, error=?e, "Failed to release reserved usage");
		return Err(e);
	}

	Ok(())
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
	async fn metered_tenant_requires_a_database() {
		let ctx = make_ctx(Some(100), 200, false);
		assert_eq!(check_quota(None, &ctx).await, QuotaCheckResult::Unavailable);
		assert_eq!(
			check_and_increment_quota(None, &ctx).await,
			QuotaCheckResult::Unavailable
		);
	}

	#[tokio::test]
	async fn failed_usage_read_or_period_reset_never_admits_work() {
		let pool = sqlx::postgres::PgPoolOptions::new()
			.connect_lazy("postgres://unused:unused@127.0.0.1/unused")
			.unwrap();
		pool.close().await;
		for expired in [false, true] {
			let ctx = make_ctx(Some(100), 200, expired);
			assert_eq!(
				check_quota(Some(&pool), &ctx).await,
				QuotaCheckResult::Unavailable
			);
			assert_eq!(
				check_and_increment_quota(Some(&pool), &ctx).await,
				QuotaCheckResult::Unavailable
			);
		}
	}
}
