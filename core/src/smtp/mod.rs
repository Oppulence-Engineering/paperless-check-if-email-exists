mod connect;
mod error;
mod gmail;
mod headless;
mod http_api;
mod outlook;
mod parser;
pub mod verif_method;
mod yahoo;

use crate::util::input_output::CheckEmailInput;
use crate::EmailAddress;
use connect::check_smtp_with_retry;
pub use connect::probe_smtp_greeting;
use hickory_proto::rr::rdata::MX;
use hickory_proto::rr::Name;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::default::Default;
use verif_method::{
	EmailProvider, EverythingElseVerifMethod, GmailVerifMethod, HotmailB2BVerifMethod,
	HotmailB2CVerifMethod, VerifMethodSmtp, VerifMethodSmtpConfig, YahooVerifMethod,
};

pub use crate::mx::{is_gmail, is_hotmail, is_hotmail_b2b, is_hotmail_b2c, is_yahoo};
pub use error::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct SmtpDebugVerifMethodSmtp {
	/// The host we connected to via SMTP.
	pub host: String,
	/// The proxy used for the SMTP connection.
	pub verif_method: VerifMethodSmtpConfig,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum SmtpDebugVerifMethod {
	/// Email verification was done via SMTP.
	Smtp(SmtpDebugVerifMethodSmtp),
	/// Email verification was done via an HTTP API.
	Api,
	/// Email verification was done via a headless browser.
	Headless,
	/// Email verification was skipped.
	#[default]
	Skipped,
}

/// Details that we gathered from connecting to this email via SMTP
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SmtpDetails {
	/// Are we able to connect to the SMTP server?
	pub can_connect_smtp: bool,
	/// Is this email account's inbox full?
	pub has_full_inbox: bool,
	/// Does this domain have a catch-all email address?
	pub is_catch_all: bool,
	/// Can we send an email to this address?
	pub is_deliverable: bool,
	/// Is the email blocked or disabled by the provider?
	pub is_disabled: bool,
}

/// Debug information on how the SMTP verification went.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct SmtpDebug {
	/// The verification method used for the email.
	pub verif_method: SmtpDebugVerifMethod,
}

/// Try MX hosts within one budget, without replacing a definitive recipient answer.
pub(crate) async fn check_smtp_mx(
	to_email: &EmailAddress,
	hosts: &[&MX],
	domain: &str,
	input: &CheckEmailInput,
) -> (Result<SmtpDetails, SmtpError>, SmtpDebug) {
	let mut hosts = hosts.to_vec();
	hosts.shuffle(&mut rand::rng());
	hosts.sort_by_key(|mx| mx.preference);
	let budget = hosts
		.iter()
		.filter_map(|mx| {
			input
				.verif_method
				.smtp_config(EmailProvider::from_mx_host(&mx.exchange.to_string()))
				.and_then(|config| config.smtp_timeout)
		})
		.max()
		.unwrap_or(std::time::Duration::from_secs(60));
	let deadline = tokio::time::Instant::now() + budget;
	let mut last = (Err(SmtpError::Timeout(budget)), SmtpDebug::default());
	for (index, mx) in hosts.iter().enumerate() {
		let remaining = deadline.saturating_duration_since(tokio::time::Instant::now());
		if remaining.is_zero() {
			break;
		}
		// Leave a chance for backups even when the preferred server never responds.
		let attempt_budget = remaining / (hosts.len() - index) as u32;
		last = match tokio::time::timeout(
			attempt_budget,
			check_smtp(to_email, &mx.exchange, domain, input),
		)
		.await
		{
			Ok(result) => result,
			Err(_) => {
				let debug = input
					.verif_method
					.smtp_config(EmailProvider::from_mx_host(&mx.exchange.to_string()))
					.map(|config| SmtpDebug {
						verif_method: SmtpDebugVerifMethod::Smtp(SmtpDebugVerifMethodSmtp {
							host: mx.exchange.to_string(),
							verif_method: config.clone(),
						}),
					})
					.unwrap_or_default();
				(Err(SmtpError::Timeout(attempt_budget)), debug)
			}
		};
		tracing::debug!(target: crate::LOG_TARGET, mx_host=%mx.exchange, result=?last.0, "MX attempt completed");
		let retryable = matches!(
			&last.0,
			Err(SmtpError::IOError(_) | SmtpError::Timeout(_))
				| Err(SmtpError::AsyncSmtpError(
					async_smtp::error::Error::Io(_) | async_smtp::error::Error::Transient(_)
				)) | Err(SmtpError::Socks5(fast_socks5::SocksError::ReplyError(
				fast_socks5::ReplyError::NetworkUnreachable
					| fast_socks5::ReplyError::HostUnreachable
					| fast_socks5::ReplyError::ConnectionRefused
					| fast_socks5::ReplyError::ConnectionTimeout
					| fast_socks5::ReplyError::TtlExpired
			)))
		);
		if !retryable || !matches!(last.1.verif_method, SmtpDebugVerifMethod::Smtp(_)) {
			break;
		}
	}
	last
}

