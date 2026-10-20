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

fn close_smtp<S: AsyncBufRead + AsyncWrite + Unpin + Send + 'static>(
	mut transport: SmtpTransport<S>,
) {
	// Cleanup must never consume the recipient verdict's deadline.
	tokio::spawn(async move {
		let cleanup =
			tokio::time::timeout(std::time::Duration::from_millis(250), transport.quit()).await;
		if !matches!(cleanup, Ok(Ok(_))) {
			tracing::debug!(target: LOG_TARGET, result=?cleanup, "SMTP cleanup failed after recipient evidence was finalized");
		}
	});
}

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
			close_smtp($client);

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
	allow_private_test_server: bool,
) -> Result<BufStream<Box<dyn AsyncReadWrite>>, SmtpError> {
	// hostname verification fails if it ends with '.', for example, using
	// SOCKS5 proxies we can `io: incomplete` error.
	let clean_host = host.trim_end_matches('.').to_string();
	let resolve = |host: String, port| async move {
		#[cfg(test)]
		if allow_private_test_server {
			return Ok(tokio::net::lookup_host((host.as_str(), port))
				.await?
				.collect::<Vec<_>>());
		}
		let _ = allow_private_test_server;
		crate::network::resolve_public_addresses(&host, port).await
	};
	let targets = resolve(clean_host, port).await?;
	let stream: Box<dyn AsyncReadWrite> = match proxy {
		Some(proxy) => {
			let proxies = resolve(proxy.host.clone(), proxy.port).await?;
			// A literal IP prevents the proxy from performing an unchecked second DNS lookup.
			let target_ip = targets[0].ip().to_string();
			// Bounded like the direct connect below: without a timeout an
			// unreachable proxy held the check until the overall SMTP timeout
			// and was then reported as a slow mail server.
			let mut config = Config::default();
			config.set_connect_timeout(
				proxy
					.timeout_ms
					.map(std::time::Duration::from_millis)
					.unwrap_or(SMTP_CONNECT_TIMEOUT),
			);

			let socks_stream =
				if let (Some(username), Some(password)) = (&proxy.username, &proxy.password) {
					Socks5Stream::connect_with_password(
						proxies[0],
						target_ip,
						port,
						username.clone(),
						password.clone(),
						config,
					)
					.await?
				} else {
					Socks5Stream::connect(proxies[0], target_ip, port, config).await?
				};
			Box::new(socks_stream)
		}
		None => {
			// A firewall that drops outbound SYNs (cloud hosts often block
			// port 25) otherwise holds this worker slot for the kernel's ~2
			// minute connect timeout. Failing here as an I/O error also tells
			// callers that no SMTP conversation took place.
			let tcp_stream =
				tokio::time::timeout(SMTP_CONNECT_TIMEOUT, TcpStream::connect(targets.as_slice()))
					.await
					.map_err(|_| {
						std::io::Error::new(
							std::io::ErrorKind::TimedOut,
							"SMTP connection timed out",
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
	probe_smtp_greeting_inner(host, port, proxy, timeout, false).await
}

async fn probe_smtp_greeting_inner(
	host: &str,
	port: u16,
	proxy: Option<&CheckEmailInputProxy>,
	timeout: std::time::Duration,
	allow_private_test_server: bool,
) -> Result<String, SmtpError> {
	let probe = async {
		let mut stream = open_stream(host, port, proxy, allow_private_test_server).await?;
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

	let allow_private_test_server = false;
	#[cfg(test)]
	let allow_private_test_server =
		allow_private_test_server || verif_method.config.allow_private_test_server;
	let stream = open_stream(
		mx_host,
		verif_method.config.smtp_port,
		verif_method.proxy.as_ref(),
		allow_private_test_server,
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

	let is_catch_all = smtp_is_catch_all(&mut smtp_transport, domain, mx_host, to_email).await;
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

			close_smtp(smtp_transport);
			smtp_transport = connect_to_smtp_host(to_email, mx_host, verif_method).await?;
			result = check_email_deliverability(&mut smtp_transport, to_email).await;
		}
	}

	close_smtp(smtp_transport);
	let deliverability = result?;

	// A failed catch-all probe is unknown, never evidence that catch-all is false.
	Ok((is_catch_all?, deliverability))
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

	#[tokio::test]
	async fn recipient_evidence_survives_catch_all_and_cleanup_edge_cases() {
		use crate::smtp::verif_method::VerifMethodSmtpConfig;
		use std::sync::{
			atomic::{AtomicBool, Ordering},
			Arc,
		};
		use tokio::io::BufReader;

		for (random_reply, target_reply, hang_quit, expected) in [
			("451 4.7.1 Greylisted\r\n", "250 OK\r\n", false, None),
			(
				"250 OK\r\n",
				"550 5.1.1 User unknown\r\n",
				false,
				Some(false),
			),
			(
				"550 5.1.1 User unknown\r\n",
				"550 5.7.1 Recipient address rejected: Authentication required\r\n",
				false,
				None,
			),
			("550 5.1.1 User unknown\r\n", "250 OK\r\n", true, Some(true)),
			(
				"550 5.1.1 User unknown\r\n",
				"550 5.1.1 User unknown\r\n",
				true,
				Some(false),
			),
		] {
			let (listener, port) = listener().await;
			let target_seen = Arc::new(AtomicBool::new(false));
			let seen = target_seen.clone();
			let server = tokio::spawn(async move {
				let (socket, _) = listener.accept().await.unwrap();
				let (reader, mut writer) = socket.into_split();
				writer
					.write_all(b"220 audit.example ESMTP\r\n")
					.await
					.unwrap();
				let mut lines = BufReader::new(reader).lines();
				while let Some(line) = lines.next_line().await.unwrap() {
					let reply = if line.starts_with("EHLO") || line.starts_with("MAIL") {
						"250 OK\r\n"
					} else if line.starts_with("RCPT") {
						if line.contains("target@audit.example") {
							seen.store(true, Ordering::SeqCst);
							target_reply
						} else {
							random_reply
						}
					} else if line.starts_with("QUIT") {
						if hang_quit {
							tokio::time::sleep(Duration::from_secs(2)).await;
						}
						"221 Bye\r\n"
					} else {
						"500 unexpected\r\n"
					};
					if writer.write_all(reply.as_bytes()).await.is_err() || line.starts_with("QUIT")
					{
						break;
					}
				}
			});
			let method = VerifMethodSmtp::new(
				VerifMethodSmtpConfig {
					allow_private_test_server: true,
					hello_name: "audit.example".into(),
					from_email: "sender@audit.example".into(),
					smtp_port: port,
					smtp_timeout: Some(Duration::from_millis(300)),
					retries: 1,
					..Default::default()
				},
				None,
			);
			let email = EmailAddress::new("target@audit.example".into()).unwrap();
			let result =
				check_smtp_without_retry(&email, "127.0.0.1", "audit.example", &method).await;
			assert!(
				target_seen.load(Ordering::SeqCst),
				"target recipient was skipped"
			);
			match expected {
				None => assert!(
					matches!(result, Err(SmtpError::AsyncSmtpError(_))),
					"{:?}",
					result
				),
				Some(deliverable) => assert_eq!(result.unwrap().is_deliverable, deliverable),
			}
			server.abort();
		}
	}

	#[tokio::test]
	async fn smtp_preserves_case_sensitive_recipient_spelling() {
		use crate::smtp::verif_method::VerifMethodSmtpConfig;
		use tokio::io::BufReader;
		for (address, expected) in [
			("Sales@audit.example", true),
			("sales@audit.example", false),
		] {
			let (listener, port) = listener().await;
			let server = tokio::spawn(async move {
				let (socket, _) = listener.accept().await.unwrap();
				let (reader, mut writer) = socket.into_split();
				writer
					.write_all(b"220 audit.example ESMTP\r\n")
					.await
					.unwrap();
				let mut lines = BufReader::new(reader).lines();
				while let Some(line) = lines.next_line().await.unwrap() {
					let reply = if line.starts_with("EHLO") || line.starts_with("MAIL") {
						"250 OK\r\n"
					} else if line.starts_with("RCPT") {
						if line.contains("<Sales@audit.example>") {
							"250 OK\r\n"
						} else {
							"550 5.1.1 User unknown\r\n"
						}
					} else if line.starts_with("QUIT") {
						"221 Bye\r\n"
					} else {
						"500 unexpected\r\n"
					};
					if writer.write_all(reply.as_bytes()).await.is_err() || line.starts_with("QUIT")
					{
						break;
					}
				}
			});
			let method = VerifMethodSmtp::new(
				VerifMethodSmtpConfig {
					allow_private_test_server: true,
					hello_name: "audit.example".into(),
					from_email: "sender@audit.example".into(),
					smtp_port: port,
					smtp_timeout: Some(TIMEOUT),
					retries: 1,
					..Default::default()
				},
				None,
			);
			let email = EmailAddress::new(address.into()).unwrap();
			let result = check_smtp_without_retry(&email, "127.0.0.1", "audit.example", &method)
				.await
				.unwrap();
			assert_eq!(result.is_deliverable, expected, "{address}");
			server.abort();
		}
	}

	#[tokio::test]
	async fn mx_failover_reaches_backup_and_stops_at_recipient_rejection() {
		use crate::smtp::{check_smtp_mx, verif_method::VerifMethod, SmtpDebugVerifMethod};
		use crate::CheckEmailInput;
		use hickory_proto::rr::{rdata::MX, Name};
		use tokio::io::BufReader;

		let (backup_listener, port) = listener().await;
		let server = tokio::spawn(async move {
			let (socket, _) = backup_listener.accept().await.unwrap();
			let (reader, mut writer) = socket.into_split();
			writer
				.write_all(b"220 backup.example ESMTP\r\n")
				.await
				.unwrap();
			let mut lines = BufReader::new(reader).lines();
			while let Some(line) = lines.next_line().await.unwrap() {
				let reply = if line.starts_with("RCPT") {
					"550 5.1.1 User unknown\r\n"
				} else if line.starts_with("QUIT") {
					"221 Bye\r\n"
				} else {
					"250 OK\r\n"
				};
				if writer.write_all(reply.as_bytes()).await.is_err() || line.starts_with("QUIT") {
					break;
				}
			}
		});
		let mut input = CheckEmailInput {
			verif_method: VerifMethod::new_with_same_config_for_all(
				None,
				"audit.example".into(),
				"sender@audit.example".into(),
				port,
				Some(Duration::from_millis(600)),
				1,
			),
			..Default::default()
		};
		let crate::smtp::verif_method::EverythingElseVerifMethod::Smtp(smtp) =
			&mut input.verif_method.everything_else;
		smtp.allow_private_test_server = true;
		let primary = MX::new(10, Name::from_ascii("127.0.0.2.").unwrap());
		let backup = MX::new(20, Name::from_ascii("127.0.0.1.").unwrap());
		let unused = MX::new(30, Name::from_ascii("127.0.0.3.").unwrap());
		let email = EmailAddress::new("target@audit.example".into()).unwrap();
		let (result, debug) = check_smtp_mx(
			&email,
			&[&unused, &backup, &primary],
			"audit.example",
			&input,
		)
		.await;
		assert!(!result.unwrap().is_deliverable);
		assert!(
			matches!(debug.verif_method, SmtpDebugVerifMethod::Smtp(c) if c.host == "127.0.0.1.")
		);
		server.abort();

		// A silent MX gets only its share of the total budget.
		let (silent, silent_port) = listener().await;
		let hold = tokio::spawn(async move {
			let _connection = silent.accept().await.unwrap();
			tokio::time::sleep(Duration::from_secs(2)).await;
		});
		let mut silent_input = CheckEmailInput {
			verif_method: VerifMethod::new_with_same_config_for_all(
				None,
				"audit.example".into(),
				"sender@audit.example".into(),
				silent_port,
				Some(Duration::from_millis(100)),
				1,
			),
			..Default::default()
		};
		let crate::smtp::verif_method::EverythingElseVerifMethod::Smtp(smtp) =
			&mut silent_input.verif_method.everything_else;
		smtp.allow_private_test_server = true;
		let started = tokio::time::Instant::now();
		let (result, debug) =
			check_smtp_mx(&email, &[&backup, &unused], "audit.example", &silent_input).await;
		assert!(result.is_err());
		assert!(started.elapsed() < Duration::from_millis(250));
		assert!(
			matches!(debug.verif_method, SmtpDebugVerifMethod::Smtp(c) if c.host == "127.0.0.3.")
		);
		hold.abort();
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
		let command = proto.read_command().await?;
		assert!(matches!(
			command.2,
			fast_socks5::util::target_addr::TargetAddr::Ip(_)
		));
		let (proto, _, target) = command.resolve_dns().await?;
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
	async fn untrusted_mx_and_proxy_addresses_never_reach_local_services() {
		let (listener, port) = listener().await;
		let proxy = CheckEmailInputProxy {
			host: "127.0.0.1".into(),
			port,
			..Default::default()
		};
		for (host, proxy) in [
			("127.0.0.1", None),
			("::ffff:127.0.0.1", None),
			("127.0.0.1", Some(&proxy)),
			("8.8.8.8", Some(&proxy)),
		] {
			assert!(matches!(
				probe_smtp_greeting(host, port, proxy, TIMEOUT).await,
				Err(SmtpError::IOError(ref err)) if err.kind() == std::io::ErrorKind::PermissionDenied
			));
		}
		assert!(
			tokio::time::timeout(Duration::from_millis(20), listener.accept())
				.await
				.is_err()
		);
	}

	#[tokio::test]
	async fn probe_reads_the_greeting_directly_and_through_a_proxy() {
		let mx_port = greeting_server().await;
		let proxy = socks5_proxy("p").await;

		for proxy in [None, Some(&proxy)] {
			let greeting =
				probe_smtp_greeting_inner("127.0.0.1.", mx_port, proxy, TIMEOUT, true).await;
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
			let result =
				probe_smtp_greeting_inner("127.0.0.1", port, Some(proxy), TIMEOUT, true).await;
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
		let result = probe_smtp_greeting_inner("127.0.0.1", silent_port, None, short, true).await;
		assert!(
			matches!(result, Err(SmtpError::Timeout(d)) if d == short),
			"{:?}",
			result
		);
	}
}
