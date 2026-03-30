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
