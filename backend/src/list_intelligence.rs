use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{PgPool, Row};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum PolicyDecision {
	Send,
	Review,
	Suppress,
}

impl PolicyDecision {
	pub fn as_str(self) -> &'static str {
		match self {
			Self::Send => "send",
			Self::Review => "review",
			Self::Suppress => "suppress",
		}
	}
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct PolicyRules {
	pub send: Option<PolicyCondition>,
	pub review: Option<PolicyCondition>,
	pub suppress: Option<PolicyCondition>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct PolicyCondition {
	pub score_min: Option<i16>,
	pub score_max: Option<i16>,
	pub category: Option<Vec<String>>,
	pub safe_to_send: Option<bool>,
	pub reason_codes_any: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct PolicyEvaluationInput<'a> {
	pub score: Option<i16>,
	pub category: Option<&'a str>,
	pub safe_to_send: Option<bool>,
	pub reason_codes: &'a [String],
}

#[derive(Debug, Clone, Default)]
pub struct SegmentRowContext<'a> {
	pub score: Option<i16>,
	pub category: Option<&'a str>,
	pub safe_to_send: Option<bool>,
	pub reason_codes: &'a [String],
	pub is_role_account: Option<bool>,
	pub is_disposable: Option<bool>,
	pub is_catch_all: Option<bool>,
	pub age_days: Option<i64>,
	pub change_type: Option<&'a str>,
	pub policy_decision: Option<PolicyDecision>,
}

#[derive(Debug, Clone)]
pub struct VerificationSnapshot {
	pub task_id: i32,
	pub tenant_id: Uuid,
	pub canonical_email: String,
	pub list_id: Option<i32>,
	pub pipeline_run_id: Option<i64>,
	pub score: Option<i16>,
	pub category: Option<String>,
	pub safe_to_send: Option<bool>,
	pub reason_codes: Option<Vec<String>>,
	pub completed_at: DateTime<Utc>,
}

pub fn validate_policy_rules(value: &Value) -> Result<(), String> {
	let rules: PolicyRules = serde_json::from_value(value.clone())
		.map_err(|err| format!("Invalid policy rules: {err}"))?;
	for (name, condition) in [
		("send", rules.send.as_ref()),
		("review", rules.review.as_ref()),
		("suppress", rules.suppress.as_ref()),
	] {
		if let Some(condition) = condition {
			validate_policy_condition(name, condition)?;
		}
	}
	Ok(())
}

fn validate_policy_condition(name: &str, condition: &PolicyCondition) -> Result<(), String> {
	if let Some(score) = condition.score_min {
		validate_score(name, "score_min", score)?;
	}
	if let Some(score) = condition.score_max {
		validate_score(name, "score_max", score)?;
	}
	if let (Some(min), Some(max)) = (condition.score_min, condition.score_max) {
		if min > max {
			return Err(format!("{name}.score_min cannot exceed {name}.score_max"));
		}
	}
	if condition
		.category
		.as_ref()
		.is_some_and(|values| values.iter().any(|value| value.trim().is_empty()))
	{
		return Err(format!("{name}.category cannot contain empty values"));
	}
	if condition
		.reason_codes_any
		.as_ref()
		.is_some_and(|values| values.iter().any(|value| value.trim().is_empty()))
	{
		return Err(format!(
			"{name}.reason_codes_any cannot contain empty values"
		));
	}
	Ok(())
}

fn validate_score(rule: &str, field: &str, score: i16) -> Result<(), String> {
	if !(0..=100).contains(&score) {
		return Err(format!("{rule}.{field} must be between 0 and 100"));
	}
	Ok(())
}

pub fn evaluate_policy(
	rules_value: &Value,
	input: &PolicyEvaluationInput<'_>,
) -> Result<PolicyDecision, String> {
	let rules: PolicyRules = serde_json::from_value(rules_value.clone())
		.map_err(|err| format!("Invalid policy rules: {err}"))?;
	for (decision, condition) in [
		(PolicyDecision::Suppress, rules.suppress.as_ref()),
		(PolicyDecision::Send, rules.send.as_ref()),
		(PolicyDecision::Review, rules.review.as_ref()),
	] {
		if condition.is_some_and(|condition| condition_matches(condition, input)) {
			return Ok(decision);
		}
	}

	Ok(default_policy_decision(input))
}

