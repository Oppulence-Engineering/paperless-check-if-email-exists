mod test_helpers;

use crate::test_helpers::{
	ensure_test_amqp_url, insert_api_key_with_scopes, insert_tenant, test_db_url, TestDb,
};
use anyhow::Result;
use chrono::{Duration as ChronoDuration, Utc};
use reacher_backend::config::{
	BackendConfig, PostgresConfig, RabbitMQConfig, StorageConfig, WorkerConfig,
};
use reacher_backend::http::create_routes;
use reacher_backend::outcomes::{
	apply_post_verification_outcome_check, default_outcome_policy_rules, enrich_outcome_context,
	fetch_or_create_default_policy, ingest_outcomes, IngestOutcome, OutcomeType,
};
use reacher_backend::scoring::{compute_score_with_context, ScoringContext};
use serde_json::json;
use serial_test::serial;
use std::sync::Arc;
use uuid::Uuid;
use warp::http::StatusCode;
use warp::test::request;

// ----------------------------------------------------------------------------
// Setup
// ----------------------------------------------------------------------------

async fn worker_config() -> Arc<BackendConfig> {
	let amqp_url = ensure_test_amqp_url().await;
	let mut c = BackendConfig::empty();
	c.backend_name = "test-outcomes".to_string();
	c.storage = Some(StorageConfig::Postgres(PostgresConfig {
		db_url: test_db_url(),
		read_replica_url: None,
		extra: None,
	}));
	c.worker = WorkerConfig {
		enable: true,
		rabbitmq: Some(RabbitMQConfig {
			url: amqp_url,
			concurrency: 4,
		}),
		webhook: None,
	};
	c.connect().await.expect("config connect");
	Arc::new(c)
}

async fn count_outcomes(pool: &sqlx::PgPool, tenant_id: Uuid, email: &str) -> i64 {
	sqlx::query_scalar::<_, i64>(
		"SELECT COUNT(*) FROM verification_outcomes WHERE tenant_id = $1 AND canonical_email = $2",
	)
	.bind(tenant_id)
	.bind(email)
	.fetch_one(pool)
	.await
	.unwrap_or(0)
}

async fn count_suppressions(pool: &sqlx::PgPool, tenant_id: Uuid, email: &str) -> i64 {
	sqlx::query_scalar::<_, i64>(
		"SELECT COUNT(*) FROM v1_suppression_entries WHERE tenant_id = $1 AND email = $2",
	)
	.bind(tenant_id)
	.bind(email)
	.fetch_one(pool)
	.await
	.unwrap_or(0)
}

async fn fetch_suppression_reason(pool: &sqlx::PgPool, tenant_id: Uuid, email: &str) -> String {
	sqlx::query_scalar::<_, String>(
		"SELECT reason::TEXT FROM v1_suppression_entries WHERE tenant_id = $1 AND email = $2 ORDER BY id DESC LIMIT 1",
	)
	.bind(tenant_id)
	.bind(email)
	.fetch_one(pool)
	.await
	.unwrap_or_default()
}

fn ingest(email: &str, outcome_type: OutcomeType) -> IngestOutcome {
	IngestOutcome {
		email: email.to_string(),
		outcome_type,
		occurred_at: Utc::now(),
		source: Some("test".to_string()),
		campaign_id: None,
		metadata: None,
	}
}

// ----------------------------------------------------------------------------
// Ingest tests
// ----------------------------------------------------------------------------

#[tokio::test]
#[serial]
async fn test_post_single_outcome_creates_row() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "outcomes-single", Some(1000), 0).await;
	let (api_key, _) = insert_api_key_with_scopes(db.pool(), tenant_id, &["outcomes.write"]).await;
	let config = worker_config().await;

	let response = request()
		.method("POST")
		.path("/v1/outcomes")
		.header("authorization", format!("Bearer {}", api_key))
		.json(&json!({
			"outcomes": [{
				"email": "User@Example.com",
				"type": "delivered",
				"occurred_at": "2026-05-10T12:00:00Z",
				"source": "sendgrid"
			}]
		}))
		.reply(&create_routes(Arc::clone(&config)))
		.await;
	assert_eq!(response.status(), StatusCode::ACCEPTED);

	assert_eq!(
		count_outcomes(db.pool(), tenant_id, "user@example.com").await,
		1
	);
	let body: serde_json::Value = serde_json::from_slice(response.body())?;
	assert_eq!(body["accepted"], 1);
	assert_eq!(body["rejected"], 0);
	Ok(())
}

