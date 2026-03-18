use std::time::Duration;

use check_if_email_exists::smtp::verif_method::{
	HotmailB2CVerifMethod, VerifMethodSmtpConfig, YahooVerifMethod, DEFAULT_PROXY_ID,
};
use serde::{Deserialize, Serialize};

/// These types are for backward compatibility. The previous API allowed for
/// - yahoo_verif_method: Smtp, Headless, Api
/// - hotmailb2c_verif_method: Smtp, Headless
/// - hotmailb2b_verif_method: Smtp
/// - gmail_verif_method: Smtp
///
/// We keep these types for backward compatibility.

#[derive(Debug, Default, Deserialize, Serialize)]
pub enum BackwardCompatYahooVerifMethod {
	Api,
	#[default]
	Headless,
	Smtp,
}

impl BackwardCompatYahooVerifMethod {
	pub fn to_yahoo_verif_method(
		&self,
		// If set, this will use the "default" proxy configuration.
		use_default_proxy: bool,
		hello_name: String,
		from_email: String,
		smtp_timeout: Option<Duration>,
		smtp_port: u16,
		retries: usize,
	) -> YahooVerifMethod {
		match self {
			BackwardCompatYahooVerifMethod::Api => YahooVerifMethod::Api,
			BackwardCompatYahooVerifMethod::Headless => YahooVerifMethod::Headless,
			BackwardCompatYahooVerifMethod::Smtp => YahooVerifMethod::Smtp(VerifMethodSmtpConfig {
				from_email,
				hello_name,
				smtp_port,
				smtp_timeout,
				proxy: if use_default_proxy {
					Some(DEFAULT_PROXY_ID.to_string())
				} else {
					None
				},
				retries,
			}),
		}
	}
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub enum BackwardCompatHotmailB2CVerifMethod {
	#[default]
	Headless,
	Smtp,
}

impl BackwardCompatHotmailB2CVerifMethod {
	pub fn to_hotmailb2c_verif_method(
		&self,
		// If set, this will use the "default" proxy configuration.
		use_default_proxy: bool,
		hello_name: String,
		from_email: String,
		smtp_timeout: Option<Duration>,
		smtp_port: u16,
		retries: usize,
	) -> HotmailB2CVerifMethod {
		match self {
			BackwardCompatHotmailB2CVerifMethod::Headless => HotmailB2CVerifMethod::Headless,
			BackwardCompatHotmailB2CVerifMethod::Smtp => {
				HotmailB2CVerifMethod::Smtp(VerifMethodSmtpConfig {
					from_email,
					hello_name,
					smtp_port,
					smtp_timeout,
					proxy: if use_default_proxy {
						Some(DEFAULT_PROXY_ID.to_string())
					} else {
						None
					},
					retries,
				})
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_yahoo_api() {
		let m = BackwardCompatYahooVerifMethod::Api;
		let result = m.to_yahoo_verif_method(false, "h".into(), "f".into(), None, 25, 1);
		assert!(matches!(result, YahooVerifMethod::Api));
	}

	#[test]
	fn test_yahoo_headless() {
		let m = BackwardCompatYahooVerifMethod::Headless;
		let result = m.to_yahoo_verif_method(false, "h".into(), "f".into(), None, 25, 1);
		assert!(matches!(result, YahooVerifMethod::Headless));
	}

	#[test]
	fn test_yahoo_smtp_with_proxy() {
		let m = BackwardCompatYahooVerifMethod::Smtp;
		let result = m.to_yahoo_verif_method(
			true,
			"hello".into(),
			"from@e.com".into(),
			Some(Duration::from_secs(30)),
			465,
			3,
		);
		match result {
			YahooVerifMethod::Smtp(c) => {
				assert_eq!(c.from_email, "from@e.com");
				assert_eq!(c.hello_name, "hello");
				assert_eq!(c.smtp_port, 465);
				assert_eq!(c.retries, 3);
				assert!(c.proxy.is_some());
			}
			_ => panic!("Expected Smtp"),
		}
	}

	#[test]
	fn test_yahoo_smtp_no_proxy() {
		let m = BackwardCompatYahooVerifMethod::Smtp;
		let result = m.to_yahoo_verif_method(false, "h".into(), "f".into(), None, 25, 1);
		match result {
			YahooVerifMethod::Smtp(c) => assert!(c.proxy.is_none()),
			_ => panic!("Expected Smtp"),
		}
	}

	#[test]
	fn test_hotmail_headless() {
		let m = BackwardCompatHotmailB2CVerifMethod::Headless;
		let result = m.to_hotmailb2c_verif_method(false, "h".into(), "f".into(), None, 25, 1);
		assert!(matches!(result, HotmailB2CVerifMethod::Headless));
	}

	#[test]
	fn test_hotmail_smtp_with_proxy() {
		let m = BackwardCompatHotmailB2CVerifMethod::Smtp;
		let result = m.to_hotmailb2c_verif_method(true, "h".into(), "f@e.com".into(), None, 587, 2);
		match result {
			HotmailB2CVerifMethod::Smtp(c) => {
				assert_eq!(c.smtp_port, 587);
				assert!(c.proxy.is_some());
			}
			_ => panic!("Expected Smtp"),
		}
	}

	#[test]
	fn test_serde_roundtrip() {
		let json = serde_json::to_string(&BackwardCompatYahooVerifMethod::Api).unwrap();
		let _: BackwardCompatYahooVerifMethod = serde_json::from_str(&json).unwrap();

		let json = serde_json::to_string(&BackwardCompatHotmailB2CVerifMethod::Smtp).unwrap();
		let _: BackwardCompatHotmailB2CVerifMethod = serde_json::from_str(&json).unwrap();
	}
}
