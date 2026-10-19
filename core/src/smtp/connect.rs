use async_recursion::async_recursion;
use async_smtp::commands::{MailCommand, RcptCommand};
use async_smtp::extension::ClientId;
use async_smtp::{SmtpClient, SmtpTransport};
use fast_socks5::client::Config;
use fast_socks5::{client::Socks5Stream, Result};
use rand::rngs::SmallRng;
use rand::{distr::Alphanumeric, RngExt};
use std::iter;
use std::str::FromStr;
use tokio::io::{AsyncBufRead, AsyncBufReadExt, AsyncRead, AsyncWrite, BufStream};
use tokio::net::TcpStream;

use super::parser::{self, RcptRejection};
use super::verif_method::VerifMethodSmtp;
use super::{SmtpDetails, SmtpError};
use crate::rules::{has_rule, Rule};
use crate::{CheckEmailInputProxy, EmailAddress, LOG_TARGET};

/// How long to wait for the TCP handshake with an MX host. A reachable mail
/// server answers in well under a second.
const SMTP_CONNECT_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(10);

// Define a new trait that combines AsyncRead, AsyncWrite, and Unpin
trait AsyncReadWrite: AsyncRead + AsyncWrite + Unpin + Send {}
impl<T: AsyncRead + AsyncWrite + Unpin + Send> AsyncReadWrite for T {}

/// Try to send an smtp command, close and return Err if fails.
macro_rules! try_smtp (
    ($res: expr, $client: ident, $to_email: expr, $host: expr, $port: expr) => ({
		if let Err(err) = $res {
			tracing::debug!(
				target: LOG_TARGET,
				email=$to_email.to_string(),
				mx_host=$host,
				port=$port,
				error=?err,
				"Closing connection due to error"
			);
			// Try to close the connection, but ignore if there's an error.
			let _ = $client.quit().await;

			return Err(SmtpError::AsyncSmtpError(err));
		}
    })
);

/// Opens a byte stream to `host:port`, through the SOCKS5 proxy when one is
/// given.
async fn open_stream(
	host: &str,
	port: u16,
	proxy: Option<&CheckEmailInputProxy>,
) -> Result<BufStream<Box<dyn AsyncReadWrite>>, SmtpError> {
	// hostname verification fails if it ends with '.', for example, using
	// SOCKS5 proxies we can `io: incomplete` error.
	let clean_host = host.trim_end_matches('.').to_string();
	let stream: Box<dyn AsyncReadWrite> = match proxy {
		Some(proxy) => {
			let mut config = Config::default();
			if let Some(timeout_ms) = proxy.timeout_ms {
				config.set_connect_timeout(std::time::Duration::from_millis(timeout_ms));
			}

			let socks_stream = if let (Some(username), Some(password)) =
				(&proxy.username, &proxy.password)
			{
				Socks5Stream::connect_with_password(
					(proxy.host.as_ref(), proxy.port),
					clean_host,
					port,
					username.clone(),
					password.clone(),
					config,
				)
				.await?
			} else {
				Socks5Stream::connect((proxy.host.as_ref(), proxy.port), clean_host, port, config)
					.await?
			};
			Box::new(socks_stream)
		}
		None => {
			let address = format!("{}:{}", clean_host, port);
			// A firewall that drops outbound SYNs (cloud hosts often block
			// port 25) otherwise holds this worker slot for the kernel's ~2
			// minute connect timeout. Failing here as an I/O error also tells
			// callers that no SMTP conversation took place.
			let tcp_stream =
				tokio::time::timeout(SMTP_CONNECT_TIMEOUT, TcpStream::connect(&address))
					.await
					.map_err(|_| {
						std::io::Error::new(
							std::io::ErrorKind::TimedOut,
							format!("connecting to {address} timed out"),
						)
					})??;
			Box::new(tcp_stream)
		}
	};
	Ok(BufStream::new(stream))
}

/// Connects to `host:port` the way a verification does, directly or through
/// `proxy`, and returns the server's first greeting line. Used to diagnose
/// the SMTP path without verifying any address.
pub async fn probe_smtp_greeting(
	host: &str,
	port: u16,
	proxy: Option<&CheckEmailInputProxy>,
	timeout: std::time::Duration,
) -> Result<String, SmtpError> {
	let probe = async {
		let mut stream = open_stream(host, port, proxy).await?;
		let mut greeting = String::new();
		stream.read_line(&mut greeting).await?;
		Ok(greeting.trim_end().to_string())
	};
	tokio::time::timeout(timeout, probe)
		.await
		.map_err(|_| SmtpError::Timeout(timeout))?
}