#[tokio::test]
#[serial]
async fn test_post_batch_outcomes_all_stored() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "outcomes-batch", Some(10000), 0).await;
	let (api_key, _) = insert_api_key_with_scopes(db.pool(), tenant_id, &["outcomes.write"]).await;
	let config = worker_config().await;

	let mut outcomes = Vec::new();
	for i in 0..50 {
		outcomes.push(json!({
			"email": format!("user{}@example.com", i),
			"type": "delivered",
			"occurred_at": format!("2026-05-{:02}T12:00:00Z", (i % 28) + 1),
			"source": "sendgrid"
		}));
	}
	let response = request()
		.method("POST")
		.path("/v1/outcomes")
		.header("authorization", format!("Bearer {}", api_key))
		.json(&json!({ "outcomes": outcomes }))
		.reply(&create_routes(Arc::clone(&config)))
		.await;
	assert_eq!(response.status(), StatusCode::ACCEPTED);

	let body: serde_json::Value = serde_json::from_slice(response.body())?;
	assert_eq!(body["accepted"], 50);
	let total: i64 =
		sqlx::query_scalar("SELECT COUNT(*) FROM verification_outcomes WHERE tenant_id = $1")
			.bind(tenant_id)
			.fetch_one(db.pool())
			.await?;
	assert_eq!(total, 50);
	Ok(())
}

#[tokio::test]
#[serial]
async fn test_post_outcomes_invalid_type_rejected() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "outcomes-invalid-type", Some(1000), 0).await;
	let (api_key, _) = insert_api_key_with_scopes(db.pool(), tenant_id, &["outcomes.write"]).await;
	let config = worker_config().await;

	let response = request()
		.method("POST")
		.path("/v1/outcomes")
		.header("authorization", format!("Bearer {}", api_key))
		.json(&json!({
			"outcomes": [{
				"email": "x@y.com",
				"type": "totally_made_up",
				"occurred_at": "2026-05-10T12:00:00Z"
			}]
		}))
		.reply(&create_routes(Arc::clone(&config)))
		.await;
	assert_eq!(response.status(), StatusCode::BAD_REQUEST);
	Ok(())
}

#[tokio::test]
#[serial]
async fn test_post_outcomes_missing_scope_rejected() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "outcomes-no-scope", Some(1000), 0).await;
	let (api_key, _) = insert_api_key_with_scopes(db.pool(), tenant_id, &["bulk"]).await;
	let config = worker_config().await;

	let response = request()
		.method("POST")
		.path("/v1/outcomes")
		.header("authorization", format!("Bearer {}", api_key))
		.json(&json!({
			"outcomes": [{
				"email": "x@y.com",
				"type": "delivered",
				"occurred_at": "2026-05-10T12:00:00Z"
			}]
		}))
		.reply(&create_routes(Arc::clone(&config)))
		.await;
	assert_eq!(response.status(), StatusCode::FORBIDDEN);
	Ok(())
}

#[tokio::test]
#[serial]
async fn test_post_outcomes_idempotent_duplicate_suppressed() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "outcomes-idempotent", Some(1000), 0).await;
	let policy = default_outcome_policy_rules();
	let outcomes = vec![ingest("dup@example.com", OutcomeType::Delivered)];

	let s1 = ingest_outcomes(db.pool(), tenant_id, &policy, &outcomes).await;
	let s2 = ingest_outcomes(db.pool(), tenant_id, &policy, &outcomes).await;
	assert_eq!(s1.accepted, 1);
	assert_eq!(s2.accepted, 1, "second call still 'accepts' the request");
	assert_eq!(
		count_outcomes(db.pool(), tenant_id, "dup@example.com").await,
		1
	);
	Ok(())
}