fn condition_matches(condition: &PolicyCondition, input: &PolicyEvaluationInput<'_>) -> bool {
	if let Some(min) = condition.score_min {
		if input.score.map(|score| score < min).unwrap_or(true) {
			return false;
		}
	}
	if let Some(max) = condition.score_max {
		if input.score.map(|score| score > max).unwrap_or(true) {
			return false;
		}
	}
	if let Some(categories) = &condition.category {
		if !input
			.category
			.is_some_and(|category| categories.iter().any(|value| value == category))
		{
			return false;
		}
	}
	if let Some(expected) = condition.safe_to_send {
		if input.safe_to_send != Some(expected) {
			return false;
		}
	}
	if let Some(any_codes) = &condition.reason_codes_any {
		if !any_codes
			.iter()
			.any(|code| input.reason_codes.iter().any(|value| value == code))
		{
			return false;
		}
	}
	true
}

fn default_policy_decision(input: &PolicyEvaluationInput<'_>) -> PolicyDecision {
	if input.safe_to_send == Some(true) && input.score.unwrap_or_default() >= 80 {
		return PolicyDecision::Send;
	}
	if input.safe_to_send == Some(false)
		|| input.category == Some("invalid")
		|| input.score.unwrap_or(100) <= 30
	{
		return PolicyDecision::Suppress;
	}
	PolicyDecision::Review
}

pub fn validate_segment_filter(filter: &Value) -> Result<(), String> {
	let object = filter
		.as_object()
		.ok_or_else(|| "Segment filter must be a JSON object".to_string())?;
	for (key, value) in object {
		match key.as_str() {
			"category" | "change_type" => validate_string_or_string_array(key, value)?,
			"safe_to_send" | "is_role_account" | "is_disposable" | "is_catch_all" => {
				if !value.is_boolean() {
					return Err(format!("{key} must be a boolean"));
				}
			}
			"score_min" | "score_max" => {
				let Some(score) = value.as_i64() else {
					return Err(format!("{key} must be an integer"));
				};
				if !(0..=100).contains(&score) {
					return Err(format!("{key} must be between 0 and 100"));
				}
			}
			"freshness_days_max" => {
				let Some(days) = value.as_i64() else {
					return Err("freshness_days_max must be an integer".to_string());
				};
				if days < 0 {
					return Err("freshness_days_max must be non-negative".to_string());
				}
			}
			"reason_codes_any" => validate_string_array(key, value)?,
			"policy_decision" => {
				let Some(decision) = value.as_str() else {
					return Err("policy_decision must be a string".to_string());
				};
				if !matches!(decision, "send" | "review" | "suppress") {
					return Err("policy_decision must be send, review, or suppress".to_string());
				}
			}
			_ => return Err(format!("Unsupported segment filter field: {key}")),
		}
	}

	if let (Some(min), Some(max)) = (
		object.get("score_min").and_then(Value::as_i64),
		object.get("score_max").and_then(Value::as_i64),
	) {
		if min > max {
			return Err("score_min cannot exceed score_max".to_string());
		}
	}

	Ok(())
}

fn validate_string_or_string_array(key: &str, value: &Value) -> Result<(), String> {
	if value.is_string() {
		return Ok(());
	}
	validate_string_array(key, value)
}

fn validate_string_array(key: &str, value: &Value) -> Result<(), String> {
	let Some(values) = value.as_array() else {
		return Err(format!("{key} must be a string array"));
	};
	if values
		.iter()
		.any(|value| value.as_str().is_none_or(|value| value.trim().is_empty()))
	{
		return Err(format!("{key} cannot contain non-string or empty values"));
	}
	Ok(())
}