/// Connect to an SMTP host and return the configured client transport.
async fn connect_to_smtp_host(
	to_email: &EmailAddress,
	mx_host: &str,
	verif_method: &VerifMethodSmtp,
) -> Result<SmtpTransport<BufStream<Box<dyn AsyncReadWrite>>>, SmtpError> {
	let clean_host = mx_host.trim_end_matches('.').to_string();
	let smtp_client = SmtpClient::new()
		.hello_name(ClientId::Domain(verif_method.config.hello_name.to_string()))
		// Sometimes, using socks5 proxy, we get an `io: incomplete` error
		// when using pipelining and sending two consecutive RCPT TO commands.
		.pipelining(false);

	let stream = open_stream(
		mx_host,
		verif_method.config.smtp_port,
		verif_method.proxy.as_ref(),
	)
	.await?;

	let mut smtp_transport = SmtpTransport::new(smtp_client, stream).await?;

	// Set "MAIL FROM"
	let from_email = EmailAddress::from_str(&verif_method.config.from_email).unwrap_or_else(|_| {
		tracing::warn!(
			target: LOG_TARGET,
			from_email=verif_method.config.from_email,
			"Invalid 'from_email' provided, using default: 'user@example.org'"
		);
		EmailAddress::from_str("user@example.org").expect("Default email is valid")
	});
	try_smtp!(
		smtp_transport
			.get_mut()
			.command(MailCommand::new(Some(from_email.into_inner()), vec![]))
			.await,
		smtp_transport,
		to_email,
		clean_host,
		verif_method.config.smtp_port
	);

	Ok(smtp_transport)
}

/// Description of the deliverability information we can gather from
/// communicating with the SMTP server.
struct Deliverability {
	/// Is this email account's inbox full?
	has_full_inbox: bool,
	/// Can we send an email to this address?
	is_deliverable: bool,
	/// Is the email blocked or disabled by the provider?
	is_disabled: bool,
}

/// Checks deliverability of a target email address using the provided SMTP transport.
async fn check_email_deliverability<S: AsyncBufRead + AsyncWrite + Unpin + Send>(
	smtp_transport: &mut SmtpTransport<S>,
	to_email: &EmailAddress,
) -> Result<Deliverability, SmtpError> {
	match smtp_transport
		.get_mut()
		.command(RcptCommand::new(to_email.clone().into_inner(), vec![]))
		.await
	{
		// According to RFC 5321, `RCPT TO` command succeeds with 250 and
		// 251 codes only (no 3xx codes at all):
		// https://tools.ietf.org/html/rfc5321#page-56
		//
		// Where the 251 code is used for forwarding, which is not our case,
		// because we always deliver to the SMTP server hosting the address
		// itself.
		//
		// So, if `response.is_positive()` (which is a condition for
		// returning `Ok` from the `command()` method above), then delivery
		// succeeds, accordingly to RFC 5321.
		Ok(_) => Ok(Deliverability {
			has_full_inbox: false,
			is_deliverable: true, // response.is_positive()
			is_disabled: false,
		}),
		Err(err) => {
			// We cast to lowercase, because the parser's matched strings are
			// all lowercase.
			let err_string = err.to_string().to_lowercase();

			match parser::classify_rcpt_rejection(&err_string, to_email) {
				// Return the error as-is so that the check is retried, and
				// eventually reported as unknown.
				RcptRejection::Inconclusive => Err(SmtpError::AsyncSmtpError(err)),
				RcptRejection::Invalid => Ok(Deliverability {
					has_full_inbox: false,
					is_deliverable: false,
					is_disabled: false,
				}),
				RcptRejection::Disabled => Ok(Deliverability {
					has_full_inbox: false,
					is_deliverable: false,
					is_disabled: true,
				}),
				RcptRejection::FullInbox => Ok(Deliverability {
					has_full_inbox: true,
					is_deliverable: false,
					is_disabled: false,
				}),
				RcptRejection::Deliverable => Ok(Deliverability {
					has_full_inbox: false,
					is_deliverable: true,
					is_disabled: false,
				}),
			}
		}
	}
}

/// Checks if the domain has a catch-all email setup.
async fn smtp_is_catch_all<S: AsyncBufRead + AsyncWrite + Unpin + Send>(
	smtp_transport: &mut SmtpTransport<S>,
	domain: &str,
	host: &str,
	to_email: &EmailAddress,
) -> Result<bool, SmtpError> {
	if has_rule(domain, host, &Rule::SkipCatchAll) {
		tracing::debug!(
			target: LOG_TARGET,
			email=to_email.to_string(),
			domain=domain,
			"Skipping catch-all check"
		);
		return Ok(false);
	}

	// Create a random 15-char alphanumerical string.
	let mut rng: SmallRng = rand::make_rng();
	let random_email: String = iter::repeat_with(|| rng.sample(Alphanumeric))
		.map(char::from)
		.take(15)
		.collect();
	let random_email = EmailAddress::new(format!("{random_email}@{domain}"))?;

	check_email_deliverability(smtp_transport, &random_email)
		.await
		.map(|result| result.is_deliverable)
}

