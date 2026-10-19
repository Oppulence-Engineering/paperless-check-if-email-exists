use crate::bounce_risk::{BounceRiskAssessment, BounceRiskRequestContext, SignalBundle};
use crate::config::BackendConfig;
use crate::decision::engine::{evaluate as evaluate_decision, DecisionInput};
use crate::decision::types::{PolicyEvaluation, PolicyMode, Recommendation};
use crate::scoring::{
	catch_all_severity, compute_freshness_at, compute_score, compute_score_with_context,
	provider_reputation_context, DomainSignalContext, EmailScore, PatternContext, ScoreInsights,
	ScoringContext, TenantHistoryContext,
};
use check_if_email_exists::{CheckEmailOutput, LOG_TARGET};
use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::{Map, Value};
use sqlx::{PgPool, Row};
use std::ops::Deref;
use tracing::warn;
use uuid::Uuid;

pub fn scored_json(output: &CheckEmailOutput) -> Result<Value, serde_json::Error> {
	let score = compute_score(output);
	scored_json_with_score(output, &score)
}

pub fn scored_json_with_score(
	output: &CheckEmailOutput,
	email_score: &EmailScore,
) -> Result<Value, serde_json::Error> {
	scored_json_with_score_and_insights(output, email_score, None)
}

pub fn scored_json_with_score_and_insights(
	output: &CheckEmailOutput,
	email_score: &EmailScore,
	insights: Option<&ScoreInsights>,
) -> Result<Value, serde_json::Error> {
	let mut scored = serde_json::to_value(output)?;
	let mut score = serde_json::to_value(email_score)?;

	// Surface domain typo suggestion in score object (#31)
	if let Some(suggestion) = &output.syntax.suggestion {
		if let Some(obj) = score.as_object_mut() {
			obj.insert(
				"domain_suggestion".into(),
				Value::String(suggestion.clone()),
			);
		}
	}

	// Surface canonical (normalized) email in score object (#32)
	let canonical = crate::http::v1::lists::canonicalize::canonicalize_email(&output.input);
	if let Some(ref canon) = canonical {
		if canon != &output.input {
			if let Some(obj) = score.as_object_mut() {
				obj.insert("normalized_email".into(), Value::String(canon.clone()));
			}
		}
	}

	// Add catch-all severity tier (#30). The same tier drives the score
	// penalty and safe_to_send in `compute_score`.
	if let Some(tier) = catch_all_severity(&email_score.signals) {
		if let Some(obj) = score.as_object_mut() {
			obj.insert(
				"catch_all_severity".into(),
				Value::String(tier.as_str().to_string()),
			);
		}
	}

	if let Some(insights) = insights {
		inject_score_insights(&mut score, insights)?;
		// Contextual insights can soften or raise the two-tier catch-all grade.
		if let Some(catch_all) = &insights.catch_all {
			if let Some(obj) = score.as_object_mut() {
				let tier = match catch_all.severity {
					crate::scoring::InsightCatchAllSeverity::Low => "low",
					crate::scoring::InsightCatchAllSeverity::Medium => "medium",
					crate::scoring::InsightCatchAllSeverity::High => "high",
				};
				obj.insert("catch_all_severity".into(), Value::String(tier.to_string()));
			}
		}
	}

	match &mut scored {
		Value::Object(map) => {
			map.insert("score".into(), score);
			Ok(scored)
		}
		_ => {
			let mut map = Map::new();
			map.insert("result".into(), scored);
			map.insert("score".into(), score);
			Ok(Value::Object(map))
		}
	}
}

fn inject_score_insights(
	score: &mut Value,
	insights: &ScoreInsights,
) -> Result<(), serde_json::Error> {
	let Some(obj) = score.as_object_mut() else {
		return Ok(());
	};
	obj.insert("confidence".into(), Value::from(insights.confidence));
	obj.insert(
		"confidence_level".into(),
		serde_json::to_value(&insights.confidence_level)?,
	);
	obj.insert(
		"confidence_factors".into(),
		serde_json::to_value(&insights.confidence_factors)?,
	);
	if let Some(catch_all) = &insights.catch_all {
		obj.insert("catch_all".into(), serde_json::to_value(catch_all)?);
	}
	if let Some(partial_confidence) = &insights.partial_confidence {
		obj.insert(
			"partial_confidence".into(),
			serde_json::to_value(partial_confidence)?,
		);
	}
	Ok(())
}