#[tokio::test]
#[serial]
async fn test_post_outcomes_cross_tenant_isolation() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_a = insert_tenant(db.pool(), "outcomes-iso-a", Some(1000), 0).await;
	let tenant_b = insert_tenant(db.pool(), "outcomes-iso-b", Some(1000), 0).await;
	let policy = default_outcome_policy_rules();

	ingest_outcomes(
		db.pool(),
		tenant_a,
		&policy,
		&[ingest("shared@example.com", OutcomeType::HardBounce)],
	)
	.await;

	assert_eq!(
		count_outcomes(db.pool(), tenant_a, "shared@example.com").await,
		1
	);
	assert_eq!(
		count_outcomes(db.pool(), tenant_b, "shared@example.com").await,
		0
	);
	Ok(())
}

#[tokio::test]
#[serial]
async fn test_csv_upload_happy_path() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "outcomes-csv", Some(1000), 0).await;
	let (api_key, _) = insert_api_key_with_scopes(db.pool(), tenant_id, &["outcomes.write"]).await;
	let config = worker_config().await;

	let csv = b"email,outcome_type,occurred_at,source\n\
	            a@example.com,delivered,2026-05-10T12:00:00Z,sendgrid\n\
	            b@example.com,hard_bounce,2026-05-10T12:01:00Z,sendgrid\n";
	let body = build_multipart_csv("file", "outcomes.csv", csv);

	let response = request()
		.method("POST")
		.path("/v1/outcomes/upload")
		.header("authorization", format!("Bearer {}", api_key))
		.header(
			"content-type",
			"multipart/form-data; boundary=----TestBoundary",
		)
		.body(body)
		.reply(&create_routes(Arc::clone(&config)))
		.await;
	assert_eq!(
		response.status(),
		StatusCode::ACCEPTED,
		"body: {:?}",
		response.body()
	);

	let body: serde_json::Value = serde_json::from_slice(response.body())?;
	assert_eq!(body["accepted"], 2);
	assert_eq!(body["rejected"], 0);
	Ok(())
}

#[tokio::test]
#[serial]
async fn test_csv_upload_partial_rejection_keeps_valid_rows() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "outcomes-csv-mixed", Some(1000), 0).await;
	let (api_key, _) = insert_api_key_with_scopes(db.pool(), tenant_id, &["outcomes.write"]).await;
	let config = worker_config().await;

	let csv = b"email,outcome_type,occurred_at,source\n\
	            ok@example.com,delivered,2026-05-10T12:00:00Z,sendgrid\n\
	            ,hard_bounce,2026-05-10T12:01:00Z,sendgrid\n\
	            bad@example.com,not_a_type,2026-05-10T12:02:00Z,sendgrid\n";
	let body = build_multipart_csv("file", "outcomes.csv", csv);

	let response = request()
		.method("POST")
		.path("/v1/outcomes/upload")
		.header("authorization", format!("Bearer {}", api_key))
		.header(
			"content-type",
			"multipart/form-data; boundary=----TestBoundary",
		)
		.body(body)
		.reply(&create_routes(Arc::clone(&config)))
		.await;
	assert_eq!(response.status(), StatusCode::ACCEPTED);

	let body: serde_json::Value = serde_json::from_slice(response.body())?;
	assert_eq!(body["accepted"], 1, "only ok@ should accept");
	assert_eq!(body["rejected"], 2);
	let errors = body["errors"].as_array().unwrap();
	assert_eq!(errors.len(), 2);
	Ok(())
}

// ----------------------------------------------------------------------------
// Policy CRUD tests
// ----------------------------------------------------------------------------

