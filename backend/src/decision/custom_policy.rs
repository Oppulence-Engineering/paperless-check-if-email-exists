use crate::decision::types::PolicyDecision;
use check_if_email_exists::LOG_TARGET;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{PgPool, Row};
use uuid::Uuid;

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

/// The tenant's active custom profile, most recently updated first, as
/// `(profile_key, rules)`. Rules that no longer validate are skipped with a
/// warning, so the policy falls back to "unavailable" instead of guessing.
pub async fn load_active_profile(
	pool: &PgPool,
	tenant_id: Uuid,
) -> Result<Option<(String, CustomPolicyRules)>, sqlx::Error> {
	let row = sqlx::query(
		"SELECT profile_key, rules FROM tenant_policy_profiles \
		 WHERE tenant_id = $1 AND status = 'active' \
		 ORDER BY updated_at DESC, id DESC LIMIT 1",
	)
	.bind(tenant_id)
	.fetch_optional(pool)
	.await?;
	let Some(row) = row else {
		return Ok(None);
	};
	let profile_key: String = row.get("profile_key");
	match parse_and_validate_rules(&row.get::<Value, _>("rules")) {
		Ok(rules) => Ok(Some((profile_key, rules))),
		Err(error) => {
			tracing::warn!(target: LOG_TARGET, tenant_id = %tenant_id, profile_key = %profile_key, error = %error, "Ignoring invalid custom policy profile");
			Ok(None)
		}
	}
}

/// The values of one verification that custom rules can compare against.
/// Each field matches a name accepted by [`validate_rules`].
#[derive(Debug, Clone)]
pub struct CustomPolicyFacts<'a> {
	pub score: i16,
	pub category: &'a str,
	pub safe_to_send: bool,
	pub is_disposable: bool,
	pub is_role_account: bool,
	pub is_catch_all: bool,
	pub bounce_risk_category: Option<&'a str>,
	pub active_suppression: bool,
}

/// Applies the rules in order: the first matching rule decides, with its
/// reason code. When none match, the profile's default decision applies.
pub fn evaluate_rules(
	rules: &CustomPolicyRules,
	facts: &CustomPolicyFacts<'_>,
) -> (PolicyDecision, Option<String>) {
	rules
		.rules
		.iter()
		.find(|rule| rule_matches(rule, facts))
		.map(|rule| (rule.decision, rule.reason_code.clone()))
		.unwrap_or((rules.default_decision, None))
}

fn rule_matches(rule: &CustomPolicyRule, facts: &CustomPolicyFacts<'_>) -> bool {
	let actual = fact_value(&rule.field, facts);
	match rule.operator {
		CustomPolicyOperator::Eq => actual == rule.value,
		CustomPolicyOperator::Neq => actual != rule.value,
		CustomPolicyOperator::Lt
		| CustomPolicyOperator::Lte
		| CustomPolicyOperator::Gt
		| CustomPolicyOperator::Gte => match (actual.as_f64(), rule.value.as_f64()) {
			(Some(actual), Some(expected)) => match rule.operator {
				CustomPolicyOperator::Lt => actual < expected,
				CustomPolicyOperator::Lte => actual <= expected,
				CustomPolicyOperator::Gt => actual > expected,
				_ => actual >= expected,
			},
			// A numeric comparison against a non-number never matches.
			_ => false,
		},
	}
}

fn fact_value(field: &str, facts: &CustomPolicyFacts<'_>) -> Value {
	match field {
		"score" => Value::from(facts.score),
		"category" => Value::from(facts.category),
		"safe_to_send" => Value::from(facts.safe_to_send),
		"is_disposable" => Value::from(facts.is_disposable),
		"is_role_account" => Value::from(facts.is_role_account),
		"is_catch_all" => Value::from(facts.is_catch_all),
		"bounce_risk_category" => facts
			.bounce_risk_category
			.map(Value::from)
			.unwrap_or(Value::Null),
		"active_suppression" => Value::from(facts.active_suppression),
		// validate_rules rejects every other field before rules are stored.
		_ => Value::Null,
	}
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

	fn facts() -> CustomPolicyFacts<'static> {
		CustomPolicyFacts {
			score: 72,
			category: "risky",
			safe_to_send: false,
			is_disposable: false,
			is_role_account: true,
			is_catch_all: false,
			bounce_risk_category: Some("medium"),
			active_suppression: false,
		}
	}

	#[test]
	fn first_matching_rule_decides_and_default_applies_otherwise() {
		let rules = parse_and_validate_rules(&serde_json::json!({
			"default_decision": "send",
			"rules": [
				{"field": "score", "operator": "lt", "value": 50, "decision": "drop", "reason_code": "low_score"},
				{"field": "is_role_account", "operator": "eq", "value": true, "decision": "review", "reason_code": "role"},
				{"field": "category", "operator": "eq", "value": "risky", "decision": "suppress"}
			]
		}))
		.unwrap();

		// Score 72 skips the first rule; the role rule matches before the
		// category rule.
		assert_eq!(
			evaluate_rules(&rules, &facts()),
			(PolicyDecision::Review, Some("role".to_string()))
		);

		let clean = CustomPolicyFacts {
			is_role_account: false,
			category: "valid",
			..facts()
		};
		assert_eq!(evaluate_rules(&rules, &clean), (PolicyDecision::Send, None));

		let low = CustomPolicyFacts {
			score: 49,
			..facts()
		};
		assert_eq!(
			evaluate_rules(&rules, &low).0,
			PolicyDecision::Drop,
			"lt is strict and numeric"
		);
	}

	#[test]
	fn optional_and_string_fields_compare_by_value() {
		let rules = parse_and_validate_rules(&serde_json::json!({
			"default_decision": "send",
			"rules": [
				{"field": "bounce_risk_category", "operator": "neq", "value": "medium", "decision": "drop"},
				{"field": "score", "operator": "gte", "value": 72, "decision": "review"}
			]
		}))
		.unwrap();
		assert_eq!(evaluate_rules(&rules, &facts()).0, PolicyDecision::Review);

		let no_bounce_risk = CustomPolicyFacts {
			bounce_risk_category: None,
			..facts()
		};
		assert_eq!(
			evaluate_rules(&rules, &no_bounce_risk).0,
			PolicyDecision::Drop,
			"a missing assessment is not equal to \"medium\""
		);
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