pub fn scored_response(output: &CheckEmailOutput) -> Result<Vec<u8>, serde_json::Error> {
	serde_json::to_vec(&scored_json(output)?)
}

pub fn scored_json_with_freshness(
	output: &CheckEmailOutput,
	completed_at: Option<DateTime<Utc>>,
) -> Result<Value, serde_json::Error> {
	let mut value = scored_json(output)?;
	if let Some(ts) = completed_at {
		inject_freshness_into_result(&mut value, ts);
	}
	Ok(value)
}

pub fn scored_response_fresh(output: &CheckEmailOutput) -> Result<Vec<u8>, serde_json::Error> {
	serde_json::to_vec(&scored_json_with_freshness(output, Some(Utc::now()))?)
}

pub fn inject_freshness_into_result(result: &mut Value, completed_at: DateTime<Utc>) {
	inject_freshness_into_result_at(result, completed_at, Utc::now());
}

pub fn inject_freshness_into_result_at(
	result: &mut Value,
	completed_at: DateTime<Utc>,
	now: DateTime<Utc>,
) {
	if let Some(score_obj) = result.get_mut("score").and_then(Value::as_object_mut) {
		let info = compute_freshness_at(completed_at, now);
		score_obj.insert("verified_at".into(), Value::String(info.verified_at));
		score_obj.insert("age_days".into(), Value::from(info.age_days));
		score_obj.insert(
			"freshness".into(),
			serde_json::to_value(&info.freshness).unwrap_or(Value::Null),
		);
	}
}

#[derive(Debug)]
pub struct PreparedVerificationResponse {
	pub json: Value,
	pub body: Vec<u8>,
	pub score: EmailScore,
	pub canonical_email: Option<String>,
	pub bounce_risk: Option<BounceRiskAssessment>,
	pub bounce_risk_signals: Option<Value>,
	pub recommendation: Option<Recommendation>,
	pub policy_evaluation: Option<PolicyEvaluation>,
}

impl Serialize for PreparedVerificationResponse {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		self.json.serialize(serializer)
	}
}

#[derive(Debug)]
pub struct PreparedCheckEmailSuccess {
	pub output: CheckEmailOutput,
	pub response: PreparedVerificationResponse,
}

impl Deref for PreparedCheckEmailSuccess {
	type Target = CheckEmailOutput;

	fn deref(&self) -> &Self::Target {
		&self.output
	}
}

impl Serialize for PreparedCheckEmailSuccess {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		self.response.serialize(serializer)
	}
}

pub async fn prepare_check_email_success(
	config: &BackendConfig,
	output: CheckEmailOutput,
	tenant_id: Option<Uuid>,
	completed_at: DateTime<Utc>,
	allow_external_enrichment: bool,
) -> Result<PreparedCheckEmailSuccess, anyhow::Error> {
	let response = prepare_verification_response(
		config,
		&output,
		tenant_id,
		completed_at,
		allow_external_enrichment,
	)
	.await?;

	Ok(PreparedCheckEmailSuccess { output, response })
}

