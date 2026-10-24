#[path = "support/api_harness_support.rs"]
mod api_harness_support;
mod test_helpers;

use api_harness_support::{
	build_runtime, execute_cases, seed_upgrade_fixtures, upgrade_safe_cases,
};
use serial_test::serial;
use test_helpers::{ensure_test_amqp_url, restore_migration_fixture_to_head, MigrationFixture};

#[tokio::test]
#[serial]
async fn v410_fixture_migrates_to_head_and_passes_upgrade_safe_api_subset() {
	let db = restore_migration_fixture_to_head(MigrationFixture::V410).await;
	let amqp = ensure_test_amqp_url().await;
	let fixtures = seed_upgrade_fixtures(db.pool()).await;
	let runtime = build_runtime(db.db_url(), &amqp).await;
	let cases = upgrade_safe_cases();

	assert!(
		!cases.is_empty(),
		"upgrade-safe harness subset must not be empty"
	);
	execute_cases(&cases, &fixtures, &runtime).await;
}

/// Staging and production ran the migrations of master at 0f780f9. Four
/// older migrations reached master later, through develop. A crashed staging
/// start also applied 20260401000001 before it failed. Both databases must
/// migrate to head.
#[tokio::test]
#[serial]
async fn databases_without_the_late_spring_migrations_migrate_to_head() {
	use test_helpers::migrate_scratch_db_to_head;

	for missing in [
		&[
			"20260401000001",
			"20260402000001",
			"20260403000001",
			"20260510000001",
		][..],
		&["20260402000001", "20260403000001", "20260510000001"][..],
	] {
		let db = migrate_scratch_db_to_head(
			|prefix| {
				prefix < "20261019000001"
					&& !missing.iter().any(|version| prefix.starts_with(version))
			},
			None,
		)
		.await;
		let tables: i64 = sqlx::query_scalar(
			"SELECT COUNT(*) FROM information_schema.tables WHERE table_name IN \
			 ('v1_remediation_plans', 'v1_alerts', 'verification_delayed_rechecks', 'verification_outcomes')",
		)
		.fetch_one(db.pool())
		.await
		.expect("count tables");
		assert_eq!(tables, 4, "missing tables after {missing:?}");
	}
}

/// 20261019000001 re-keys Gmail suppressions to the canonical address, which
/// merges the spellings of one mailbox. The entry that stays active must
/// cover the mailbox at least as long as the merged ones did together.
#[tokio::test]
#[serial]
async fn gmail_suppression_aliases_keep_their_coverage_when_merged() {
	use reacher_backend::suppression::has_active_suppression;
	use std::path::Path;
	use test_helpers::migrate_scratch_db_to_head;

	let seed = Path::new(env!("CARGO_MANIFEST_DIR"))
		.join("tests/fixtures/migration_upgrade/gmail_suppression_aliases_seed.sql");
	let db = migrate_scratch_db_to_head(|prefix| prefix < "20261019000001", Some(seed)).await;
	let pool = db.pool();
	let tenant_id: uuid::Uuid = "6a1f0c3e-6c55-4f4e-9f0b-2c1d7a9e5b01".parse().unwrap();

	// The older permanent entry outlasts the newer expired one.
	for spelling in [
		"user@gmail.com",
		"u.ser@gmail.com",
		"u.s.e.r+promo@googlemail.com",
	] {
		assert!(
			has_active_suppression(pool, tenant_id, spelling)
				.await
				.unwrap(),
			"{} must stay suppressed",
			spelling
		);
	}
	assert!(
		has_active_suppression(pool, tenant_id, "re.moved@gmail.com")
			.await
			.unwrap()
	);

	let survivors: Vec<(String, String, Option<chrono::DateTime<chrono::Utc>>)> = sqlx::query_as(
		"SELECT canonical_email, reason::TEXT, expires_at FROM v1_suppression_entries \
			 WHERE tenant_id = $1 AND status = 'active' ORDER BY canonical_email",
	)
	.bind(tenant_id)
	.fetch_all(pool)
	.await
	.unwrap();
	let survivor = |key: &str| {
		let matching: Vec<_> = survivors.iter().filter(|(k, _, _)| k == key).collect();
		assert_eq!(
			matching.len(),
			1,
			"one active entry for {key}: {survivors:?}"
		);
		matching[0].clone()
	};
	// The entry that covers the mailbox keeps its own reason.
	let (_, reason, expires_at) = survivor("user@gmail.com");
	assert_eq!((reason.as_str(), expires_at), ("bounce", None));
	// Of two temporary entries, the one that runs longer stays.
	let (_, _, expires_at) = survivor("tempone@gmail.com");
	let expires_at = expires_at.expect("still temporary");
	assert!(expires_at > chrono::Utc::now() + chrono::Duration::days(30));
	let (_, reason, _) = survivor("removed@gmail.com");
	assert_eq!(reason, "complaint");
	let inactive: i64 = sqlx::query_scalar(
		"SELECT COUNT(*) FROM v1_suppression_entries WHERE tenant_id = $1 AND status = 'inactive'",
	)
	.bind(tenant_id)
	.fetch_one(pool)
	.await
	.unwrap();
	assert_eq!(inactive, 1, "a removed entry is not revived");
}

