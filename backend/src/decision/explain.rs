use crate::bounce_risk::{BounceRiskAssessment, BounceRiskCategory};
use crate::decision::types::{DecisionReason, DecisionSeverity};
use crate::scoring::EmailScore;
use serde_json::json;

pub struct ExplanationInput<'a> {
	pub score: &'a EmailScore,
	pub result_age_days: Option<i64>,
	pub domain_suggestion: Option<&'a str>,
	pub bounce_risk: Option<&'a BounceRiskAssessment>,
	pub previous_hard_bounce: bool,
}

pub fn collect_explanations(input: &ExplanationInput<'_>) -> Vec<DecisionReason> {
	let mut reasons = Vec::new();
	let signals = &input.score.signals;

	if signals.smtp_is_catch_all {
		reasons.push(reason(
			if signals.is_free_provider {
				"catch_all_domain"
			} else {
				"catch_all_corporate_domain"
			},
			DecisionSeverity::Warning,
			"Domain accepts catch-all mail and may produce false positives.",
			json!({
				"is_catch_all": true,
				"is_free_provider": signals.is_free_provider,
			}),
		));
	}

	if let Some(suggestion) = input.domain_suggestion {
		reasons.push(reason(
			"possible_domain_typo",
			DecisionSeverity::Warning,
			"Email has a possible domain typo with a suggested correction.",
			json!({
				"suggested_email": suggestion,
				"score": input.score.score,
			}),
		));
	}

	if let Some(age_days) = input.result_age_days {
		if age_days > 30 {
			reasons.push(reason(
				"stale_verification",
				DecisionSeverity::Warning,
				"Verification is stale and should be refreshed before automation.",
				json!({
					"age_days": age_days,
					"freshness_threshold_days": 30,
				}),
			));
		}
	}

	if signals.is_role_account {
		reasons.push(reason(
			"role_account",
			DecisionSeverity::Info,
			"Address appears to be a role account rather than an individual recipient.",
			json!({"is_role_account": true}),
		));
	}

	if signals.is_disposable {
		reasons.push(reason(
			"disposable_provider",
			DecisionSeverity::Critical,
			"Address belongs to a known disposable email provider.",
			json!({"is_disposable": true}),
		));
	}

	if weak_mail_infrastructure(input) {
		reasons.push(reason(
			"weak_mail_infrastructure",
			DecisionSeverity::Warning,
			"Domain mail infrastructure is weak or verification could not connect reliably.",
			json!({
				"has_mx_records": signals.has_mx_records,
				"smtp_error": signals.smtp_error,
				"smtp_can_connect": signals.smtp_can_connect,
				"bounce_risk_category": input.bounce_risk.map(|risk| serde_json::to_value(&risk.category).unwrap_or(serde_json::Value::Null)),
			}),
		));
	}

	if input.previous_hard_bounce {
		reasons.push(reason(
			"previously_bounced_for_tenant",
			DecisionSeverity::Critical,
			"Recipient previously produced a hard bounce for this tenant.",
			json!({"previous_outcome": "bounce_hard"}),
		));
	}

	reasons
}

fn weak_mail_infrastructure(input: &ExplanationInput<'_>) -> bool {
	let signals = &input.score.signals;
	!signals.has_mx_records
		|| signals.smtp_error
		|| !signals.smtp_can_connect
		|| input
			.bounce_risk
			.map(|risk| {
				matches!(
					risk.category,
					BounceRiskCategory::High | BounceRiskCategory::Dangerous
				)
			})
			.unwrap_or(false)
}

fn reason(
	code: &str,
	severity: DecisionSeverity,
	message: &str,
	evidence: serde_json::Value,
) -> DecisionReason {
	DecisionReason {
		code: code.to_string(),
		severity,
		message: message.to_string(),
		evidence,
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

	fn input(score: &EmailScore) -> ExplanationInput<'_> {
		ExplanationInput {
			score,
			result_age_days: None,
			domain_suggestion: None,
			bounce_risk: None,
			previous_hard_bounce: false,
		}
	}

	fn code_exists(reasons: &[DecisionReason], code: &str) -> bool {
		reasons.iter().any(|reason| reason.code == code)
	}

	#[test]
	fn explains_catch_all_corporate_domain() {
		let mut score = base_score();
		score.signals.smtp_is_catch_all = true;
		let reasons = collect_explanations(&input(&score));
		assert!(code_exists(&reasons, "catch_all_corporate_domain"));
		assert_eq!(reasons[0].evidence["is_catch_all"], true);
	}

	#[test]
	fn explains_possible_domain_typo() {
		let score = base_score();
		let input = ExplanationInput {
			domain_suggestion: Some("user@gmail.com"),
			..input(&score)
		};
		let reasons = collect_explanations(&input);
		assert!(code_exists(&reasons, "possible_domain_typo"));
		assert_eq!(reasons[0].evidence["suggested_email"], "user@gmail.com");
	}

	#[test]
	fn explains_stale_verification() {
		let score = base_score();
		let input = ExplanationInput {
			result_age_days: Some(87),
			..input(&score)
		};
		let reasons = collect_explanations(&input);
		assert!(code_exists(&reasons, "stale_verification"));
		assert_eq!(reasons[0].evidence["age_days"], 87);
	}

	#[test]
	fn explains_role_account() {
		let mut score = base_score();
		score.signals.is_role_account = true;
		let reasons = collect_explanations(&input(&score));
		assert!(code_exists(&reasons, "role_account"));
	}

	#[test]
	fn explains_disposable_provider() {
		let mut score = base_score();
		score.signals.is_disposable = true;
		let reasons = collect_explanations(&input(&score));
		assert!(code_exists(&reasons, "disposable_provider"));
	}

	#[test]
	fn explains_weak_mail_infrastructure_from_dns_and_smtp() {
		let mut score = base_score();
		score.signals.has_mx_records = false;
		score.signals.smtp_can_connect = false;
		let reasons = collect_explanations(&input(&score));
		assert!(code_exists(&reasons, "weak_mail_infrastructure"));
	}

	#[test]
	fn explains_weak_mail_infrastructure_from_bounce_risk() {
		let score = base_score();
		let bounce_risk = BounceRiskAssessment {
			score: 81,
			category: BounceRiskCategory::High,
			confidence: 0.8,
			action: BounceRecommendedAction::VerifyManually,
			model_version: "test".to_string(),
			scored_at: "2026-06-30T12:00:00Z".to_string(),
			risk_factors: vec![],
		};
		let input = ExplanationInput {
			bounce_risk: Some(&bounce_risk),
			..input(&score)
		};
		let reasons = collect_explanations(&input);
		assert!(code_exists(&reasons, "weak_mail_infrastructure"));
	}

	#[test]
	fn explains_previous_hard_bounce() {
		let score = base_score();
		let input = ExplanationInput {
			previous_hard_bounce: true,
			..input(&score)
		};
		let reasons = collect_explanations(&input);
		assert!(code_exists(&reasons, "previously_bounced_for_tenant"));
	}
}
