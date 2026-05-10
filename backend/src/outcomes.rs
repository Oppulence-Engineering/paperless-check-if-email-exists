// Reacher - Email Verification
// Copyright (C) 2018-2023 Reacher

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! Campaign outcome feedback loop.
//!
//! Customers POST outcomes from their ESPs (delivered, hard_bounce, soft_bounce,
//! complaint, open, click, unsubscribe) and we use those signals to:
//!
//! 1. Auto-add hard bounces / complaints / unsubscribes to the suppression list
//!    according to a per-tenant `OutcomePolicyRules`.
//! 2. Enrich `ScoringContext` so future verifications of the same address use
//!    real-world ground truth instead of just SMTP heuristics.

use crate::http::v1::lists::canonicalize::canonicalize_email;
use check_if_email_exists::LOG_TARGET;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use uuid::Uuid;

/// SQL enum mirror. Must stay in sync with `outcome_type` in
/// `migrations/20260510000001_campaign_outcomes.up.sql`.
#[derive(
	Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type, utoipa::ToSchema,
)]
#[sqlx(type_name = "outcome_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum OutcomeType {
	Delivered,
	HardBounce,
	SoftBounce,
	Complaint,
	Open,
	Click,
	Unsubscribe,
}

impl OutcomeType {
	pub fn as_str(&self) -> &'static str {
		match self {
			OutcomeType::Delivered => "delivered",
			OutcomeType::HardBounce => "hard_bounce",
			OutcomeType::SoftBounce => "soft_bounce",
			OutcomeType::Complaint => "complaint",
			OutcomeType::Open => "open",
			OutcomeType::Click => "click",
			OutcomeType::Unsubscribe => "unsubscribe",
		}
	}
}

