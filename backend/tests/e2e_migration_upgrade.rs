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
			|prefix| !missing.iter().any(|version| prefix.starts_with(version)),
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
