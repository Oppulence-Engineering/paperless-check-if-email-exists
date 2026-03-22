use crate::config::ThrottleConfig;
use crate::tenant::context::TenantContext;
use crate::tenant::models::{ApiKeyStatus, PlanTier, TenantStatus};
use anyhow::{bail, Result};
use chrono::Utc;
use hmac::{Hmac, Mac};
use rand::Rng;
use sha2::Sha256;
use sqlx::PgPool;
use uuid::Uuid;

type HmacSha256 = Hmac<Sha256>;

const API_KEY_PREFIX: &str = "rch_live_";

/// Get the HMAC server secret from the environment, or None if not set.
fn get_hmac_secret() -> Option<String> {
	std::env::var("RCH_API_KEY_HMAC_SECRET").ok()
}

/// Generate a new API key. Returns (full_key, key_prefix, key_hash).
pub fn generate_api_key() -> (String, String, String) {
	let random_bytes: [u8; 16] = rand::thread_rng().gen();
	let random_hex = hex::encode(random_bytes);
	let full_key = format!("{}{}", API_KEY_PREFIX, random_hex);
	// Use 24-char prefix to capture 15 random hex chars for better key identification
	let key_prefix = full_key[..24.min(full_key.len())].to_string();
	let key_hash = hash_api_key(&full_key);
	(full_key, key_prefix, key_hash)
}

/// Hash an API key using HMAC-SHA256 with a server-side secret (if configured)
/// or plain SHA-256 as fallback. The HMAC approach ensures that a leaked DB
/// dump cannot be used to recover API keys even with brute force.
pub fn hash_api_key(key: &str) -> String {
	if let Some(secret) = get_hmac_secret() {
		let mut mac =
			HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC accepts any key length");
		mac.update(key.as_bytes());
		hex::encode(mac.finalize().into_bytes())
	} else {
		// Fallback: bare SHA-256 (acceptable for 128-bit random keys)
		use sha2::Digest;
		let mut hasher = Sha256::new();
		hasher.update(key.as_bytes());
		hex::encode(hasher.finalize())
	}
}