/// One outcome event ingested via API or CSV.
#[derive(Debug, Clone, Deserialize, Serialize, utoipa::ToSchema)]
pub struct IngestOutcome {
	pub email: String,
	#[serde(rename = "type")]
	pub outcome_type: OutcomeType,
	pub occurred_at: DateTime<Utc>,
	#[serde(default)]
	pub source: Option<String>,
	#[serde(default)]
	pub campaign_id: Option<String>,
	#[serde(default)]
	pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct IngestOutcomesRequest {
	pub outcomes: Vec<IngestOutcome>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct IngestOutcomesResponse {
	pub accepted: usize,
	pub rejected: usize,
	pub suppressed: usize,
	pub policy_id: i64,
	pub errors: Vec<IngestRowError>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct IngestRowError {
	pub index: usize,
	pub email: String,
	pub message: String,
}

/// Decoded outcome-policy rules. Loose JSON in storage, this is the typed view.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct OutcomePolicyRules {
	pub hard_bounce: RuleAction,
	pub complaint: RuleAction,
	pub soft_bounce: RuleAction,
	pub unsubscribe: RuleAction,
	pub delivered: RuleAction,
	pub open: RuleAction,
	pub click: RuleAction,
	pub outcome_ttl_days: i32,
}

impl Default for OutcomePolicyRules {
	fn default() -> Self {
		default_outcome_policy_rules()
	}
}

pub fn default_outcome_policy_rules() -> OutcomePolicyRules {
	OutcomePolicyRules {
		hard_bounce: RuleAction::Suppress {
			score_override: Some("invalid".to_string()),
		},
		complaint: RuleAction::SuppressAndUnsubscribe {
			score_override: Some("invalid".to_string()),
		},
		soft_bounce: RuleAction::SuppressAfter {
			threshold_count: 3,
			threshold_window_days: 30,
		},
		unsubscribe: RuleAction::Suppress {
			score_override: None,
		},
		delivered: RuleAction::ScoreBoost { boost: 5 },
		open: RuleAction::ScoreBoost { boost: 3 },
		click: RuleAction::ScoreBoost { boost: 8 },
		outcome_ttl_days: 90,
	}
}

pub fn default_outcome_policy_rules_json() -> serde_json::Value {
	serde_json::to_value(default_outcome_policy_rules())
		.unwrap_or_else(|_| serde_json::json!({}))
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum RuleAction {
	Suppress {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		score_override: Option<String>,
	},
	SuppressAndUnsubscribe {
		#[serde(default, skip_serializing_if = "Option::is_none")]
		score_override: Option<String>,
	},
	SuppressAfter {
		threshold_count: i64,
		threshold_window_days: i32,
	},
	ScoreBoost {
		boost: i16,
	},
	Informational,
}

impl Default for RuleAction {
	fn default() -> Self {
		RuleAction::Informational
	}
}

/// Aggregated outcome signal for a single (tenant, canonical_email) — fed into
/// `ScoringContext` so future verifications reflect ground truth.
#[derive(Debug, Default, Clone)]
pub struct OutcomeContext {
	pub has_hard_bounce: bool,
	pub has_complaint: bool,
	pub has_unsubscribe: bool,
	pub delivered_count: i64,
	pub open_count: i64,
	pub click_count: i64,
	pub soft_bounce_count: i64,
	pub latest_engagement_days_ago: Option<i64>,
}

impl OutcomeContext {
	/// True when an outcome is so definitive it should override SMTP-derived score.
	pub fn forces_invalid(&self) -> bool {
		self.has_hard_bounce || self.has_complaint
	}

	pub fn engagement_boost(&self) -> i16 {
		let mut boost: i16 = 0;
		if self.delivered_count > 0 {
			boost = boost.saturating_add(5);
		}
		if self.open_count > 0 {
			boost = boost.saturating_add(3);
		}
		if self.click_count > 0 {
			boost = boost.saturating_add(8);
		}
		boost.min(15)
	}
}

// ----------------------------------------------------------------------------
// Ingestion
// ----------------------------------------------------------------------------

pub struct IngestSummary {
	pub accepted: usize,
	pub rejected: usize,
	pub suppressed: usize,
	pub errors: Vec<IngestRowError>,
}

pub async fn ingest_outcomes(
	pg_pool: &PgPool,
	tenant_id: Uuid,
	policy: &OutcomePolicyRules,
	outcomes: &[IngestOutcome],
) -> IngestSummary {
	let mut accepted = 0usize;
	let mut rejected = 0usize;
	let mut suppressed = 0usize;
	let mut errors = Vec::new();

	for (index, outcome) in outcomes.iter().enumerate() {
		match ingest_one(pg_pool, tenant_id, policy, outcome).await {
			Ok(action_taken) => {
				accepted += 1;
				if action_taken {
					suppressed += 1;
				}
			}
			Err(message) => {
				rejected += 1;
				errors.push(IngestRowError {
					index,
					email: outcome.email.clone(),
					message,
				});
			}
		}
	}

	IngestSummary {
		accepted,
		rejected,
		suppressed,
		errors,
	}
}

async fn ingest_one(
	pg_pool: &PgPool,
	tenant_id: Uuid,
	policy: &OutcomePolicyRules,
	outcome: &IngestOutcome,
) -> Result<bool, String> {
	let canonical = canonicalize_email(&outcome.email)
		.ok_or_else(|| "empty or unparseable email".to_string())?;

	let source = outcome.source.clone().unwrap_or_default();
	let action = decide_action(policy, outcome.outcome_type);
	let policy_action_label: Option<&'static str> = action_label(&action);

	let inserted = sqlx::query_scalar::<_, Option<i64>>(
		r#"
		INSERT INTO verification_outcomes
			(tenant_id, canonical_email, outcome_type, occurred_at, source, campaign_id, metadata, policy_action)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
		ON CONFLICT (tenant_id, canonical_email, outcome_type, occurred_at, source) DO NOTHING
		RETURNING id
		"#,
	)
	.bind(tenant_id)
	.bind(&canonical)
	.bind(outcome.outcome_type)
	.bind(outcome.occurred_at)
	.bind(&source)
	.bind(outcome.campaign_id.as_deref())
	.bind(outcome.metadata.as_ref())
	.bind(policy_action_label)
	.fetch_optional(pg_pool)
	.await
	.map_err(|err| format!("db insert: {err}"))?;

	if inserted.flatten().is_none() {
		// Duplicate ingest — that's fine, treat as accepted with no action.
		return Ok(false);
	}

	let action_taken = match action {
		Action::Suppress { reason, .. } => {
			add_to_suppression(pg_pool, tenant_id, &canonical, reason).await
		}
		Action::SuppressAndUnsubscribe { reason, .. } => {
			let s1 = add_to_suppression(pg_pool, tenant_id, &canonical, reason).await;
			let s2 = add_to_suppression(pg_pool, tenant_id, &canonical, "unsubscribe").await;
			s1 || s2
		}
		Action::SuppressAfter {
			threshold_count,
			threshold_window_days,
		} => {
			let count = count_recent_outcomes(
				pg_pool,
				tenant_id,
				&canonical,
				outcome.outcome_type,
				threshold_window_days,
			)
			.await;
			if count >= threshold_count {
				add_to_suppression(pg_pool, tenant_id, &canonical, "bounce").await
			} else {
				false
			}
		}
		Action::ScoreBoost => false,
		Action::Informational => false,
	};

	if action_taken {
		// Re-stamp policy_action with the actual outcome.
		let _ = sqlx::query(
			"UPDATE verification_outcomes SET policy_action = $2 WHERE tenant_id = $1 AND canonical_email = $3 AND outcome_type = $4 AND occurred_at = $5 AND source = $6"
		)
		.bind(tenant_id)
		.bind("suppressed")
		.bind(&canonical)
		.bind(outcome.outcome_type)
		.bind(outcome.occurred_at)
		.bind(&source)
		.execute(pg_pool)
		.await;
	}

	Ok(action_taken)
}

#[derive(Debug)]
enum Action {
	Suppress {
		reason: &'static str,
	},
	SuppressAndUnsubscribe {
		reason: &'static str,
	},
	SuppressAfter {
		threshold_count: i64,
		threshold_window_days: i32,
	},
	ScoreBoost,
	Informational,
}

fn decide_action(policy: &OutcomePolicyRules, outcome_type: OutcomeType) -> Action {
	let rule = match outcome_type {
		OutcomeType::HardBounce => &policy.hard_bounce,
		OutcomeType::Complaint => &policy.complaint,
		OutcomeType::SoftBounce => &policy.soft_bounce,
		OutcomeType::Unsubscribe => &policy.unsubscribe,
		OutcomeType::Delivered => &policy.delivered,
		OutcomeType::Open => &policy.open,
		OutcomeType::Click => &policy.click,
	};
	let default_reason = match outcome_type {
		OutcomeType::HardBounce | OutcomeType::SoftBounce => "bounce",
		OutcomeType::Complaint => "complaint",
		OutcomeType::Unsubscribe => "unsubscribe",
		_ => "auto_invalid",
	};
	match rule {
		RuleAction::Suppress { .. } => Action::Suppress {
			reason: default_reason,
		},
		RuleAction::SuppressAndUnsubscribe { .. } => Action::SuppressAndUnsubscribe {
			reason: default_reason,
		},
		RuleAction::SuppressAfter {
			threshold_count,
			threshold_window_days,
		} => Action::SuppressAfter {
			threshold_count: *threshold_count,
			threshold_window_days: *threshold_window_days,
		},
		RuleAction::ScoreBoost { .. } => Action::ScoreBoost,
		RuleAction::Informational => Action::Informational,
	}
}

fn action_label(action: &Action) -> Option<&'static str> {
	Some(match action {
		Action::Suppress { .. } | Action::SuppressAndUnsubscribe { .. } => "suppressed",
		Action::SuppressAfter { .. } => "pending",
		Action::ScoreBoost => "boosted",
		Action::Informational => "informational",
	})
}

async fn add_to_suppression(
	pg_pool: &PgPool,
	tenant_id: Uuid,
	canonical_email: &str,
	reason: &str,
) -> bool {
	let result = sqlx::query(
		r#"
		INSERT INTO v1_suppression_entries (tenant_id, email, reason, source)
		VALUES ($1, $2, $3::suppression_reason, 'outcome_feedback')
		ON CONFLICT (tenant_id, email) DO NOTHING
		"#,
	)
	.bind(tenant_id)
	.bind(canonical_email)
	.bind(reason)
	.execute(pg_pool)
	.await;

	match result {
		Ok(r) if r.rows_affected() > 0 => true,
		Ok(_) => false,
		Err(e) => {
			tracing::warn!(
				target: LOG_TARGET,
				tenant_id = %tenant_id,
				error = ?e,
				"Failed to add to suppression list from outcome ingest"
			);
			false
		}
	}
}

async fn count_recent_outcomes(
	pg_pool: &PgPool,
	tenant_id: Uuid,
	canonical_email: &str,
	outcome_type: OutcomeType,
	window_days: i32,
) -> i64 {
	sqlx::query_scalar::<_, i64>(
		r#"
		SELECT COUNT(*)
		FROM verification_outcomes
		WHERE tenant_id = $1
			AND canonical_email = $2
			AND outcome_type = $3
			AND occurred_at >= NOW() - ($4::INT * INTERVAL '1 day')
		"#,
	)
	.bind(tenant_id)
	.bind(canonical_email)
	.bind(outcome_type)
	.bind(window_days)
	.fetch_one(pg_pool)
	.await
	.unwrap_or(0)
}

// ----------------------------------------------------------------------------
// Policy fetch / lazy default
// ----------------------------------------------------------------------------

pub async fn fetch_or_create_default_policy(
	pg_pool: &PgPool,
	tenant_id: Uuid,
) -> (i64, OutcomePolicyRules) {
	if let Some((id, rules)) = fetch_default_policy(pg_pool, tenant_id).await {
		return (id, rules);
	}
	create_default_policy(pg_pool, tenant_id).await
}

async fn fetch_default_policy(
	pg_pool: &PgPool,
	tenant_id: Uuid,
) -> Option<(i64, OutcomePolicyRules)> {
	let row = sqlx::query(
		"SELECT id, rules FROM v1_outcome_policies WHERE tenant_id = $1 AND is_default = true",
	)
	.bind(tenant_id)
	.fetch_optional(pg_pool)
	.await
	.ok()??;
	let id: i64 = row.get("id");
	let rules: serde_json::Value = row.get("rules");
	let rules: OutcomePolicyRules = serde_json::from_value(rules).unwrap_or_default();
	Some((id, rules))
}

async fn create_default_policy(
	pg_pool: &PgPool,
	tenant_id: Uuid,
) -> (i64, OutcomePolicyRules) {
	let rules = default_outcome_policy_rules();
	let rules_json = default_outcome_policy_rules_json();
	let id = sqlx::query_scalar::<_, i64>(
		r#"
		INSERT INTO v1_outcome_policies (tenant_id, name, is_default, rules)
		VALUES ($1, 'default', true, $2)
		ON CONFLICT (tenant_id, name) DO UPDATE SET rules = EXCLUDED.rules
		RETURNING id
		"#,
	)
	.bind(tenant_id)
	.bind(&rules_json)
	.fetch_one(pg_pool)
	.await
	.unwrap_or(0);
	(id, rules)
}

// ----------------------------------------------------------------------------
// Scoring-context enrichment
// ----------------------------------------------------------------------------

pub async fn enrich_outcome_context(
	read_pool: Option<&PgPool>,
	tenant_id: Option<Uuid>,
	canonical_email: &str,
	completed_at: DateTime<Utc>,
) -> OutcomeContext {
	let (Some(pool), Some(tenant_id)) = (read_pool, tenant_id) else {
		return OutcomeContext::default();
	};
	if canonical_email.is_empty() {
		return OutcomeContext::default();
	}

	let ttl_days = fetch_default_policy(pool, tenant_id)
		.await
		.map(|(_, r)| r.outcome_ttl_days)
		.unwrap_or(90);

	let row = sqlx::query(
		r#"
		SELECT
			BOOL_OR(outcome_type = 'hard_bounce'::outcome_type) AS has_hard_bounce,
			BOOL_OR(outcome_type = 'complaint'::outcome_type) AS has_complaint,
			BOOL_OR(outcome_type = 'unsubscribe'::outcome_type) AS has_unsubscribe,
			COUNT(*) FILTER (WHERE outcome_type = 'delivered'::outcome_type) AS delivered_count,
			COUNT(*) FILTER (WHERE outcome_type = 'open'::outcome_type) AS open_count,
			COUNT(*) FILTER (WHERE outcome_type = 'click'::outcome_type) AS click_count,
			COUNT(*) FILTER (WHERE outcome_type = 'soft_bounce'::outcome_type) AS soft_bounce_count,
			MAX(occurred_at) FILTER (WHERE outcome_type IN ('open'::outcome_type, 'click'::outcome_type)) AS latest_engagement
		FROM verification_outcomes
		WHERE tenant_id = $1
			AND canonical_email = $2
			AND occurred_at >= $3 - ($4::INT * INTERVAL '1 day')
		"#,
	)
	.bind(tenant_id)
	.bind(canonical_email)
	.bind(completed_at)
	.bind(ttl_days)
	.fetch_optional(pool)
	.await;

	let row = match row {
		Ok(Some(r)) => r,
		_ => return OutcomeContext::default(),
	};

	let latest_engagement: Option<DateTime<Utc>> = row.try_get("latest_engagement").ok().flatten();
	OutcomeContext {
		has_hard_bounce: row.try_get("has_hard_bounce").ok().unwrap_or(false),
		has_complaint: row.try_get("has_complaint").ok().unwrap_or(false),
		has_unsubscribe: row.try_get("has_unsubscribe").ok().unwrap_or(false),
		delivered_count: row.try_get("delivered_count").unwrap_or(0),
		open_count: row.try_get("open_count").unwrap_or(0),
		click_count: row.try_get("click_count").unwrap_or(0),
		soft_bounce_count: row.try_get("soft_bounce_count").unwrap_or(0),
		latest_engagement_days_ago: latest_engagement
			.map(|t| (completed_at - t).num_days().max(0)),
	}
}

// ----------------------------------------------------------------------------
// Worker post-completion hook
// ----------------------------------------------------------------------------

/// Called by the worker after a verification completes. If the customer has
/// already ingested a hard bounce / complaint / unsubscribe for this email,
/// suppression should be applied even though scoring already ran. (The reverse
/// case — outcome arrives FIRST then verification runs — is handled by
/// `enrich_outcome_context` feeding into `ScoringContext`.)
pub async fn apply_post_verification_outcome_check(
	pg_pool: &PgPool,
	tenant_id: Uuid,
	email: &str,
) {
	let Some(canonical) = canonicalize_email(email) else {
		return;
	};
	let row = sqlx::query(
		r#"
		SELECT
			BOOL_OR(outcome_type = 'hard_bounce'::outcome_type) AS hard_bounce,
			BOOL_OR(outcome_type = 'complaint'::outcome_type) AS complaint,
			BOOL_OR(outcome_type = 'unsubscribe'::outcome_type) AS unsubscribe
		FROM verification_outcomes
		WHERE tenant_id = $1 AND canonical_email = $2
		"#,
	)
	.bind(tenant_id)
	.bind(&canonical)
	.fetch_optional(pg_pool)
	.await;

	let Ok(Some(row)) = row else {
		return;
	};
	let hard_bounce: bool = row.try_get("hard_bounce").unwrap_or(false);
	let complaint: bool = row.try_get("complaint").unwrap_or(false);
	let unsubscribe: bool = row.try_get("unsubscribe").unwrap_or(false);

	if hard_bounce {
		add_to_suppression(pg_pool, tenant_id, &canonical, "bounce").await;
	}
	if complaint {
		add_to_suppression(pg_pool, tenant_id, &canonical, "complaint").await;
	}
	if unsubscribe {
		add_to_suppression(pg_pool, tenant_id, &canonical, "unsubscribe").await;
	}
}

// ----------------------------------------------------------------------------
// Tests
// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn default_policy_round_trips_through_json() {
		let json = default_outcome_policy_rules_json();
		let back: OutcomePolicyRules = serde_json::from_value(json).unwrap();
		assert_eq!(back.outcome_ttl_days, 90);
		assert!(matches!(back.hard_bounce, RuleAction::Suppress { .. }));
		assert!(matches!(
			back.complaint,
			RuleAction::SuppressAndUnsubscribe { .. }
		));
		assert!(matches!(back.delivered, RuleAction::ScoreBoost { boost: 5 }));
	}