pub async fn prepare_verification_response(
	config: &BackendConfig,
	output: &CheckEmailOutput,
	tenant_id: Option<Uuid>,
	completed_at: DateTime<Utc>,
	allow_external_enrichment: bool,
) -> Result<PreparedVerificationResponse, anyhow::Error> {
	let canonical_email = crate::http::v1::lists::canonicalize::canonicalize_email(&output.input);
	let read_pool = config.get_read_pg_pool();
	let write_pool = config.get_pg_pool();
	let bounce_risk_service = config.get_bounce_risk_service();
	let warn_enrichment_failed = |error: &anyhow::Error| {
		let email_domain = output
			.input
			.rsplit_once('@')
			.map(|(_, domain)| domain)
			.unwrap_or("unknown");
		warn!(
			target: LOG_TARGET,
			error = ?error,
			email_domain = %email_domain,
			tenant_id = ?tenant_id,
			"Bounce-risk enrichment failed, continuing without enrichment"
		);
	};

	let collected_signals = bounce_risk_service
		.collect_signals(
			output,
			read_pool.as_ref(),
			write_pool.as_ref(),
			&BounceRiskRequestContext {
				tenant_id,
				completed_at,
				allow_external_enrichment,
			},
		)
		.await
		.unwrap_or_else(|error| {
			warn_enrichment_failed(&error);
			None
		});

	let scoring_context = build_scoring_context(
		read_pool.as_ref(),
		tenant_id,
		output,
		completed_at,
		collected_signals
			.as_ref()
			.map(|collected| &collected.signals),
	)
	.await;
	let score_computation = compute_score_with_context(output, &scoring_context);
	let email_score = score_computation.score.clone();

	// Bounce risk is scored against the same contextual score that is stored
	// and shown, so its history comparison and overrides agree with it.
	let bounce_risk_result = collected_signals.and_then(|collected| {
		bounce_risk_service
			.assess(collected, &email_score, completed_at)
			.map_err(|error| warn_enrichment_failed(&error))
			.ok()
	});
	let mut value = scored_json_with_score_and_insights(
		output,
		&email_score,
		Some(&score_computation.insights),
	)?;
	inject_freshness_into_result(&mut value, completed_at);

	let (bounce_risk, bounce_risk_signals) = if let Some(result) = bounce_risk_result {
		if let Some(result_obj) = value.as_object_mut() {
			result_obj.insert(
				"bounce_risk".into(),
				serde_json::to_value(&result.assessment)?,
			);
		}
		(
			Some(result.assessment),
			Some(serde_json::to_value(&result.signals)?),
		)
	} else {
		(None, None)
	};

	let evaluated_at = Utc::now();
	// ponytail: Read the tenant default per check; carry it in task context if DB load grows.
	let policy_mode = if let (Some(tenant_id), Some(pool)) =
		(tenant_id, write_pool.as_ref().or(read_pool.as_ref()))
	{
		let configured: String =
			sqlx::query_scalar("SELECT default_policy_mode FROM tenants WHERE id = $1")
				.bind(tenant_id)
				.fetch_one(pool)
				.await?;
		serde_json::from_value(serde_json::Value::String(configured))?
	} else {
		PolicyMode::Deliverability
	};
	// A single check must honour the tenant's suppression list and bounce
	// history like list remediation does, or a suppressed address gets "Send".
	let active_suppression = match (tenant_id, read_pool.as_ref()) {
		(Some(tenant_id), Some(pool)) => {
			crate::suppression::has_active_suppression(pool, tenant_id, &output.input).await?
		}
		_ => false,
	};
	let decision_input = DecisionInput {
		score: &email_score,
		completed_at,
		evaluated_at,
		policy_mode,
		policy_profile_key: None,
		domain_suggestion: output.syntax.suggestion.as_deref(),
		suggested_email: output.syntax.suggestion.clone(),
		bounce_risk: bounce_risk.as_ref(),
		active_suppression,
		previous_hard_bounce: scoring_context.outcomes.has_hard_bounce,
	};
	let (recommendation, policy_evaluation) = evaluate_decision(&decision_input);
	if let Some(result_obj) = value.as_object_mut() {
		result_obj.insert(
			"recommendation".into(),
			serde_json::to_value(&recommendation)?,
		);
		result_obj.insert(
			"policy_evaluation".into(),
			serde_json::to_value(&policy_evaluation)?,
		);
	}

	let body = serde_json::to_vec(&value)?;

	Ok(PreparedVerificationResponse {
		json: value,
		body,
		score: email_score,
		canonical_email,
		bounce_risk,
		bounce_risk_signals,
		recommendation: Some(recommendation),
		policy_evaluation: Some(policy_evaluation),
	})
}

