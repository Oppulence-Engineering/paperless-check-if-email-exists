//! Cached, bounded SMTP greeting probes; HTTP health requests never open SMTP.

use crate::config::BackendConfig;
use check_if_email_exists::smtp::verif_method::EmailProvider;
use check_if_email_exists::smtp::{probe_smtp_greeting, SmtpError};
use check_if_email_exists::LOG_TARGET;
use fast_socks5::SocksError;
use serde::Serialize;
use std::{
	collections::{BTreeMap, HashSet},
	sync::{Arc, RwLock},
	time::{Duration, Instant},
};
use tracing::{error, info, warn};

/// A greeting from this public target demonstrates reachability to that target.
const PROBE_MX_HOST: &str = "gmail-smtp-in.l.google.com";
/// Longer than the 10 second direct connect timeout, so a blocked port shows
/// up as a connect failure rather than a missing greeting.
const PROBE_TIMEOUT: Duration = Duration::from_secs(20);
const PROBE_INTERVAL: Duration = Duration::from_secs(60);
const MAX_AGE: Duration = Duration::from_secs(180);

#[derive(Debug, Default)]
pub struct SmtpHealth {
	last: RwLock<Option<(Instant, SmtpCapability)>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct SmtpCapability {
	pub status: &'static str,
	pub checked_at: Option<i64>,
	routes: BTreeMap<String, Diagnosis>,
}

impl SmtpHealth {
	pub fn snapshot(&self, enabled: bool) -> SmtpCapability {
		let last = self.last.read().expect("SMTP health lock");
		let mut snapshot =
			last.as_ref()
				.map(|(_, value)| value.clone())
				.unwrap_or(SmtpCapability {
					status: "pending",
					checked_at: None,
					routes: BTreeMap::new(),
				});
		if !enabled {
			snapshot.status = "disabled";
		} else if last.as_ref().is_some_and(|(at, _)| at.elapsed() > MAX_AGE) {
			snapshot.status = "stale";
		}
		snapshot
	}

	fn record(&self, routes: BTreeMap<String, Diagnosis>) {
		let ready = !routes.is_empty() && routes.values().all(|status| *status == Diagnosis::Ready);
		*self.last.write().expect("SMTP health lock") = Some((
			Instant::now(),
			SmtpCapability {
				status: if ready { "ready" } else { "unavailable" },
				checked_at: Some(chrono::Utc::now().timestamp()),
				routes,
			},
		));
	}
}

pub fn require_smtp_ready(config: &BackendConfig) -> Result<(), crate::http::ReacherResponseError> {
	if config.verification_backend == crate::verification::VerificationBackend::Millionverifier {
		return config
			.millionverifier
			.validate(config.verification_backend)
			.map_err(|_| {
				crate::http::ReacherResponseError::new(
				warp::http::StatusCode::SERVICE_UNAVAILABLE,
				"MillionVerifier is not configured with a live API key; no check was dispatched or charged.",
			)
			});
	}
	if config.smtp_health_enabled && config.smtp_health.snapshot(true).status != "ready" {
		return Err(crate::http::ReacherResponseError::new(
			warp::http::StatusCode::SERVICE_UNAVAILABLE,
			"SMTP verification capability is unavailable; no new check was dispatched or charged.",
		));
	}
	Ok(())
}

fn smtp_providers() -> [EmailProvider; 7] {
	[
		EmailProvider::Gmail,
		EmailProvider::HotmailB2B,
		EmailProvider::HotmailB2C,
		EmailProvider::Mimecast,
		EmailProvider::Proofpoint,
		EmailProvider::Yahoo,
		EmailProvider::EverythingElse,
	]
}

pub fn validate_smtp_identity(config: &BackendConfig) -> Result<(), anyhow::Error> {
	if config.verification_backend == crate::verification::VerificationBackend::Millionverifier {
		return config.millionverifier.validate(config.verification_backend);
	}
	if !config.smtp_health_enabled {
		return Ok(());
	}
	let method = config.get_verif_method();
	for provider in smtp_providers() {
		if let Some(route) = method.smtp_config(provider) {
			anyhow::ensure!(smtp_identity_warnings(&route.hello_name, &route.from_email).is_empty(),
				"SMTP requires a public HELO hostname and valid sender address; configure RCH__HELLO_NAME and RCH__FROM_EMAIL");
		}
	}
	Ok(())
}

pub async fn monitor_smtp_egress(config: Arc<BackendConfig>) {
	if config.verification_backend == crate::verification::VerificationBackend::Millionverifier {
		return;
	}
	if !config.smtp_health_enabled {
		warn!(target: LOG_TARGET, "SMTP capability monitoring and admission guard are disabled");
		return;
	}
	let mut interval = tokio::time::interval(PROBE_INTERVAL);
	interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
	loop {
		interval.tick().await;
		log_smtp_egress_checks(&config).await;
	}
}

/// Problems with the EHLO name or MAIL FROM address that make receiving
/// servers reject the conversation before the mailbox is ever checked.
pub fn smtp_identity_warnings(hello_name: &str, from_email: &str) -> Vec<String> {
	let mut warnings = Vec::new();
	if !is_public_domain(hello_name) {
		warnings.push(format!(
			"hello_name {:?} is not a public domain; many mail servers reject EHLO from it. \
			 Set RCH__HELLO_NAME to a domain you own, ideally the reverse DNS of the sending IP.",
			hello_name
		));
	}
	let from_domain = from_email.rsplit_once('@').map(|(_, domain)| domain);
	if !from_domain.map(is_public_domain).unwrap_or(false)
		|| !check_if_email_exists::syntax::check_syntax(from_email).is_valid_syntax
	{
		warnings.push(format!(
			"from_email {:?} does not use a public domain; servers such as Gmail reject the \
			 MAIL FROM command. Set RCH__FROM_EMAIL to an address on a domain you own.",
			from_email
		));
	}
	warnings
}

fn is_public_domain(name: &str) -> bool {
	let name = name.trim().trim_end_matches('.').to_ascii_lowercase();
	let Some((_, tld)) = name.rsplit_once('.') else {
		return false;
	};
	name.len() <= 253
		&& name.parse::<std::net::IpAddr>().is_err()
		&& tld.len() >= 2
		&& tld.bytes().all(|byte| byte.is_ascii_alphabetic())
		&& !matches!(
			tld,
			"localhost" | "local" | "internal" | "lan" | "test" | "invalid"
		) && name.split('.').all(|label| {
		!label.is_empty()
			&& label.len() <= 63
			&& !label.starts_with('-')
			&& !label.ends_with('-')
			&& label
				.bytes()
				.all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
	})
}

/// What a probe of one SMTP route found.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum Diagnosis {
	/// The probe target sent a 220 greeting; recipient acceptance is untested.
	Ready,
	/// The mail server answered but refused the session, usually because the
	/// sending IP is on a blocklist.
	SessionRefused,
	/// The connection opened but no greeting arrived.
	NoGreeting,
	/// The proxy rejected the username or password, or needs ones that are
	/// not configured.
	ProxyAuthRejected,
	/// The proxy accepted us but would not connect to the mail server.
	ProxyRefusedTarget,
	/// The proxy itself could not be reached.
	ProxyUnreachable,
	/// This host could not open port 25 itself.
	DirectBlocked,
	IdentityUnresolvable,
}

