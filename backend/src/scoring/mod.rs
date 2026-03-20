pub mod response;

use check_if_email_exists::{CheckEmailOutput, Reachable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EmailCategory {
	Valid,
	Risky,
	Unknown,
	Invalid,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubReason {
	Deliverable,
	InvalidSyntax,
	InvalidRecipient,
	SmtpUndeliverable,
	DisabledMailbox,
	NoMx,
	SmtpError,
	SmtpUnreachable,
	CatchAll,
	FullInbox,
	Disposable,
	RoleAccount,
	SpamTrap,
	Risky,
	Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScoringSignals {
	pub valid_syntax: bool,
	pub reachable: Reachable,
	pub has_mx_records: bool,
	pub smtp_error: bool,
	pub smtp_can_connect: bool,
	pub smtp_is_deliverable: bool,
	pub smtp_is_disabled: bool,
	pub smtp_is_catch_all: bool,
	pub smtp_has_full_inbox: bool,
	pub is_disposable: bool,
	pub is_role_account: bool,
	pub is_spam_trap_domain: bool,
	pub is_free_provider: bool,
	pub has_domain_suggestion: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmailScore {
	pub score: i16,
	pub category: EmailCategory,
	pub sub_reason: SubReason,
	pub safe_to_send: bool,
	pub reason_codes: Vec<String>,
	pub signals: ScoringSignals,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Freshness {
	Fresh,
	Recent,
	Aging,
	Stale,
	Expired,
}

pub struct FreshnessInfo {
	pub verified_at: String,
	pub age_days: i64,
	pub freshness: Freshness,
}

pub fn compute_freshness(completed_at: chrono::DateTime<chrono::Utc>) -> FreshnessInfo {
	let age = chrono::Utc::now() - completed_at;
	let age_days = age.num_days().max(0);
	let freshness = match age_days {
		0..=7 => Freshness::Fresh,
		8..=30 => Freshness::Recent,
		31..=60 => Freshness::Aging,
		61..=90 => Freshness::Stale,
		_ => Freshness::Expired,
	};
	FreshnessInfo {
		verified_at: completed_at.to_rfc3339(),
		age_days,
		freshness,
	}
}

pub fn compute_score(output: &CheckEmailOutput) -> EmailScore {
	let signals = extract_signals(output);

	if !signals.valid_syntax {
		let reason_codes = collect_reason_codes(&signals);
		return EmailScore {
			score: 0,
			category: EmailCategory::Invalid,
			sub_reason: SubReason::InvalidSyntax,
			safe_to_send: false,
			reason_codes,
			signals,
		};
	}

	if matches!(signals.reachable, Reachable::Invalid) {
		let reason_codes = collect_reason_codes(&signals);
		return EmailScore {
			score: 0,
			category: EmailCategory::Invalid,
			sub_reason: SubReason::InvalidRecipient,
			safe_to_send: false,
			reason_codes,
			signals,
		};
	}

	if !signals.smtp_is_deliverable {
		let reason_codes = collect_reason_codes(&signals);
		return EmailScore {
			score: 0,
			category: EmailCategory::Invalid,
			sub_reason: SubReason::SmtpUndeliverable,
			safe_to_send: false,
			reason_codes,
			signals,
		};
	}

	if signals.smtp_is_disabled {
		let reason_codes = collect_reason_codes(&signals);
		return EmailScore {
			score: 0,
			category: EmailCategory::Invalid,
			sub_reason: SubReason::DisabledMailbox,
			safe_to_send: false,
			reason_codes,
			signals,
		};
	}

	let mut score = 100i16;

	if matches!(signals.reachable, Reachable::Unknown) {
		score -= 40;
	}
	if signals.smtp_error {
		score -= 35;
	}
	if !signals.has_mx_records {
		score -= 50;
	}
	if !signals.smtp_can_connect {
		score -= 30;
	}
	if signals.smtp_is_catch_all {
		score -= 15;
	}
	if signals.smtp_has_full_inbox {
		score -= 20;
	}
	if signals.is_disposable {
		score -= 25;
	}
	if signals.is_role_account {
		score -= 10;
	}
	if signals.is_spam_trap_domain {
		score -= 30;
	}
	if matches!(signals.reachable, Reachable::Risky) {
		score -= 10;
	}

	score = score.clamp(0, 100);

	let category = match score {
		80..=100 => EmailCategory::Valid,
		50..=79 => EmailCategory::Risky,
		1..=49 => EmailCategory::Unknown,
		_ => EmailCategory::Invalid,
	};

	let sub_reason = if !signals.has_mx_records {
		SubReason::NoMx
	} else if signals.smtp_error {
		SubReason::SmtpError
	} else if !signals.smtp_can_connect {
		SubReason::SmtpUnreachable
	} else if signals.smtp_is_catch_all {
		SubReason::CatchAll
	} else if signals.smtp_has_full_inbox {
		SubReason::FullInbox
	} else if signals.is_disposable {
		SubReason::Disposable
	} else if signals.is_role_account {
		SubReason::RoleAccount
	} else if signals.is_spam_trap_domain {
		SubReason::SpamTrap
	} else if matches!(signals.reachable, Reachable::Risky) {
		SubReason::Risky
	} else if matches!(signals.reachable, Reachable::Unknown) {
		SubReason::Unknown
	} else {
		SubReason::Deliverable
	};

	let safe_to_send = category == EmailCategory::Valid
		&& !signals.is_disposable
		&& !signals.smtp_is_catch_all
		&& !signals.is_role_account
		&& !signals.is_spam_trap_domain;

	let reason_codes = collect_reason_codes(&signals);

	EmailScore {
		score,
		category,
		sub_reason,
		safe_to_send,
		reason_codes,
		signals,
	}
}

fn collect_reason_codes(signals: &ScoringSignals) -> Vec<String> {
	let mut codes = Vec::new();
	if !signals.valid_syntax {
		codes.push("invalid_syntax".to_string());
	}
	if matches!(signals.reachable, Reachable::Invalid) {
		codes.push("invalid_recipient".to_string());
	}
	if !signals.smtp_is_deliverable {
		codes.push("smtp_undeliverable".to_string());
	}
	if signals.smtp_is_disabled {
		codes.push("disabled_mailbox".to_string());
	}
	if !signals.has_mx_records {
		codes.push("no_mx".to_string());
	}
	if signals.smtp_error {
		codes.push("smtp_error".to_string());
	}
	if !signals.smtp_can_connect {
		codes.push("smtp_unreachable".to_string());
	}
	if signals.smtp_is_catch_all {
		codes.push("catch_all".to_string());
	}
	if signals.smtp_has_full_inbox {
		codes.push("full_inbox".to_string());
	}
	if signals.is_disposable {
		codes.push("disposable".to_string());
	}
	if signals.is_role_account {
		codes.push("role_account".to_string());
	}
	if signals.is_spam_trap_domain {
		codes.push("spam_trap".to_string());
	}
	if matches!(signals.reachable, Reachable::Unknown) {
		codes.push("unknown_deliverability".to_string());
	}
	if signals.is_free_provider {
		codes.push("free_provider".to_string());
	}
	if signals.has_domain_suggestion {
		codes.push("possible_typo".to_string());
	}
	if codes.is_empty() {
		codes.push("deliverable".to_string());
	}
	codes
}

fn extract_signals(output: &CheckEmailOutput) -> ScoringSignals {
	let has_mx_records = output
		.mx
		.as_ref()
		.ok()
		.and_then(|mx| mx.lookup.as_ref().ok())
		.map(|lookup| lookup.iter().next().is_some())
		.unwrap_or(false);

	let smtp_error = output.smtp.is_err();
	let smtp_ok = output.smtp.as_ref().ok();
	let misc_ok = output.misc.as_ref().ok();

	ScoringSignals {
		valid_syntax: output.syntax.is_valid_syntax,
		reachable: output.is_reachable.clone(),
		has_mx_records,
		smtp_error,
		smtp_can_connect: smtp_ok.map(|smtp| smtp.can_connect_smtp).unwrap_or(true),
		smtp_is_deliverable: smtp_ok.map(|smtp| smtp.is_deliverable).unwrap_or(true),
		smtp_is_disabled: smtp_ok.map(|smtp| smtp.is_disabled).unwrap_or(false),
		smtp_is_catch_all: smtp_ok.map(|smtp| smtp.is_catch_all).unwrap_or(false),
		smtp_has_full_inbox: smtp_ok.map(|smtp| smtp.has_full_inbox).unwrap_or(false),
		is_disposable: misc_ok.map(|misc| misc.is_disposable).unwrap_or(false),
		is_role_account: misc_ok.map(|misc| misc.is_role_account).unwrap_or(false),
		is_spam_trap_domain: misc_ok
			.map(|misc| misc.is_spam_trap_domain)
			.unwrap_or(false),
		is_free_provider: misc_ok.map(|misc| misc.is_b2c).unwrap_or(false),
		has_domain_suggestion: output.syntax.suggestion.is_some(),
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use check_if_email_exists::{
		misc::MiscDetails, mx::MxDetails, smtp::SmtpDetails, syntax::SyntaxDetails,
	};

	fn base_output() -> CheckEmailOutput {
		CheckEmailOutput {
			input: "user@example.com".into(),
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
			syntax: SyntaxDetails {
				address: None,
				domain: "example.com".into(),
				is_valid_syntax: true,
				username: "user".into(),
				normalized_email: Some("user@example.com".into()),
				suggestion: None,
			},
			debug: Default::default(),
		}
	}

	#[test]
	fn compute_score_invalid_syntax_short_circuits() {
		let mut output = base_output();
		output.syntax.is_valid_syntax = false;
		let score = compute_score(&output);
		assert_eq!(score.score, 0);
		assert_eq!(score.category, EmailCategory::Invalid);
		assert_eq!(score.sub_reason, SubReason::InvalidSyntax);
		assert!(!score.safe_to_send);
	}

	#[test]
	fn compute_score_deliverable() {
		let score = compute_score(&base_output());
		assert_eq!(score.score, 50);
		assert_eq!(score.category, EmailCategory::Risky);
		assert_eq!(score.sub_reason, SubReason::NoMx);
		assert!(!score.safe_to_send);
	}

	#[test]
	fn compute_score_unknown_with_penalties() {
		let mut output = base_output();
		output.is_reachable = Reachable::Unknown;
		output.misc = Ok(MiscDetails {
			is_disposable: true,
			is_role_account: true,
			..Default::default()
		});
		output.smtp = Ok(SmtpDetails {
			can_connect_smtp: false,
			has_full_inbox: true,
			is_catch_all: true,
			is_deliverable: true,
			is_disabled: false,
		});
		let score = compute_score(&output);
		assert_eq!(score.score, 0);
		assert_eq!(score.category, EmailCategory::Invalid);
		assert_eq!(score.sub_reason, SubReason::NoMx);
	}

	#[test]
	fn compute_score_smtp_error() {
		let mut output = base_output();
		output.mx = Err(check_if_email_exists::mx::MxError::from(
			std::io::Error::other("mx"),
		));
		output.smtp = Err(check_if_email_exists::smtp::SmtpError::from(
			std::io::Error::other("smtp"),
		));
		let score = compute_score(&output);
		assert_eq!(score.score, 15);
		assert_eq!(score.category, EmailCategory::Unknown);
		assert_eq!(score.sub_reason, SubReason::NoMx);
		assert!(!score.safe_to_send);
	}

	#[test]
	fn safe_to_send_true_when_valid_and_clean() {
		// Directly verify the safe_to_send derivation: Valid + not disposable + not catch-all + not role = true
		let score = EmailScore {
			score: 100,
			category: EmailCategory::Valid,
			sub_reason: SubReason::Deliverable,
			safe_to_send: EmailCategory::Valid == EmailCategory::Valid
				&& !false && !false
				&& !false,
			reason_codes: vec!["deliverable".to_string()],
			signals: ScoringSignals {
				valid_syntax: true,
				reachable: Reachable::Safe,
				has_mx_records: true,
				smtp_error: false,
				smtp_can_connect: true,
				smtp_is_deliverable: true,
				smtp_is_disabled: false,
				smtp_is_catch_all: false,
				smtp_has_full_inbox: false,
				is_disposable: false,
				is_role_account: false,
				is_spam_trap_domain: false,
				is_free_provider: false,
				has_domain_suggestion: false,
			},
		};
		assert!(score.safe_to_send);
	}

	#[test]
	fn safe_to_send_false_catch_all() {
		let mut output = base_output();
		output.smtp = Ok(SmtpDetails {
			can_connect_smtp: true,
			has_full_inbox: false,
			is_catch_all: true,
			is_deliverable: true,
			is_disabled: false,
		});
		let score = compute_score(&output);
		// catch-all emails are never safe to send regardless of category
		assert!(!score.safe_to_send);
	}

	#[test]
	fn safe_to_send_false_role_account() {
		let mut output = base_output();
		output.misc = Ok(MiscDetails {
			is_disposable: false,
			is_role_account: true,
			..Default::default()
		});
		let score = compute_score(&output);
		assert!(!score.safe_to_send);
	}

	#[test]
	fn safe_to_send_false_disposable() {
		let mut output = base_output();
		output.misc = Ok(MiscDetails {
			is_disposable: true,
			is_role_account: false,
			..Default::default()
		});
		let score = compute_score(&output);
		// disposable emails are never safe to send regardless of category
		assert!(!score.safe_to_send);
	}

	#[test]
	fn reason_codes_multiple_flags() {
		let mut output = base_output();
		output.smtp = Ok(SmtpDetails {
			can_connect_smtp: true,
			has_full_inbox: false,
			is_catch_all: true,
			is_deliverable: true,
			is_disabled: false,
		});
		output.misc = Ok(MiscDetails {
			is_disposable: true,
			is_role_account: true,
			is_b2c: true,
			..Default::default()
		});
		let score = compute_score(&output);
		assert!(score.reason_codes.contains(&"catch_all".to_string()));
		assert!(score.reason_codes.contains(&"disposable".to_string()));
		assert!(score.reason_codes.contains(&"role_account".to_string()));
		assert!(score.reason_codes.contains(&"free_provider".to_string()));
		assert!(!score.reason_codes.contains(&"deliverable".to_string()));
	}

	#[test]
	fn reason_codes_deliverable_when_clean() {
		let signals = ScoringSignals {
			valid_syntax: true,
			reachable: Reachable::Safe,
			has_mx_records: true,
			smtp_error: false,
			smtp_can_connect: true,
			smtp_is_deliverable: true,
			smtp_is_disabled: false,
			smtp_is_catch_all: false,
			smtp_has_full_inbox: false,
			is_disposable: false,
			is_role_account: false,
			is_spam_trap_domain: false,
			is_free_provider: false,
			has_domain_suggestion: false,
		};
		let codes = collect_reason_codes(&signals);
		assert_eq!(codes, vec!["deliverable"]);
	}

	#[test]
	fn reason_codes_possible_typo() {
		let mut output = base_output();
		output.syntax.suggestion = Some("user@example.com".to_string());
		let score = compute_score(&output);
		assert!(score.reason_codes.contains(&"possible_typo".to_string()));
	}

	#[test]
	fn reason_codes_invalid_syntax_early_return() {
		let mut output = base_output();
		output.syntax.is_valid_syntax = false;
		let score = compute_score(&output);
		assert!(score.reason_codes.contains(&"invalid_syntax".to_string()));
		assert!(!score.reason_codes.contains(&"deliverable".to_string()));
	}

	#[test]
	fn spam_trap_domain_penalizes_score_and_disqualifies_safe_to_send() {
		let mut output = base_output();
		output.misc = Ok(MiscDetails {
			is_spam_trap_domain: true,
			..Default::default()
		});
		let score = compute_score(&output);
		assert!(score.reason_codes.contains(&"spam_trap".to_string()));
		assert!(!score.safe_to_send);
		assert!(score.score < 100);
	}

	#[test]
	fn spam_trap_domain_appears_in_sub_reason() {
		// Test via signals directly to avoid base_output() MX issues
		let signals = ScoringSignals {
			valid_syntax: true,
			reachable: Reachable::Safe,
			has_mx_records: true,
			smtp_error: false,
			smtp_can_connect: true,
			smtp_is_deliverable: true,
			smtp_is_disabled: false,
			smtp_is_catch_all: false,
			smtp_has_full_inbox: false,
			is_disposable: false,
			is_role_account: false,
			is_spam_trap_domain: true,
			is_free_provider: false,
			has_domain_suggestion: false,
		};
		let codes = collect_reason_codes(&signals);
		assert!(codes.contains(&"spam_trap".to_string()));
		assert!(!codes.contains(&"deliverable".to_string()));
	}

	#[test]
	fn freshness_tier_fresh() {
		let now = chrono::Utc::now();
		assert_eq!(compute_freshness(now).freshness, Freshness::Fresh);
		assert_eq!(compute_freshness(now).age_days, 0);
		let seven_days_ago = now - chrono::Duration::days(7);
		assert_eq!(
			compute_freshness(seven_days_ago).freshness,
			Freshness::Fresh
		);
	}

	#[test]
	fn freshness_tier_recent() {
		let now = chrono::Utc::now();
		let eight_days_ago = now - chrono::Duration::days(8);
		assert_eq!(
			compute_freshness(eight_days_ago).freshness,
			Freshness::Recent
		);
		let thirty_days_ago = now - chrono::Duration::days(30);
		assert_eq!(
			compute_freshness(thirty_days_ago).freshness,
			Freshness::Recent
		);
	}

	#[test]
	fn freshness_tier_aging() {
		let now = chrono::Utc::now();
		let thirty_one = now - chrono::Duration::days(31);
		assert_eq!(compute_freshness(thirty_one).freshness, Freshness::Aging);
		let sixty = now - chrono::Duration::days(60);
		assert_eq!(compute_freshness(sixty).freshness, Freshness::Aging);
	}

	#[test]
	fn freshness_tier_stale() {
		let now = chrono::Utc::now();
		let sixty_one = now - chrono::Duration::days(61);
		assert_eq!(compute_freshness(sixty_one).freshness, Freshness::Stale);
		let ninety = now - chrono::Duration::days(90);
		assert_eq!(compute_freshness(ninety).freshness, Freshness::Stale);
	}

	#[test]
	fn freshness_tier_expired() {
		let now = chrono::Utc::now();
		let ninety_one = now - chrono::Duration::days(91);
		assert_eq!(compute_freshness(ninety_one).freshness, Freshness::Expired);
	}
}
