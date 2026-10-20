mod normalize;

use crate::EmailAddress;
use levenshtein::levenshtein;
use normalize::normalize_email;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Syntax information after parsing an email address
#[derive(Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct SyntaxDetails {
	/// The email address as a async_smtp `EmailAddress`. It will be `None` if
	/// the email address is ill-formed.
	pub address: Option<EmailAddress>,
	/// The domain name, after "@". It will be the empty string if the email
	/// address if ill-formed.
	pub domain: String,
	/// Does the email have a valid syntax?
	pub is_valid_syntax: bool,
	/// The username, before "@". It will be the empty string if the email
	/// address if ill-formed.
	pub username: String,
	/// The normalized form of `address`. It will be `None` if the email
	/// address is ill-formed.
	pub normalized_email: Option<String>,
	pub suggestion: Option<String>,
}

impl Default for SyntaxDetails {
	fn default() -> Self {
		SyntaxDetails {
			address: None,
			domain: "".into(),
			is_valid_syntax: false,
			username: "".into(),
			normalized_email: None,
			suggestion: None,
		}
	}
}

/// From an `email_address` string, compute syntax information about it, such as
/// username and domain.
pub fn check_syntax(email_address: &str) -> SyntaxDetails {
	// This is the format check that mailchecker::is_valid runs. is_valid also
	// fails disposable domains, which made every disposable or relay address
	// (duck.com, simplelogin.com...) "invalid syntax" and left
	// misc.is_disposable always false.
	let email_address = match EmailAddress::from_str(email_address) {
		Ok(m) if fast_chemail::is_valid_email(email_address) => m,
		_ => return SyntaxDetails::default(),
	};

	let iter: &str = email_address.as_ref();
	let mut iter = iter.split('@');
	let username: String = iter
		.next()
		.expect("We checked above that email is valid. qed.")
		.into();
	// Domain names are case-insensitive, so lowercase the domain for the
	// lookups keyed on it (domain rules, provider detection). The local part
	// keeps its case: RFC 5321 lets the receiving server treat it as
	// case-sensitive.
	let domain: String = iter
		.next()
		.expect("We checked above that email is valid. qed.")
		.to_ascii_lowercase();
	let normalized_email = normalize_email(&username, &domain);

	SyntaxDetails {
		address: Some(email_address),
		domain,
		is_valid_syntax: true,
		username,
		normalized_email: Some(normalized_email),
		suggestion: None,
	}
}

const MAIL_PROVIDERS: &[&str] = &[
	"gmail.com",
	"yahoo.com",
	"outlook.com",
	"hotmail.com",
	"protonmail.com",
	"icloud.com",
	"yandex.com",
];
/// Real mailbox domains within two edits of a provider above. Without this
/// list, mail.com and ymail.com would be "corrected" to gmail.com.
const REAL_DOMAINS_NEAR_PROVIDERS: &[&str] = &[
	"ymail.com",
	"mail.com",
	"email.com",
	"yahoo.ca",
	"hotmail.ca",
	"protonmail.ch",
];

// Supplies the syntax parameter with a suggestion that matches the mail domain best by levenshtein
// distance.
pub fn get_similar_mail_provider(syntax: &mut SyntaxDetails) {
	let domain = syntax.domain.to_ascii_lowercase();
	if MAIL_PROVIDERS.contains(&domain.as_str())
		|| REAL_DOMAINS_NEAR_PROVIDERS.contains(&domain.as_str())
	{
		return;
	}

	let closest = MAIL_PROVIDERS
		.iter()
		.map(|provider| (levenshtein(&domain, provider), provider))
		.filter(|(distance, _)| *distance < 3)
		.min_by_key(|(distance, _)| *distance);

	if let Some((_, provider)) = closest {
		syntax.suggestion = Some(format!("{}@{}", syntax.username, provider));
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn should_return_invalid_for_invalid_email() {
		assert_eq!(
			check_syntax("foo"),
			SyntaxDetails {
				address: None,
				domain: "".into(),
				is_valid_syntax: false,
				username: "".into(),
				normalized_email: None,
				suggestion: None,
			}
		);
	}

	#[test]
	fn should_return_invalid_for_invalid_email_with_at() {
		assert_eq!(
			check_syntax("foo@bar"),
			SyntaxDetails {
				address: None,
				domain: "".into(),
				is_valid_syntax: false,
				username: "".into(),
				normalized_email: None,
				suggestion: None,
			}
		);
	}

	#[test]
	fn should_work_for_valid_email() {
		assert_eq!(
			check_syntax("foo@bar.com"),
			SyntaxDetails {
				address: Some(EmailAddress::new("foo@bar.com".into()).unwrap()),
				domain: "bar.com".into(),
				is_valid_syntax: true,
				username: "foo".into(),
				normalized_email: Some("foo@bar.com".into()),
				suggestion: None,
			}
		);
	}

	#[test]
	fn should_accept_disposable_and_relay_domains() {
		// Disposable domains are reported by misc.is_disposable. They say
		// nothing about whether the address is well-formed.
		for email in [
			"foo@mailinator.com",
			"foo@duck.com",
			"foo@simplelogin.com",
			"foo@anonaddy.com",
			"foo@addy.io",
			"foo@mozmail.com",
		] {
			assert!(check_syntax(email).is_valid_syntax, "{}", email);
		}

		assert!(!check_syntax("foo,bar@mailinator.com").is_valid_syntax);
	}

	#[test]
	fn should_lowercase_the_domain_but_not_the_local_part() {
		let syntax = check_syntax("John.Smith@Hotmail.COM");
		assert_eq!(syntax.domain, "hotmail.com");
		assert_eq!(syntax.username, "John.Smith");
		assert_eq!(
			syntax.normalized_email,
			Some("John.Smith@hotmail.com".into())
		);
	}

	fn suggestion_for(email: &str) -> Option<String> {
		let (username, domain) = email.split_once('@').expect("test email has an @");
		let mut syntax = SyntaxDetails {
			domain: domain.into(),
			is_valid_syntax: true,
			username: username.into(),
			..Default::default()
		};
		get_similar_mail_provider(&mut syntax);
		syntax.suggestion
	}

	#[test]
	fn should_suggest_a_correct_mail_if_similar() {
		for (typo, fixed) in [
			("test@gmali.com", "test@gmail.com"),
			("test@gmail.co", "test@gmail.com"),
			("test@gmail.cm", "test@gmail.com"),
			("test@yaho.com", "test@yahoo.com"),
			("test@hotmial.com", "test@hotmail.com"),
			("test@outlok.com", "test@outlook.com"),
			("test@icloud.co", "test@icloud.com"),
		] {
			assert_eq!(suggestion_for(typo), Some(fixed.to_string()), "{}", typo);
		}
	}

	#[test]
	fn should_not_suggest_for_real_mail_domains() {
		// Every provider itself, real domains one or two edits away from a
		// provider, and country variants of a provider are not typos.
		for email in [
			"test@gmail.com",
			"test@Gmail.com",
			"test@yahoo.com",
			"test@outlook.com",
			"test@hotmail.com",
			"test@protonmail.com",
			"test@icloud.com",
			"test@yandex.com",
			"test@ymail.com",
			"test@mail.com",
			"test@email.com",
			"test@yahoo.ca",
			"test@hotmail.ca",
			"test@protonmail.ch",
			"test@company.com",
		] {
			assert_eq!(suggestion_for(email), None, "{}", email);
		}
	}
}