async fn build_scoring_context(
	read_pool: Option<&PgPool>,
	tenant_id: Option<Uuid>,
	output: &CheckEmailOutput,
	completed_at: DateTime<Utc>,
	bounce_signals: Option<&SignalBundle>,
) -> ScoringContext {
	let mut context = ScoringContext {
		provider_reputation: provider_reputation_context(
			output.provider.as_ref(),
			output.provider_confidence.as_ref(),
		),
		..Default::default()
	};

	if let Some(signals) = bounce_signals {
		context.domain = DomainSignalContext {
			domain_age_days: signals.domain_age_days,
			website_present: signals.website_present,
			has_spf: signals.has_spf,
			has_dkim: signals.has_dkim,
			has_dmarc: signals.has_dmarc,
		};
	}

	if let (Some(pool), Some(tenant_id)) = (read_pool, tenant_id) {
		if let Err(error) =
			enrich_tenant_scoring_context(pool, tenant_id, output, completed_at, &mut context).await
		{
			warn!(
				target: LOG_TARGET,
				error = ?error,
				tenant_id = %tenant_id,
				email = %output.input,
				"Tenant scoring context failed, continuing without history"
			);
		}

		let canonical = crate::http::v1::lists::canonicalize::canonicalize_email(&output.input)
			.unwrap_or_else(|| output.input.trim().to_lowercase());
		context.outcomes = crate::outcomes::enrich_outcome_context(
			Some(pool),
			Some(tenant_id),
			&canonical,
			completed_at,
		)
		.await;
	}

	context
}

async fn enrich_tenant_scoring_context(
	pool: &PgPool,
	tenant_id: Uuid,
	output: &CheckEmailOutput,
	completed_at: DateTime<Utc>,
	context: &mut ScoringContext,
) -> Result<(), anyhow::Error> {
	let canonical_email = crate::http::v1::lists::canonicalize::canonicalize_email(&output.input);
	let lowered_email = output.input.trim().to_lowercase();

	let rows = sqlx::query(
		r#"
		SELECT
			COALESCE(score_category, result->'score'->>'category') AS category,
			safe_to_send,
			score,
			completed_at
		FROM v1_task_result
		WHERE tenant_id = $1
		  AND task_state = 'completed'
		  AND result IS NOT NULL
		  AND is_duplicate = false
		  AND completed_at IS NOT NULL
		  AND completed_at >= $4 - INTERVAL '180 days'
		  AND completed_at < $4
		  AND (
			($2::TEXT IS NOT NULL AND canonical_email = $2)
			OR lower(COALESCE(canonical_email, payload->'input'->>'to_email', result->>'input', '')) = $3
		  )
		ORDER BY completed_at DESC
		LIMIT 20
		"#,
	)
	.bind(tenant_id)
	.bind(&canonical_email)
	.bind(&lowered_email)
	.bind(completed_at)
	.fetch_all(pool)
	.await?;

	let rows: Vec<TenantHistoryRow> = rows
		.iter()
		.map(|row| TenantHistoryRow {
			category: row.get("category"),
			safe_to_send: row.get("safe_to_send"),
			score: row.get("score"),
			completed_at: row.get("completed_at"),
		})
		.collect();
	context.tenant_history = summarize_tenant_history(&rows, completed_at);

	let domain = normalized_domain(output);
	let Some(pattern) = infer_local_pattern(&lowered_email) else {
		return Ok(());
	};
	let Some(domain) = domain else {
		return Ok(());
	};

	let domain_like = format!("%@{}", domain);
	let pattern_rows = sqlx::query(
		r#"
		SELECT
			lower(COALESCE(canonical_email, payload->'input'->>'to_email', result->>'input', '')) AS email,
			COALESCE(score_category, result->'score'->>'category') AS category,
			safe_to_send,
			score
		FROM v1_task_result
		WHERE tenant_id = $1
		  AND task_state = 'completed'
		  AND result IS NOT NULL
		  AND is_duplicate = false
		  AND completed_at IS NOT NULL
		  AND completed_at >= $2 - INTERVAL '180 days'
		  AND completed_at < $2
		  AND lower(COALESCE(canonical_email, payload->'input'->>'to_email', result->>'input', '')) LIKE $3
		ORDER BY completed_at DESC
		LIMIT 250
		"#,
	)
	.bind(tenant_id)
	.bind(completed_at)
	.bind(&domain_like)
	.fetch_all(pool)
	.await?;

	let mut pattern_context = PatternContext {
		pattern: Some(pattern.clone()),
		..Default::default()
	};
	for row in pattern_rows {
		let email = row.get::<String, _>("email");
		if email == lowered_email || canonical_email.as_deref() == Some(email.as_str()) {
			continue;
		}
		let category = row.get::<Option<String>, _>("category");
		let safe_to_send = row.get::<Option<bool>, _>("safe_to_send").unwrap_or(false);
		let score = row.get::<Option<i16>, _>("score").unwrap_or(0);
		let verified = safe_to_send || category.as_deref() == Some("valid") || score >= 80;
		if !verified {
			continue;
		}
		pattern_context.verified_domain_count_180d += 1;
		if infer_local_pattern(&email).as_deref() == Some(pattern.as_str()) {
			pattern_context.verified_same_pattern_count_180d += 1;
		}
	}
	context.pattern = pattern_context;

	Ok(())
}