#[tokio::test]
#[serial]
async fn test_outcome_policy_full_crud() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "outcomes-crud", Some(1000), 0).await;
	let (api_key, _) = insert_api_key_with_scopes(db.pool(), tenant_id, &["settings"]).await;
	let config = worker_config().await;
	let routes = create_routes(Arc::clone(&config));

	let create = request()
		.method("POST")
		.path("/v1/outcome-policies")
		.header("authorization", format!("Bearer {}", api_key))
		.json(&json!({"name":"production","is_default":true,"rules": serde_json::to_value(default_outcome_policy_rules()).unwrap()}))
		.reply(&routes)
		.await;
	assert_eq!(create.status(), StatusCode::CREATED);
	let body: serde_json::Value = serde_json::from_slice(create.body())?;
	let policy_id = body["id"].as_i64().unwrap();

	let list = request()
		.method("GET")
		.path("/v1/outcome-policies")
		.header("authorization", format!("Bearer {}", api_key))
		.reply(&routes)
		.await;
	assert_eq!(list.status(), StatusCode::OK);
	let body: serde_json::Value = serde_json::from_slice(list.body())?;
	assert_eq!(body["total"], 1);

	let patch = request()
		.method("PATCH")
		.path(&format!("/v1/outcome-policies/{}", policy_id))
		.header("authorization", format!("Bearer {}", api_key))
		.json(&json!({"name":"production-v2"}))
		.reply(&routes)
		.await;
	assert_eq!(patch.status(), StatusCode::OK);
	let body: serde_json::Value = serde_json::from_slice(patch.body())?;
	assert_eq!(body["name"], "production-v2");

	let del = request()
		.method("DELETE")
		.path(&format!("/v1/outcome-policies/{}", policy_id))
		.header("authorization", format!("Bearer {}", api_key))
		.reply(&routes)
		.await;
	assert_eq!(del.status(), StatusCode::OK);

	let count: i64 =
		sqlx::query_scalar("SELECT COUNT(*) FROM v1_outcome_policies WHERE tenant_id = $1")
			.bind(tenant_id)
			.fetch_one(db.pool())
			.await?;
	assert_eq!(count, 0);
	Ok(())
}

#[tokio::test]
#[serial]
async fn test_outcome_policy_unique_default_per_tenant() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "outcomes-default", Some(1000), 0).await;

	let rules = serde_json::to_value(default_outcome_policy_rules())?;
	sqlx::query(
		"INSERT INTO v1_outcome_policies (tenant_id, name, is_default, rules) VALUES ($1, 'a', true, $2)",
	)
	.bind(tenant_id)
	.bind(&rules)
	.execute(db.pool())
	.await?;

	let dup = sqlx::query(
		"INSERT INTO v1_outcome_policies (tenant_id, name, is_default, rules) VALUES ($1, 'b', true, $2)",
	)
	.bind(tenant_id)
	.bind(&rules)
	.execute(db.pool())
	.await;
	assert!(
		dup.is_err(),
		"second is_default=true should violate unique partial index"
	);
	Ok(())
}

#[tokio::test]
#[serial]
async fn test_lazy_default_policy_created_on_first_ingest() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "outcomes-lazy", Some(1000), 0).await;

	let before: i64 =
		sqlx::query_scalar("SELECT COUNT(*) FROM v1_outcome_policies WHERE tenant_id = $1")
			.bind(tenant_id)
			.fetch_one(db.pool())
			.await?;
	assert_eq!(before, 0);

	let (id, rules) = fetch_or_create_default_policy(db.pool(), tenant_id).await;
	assert!(id > 0);
	assert_eq!(rules.outcome_ttl_days, 90);

	let after: i64 = sqlx::query_scalar(
		"SELECT COUNT(*) FROM v1_outcome_policies WHERE tenant_id = $1 AND is_default = true",
	)
	.bind(tenant_id)
	.fetch_one(db.pool())
	.await?;
	assert_eq!(after, 1);
	Ok(())
}

#[tokio::test]
#[serial]
async fn test_outcome_policy_invalid_rules_rejected() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "outcomes-bad-rules", Some(1000), 0).await;
	let (api_key, _) = insert_api_key_with_scopes(db.pool(), tenant_id, &["settings"]).await;
	let config = worker_config().await;

	let response = request()
		.method("POST")
		.path("/v1/outcome-policies")
		.header("authorization", format!("Bearer {}", api_key))
		.json(&json!({"name":"bogus","rules":{"hard_bounce":{"action":"completely_fake"}}}))
		.reply(&create_routes(Arc::clone(&config)))
		.await;
	assert_eq!(response.status(), StatusCode::BAD_REQUEST);
	Ok(())
}