fn diagnose(result: &Result<String, SmtpError>) -> Diagnosis {
	match result {
		Ok(greeting) if greeting.starts_with("220") => Diagnosis::Ready,
		Ok(greeting) if greeting.is_empty() => Diagnosis::NoGreeting,
		Ok(_) => Diagnosis::SessionRefused,
		Err(SmtpError::Socks5(
			SocksError::AuthenticationRejected(_) | SocksError::AuthMethodUnacceptable(_),
		)) => Diagnosis::ProxyAuthRejected,
		Err(SmtpError::Socks5(SocksError::ReplyError(_))) => Diagnosis::ProxyRefusedTarget,
		Err(SmtpError::Socks5(_)) => Diagnosis::ProxyUnreachable,
		Err(SmtpError::Timeout(_)) => Diagnosis::NoGreeting,
		Err(_) => Diagnosis::DirectBlocked,
	}
}

/// Probe only configured SMTP routes, without MAIL FROM, RCPT TO or message data.
pub async fn log_smtp_egress_checks(config: &BackendConfig) {
	let method = config.get_verif_method();
	let routes: HashSet<_> = IntoIterator::into_iter(smtp_providers())
		.filter_map(|provider| method.smtp_config(provider))
		.map(|route| {
			(
				route.proxy.clone(),
				route.smtp_port,
				route.hello_name.clone(),
			)
		})
		.collect();
	let checks = routes.into_iter().map(|(proxy_id, port, hello)| {
		let method = &method;
		async move {
			let route = format!("{}:{port}/{hello}", proxy_id.as_deref().unwrap_or("direct"));
			let diagnosis = tokio::time::timeout(PROBE_TIMEOUT, async {
				if check_if_email_exists::network::resolve_public_addresses(&hello, port)
					.await
					.is_err()
				{
					return Diagnosis::IdentityUnresolvable;
				}
				let proxy = proxy_id.as_ref().and_then(|id| method.proxies.get(id));
				if proxy_id.is_some() && proxy.is_none() {
					return Diagnosis::ProxyUnreachable;
				}
				diagnose(&probe_smtp_greeting(PROBE_MX_HOST, port, proxy, PROBE_TIMEOUT).await)
			})
			.await
			.unwrap_or(Diagnosis::NoGreeting);
			log_diagnosis(&route, diagnosis);
			(route, diagnosis)
		}
	});
	config.smtp_health.record(
		futures::future::join_all(checks)
			.await
			.into_iter()
			.collect(),
	);
}