pub fn segment_matches(filter: &Value, ctx: &SegmentRowContext<'_>) -> bool {
	let Some(object) = filter.as_object() else {
		return false;
	};
	for (key, value) in object {
		let matches = match key.as_str() {
			"category" => string_field_matches(value, ctx.category),
			"safe_to_send" => bool_field_matches(value, ctx.safe_to_send),
			"score_min" => ctx
				.score
				.map(|score| score >= value.as_i64().unwrap_or_default() as i16)
				.unwrap_or(false),
			"score_max" => ctx
				.score
				.map(|score| score <= value.as_i64().unwrap_or_default() as i16)
				.unwrap_or(false),
			"reason_codes_any" => value.as_array().is_some_and(|expected| {
				expected.iter().filter_map(Value::as_str).any(|code| {
					ctx.reason_codes
						.iter()
						.any(|current_code| current_code == code)
				})
			}),
			"is_role_account" => bool_field_matches(value, ctx.is_role_account),
			"is_disposable" => bool_field_matches(value, ctx.is_disposable),
			"is_catch_all" => bool_field_matches(value, ctx.is_catch_all),
			"freshness_days_max" => ctx
				.age_days
				.map(|age_days| age_days <= value.as_i64().unwrap_or_default())
				.unwrap_or(false),
			"change_type" => string_field_matches(value, ctx.change_type),
			"policy_decision" => ctx
				.policy_decision
				.is_some_and(|decision| value.as_str() == Some(decision.as_str())),
			_ => false,
		};
		if !matches {
			return false;
		}
	}
	true
}

fn string_field_matches(expected: &Value, actual: Option<&str>) -> bool {
	match expected {
		Value::String(value) => actual == Some(value.as_str()),
		Value::Array(values) => actual.is_some_and(|actual| {
			values
				.iter()
				.filter_map(Value::as_str)
				.any(|value| value == actual)
		}),
		_ => false,
	}
}

fn bool_field_matches(expected: &Value, actual: Option<bool>) -> bool {
	expected
		.as_bool()
		.is_some_and(|expected| actual == Some(expected))
}

pub fn classify_change(
	previous: Option<&VerificationSnapshot>,
	current: &VerificationSnapshot,
) -> &'static str {
	let Some(previous) = previous else {
		return "new";
	};

	let previous_category = previous.category.as_deref();
	let current_category = current.category.as_deref();
	if previous_category == current_category && previous.safe_to_send == current.safe_to_send {
		return "unchanged";
	}
	if current_category == Some("invalid") && previous_category != Some("invalid") {
		return "became_invalid";
	}
	if current_category == Some("risky") && previous_category != Some("risky") {
		return "became_risky";
	}
	if current.safe_to_send == Some(true) && previous.safe_to_send != Some(true)
		|| current_category == Some("valid") && previous_category != Some("valid")
	{
		return "became_safe";
	}

	let previous_rank = category_rank(previous_category);
	let current_rank = category_rank(current_category);
	if current_rank > previous_rank {
		"improved"
	} else if current_rank < previous_rank {
		"degraded"
	} else if current.score.unwrap_or_default() > previous.score.unwrap_or_default() {
		"improved"
	} else if current.score.unwrap_or_default() < previous.score.unwrap_or_default() {
		"degraded"
	} else {
		"unchanged"
	}
}

fn category_rank(category: Option<&str>) -> i32 {
	match category {
		Some("valid") => 4,
		Some("unknown") => 3,
		Some("risky") => 2,
		Some("invalid") => 1,
		_ => 0,
	}
}

pub fn diff_group_for_change(change_type: &str) -> &'static str {
	match change_type {
		"became_invalid" => "newly_invalid",
		"became_risky" => "newly_risky",
		"became_safe" => "newly_safe",
		"improved" => "improved",
		"degraded" => "degraded",
		_ => "unchanged",
	}
}

pub async fn load_effective_policy_rules_for_list(
	pg_pool: &PgPool,
	tenant_id: Uuid,
	list_id: i32,
) -> Result<Value, sqlx::Error> {
	if let Some(rules) = sqlx::query_scalar::<_, Value>(
		r#"
		SELECT sp.rules
		FROM v1_lists l
		INNER JOIN v1_score_policies sp ON sp.id = l.policy_id AND sp.tenant_id = l.tenant_id
		WHERE l.id = $1 AND l.tenant_id = $2
		"#,
	)
	.bind(list_id)
	.bind(tenant_id)
	.fetch_optional(pg_pool)
	.await?
	{
		return Ok(rules);
	}

	if let Some(rules) = sqlx::query_scalar::<_, Value>(
		r#"
		SELECT sp.rules
		FROM v1_lists l
		INNER JOIN v1_pipelines p ON p.id = l.pipeline_id AND p.tenant_id = l.tenant_id
		INNER JOIN v1_score_policies sp
			ON sp.id = NULLIF(p.policy_config->>'policy_id', '')::BIGINT
			AND sp.tenant_id = p.tenant_id
		WHERE l.id = $1 AND l.tenant_id = $2
		"#,
	)
	.bind(list_id)
	.bind(tenant_id)
	.fetch_optional(pg_pool)
	.await?
	{
		return Ok(rules);
	}

	Ok(sqlx::query_scalar::<_, Value>(
		r#"
		SELECT rules
		FROM v1_score_policies
		WHERE tenant_id = $1 AND is_default = true
		ORDER BY updated_at DESC
		LIMIT 1
		"#,
	)
	.bind(tenant_id)
	.fetch_optional(pg_pool)
	.await?
	.unwrap_or_else(|| serde_json::json!({})))
}