#[tokio::test]
#[serial]
async fn casefold_collisions_keep_protection_history_and_historical_charges() {
	let seed = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
		.join("tests/fixtures/migration_upgrade/casefold_seed.sql");
	let db =
		test_helpers::migrate_scratch_db_to_head(|prefix| prefix < "20261021000001", Some(seed))
			.await;
	let pool = db.pool();
	let tenant: uuid::Uuid = "33333333-3333-4333-8333-333333333333".parse().unwrap();
	let rows: (i64,i64,i64) = sqlx::query_as("SELECT count(*),count(*) FILTER (WHERE status='active'),count(*) FILTER (WHERE status='active' AND expires_at IS NULL) FROM v1_suppression_entries WHERE tenant_id=$1").bind(tenant).fetch_one(pool).await.unwrap();
	assert_eq!(rows, (2, 1, 1));
	assert!(reacher_backend::suppression::has_active_suppression(
		pool,
		tenant,
		"SALES+TAG@EXAMPLE.COM"
	)
	.await
	.unwrap());
	let events: i64 = sqlx::query_scalar(
		"SELECT count(*) FROM v1_suppression_events WHERE tenant_id=$1 AND event_type='created'",
	)
	.bind(tenant)
	.fetch_one(pool)
	.await
	.unwrap();
	assert_eq!(events, 2);
	let outcomes: (i64,i64) = sqlx::query_as("SELECT count(*),count(*) FILTER (WHERE identity_duplicate_of IS NULL) FROM verification_outcomes WHERE tenant_id=$1").bind(tenant).fetch_one(pool).await.unwrap();
	assert_eq!(outcomes, (2, 1));
	let tasks: (i64,i64) = sqlx::query_as("SELECT count(*),count(*) FILTER (WHERE NOT is_duplicate) FROM v1_task_result WHERE tenant_id=$1").bind(tenant).fetch_one(pool).await.unwrap();
	assert_eq!(tasks, (2, 1));
	let charges: i32 = sqlx::query_scalar("SELECT used_this_period FROM tenants WHERE id=$1")
		.bind(tenant)
		.fetch_one(pool)
		.await
		.unwrap();
	assert_eq!(charges, 7);
	// A compatibility binary can still write a mixed-case key after migration.
	sqlx::query("INSERT INTO v1_suppression_entries (tenant_id,email,canonical_email,reason) VALUES ($1,'Other@Example.com','Other@example.com','manual')").bind(tenant).execute(pool).await.unwrap();
	let stored: String = sqlx::query_scalar("SELECT canonical_email FROM v1_suppression_entries WHERE tenant_id=$1 AND email='Other@Example.com'").bind(tenant).fetch_one(pool).await.unwrap();
	assert_eq!(stored, "other@example.com");
}

#[tokio::test]
#[serial]
async fn casefold_migration_refuses_inflight_identity_collisions() {
	let db = test_helpers::TestDb::start().await;
	let tenant = test_helpers::insert_tenant(db.pool(), "inflight-casefold", Some(100), 2).await;
	let job: i32=sqlx::query_scalar("INSERT INTO v1_bulk_job (tenant_id,total_records,status) VALUES ($1,2,'running') RETURNING id").bind(tenant).fetch_one(db.pool()).await.unwrap();
	sqlx::query("INSERT INTO v1_task_result (tenant_id,job_id,payload,canonical_email,task_state) VALUES ($1,$2,'{}','case@example.com','queued'),($1,$2,'{}','case@example.com','running')").bind(tenant).bind(job).execute(db.pool()).await.unwrap();
	let migration = include_str!("../migrations/20261021000001_mailbox_identity_casefold.up.sql");
	let guard = migration
		.split("CREATE OR REPLACE FUNCTION")
		.next()
		.unwrap();
	let mut tx = db.pool().begin().await.unwrap();
	let error = sqlx::raw_sql(guard).execute(&mut *tx).await.unwrap_err();
	assert!(error
		.to_string()
		.contains("requires colliding jobs to finish"));
	tx.rollback().await.unwrap();
	let retained: i64 = sqlx::query_scalar(
		"SELECT count(*) FROM v1_task_result WHERE job_id=$1 AND NOT is_duplicate",
	)
	.bind(job)
	.fetch_one(db.pool())
	.await
	.unwrap();
	assert_eq!(retained, 2);
}
