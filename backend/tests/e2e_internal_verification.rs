mod test_helpers;

use reacher_backend::http::ReacherResponseError;
use reacher_backend::http::{create_routes, resolve_tenant};
use serde_json::{json, Value};
use test_helpers::{
	build_test_config, insert_api_key, insert_tenant, ConfigProfile, TestDb, ADMIN_SECRET,
};

#[tokio::test]
async fn internal_verification_keeps_tenant_limits_without_granting_public_api_access() {
	let db = TestDb::start().await;
	let tenant = insert_tenant(db.pool(), "internal-free", Some(2), 0).await;
	sqlx::query("UPDATE tenants SET plan_tier='free' WHERE id=$1")
		.bind(tenant)
		.execute(db.pool())
		.await
		.unwrap();
	let config = build_test_config(ConfigProfile::AdminSecret, Some(db.db_url()), None).await;
	let routes = create_routes(config.clone());
	let response = warp::test::request()
		.method("POST")
		.path(&format!("/v1/admin/tenants/{tenant}/api-keys"))
		.header("x-reacher-secret", ADMIN_SECRET)
		.json(&json!({"scopes":["verify","internal:verification"]}))
		.reply(&routes)
		.await;
	assert_eq!(response.status(), 201);
	let key: Value = serde_json::from_slice(response.body()).unwrap();
	let auth = format!("Bearer {}", key["key"].as_str().unwrap());
	let context = resolve_tenant(config);
	for path in [
		"/v1/check_email".to_owned(),
		format!("/v1/operations/{}", uuid::Uuid::new_v4()),
	] {
		let method = if path == "/v1/check_email" {
			"POST"
		} else {
			"GET"
		};
		let ctx = warp::test::request()
			.method(method)
			.path(&path)
			.header("authorization", &auth)
			.filter(&context)
			.await
			.unwrap();
		assert_eq!(ctx.tenant_id, Some(tenant));
		assert_eq!(ctx.api_key_id.unwrap().to_string(), key["id"]);
		assert!(!ctx.is_legacy);
		assert_eq!(ctx.monthly_email_limit, Some(2));
		assert_eq!(
			ctx.plan_tier,
			reacher_backend::tenant::models::PlanTier::Free
		);
	}
	for (method, path) in [
		("GET", "/v1/me"),
		("POST", "/v1/research"),
		("POST", "/v1/bulk"),
		("GET", "/v1/check_email"),
		(
			"POST",
			"/v1/operations/00000000-0000-0000-0000-000000000001",
		),
	] {
		assert_eq!(
			warp::test::request()
				.method(method)
				.path(path)
				.header("authorization", &auth)
				.filter(&context)
				.await
				.unwrap_err()
				.find::<ReacherResponseError>()
				.unwrap()
				.code,
			403
		);
	}
	let (public_key, _) = insert_api_key(db.pool(), tenant).await;
	assert_eq!(
		warp::test::request()
			.method("POST")
			.path("/v1/check_email")
			.header("authorization", format!("Bearer {public_key}"))
			.filter(&context)
			.await
			.unwrap_err()
			.find::<ReacherResponseError>()
			.unwrap()
			.code,
		403
	);
	let missing = warp::test::request()
		.path(&format!("/v1/operations/{}", uuid::Uuid::new_v4()))
		.header("authorization", &auth)
		.reply(&routes)
		.await;
	assert_eq!(missing.status(), 404);
	sqlx::query("UPDATE api_keys SET status='revoked' WHERE id=$1")
		.bind(key["id"].as_str().unwrap().parse::<uuid::Uuid>().unwrap())
		.execute(db.pool())
		.await
		.unwrap();
	assert_eq!(
		warp::test::request()
			.method("POST")
			.path("/v1/check_email")
			.header("authorization", &auth)
			.filter(&context)
			.await
			.unwrap_err()
			.find::<ReacherResponseError>()
			.unwrap()
			.code,
		401
	);

	let owner = insert_tenant(db.pool(), "internal-cannot-self-grant", Some(2), 0).await;
	let (owner_key, owner_key_id) = insert_api_key(db.pool(), owner).await;
	for (method, path) in [
		("POST", "/v1/me/api-keys".to_owned()),
		("PATCH", format!("/v1/me/api-keys/{owner_key_id}")),
	] {
		let response = warp::test::request()
			.method(method)
			.path(&path)
			.header("authorization", format!("Bearer {owner_key}"))
			.json(&json!({"scopes":["verify","internal:verification"]}))
			.reply(&routes)
			.await;
		assert_eq!(response.status(), 400, "{method} {path}");
	}
}
