use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "tenant_status", rename_all = "lowercase")]
pub enum TenantStatus {
	Active,
	Suspended,
	Deactivated,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "plan_tier", rename_all = "lowercase")]
pub enum PlanTier {
	Free,
	Starter,
	Professional,
	Enterprise,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "api_key_status", rename_all = "lowercase")]
pub enum ApiKeyStatus {
	Active,
	Revoked,
	Expired,
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct Tenant {
	pub id: Uuid,
	pub name: String,
	pub slug: String,
	pub contact_email: String,
	pub plan_name: String,
	pub plan_tier: PlanTier,
	pub max_requests_per_second: Option<i32>,
	pub max_requests_per_minute: Option<i32>,
	pub max_requests_per_hour: Option<i32>,
	pub max_requests_per_day: Option<i32>,
	pub monthly_email_limit: Option<i32>,
	pub used_this_period: i32,
	pub period_reset_at: DateTime<Utc>,
	pub status: TenantStatus,
	pub default_webhook_url: Option<String>,
	pub webhook_signing_secret: Option<String>,
	pub result_retention_days: i32,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct ApiKey {
	pub id: Uuid,
	pub tenant_id: Uuid,
	pub key_prefix: String,
	pub key_hash: String,
	pub name: String,
	pub scopes: Vec<String>,
	pub status: ApiKeyStatus,
	pub last_used_at: Option<DateTime<Utc>>,
	pub expires_at: Option<DateTime<Utc>>,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_tenant_status_serde_roundtrip() {
		for status in [
			TenantStatus::Active,
			TenantStatus::Suspended,
			TenantStatus::Deactivated,
		] {
			let json = serde_json::to_string(&status).unwrap();
			let back: TenantStatus = serde_json::from_str(&json).unwrap();
			assert_eq!(status, back);
		}
	}

	#[test]
	fn test_plan_tier_serde_roundtrip() {
		for tier in [
			PlanTier::Free,
			PlanTier::Starter,
			PlanTier::Professional,
			PlanTier::Enterprise,
		] {
			let json = serde_json::to_string(&tier).unwrap();
			let back: PlanTier = serde_json::from_str(&json).unwrap();
			assert_eq!(tier, back);
		}
	}

	#[test]
	fn test_api_key_status_serde_roundtrip() {
		for status in [
			ApiKeyStatus::Active,
			ApiKeyStatus::Revoked,
			ApiKeyStatus::Expired,
		] {
			let json = serde_json::to_string(&status).unwrap();
			let back: ApiKeyStatus = serde_json::from_str(&json).unwrap();
			assert_eq!(status, back);
		}
	}

	#[test]
	fn test_plan_tier_equality() {
		assert_eq!(PlanTier::Free, PlanTier::Free);
		assert_ne!(PlanTier::Free, PlanTier::Enterprise);
	}

	#[test]
	fn test_api_key_status_clone() {
		let s = ApiKeyStatus::Active;
		let s2 = s.clone();
		assert_eq!(s, s2);
	}
}