/// Resolve a TenantContext from a raw API key by looking up the key hash
/// in the database, joining with the tenants table. Validates key status,
/// expiry, and tenant status.
pub async fn resolve_from_api_key(
	pg_pool: &PgPool,
	raw_key: &str,
	global_throttle: &ThrottleConfig,
) -> Result<TenantContext> {
	let key_hash = hash_api_key(raw_key);

	// Dual-probe: try current hash first, then fallback to legacy SHA-256
	// This ensures existing keys (hashed with bare SHA-256) still work after
	// deploying HMAC-SHA256. On match with legacy hash, rehash to HMAC.
	let legacy_hash = if get_hmac_secret().is_some() {
		// Compute legacy SHA-256 hash for fallback lookup
		use sha2::Digest;
		let mut hasher = Sha256::new();
		hasher.update(raw_key.as_bytes());
		Some(hex::encode(hasher.finalize()))
	} else {
		None // No HMAC configured, primary hash is already SHA-256
	};

	use sqlx::Row;

	let row = sqlx::query(
		r#"
		SELECT
			ak.id as api_key_id,
			ak.key_hash,
			ak.status::TEXT as api_key_status,
			ak.expires_at,
			ak.scopes,
			t.id as tenant_id,
			t.name as tenant_name,
			t.plan_tier::TEXT as plan_tier,
			t.status::TEXT as tenant_status,
			t.max_requests_per_second,
			t.max_requests_per_minute,
			t.max_requests_per_hour,
			t.max_requests_per_day,
			t.monthly_email_limit,
			t.used_this_period,
			t.period_reset_at,
			t.default_webhook_url,
			t.webhook_signing_secret,
			t.result_retention_days
		FROM api_keys ak
		JOIN tenants t ON ak.tenant_id = t.id
		WHERE ak.key_hash = $1 OR ak.key_hash = $2
		"#,
	)
	.bind(&key_hash)
	.bind(legacy_hash.as_deref().unwrap_or(""))
	.fetch_optional(pg_pool)
	.await?;

	let row = match row {
		Some(r) => r,
		None => bail!("Invalid API key"),
	};

	let key_hash_from_db: String = row.get("key_hash");

	// If matched via legacy hash (not the current HMAC hash), upgrade in background
	let matched_via_legacy = legacy_hash
		.as_deref()
		.map_or(false, |lh| key_hash_from_db == lh);
	if matched_via_legacy {
		let pool = pg_pool.clone();
		let new_hash = key_hash.clone();
		let api_key_id: Uuid = row.get("api_key_id");
		tokio::spawn(async move {
			let _ = sqlx::query("UPDATE api_keys SET key_hash = $1 WHERE id = $2")
				.bind(new_hash)
				.bind(api_key_id)
				.execute(&pool)
				.await;
		});
	}

	// Validate API key status
	let api_key_status: String = row.get("api_key_status");
	match api_key_status.as_str() {
		"revoked" => bail!("API key has been revoked"),
		"expired" => bail!("API key has expired"),
		"active" => {}
		_ => bail!("Unknown API key status: {}", api_key_status),
	}

	// Check expiry
	let expires_at: Option<chrono::DateTime<Utc>> = row.get("expires_at");
	if let Some(expires_at) = expires_at {
		if expires_at < Utc::now() {
			bail!("API key has expired");
		}
	}

	// Validate tenant status
	let tenant_status_str: String = row.get("tenant_status");
	let tenant_status = match tenant_status_str.as_str() {
		"active" => TenantStatus::Active,
		"suspended" => {
			bail!("Tenant account is suspended")
		}
		"deactivated" => {
			bail!("Tenant account is deactivated")
		}
		_ => bail!("Unknown tenant status: {}", tenant_status_str),
	};

	let plan_tier_str: String = row.get("plan_tier");
	let plan_tier = match plan_tier_str.as_str() {
		"free" => PlanTier::Free,
		"starter" => PlanTier::Starter,
		"professional" => PlanTier::Professional,
		"enterprise" => PlanTier::Enterprise,
		_ => bail!("Unknown plan tier: {}", plan_tier_str),
	};

	// Build throttle config: use tenant overrides where present, fall back to global
	let throttle = ThrottleConfig {
		max_requests_per_second: row
			.get::<Option<i32>, _>("max_requests_per_second")
			.map(|v| v as u32)
			.or(global_throttle.max_requests_per_second),
		max_requests_per_minute: row
			.get::<Option<i32>, _>("max_requests_per_minute")
			.map(|v| v as u32)
			.or(global_throttle.max_requests_per_minute),
		max_requests_per_hour: row
			.get::<Option<i32>, _>("max_requests_per_hour")
			.map(|v| v as u32)
			.or(global_throttle.max_requests_per_hour),
		max_requests_per_day: row
			.get::<Option<i32>, _>("max_requests_per_day")
			.map(|v| v as u32)
			.or(global_throttle.max_requests_per_day),
	};

	let api_key_id: Uuid = row.get("api_key_id");
	let scopes: Vec<String> = row.get("scopes");

	// Fire-and-forget update of last_used_at
	let pool = pg_pool.clone();
	tokio::spawn(async move {
		let _ = update_last_used_at(&pool, api_key_id).await;
	});

	Ok(TenantContext {
		tenant_id: Some(row.get("tenant_id")),
		api_key_id: Some(api_key_id),
		tenant_name: row.get("tenant_name"),
		plan_tier,
		status: tenant_status,
		throttle,
		monthly_email_limit: row.get("monthly_email_limit"),
		used_this_period: row.get("used_this_period"),
		period_reset_at: row.get("period_reset_at"),
		default_webhook_url: row.get("default_webhook_url"),
		webhook_signing_secret: row.get("webhook_signing_secret"),
		result_retention_days: row.get("result_retention_days"),
		is_legacy: false,
		scopes,
	})
}

