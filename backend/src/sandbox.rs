use check_if_email_exists::{
	misc::MiscDetails,
	mx::{MxDetails, MxLookup},
	smtp::SmtpDetails,
	syntax::check_syntax,
	CheckEmailOutput, Reachable,
};
use hickory_resolver::{
	lookup::Lookup,
	proto::op::Query,
	proto::rr::{rdata::MX, Name, RData, RecordType},
};

/// MX details holding one record for `domain`. Tests reuse it to build
/// outputs whose SMTP flags count, since those need an MX host.
pub(crate) fn sandbox_mx(domain: &str) -> MxDetails {
	// The query name is not part of the output, so a name hickory rejects
	// falls back to a fixed one rather than failing the request.
	let domain = Name::from_ascii(domain)
		.or_else(|_| Name::from_ascii("example.com."))
		.expect("fallback sandbox domain is valid");
	let exchange = Name::from_ascii("mail.example.com.").expect("sandbox MX is valid");
	MxDetails::from(MxLookup::from(Lookup::from_rdata(
		Query::query(domain, RecordType::MX),
		RData::MX(MX::new(10, exchange)),
	)))
}

/// Generates a deterministic mock CheckEmailOutput based on the email address.
/// The domain controls the scenario:
/// - `*@valid.example.com` → safe, deliverable
/// - `*@risky.example.com` → risky, catch-all
/// - `*@invalid.example.com` → invalid, undeliverable
/// - `*@unknown.example.com` → unknown, smtp error
/// - `*@disposable.example.com` → risky, disposable
/// - Any other domain → safe, deliverable (default)
pub fn sandbox_check(email: &str) -> CheckEmailOutput {
	// Same syntax gate as a real check, so a malformed address comes back
	// invalid instead of "safe", and never reaches DNS name parsing.
	let syntax = check_syntax(email);
	if !syntax.is_valid_syntax {
		return CheckEmailOutput {
			external_verification: None,
			input: email.to_string(),
			is_reachable: Reachable::Invalid,
			syntax,
			misc: Ok(MiscDetails::default()),
			mx: Ok(MxDetails::default()),
			smtp: Ok(SmtpDetails {
				can_connect_smtp: false,
				has_full_inbox: false,
				is_catch_all: false,
				is_deliverable: false,
				is_disabled: false,
			}),
			provider: None,
			provider_rules_applied: false,
			provider_rejection_reason: None,
			provider_confidence: None,
			debug: Default::default(),
		};
	}

	let domain_lower = syntax.domain.to_lowercase();

	match domain_lower.as_str() {
		"invalid.example.com" => CheckEmailOutput {
			external_verification: None,
			input: email.to_string(),
			is_reachable: Reachable::Invalid,
			misc: Ok(MiscDetails::default()),
			mx: Ok(sandbox_mx(&domain_lower)),
			smtp: Ok(SmtpDetails {
				can_connect_smtp: true,
				has_full_inbox: false,
				is_catch_all: false,
				is_deliverable: false,
				is_disabled: false,
			}),
			syntax,
			provider: None,
			provider_rules_applied: false,
			provider_rejection_reason: None,
			provider_confidence: None,
			debug: Default::default(),
		},
		"risky.example.com" => CheckEmailOutput {
			external_verification: None,
			input: email.to_string(),
			is_reachable: Reachable::Risky,
			misc: Ok(MiscDetails::default()),
			mx: Ok(sandbox_mx(&domain_lower)),
			smtp: Ok(SmtpDetails {
				can_connect_smtp: true,
				has_full_inbox: false,
				is_catch_all: true,
				is_deliverable: true,
				is_disabled: false,
			}),
			syntax,
			provider: None,
			provider_rules_applied: false,
			provider_rejection_reason: None,
			provider_confidence: None,
			debug: Default::default(),
		},
		"unknown.example.com" => CheckEmailOutput {
			external_verification: None,
			input: email.to_string(),
			is_reachable: Reachable::Unknown,
			misc: Ok(MiscDetails::default()),
			mx: Ok(sandbox_mx(&domain_lower)),
			smtp: Err(check_if_email_exists::smtp::SmtpError::from(
				std::io::Error::other("sandbox: simulated SMTP timeout"),
			)),
			syntax,
			provider: None,
			provider_rules_applied: false,
			provider_rejection_reason: None,
			provider_confidence: None,
			debug: Default::default(),
		},
		"disposable.example.com" => CheckEmailOutput {
			external_verification: None,
			input: email.to_string(),
			is_reachable: Reachable::Risky,
			misc: Ok(MiscDetails {
				is_disposable: true,
				..Default::default()
			}),
			mx: Ok(sandbox_mx(&domain_lower)),
			smtp: Ok(SmtpDetails {
				can_connect_smtp: true,
				has_full_inbox: false,
				is_catch_all: false,
				is_deliverable: true,
				is_disabled: false,
			}),
			syntax,
			provider: None,
			provider_rules_applied: false,
			provider_rejection_reason: None,
			provider_confidence: None,
			debug: Default::default(),
		},
		_ => CheckEmailOutput {
			external_verification: None,
			input: email.to_string(),
			is_reachable: Reachable::Safe,
			misc: Ok(MiscDetails::default()),
			mx: Ok(sandbox_mx(&domain_lower)),
			smtp: Ok(SmtpDetails {
				can_connect_smtp: true,
				has_full_inbox: false,
				is_catch_all: false,
				is_deliverable: true,
				is_disabled: false,
			}),
			syntax,
			provider: None,
			provider_rules_applied: false,
			provider_rejection_reason: None,
			provider_confidence: None,
			debug: Default::default(),
		},
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn sandbox_rejects_malformed_addresses_without_panicking() {
		for email in [
			"a@b..com",
			"a@bü.com",
			"a@b c.com",
			"user@",
			"@valid.example.com",
		] {
			let result = sandbox_check(email);
			assert_eq!(result.is_reachable, Reachable::Invalid, "{}", email);
			assert!(!result.syntax.is_valid_syntax, "{}", email);
			assert_eq!(
				crate::scoring::compute_score(&result).category,
				crate::scoring::EmailCategory::Invalid,
				"{}",
				email
			);
		}
	}

	#[test]
	fn sandbox_valid_domain() {
		let result = sandbox_check("test@valid.example.com");
		assert_eq!(result.is_reachable, Reachable::Safe);
		assert!(crate::scoring::compute_score(&result).safe_to_send);
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
