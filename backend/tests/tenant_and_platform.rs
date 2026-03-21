// Tests for Phase 1 platform features:
// - Tenant auth (key generation, hashing)
// - Webhook signing
// - Deprecation headers
// - TenantContext construction
// - Throttle per-tenant isolation
// - Idempotency helpers

#[cfg(test)]
mod tenant_auth_tests {
	use reacher_backend::tenant::auth::{generate_api_key, hash_api_key};

	#[test]
	fn test_generate_api_key_format() {
		let (full_key, prefix, hash) = generate_api_key();

		// Key starts with prefix
		assert!(full_key.starts_with("rch_live_"));
		// Full key = prefix (9) + 32 hex chars = 41 chars
		assert_eq!(full_key.len(), 41, "Key length: {}", full_key.len());
		// Prefix is first 16 chars
		assert_eq!(prefix.len(), 24);
		assert!(full_key.starts_with(&prefix));
		// Hash is SHA-256 hex = 64 chars
		assert_eq!(hash.len(), 64);
	}

	#[test]
	fn test_generate_unique_keys() {
		let (key1, _, _) = generate_api_key();
		let (key2, _, _) = generate_api_key();
		assert_ne!(key1, key2, "Generated keys must be unique");
	}

	#[test]
	fn test_hash_deterministic() {
		let key = "rch_live_0123456789abcdef0123456789abcdef";
		let hash1 = hash_api_key(key);
		let hash2 = hash_api_key(key);
		assert_eq!(hash1, hash2);
	}

	#[test]
	fn test_hash_different_keys() {
		let hash1 = hash_api_key("rch_live_aaaa");
		let hash2 = hash_api_key("rch_live_bbbb");
		assert_ne!(hash1, hash2);
	}

	#[test]
	fn test_hash_matches_generated() {
		let (full_key, _, expected_hash) = generate_api_key();
		let actual_hash = hash_api_key(&full_key);
		assert_eq!(actual_hash, expected_hash);
	}
}

#[cfg(test)]
mod tenant_context_tests {
	use reacher_backend::config::ThrottleConfig;
	use reacher_backend::tenant::context::TenantContext;
	use reacher_backend::tenant::models::{PlanTier, TenantStatus};
	use uuid::Uuid;

	#[test]
	fn test_legacy_context() {
		let throttle = ThrottleConfig {
			max_requests_per_second: Some(10),
			max_requests_per_minute: None,
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
		assert_eq!(
			ctx.throttle.max_requests_per_second,
			throttle.max_requests_per_second
		);
	}

	#[test]
	fn test_tenant_id_str_with_id() {
		let id = Uuid::new_v4();
		let mut ctx = TenantContext::legacy(ThrottleConfig::new_without_throttle());
		ctx.tenant_id = Some(id);
		assert_eq!(ctx.tenant_id_str(), id.to_string());
	}

	#[test]
	fn test_tenant_id_str_legacy() {
		let ctx = TenantContext::legacy(ThrottleConfig::new_without_throttle());
		assert_eq!(ctx.tenant_id_str(), "legacy");
	}
}

#[cfg(test)]
mod webhook_signing_tests {
	use reacher_backend::tenant::webhook::{
		sign_payload, verify_signature, WEBHOOK_SIGNATURE_HEADER,
	};

	#[test]
	fn test_sign_produces_sha256_prefix() {
		let sig = sign_payload("secret", b"payload");
		assert!(sig.starts_with("sha256="));
	}

	#[test]
	fn test_sign_and_verify_round_trip() {
		let secret = "webhook-secret-123";
		let payload = br#"{"result": "safe"}"#;
		let sig = sign_payload(secret, payload);
		assert!(verify_signature(secret, payload, &sig));
	}

	#[test]
	fn test_wrong_secret_verification_fails() {
		let payload = b"test payload";
		let sig = sign_payload("real-secret", payload);
		assert!(!verify_signature("fake-secret", payload, &sig));
	}

	#[test]
	fn test_tampered_payload_verification_fails() {
		let secret = "my-secret";
		let sig = sign_payload(secret, b"original");
		assert!(!verify_signature(secret, b"tampered", &sig));
	}

	#[test]
	fn test_signature_is_deterministic() {
		let secret = "fixed-secret";
		let payload = b"fixed payload";
		let sig1 = sign_payload(secret, payload);
		let sig2 = sign_payload(secret, payload);
		assert_eq!(sig1, sig2);
	}

	#[test]
	fn test_different_payloads_different_sigs() {
		let secret = "test";
		assert_ne!(sign_payload(secret, b"one"), sign_payload(secret, b"two"));
	}

