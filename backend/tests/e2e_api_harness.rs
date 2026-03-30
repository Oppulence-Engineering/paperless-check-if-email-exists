mod api_harness_support;
mod test_helpers;

use api_harness_support::{
	auth_failure_cases, build_runtime, canonical_cases, conflict_cases, execute_cases,
	harness_keys, inventory_keys, missing_resource_cases, openapi_keys, seed_fixtures,
	validation_failure_cases,
};
use reacher_backend::http::routes::all_route_specs;
use serial_test::serial;
use test_helpers::{ensure_test_amqp_url, TestDb};

#[tokio::test]
#[serial]
async fn route_inventory_matches_runtime_openapi_surface() {
	let inventory = inventory_keys();
	let openapi = openapi_keys();
	assert_eq!(
		inventory, openapi,
		"route inventory and runtime OpenAPI method/path sets diverged"
	);
}

#[test]
fn canonical_harness_manifest_covers_every_inventory_route_once() {
	let inventory = inventory_keys();
	let cases = canonical_cases();
	let harness = harness_keys(&cases);
	assert_eq!(
		cases.len(),
		harness.len(),
		"canonical harness contains duplicate method/path entries"
	);
	assert_eq!(
		inventory, harness,
		"canonical harness and route inventory method/path sets diverged"
	);
}

#[tokio::test]
#[serial]
async fn canonical_route_harness_invokes_every_live_route() {
	let db = TestDb::start().await;
	let amqp = ensure_test_amqp_url().await;
	let fixtures = seed_fixtures(db.pool()).await;
	let runtime = build_runtime(db.db_url(), &amqp).await;
	let cases = canonical_cases();

	assert_eq!(
		cases.len(),
		all_route_specs().len(),
		"canonical case count must equal live route count"
	);
	execute_cases(&cases, &fixtures, &runtime).await;
}

#[tokio::test]
#[serial]
async fn behavior_harness_covers_auth_and_scope_failures() {
	let db = TestDb::start().await;
	let amqp = ensure_test_amqp_url().await;
	let fixtures = seed_fixtures(db.pool()).await;
	let runtime = build_runtime(db.db_url(), &amqp).await;
	let cases = auth_failure_cases();

	execute_cases(&cases, &fixtures, &runtime).await;
}

#[tokio::test]
#[serial]
async fn behavior_harness_covers_validation_failures() {
	let db = TestDb::start().await;
	let amqp = ensure_test_amqp_url().await;
	let fixtures = seed_fixtures(db.pool()).await;
	let runtime = build_runtime(db.db_url(), &amqp).await;
	let cases = validation_failure_cases();

	execute_cases(&cases, &fixtures, &runtime).await;
}

#[tokio::test]
#[serial]
async fn behavior_harness_covers_missing_resource_failures() {
	let db = TestDb::start().await;
	let amqp = ensure_test_amqp_url().await;
	let fixtures = seed_fixtures(db.pool()).await;
	let runtime = build_runtime(db.db_url(), &amqp).await;
	let cases = missing_resource_cases();

	execute_cases(&cases, &fixtures, &runtime).await;
}

#[tokio::test]
#[serial]
async fn behavior_harness_covers_conflict_failures() {
	let db = TestDb::start().await;
	let amqp = ensure_test_amqp_url().await;
	let fixtures = seed_fixtures(db.pool()).await;
	let runtime = build_runtime(db.db_url(), &amqp).await;
	let cases = conflict_cases();

	execute_cases(&cases, &fixtures, &runtime).await;
}
