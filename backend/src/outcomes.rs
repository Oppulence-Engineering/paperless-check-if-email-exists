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
use sqlx::{PgConnection, PgPool, Row};
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
	serde_json::to_value(default_outcome_policy_rules()).unwrap_or_else(|_| serde_json::json!({}))
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
	/// The tenant's default outcome policy, which decides what each outcome
	/// does to scoring.
	pub rules: OutcomePolicyRules,
}

impl OutcomeContext {
	/// True when an outcome is so definitive it should override SMTP-derived score.
	pub fn forces_invalid(&self) -> bool {
		self.forced_invalid_reason().is_some()
	}

	/// The outcome that forces this address invalid, if any: one that was
	/// reported and whose rule overrides the score to "invalid".
	pub fn forced_invalid_reason(&self) -> Option<OutcomeType> {
		if self.has_hard_bounce && overrides_to_invalid(&self.rules.hard_bounce) {
			Some(OutcomeType::HardBounce)
		} else if self.has_complaint && overrides_to_invalid(&self.rules.complaint) {
			Some(OutcomeType::Complaint)
		} else {
			None
		}
	}

	/// A reported hard bounce the tenant's policy acts on. An informational
	/// or score-boost rule means the tenant chose not to treat it as blocking.
	pub fn hard_bounce_counts(&self) -> bool {
		self.has_hard_bounce
			&& !matches!(
				self.rules.hard_bounce,
				RuleAction::Informational | RuleAction::ScoreBoost { .. }
			)
	}

	/// Score boost from engagement, using the tenant's ScoreBoost rules and
	/// capped at 15. Other rule kinds add nothing.
	pub fn engagement_boost(&self) -> i16 {
		let boost = |count: i64, rule: &RuleAction| match rule {
			RuleAction::ScoreBoost { boost } if count > 0 => (*boost).max(0),
			_ => 0,
		};
		boost(self.delivered_count, &self.rules.delivered)
			.saturating_add(boost(self.open_count, &self.rules.open))
			.saturating_add(boost(self.click_count, &self.rules.click))
			.min(15)
	}
}