#[tokio::test]
#[serial]
async fn test_outcome_policy_patch_swap_default() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "outcomes-swap", Some(1000), 0).await;
	let (api_key, _) = insert_api_key_with_scopes(db.pool(), tenant_id, &["settings"]).await;
	let config = worker_config().await;
	let routes = create_routes(Arc::clone(&config));

	let rules = serde_json::to_value(default_outcome_policy_rules())?;
	let id_a: i64 = sqlx::query_scalar(
		"INSERT INTO v1_outcome_policies (tenant_id, name, is_default, rules) VALUES ($1, 'a', true, $2) RETURNING id",
	)
	.bind(tenant_id)
	.bind(&rules)
	.fetch_one(db.pool())
	.await?;
	let id_b: i64 = sqlx::query_scalar(
		"INSERT INTO v1_outcome_policies (tenant_id, name, is_default, rules) VALUES ($1, 'b', false, $2) RETURNING id",
	)
	.bind(tenant_id)
	.bind(&rules)
	.fetch_one(db.pool())
	.await?;

	let patch = request()
		.method("PATCH")
		.path(&format!("/v1/outcome-policies/{}", id_b))
		.header("authorization", format!("Bearer {}", api_key))
		.json(&json!({"is_default": true}))
		.reply(&routes)
		.await;
	assert_eq!(patch.status(), StatusCode::OK);

	let a_default: bool =
		sqlx::query_scalar("SELECT is_default FROM v1_outcome_policies WHERE id = $1")
			.bind(id_a)
			.fetch_one(db.pool())
			.await?;
	let b_default: bool =
		sqlx::query_scalar("SELECT is_default FROM v1_outcome_policies WHERE id = $1")
			.bind(id_b)
			.fetch_one(db.pool())
			.await?;
	assert!(!a_default);
	assert!(b_default);
	Ok(())
}

// ----------------------------------------------------------------------------
// Suppression integration
// ----------------------------------------------------------------------------

#[tokio::test]
#[serial]
async fn test_hard_bounce_creates_suppression() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "outcomes-hb-supp", Some(1000), 0).await;
	let policy = default_outcome_policy_rules();
	ingest_outcomes(
		db.pool(),
		tenant_id,
		&policy,
		&[ingest("hb@example.com", OutcomeType::HardBounce)],
	)
	.await;
	assert_eq!(
		count_suppressions(db.pool(), tenant_id, "hb@example.com").await,
		1
	);
	assert_eq!(
		fetch_suppression_reason(db.pool(), tenant_id, "hb@example.com").await,
		"bounce"
	);
	Ok(())
}

#[tokio::test]
#[serial]
async fn test_complaint_creates_suppression() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "outcomes-cmp-supp", Some(1000), 0).await;
	let policy = default_outcome_policy_rules();
	ingest_outcomes(
		db.pool(),
		tenant_id,
		&policy,
		&[ingest("cmp@example.com", OutcomeType::Complaint)],
	)
	.await;
	assert!(count_suppressions(db.pool(), tenant_id, "cmp@example.com").await >= 1);
	let reason = fetch_suppression_reason(db.pool(), tenant_id, "cmp@example.com").await;
	assert!(
		reason == "complaint" || reason == "unsubscribe",
		"expected complaint/unsubscribe, got {}",
		reason
	);
	Ok(())
}

#[tokio::test]
#[serial]
async fn test_soft_bounce_threshold_required_for_suppression() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "outcomes-sb-thresh", Some(1000), 0).await;
	let policy = default_outcome_policy_rules();

	for i in 0..2 {
		let mut o = ingest("sb@example.com", OutcomeType::SoftBounce);
		o.occurred_at = Utc::now() - ChronoDuration::days(i);
		o.source = Some(format!("src-{}", i));
		ingest_outcomes(db.pool(), tenant_id, &policy, &[o]).await;
	}
	assert_eq!(
		count_suppressions(db.pool(), tenant_id, "sb@example.com").await,
		0
	);

	let mut third = ingest("sb@example.com", OutcomeType::SoftBounce);
	third.occurred_at = Utc::now() - ChronoDuration::days(2);
	third.source = Some("src-2".to_string());
	ingest_outcomes(db.pool(), tenant_id, &policy, &[third]).await;
	assert_eq!(
		count_suppressions(db.pool(), tenant_id, "sb@example.com").await,
		1
	);
	Ok(())
}

