use crate::bounce_risk::{
	BounceRiskAssessment, BounceRiskCategory, RecommendedAction as BounceRecommendedAction,
};
use crate::decision::explain::{collect_explanations, ExplanationInput};
use crate::decision::policy::{evaluate_policy, PolicyInput};
use crate::decision::types::{
	DecisionConfidence, DecisionPriority, PolicyDecision, PolicyEvaluation, PolicyMode,
	Recommendation, RecommendedAction,
};
use crate::scoring::{EmailCategory, EmailScore, SubReason};
use chrono::{DateTime, Utc};

pub const DECISION_ENGINE_VERSION: &str = "decision_v1";

pub struct DecisionInput<'a> {
	pub score: &'a EmailScore,
	pub completed_at: DateTime<Utc>,
	pub evaluated_at: DateTime<Utc>,
	pub policy_mode: PolicyMode,
	pub policy_profile_key: Option<String>,
	pub domain_suggestion: Option<&'a str>,
	pub suggested_email: Option<String>,
	pub bounce_risk: Option<&'a BounceRiskAssessment>,
	pub active_suppression: bool,
	pub previous_hard_bounce: bool,
}

pub fn evaluate(input: &DecisionInput<'_>) -> (Recommendation, PolicyEvaluation) {
	let result_age_days = (input.evaluated_at - input.completed_at).num_days().max(0);
	let mut reasons = collect_explanations(&ExplanationInput {
		score: input.score,
		result_age_days: Some(result_age_days),
		domain_suggestion: input.domain_suggestion,
		bounce_risk: input.bounce_risk,
		previous_hard_bounce: input.previous_hard_bounce,
	});

	let policy_evaluation = evaluate_policy(&PolicyInput {
		mode: input.policy_mode,
		policy_profile_key: input.policy_profile_key.clone(),
		score: input.score,
		result_age_days,
		bounce_risk: input.bounce_risk,
		active_suppression: input.active_suppression,
		previous_hard_bounce: input.previous_hard_bounce,
		evaluated_at: input.evaluated_at,
	});

	let action = recommended_action(input, policy_evaluation.decision);
	let confidence = confidence(input);
	let priority = priority(action);
	let summary = summary_for(action);

	if input.active_suppression {
		reasons.insert(
			0,
			crate::decision::types::DecisionReason {
				code: "active_suppression".to_string(),
				severity: crate::decision::types::DecisionSeverity::Blocking,
				message: "Recipient is actively suppressed for this tenant.".to_string(),
				evidence: serde_json::json!({"active_suppression": true}),
			},
		);
	}
	for policy_reason in &policy_evaluation.reasons {
		if !reasons
			.iter()
			.any(|reason| reason.code == policy_reason.code)
		{
			reasons.push(policy_reason.clone());
		}
	}

	let recommendation = Recommendation {
		action,
		policy_mode: input.policy_mode,
		policy_profile_key: input.policy_profile_key.clone(),
		confidence,
		priority,
		summary: summary.to_string(),
		reasons: reasons.clone(),
		suggested_email: input.suggested_email.clone(),
		engine_version: DECISION_ENGINE_VERSION.to_string(),
		evaluated_at: input.evaluated_at,
	};

	(recommendation, policy_evaluation)
}

