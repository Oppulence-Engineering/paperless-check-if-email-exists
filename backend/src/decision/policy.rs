use crate::bounce_risk::{BounceRiskAssessment, BounceRiskCategory};
use crate::decision::types::{
	DecisionReason, DecisionSeverity, PolicyDecision, PolicyEvaluation, PolicyMode,
};
use crate::scoring::{EmailCategory, EmailScore, SubReason};
use chrono::{DateTime, Utc};

pub const POLICY_ENGINE_VERSION: &str = "policy_v1";

pub struct PolicyInput<'a> {
	pub mode: PolicyMode,
	pub policy_profile_key: Option<String>,
	pub score: &'a EmailScore,
	pub result_age_days: i64,
	pub bounce_risk: Option<&'a BounceRiskAssessment>,
	pub active_suppression: bool,
	pub previous_hard_bounce: bool,
	pub evaluated_at: DateTime<Utc>,
}

pub fn evaluate_policy(input: &PolicyInput<'_>) -> PolicyEvaluation {
	let mut reasons = Vec::new();
	let decision = if input.active_suppression {
		reasons.push(reason(
			"actively_suppressed",
			DecisionSeverity::Blocking,
			"Recipient is actively suppressed for this tenant.",
		));
		PolicyDecision::Drop
	} else if is_hard_invalid(input.score) {
		reasons.push(reason(
			"hard_invalid",
			DecisionSeverity::Blocking,
			"Verification result is hard invalid.",
		));
		PolicyDecision::Drop
	} else if input.previous_hard_bounce {
		reasons.push(reason(
			"previous_hard_bounce",
			DecisionSeverity::Critical,
			"Recipient has a previous hard bounce for this tenant.",
		));
		PolicyDecision::Suppress
	} else {
		match input.mode {
			PolicyMode::Growth => evaluate_growth(input, &mut reasons),
			PolicyMode::Deliverability => evaluate_deliverability(input, &mut reasons),
			PolicyMode::SignupProtection => evaluate_signup_protection(input, &mut reasons),
			PolicyMode::EnterpriseStrict => evaluate_enterprise_strict(input, &mut reasons),
			PolicyMode::Custom => {
				reasons.push(reason(
					"custom_policy_unavailable",
					DecisionSeverity::Warning,
					"Custom policy profile was not loaded for this evaluation.",
				));
				PolicyDecision::Review
			}
		}
	};

	PolicyEvaluation {
		mode: input.mode,
		policy_profile_key: input.policy_profile_key.clone(),
		decision,
		reasons,
		evaluated_at: input.evaluated_at,
		result_age_days: Some(input.result_age_days),
		engine_version: POLICY_ENGINE_VERSION.to_string(),
	}
}

fn evaluate_growth(input: &PolicyInput<'_>, reasons: &mut Vec<DecisionReason>) -> PolicyDecision {
	if input.score.signals.is_disposable || input.score.signals.is_spam_trap_domain {
		reasons.push(reason(
			"growth_blocked_negative_signal",
			DecisionSeverity::Critical,
			"Growth mode still blocks disposable or spam-trap addresses.",
		));
		return PolicyDecision::Suppress;
	}
	if bounce_risk_is_dangerous(input) {
		reasons.push(reason(
			"bounce_risk_dangerous",
			DecisionSeverity::Critical,
			"Bounce-risk assessment is too dangerous for growth mode.",
		));
		return PolicyDecision::Suppress;
	}
	if matches!(
		input.score.category,
		EmailCategory::Valid | EmailCategory::Risky
	) {
		PolicyDecision::Send
	} else {
		reasons.push(reason(
			"growth_requires_valid_or_risky",
			DecisionSeverity::Warning,
			"Growth mode sends valid and risky rows, but this result is lower confidence.",
		));
		PolicyDecision::Review
	}
}