#[tokio::test]
#[serial]
async fn test_apply_post_verification_hook_suppresses_when_outcome_exists() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "outcomes-post-hook", Some(1000), 0).await;
	let policy = default_outcome_policy_rules();
	ingest_outcomes(
		db.pool(),
		tenant_id,
		&policy,
		&[ingest("posthook@example.com", OutcomeType::HardBounce)],
	)
	.await;
	sqlx::query("DELETE FROM v1_suppression_entries WHERE tenant_id = $1 AND email = $2")
		.bind(tenant_id)
		.bind("posthook@example.com")
		.execute(db.pool())
		.await?;
	assert_eq!(
		count_suppressions(db.pool(), tenant_id, "posthook@example.com").await,
		0
	);

	apply_post_verification_outcome_check(db.pool(), tenant_id, "posthook@example.com").await;
	assert_eq!(
		count_suppressions(db.pool(), tenant_id, "posthook@example.com").await,
		1
	);
	Ok(())
}

// ----------------------------------------------------------------------------
// Scoring integration
// ----------------------------------------------------------------------------

#[tokio::test]
#[serial]
async fn test_scoring_unaffected_when_no_outcomes() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "outcomes-no-data", Some(1000), 0).await;
	let ctx = enrich_outcome_context(
		Some(db.pool()),
		Some(tenant_id),
		"absent@example.com",
		Utc::now(),
	)
	.await;
	assert!(!ctx.forces_invalid());
	assert_eq!(ctx.engagement_boost(), 0);
	Ok(())
}

#[tokio::test]
#[serial]
async fn test_hard_bounce_forces_invalid_in_scoring() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "outcomes-score-hb", Some(1000), 0).await;
	let policy = default_outcome_policy_rules();
	ingest_outcomes(
		db.pool(),
		tenant_id,
		&policy,
		&[ingest("hb@example.com", OutcomeType::HardBounce)],
	)
	.await;

	let ctx = enrich_outcome_context(
		Some(db.pool()),
		Some(tenant_id),
		"hb@example.com",
		Utc::now(),
	)
	.await;
	assert!(ctx.has_hard_bounce);
	assert!(ctx.forces_invalid());

	let mut output = check_if_email_exists::CheckEmailOutput::default();
	output.input = "hb@example.com".to_string();
	output.is_reachable = check_if_email_exists::Reachable::Safe;

	let scoring_ctx = ScoringContext {
		outcomes: ctx,
		..Default::default()
	};
	let result = compute_score_with_context(&output, &scoring_ctx);
	assert_eq!(
		result.score.category,
		reacher_backend::scoring::EmailCategory::Invalid
	);
	assert!(!result.score.safe_to_send);
	assert!(result
		.score
		.reason_codes
		.contains(&"outcome_hard_bounce".to_string()));
	Ok(())
}

#[tokio::test]
#[serial]
async fn test_complaint_forces_invalid_in_scoring() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "outcomes-score-cmp", Some(1000), 0).await;
	let policy = default_outcome_policy_rules();
	ingest_outcomes(
		db.pool(),
		tenant_id,
		&policy,
		&[ingest("cmp@example.com", OutcomeType::Complaint)],
	)
	.await;
	let ctx = enrich_outcome_context(
		Some(db.pool()),
		Some(tenant_id),
		"cmp@example.com",
		Utc::now(),
	)
	.await;
	assert!(ctx.has_complaint);

	let mut output = check_if_email_exists::CheckEmailOutput::default();
	output.input = "cmp@example.com".to_string();
	output.is_reachable = check_if_email_exists::Reachable::Safe;

	let result = compute_score_with_context(
		&output,
		&ScoringContext {
			outcomes: ctx,
			..Default::default()
		},
	);
	assert_eq!(
		result.score.category,
		reacher_backend::scoring::EmailCategory::Invalid
	);
	assert!(result
		.score
		.reason_codes
		.contains(&"outcome_complaint".to_string()));
	Ok(())
}

