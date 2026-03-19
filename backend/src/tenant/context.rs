use crate::config::ThrottleConfig;
use crate::tenant::models::{PlanTier, TenantStatus};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Lightweight context struct that flows through warp filters to every handler.
/// Contains all tenant-scoped information needed for authorization, throttling,
/// and resource scoping.
#[derive(Debug, Clone)]
pub struct TenantContext {
	pub tenant_id: Option<Uuid>,
	pub api_key_id: Option<Uuid>,
	pub tenant_name: String,
	pub plan_tier: PlanTier,
	pub status: TenantStatus,
	pub throttle: ThrottleConfig,
	pub monthly_email_limit: Option<i32>,
	pub used_this_period: i32,
	pub period_reset_at: DateTime<Utc>,
	pub default_webhook_url: Option<String>,
	pub webhook_signing_secret: Option<String>,
	pub result_retention_days: i32,
	pub is_legacy: bool,
}

impl TenantContext {
	/// Creates a synthetic TenantContext for callers using the legacy
	/// `x-reacher-secret` header or for open/self-hosted mode with no auth.
	/// This provides unlimited quotas and no tenant scoping.
	pub fn legacy(global_throttle: ThrottleConfig) -> Self {
		Self {
			tenant_id: None,
			api_key_id: None,
			tenant_name: "legacy".to_string(),
			plan_tier: PlanTier::Enterprise,
			status: TenantStatus::Active,
			throttle: global_throttle,
			monthly_email_limit: None,
			used_this_period: 0,
			period_reset_at: Utc::now(),
			default_webhook_url: None,
			webhook_signing_secret: None,
			result_retention_days: 30,
			is_legacy: true,
		}
	}

	/// Returns the tenant_id as a string for use in idempotency keys and
	/// other contexts that need a string identifier.
	pub fn tenant_id_str(&self) -> String {
		self.tenant_id
			.map(|id| id.to_string())
			.unwrap_or_else(|| "legacy".to_string())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use uuid::Uuid;

	#[test]
	fn test_legacy_context_fields() {
		let throttle = ThrottleConfig {
			max_requests_per_second: Some(10),
			max_requests_per_minute: Some(100),
			max_requests_per_hour: None,
			max_requests_per_day: None,
		};
		let ctx = TenantContext::legacy(throttle.clone());

		assert!(ctx.is_legacy);
		assert!(ctx.tenant_id.is_none());
		assert!(ctx.api_key_id.is_none());
		assert_eq!(ctx.tenant_name, "legacy");
		assert_eq!(ctx.plan_tier, PlanTier::Enterprise);
		assert_eq!(ctx.status, TenantStatus::Active);
		assert!(ctx.monthly_email_limit.is_none());
		assert_eq!(ctx.used_this_period, 0);
		assert!(ctx.default_webhook_url.is_none());
		assert!(ctx.webhook_signing_secret.is_none());
		assert_eq!(ctx.result_retention_days, 30);
		assert_eq!(ctx.throttle.max_requests_per_second, Some(10));
		assert_eq!(ctx.throttle.max_requests_per_minute, Some(100));
	}

	#[test]
	fn test_tenant_id_str_with_uuid() {
		let id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
		let mut ctx = TenantContext::legacy(ThrottleConfig::new_without_throttle());
		ctx.tenant_id = Some(id);
		assert_eq!(ctx.tenant_id_str(), "550e8400-e29b-41d4-a716-446655440000");
	}

	#[test]
	fn test_tenant_id_str_legacy() {
		let ctx = TenantContext::legacy(ThrottleConfig::new_without_throttle());
		assert_eq!(ctx.tenant_id_str(), "legacy");
	}

	#[test]
	fn test_clone() {
		let ctx = TenantContext::legacy(ThrottleConfig::new_without_throttle());
		let ctx2 = ctx.clone();
		assert_eq!(ctx.tenant_name, ctx2.tenant_name);
		assert_eq!(ctx.is_legacy, ctx2.is_legacy);
	}
}