fn recommended_action(
	input: &DecisionInput<'_>,
	policy_decision: PolicyDecision,
) -> RecommendedAction {
	if input.active_suppression || is_hard_invalid(input.score) {
		return RecommendedAction::Drop;
	}

	if input.previous_hard_bounce
		|| input.score.signals.is_spam_trap_domain
		|| disposable_should_suppress(input)
	{
		return RecommendedAction::Suppress;
	}

	if input.domain_suggestion.is_some()
		&& matches!(
			input.score.category,
			EmailCategory::Valid | EmailCategory::Risky | EmailCategory::Unknown
		) {
		return RecommendedAction::FixThenSend;
	}

	if matches!(
		input.bounce_risk.map(|risk| &risk.category),
		Some(BounceRiskCategory::Medium)
	) || matches!(
		input.bounce_risk.map(|risk| &risk.action),
		Some(BounceRecommendedAction::SendWithCaution)
	) {
		return RecommendedAction::SendWithCaution;
	}

	match policy_decision {
		PolicyDecision::Send => RecommendedAction::Send,
		PolicyDecision::Review => RecommendedAction::Review,
		PolicyDecision::Suppress => RecommendedAction::Suppress,
		PolicyDecision::Drop => RecommendedAction::Drop,
	}
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

fn disposable_should_suppress(input: &DecisionInput<'_>) -> bool {
	input.score.signals.is_disposable
		&& matches!(
			input.policy_mode,
			PolicyMode::Deliverability
				| PolicyMode::SignupProtection
				| PolicyMode::EnterpriseStrict
		)
}

fn confidence(input: &DecisionInput<'_>) -> DecisionConfidence {
	if input.active_suppression || is_hard_invalid(input.score) {
		DecisionConfidence::High
	} else if input.bounce_risk.is_some() {
		DecisionConfidence::Medium
	} else {
		DecisionConfidence::Low
	}
}

fn priority(action: RecommendedAction) -> DecisionPriority {
	match action {
		RecommendedAction::Drop | RecommendedAction::Suppress => DecisionPriority::Blocking,
		RecommendedAction::Review | RecommendedAction::FixThenSend => DecisionPriority::High,
		RecommendedAction::SendWithCaution => DecisionPriority::Medium,
		RecommendedAction::Send => DecisionPriority::Low,
	}
}

fn summary_for(action: RecommendedAction) -> &'static str {
	match action {
		RecommendedAction::Send => "Email is acceptable for the selected policy.",
		RecommendedAction::SendWithCaution => {
			"Email can be used, but elevated risk signals should be monitored."
		}
		RecommendedAction::Review => "Email should be reviewed before automated use.",
		RecommendedAction::Suppress => {
			"Email should be added to or kept in suppression state before use."
		}
		RecommendedAction::Drop => "Email should be excluded from this workflow.",
		RecommendedAction::FixThenSend => "Email has a safe correction available before use.",
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

	fn input(score: &EmailScore) -> DecisionInput<'_> {
		let evaluated_at = "2026-06-30T12:00:00Z".parse::<DateTime<Utc>>().unwrap();
		DecisionInput {
			score,
			completed_at: evaluated_at,
			evaluated_at,
			policy_mode: PolicyMode::Deliverability,
			policy_profile_key: None,
			domain_suggestion: None,
			suggested_email: None,
			bounce_risk: None,
			active_suppression: false,
			previous_hard_bounce: false,
		}
	}

	fn bounce_risk(
		category: BounceRiskCategory,
		action: BounceRecommendedAction,
	) -> BounceRiskAssessment {
		BounceRiskAssessment {
			score: 50,
			category,
			confidence: 0.7,
			action,
			model_version: "test".to_string(),
			scored_at: "2026-06-30T12:00:00Z".to_string(),
			risk_factors: vec![],
		}
	}

	#[test]
	fn sends_clean_safe_result() {
		let score = base_score();
		let (recommendation, policy) = evaluate(&input(&score));
		assert_eq!(recommendation.action, RecommendedAction::Send);
		assert_eq!(policy.decision, PolicyDecision::Send);
	}

	#[test]
	fn sends_with_caution_for_medium_bounce_risk() {
		let score = base_score();
		let risk = bounce_risk(
			BounceRiskCategory::Medium,
			BounceRecommendedAction::SendWithCaution,
		);
		let input = DecisionInput {
			bounce_risk: Some(&risk),
			..input(&score)
		};
		let (recommendation, _) = evaluate(&input);
		assert_eq!(recommendation.action, RecommendedAction::SendWithCaution);
	}

	#[test]
	fn reviews_risky_result() {
		let mut score = base_score();
		score.category = EmailCategory::Risky;
		score.safe_to_send = false;
		let (recommendation, policy) = evaluate(&input(&score));
		assert_eq!(recommendation.action, RecommendedAction::Review);
		assert_eq!(policy.decision, PolicyDecision::Review);
	}

	#[test]
	fn suppresses_disposable_result_under_deliverability_policy() {
		let mut score = base_score();
		score.signals.is_disposable = true;
		let (recommendation, policy) = evaluate(&input(&score));
		assert_eq!(recommendation.action, RecommendedAction::Suppress);
		assert_eq!(policy.decision, PolicyDecision::Suppress);
	}

	#[test]
	fn drops_hard_invalid_result() {
		let mut score = base_score();
		score.category = EmailCategory::Invalid;
		score.sub_reason = SubReason::InvalidSyntax;
		score.safe_to_send = false;
		let (recommendation, policy) = evaluate(&input(&score));
		assert_eq!(recommendation.action, RecommendedAction::Drop);
		assert_eq!(policy.decision, PolicyDecision::Drop);
	}

	#[test]
	fn fixes_then_sends_when_domain_suggestion_exists() {
		let score = base_score();
		let input = DecisionInput {
			domain_suggestion: Some("user@gmail.com"),
			suggested_email: Some("user@gmail.com".to_string()),
			..input(&score)
		};
		let (recommendation, _) = evaluate(&input);
		assert_eq!(recommendation.action, RecommendedAction::FixThenSend);
		assert_eq!(
			recommendation.suggested_email.as_deref(),
			Some("user@gmail.com")
		);
	}

	#[test]
	fn active_suppression_blocks_send() {
		let score = base_score();
		let input = DecisionInput {
			active_suppression: true,
			..input(&score)
		};
		let (recommendation, policy) = evaluate(&input);
		assert_eq!(recommendation.action, RecommendedAction::Drop);
		assert_eq!(policy.decision, PolicyDecision::Drop);
		assert_eq!(recommendation.reasons[0].code, "active_suppression");
	}
}