fn log_diagnosis(route: &str, diagnosis: Diagnosis) {
	if diagnosis == Diagnosis::Ready {
		info!(target: LOG_TARGET, route, probe = PROBE_MX_HOST, "SMTP probe target sent a 220 greeting");
	} else {
		error!(target: LOG_TARGET, route, probe = PROBE_MX_HOST, diagnosis = ?diagnosis,
			"SMTP capability probe failed; new verification admission is paused");
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use fast_socks5::ReplyError;
	use std::io;

	#[test]
	fn tells_apart_each_way_the_smtp_route_fails() {
		let io_error = |kind| io::Error::new(kind, "test");
		for (result, expected) in [
			(Ok("220 mx.google.com ESMTP".to_string()), Diagnosis::Ready),
			(
				Ok("554 5.7.1 Client host blocked".to_string()),
				Diagnosis::SessionRefused,
			),
			// The server closed the connection without a word.
			(Ok(String::new()), Diagnosis::NoGreeting),
			(
				Err(SmtpError::Timeout(PROBE_TIMEOUT)),
				Diagnosis::NoGreeting,
			),
			(
				Err(SmtpError::Socks5(SocksError::AuthenticationRejected(
					"rejected".into(),
				))),
				Diagnosis::ProxyAuthRejected,
			),
			(
				Err(SmtpError::Socks5(SocksError::ReplyError(
					ReplyError::ConnectionNotAllowed,
				))),
				Diagnosis::ProxyRefusedTarget,
			),
			(
				Err(SmtpError::Socks5(SocksError::Io(io_error(
					io::ErrorKind::ConnectionRefused,
				)))),
				Diagnosis::ProxyUnreachable,
			),
			(
				Err(SmtpError::IOError(io_error(io::ErrorKind::TimedOut))),
				Diagnosis::DirectBlocked,
			),
		] {
			assert_eq!(diagnose(&result), expected, "{:?}", result);
		}
	}

	#[test]
	fn flags_identities_that_mail_servers_reject() {
		for (hello_name, from_email, expected) in [
			("localhost", "hello@localhost", 2),
			("", "", 2),
			("mailer", "bounce@mailer", 2),
			("verify.local", "a@verify.local", 2),
			("127.0.0.1", "a@127.0.0.1", 2),
			("bad_.example.com", "@example.com", 2),
			("example.com", "hello@localhost", 1),
			("verify.example.com", "verify@example.com", 0),
			("verify.example.com.", "verify@Example.COM", 0),
		] {
			assert_eq!(
				smtp_identity_warnings(hello_name, from_email).len(),
				expected,
				"hello_name={:?} from_email={:?}",
				hello_name,
				from_email
			);
		}
	}

	#[test]
	fn admission_requires_recent_success_and_identity_validation_covers_overrides() {
		let mut config = BackendConfig::empty();
		config.smtp_health_enabled = true;
		assert!(validate_smtp_identity(&config).is_err());
		config.hello_name = "verify.example.com".into();
		config.from_email = "verify@example.com".into();
		assert!(validate_smtp_identity(&config).is_ok());
		config.overrides.gmail = Some(
			check_if_email_exists::smtp::verif_method::GmailVerifMethod::Smtp(
				check_if_email_exists::smtp::verif_method::VerifMethodSmtpConfig {
					hello_name: "localhost".into(),
					..Default::default()
				},
			),
		);
		assert!(validate_smtp_identity(&config).is_err());
		config.overrides.gmail = None;
		assert!(require_smtp_ready(&config).is_err());
		config
			.smtp_health
			.record(BTreeMap::from([("direct".into(), Diagnosis::Ready)]));
		assert!(require_smtp_ready(&config).is_ok());
		config.smtp_health.last.write().unwrap().as_mut().unwrap().0 =
			Instant::now() - MAX_AGE - Duration::from_secs(1);
		assert_eq!(config.smtp_health.snapshot(true).status, "stale");
		assert!(require_smtp_ready(&config).is_err());
		config.smtp_health.record(BTreeMap::from([(
			"direct".into(),
			Diagnosis::ProxyRefusedTarget,
		)]));
		assert!(require_smtp_ready(&config).is_err());
		config.smtp_health_enabled = false;
		assert!(require_smtp_ready(&config).is_ok());
	}

	#[tokio::test]
	async fn health_requests_use_cached_evidence_and_liveness_stays_independent() {
		let mut config = BackendConfig::empty();
		config.smtp_health_enabled = true;
		let config = Arc::new(config);
		let routes = crate::http::create_routes(Arc::clone(&config));
		for _ in 0..2 {
			let response = warp::test::request().path("/readyz").reply(&routes).await;
			assert_eq!(response.status(), 503);
			let body: serde_json::Value = serde_json::from_slice(response.body()).unwrap();
			assert_eq!(body["checks"]["smtp"]["status"], "pending");
			assert!(body["checks"]["smtp"]["checked_at"].is_null());
		}
		assert_eq!(
			warp::test::request()
				.path("/healthz")
				.reply(&routes)
				.await
				.status(),
			200
		);
		let metrics = warp::test::request().path("/metrics").reply(&routes).await;
		assert!(std::str::from_utf8(metrics.body())
			.unwrap()
			.contains("reacher_smtp_capability_ready 0"));
		config
			.smtp_health
			.record(BTreeMap::from([("direct".into(), Diagnosis::Ready)]));
		assert_eq!(
			warp::test::request()
				.path("/readyz")
				.reply(&routes)
				.await
				.status(),
			200
		);
	}
}
