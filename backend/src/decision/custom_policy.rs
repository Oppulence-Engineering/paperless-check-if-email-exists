use crate::decision::types::PolicyDecision;
use serde::{Deserialize, Serialize};
use serde_json::Value;

const MAX_RULES: usize = 25;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomPolicyRules {
	#[serde(default)]
	pub rules: Vec<CustomPolicyRule>,
	pub default_decision: PolicyDecision,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomPolicyRule {
	pub field: String,
	pub operator: CustomPolicyOperator,
	pub value: Value,
	pub decision: PolicyDecision,
	#[serde(default)]
	pub reason_code: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CustomPolicyOperator {
	Eq,
	Neq,
	Lt,
	Lte,
	Gt,
	Gte,
}

pub fn parse_and_validate_rules(value: &Value) -> Result<CustomPolicyRules, String> {
	let rules: CustomPolicyRules =
		serde_json::from_value(value.clone()).map_err(|err| err.to_string())?;
	validate_rules(&rules)?;
	Ok(rules)
}

pub fn validate_rules(rules: &CustomPolicyRules) -> Result<(), String> {
	if rules.rules.len() > MAX_RULES {
		return Err(format!(
			"custom policies support at most {} rules",
			MAX_RULES
		));
	}

	for rule in &rules.rules {
		if !is_allowed_field(&rule.field) {
			return Err(format!("unsupported custom policy field '{}'", rule.field));
		}
		if requires_number(rule.operator) && !rule.value.is_number() {
			return Err(format!(
				"custom policy field '{}' requires a numeric comparison value",
				rule.field
			));
		}
	}

	Ok(())
}

fn is_allowed_field(field: &str) -> bool {
	matches!(
		field,
		"score"
			| "category"
			| "safe_to_send"
			| "is_disposable"
			| "is_role_account"
			| "is_catch_all"
			| "bounce_risk_category"
			| "active_suppression"
	)
}

fn requires_number(operator: CustomPolicyOperator) -> bool {
	matches!(
		operator,
		CustomPolicyOperator::Lt
			| CustomPolicyOperator::Lte
			| CustomPolicyOperator::Gt
			| CustomPolicyOperator::Gte
	)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn validates_supported_rule_shape() {
		let rules = parse_and_validate_rules(&serde_json::json!({
			"default_decision": "review",
			"rules": [
				{
					"field": "score",
					"operator": "gte",
					"value": 90,
					"decision": "send",
					"reason_code": "high_score"
				}
			]
		}))
		.unwrap();

		assert_eq!(rules.default_decision, PolicyDecision::Review);
		assert_eq!(rules.rules[0].decision, PolicyDecision::Send);
	}

	#[test]
	fn rejects_unsupported_field() {
		let error = parse_and_validate_rules(&serde_json::json!({
			"default_decision": "review",
			"rules": [
				{
					"field": "tenant_id",
					"operator": "eq",
					"value": "x",
					"decision": "drop"
				}
			]
		}))
		.unwrap_err();

		assert!(error.contains("unsupported custom policy field"));
	}

	#[test]
	fn rejects_non_numeric_threshold_for_numeric_operator() {
		let error = parse_and_validate_rules(&serde_json::json!({
			"default_decision": "review",
			"rules": [
				{
					"field": "score",
					"operator": "gte",
					"value": "high",
					"decision": "send"
				}
			]
		}))
		.unwrap_err();

		assert!(error.contains("requires a numeric comparison value"));
	}
}
