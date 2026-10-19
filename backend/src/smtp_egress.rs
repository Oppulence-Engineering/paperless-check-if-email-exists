//! Startup checks for the outbound SMTP path.
//!
//! Every verification depends on opening port 25 to the recipient's mail
//! server, directly or through a SOCKS5 proxy. When the host or the proxy
//! blocks that port (most cloud and proxy providers do by default), the proxy
//! rejects our credentials, or the EHLO identity is rejected, each check comes
//! back "unknown" and nothing else points at the cause. These checks put it in
//! the logs at boot.

use crate::config::BackendConfig;
use check_if_email_exists::smtp::{probe_smtp_greeting, SmtpError};
use check_if_email_exists::{CheckEmailInputProxy, LOG_TARGET};
use fast_socks5::SocksError;
use std::time::Duration;
use tracing::{error, info, warn};

/// A mail exchanger that greets any host whose port 25 is open, so its
/// greeting proves the whole route works.
const PROBE_MX_HOST: &str = "gmail-smtp-in.l.google.com";
const PROBE_MX_PORT: u16 = 25;
/// Longer than the 10 second direct connect timeout, so a blocked port shows
/// up as a connect failure rather than a missing greeting.
const PROBE_TIMEOUT: Duration = Duration::from_secs(20);

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
	if !from_domain.map(is_public_domain).unwrap_or(false) {
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
	!tld.is_empty()
		&& !matches!(
			tld,
			"localhost" | "local" | "internal" | "lan" | "test" | "invalid"
		) && name.split('.').all(|label| !label.is_empty())
}

/// What a probe of one SMTP route found.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Diagnosis {
	/// The mail server greeted with 220: verifications can run.
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

/// Logs SMTP identity problems, then opens an SMTP connection the way a
/// verification would, through each configured proxy and directly when the
/// default route has none, and logs what it found. Never fails startup.
pub async fn log_smtp_egress_checks(config: &BackendConfig) {
	for warning in smtp_identity_warnings(&config.hello_name, &config.from_email) {
		warn!(target: LOG_TARGET, "{}", warning);
	}

	if config.proxy.is_none() {
		probe_route("direct", None).await;
	}
	for (proxy_id, proxy) in &config.get_verif_method().proxies {
		let route = format!("proxy {} ({}:{})", proxy_id, proxy.host, proxy.port);
		probe_route(&route, Some(proxy)).await;
	}
}

async fn probe_route(route: &str, proxy: Option<&CheckEmailInputProxy>) {
	let result = probe_smtp_greeting(PROBE_MX_HOST, PROBE_MX_PORT, proxy, PROBE_TIMEOUT).await;
	let detail = match &result {
		Ok(greeting) => greeting.clone(),
		Err(probe_error) => probe_error.to_string(),
	};
	log_diagnosis(route, diagnose(&result), &detail);
}

fn log_diagnosis(route: &str, diagnosis: Diagnosis, detail: &str) {
	let probe = format!("{}:{}", PROBE_MX_HOST, PROBE_MX_PORT);
	match diagnosis {
		Diagnosis::Ready => info!(
			target: LOG_TARGET, route, probe, greeting = detail,
			"SMTP route works: the mail server greeted us"
		),
		Diagnosis::SessionRefused => warn!(
			target: LOG_TARGET, route, probe, greeting = detail,
			"The mail server answered but refused the session. The sending IP (the proxy's \
			 exit IP when a proxy is used) may be on a blocklist; verifications on this \
			 route may come back unknown."
		),
		Diagnosis::NoGreeting => error!(
			target: LOG_TARGET, route, probe, reason = detail,
			"Connected, but the mail server never sent its greeting, so verifications on \
			 this route come back unknown. Proxies that block port 25 often drop the traffic \
			 silently: ask the proxy provider whether outbound port 25 is allowed."
		),
		Diagnosis::ProxyAuthRejected => error!(
			target: LOG_TARGET, route, probe, reason = detail,
			"The SOCKS5 proxy rejected our credentials, so every verification routed through \
			 it comes back unknown. Check RCH__PROXY__USERNAME and RCH__PROXY__PASSWORD (or \
			 the proxy's entry in overrides.proxies)."
		),
		Diagnosis::ProxyRefusedTarget => error!(
			target: LOG_TARGET, route, probe, reason = detail,
			"The SOCKS5 proxy accepted us but would not connect to the mail server on port \
			 25, so every verification routed through it comes back unknown. Most proxy \
			 providers block port 25 to stop spam: ask the provider to enable SMTP on this \
			 proxy, or switch to a proxy sold for SMTP use."
		),
		Diagnosis::ProxyUnreachable => error!(
			target: LOG_TARGET, route, probe, reason = detail,
			"Could not reach the SOCKS5 proxy, so every verification routed through it comes \
			 back unknown. Check RCH__PROXY__HOST and RCH__PROXY__PORT, and that the proxy \
			 allows connections from this host's IP."
		),
		Diagnosis::DirectBlocked => error!(
			target: LOG_TARGET, route, probe, reason = detail,
			"Outbound SMTP port 25 is unreachable from this host, so verifications that \
			 connect directly come back unknown. Ask the hosting provider to lift its port \
			 25 block (on Hetzner Cloud, a limit request), route SMTP through a SOCKS5 proxy \
			 that allows port 25 (RCH__PROXY__HOST and RCH__PROXY__PORT), or run the worker \
			 on a host that allows outbound port 25."
		),
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
}
