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
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmailScore {
	pub score: i16,
	pub category: EmailCategory,
	pub sub_reason: SubReason,
	pub signals: ScoringSignals,
}

pub fn compute_score(output: &CheckEmailOutput) -> EmailScore {
	let signals = extract_signals(output);

	if !signals.valid_syntax {
		return EmailScore {
			score: 0,
			category: EmailCategory::Invalid,
			sub_reason: SubReason::InvalidSyntax,
			signals,
		};
	}

	if matches!(signals.reachable, Reachable::Invalid) {
		return EmailScore {
			score: 0,
			category: EmailCategory::Invalid,
			sub_reason: SubReason::InvalidRecipient,
			signals,
		};
	}

	if !signals.smtp_is_deliverable {
		return EmailScore {
			score: 0,
			category: EmailCategory::Invalid,
			sub_reason: SubReason::SmtpUndeliverable,
			signals,
		};
	}

	if signals.smtp_is_disabled {
		return EmailScore {
			score: 0,
			category: EmailCategory::Invalid,
			sub_reason: SubReason::DisabledMailbox,
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
	} else if matches!(signals.reachable, Reachable::Risky) {
		SubReason::Risky
	} else if matches!(signals.reachable, Reachable::Unknown) {
		SubReason::Unknown
	} else {
		SubReason::Deliverable
	};

	EmailScore {
		score,
		category,
		sub_reason,
		signals,
	}
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
	}

	#[test]
	fn compute_score_deliverable() {
		let score = compute_score(&base_output());
		assert_eq!(score.score, 50);
		assert_eq!(score.category, EmailCategory::Risky);
		assert_eq!(score.sub_reason, SubReason::NoMx);
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
	}
}