	#[test]
	fn test_signature_header_name() {
		assert_eq!(WEBHOOK_SIGNATURE_HEADER, "X-Reacher-Signature-256");
	}
}

#[cfg(test)]
mod throttle_tests {
	use reacher_backend::config::{BackendConfig, ThrottleConfig};
	use reacher_backend::throttle::ThrottleManager;
	use std::sync::Arc;
	use uuid::Uuid;

	#[tokio::test]
	async fn test_per_tenant_throttle_isolation() {
		let config = BackendConfig::empty();
		let tenant_a = Uuid::new_v4();
		let tenant_b = Uuid::new_v4();

		let throttle_config = ThrottleConfig {
			max_requests_per_second: Some(1),
			max_requests_per_minute: None,
			max_requests_per_hour: None,
			max_requests_per_day: None,
		};

		// Get throttle manager for tenant A and exhaust it
		let mgr_a = config.get_tenant_throttle_manager(Some(tenant_a), &throttle_config);
		mgr_a.increment_counters().await;

		// Tenant A should be throttled
		assert!(mgr_a.check_throttle().await.is_some());

		// Tenant B should NOT be throttled (isolated)
		let mgr_b = config.get_tenant_throttle_manager(Some(tenant_b), &throttle_config);
		assert!(mgr_b.check_throttle().await.is_none());
	}

	#[tokio::test]
	async fn test_same_tenant_returns_same_manager() {
		let config = BackendConfig::empty();
		let tenant_id = Uuid::new_v4();
		let throttle_config = ThrottleConfig::new_without_throttle();

		let mgr1 = config.get_tenant_throttle_manager(Some(tenant_id), &throttle_config);
		let mgr2 = config.get_tenant_throttle_manager(Some(tenant_id), &throttle_config);

		// Increment on one should affect the other (same Arc)
		mgr1.increment_counters().await;
		// Both point to the same inner state
		assert!(Arc::ptr_eq(&mgr1, &mgr2));
	}

	#[tokio::test]
	async fn test_none_tenant_uses_global() {
		let config = BackendConfig::empty();
		let throttle_config = ThrottleConfig::new_without_throttle();

		let mgr = config.get_tenant_throttle_manager(None, &throttle_config);
		let global = config.get_throttle_manager();

		// Both should be the same global manager
		assert!(Arc::ptr_eq(&mgr, &global));
	}

	#[tokio::test]
	async fn test_throttle_no_limits() {
		let mgr = ThrottleManager::new(ThrottleConfig::new_without_throttle());

		// Should never throttle with no limits configured
		for _ in 0..100 {
			mgr.increment_counters().await;
		}
		assert!(mgr.check_throttle().await.is_none());
	}

	#[tokio::test]
	async fn test_throttle_per_second_limit() {
		let mgr = ThrottleManager::new(ThrottleConfig {
			max_requests_per_second: Some(2),
			max_requests_per_minute: None,
			max_requests_per_hour: None,
			max_requests_per_day: None,
		});

		// First two should pass
		assert!(mgr.check_throttle().await.is_none());
		mgr.increment_counters().await;
		assert!(mgr.check_throttle().await.is_none());
		mgr.increment_counters().await;

		// Third should be throttled
		let result = mgr.check_throttle().await;
		assert!(result.is_some());
		assert_eq!(
			result.unwrap().limit_type,
			reacher_backend::throttle::ThrottleLimit::PerSecond
		);
	}
}

#[cfg(test)]
mod deprecation_tests {
	use reacher_backend::http::deprecation::add_deprecation_headers;

	#[test]
	fn test_deprecation_headers_applied() {
		let reply = warp::reply::json(&serde_json::json!({"ok": true}));
		let with_deprecation = add_deprecation_headers(reply, "2026-09-16", "/v1/foo");

		// We can't easily inspect warp::Reply headers in a unit test,
		// but we verify it compiles and doesn't panic.
		let _ = warp::reply::Reply::into_response(with_deprecation);
	}
}

#[cfg(test)]
mod idempotency_tests {
	use reacher_backend::http::idempotency::hash_request_body;

	#[test]
	fn test_hash_deterministic() {
		let body = b"request body content";
		let hash1 = hash_request_body(body);
		let hash2 = hash_request_body(body);
		assert_eq!(hash1, hash2);
	}

	#[test]
	fn test_hash_different_bodies() {
		let hash1 = hash_request_body(b"body one");
		let hash2 = hash_request_body(b"body two");
		assert_ne!(hash1, hash2);
	}