/// Creates an SMTP future for email verification.
async fn create_smtp_future(
	to_email: &EmailAddress,
	mx_host: &str,
	domain: &str,
	verif_method: &VerifMethodSmtp,
) -> Result<(bool, Deliverability), SmtpError> {
	// FIXME If the SMTP is not connectable, we should actually return an
	// Ok(SmtpDetails { can_connect_smtp: false, ... }).
	let mut smtp_transport = connect_to_smtp_host(to_email, mx_host, verif_method).await?;

	let is_catch_all = smtp_is_catch_all(&mut smtp_transport, domain, mx_host, to_email)
		.await
		.unwrap_or(false);
	let deliverability = if is_catch_all {
		Deliverability {
			has_full_inbox: false,
			is_deliverable: true,
			is_disabled: false,
		}
	} else {
		let mut result = check_email_deliverability(&mut smtp_transport, to_email).await;

		// Some SMTP servers automatically close the connection after an error,
		// so we should reconnect to perform a next command.
		//
		// Unfortunately `smtp_transport.is_connected()` doesn't report about this,
		// so we can only check for "io: incomplete" SMTP error being returned.
		// https://github.com/async-email/async-smtp/issues/37
		if let Err(e) = &result {
			if parser::is_err_io_errors(e) {
				tracing::debug!(
					target: LOG_TARGET,
					email=to_email.to_string(),
					error=?e,
					"Got `io: incomplete` error, reconnecting"
				);

				let _ = smtp_transport.quit().await;
				smtp_transport = connect_to_smtp_host(to_email, mx_host, verif_method).await?;
				result = check_email_deliverability(&mut smtp_transport, to_email).await;
			}
		}

		result?
	};

	// The verdict is already known. Servers that hang up right after a 550
	// make QUIT fail, and that must not turn a definitive answer into an error.
	if let Err(error) = smtp_transport.quit().await {
		tracing::debug!(
			target: LOG_TARGET,
			email=to_email.to_string(),
			error=?error,
			"QUIT failed after the deliverability check, keeping the result"
		);
	}

	Ok((is_catch_all, deliverability))
}

/// Get all email details we can from one single `EmailAddress`, without
/// retries.
async fn check_smtp_without_retry(
	to_email: &EmailAddress,
	mx_host: &str,
	domain: &str,
	verif_method: &VerifMethodSmtp,
) -> Result<SmtpDetails, SmtpError> {
	let fut = create_smtp_future(to_email, mx_host, domain, verif_method);

	let (is_catch_all, deliverability) = match verif_method.config.smtp_timeout {
		Some(smtp_timeout) => {
			let timeout = tokio::time::timeout(smtp_timeout, fut);

			match timeout.await {
				Ok(result) => result?,
				Err(_) => return Err(SmtpError::Timeout(smtp_timeout)),
			}
		}
		None => fut.await?,
	};

	Ok(SmtpDetails {
		can_connect_smtp: true,
		has_full_inbox: deliverability.has_full_inbox,
		is_catch_all,
		is_deliverable: deliverability.is_deliverable,
		is_disabled: deliverability.is_disabled,
	})
}