fn evaluate_deliverability(
	input: &PolicyInput<'_>,
	reasons: &mut Vec<DecisionReason>,
) -> PolicyDecision {
	if input.score.signals.is_disposable || input.score.signals.is_spam_trap_domain {
		reasons.push(reason(
			"deliverability_suppression_signal",
			DecisionSeverity::Critical,
			"Deliverability mode suppresses disposable or spam-trap addresses.",
		));
		return PolicyDecision::Suppress;
	}
	if !input.score.safe_to_send {
		reasons.push(reason(
			"safe_to_send_false",
			DecisionSeverity::Warning,
			"Deliverability mode requires safe_to_send=true.",
		));
		return PolicyDecision::Review;
	}
	if input.result_age_days > 30 {
		reasons.push(reason(
			"result_stale",
			DecisionSeverity::Warning,
			"Deliverability mode requires verification freshness <= 30 days.",
		));
		return PolicyDecision::Review;
	}
	if bounce_risk_is_high_or_worse(input) {
		reasons.push(reason(
			"bounce_risk_too_high",
			DecisionSeverity::Warning,
			"Deliverability mode blocks high bounce-risk rows from automatic send.",
		));
		return PolicyDecision::Review;
	}
	PolicyDecision::Send
}

fn evaluate_signup_protection(
	input: &PolicyInput<'_>,
	reasons: &mut Vec<DecisionReason>,
) -> PolicyDecision {
	if input.score.signals.is_disposable
		|| input.score.signals.is_spam_trap_domain
		|| bounce_risk_is_high_or_worse(input)
	{
		reasons.push(reason(
			"signup_high_risk_signal",
			DecisionSeverity::Critical,
			"Signup protection mode suppresses high-risk signups.",
		));
		return PolicyDecision::Suppress;
	}
	if input.score.signals.smtp_is_catch_all
		|| matches!(input.score.category, EmailCategory::Unknown)
		|| matches!(
			input.score.sub_reason,
			SubReason::SmtpError | SubReason::SmtpUnreachable
		) {
		reasons.push(reason(
			"signup_requires_review",
			DecisionSeverity::Warning,
			"Signup protection mode reviews ambiguous verification results.",
		));
		return PolicyDecision::Review;
	}
	if input.score.safe_to_send || input.score.category == EmailCategory::Valid {
		PolicyDecision::Send
	} else {
		PolicyDecision::Review
	}
}

fn evaluate_enterprise_strict(
	input: &PolicyInput<'_>,
	reasons: &mut Vec<DecisionReason>,
) -> PolicyDecision {
	if input.score.signals.is_disposable || input.score.signals.is_spam_trap_domain {
		reasons.push(reason(
			"enterprise_suppression_signal",
			DecisionSeverity::Critical,
			"Enterprise strict mode suppresses disposable or spam-trap addresses.",
		));
		return PolicyDecision::Suppress;
	}
	if input.result_age_days > 14 {
		reasons.push(reason(
			"result_stale",
			DecisionSeverity::Warning,
			"Enterprise strict mode requires verification freshness <= 14 days.",
		));
		return PolicyDecision::Review;
	}
	if !input.score.safe_to_send
		|| input.score.signals.smtp_is_catch_all
		|| input.score.signals.is_role_account
	{
		reasons.push(reason(
			"enterprise_strict_requires_clean_row",
			DecisionSeverity::Warning,
			"Enterprise strict mode sends only clean individual recipients.",
		));
		return PolicyDecision::Review;
	}
	if !bounce_risk_is_safe_or_low(input) {
		reasons.push(reason(
			"bounce_risk_too_high",
			DecisionSeverity::Warning,
			"Enterprise strict mode requires safe or low bounce risk.",
		));
		return PolicyDecision::Review;
	}
	PolicyDecision::Send
}

fn is_hard_invalid(score: &EmailScore) -> bool {
	score.category == EmailCategory::Invalid
		|| matches!(
			score.sub_reason,
			SubReason::InvalidSyntax
				| SubReason::InvalidRecipient
				| SubReason::ProviderRejected
				| SubReason::SmtpUndeliverable
				| SubReason::DisabledMailbox
		)
}