pub async fn record_verification_change_event(
	pg_pool: &PgPool,
	task_result_id: i32,
) -> Result<(), sqlx::Error> {
	let Some(current) = load_snapshot_by_task_id(pg_pool, task_result_id).await? else {
		return Ok(());
	};
	let previous = load_previous_snapshot(pg_pool, &current).await?;
	let change_type = classify_change(previous.as_ref(), &current);
	if change_type == "unchanged" {
		return Ok(());
	}

	let inserted_event = sqlx::query(
		r#"
		INSERT INTO verification_change_events (
			tenant_id, canonical_email, current_task_result_id, previous_task_result_id,
			current_list_id, previous_list_id, current_pipeline_run_id, previous_pipeline_run_id,
			current_score, previous_score, current_category, previous_category,
			current_safe_to_send, previous_safe_to_send, current_reason_codes, previous_reason_codes,
			change_type
		)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
		ON CONFLICT (current_task_result_id) DO NOTHING
		RETURNING id
		"#,
	)
	.bind(current.tenant_id)
	.bind(&current.canonical_email)
	.bind(current.task_id)
	.bind(previous.as_ref().map(|snapshot| snapshot.task_id))
	.bind(current.list_id)
	.bind(previous.as_ref().and_then(|snapshot| snapshot.list_id))
	.bind(current.pipeline_run_id)
	.bind(
		previous
			.as_ref()
			.and_then(|snapshot| snapshot.pipeline_run_id),
	)
	.bind(current.score)
	.bind(previous.as_ref().and_then(|snapshot| snapshot.score))
	.bind(&current.category)
	.bind(
		previous
			.as_ref()
			.and_then(|snapshot| snapshot.category.as_ref()),
	)
	.bind(current.safe_to_send)
	.bind(previous.as_ref().and_then(|snapshot| snapshot.safe_to_send))
	.bind(&current.reason_codes)
	.bind(
		previous
			.as_ref()
			.and_then(|snapshot| snapshot.reason_codes.as_ref()),
	)
	.bind(change_type)
	.fetch_optional(pg_pool)
	.await?;

	let Some(event_row) = inserted_event else {
		return Ok(());
	};
	if change_type == "new" {
		return Ok(());
	}

	let event_id: i64 = event_row.get("id");
	let title = format!("{} changed to {}", current.canonical_email, change_type);
	let body = match previous
		.as_ref()
		.and_then(|snapshot| snapshot.category.as_ref())
	{
		Some(previous_category) => format!(
			"Verification changed from {} to {}",
			previous_category,
			current.category.as_deref().unwrap_or("unknown")
		),
		None => format!(
			"Verification changed to {}",
			current.category.as_deref().unwrap_or("unknown")
		),
	};

	sqlx::query(
		r#"
		INSERT INTO v1_alerts (
			tenant_id, type, status, change_event_id, canonical_email, title, body, metadata
		)
		VALUES ($1, $2, 'unread', $3, $4, $5, $6, $7)
		ON CONFLICT (change_event_id) DO NOTHING
		"#,
	)
	.bind(current.tenant_id)
	.bind(change_type)
	.bind(event_id)
	.bind(&current.canonical_email)
	.bind(title)
	.bind(body)
	.bind(serde_json::json!({
		"current_task_result_id": current.task_id,
		"previous_task_result_id": previous.as_ref().map(|snapshot| snapshot.task_id),
		"current_list_id": current.list_id,
		"previous_list_id": previous.as_ref().and_then(|snapshot| snapshot.list_id),
	}))
	.execute(pg_pool)
	.await?;

	Ok(())
}

