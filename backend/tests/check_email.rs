// Reacher - Email Verification
// Copyright (C) 2018-2023 Reacher

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

#[cfg(test)]
mod tests {
	use std::sync::Arc;

	use reacher_backend::config::BackendConfig;
	use reacher_backend::http::{create_routes, CheckEmailRequest, REACHER_SECRET_HEADER};
	use serde_json::Value;
	use warp::http::StatusCode;
	use warp::test::request;

	fn create_backend_config(header_secret: &str) -> Arc<BackendConfig> {
		let mut config = BackendConfig::empty();
		config.header_secret = Some(header_secret.to_string());
		Arc::new(config)
	}

	fn create_open_config() -> Arc<BackendConfig> {
		let mut config = BackendConfig::empty();
		config.header_secret = None;
		Arc::new(config)
	}

	fn parse_json(body: &[u8]) -> Value {
		serde_json::from_slice(body).expect("response body should be valid JSON")
	}

	fn assert_invalid_scored_response(
		body: &[u8],
		input: &str,
		is_valid_syntax: bool,
		address: Option<&str>,
		domain: &str,
		username: &str,
		sub_reason: &str,
	) {
		let body = parse_json(body);
		assert_eq!(body["input"], input);
		assert_eq!(body["is_reachable"], "invalid");
		assert_eq!(body["syntax"]["is_valid_syntax"], is_valid_syntax);
		assert_eq!(body["syntax"]["domain"], domain);
		assert_eq!(body["syntax"]["username"], username);
		match address {
			Some(address) => assert_eq!(body["syntax"]["address"], address),
			None => assert!(body["syntax"]["address"].is_null()),
		}
		assert!(body["misc"].is_object());
		assert!(body["mx"].is_object() || body["mx"].is_null());
		assert!(body["smtp"].is_object() || body["smtp"].is_null());
		assert!(body["provider"].is_null());
		assert_eq!(body["provider_rules_applied"], false);
		assert!(body["provider_rejection_reason"].is_null());
		assert!(body["provider_confidence"].is_null());
		assert_eq!(body["score"]["score"], 0);
		assert_eq!(body["score"]["category"], "invalid");
		assert_eq!(body["score"]["sub_reason"], sub_reason);
		assert_eq!(body["score"]["safe_to_send"], false);
		assert!(body["score"]["reason_codes"].is_array());
		assert!(body["score"]["reason_codes"]
			.as_array()
			.unwrap()
			.iter()
			.any(|v| v.as_str() == Some(sub_reason)));
		assert!(body["score"]["signals"].is_object());
	}

	// --- V0 check_email with legacy auth ---