/// Get all email details we can from one single `EmailAddress`.
/// Retry the SMTP connection on error, in particular to avoid greylisting.
#[async_recursion]
pub async fn check_smtp_with_retry(
	to_email: &EmailAddress,
	mx_host: &str,
	domain: &str,
	verif_method: &VerifMethodSmtp,
	// Number of remaining retries.
	count: usize,
) -> Result<SmtpDetails, SmtpError> {
	tracing::debug!(
		target: LOG_TARGET,
		email=to_email.to_string(),
		attempt=verif_method.config.retries - count + 1,
		mx_host=mx_host,
		port=verif_method.config.smtp_port,
		using_proxy=verif_method.proxy.is_some(),
		"Check SMTP"
	);

	let result = check_smtp_without_retry(to_email, mx_host, domain, verif_method).await;

	tracing::debug!(
		target: LOG_TARGET,
		email=to_email.to_string(),
		attempt=verif_method.config.retries - count + 1,
		mx_host=mx_host,
		port=verif_method.config.smtp_port,
		result=?result,
		"Got SMTP check result"
	);

	match &result {
		// Don't retry if we used Hotmail or Yahoo API. This two options should
		// be non-callable, as this function only deals with actual SMTP
		// connection errors.
		Err(SmtpError::HeadlessError(_)) => result,
		Err(SmtpError::YahooError(_)) => result,
		Err(SmtpError::GmailError(_)) => result,
		// Retries exist to get past greylisting. A timed-out attempt already
		// used its whole budget, and an immediate retry only doubles the wait.
		Err(SmtpError::Timeout(_)) => result,
		// Only retry if the SMTP error was unknown.
		Err(err) if err.get_description().is_none() => {
			if count <= 1 {
				result
			} else {
				tracing::debug!(
					target: LOG_TARGET,
					email=to_email.to_string(),
					"Potential greylisting detected, retrying"
				);
				check_smtp_with_retry(to_email, mx_host, domain, verif_method, count - 1).await
			}
		}
		_ => result,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use fast_socks5::server::{
		run_tcp_proxy, DnsResolveHelper as _, Socks5ServerProtocol, SocksServerError,
	};
	use fast_socks5::{ReplyError, SocksError};
	use std::time::Duration;
	use tokio::io::AsyncWriteExt;
	use tokio::net::TcpListener;

	const TIMEOUT: Duration = Duration::from_secs(5);

	async fn listener() -> (TcpListener, u16) {
		let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
		let port = listener.local_addr().unwrap().port();
		(listener, port)
	}

	/// A mail server that greets every connection.
	async fn greeting_server() -> u16 {
		let (listener, port) = listener().await;
		tokio::spawn(async move {
			while let Ok((mut socket, _)) = listener.accept().await {
				let _ = socket.write_all(b"220 mx.test ESMTP\r\n").await;
			}
		});
		port
	}

	/// A port that refuses connections, like a blocked port 25.
	async fn closed_port() -> u16 {
		listener().await.1
	}

	async fn serve_socks5(socket: TcpStream) -> std::result::Result<(), SocksServerError> {
		let (proto, _) = Socks5ServerProtocol::accept_password_auth(socket, |user, pass| {
			user == "u" && pass == "p"
		})
		.await?;
		let (proto, _, target) = proto.read_command().await?.resolve_dns().await?;
		run_tcp_proxy(proto, &target, TIMEOUT, false).await?;
		Ok(())
	}

	/// A SOCKS5 proxy that only accepts user "u" with password "p".
	async fn socks5_proxy(password: &str) -> CheckEmailInputProxy {
		let (listener, port) = listener().await;
		tokio::spawn(async move {
			while let Ok((socket, _)) = listener.accept().await {
				tokio::spawn(serve_socks5(socket));
			}
		});
		CheckEmailInputProxy {
			host: "127.0.0.1".into(),
			port,
			username: Some("u".into()),
			password: Some(password.into()),
			timeout_ms: None,
		}
	}

	#[tokio::test]
	async fn probe_reads_the_greeting_directly_and_through_a_proxy() {
		let mx_port = greeting_server().await;
		let proxy = socks5_proxy("p").await;

		for proxy in [None, Some(&proxy)] {
			let greeting = probe_smtp_greeting("127.0.0.1.", mx_port, proxy, TIMEOUT).await;
			assert_eq!(
				greeting.map_err(|e| e.to_string()),
				Ok("220 mx.test ESMTP".to_string()),
				"proxy: {:?}",
				proxy.map(|p| p.port)
			);
		}
	}

	#[tokio::test]
	async fn probe_fails_with_the_reason_the_path_is_broken() {
		let mx_port = greeting_server().await;
		let blocked_port = closed_port().await;
		let good_proxy = socks5_proxy("p").await;
		let bad_password = socks5_proxy("wrong").await;
		let unreachable_proxy = CheckEmailInputProxy {
			port: closed_port().await,
			..good_proxy.clone()
		};

		// Each cause surfaces as a distinct error, so operators can be told
		// which one to fix.
		let cases: [(_, _, _, fn(&SocksError) -> bool); 3] = [
			(&bad_password, mx_port, "rejected credentials", |e| {
				matches!(e, SocksError::AuthenticationRejected(_))
			}),
			(
				&good_proxy,
				blocked_port,
				"proxy cannot reach the MX",
				|e| matches!(e, SocksError::ReplyError(ReplyError::ConnectionRefused)),
			),
			(&unreachable_proxy, mx_port, "proxy unreachable", |e| {
				matches!(e, SocksError::ConnectError(_))
			}),
		];
		for (proxy, port, case, expected) in cases {
			let result = probe_smtp_greeting("127.0.0.1", port, Some(proxy), TIMEOUT).await;
			assert!(
				matches!(&result, Err(SmtpError::Socks5(e)) if expected(e)),
				"{}: {:?}",
				case,
				result
			);
		}

		// A server that accepts but never greets.
		let (silent, silent_port) = listener().await;
		let _hold = tokio::spawn(async move {
			let accepted = silent.accept().await;
			tokio::time::sleep(TIMEOUT).await;
			drop(accepted);
		});
		let short = Duration::from_millis(200);
		let result = probe_smtp_greeting("127.0.0.1", silent_port, None, short).await;
		assert!(
			matches!(result, Err(SmtpError::Timeout(d)) if d == short),
			"{:?}",
			result
		);
	}
}