#[tokio::test]
#[serial]
async fn test_engagement_boosts_score() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "outcomes-score-engage", Some(1000), 0).await;
	let policy = default_outcome_policy_rules();
	let outcomes = vec![
		ingest("eng@example.com", OutcomeType::Delivered),
		ingest("eng@example.com", OutcomeType::Open),
		ingest("eng@example.com", OutcomeType::Click),
	];
	let mut owned = outcomes.clone();
	for (i, o) in owned.iter_mut().enumerate() {
		o.source = Some(format!("src-{}", i));
	}
	ingest_outcomes(db.pool(), tenant_id, &policy, &owned).await;

	let ctx = enrich_outcome_context(
		Some(db.pool()),
		Some(tenant_id),
		"eng@example.com",
		Utc::now(),
	)
	.await;
	assert!(ctx.delivered_count >= 1);
	assert!(ctx.open_count >= 1);
	assert!(ctx.click_count >= 1);
	assert_eq!(ctx.engagement_boost(), 15);
	Ok(())
}

#[tokio::test]
#[serial]
async fn test_old_outcomes_ignored_by_ttl() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "outcomes-ttl", Some(1000), 0).await;

	let custom_rules = json!({
		"hard_bounce":  { "action": "suppress", "score_override": "invalid" },
		"complaint":    { "action": "suppress_and_unsubscribe", "score_override": "invalid" },
		"soft_bounce":  { "action": "suppress_after", "threshold_count": 3, "threshold_window_days": 30 },
		"unsubscribe":  { "action": "suppress" },
		"delivered":    { "action": "score_boost", "boost": 5 },
		"open":         { "action": "score_boost", "boost": 3 },
		"click":        { "action": "score_boost", "boost": 8 },
		"outcome_ttl_days": 7
	});
	sqlx::query(
		"INSERT INTO v1_outcome_policies (tenant_id, name, is_default, rules) VALUES ($1, 'short-ttl', true, $2)",
	)
	.bind(tenant_id)
	.bind(&custom_rules)
	.execute(db.pool())
	.await?;

	sqlx::query(
		"INSERT INTO verification_outcomes (tenant_id, canonical_email, outcome_type, occurred_at, source) VALUES ($1, $2, 'delivered', NOW() - INTERVAL '30 days', 'old')",
	)
	.bind(tenant_id)
	.bind("ttl@example.com")
	.execute(db.pool())
	.await?;

	let ctx = enrich_outcome_context(
		Some(db.pool()),
		Some(tenant_id),
		"ttl@example.com",
		Utc::now(),
	)
	.await;
	assert_eq!(
		ctx.delivered_count, 0,
		"30-day-old outcome should be filtered by 7-day TTL"
	);
	Ok(())
}

// ----------------------------------------------------------------------------
// Listing endpoint
// ----------------------------------------------------------------------------

#[tokio::test]
#[serial]
async fn test_list_outcomes_filter_by_email() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "outcomes-list-email", Some(1000), 0).await;
	let (api_key, _) = insert_api_key_with_scopes(db.pool(), tenant_id, &["outcomes.read"]).await;
	let config = worker_config().await;
	let policy = default_outcome_policy_rules();
	ingest_outcomes(
		db.pool(),
		tenant_id,
		&policy,
		&[
			ingest("a@example.com", OutcomeType::Delivered),
			ingest("b@example.com", OutcomeType::Delivered),
		],
	)
	.await;

	let response = request()
		.method("GET")
		.path("/v1/outcomes?email=a@example.com")
		.header("authorization", format!("Bearer {}", api_key))
		.reply(&create_routes(Arc::clone(&config)))
		.await;
	assert_eq!(response.status(), StatusCode::OK);
	let body: serde_json::Value = serde_json::from_slice(response.body())?;
	assert_eq!(body["total"], 1);
	assert_eq!(body["outcomes"][0]["email"], "a@example.com");
	Ok(())
}