async fn load_snapshot_by_task_id(
	pg_pool: &PgPool,
	task_result_id: i32,
) -> Result<Option<VerificationSnapshot>, sqlx::Error> {
	let row = sqlx::query(
		r#"
		SELECT
			id,
			tenant_id,
			canonical_email,
			(extra->>'list_id')::INTEGER AS list_id,
			(extra->>'pipeline_run_id')::BIGINT AS pipeline_run_id,
			score,
			score_category,
			safe_to_send,
			reason_codes,
			completed_at
		FROM v1_task_result
		WHERE id = $1
		  AND tenant_id IS NOT NULL
		  AND canonical_email IS NOT NULL
		  AND is_duplicate = false
		  AND task_state = 'completed'::task_state
		  AND result IS NOT NULL
		  AND completed_at IS NOT NULL
		"#,
	)
	.bind(task_result_id)
	.fetch_optional(pg_pool)
	.await?;

	Ok(row.map(row_to_snapshot))
}

async fn load_previous_snapshot(
	pg_pool: &PgPool,
	current: &VerificationSnapshot,
) -> Result<Option<VerificationSnapshot>, sqlx::Error> {
	let row = sqlx::query(
		r#"
		SELECT
			id,
			tenant_id,
			canonical_email,
			(extra->>'list_id')::INTEGER AS list_id,
			(extra->>'pipeline_run_id')::BIGINT AS pipeline_run_id,
			score,
			score_category,
			safe_to_send,
			reason_codes,
			completed_at
		FROM v1_task_result
		WHERE tenant_id = $1
		  AND canonical_email = $2
		  AND id <> $3
		  AND task_state = 'completed'::task_state
		  AND result IS NOT NULL
		  AND completed_at IS NOT NULL
		  AND (completed_at, id) < ($4, $3)
		ORDER BY completed_at DESC, id DESC
		LIMIT 1
		"#,
	)
	.bind(current.tenant_id)
	.bind(&current.canonical_email)
	.bind(current.task_id)
	.bind(current.completed_at)
	.fetch_optional(pg_pool)
	.await?;

	Ok(row.map(row_to_snapshot))
}