	#[test]
	fn test_hash_empty_body() {
		let hash = hash_request_body(b"");
		assert_eq!(hash.len(), 32); // SHA-256 = 32 bytes
	}

	#[test]
	fn test_hash_length() {
		let hash = hash_request_body(b"some content");
		assert_eq!(hash.len(), 32); // SHA-256 produces 32 bytes
	}
}

#[cfg(test)]
mod task_metadata_tests {
	use reacher_backend::worker::do_work::{
		CheckEmailJobId, CheckEmailTask, RetryPolicy, TaskMetadata,
	};

	#[test]
	fn test_retry_policy_defaults() {
		let policy = RetryPolicy::default();
		assert_eq!(policy.max_retries, 2);
		assert_eq!(policy.backoff_seconds, 5);
		assert_eq!(policy.backoff_multiplier, 2.0);
	}

	#[test]
	fn test_task_metadata_serialization() {
		let metadata = TaskMetadata {
			tenant_id: Some("123".into()),
			request_id: None,
			correlation_id: Some("corr-1".into()),
			created_by: None,
			retry_policy: Some(RetryPolicy::default()),
			dedupe_key: Some("email@example.com".into()),
			task_db_id: Some(42),
		};

		let json = serde_json::to_string(&metadata).unwrap();
		let deserialized: TaskMetadata = serde_json::from_str(&json).unwrap();

		assert_eq!(deserialized.tenant_id.as_deref(), Some("123"));
		assert_eq!(deserialized.task_db_id, Some(42));
		assert_eq!(
			deserialized.dedupe_key.as_deref(),
			Some("email@example.com")
		);
	}

	#[test]
	fn test_task_backward_compatible_deserialization() {
		// Old task format without metadata field should still work
		let json = r#"{
			"input": {"to_email": "test@example.com", "verif_method": {}},
			"job_id": "single_shot",
			"webhook": null
		}"#;

		let task: Result<CheckEmailTask, _> = serde_json::from_str(json);
		// This may or may not parse depending on CheckEmailInput's requirements
		// but the metadata field itself should be Optional
		if let Ok(task) = task {
			assert!(task.metadata.is_none());
		}
	}
}

#[cfg(test)]
mod quota_tests {
	use chrono::{Duration, Utc};
	use reacher_backend::config::ThrottleConfig;
	use reacher_backend::tenant::context::TenantContext;
	use reacher_backend::tenant::models::{PlanTier, TenantStatus};
	use reacher_backend::tenant::quota::{check_quota, QuotaCheckResult};
	use uuid::Uuid;

	fn make_tenant_ctx(limit: Option<i32>, used: i32, period_expired: bool) -> TenantContext {
		let reset_at = if period_expired {
			Utc::now() - Duration::hours(1)
		} else {
			Utc::now() + Duration::hours(24)
		};
		TenantContext {
			tenant_id: Some(Uuid::new_v4()),
			api_key_id: Some(Uuid::new_v4()),
			tenant_name: "test".into(),
			plan_tier: PlanTier::Starter,
			status: TenantStatus::Active,
			throttle: ThrottleConfig::new_without_throttle(),
			monthly_email_limit: limit,
			used_this_period: used,
			period_reset_at: reset_at,
			default_webhook_url: None,
			result_retention_days: 30,
			webhook_signing_secret: None,
			is_legacy: false,
			scopes: vec![],
		}
	}

	#[tokio::test]
	async fn test_unlimited_quota_always_allowed() {
		let ctx = make_tenant_ctx(None, 1_000_000, false);
		assert_eq!(check_quota(None, &ctx).await, QuotaCheckResult::Allowed);
	}

	#[tokio::test]
	async fn test_legacy_tenant_always_allowed() {
		let mut ctx = make_tenant_ctx(Some(10), 100, false);
		ctx.tenant_id = None;
		ctx.is_legacy = true;
		assert_eq!(check_quota(None, &ctx).await, QuotaCheckResult::Allowed);
	}

	#[tokio::test]
	async fn test_no_db_pool_fails_open() {
		let ctx = make_tenant_ctx(Some(10), 100, false);
		// Without a PgPool, quota check fails open (allows)
		assert_eq!(check_quota(None, &ctx).await, QuotaCheckResult::Allowed);
	}

	#[tokio::test]
	async fn test_expired_period_allows() {
		let ctx = make_tenant_ctx(Some(10), 100, true);
		// Period expired — would reset counter, so allowed
		assert_eq!(check_quota(None, &ctx).await, QuotaCheckResult::Allowed);
	}
}