fn overrides_to_invalid(rule: &RuleAction) -> bool {
	matches!(
		rule,
		RuleAction::Suppress { score_override: Some(target) }
			| RuleAction::SuppressAndUnsubscribe { score_override: Some(target) }
			if target == "invalid"
	)
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

	// The outcome row and the suppressions it causes commit together. A
	// duplicate row therefore means its action already ran, and a failed
	// action stores no row, so the client's retry runs it again. Replaying the
	// action for a duplicate would re-suppress an address that the customer
	// has removed from the suppression list since.
	let mut tx = pg_pool
		.begin()
		.await
		.map_err(|err| format!("db begin: {err}"))?;
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
	.fetch_optional(&mut *tx)
	.await
	.map_err(|err| format!("db insert: {err}"))?;

	let Some(outcome_id) = inserted.flatten() else {
		// Duplicate ingest: accepted, and its action committed with it.
		return Ok(false);
	};

	let action_taken = apply_outcome_action(
		&mut tx,
		tenant_id,
		&outcome.email,
		&canonical,
		outcome,
		action,
	)
	.await?;

	if action_taken {
		// Re-stamp policy_action with the actual outcome.
		sqlx::query("UPDATE verification_outcomes SET policy_action = 'suppressed' WHERE id = $1")
			.bind(outcome_id)
			.execute(&mut *tx)
			.await
			.map_err(|err| format!("db update: {err}"))?;
	}

	tx.commit()
		.await
		.map_err(|err| format!("db commit: {err}"))?;
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

/// Run the policy action for one outcome on the ingest transaction. Returns
/// whether a suppression was created; a failed suppression write is an
/// error, so the outcome is not stored and the client retries the row.
async fn apply_outcome_action(
	conn: &mut PgConnection,
	tenant_id: Uuid,
	email: &str,
	canonical: &str,
	outcome: &IngestOutcome,
	action: Action,
) -> Result<bool, String> {
	Ok(match action {
		Action::Suppress { reason, .. } => {
			add_to_suppression(conn, tenant_id, email, reason).await?
		}
		Action::SuppressAndUnsubscribe { reason, .. } => {
			let s1 = add_to_suppression(conn, tenant_id, email, reason).await?;
			let s2 = add_to_suppression(conn, tenant_id, email, "unsubscribe").await?;
			s1 || s2
		}
		Action::SuppressAfter {
			threshold_count,
			threshold_window_days,
		} => {
			// On the transaction, so the outcome just inserted counts.
			let count = count_recent_outcomes(
				conn,
				tenant_id,
				canonical,
				outcome.outcome_type,
				threshold_window_days,
			)
			.await
			.map_err(|err| format!("db count: {err}"))?;
			if count >= threshold_count {
				add_to_suppression(conn, tenant_id, email, "bounce").await?
			} else {
				false
			}
		}
		Action::ScoreBoost => false,
		Action::Informational => false,
	})
}

async fn add_to_suppression(
	conn: &mut PgConnection,
	tenant_id: Uuid,
	email: &str,
	reason: &str,
) -> Result<bool, String> {
	crate::suppression::insert_active_suppression(
		conn,
		tenant_id,
		email,
		reason,
		"outcome_feedback",
		"outcome",
		"outcome_ingest",
	)
	.await
	.map_err(|e| {
		tracing::warn!(
			target: LOG_TARGET,
			tenant_id = %tenant_id,
			error = ?e,
			"Failed to add to suppression list from outcome ingest"
		);
		format!("suppression write: {e}")
	})
}

async fn count_recent_outcomes(
	conn: &mut PgConnection,
	tenant_id: Uuid,
	canonical_email: &str,
	outcome_type: OutcomeType,
	window_days: i32,
) -> Result<i64, sqlx::Error> {
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
	.fetch_one(conn)
	.await
}

// ----------------------------------------------------------------------------
// Policy fetch / lazy default
// ----------------------------------------------------------------------------

/// The tenant's default outcome policy, created with the built-in rules on
/// first use. A database error is returned, not treated as "no policy": the
/// fallback must never replace rules that a tenant has customised.
pub async fn fetch_or_create_default_policy(
	pg_pool: &PgPool,
	tenant_id: Uuid,
) -> Result<(i64, OutcomePolicyRules), sqlx::Error> {
	if let Some(found) = fetch_policy(
		pg_pool,
		tenant_id,
		"SELECT id, rules FROM v1_outcome_policies WHERE tenant_id = $1 AND is_default = true",
	)
	.await?
	{
		return Ok(found);
	}
	let rules_json = default_outcome_policy_rules_json();
	let inserted = sqlx::query_scalar::<_, i64>(
		r#"
		INSERT INTO v1_outcome_policies (tenant_id, name, is_default, rules)
		VALUES ($1, 'default', true, $2)
		ON CONFLICT (tenant_id, name) DO NOTHING
		RETURNING id
		"#,
	)
	.bind(tenant_id)
	.bind(&rules_json)
	.fetch_optional(pg_pool)
	.await?;
	if let Some(id) = inserted {
		return Ok((id, default_outcome_policy_rules()));
	}
	// A policy named "default" exists but is not the default one (the tenant
	// unset the flag, or a concurrent request created it). Use its rules as
	// they are.
	fetch_policy(
		pg_pool,
		tenant_id,
		"SELECT id, rules FROM v1_outcome_policies WHERE tenant_id = $1 AND name = 'default'",
	)
	.await?
	.ok_or(sqlx::Error::RowNotFound)
}

async fn fetch_policy(
	pg_pool: &PgPool,
	tenant_id: Uuid,
	sql: &str,
) -> Result<Option<(i64, OutcomePolicyRules)>, sqlx::Error> {
	let Some(row) = sqlx::query(sql)
		.bind(tenant_id)
		.fetch_optional(pg_pool)
		.await?
	else {
		return Ok(None);
	};
	let id: i64 = row.get("id");
	let rules: serde_json::Value = row.get("rules");
	let rules: OutcomePolicyRules = serde_json::from_value(rules).unwrap_or_default();
	Ok(Some((id, rules)))
}

/// The rules of the tenant's default outcome policy, for read-only use.
/// Best effort: without a policy, or when the read fails, the built-in rules
/// apply. Nothing is created here.
async fn default_policy_rules(pg_pool: &PgPool, tenant_id: Uuid) -> OutcomePolicyRules {
	fetch_policy(
		pg_pool,
		tenant_id,
		"SELECT id, rules FROM v1_outcome_policies WHERE tenant_id = $1 AND is_default = true",
	)
	.await
	.ok()
	.flatten()
	.map(|(_, rules)| rules)
	.unwrap_or_default()
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

	let rules = default_policy_rules(pool, tenant_id).await;
	let ttl_days = rules.outcome_ttl_days;

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
			AND occurred_at <= $3
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
		Ok(None) => return OutcomeContext::default(),
		Err(error) => {
			tracing::warn!(target: LOG_TARGET, tenant_id = %tenant_id, error = ?error, "Outcome lookup failed, scoring without outcome feedback");
			return OutcomeContext::default();
		}
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
		latest_engagement_days_ago: latest_engagement.map(|t| (completed_at - t).num_days().max(0)),
		rules,
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
pub async fn apply_post_verification_outcome_check(pg_pool: &PgPool, tenant_id: Uuid, email: &str) {
	let Some(canonical) = canonicalize_email(email) else {
		return;
	};
	// The same policy and expiry as ingestion and scoring, so a verification
	// never suppresses an address over an outcome the policy ignores.
	let rules = default_policy_rules(pg_pool, tenant_id).await;
	let row = sqlx::query(
		r#"
		SELECT
			BOOL_OR(outcome_type = 'hard_bounce'::outcome_type) AS hard_bounce,
			BOOL_OR(outcome_type = 'complaint'::outcome_type) AS complaint,
			BOOL_OR(outcome_type = 'unsubscribe'::outcome_type) AS unsubscribe
		FROM verification_outcomes
		WHERE tenant_id = $1 AND canonical_email = $2
			AND occurred_at >= NOW() - ($3::INT * INTERVAL '1 day')
		"#,
	)
	.bind(tenant_id)
	.bind(&canonical)
	.bind(rules.outcome_ttl_days)
	.fetch_optional(pg_pool)
	.await;

	let Ok(Some(row)) = row else {
		return;
	};
	let mut conn = match pg_pool.acquire().await {
		Ok(conn) => conn,
		Err(error) => {
			tracing::warn!(target: LOG_TARGET, tenant_id = %tenant_id, error = ?error, "No connection for the post-verification outcome check");
			return;
		}
	};
	// Failures are logged inside add_to_suppression; the next verification of
	// this address runs the check again.
	for (column, rule, reason) in [
		("hard_bounce", &rules.hard_bounce, "bounce"),
		("complaint", &rules.complaint, "complaint"),
		("unsubscribe", &rules.unsubscribe, "unsubscribe"),
	] {
		let seen: bool = row.try_get(column).ok().flatten().unwrap_or(false);
		let suppresses = matches!(
			rule,
			RuleAction::Suppress { .. } | RuleAction::SuppressAndUnsubscribe { .. }
		);
		if seen && suppresses {
			let _ = add_to_suppression(&mut conn, tenant_id, email, reason).await;
		}
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
		assert!(matches!(
			back.delivered,
			RuleAction::ScoreBoost { boost: 5 }
		));
	}

	#[test]
	fn engagement_boost_caps_at_15() {
		let mut ctx = OutcomeContext::default();
		ctx.delivered_count = 100;
		ctx.open_count = 50;
		ctx.click_count = 25;
		assert_eq!(ctx.engagement_boost(), 15);

		// The tenant's rules set the boosts: informational events add nothing.
		ctx.rules.click = RuleAction::Informational;
		ctx.rules.open = RuleAction::ScoreBoost { boost: 2 };
		assert_eq!(ctx.engagement_boost(), 5 + 2);
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
		assert_eq!(ctx.forced_invalid_reason(), Some(OutcomeType::Complaint));

		// A tenant whose policy keeps hard bounces informational does not want
		// them to override verification; its complaint rule still applies.
		ctx.has_hard_bounce = true;
		ctx.rules.hard_bounce = RuleAction::Informational;
		assert_eq!(ctx.forced_invalid_reason(), Some(OutcomeType::Complaint));
		assert!(!ctx.hard_bounce_counts());
		ctx.has_complaint = false;
		assert!(!ctx.forces_invalid());
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
			Action::SuppressAndUnsubscribe {
				reason: "complaint"
			}
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