fn row_to_snapshot(row: sqlx::postgres::PgRow) -> VerificationSnapshot {
	VerificationSnapshot {
		task_id: row.get("id"),
		tenant_id: row.get("tenant_id"),
		canonical_email: row.get("canonical_email"),
		list_id: row.get("list_id"),
		pipeline_run_id: row.get("pipeline_run_id"),
		score: row.get("score"),
		category: row.get("score_category"),
		safe_to_send: row.get("safe_to_send"),
		reason_codes: row.get("reason_codes"),
		completed_at: row.get("completed_at"),
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	fn snapshot(category: &str, safe_to_send: bool, score: i16) -> VerificationSnapshot {
		VerificationSnapshot {
			task_id: 1,
			tenant_id: Uuid::nil(),
			canonical_email: "user@example.com".to_string(),
			list_id: None,
			pipeline_run_id: None,
			score: Some(score),
			category: Some(category.to_string()),
			safe_to_send: Some(safe_to_send),
			reason_codes: Some(Vec::new()),
			completed_at: Utc::now(),
		}
	}

	#[test]
	fn classify_missing_prior_as_new() {
		assert_eq!(classify_change(None, &snapshot("valid", true, 95)), "new");
	}

	#[test]
	fn classify_valid_to_invalid() {
		assert_eq!(
			classify_change(
				Some(&snapshot("valid", true, 95)),
				&snapshot("invalid", false, 0)
			),
			"became_invalid"
		);
	}

	#[test]
	fn classify_invalid_to_valid() {
		assert_eq!(
			classify_change(
				Some(&snapshot("invalid", false, 0)),
				&snapshot("valid", true, 95)
			),
			"became_safe"
		);
	}

	#[test]
	fn classify_valid_to_risky() {
		assert_eq!(
			classify_change(
				Some(&snapshot("valid", true, 95)),
				&snapshot("risky", false, 55)
			),
			"became_risky"
		);
	}

	#[test]
	fn classify_same_category_as_unchanged() {
		assert_eq!(
			classify_change(
				Some(&snapshot("valid", true, 95)),
				&snapshot("valid", true, 90)
			),
			"unchanged"
		);
	}

	#[test]
	fn classify_rank_improvement_when_no_named_transition_applies() {
		assert_eq!(
			classify_change(
				Some(&snapshot("risky", false, 40)),
				&snapshot("unknown", false, 70)
			),
			"improved"
		);
	}

	#[test]
	fn classify_rank_degradation_when_no_named_transition_applies() {
		assert_eq!(
			classify_change(
				Some(&snapshot("valid", true, 95)),
				&snapshot("unknown", false, 70)
			),
			"degraded"
		);
	}

	#[test]
	fn diff_group_maps_named_change_types() {
		assert_eq!(diff_group_for_change("became_invalid"), "newly_invalid");
		assert_eq!(diff_group_for_change("became_risky"), "newly_risky");
		assert_eq!(diff_group_for_change("became_safe"), "newly_safe");
		assert_eq!(diff_group_for_change("improved"), "improved");
		assert_eq!(diff_group_for_change("degraded"), "degraded");
		assert_eq!(diff_group_for_change("new"), "unchanged");
	}

	#[test]
	fn policy_rules_evaluate_thresholds() {
		let rules = serde_json::json!({
			"send": {"score_min": 90, "safe_to_send": true},
			"review": {"score_min": 50},
			"suppress": {"score_max": 30}
		});
		assert_eq!(
			evaluate_policy(
				&rules,
				&PolicyEvaluationInput {
					score: Some(95),
					category: Some("valid"),
					safe_to_send: Some(true),
					reason_codes: &[],
				}
			)
			.unwrap(),
			PolicyDecision::Send
		);
		assert_eq!(
			evaluate_policy(
				&rules,
				&PolicyEvaluationInput {
					score: Some(25),
					category: Some("invalid"),
					safe_to_send: Some(false),
					reason_codes: &[],
				}
			)
			.unwrap(),
			PolicyDecision::Suppress
		);
	}

	#[test]
	fn policy_suppress_rule_takes_precedence_over_send_rule() {
		let rules = serde_json::json!({
			"send": {"score_min": 90, "safe_to_send": true},
			"suppress": {"reason_codes_any": ["blocked_domain"]}
		});
		let reason_codes = vec!["blocked_domain".to_string()];
		assert_eq!(
			evaluate_policy(
				&rules,
				&PolicyEvaluationInput {
					score: Some(95),
					category: Some("valid"),
					safe_to_send: Some(true),
					reason_codes: &reason_codes,
				}
			)
			.unwrap(),
			PolicyDecision::Suppress
		);
	}

	#[test]
	fn malformed_policy_rules_are_rejected() {
		let rules = serde_json::json!({"send": {"score_min": 101}});
		assert!(validate_policy_rules(&rules).is_err());
	}

	#[test]
	fn segment_filter_matches_combinations() {
		let filter = serde_json::json!({
			"category": "risky",
			"reason_codes_any": ["catch_all"],
			"policy_decision": "review",
			"freshness_days_max": 30
		});
		let codes = vec!["catch_all".to_string()];
		assert!(validate_segment_filter(&filter).is_ok());
		assert!(segment_matches(
			&filter,
			&SegmentRowContext {
				score: Some(55),
				category: Some("risky"),
				safe_to_send: Some(false),
				reason_codes: &codes,
				is_role_account: Some(false),
				is_disposable: Some(false),
				is_catch_all: Some(true),
				age_days: Some(12),
				change_type: Some("became_risky"),
				policy_decision: Some(PolicyDecision::Review),
			}
		));
	}

	#[test]
	fn segment_filter_matches_boolean_fields() {
		let filter = serde_json::json!({
			"is_role_account": true,
			"is_disposable": false,
			"is_catch_all": true,
			"safe_to_send": false
		});
		let codes = Vec::new();
		assert!(validate_segment_filter(&filter).is_ok());
		assert!(segment_matches(
			&filter,
			&SegmentRowContext {
				score: Some(55),
				category: Some("risky"),
				safe_to_send: Some(false),
				reason_codes: &codes,
				is_role_account: Some(true),
				is_disposable: Some(false),
				is_catch_all: Some(true),
				age_days: Some(1),
				change_type: Some("became_risky"),
				policy_decision: Some(PolicyDecision::Review),
			}
		));
	}

	#[test]
	fn segment_filter_rejects_invalid_ranges() {
		assert!(
			validate_segment_filter(&serde_json::json!({"score_min": 90, "score_max": 40}))
				.is_err()
		);
		assert!(validate_segment_filter(&serde_json::json!({"freshness_days_max": -1})).is_err());
	}

	#[test]
	fn unknown_segment_filter_field_is_rejected() {
		assert!(validate_segment_filter(&serde_json::json!({"sql": "score > 10"})).is_err());
	}
}