/// Get all email details we can from one single `EmailAddress`, without
/// retries.
pub async fn check_smtp(
	to_email: &EmailAddress,
	host: &Name,
	domain: &str,
	input: &CheckEmailInput,
) -> (Result<SmtpDetails, SmtpError>, SmtpDebug) {
	let host_str = host.to_string();
	let to_email_str = to_email.to_string();
	let email_provider = EmailProvider::from_mx_host(&host_str);

	// Handle all non-SMTP verifications first, and return early. For the rest,
	// we'll use SMTP, and return the config.
	let smtp_verif_method_config = match &email_provider {
		EmailProvider::HotmailB2C => match &input.verif_method.hotmailb2c {
			HotmailB2CVerifMethod::Headless => {
				return (
					outlook::headless::check_password_recovery(
						&to_email_str,
						&input.webdriver_addr,
						&input.webdriver_config,
					)
					.await
					.map_err(Into::into),
					SmtpDebug {
						verif_method: SmtpDebugVerifMethod::Headless,
					},
				);
			}
			HotmailB2CVerifMethod::Smtp(c) => c,
		},
		EmailProvider::Yahoo => match &input.verif_method.yahoo {
			YahooVerifMethod::Api => {
				return (
					yahoo::check_api(&to_email_str, input)
						.await
						.map_err(Into::into),
					SmtpDebug {
						verif_method: SmtpDebugVerifMethod::Api,
					},
				);
			}
			YahooVerifMethod::Headless => {
				return (
					yahoo::check_headless(
						&to_email_str,
						&input.webdriver_addr,
						&input.webdriver_config,
					)
					.await
					.map_err(Into::into),
					SmtpDebug {
						verif_method: SmtpDebugVerifMethod::Headless,
					},
				);
			}
			YahooVerifMethod::Smtp(c) => c,
		},
		EmailProvider::Gmail => match &input.verif_method.gmail {
			GmailVerifMethod::Smtp(c) => c,
		},
		EmailProvider::HotmailB2B => match &input.verif_method.hotmailb2b {
			HotmailB2BVerifMethod::Smtp(c) => c,
		},
		EmailProvider::Mimecast => match &input.verif_method.mimecast {
			verif_method::MimecastVerifMethod::Smtp(c) => c,
		},
		EmailProvider::Proofpoint => match &input.verif_method.proofpoint {
			verif_method::ProofpointVerifMethod::Smtp(c) => c,
		},
		EmailProvider::EverythingElse => match &input.verif_method.everything_else {
			EverythingElseVerifMethod::Smtp(c) => c,
		},
	}
	.clone();

	// TODO: There's surely a way to not clone here.
	let verif_method = VerifMethodSmtp::new(
		smtp_verif_method_config.clone(),
		input.verif_method.get_proxy(email_provider).cloned(),
	);

	(
		check_smtp_with_retry(
			to_email,
			&host_str,
			domain,
			&verif_method,
			verif_method.config.retries,
		)
		.await,
		SmtpDebug {
			verif_method: SmtpDebugVerifMethod::Smtp(SmtpDebugVerifMethodSmtp {
				host: host_str,
				verif_method: smtp_verif_method_config,
			}),
		},
	)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::smtp::verif_method::GmailVerifMethod;
	use crate::smtp::verif_method::VerifMethod;
	use crate::smtp::verif_method::VerifMethodSmtpConfig;
	use crate::CheckEmailInputBuilder;
	use crate::EmailAddress;
	use hickory_proto::rr::Name;
	use std::{str::FromStr, time::Duration};
	use tokio::runtime::Runtime;

	#[test]
	fn should_timeout() {
		let runtime = Runtime::new().unwrap();

		let to_email = EmailAddress::from_str("foo@gmail.com").unwrap();
		let host = Name::from_str("alt4.aspmx.l.google.com.").unwrap();
		let input = CheckEmailInputBuilder::default()
			.to_email("foo@gmail.com".into())
			.verif_method(VerifMethod {
				gmail: GmailVerifMethod::Smtp(VerifMethodSmtpConfig {
					smtp_timeout: Some(Duration::from_millis(1)),
					retries: 1,
					..Default::default()
				}),
				..Default::default()
			})
			.build()
			.unwrap();

		let (res, smtp_debug) = runtime.block_on(check_smtp(&to_email, &host, "gmail.com", &input));
		match smtp_debug.verif_method {
			SmtpDebugVerifMethod::Smtp(SmtpDebugVerifMethodSmtp { host, verif_method }) => {
				assert_eq!(host, "alt4.aspmx.l.google.com.");
				assert_eq!(verif_method.smtp_port, 25);
				assert_eq!(verif_method.smtp_timeout, Some(Duration::from_millis(1)));
				assert_eq!(verif_method.retries, 1);
				assert_eq!(verif_method.proxy, None);
			}
			_ => panic!("Expected SmtpDebugVerifMethod::Smtp"),
		}

		match res {
			Err(SmtpError::Timeout(_)) => (), // ErrorKind == Timeout
			_ => panic!("check_smtp did not time out"),
		}
	}
}
