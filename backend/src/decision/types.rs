use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecommendedAction {
	Send,
	SendWithCaution,
	Review,
	Suppress,
	Drop,
	FixThenSend,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyMode {
	Growth,
	Deliverability,
	SignupProtection,
	EnterpriseStrict,
	Custom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DecisionConfidence {
	High,
	Medium,
	Low,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DecisionPriority {
	Low,
	Medium,
	High,
	Blocking,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DecisionSeverity {
	Info,
	Warning,
	Critical,
	Blocking,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyDecision {
	Send,
	Review,
	Suppress,
	Drop,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DecisionReason {
	pub code: String,
	pub severity: DecisionSeverity,
	pub message: String,
	#[serde(default, skip_serializing_if = "Value::is_null")]
	pub evidence: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Recommendation {
	pub action: RecommendedAction,
	pub policy_mode: PolicyMode,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub policy_profile_key: Option<String>,
	pub confidence: DecisionConfidence,
	pub priority: DecisionPriority,
	pub summary: String,
	pub reasons: Vec<DecisionReason>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub suggested_email: Option<String>,
	pub engine_version: String,
	pub evaluated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PolicyEvaluation {
	pub mode: PolicyMode,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub policy_profile_key: Option<String>,
	pub decision: PolicyDecision,
	pub reasons: Vec<DecisionReason>,
	pub evaluated_at: DateTime<Utc>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub result_age_days: Option<i64>,
	pub engine_version: String,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn recommended_actions_serialize_with_public_values() {
		let cases = [
			(RecommendedAction::Send, "send"),
			(RecommendedAction::SendWithCaution, "send_with_caution"),
			(RecommendedAction::Review, "review"),
			(RecommendedAction::Suppress, "suppress"),
			(RecommendedAction::Drop, "drop"),
			(RecommendedAction::FixThenSend, "fix_then_send"),
		];

		for (action, expected) in cases {
			let json = serde_json::to_string(&action).unwrap();
			assert_eq!(json, format!("\"{}\"", expected));
			assert_eq!(
				serde_json::from_str::<RecommendedAction>(&json).unwrap(),
				action
			);
		}
	}

	#[test]
	fn policy_modes_serialize_with_public_values() {
		let cases = [
			(PolicyMode::Growth, "growth"),
			(PolicyMode::Deliverability, "deliverability"),
			(PolicyMode::SignupProtection, "signup_protection"),
			(PolicyMode::EnterpriseStrict, "enterprise_strict"),
			(PolicyMode::Custom, "custom"),
		];

		for (mode, expected) in cases {
			let json = serde_json::to_string(&mode).unwrap();
			assert_eq!(json, format!("\"{}\"", expected));
			assert_eq!(serde_json::from_str::<PolicyMode>(&json).unwrap(), mode);
		}
	}

	#[test]
	fn recommendation_round_trips_with_required_fields() {
		let evaluated_at = "2026-06-30T12:00:00Z".parse::<DateTime<Utc>>().unwrap();
		let recommendation = Recommendation {
			action: RecommendedAction::SendWithCaution,
			policy_mode: PolicyMode::Deliverability,
			policy_profile_key: None,
			confidence: DecisionConfidence::Medium,
			priority: DecisionPriority::High,
			summary: "Use caution for this recipient.".to_string(),
			reasons: vec![DecisionReason {
				code: "catch_all_corporate_domain".to_string(),
				severity: DecisionSeverity::Warning,
				message: "Domain accepts catch-all mail.".to_string(),
				evidence: serde_json::json!({"is_catch_all": true}),
			}],
			suggested_email: None,
			engine_version: "decision_v1".to_string(),
			evaluated_at,
		};

		let json = serde_json::to_string(&recommendation).unwrap();
		let decoded: Recommendation = serde_json::from_str(&json).unwrap();
		assert_eq!(decoded, recommendation);
	}
}