	#[tokio::test]
	async fn test_input_foo_bar() {
		let resp = request()
			.path("/v0/check_email")
			.method("POST")
			.header(REACHER_SECRET_HEADER, "foobar")
			.json(&serde_json::from_str::<CheckEmailRequest>(r#"{"to_email": "foo@bar"}"#).unwrap())
			.reply(&create_routes(create_backend_config("foobar")))
			.await;

		assert_eq!(resp.status(), StatusCode::OK, "{:?}", resp.body());
		assert_invalid_scored_response(
			resp.body(),
			"foo@bar",
			false,
			None,
			"",
			"",
			"invalid_syntax",
		);
	}

	#[tokio::test]
	async fn test_input_foo_bar_baz() {
		let resp = request()
			.path("/v0/check_email")
			.method("POST")
			.header(REACHER_SECRET_HEADER, "foobar")
			.json(
				&serde_json::from_str::<CheckEmailRequest>(r#"{"to_email": "foo@bar.baz"}"#)
					.unwrap(),
			)
			.reply(&create_routes(create_backend_config("foobar")))
			.await;

		assert_eq!(resp.status(), StatusCode::OK, "{:?}", resp.body());
		assert_invalid_scored_response(
			resp.body(),
			"foo@bar.baz",
			true,
			Some("foo@bar.baz"),
			"bar.baz",
			"foo",
			"invalid_recipient",
		);
	}

	#[tokio::test]
	async fn test_reacher_secret_missing_header() {
		let resp = request()
			.path("/v0/check_email")
			.method("POST")
			.json(
				&serde_json::from_str::<CheckEmailRequest>(r#"{"to_email": "foo@bar.baz"}"#)
					.unwrap(),
			)
			.reply(&create_routes(create_backend_config("foobar")))
			.await;

		// Now returns 401 with JSON body via resolve_tenant()
		assert_eq!(resp.status(), StatusCode::UNAUTHORIZED, "{:?}", resp.body());
	}

	#[tokio::test]
	async fn test_reacher_secret_wrong_secret() {
		let resp = request()
			.path("/v0/check_email")
			.method("POST")
			.header(REACHER_SECRET_HEADER, "barbaz")
			.json(
				&serde_json::from_str::<CheckEmailRequest>(r#"{"to_email": "foo@bar.baz"}"#)
					.unwrap(),
			)
			.reply(&create_routes(create_backend_config("foobar")))
			.await;

		// Now returns 401 via resolve_tenant()
		assert_eq!(resp.status(), StatusCode::UNAUTHORIZED, "{:?}", resp.body());
	}

	#[tokio::test]
	async fn test_reacher_secret_correct_secret() {
		let resp = request()
			.path("/v0/check_email")
			.method("POST")
			.header(REACHER_SECRET_HEADER, "foobar")
			.json(&serde_json::from_str::<CheckEmailRequest>(r#"{"to_email": "foo@bar"}"#).unwrap())
			.reply(&create_routes(create_backend_config("foobar")))
			.await;

		assert_eq!(resp.status(), StatusCode::OK, "{:?}", resp.body());
		assert_invalid_scored_response(
			resp.body(),
			"foo@bar",
			false,
			None,
			"",
			"",
			"invalid_syntax",
		);
	}

	#[tokio::test]
	async fn test_reacher_to_mail_empty() {
		let resp = request()
			.path("/v0/check_email")
			.method("POST")
			.header(REACHER_SECRET_HEADER, "foobar")
			.json(&serde_json::from_str::<CheckEmailRequest>(r#"{"to_email": ""}"#).unwrap())
			.reply(&create_routes(create_backend_config("foobar")))
			.await;

		assert_eq!(resp.status(), StatusCode::BAD_REQUEST, "{:?}", resp.body());
		assert_eq!(resp.body(), r#"{"error":"to_email field is required."}"#);
	}

	// --- V0 check_email returns deprecation headers ---

	#[tokio::test]
	async fn test_v0_deprecation_headers() {
		let resp = request()
			.path("/v0/check_email")
			.method("POST")
			.header(REACHER_SECRET_HEADER, "foobar")
			.json(&serde_json::from_str::<CheckEmailRequest>(r#"{"to_email": "foo@bar"}"#).unwrap())
			.reply(&create_routes(create_backend_config("foobar")))
			.await;

		assert_eq!(resp.status(), StatusCode::OK);
		assert_eq!(resp.headers().get("Deprecation").unwrap(), "true");
		assert_eq!(resp.headers().get("Sunset").unwrap(), "2026-09-16");
		assert!(resp
			.headers()
			.get("Link")
			.unwrap()
			.to_str()
			.unwrap()
			.contains("/v1/check_email"));
	}

	// --- Open mode (no auth configured) ---

	#[tokio::test]
	async fn test_open_mode_no_auth_required() {
		let resp = request()
			.path("/v0/check_email")
			.method("POST")
			.json(&serde_json::from_str::<CheckEmailRequest>(r#"{"to_email": "foo@bar"}"#).unwrap())
			.reply(&create_routes(create_open_config()))
			.await;

		assert_eq!(resp.status(), StatusCode::OK, "{:?}", resp.body());
		assert_invalid_scored_response(
			resp.body(),
			"foo@bar",
			false,
			None,
			"",
			"",
			"invalid_syntax",
		);
	}

	// --- V1 check_email with legacy auth ---

	#[tokio::test]
	async fn test_v1_check_email_with_legacy_auth() {
		let resp = request()
			.path("/v1/check_email")
			.method("POST")
			.header(REACHER_SECRET_HEADER, "foobar")
			.json(&serde_json::from_str::<CheckEmailRequest>(r#"{"to_email": "foo@bar"}"#).unwrap())
			.reply(&create_routes(create_backend_config("foobar")))
			.await;

		assert_eq!(resp.status(), StatusCode::OK, "{:?}", resp.body());
		assert_invalid_scored_response(
			resp.body(),
			"foo@bar",
			false,
			None,
			"",
			"",
			"invalid_syntax",
		);
	}

	#[tokio::test]
	async fn test_v1_provider_specific_rejection() {
		let resp = request()
			.path("/v1/check_email")
			.method("POST")
			.header(REACHER_SECRET_HEADER, "foobar")
			.json(
				&serde_json::from_str::<CheckEmailRequest>(r#"{"to_email": "abc@gmail.com"}"#)
					.unwrap(),
			)
			.reply(&create_routes(create_backend_config("foobar")))
			.await;

		assert_eq!(resp.status(), StatusCode::OK, "{:?}", resp.body());
		let body = parse_json(resp.body());
		assert_eq!(body["is_reachable"], "invalid");
		assert_eq!(body["provider"], "gmail");
		assert_eq!(body["provider_rules_applied"], true);
		assert_eq!(
			body["provider_rejection_reason"],
			"provider_local_part_too_short"
		);
		assert_eq!(body["provider_confidence"], "high");
		assert_eq!(body["score"]["sub_reason"], "provider_rejected");
		assert!(body["score"]["reason_codes"]
			.as_array()
			.unwrap()
			.iter()
			.any(|v| v.as_str() == Some("provider_local_part_too_short")));
	}

	#[tokio::test]
	async fn test_v1_provider_specific_rejection_non_strict() {
		let resp = request()
			.path("/v1/check_email")
			.method("POST")
			.header(REACHER_SECRET_HEADER, "foobar")
			.json(
				&serde_json::from_str::<CheckEmailRequest>(
					r#"{"to_email": "abc@gmail.com", "strict_provider_rules": false}"#,
				)
				.unwrap(),
			)
			.reply(&create_routes(create_backend_config("foobar")))
			.await;

		assert_eq!(resp.status(), StatusCode::OK, "{:?}", resp.body());
		let body = parse_json(resp.body());
		assert_eq!(body["provider"], "gmail");
		assert_eq!(body["provider_rules_applied"], false);
		assert!(body["provider_rejection_reason"].is_null());
		assert_eq!(body["provider_confidence"], "high");
		assert_ne!(body["score"]["sub_reason"], "provider_rejected");
		assert!(!body["score"]["reason_codes"]
			.as_array()
			.unwrap()
			.iter()
			.any(|v| v.as_str() == Some("provider_local_part_too_short")));
	}

	#[tokio::test]
	async fn test_v1_check_email_missing_auth() {
		let resp = request()
			.path("/v1/check_email")
			.method("POST")
			.json(&serde_json::from_str::<CheckEmailRequest>(r#"{"to_email": "foo@bar"}"#).unwrap())
			.reply(&create_routes(create_backend_config("foobar")))
			.await;

		assert_eq!(resp.status(), StatusCode::UNAUTHORIZED, "{:?}", resp.body());
	}

	#[tokio::test]
	async fn test_v1_check_email_invalid_bearer() {
		let resp = request()
			.path("/v1/check_email")
			.method("POST")
			.header("Authorization", "Bearer rch_live_notavalidkey1234567890ab")
			.json(&serde_json::from_str::<CheckEmailRequest>(r#"{"to_email": "foo@bar"}"#).unwrap())
			.reply(&create_routes(create_backend_config("foobar")))
			.await;

		// API key lookup requires Postgres — returns 503 without it
		assert!(
			resp.status() == StatusCode::SERVICE_UNAVAILABLE
				|| resp.status() == StatusCode::UNAUTHORIZED,
			"Expected 503 or 401, got {}",
			resp.status()
		);
	}

	// --- Health endpoints ---

	#[tokio::test]
	async fn test_healthz() {
		let resp = request()
			.path("/healthz")
			.method("GET")
			.reply(&create_routes(create_backend_config("foobar")))
			.await;

		assert_eq!(resp.status(), StatusCode::OK);
		let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
		assert_eq!(body["status"], "ok");
	}

	#[tokio::test]
	async fn test_healthz_no_auth_required() {
		// Health endpoints should work without any auth
		let resp = request()
			.path("/healthz")
			.method("GET")
			.reply(&create_routes(create_backend_config("foobar")))
			.await;

		assert_eq!(resp.status(), StatusCode::OK);
	}

	#[tokio::test]
	async fn test_readyz_no_postgres() {
		// Without Postgres, readyz should still return (with not_configured)
		let resp = request()
			.path("/readyz")
			.method("GET")
			.reply(&create_routes(create_open_config()))
			.await;

		let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
		assert!(body["checks"]["postgres"]["status"].is_string());
		assert!(body["checks"]["rabbitmq"]["status"].is_string());
	}

	// --- V1 empty email ---

	#[tokio::test]
	async fn test_v1_to_email_empty() {
		let resp = request()
			.path("/v1/check_email")
			.method("POST")
			.header(REACHER_SECRET_HEADER, "foobar")
			.json(&serde_json::from_str::<CheckEmailRequest>(r#"{"to_email": ""}"#).unwrap())
			.reply(&create_routes(create_backend_config("foobar")))
			.await;

		assert_eq!(resp.status(), StatusCode::BAD_REQUEST, "{:?}", resp.body());
	}

	// --- Version endpoint still works ---

	#[tokio::test]
	async fn test_version_endpoint() {
		let resp = request()
			.path("/version")
			.method("GET")
			.reply(&create_routes(create_open_config()))
			.await;

		assert_eq!(resp.status(), StatusCode::OK);
	}
}