fn bounce_risk_is_high_or_worse(input: &PolicyInput<'_>) -> bool {
	input
		.bounce_risk
		.map(|risk| {
			matches!(
				risk.category,
				BounceRiskCategory::High | BounceRiskCategory::Dangerous
			)
		})
		.unwrap_or(false)
}

fn bounce_risk_is_dangerous(input: &PolicyInput<'_>) -> bool {
	input
		.bounce_risk
		.map(|risk| matches!(risk.category, BounceRiskCategory::Dangerous))
		.unwrap_or(false)
}

fn bounce_risk_is_safe_or_low(input: &PolicyInput<'_>) -> bool {
	input
		.bounce_risk
		.map(|risk| {
			matches!(
				risk.category,
				BounceRiskCategory::Safe | BounceRiskCategory::Low
			)
		})
		.unwrap_or(true)
}

fn reason(code: &str, severity: DecisionSeverity, message: &str) -> DecisionReason {
	DecisionReason {
		code: code.to_string(),
		severity,
		message: message.to_string(),
		evidence: serde_json::Value::Null,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::bounce_risk::{
		BounceRiskAssessment, BounceRiskCategory, RecommendedAction as BounceRecommendedAction,
	};
	use crate::scoring::{EmailCategory, ScoringSignals, SubReason};
	use check_if_email_exists::Reachable;

	fn base_score() -> EmailScore {
		EmailScore {
			score: 95,
			category: EmailCategory::Valid,
			sub_reason: SubReason::Deliverable,
			safe_to_send: true,
			reason_codes: vec![],
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
		}
	}

	fn input(score: &EmailScore, mode: PolicyMode) -> PolicyInput<'_> {
		PolicyInput {
			mode,
			policy_profile_key: None,
			score,
			result_age_days: 0,
			bounce_risk: None,
			active_suppression: false,
			previous_hard_bounce: false,
			evaluated_at: "2026-06-30T12:00:00Z".parse::<DateTime<Utc>>().unwrap(),
		}
	}

	fn low_bounce_risk() -> BounceRiskAssessment {
		BounceRiskAssessment {
			score: 15,
			category: BounceRiskCategory::Low,
			confidence: 0.7,
			action: BounceRecommendedAction::Send,
			model_version: "test".to_string(),
			scored_at: "2026-06-30T12:00:00Z".to_string(),
			risk_factors: vec![],
		}
	}

	#[test]
	fn clean_row_sends_in_deliverability_mode() {
		let score = base_score();
		let policy = evaluate_policy(&input(&score, PolicyMode::Deliverability));
		assert_eq!(policy.decision, PolicyDecision::Send);
	}

	#[test]
	fn same_role_account_score_differs_by_policy_mode() {
		let mut score = base_score();
		score.signals.is_role_account = true;
		score.safe_to_send = false;

		assert_eq!(
			evaluate_policy(&input(&score, PolicyMode::Growth)).decision,
			PolicyDecision::Send
		);
		assert_eq!(
			evaluate_policy(&input(&score, PolicyMode::Deliverability)).decision,
			PolicyDecision::Review
		);
		assert_eq!(
			evaluate_policy(&input(&score, PolicyMode::SignupProtection)).decision,
			PolicyDecision::Send
		);
		assert_eq!(
			evaluate_policy(&input(&score, PolicyMode::EnterpriseStrict)).decision,
			PolicyDecision::Review
		);
	}

	#[test]
	fn enterprise_strict_sends_with_low_bounce_risk() {
		let score = base_score();
		let risk = low_bounce_risk();
		let input = PolicyInput {
			bounce_risk: Some(&risk),
			..input(&score, PolicyMode::EnterpriseStrict)
		};
		assert_eq!(evaluate_policy(&input).decision, PolicyDecision::Send);
	}

	#[test]
	fn custom_policy_without_profile_reviews() {
		let score = base_score();
		let policy = evaluate_policy(&input(&score, PolicyMode::Custom));
		assert_eq!(policy.decision, PolicyDecision::Review);
		assert_eq!(policy.reasons[0].code, "custom_policy_unavailable");
	}
}
