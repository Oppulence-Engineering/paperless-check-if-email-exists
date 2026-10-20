//! Verification routing and durable paid-provider attempts.
use crate::{config::BackendConfig, worker::do_work::CheckEmailTask};
use check_if_email_exists::{
	misc::check_misc, mx::check_mx, smtp::SmtpError, syntax::check_syntax, CheckEmailOutput,
	ExternalVerification, Reachable,
};
use millionverifier_sdk::{
	apis::{configuration::Configuration, single_api_api::single_verification},
	models::SingleApiResponse,
};
use serde::{Deserialize, Serialize};
use std::{fmt, sync::OnceLock, time::Duration};
use tokio::sync::Semaphore;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationBackend {
	#[default]
	Reacher,
	Millionverifier,
}

#[derive(Default, Serialize, Deserialize)]
pub struct MillionVerifierConfig {
	#[serde(default, skip_serializing)]
	pub api_key: Option<String>,
	#[serde(skip)]
	client: OnceLock<Configuration>,
}

impl fmt::Debug for MillionVerifierConfig {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("MillionVerifierConfig")
			.field("api_key", &self.api_key.as_ref().map(|_| "<redacted>"))
			.finish()
	}
}

impl MillionVerifierConfig {
	pub fn validate(&self, backend: VerificationBackend) -> anyhow::Result<()> {
		if backend == VerificationBackend::Millionverifier {
			let key = self.api_key.as_deref().unwrap_or("").trim();
			anyhow::ensure!(
				!key.is_empty() && !key.starts_with("API_KEY_FOR_"),
				"Configure a live RCH__MILLIONVERIFIER__API_KEY before enabling MillionVerifier."
			);
		}
		Ok(())
	}

	fn client(&self) -> &Configuration {
		check_if_email_exists::initialize_crypto_provider();
		self.client.get_or_init(|| Configuration {
			client: reqwest::Client::builder()
				.connect_timeout(Duration::from_secs(5))
				.timeout(Duration::from_secs(25))
				.redirect(reqwest::redirect::Policy::none())
				.retry(reqwest::retry::never())
				.build()
				.expect("static HTTP client configuration"),
			..Configuration::default()
		})
	}
}

