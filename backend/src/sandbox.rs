use check_if_email_exists::{
	misc::MiscDetails, mx::MxDetails, smtp::SmtpDetails, syntax::SyntaxDetails, CheckEmailOutput,
	Reachable,
};

/// Generates a deterministic mock CheckEmailOutput based on the email address.
/// The domain controls the scenario:
/// - `*@valid.example.com` → safe, deliverable
/// - `*@risky.example.com` → risky, catch-all
/// - `*@invalid.example.com` → invalid, undeliverable
/// - `*@unknown.example.com` → unknown, smtp error
/// - `*@disposable.example.com` → risky, disposable
/// - Any other domain → safe, deliverable (default)
pub fn sandbox_check(email: &str) -> CheckEmailOutput {
	let domain = email.split('@').nth(1).unwrap_or("example.com");
	let username = email.split('@').next().unwrap_or("user");

	let syntax = SyntaxDetails {
		address: None,
		domain: domain.to_string(),
		is_valid_syntax: true,
		username: username.to_string(),
		normalized_email: Some(email.to_string()),
		suggestion: None,
	};

	match domain {
		"invalid.example.com" => CheckEmailOutput {
			input: email.to_string(),
			is_reachable: Reachable::Invalid,
			misc: Ok(MiscDetails::default()),
			mx: Ok(MxDetails::default()),
			smtp: Ok(SmtpDetails {
				can_connect_smtp: true,
				has_full_inbox: false,
				is_catch_all: false,
				is_deliverable: false,
				is_disabled: false,
			}),
			syntax,
			debug: Default::default(),
		},
		"risky.example.com" => CheckEmailOutput {
			input: email.to_string(),
			is_reachable: Reachable::Risky,
			misc: Ok(MiscDetails::default()),
			mx: Ok(MxDetails::default()),
			smtp: Ok(SmtpDetails {
				can_connect_smtp: true,
				has_full_inbox: false,
				is_catch_all: true,
				is_deliverable: true,
				is_disabled: false,
			}),
			syntax,
			debug: Default::default(),
		},
		"unknown.example.com" => CheckEmailOutput {
			input: email.to_string(),
			is_reachable: Reachable::Unknown,
			misc: Ok(MiscDetails::default()),
			mx: Ok(MxDetails::default()),
			smtp: Err(check_if_email_exists::smtp::SmtpError::from(
				std::io::Error::other("sandbox: simulated SMTP timeout"),
			)),
			syntax,
			debug: Default::default(),
		},
		"disposable.example.com" => CheckEmailOutput {
			input: email.to_string(),
			is_reachable: Reachable::Risky,
			misc: Ok(MiscDetails {
				is_disposable: true,
				..Default::default()
			}),
			mx: Ok(MxDetails::default()),
			smtp: Ok(SmtpDetails {
				can_connect_smtp: true,
				has_full_inbox: false,
				is_catch_all: false,
				is_deliverable: true,
				is_disabled: false,
			}),
			syntax,
			debug: Default::default(),
		},
		_ => CheckEmailOutput {
			input: email.to_string(),
			is_reachable: Reachable::Safe,
			misc: Ok(MiscDetails::default()),
			mx: Ok(MxDetails::default()),
			smtp: Ok(SmtpDetails {
				can_connect_smtp: true,
				has_full_inbox: false,
				is_catch_all: false,
				is_deliverable: true,
				is_disabled: false,
			}),
			syntax,
			debug: Default::default(),
		},
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sandbox_valid_domain() {
		let result = sandbox_check("test@valid.example.com");
		assert_eq!(result.is_reachable, Reachable::Safe);
	}

	#[test]
	fn sandbox_invalid_domain() {
		let result = sandbox_check("test@invalid.example.com");
		assert_eq!(result.is_reachable, Reachable::Invalid);
		assert_eq!(result.smtp.as_ref().unwrap().is_deliverable, false);
	}

	#[test]
	fn sandbox_risky_domain() {
		let result = sandbox_check("test@risky.example.com");
		assert_eq!(result.is_reachable, Reachable::Risky);
		assert!(result.smtp.as_ref().unwrap().is_catch_all);
	}

	#[test]
	fn sandbox_unknown_domain() {
		let result = sandbox_check("test@unknown.example.com");
		assert_eq!(result.is_reachable, Reachable::Unknown);
		assert!(result.smtp.is_err());
	}

	#[test]
	fn sandbox_disposable_domain() {
		let result = sandbox_check("test@disposable.example.com");
		assert!(result.misc.as_ref().unwrap().is_disposable);
	}

	#[test]
	fn sandbox_default_domain() {
		let result = sandbox_check("test@anything.com");
		assert_eq!(result.is_reachable, Reachable::Safe);
	}
}