	#[test]
	fn engagement_boost_caps_at_15() {
		let mut ctx = OutcomeContext::default();
		ctx.delivered_count = 100;
		ctx.open_count = 50;
		ctx.click_count = 25;
		assert_eq!(ctx.engagement_boost(), 15);
	}

	#[test]
	fn forces_invalid_when_hard_bounce_or_complaint() {
		let mut ctx = OutcomeContext::default();
		assert!(!ctx.forces_invalid());
		ctx.has_hard_bounce = true;
		assert!(ctx.forces_invalid());
		ctx.has_hard_bounce = false;
		ctx.has_complaint = true;
		assert!(ctx.forces_invalid());
	}

	#[test]
	fn decide_action_maps_outcome_to_rule() {
		let policy = default_outcome_policy_rules();
		assert!(matches!(
			decide_action(&policy, OutcomeType::HardBounce),
			Action::Suppress { reason: "bounce" }
		));
		assert!(matches!(
			decide_action(&policy, OutcomeType::Complaint),
			Action::SuppressAndUnsubscribe { reason: "complaint" }
		));
		assert!(matches!(
			decide_action(&policy, OutcomeType::SoftBounce),
			Action::SuppressAfter {
				threshold_count: 3,
				threshold_window_days: 30
			}
		));
		assert!(matches!(
			decide_action(&policy, OutcomeType::Open),
			Action::ScoreBoost
		));
	}
}