/// Resolve a TenantContext directly from a tenant UUID.
/// Used by background tasks (e.g. reverification) that don't have an API key.
pub async fn resolve_tenant_context_by_id(
	pg_pool: &PgPool,
	tenant_id: Uuid,
) -> Result<TenantContext> {
	let row = sqlx::query(
		r#"
		SELECT
			id, name, plan_tier::TEXT, status::TEXT,
			max_requests_per_second, max_requests_per_minute,
			max_requests_per_hour, max_requests_per_day,
			monthly_email_limit, used_this_period, period_reset_at,
			default_webhook_url, webhook_signing_secret, result_retention_days
		FROM tenants
		WHERE id = $1
		"#,
	)
	.bind(tenant_id)
	.fetch_optional(pg_pool)
	.await?;

	let row = match row {
		Some(r) => r,
		None => bail!("Tenant not found: {}", tenant_id),
	};

	use sqlx::Row;
	let status_str: String = row.get("status");
	let tenant_status: TenantStatus = match status_str.as_str() {
		"active" => TenantStatus::Active,
		"suspended" => TenantStatus::Suspended,
		"deactivated" => TenantStatus::Deactivated,
		_ => bail!("Unknown tenant status: {}", status_str),
	};

	match tenant_status {
		TenantStatus::Suspended => bail!("Tenant account is suspended"),
		TenantStatus::Deactivated => bail!("Tenant account is deactivated"),
		TenantStatus::Active => {}
	}

	let plan_str: String = row.get("plan_tier");
	let plan_tier: PlanTier = match plan_str.as_str() {
		"free" => PlanTier::Free,
		"starter" => PlanTier::Starter,
		"professional" => PlanTier::Professional,
		"enterprise" => PlanTier::Enterprise,
		_ => bail!("Unknown plan tier: {}", plan_str),
	};

	// No global throttle fallback: background tasks (reverification) run at
	// the tenant's own rate limits without inheriting server-wide defaults.
	// This differs from resolve_from_api_key which merges with global_throttle.
	let throttle = ThrottleConfig {
		max_requests_per_second: row
			.get::<Option<i32>, _>("max_requests_per_second")
			.map(|v| v as u32),
		max_requests_per_minute: row
			.get::<Option<i32>, _>("max_requests_per_minute")
			.map(|v| v as u32),
		max_requests_per_hour: row
			.get::<Option<i32>, _>("max_requests_per_hour")
			.map(|v| v as u32),
		max_requests_per_day: row
			.get::<Option<i32>, _>("max_requests_per_day")
			.map(|v| v as u32),
	};

	Ok(TenantContext {
		tenant_id: Some(row.get("id")),
		api_key_id: None,
		tenant_name: row.get("name"),
		plan_tier,
		status: tenant_status,
		throttle,
		monthly_email_limit: row.get("monthly_email_limit"),
		used_this_period: row.get("used_this_period"),
		period_reset_at: row.get("period_reset_at"),
		default_webhook_url: row.get("default_webhook_url"),
		webhook_signing_secret: row.get("webhook_signing_secret"),
		result_retention_days: row.get("result_retention_days"),
		is_legacy: false,
		scopes: vec![],
	})
}

/// Update the last_used_at timestamp for an API key.
async fn update_last_used_at(pg_pool: &PgPool, api_key_id: Uuid) -> Result<()> {
	sqlx::query!(
		"UPDATE api_keys SET last_used_at = NOW() WHERE id = $1",
		api_key_id
	)
	.execute(pg_pool)
	.await?;
	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_generate_api_key() {
		let (full_key, prefix, hash) = generate_api_key();
		assert!(full_key.starts_with(API_KEY_PREFIX));
		assert_eq!(prefix.len(), 24);
		assert_eq!(hash, hash_api_key(&full_key));
	}

	#[test]
	fn test_hash_round_trip() {
		let key = "rch_live_abcdef1234567890abcdef12";
		let hash1 = hash_api_key(key);
		let hash2 = hash_api_key(key);
		assert_eq!(hash1, hash2);
		assert_ne!(hash1, hash_api_key("rch_live_different_key_entirely"));
	}
}