/// One earlier completed verification of the same address.
struct TenantHistoryRow {
	category: Option<String>,
	safe_to_send: Option<bool>,
	score: Option<i16>,
	completed_at: Option<DateTime<Utc>>,
}

/// Rows must be newest first. Only results that said something about the
/// mailbox count: a safe result is positive evidence and an invalid one
/// conflicts. Unknown results carry no evidence either way, so rechecking an
/// address that could not be verified neither raises nor lowers its score.
fn summarize_tenant_history(
	rows: &[TenantHistoryRow],
	completed_at: DateTime<Utc>,
) -> TenantHistoryContext {
	let mut history = TenantHistoryContext::default();
	for row in rows {
		history.total_count_180d += 1;
		let category = row.category.as_deref();
		let is_safe = row.safe_to_send == Some(true)
			|| category == Some("valid")
			|| row.score.map(|score| score >= 80).unwrap_or(false);
		if is_safe {
			history.safe_count_180d += 1;
			if history.latest_safe_days_ago.is_none() {
				history.latest_safe_days_ago = row
					.completed_at
					.map(|previous| (completed_at - previous).num_days().max(0));
			}
		} else if category == Some("invalid") {
			history.inconsistent_count_180d += 1;
		}
	}
	history
}

fn normalized_domain(output: &CheckEmailOutput) -> Option<String> {
	if !output.syntax.domain.trim().is_empty() {
		return Some(output.syntax.domain.trim().to_lowercase());
	}
	output
		.input
		.rsplit_once('@')
		.map(|(_, domain)| domain.trim().to_lowercase())
		.filter(|domain| !domain.is_empty())
}

fn infer_local_pattern(email: &str) -> Option<String> {
	let local = email.split('@').next()?.trim().to_lowercase();
	if local.is_empty() {
		return None;
	}
	if local.matches('.').count() == 1 {
		let mut parts = local.split('.');
		if plausible_name_part(parts.next()?) && plausible_name_part(parts.next()?) {
			return Some("first.last".to_string());
		}
	}
	if local.matches('_').count() == 1 {
		let mut parts = local.split('_');
		if plausible_name_part(parts.next()?) && plausible_name_part(parts.next()?) {
			return Some("first_last".to_string());
		}
	}
	if local.matches('-').count() == 1 {
		let mut parts = local.split('-');
		if plausible_name_part(parts.next()?) && plausible_name_part(parts.next()?) {
			return Some("first-last".to_string());
		}
	}
	if local.len() >= 6 && local.chars().all(|ch| ch.is_ascii_alphanumeric()) {
		return Some("firstlast".to_string());
	}
	None
}