#[tokio::test]
#[serial]
async fn test_list_outcomes_filter_by_type_and_source() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "outcomes-list-tsrc", Some(1000), 0).await;
	let (api_key, _) = insert_api_key_with_scopes(db.pool(), tenant_id, &["outcomes.read"]).await;
	let config = worker_config().await;
	let policy = default_outcome_policy_rules();

	let mut a = ingest("x@example.com", OutcomeType::HardBounce);
	a.source = Some("sendgrid".to_string());
	let mut b = ingest("y@example.com", OutcomeType::Delivered);
	b.source = Some("postmark".to_string());
	ingest_outcomes(db.pool(), tenant_id, &policy, &[a, b]).await;

	let r1 = request()
		.method("GET")
		.path("/v1/outcomes?type=hard_bounce")
		.header("authorization", format!("Bearer {}", api_key))
		.reply(&create_routes(Arc::clone(&config)))
		.await;
	let body: serde_json::Value = serde_json::from_slice(r1.body())?;
	assert_eq!(body["total"], 1);
	assert_eq!(body["outcomes"][0]["type"], "hard_bounce");

	let r2 = request()
		.method("GET")
		.path("/v1/outcomes?source=postmark")
		.header("authorization", format!("Bearer {}", api_key))
		.reply(&create_routes(Arc::clone(&config)))
		.await;
	let body: serde_json::Value = serde_json::from_slice(r2.body())?;
	assert_eq!(body["total"], 1);
	assert_eq!(body["outcomes"][0]["source"], "postmark");
	Ok(())
}

#[tokio::test]
#[serial]
async fn test_list_outcomes_pagination() -> Result<()> {
	let db = TestDb::start().await;
	let tenant_id = insert_tenant(db.pool(), "outcomes-list-page", Some(1000), 0).await;
	let (api_key, _) = insert_api_key_with_scopes(db.pool(), tenant_id, &["outcomes.read"]).await;
	let config = worker_config().await;
	let policy = default_outcome_policy_rules();

	let mut outcomes = Vec::new();
	for i in 0..15 {
		let mut o = ingest(&format!("p{}@example.com", i), OutcomeType::Delivered);
		o.occurred_at = Utc::now() - ChronoDuration::seconds(i as i64);
		outcomes.push(o);
	}
	ingest_outcomes(db.pool(), tenant_id, &policy, &outcomes).await;

	let r1 = request()
		.method("GET")
		.path("/v1/outcomes?limit=10&offset=0")
		.header("authorization", format!("Bearer {}", api_key))
		.reply(&create_routes(Arc::clone(&config)))
		.await;
	let body: serde_json::Value = serde_json::from_slice(r1.body())?;
	assert_eq!(body["total"], 15);
	assert_eq!(body["outcomes"].as_array().unwrap().len(), 10);

	let r2 = request()
		.method("GET")
		.path("/v1/outcomes?limit=10&offset=10")
		.header("authorization", format!("Bearer {}", api_key))
		.reply(&create_routes(Arc::clone(&config)))
		.await;
	let body: serde_json::Value = serde_json::from_slice(r2.body())?;
	assert_eq!(body["outcomes"].as_array().unwrap().len(), 5);
	Ok(())
}

// ----------------------------------------------------------------------------
// Schema sanity
// ----------------------------------------------------------------------------

#[tokio::test]
#[serial]
async fn test_outcomes_table_and_indexes_present() -> Result<()> {
	let db = TestDb::start().await;
	let table_exists: bool = sqlx::query_scalar(
		"SELECT EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = 'verification_outcomes')",
	)
	.fetch_one(db.pool())
	.await?;
	assert!(table_exists);

	for ix in &[
		"idx_verification_outcomes_lookup",
		"idx_verification_outcomes_recent",
		"idx_verification_outcomes_campaign",
		"idx_outcome_policies_one_default",
	] {
		let exists: bool =
			sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM pg_indexes WHERE indexname = $1)")
				.bind(ix)
				.fetch_one(db.pool())
				.await?;
		assert!(exists, "index {} should exist", ix);
	}
	Ok(())
}

// ----------------------------------------------------------------------------
// Multipart helper
// ----------------------------------------------------------------------------

fn build_multipart_csv(field_name: &str, filename: &str, content: &[u8]) -> Vec<u8> {
	let boundary = "----TestBoundary";
	let mut body = Vec::new();
	body.extend_from_slice(format!("--{}\r\n", boundary).as_bytes());
	body.extend_from_slice(
		format!(
			"Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
			field_name, filename
		)
		.as_bytes(),
	);
	body.extend_from_slice(b"Content-Type: text/csv\r\n\r\n");
	body.extend_from_slice(content);
	body.extend_from_slice(format!("\r\n--{}--\r\n", boundary).as_bytes());
	body
}
