//! Startup checks for the outbound SMTP path.
//!
//! Every verification depends on opening port 25 to the recipient's mail
//! server. When the host blocks that port (most cloud providers do by default,
//! DigitalOcean included) or the EHLO identity is rejected, each check comes
//! back "unknown" and nothing else points at the cause. These checks put it in
//! the logs at boot.

use crate::config::BackendConfig;
use check_if_email_exists::LOG_TARGET;
use std::time::Duration;
use tokio::net::TcpStream;
use tracing::{error, info, warn};

/// A mail exchanger that accepts connections from anywhere port 25 is open.
const PROBE_MX: &str = "gmail-smtp-in.l.google.com:25";
const PROBE_TIMEOUT: Duration = Duration::from_secs(10);

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
		&& !matches!(tld, "localhost" | "local" | "internal" | "lan" | "test" | "invalid")
		&& name.split('.').all(|label| !label.is_empty())
}

/// Logs SMTP identity problems and, when no proxy is configured, whether this
/// host can open outbound port 25 at all. Never fails startup.
pub async fn log_smtp_egress_checks(config: &BackendConfig) {
	for warning in smtp_identity_warnings(&config.hello_name, &config.from_email) {
		warn!(target: LOG_TARGET, "{}", warning);
	}

	if config.proxy.is_some() || !config.overrides.proxies.is_empty() {
		info!(target: LOG_TARGET, "SMTP verification is routed through a SOCKS5 proxy");
		return;
	}

	match tokio::time::timeout(PROBE_TIMEOUT, TcpStream::connect(PROBE_MX)).await {
		Ok(Ok(_)) => info!(target: LOG_TARGET, "Outbound SMTP port 25 is reachable"),
		Ok(Err(probe_error)) => log_blocked_egress(&probe_error.to_string()),
		Err(_) => log_blocked_egress("timed out"),
	}
}

fn log_blocked_egress(reason: &str) {
	error!(
		target: LOG_TARGET,
		probe = PROBE_MX,
		reason = reason,
		"Outbound SMTP port 25 is unreachable from this host, so every verification will \
		 come back unknown. Route SMTP through a SOCKS5 proxy (RCH__PROXY__HOST and \
		 RCH__PROXY__PORT) or run the worker on a host that allows outbound port 25."
	);
}

#[cfg(test)]
mod tests {
	use super::*;

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