// shortcut: four concurrent requests per process; coordinate globally before increasing worker replicas substantially.
static PROVIDER_CONCURRENCY: Semaphore = Semaphore::const_new(4);

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct VerificationError(pub &'static str);

pub async fn verify_task(
	config: &BackendConfig,
	task: &CheckEmailTask,
) -> Result<CheckEmailOutput, VerificationError> {
	if task.verification_backend != config.verification_backend {
		return Err(VerificationError(
			"Verification backend changed since admission; explicitly retry this task.",
		));
	}
	if task.verification_backend == VerificationBackend::Reacher {
		return Ok(check_if_email_exists::check_email(&task.input).await);
	}
	config
		.millionverifier
		.validate(task.verification_backend)
		.map_err(|_| VerificationError("MillionVerifier is not configured with a live API key."))?;
	let syntax = check_syntax(&task.input.to_email);
	let detected = check_if_email_exists::provider::detect_provider(&syntax.domain, None);
	let provider_rejected = task.input.strict_provider_rules.unwrap_or(true)
		&& detected.as_ref().is_some_and(|provider| {
			check_if_email_exists::provider::should_apply_provider_rule(provider)
				&& check_if_email_exists::provider::validate_provider_email(
					provider,
					&syntax.username,
				)
				.is_err()
		});
	if !syntax.is_valid_syntax || provider_rejected {
		// Core returns before DNS/SMTP for invalid syntax.
		return Ok(check_if_email_exists::check_email(&task.input).await);
	}
	let pool = config.get_pg_pool().ok_or(VerificationError(
		"MillionVerifier requires durable task storage.",
	))?;
	let metadata = task.metadata.as_ref().ok_or(VerificationError(
		"MillionVerifier requires a durable task identity.",
	))?;
	let task_id = metadata
		.task_db_id
		.ok_or(VerificationError("Missing task identity."))?;
	let dispatch = metadata
		.dispatch_id
		.ok_or(VerificationError("Missing dispatch identity."))?;
	let lease = metadata
		.lease_id
		.ok_or(VerificationError("Missing task lease."))?;
	let _permit = PROVIDER_CONCURRENCY
		.acquire()
		.await
		.expect("static semaphore");
	let started = chrono::Utc::now();
	let inserted = sqlx::query(
		"INSERT INTO verification_provider_attempts (dispatch_id, task_id)
		 SELECT $1, id FROM v1_task_result
		 WHERE id = $2 AND dispatch_id = $1 AND lease_id = $3 AND lease_expires_at > NOW()
		 ON CONFLICT (dispatch_id) DO NOTHING",
	)
	.bind(dispatch)
	.bind(task_id)
	.bind(lease)
	.execute(&pool)
	.await
	.map_err(|_| {
		VerificationError("Could not record the provider attempt; no request was sent.")
	})?;

	let response: SingleApiResponse = if inserted.rows_affected() == 0 {
		let stored: Option<Option<serde_json::Value>> = sqlx::query_scalar(
			"SELECT a.response FROM verification_provider_attempts a
			 JOIN v1_task_result t ON t.id = a.task_id
			 WHERE a.dispatch_id = $1 AND t.id = $2 AND t.lease_id = $3 AND t.lease_expires_at > NOW()",
		)
		.bind(dispatch)
		.bind(task_id)
		.bind(lease)
		.fetch_optional(&pool)
		.await
		.map_err(|_| VerificationError("Could not read the recorded provider attempt."))?;
		let stored = stored.flatten().ok_or(VerificationError(
			"Provider outcome is unknown after an interrupted attempt; no automatic paid retry was made.",
		))?;
		serde_json::from_value(stored)
			.map_err(|_| VerificationError("The recorded provider response could not be read."))?
	} else {
		let response = single_verification(
			config.millionverifier.client(),
			config
				.millionverifier
				.api_key
				.as_deref()
				.expect("validated key"),
			&task.input.to_email,
			Some(20),
		)
		.await
		.map_err(|_| {
			VerificationError(
			"MillionVerifier request failed; the outcome may be unknown. No automatic paid retry was made.",
		)
		})?;
		// Persist before scoring, webhooks or billing, so completion can resume without another call.
		let stored = sqlx::query(
			"UPDATE verification_provider_attempts a SET response = $2
			 WHERE a.dispatch_id = $1 AND EXISTS
			 (SELECT 1 FROM v1_task_result t WHERE t.id = a.task_id
			  AND t.lease_id = $3 AND t.lease_expires_at > NOW())",
		)
		.bind(dispatch)
		.bind(serde_json::to_value(&response).expect("generated response serialization"))
		.bind(lease)
		.execute(&pool)
		.await
		.map_err(|_| {
			VerificationError(
				"Provider response could not be saved; no automatic paid retry was made.",
			)
		})?;
		if stored.rows_affected() != 1 {
			return Err(VerificationError(
				"Task ownership changed while recording the provider response.",
			));
		}
		response
	};
	let mut output = map_response(&task.input.to_email, response).await?;
	if let Some(provider) = detected {
		output.provider_rules_applied = task.input.strict_provider_rules.unwrap_or(true)
			&& check_if_email_exists::provider::should_apply_provider_rule(&provider);
		output.provider = Some(provider.provider);
		output.provider_confidence = Some(provider.confidence);
	}
	// DNS evidence is local; no SMTP connection or synthetic SMTP success is produced.
	output.mx = tokio::time::timeout(Duration::from_secs(5), check_mx(&output.syntax))
		.await
		.unwrap_or_else(|_| {
			Err(std::io::Error::new(std::io::ErrorKind::TimedOut, "MX lookup timed out").into())
		});
	output.debug.backend_name = config.backend_name.clone();
	output.debug.start_time = started;
	output.debug.end_time = chrono::Utc::now();
	output.debug.duration = (output.debug.end_time - started)
		.to_std()
		.unwrap_or_default();
	Ok(output)
}

async fn map_response(
	email: &str,
	response: SingleApiResponse,
) -> Result<CheckEmailOutput, VerificationError> {
	if response.result.as_deref() == Some("error")
		|| response.error.as_deref().is_some_and(|s| !s.is_empty())
	{
		return Err(VerificationError("MillionVerifier rejected the verification request; check API credentials and credit balance."));
	}
	if response.livemode != Some(true) {
		return Err(VerificationError(
			"MillionVerifier returned a demo response; it cannot verify a customer address.",
		));
	}
	if !response
		.email
		.as_deref()
		.is_some_and(|value| value.eq_ignore_ascii_case(email))
	{
		return Err(VerificationError(
			"MillionVerifier returned a result for a different address.",
		));
	}
	if response.free.is_none() || response.role.is_none() {
		return Err(VerificationError(
			"MillionVerifier omitted required account risk flags.",
		));
	}
	let (result, code, reachable) = match (response.result.as_deref(), response.resultcode) {
		(Some("ok"), Some(1)) if response.quality.as_deref() == Some("good") => {
			("ok", 1, Reachable::Safe)
		}
		(Some("catch_all"), Some(2)) => ("catch_all", 2, Reachable::Risky),
		(Some("unknown"), Some(3)) => ("unknown", 3, Reachable::Unknown),
		(Some("disposable"), Some(5)) => ("disposable", 5, Reachable::Risky),
		(Some("invalid"), Some(6)) => ("invalid", 6, Reachable::Invalid),
		_ => {
			return Err(VerificationError(
				"MillionVerifier returned an unrecognized or inconsistent result.",
			))
		}
	};
	let mut syntax = check_syntax(email);
	if !syntax.is_valid_syntax {
		return Err(VerificationError(
			"The provider result has invalid email syntax.",
		));
	}
	let mut misc = check_misc(&syntax, false, None).await;
	misc.is_disposable |= result == "disposable";
	misc.is_role_account |= response.role == Some(true);
	misc.is_b2c |= response.free == Some(true);
	if let Some(suggestion) = response.didyoumean.filter(|s| !s.is_empty() && s != email) {
		if check_syntax(&suggestion).is_valid_syntax {
			syntax.suggestion = Some(suggestion);
		}
	}
	let is_reachable = if reachable == Reachable::Safe
		&& (misc.is_disposable
			|| misc.is_role_account
			|| misc.is_spam_trap_domain
			|| syntax.suggestion.is_some())
	{
		Reachable::Risky
	} else {
		reachable
	};
	Ok(CheckEmailOutput {
		input: email.to_owned(),
		is_reachable,
		misc: Ok(misc),
		smtp: Err(SmtpError::AnyhowError(anyhow::anyhow!(
			"Not performed: verified by MillionVerifier."
		))),
		syntax,
		external_verification: Some(ExternalVerification {
			service: "millionverifier".into(),
			result: result.into(),
			result_code: code,
			subresult: response.subresult.unwrap_or_default(),
		}),
		..Default::default()
	})
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::scoring::{compute_score_with_context, EmailCategory, ScoringContext};
	use serde_json::json;
	use tokio::io::{AsyncReadExt, AsyncWriteExt};

	fn response(result: &str, code: i32) -> SingleApiResponse {
		serde_json::from_value(json!({
			"email": "person@example.com", "quality": "good", "result": result,
			"resultcode": code, "subresult": "ok", "free": false, "role": false,
			"didyoumean": "", "credits": 100, "executiontime": 1, "error": "", "livemode": true
		}))
		.unwrap()
	}

	#[tokio::test]
	async fn provider_statuses_survive_scoring_and_positive_history() {
		for (result, code, category, safe) in [
			("ok", 1, EmailCategory::Valid, true),
			("catch_all", 2, EmailCategory::Risky, false),
			("unknown", 3, EmailCategory::Unknown, false),
			("disposable", 5, EmailCategory::Risky, false),
			("invalid", 6, EmailCategory::Invalid, false),
		] {
			let output = map_response("person@example.com", response(result, code))
				.await
				.unwrap();
			let mut context = ScoringContext::default();
			context.tenant_history.safe_count_180d = 100;
			context.tenant_history.latest_safe_days_ago = Some(0);
			let scored = compute_score_with_context(&output, &context);
			assert_eq!(scored.score.category, category, "{result}");
			assert_eq!(scored.score.safe_to_send, safe, "{result}");
			assert!(!scored.score.signals.smtp_can_connect);
			let serialized = serde_json::to_value(output).unwrap();
			assert_eq!(
				serialized["external_verification"]["service"],
				"millionverifier"
			);
			assert!(serialized["smtp"]["error"].is_object());
			assert!(serialized.get("credits").is_none());
		}
	}

	#[tokio::test]
	async fn demo_errors_missing_flags_and_inconsistent_responses_fail_closed() {
		let mut fixtures = vec![
			response("error", 4),
			response("ok", 6),
			response("new_status", 9),
		];
		let mut demo = response("ok", 1);
		demo.livemode = Some(false);
		fixtures.push(demo);
		let mut flags = response("ok", 1);
		flags.role = None;
		fixtures.push(flags);
		let mut wrong_email = response("ok", 1);
		wrong_email.email = Some("someoneelse@example.com".into());
		fixtures.push(wrong_email);
		for response in fixtures {
			assert!(map_response("person@example.com", response).await.is_err());
		}
		let mut role = response("ok", 1);
		role.role = Some(true);
		let output = map_response("person@example.com", role).await.unwrap();
		assert!(!crate::scoring::compute_score(&output).safe_to_send);
	}

	#[test]
	fn secrets_are_not_serialized_logged_or_accepted_as_demo_keys() {
		let mut config = MillionVerifierConfig::default();
		config.api_key = Some("fake-secret-for-test".into());
		assert!(!format!("{config:?}").contains("fake-secret-for-test"));
		assert!(!serde_json::to_string(&config)
			.unwrap()
			.contains("fake-secret-for-test"));
		config
			.validate(VerificationBackend::Millionverifier)
			.unwrap();
		config.api_key = Some("API_KEY_FOR_OK".into());
		assert!(config
			.validate(VerificationBackend::Millionverifier)
			.is_err());
	}

	#[tokio::test]
	async fn generated_client_encodes_queries_and_decodes_http_200_errors() {
		let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
		let addr = listener.local_addr().unwrap();
		let server = tokio::spawn(async move {
			let (mut stream, _) = listener.accept().await.unwrap();
			let mut bytes = vec![0; 8192];
			let n = stream.read(&mut bytes).await.unwrap();
			let request = String::from_utf8_lossy(&bytes[..n]);
			let target = request.split_whitespace().nth(1).unwrap();
			let url = reqwest::Url::parse(&format!("http://localhost{target}")).unwrap();
			let params: std::collections::HashMap<_, _> = url.query_pairs().collect();
			assert_eq!(url.path(), "/api/v3/");
			assert_eq!(params["api"], "secret&value");
			assert_eq!(params["email"], "person+tag@example.com");
			let body = r#"{"result":"error","resultcode":4,"error":"insufficient_credits","subresult":"","livemode":true}"#;
			stream.write_all(format!(
				"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len()
			).as_bytes()).await.unwrap();
		});
		let mut config = MillionVerifierConfig::default().client().clone();
		config.base_path = format!("http://{addr}");
		let response =
			single_verification(&config, "secret&value", "person+tag@example.com", Some(20))
				.await
				.unwrap();
		assert_eq!(response.error.as_deref(), Some("insufficient_credits"));
		assert!(map_response("person+tag@example.com", response)
			.await
			.is_err());
		server.await.unwrap();
	}

	#[tokio::test]
	async fn generated_transport_error_does_not_expose_query_credentials() {
		let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
		let addr = listener.local_addr().unwrap();
		drop(listener);
		let mut config = MillionVerifierConfig::default().client().clone();
		config.base_path = format!("http://{addr}");
		let error = single_verification(
			&config,
			"fake-secret-for-test",
			"person@example.com",
			Some(2),
		)
		.await
		.unwrap_err();
		assert!(!format!("{error:?} {error}").contains("fake-secret-for-test"));
		assert!(!format!("{error:?} {error}").contains("person@example.com"));
	}

	#[tokio::test]
	#[ignore = "Calls MillionVerifier using public demo keys; consumes no account credits"]
	async fn millionverifier_public_demo_contract() {
		for (key, result) in [
			("API_KEY_FOR_OK", "ok"),
			("API_KEY_FOR_CATCH_ALL", "catch_all"),
			("API_KEY_FOR_UNKOWN", "unknown"),
			("API_KEY_FOR_INVALID", "invalid"),
			("API_KEY_FOR_DISPOSABLE", "disposable"),
			("API_KEY_FOR_ERROR_INSUFFICIENT_CREDITS", ""),
		] {
			let config = MillionVerifierConfig::default();
			let response = single_verification(config.client(), key, "person@example.com", Some(2))
				.await
				.unwrap();
			assert_eq!(response.result.as_deref(), Some(result), "{key}");
			if result.is_empty() {
				assert_eq!(response.resultcode, Some(4));
				assert!(response
					.error
					.as_deref()
					.is_some_and(|error| !error.is_empty()));
			}
			assert_eq!(response.livemode, Some(false), "{key}");
			assert!(map_response("person@example.com", response).await.is_err());
		}
	}
}