fn plausible_name_part(value: &str) -> bool {
	value.len() >= 2 && value.chars().all(|ch| ch.is_ascii_alphanumeric())
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::scoring::{CatchAllScore, ConfidenceLevel, InsightCatchAllSeverity};
	use check_if_email_exists::{
		smtp::SmtpDetails, syntax::SyntaxDetails, CheckEmailOutput, Reachable,
	};

	#[test]
	fn tenant_history_counts_only_evidence_about_the_mailbox() {
		let now = Utc::now();
		let row = |category: Option<&str>, safe_to_send, score, days_ago| TenantHistoryRow {
			category: category.map(str::to_string),
			safe_to_send,
			score,
			completed_at: Some(now - chrono::Duration::days(days_ago)),
		};

		// Unknown results and rows without a result say nothing about the
		// mailbox, so they are neither positive nor conflicting evidence.
		let unknown_only = summarize_tenant_history(
			&[
				row(Some("unknown"), Some(false), Some(33), 1),
				row(None, None, None, 2),
			],
			now,
		);
		assert_eq!(unknown_only.total_count_180d, 2);
		assert_eq!(unknown_only.safe_count_180d, 0);
		assert_eq!(unknown_only.inconsistent_count_180d, 0);
		assert_eq!(unknown_only.latest_safe_days_ago, None);

		// Recency is measured from the newest safe result, not the newest row.
		let mixed = summarize_tenant_history(
			&[
				row(Some("unknown"), Some(false), Some(33), 1),
				row(Some("valid"), Some(true), Some(95), 12),
				row(Some("invalid"), Some(false), Some(0), 40),
			],
			now,
		);
		assert_eq!(mixed.safe_count_180d, 1);
		assert_eq!(mixed.inconsistent_count_180d, 1);
		assert_eq!(mixed.latest_safe_days_ago, Some(12));
	}

	#[test]
	fn scored_json_appends_score() {
		let value = scored_json(&CheckEmailOutput::default()).unwrap();
		assert!(value.get("score").is_some());
	}

	#[test]
	fn domain_suggestion_surfaced_in_score() {
		let mut output = CheckEmailOutput::default();
		output.syntax.suggestion = Some("user@gmail.com".to_string());
		let value = scored_json(&output).unwrap();
		let score = value.get("score").unwrap();
		assert_eq!(
			score.get("domain_suggestion").and_then(|v| v.as_str()),
			Some("user@gmail.com")
		);
	}

	#[test]
	fn no_domain_suggestion_when_none() {
		let output = CheckEmailOutput::default();
		let value = scored_json(&output).unwrap();
		let score = value.get("score").unwrap();
		assert!(score.get("domain_suggestion").is_none());
	}

	#[test]
	fn normalized_email_surfaced_when_different() {
		let mut output = CheckEmailOutput::default();
		output.input = "User+tag@Gmail.com".to_string();
		output.syntax = SyntaxDetails {
			address: None,
			domain: "gmail.com".to_string(),
			is_valid_syntax: true,
			username: "user+tag".to_string(),
			normalized_email: Some("user+tag@gmail.com".to_string()),
			suggestion: None,
		};
		let value = scored_json(&output).unwrap();
		let score = value.get("score").unwrap();
		assert_eq!(
			score.get("normalized_email").and_then(|v| v.as_str()),
			Some("user@gmail.com")
		);
	}

	#[test]
	fn catch_all_severity_high_for_corporate() {
		let mut output = CheckEmailOutput::default();
		output.input = "user@company.com".to_string();
		output.is_reachable = Reachable::Risky;
		output.mx = Ok(crate::sandbox::sandbox_mx("company.com."));
		output.smtp = Ok(SmtpDetails {
			can_connect_smtp: true,
			has_full_inbox: false,
			is_catch_all: true,
			is_deliverable: true,
			is_disabled: false,
		});
		let value = scored_json(&output).unwrap();
		let score = value.get("score").unwrap();
		assert_eq!(
			score.get("catch_all_severity").and_then(|v| v.as_str()),
			Some("high")
		);
	}

	#[test]
	fn wire_response_never_contradicts_itself_on_catch_all() {
		// The serialised response carries the tier and safe_to_send side by
		// side. A client that reads either one must reach the same conclusion,
		// so "high" and safe_to_send=true must never appear together.
		for (is_b2c, is_role_account, has_full_inbox) in [
			(true, false, false),
			(false, false, false),
			(true, true, false),
			(true, false, true),
			(false, true, false),
		] {
			let mut output = CheckEmailOutput {
				input: "user@company.com".to_string(),
				is_reachable: Reachable::Safe,
				mx: Ok(crate::sandbox::sandbox_mx("company.com.")),
				smtp: Ok(SmtpDetails {
					can_connect_smtp: true,
					has_full_inbox,
					is_catch_all: true,
					is_deliverable: true,
					is_disabled: false,
				}),
				misc: Ok(check_if_email_exists::misc::MiscDetails {
					is_b2c,
					is_role_account,
					..Default::default()
				}),
				..Default::default()
			};
			output.syntax.is_valid_syntax = true;

			let value = scored_json(&output).unwrap();
			let score = value.get("score").unwrap();
			let tier = score.get("catch_all_severity").and_then(|v| v.as_str());
			let safe = score
				.get("safe_to_send")
				.and_then(|v| v.as_bool())
				.expect("safe_to_send is required by the schema");

			assert!(
				tier.is_some(),
				"every catch-all result must carry a tier: b2c={}",
				is_b2c
			);
			if tier == Some("high") {
				assert!(
					!safe,
					"high tier reported safe_to_send=true: b2c={} role={} full={}",
					is_b2c, is_role_account, has_full_inbox
				);
			}
		}
	}

	#[test]
	fn no_catch_all_severity_when_not_catch_all() {
		let output = CheckEmailOutput::default();
		let value = scored_json(&output).unwrap();
		let score = value.get("score").unwrap();
		assert!(score.get("catch_all_severity").is_none());
	}

	#[test]
	fn contextual_score_fields_are_serialized_under_score() {
		let mut output = CheckEmailOutput::default();
		output.input = "user@company.com".to_string();
		output.is_reachable = Reachable::Risky;
		output.smtp = Ok(SmtpDetails {
			can_connect_smtp: true,
			has_full_inbox: false,
			is_catch_all: true,
			is_deliverable: true,
			is_disabled: false,
		});
		let email_score = compute_score(&output);
		let insights = ScoreInsights {
			confidence: 72,
			confidence_level: ConfidenceLevel::Medium,
			confidence_factors: vec!["pattern:first.last:verified_matches".to_string()],
			catch_all: Some(CatchAllScore {
				severity: InsightCatchAllSeverity::Low,
				confidence: 72,
				factors: vec!["pattern:first.last:verified_matches".to_string()],
			}),
			partial_confidence: None,
		};

		let value =
			scored_json_with_score_and_insights(&output, &email_score, Some(&insights)).unwrap();
		let score = value.get("score").unwrap();
		assert_eq!(
			score.get("confidence").and_then(|value| value.as_i64()),
			Some(72)
		);
		assert_eq!(
			score
				.get("confidence_level")
				.and_then(|value| value.as_str()),
			Some("medium")
		);
		assert_eq!(
			score
				.get("catch_all_severity")
				.and_then(|value| value.as_str()),
			Some("low")
		);
		assert!(score.get("catch_all").is_some());
	}

	#[tokio::test]
	async fn prepared_response_includes_bounce_risk_when_enabled() {
		let mut config = BackendConfig::empty();
		config.bounce_risk.enabled = true;
		config.refresh_bounce_risk_service();
		let output = CheckEmailOutput::default();
		let response = prepare_verification_response(&config, &output, None, Utc::now(), false)
			.await
			.unwrap();
		assert!(response.json.get("bounce_risk").is_some());
	}

	#[tokio::test]
	async fn prepared_response_ignores_bounce_risk_enrichment_failures() {
		let mut config = BackendConfig::empty();
		config.bounce_risk.enabled = true;
		config.bounce_risk.config_path = std::env::temp_dir().to_string_lossy().to_string();
		config.refresh_bounce_risk_service();

		let output = CheckEmailOutput::default();
		let response = prepare_verification_response(&config, &output, None, Utc::now(), false)
			.await
			.unwrap();
		assert!(response.json.get("bounce_risk").is_none());
	}

	#[tokio::test]
	async fn prepared_response_includes_recommendation_and_policy_evaluation() {
		let config = BackendConfig::empty();
		let output = CheckEmailOutput::default();
		let response = prepare_verification_response(&config, &output, None, Utc::now(), false)
			.await
			.unwrap();
		assert!(response.json.get("recommendation").is_some());
		assert!(response.json.get("policy_evaluation").is_some());
		assert!(response.recommendation.is_some());
		assert!(response.policy_evaluation.is_some());
	}
}
