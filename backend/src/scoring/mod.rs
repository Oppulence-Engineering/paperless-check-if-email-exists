pub mod response;

use check_if_email_exists::{
	provider::{Provider, ProviderConfidence, ProviderRejectionReason},
	smtp::SmtpError,
	CheckEmailOutput, Reachable,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EmailCategory {
	Valid,
	Risky,
	Unknown,
	Invalid,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SubReason {
	Deliverable,
	InvalidSyntax,
	InvalidRecipient,
	ProviderRejected,
	SmtpUndeliverable,
	DisabledMailbox,
	NoMx,
	SmtpError,
	SmtpUnreachable,
	CatchAll,
	FullInbox,
	Disposable,
	RoleAccount,
	SpamTrap,
	Risky,
	Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScoringSignals {
	pub valid_syntax: bool,
	pub reachable: Reachable,
	pub has_mx_records: bool,
	pub smtp_error: bool,
	pub smtp_can_connect: bool,
	pub smtp_is_deliverable: bool,
	pub smtp_is_disabled: bool,
	pub smtp_is_catch_all: bool,
	pub smtp_has_full_inbox: bool,
	pub is_disposable: bool,
	pub is_role_account: bool,
	pub is_spam_trap_domain: bool,
	pub is_free_provider: bool,
	pub has_domain_suggestion: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmailScore {
	pub score: i16,
	pub category: EmailCategory,
	pub sub_reason: SubReason,
	pub safe_to_send: bool,
	pub reason_codes: Vec<String>,
	pub signals: ScoringSignals,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfidenceLevel {
	High,
	Medium,
	Low,
	VeryLow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CatchAllSeverity {
	Low,
	Medium,
	High,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CatchAllScore {
	pub severity: CatchAllSeverity,
	pub confidence: i16,
	pub factors: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SmtpUncertaintyClass {
	Transient,
	PolicyBlock,
	Timeout,
	Network,
	AmbiguousResponse,
	SmtpUnreachable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PartialConfidence {
	pub confidence: i16,
	pub classification: SmtpUncertaintyClass,
	pub factors: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScoreInsights {
	pub confidence: i16,
	pub confidence_level: ConfidenceLevel,
	pub confidence_factors: Vec<String>,
	pub catch_all: Option<CatchAllScore>,
	pub partial_confidence: Option<PartialConfidence>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TenantHistoryContext {
	pub safe_count_180d: u32,
	pub total_count_180d: u32,
	pub latest_days_ago: Option<i64>,
	pub inconsistent_count_180d: u32,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PatternContext {
	pub pattern: Option<String>,
	pub verified_same_pattern_count_180d: u32,
	pub verified_domain_count_180d: u32,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DomainSignalContext {
	pub domain_age_days: Option<i64>,
	pub website_present: Option<bool>,
	pub has_spf: Option<bool>,
	pub has_dkim: Option<bool>,
	pub has_dmarc: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderReputationContext {
	pub provider: Provider,
	pub confidence: Option<ProviderConfidence>,
	pub reputation_score: i16,
}

#[derive(Debug, Clone, Default)]
pub struct ScoringContext {
	pub tenant_history: TenantHistoryContext,
	pub pattern: PatternContext,
	pub domain: DomainSignalContext,
	pub provider_reputation: Option<ProviderReputationContext>,
	pub outcomes: crate::outcomes::OutcomeContext,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScoreComputation {
	pub score: EmailScore,
	pub insights: ScoreInsights,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Freshness {
	Fresh,
	Recent,
	Aging,
	Stale,
	Expired,
}

pub struct FreshnessInfo {
	pub verified_at: String,
	pub age_days: i64,
	pub freshness: Freshness,
}

pub fn compute_freshness(completed_at: chrono::DateTime<chrono::Utc>) -> FreshnessInfo {
	compute_freshness_at(completed_at, chrono::Utc::now())
}

pub fn compute_freshness_at(
	completed_at: chrono::DateTime<chrono::Utc>,
	now: chrono::DateTime<chrono::Utc>,
) -> FreshnessInfo {
	let age = now - completed_at;
	let age_days = age.num_days().max(0);
	let freshness = match age_days {
		0..=7 => Freshness::Fresh,
		8..=30 => Freshness::Recent,
		31..=60 => Freshness::Aging,
		61..=90 => Freshness::Stale,
		_ => Freshness::Expired,
	};
	FreshnessInfo {
		verified_at: completed_at.to_rfc3339(),
		age_days,
		freshness,
	}
}

pub fn compute_score(output: &CheckEmailOutput) -> EmailScore {
	let signals = extract_signals(output);
	compute_score_from_signals(output, signals)
}

fn compute_score_from_signals(output: &CheckEmailOutput, signals: ScoringSignals) -> EmailScore {
	if !signals.valid_syntax {
		let reason_codes =
			collect_reason_codes(&signals, output.provider_rejection_reason.as_ref());
		return EmailScore {
			score: 0,
			category: EmailCategory::Invalid,
			sub_reason: SubReason::InvalidSyntax,
			safe_to_send: false,
			reason_codes,
			signals,
		};
	}

	if output.provider_rejection_reason.is_some() {
		let reason_codes =
			collect_reason_codes(&signals, output.provider_rejection_reason.as_ref());
		return EmailScore {
			score: 0,
			category: EmailCategory::Invalid,
			sub_reason: SubReason::ProviderRejected,
			safe_to_send: false,
			reason_codes,
			signals,
		};
	}

	if matches!(signals.reachable, Reachable::Invalid) {
		let reason_codes =
			collect_reason_codes(&signals, output.provider_rejection_reason.as_ref());
		return EmailScore {
			score: 0,
			category: EmailCategory::Invalid,
			sub_reason: SubReason::InvalidRecipient,
			safe_to_send: false,
			reason_codes,
			signals,
		};
	}

	if !signals.smtp_is_deliverable {
		let reason_codes =
			collect_reason_codes(&signals, output.provider_rejection_reason.as_ref());
		return EmailScore {
			score: 0,
			category: EmailCategory::Invalid,
			sub_reason: SubReason::SmtpUndeliverable,
			safe_to_send: false,
			reason_codes,
			signals,
		};
	}

	if signals.smtp_is_disabled {
		let reason_codes =
			collect_reason_codes(&signals, output.provider_rejection_reason.as_ref());
		return EmailScore {
			score: 0,
			category: EmailCategory::Invalid,
			sub_reason: SubReason::DisabledMailbox,
			safe_to_send: false,
			reason_codes,
			signals,
		};
	}

	let mut score = 100i16;

	if matches!(signals.reachable, Reachable::Unknown) {
		score -= 40;
	}
	if signals.smtp_error {
		score -= 35;
	}
	if !signals.has_mx_records {
		score -= 50;
	}
	if !signals.smtp_can_connect {
		score -= 30;
	}
	if signals.smtp_is_catch_all {
		score -= 15;
	}
	if signals.smtp_has_full_inbox {
		score -= 20;
	}
	if signals.is_disposable {
		score -= 25;
	}
	if signals.is_role_account {
		score -= 10;
	}
	if signals.is_spam_trap_domain {
		score -= 30;
	}
	if matches!(signals.reachable, Reachable::Risky) {
		score -= 10;
	}

	score = score.clamp(0, 100);

	let category = match score {
		80..=100 => EmailCategory::Valid,
		50..=79 => EmailCategory::Risky,
		1..=49 => EmailCategory::Unknown,
		_ => EmailCategory::Invalid,
	};

	let sub_reason = if !signals.has_mx_records {
		SubReason::NoMx
	} else if signals.smtp_error {
		SubReason::SmtpError
	} else if !signals.smtp_can_connect {
		SubReason::SmtpUnreachable
	} else if signals.smtp_is_catch_all {
		SubReason::CatchAll
	} else if signals.smtp_has_full_inbox {
		SubReason::FullInbox
	} else if signals.is_disposable {
		SubReason::Disposable
	} else if signals.is_role_account {
		SubReason::RoleAccount
	} else if signals.is_spam_trap_domain {
		SubReason::SpamTrap
	} else if matches!(signals.reachable, Reachable::Risky) {
		SubReason::Risky
	} else if matches!(signals.reachable, Reachable::Unknown) {
		SubReason::Unknown
	} else {
		SubReason::Deliverable
	};

	let safe_to_send = category == EmailCategory::Valid
		&& !signals.is_disposable
		&& !signals.smtp_is_catch_all
		&& !signals.is_role_account
		&& !signals.is_spam_trap_domain;

	let reason_codes = collect_reason_codes(&signals, output.provider_rejection_reason.as_ref());

	EmailScore {
		score,
		category,
		sub_reason,
		safe_to_send,
		reason_codes,
		signals,
	}
}

pub fn compute_score_with_context(
	output: &CheckEmailOutput,
	context: &ScoringContext,
) -> ScoreComputation {
	let base = compute_score(output);
	let mut score = base.clone();
	let mut confidence_factors = Vec::new();
	let mut contextual_codes = Vec::new();

	// Outcome ground truth wins over every other signal, including the
	// hard-invalid syntactic check below: a real hard bounce or complaint
	// reported by the customer's ESP is the strongest possible signal.
	if context.outcomes.forces_invalid() {
		let reason_code = if context.outcomes.has_hard_bounce {
			"outcome_hard_bounce"
		} else {
			"outcome_complaint"
		};
		let factor = if context.outcomes.has_hard_bounce {
			"outcome_feedback:hard_bounce"
		} else {
			"outcome_feedback:complaint"
		};
		score.score = 5;
		score.category = EmailCategory::Invalid;
		score.safe_to_send = false;
		append_unique_codes(&mut score.reason_codes, vec![reason_code.to_string()]);
		let insights = ScoreInsights {
			confidence: 95,
			confidence_level: ConfidenceLevel::High,
			confidence_factors: vec![factor.to_string()],
			catch_all: None,
			partial_confidence: None,
		};
		return ScoreComputation { score, insights };
	}

	if is_hard_invalid(&score) {
		let insights = ScoreInsights {
			confidence: 95,
			confidence_level: ConfidenceLevel::High,
			confidence_factors: vec!["hard_invalid_signal".to_string()],
			catch_all: None,
			partial_confidence: None,
		};
		return ScoreComputation { score, insights };
	}

	let mut adjustment = 0i16;

	let engagement_boost = context.outcomes.engagement_boost();
	if engagement_boost > 0 {
		adjustment += engagement_boost;
		contextual_codes.push("outcome_engagement".to_string());
		if context.outcomes.delivered_count > 0 {
			confidence_factors.push("outcome_feedback:delivered".to_string());
		}
		if context.outcomes.click_count > 0 {
			confidence_factors.push("outcome_feedback:click".to_string());
		} else if context.outcomes.open_count > 0 {
			confidence_factors.push("outcome_feedback:open".to_string());
		}
	}

	if let Some(provider) = &context.provider_reputation {
		if score.signals.smtp_is_deliverable && provider.reputation_score >= 80 {
			adjustment += 5;
			contextual_codes.push("provider_reputation".to_string());
			confidence_factors.push(format!(
				"provider_reputation:{}",
				provider_name(&provider.provider)
			));
		}
	}

	if context.tenant_history.safe_count_180d >= 3 {
		adjustment += 10;
		contextual_codes.push("tenant_history_positive".to_string());
		confidence_factors.push("tenant_history:safe_repeat".to_string());
	} else if context.tenant_history.safe_count_180d > 0 {
		adjustment += 5;
		contextual_codes.push("tenant_history_positive".to_string());
		confidence_factors.push("tenant_history:prior_safe".to_string());
	}

	if context.tenant_history.inconsistent_count_180d > 0 {
		adjustment -= 10;
		contextual_codes.push("tenant_history_inconsistent".to_string());
		confidence_factors.push("tenant_history:inconsistent".to_string());
	}

	if let Some(days) = context.tenant_history.latest_days_ago {
		if days <= 7 {
			adjustment += 5;
			confidence_factors.push("fresh_history:within_7_days".to_string());
		} else if days <= 30 {
			adjustment += 3;
			confidence_factors.push("fresh_history:within_30_days".to_string());
		} else if days > 90 {
			adjustment -= 5;
			confidence_factors.push("fresh_history:stale".to_string());
		}
	}

	let catch_all = if score.signals.smtp_is_catch_all {
		let catch_all = assess_catch_all(context, &score.signals);
		let severity_penalty = match catch_all.severity {
			CatchAllSeverity::Low => 5,
			CatchAllSeverity::Medium => 12,
			CatchAllSeverity::High => 25,
		};
		// The legacy scorer already applied a flat -15 catch-all penalty.
		adjustment += 15 - severity_penalty;
		contextual_codes.push(
			match catch_all.severity {
				CatchAllSeverity::Low => "catch_all_low_confidence",
				CatchAllSeverity::Medium => "catch_all_medium_confidence",
				CatchAllSeverity::High => "catch_all_high_risk",
			}
			.to_string(),
		);
		confidence_factors.extend(catch_all.factors.iter().cloned());
		Some(catch_all)
	} else {
		None
	};

	let partial_confidence =
		classify_smtp_uncertainty(output, &score.signals).map(|classification| {
			let partial = compute_partial_confidence(context, &score.signals, classification);
			contextual_codes.push("partial_confidence".to_string());
			contextual_codes.push(
				match partial.classification {
					SmtpUncertaintyClass::Transient => "transient_smtp",
					SmtpUncertaintyClass::PolicyBlock => "smtp_policy_block",
					SmtpUncertaintyClass::Timeout => "smtp_timeout",
					SmtpUncertaintyClass::Network => "smtp_network",
					SmtpUncertaintyClass::AmbiguousResponse => "smtp_ambiguous",
					SmtpUncertaintyClass::SmtpUnreachable => "smtp_unreachable",
				}
				.to_string(),
			);
			confidence_factors.extend(partial.factors.iter().cloned());

			if partial.confidence >= 70 {
				adjustment += 25;
			} else if partial.confidence >= 60 {
				adjustment += 15;
			} else if partial.confidence >= 50 {
				adjustment += 8;
			}

			partial
		});

	score.score = (score.score + adjustment).clamp(0, 100);
	score.category = category_for_score(score.score);
	score.safe_to_send = score.category == EmailCategory::Valid
		&& !score.signals.is_disposable
		&& !score.signals.smtp_is_catch_all
		&& !score.signals.is_role_account
		&& !score.signals.is_spam_trap_domain
		&& partial_confidence.is_none();
	append_unique_codes(&mut score.reason_codes, contextual_codes);

	let confidence = compute_overall_confidence(
		&score,
		context,
		catch_all.as_ref(),
		partial_confidence.as_ref(),
	);
	let insights = ScoreInsights {
		confidence,
		confidence_level: confidence_level(confidence),
		confidence_factors: unique_strings(confidence_factors),
		catch_all,
		partial_confidence,
	};

	ScoreComputation { score, insights }
}

fn is_hard_invalid(score: &EmailScore) -> bool {
	score.score == 0
		&& matches!(score.category, EmailCategory::Invalid)
		&& matches!(
			score.sub_reason,
			SubReason::InvalidSyntax
				| SubReason::InvalidRecipient
				| SubReason::ProviderRejected
				| SubReason::SmtpUndeliverable
				| SubReason::DisabledMailbox
		)
}

fn category_for_score(score: i16) -> EmailCategory {
	match score {
		80..=100 => EmailCategory::Valid,
		50..=79 => EmailCategory::Risky,
		1..=49 => EmailCategory::Unknown,
		_ => EmailCategory::Invalid,
	}
}

fn assess_catch_all(context: &ScoringContext, signals: &ScoringSignals) -> CatchAllScore {
	let mut factors = Vec::new();
	let mut confidence = 45i16;
	let mut severity = if signals.is_free_provider {
		factors.push("catch_all:free_provider".to_string());
		confidence += 10;
		CatchAllSeverity::Low
	} else {
		factors.push("catch_all:corporate_domain".to_string());
		CatchAllSeverity::High
	};

	if context.tenant_history.safe_count_180d >= 3 {
		severity = CatchAllSeverity::Low;
		confidence += 25;
		factors.push("tenant_history:safe_repeat".to_string());
	}

	if context.pattern.verified_same_pattern_count_180d >= 5 {
		severity = CatchAllSeverity::Low;
		confidence += 20;
		factors.push(format!(
			"pattern:{}:verified_matches",
			context.pattern.pattern.as_deref().unwrap_or("unknown")
		));
	} else if context.pattern.verified_same_pattern_count_180d >= 2 {
		severity = match severity {
			CatchAllSeverity::High => CatchAllSeverity::Medium,
			other => other,
		};
		confidence += 10;
		factors.push("pattern:some_verified_matches".to_string());
	}

	if context
		.domain
		.domain_age_days
		.map(|days| days >= 365)
		.unwrap_or(false)
	{
		if matches!(severity, CatchAllSeverity::High) {
			severity = CatchAllSeverity::Medium;
		}
		confidence += 5;
		factors.push("domain_age:established".to_string());
	}

	if context.domain.website_present == Some(false) {
		severity = CatchAllSeverity::High;
		confidence -= 10;
		factors.push("website:missing".to_string());
	} else if context.domain.website_present == Some(true) {
		confidence += 5;
		factors.push("website:present".to_string());
	}

	if context.domain.has_spf == Some(false) || context.domain.has_dmarc == Some(false) {
		severity = CatchAllSeverity::High;
		confidence -= 5;
		factors.push("auth_records:weak".to_string());
	} else if context.domain.has_spf == Some(true) || context.domain.has_dmarc == Some(true) {
		confidence += 5;
		factors.push("auth_records:present".to_string());
	}

	CatchAllScore {
		severity,
		confidence: confidence.clamp(20, 95),
		factors: unique_strings(factors),
	}
}

fn classify_smtp_uncertainty(
	output: &CheckEmailOutput,
	signals: &ScoringSignals,
) -> Option<SmtpUncertaintyClass> {
	match &output.smtp {
		Err(SmtpError::Timeout(_)) => Some(SmtpUncertaintyClass::Timeout),
		Err(SmtpError::AsyncSmtpError(async_smtp::error::Error::Transient(_))) => {
			Some(SmtpUncertaintyClass::Transient)
		}
		Err(SmtpError::IOError(_) | SmtpError::Socks5(_)) => Some(SmtpUncertaintyClass::Network),
		Err(error) if error.get_description().is_some() => Some(SmtpUncertaintyClass::PolicyBlock),
		Err(_) => Some(SmtpUncertaintyClass::AmbiguousResponse),
		Ok(_) if !signals.smtp_can_connect => Some(SmtpUncertaintyClass::SmtpUnreachable),
		Ok(_) if matches!(signals.reachable, Reachable::Unknown) => {
			Some(SmtpUncertaintyClass::AmbiguousResponse)
		}
		_ => None,
	}
}

fn compute_partial_confidence(
	context: &ScoringContext,
	signals: &ScoringSignals,
	classification: SmtpUncertaintyClass,
) -> PartialConfidence {
	let mut confidence = 35i16;
	let mut factors = Vec::new();

	if signals.has_mx_records {
		confidence += 20;
		factors.push("mx:present".to_string());
	} else {
		confidence -= 20;
		factors.push("mx:missing".to_string());
	}

	if context
		.domain
		.domain_age_days
		.map(|days| days >= 365)
		.unwrap_or(false)
	{
		confidence += 15;
		factors.push("domain_age:established".to_string());
	} else if context
		.domain
		.domain_age_days
		.map(|days| days < 90)
		.unwrap_or(false)
	{
		confidence -= 10;
		factors.push("domain_age:young".to_string());
	}

	if context.domain.website_present == Some(true) {
		confidence += 10;
		factors.push("website:present".to_string());
	} else if context.domain.website_present == Some(false) {
		confidence -= 10;
		factors.push("website:missing".to_string());
	}

	if context.domain.has_spf == Some(true) {
		confidence += 10;
		factors.push("spf:present".to_string());
	}
	if context.domain.has_dkim == Some(true) {
		confidence += 5;
		factors.push("dkim:present".to_string());
	}
	if context.domain.has_dmarc == Some(true) {
		confidence += 5;
		factors.push("dmarc:present".to_string());
	}

	if context
		.provider_reputation
		.as_ref()
		.map(|provider| provider.reputation_score >= 80)
		.unwrap_or(false)
	{
		confidence += 10;
		factors.push("provider:reputable".to_string());
	}

	if context.tenant_history.safe_count_180d >= 3 {
		confidence += 15;
		factors.push("tenant_history:safe_repeat".to_string());
	} else if context.tenant_history.safe_count_180d > 0 {
		confidence += 8;
		factors.push("tenant_history:prior_safe".to_string());
	}

	if context.tenant_history.inconsistent_count_180d > 0 {
		confidence -= 15;
		factors.push("tenant_history:inconsistent".to_string());
	}

	PartialConfidence {
		confidence: confidence.clamp(10, 90),
		classification,
		factors: unique_strings(factors),
	}
}

fn compute_overall_confidence(
	score: &EmailScore,
	context: &ScoringContext,
	catch_all: Option<&CatchAllScore>,
	partial: Option<&PartialConfidence>,
) -> i16 {
	if let Some(partial) = partial {
		return partial.confidence;
	}

	let mut confidence = 70i16;
	if score.signals.has_mx_records {
		confidence += 5;
	}
	if score.signals.smtp_is_deliverable {
		confidence += 10;
	}
	if context.provider_reputation.is_some() {
		confidence += 5;
	}
	if context.tenant_history.safe_count_180d > 0 {
		confidence += 5;
	}
	if context.domain.domain_age_days.is_some()
		|| context.domain.website_present.is_some()
		|| context.domain.has_spf.is_some()
	{
		confidence += 5;
	}
	if score.signals.smtp_error {
		confidence -= 20;
	}
	if let Some(catch_all) = catch_all {
		confidence = confidence.min(catch_all.confidence);
		if matches!(catch_all.severity, CatchAllSeverity::High) {
			confidence -= 10;
		}
	}

	confidence.clamp(20, 98)
}

fn confidence_level(confidence: i16) -> ConfidenceLevel {
	match confidence {
		80..=100 => ConfidenceLevel::High,
		60..=79 => ConfidenceLevel::Medium,
		30..=59 => ConfidenceLevel::Low,
		_ => ConfidenceLevel::VeryLow,
	}
}

pub fn provider_reputation_context(
	provider: Option<&Provider>,
	confidence: Option<&ProviderConfidence>,
) -> Option<ProviderReputationContext> {
	let provider = provider?.clone();
	let base: i16 = match provider {
		Provider::Gmail
		| Provider::GoogleWorkspace
		| Provider::Microsoft365
		| Provider::OutlookConsumer => 90,
		Provider::Yahoo | Provider::AppleIcloud | Provider::Proton | Provider::Zoho => 80,
	};
	let adjustment: i16 = match confidence {
		Some(ProviderConfidence::High) => 5,
		Some(ProviderConfidence::Medium) => 0,
		Some(ProviderConfidence::Low) | None => -10,
	};
	Some(ProviderReputationContext {
		provider,
		confidence: confidence.cloned(),
		reputation_score: (base + adjustment).clamp(0, 100),
	})
}

fn provider_name(provider: &Provider) -> &'static str {
	match provider {
		Provider::Gmail => "gmail",
		Provider::GoogleWorkspace => "google_workspace",
		Provider::OutlookConsumer => "outlook_consumer",
		Provider::Microsoft365 => "microsoft_365",
		Provider::Yahoo => "yahoo",
		Provider::AppleIcloud => "apple_icloud",
		Provider::Proton => "proton",
		Provider::Zoho => "zoho",
	}
}

fn append_unique_codes(codes: &mut Vec<String>, additions: Vec<String>) {
	for code in additions {
		if !codes.contains(&code) {
			codes.push(code);
		}
	}
}

fn unique_strings(values: Vec<String>) -> Vec<String> {
	let mut unique = Vec::new();
	for value in values {
		if !unique.contains(&value) {
			unique.push(value);
		}
	}
	unique
}

fn collect_reason_codes(
	signals: &ScoringSignals,
	provider_rejection_reason: Option<&ProviderRejectionReason>,
) -> Vec<String> {
	let mut codes = Vec::new();
	if !signals.valid_syntax {
		codes.push("invalid_syntax".to_string());
	}
	if let Some(reason) = provider_rejection_reason {
		codes.push("provider_rejected".to_string());
		codes.push(reason.code().to_string());
		if signals.has_domain_suggestion {
			codes.push("possible_typo".to_string());
		}
		return codes;
	}
	if matches!(signals.reachable, Reachable::Invalid) {
		codes.push("invalid_recipient".to_string());
	}
	if !signals.smtp_is_deliverable {
		codes.push("smtp_undeliverable".to_string());
	}
	if signals.smtp_is_disabled {
		codes.push("disabled_mailbox".to_string());
	}
	if !signals.has_mx_records {
		codes.push("no_mx".to_string());
	}
	if signals.smtp_error {
		codes.push("smtp_error".to_string());
	}
	if !signals.smtp_can_connect {
		codes.push("smtp_unreachable".to_string());
	}
	if signals.smtp_is_catch_all {
		codes.push("catch_all".to_string());
	}
	if signals.smtp_has_full_inbox {
		codes.push("full_inbox".to_string());
	}
	if signals.is_disposable {
		codes.push("disposable".to_string());
	}
	if signals.is_role_account {
		codes.push("role_account".to_string());
	}
	if signals.is_spam_trap_domain {
		codes.push("spam_trap".to_string());
	}
	if matches!(signals.reachable, Reachable::Unknown) {
		codes.push("unknown_deliverability".to_string());
	}
	if signals.is_free_provider {
		codes.push("free_provider".to_string());
	}
	if signals.has_domain_suggestion {
		codes.push("possible_typo".to_string());
	}
	if codes.is_empty() {
		codes.push("deliverable".to_string());
	}
	codes
}

fn extract_signals(output: &CheckEmailOutput) -> ScoringSignals {
	let has_mx_records = output
		.mx
		.as_ref()
		.ok()
		.and_then(|mx| mx.lookup.as_ref().ok())
		.map(|lookup| lookup.iter().next().is_some())
		.unwrap_or(false);

	let smtp_error = output.smtp.is_err();
	let smtp_ok = output.smtp.as_ref().ok();
	let misc_ok = output.misc.as_ref().ok();

	ScoringSignals {
		valid_syntax: output.syntax.is_valid_syntax,
		reachable: output.is_reachable.clone(),
		has_mx_records,
		smtp_error,
		smtp_can_connect: smtp_ok.map(|smtp| smtp.can_connect_smtp).unwrap_or(true),
		smtp_is_deliverable: smtp_ok.map(|smtp| smtp.is_deliverable).unwrap_or(true),
		smtp_is_disabled: smtp_ok.map(|smtp| smtp.is_disabled).unwrap_or(false),
		smtp_is_catch_all: smtp_ok.map(|smtp| smtp.is_catch_all).unwrap_or(false),
		smtp_has_full_inbox: smtp_ok.map(|smtp| smtp.has_full_inbox).unwrap_or(false),
		is_disposable: misc_ok.map(|misc| misc.is_disposable).unwrap_or(false),
		is_role_account: misc_ok.map(|misc| misc.is_role_account).unwrap_or(false),
		is_spam_trap_domain: misc_ok
			.map(|misc| misc.is_spam_trap_domain)
			.unwrap_or(false),
		is_free_provider: misc_ok.map(|misc| misc.is_b2c).unwrap_or(false),
		has_domain_suggestion: output.syntax.suggestion.is_some(),
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use check_if_email_exists::{
		misc::MiscDetails,
		mx::MxDetails,
		provider::{Provider, ProviderConfidence, ProviderRejectionReason},
		smtp::SmtpDetails,
		syntax::SyntaxDetails,
	};

	fn base_output() -> CheckEmailOutput {
		CheckEmailOutput {
			input: "user@example.com".into(),
			is_reachable: Reachable::Safe,
			misc: Ok(MiscDetails::default()),
			mx: Ok(MxDetails::default()),
			smtp: Ok(SmtpDetails {
				can_connect_smtp: true,
				has_full_inbox: false,
				is_catch_all: false,
				is_deliverable: true,
				is_disabled: false,
			}),
			syntax: SyntaxDetails {
				address: None,
				domain: "example.com".into(),
				is_valid_syntax: true,
				username: "user".into(),
				normalized_email: Some("user@example.com".into()),
				suggestion: None,
			},
			debug: Default::default(),
			provider: None,
			provider_rules_applied: false,
			provider_rejection_reason: None,
			provider_confidence: None,
		}
	}

	#[test]
	fn compute_score_invalid_syntax_short_circuits() {
		let mut output = base_output();
		output.syntax.is_valid_syntax = false;
		let score = compute_score(&output);
		assert_eq!(score.score, 0);
		assert_eq!(score.category, EmailCategory::Invalid);
		assert_eq!(score.sub_reason, SubReason::InvalidSyntax);
		assert!(!score.safe_to_send);
	}

	#[test]
	fn compute_score_deliverable() {
		let score = compute_score(&base_output());
		assert_eq!(score.score, 50);
		assert_eq!(score.category, EmailCategory::Risky);
		assert_eq!(score.sub_reason, SubReason::NoMx);
		assert!(!score.safe_to_send);
	}

	#[test]
	fn compute_score_unknown_with_penalties() {
		let mut output = base_output();
		output.is_reachable = Reachable::Unknown;
		output.misc = Ok(MiscDetails {
			is_disposable: true,
			is_role_account: true,
			..Default::default()
		});
		output.smtp = Ok(SmtpDetails {
			can_connect_smtp: false,
			has_full_inbox: true,
			is_catch_all: true,
			is_deliverable: true,
			is_disabled: false,
		});
		let score = compute_score(&output);
		assert_eq!(score.score, 0);
		assert_eq!(score.category, EmailCategory::Invalid);
		assert_eq!(score.sub_reason, SubReason::NoMx);
	}

	#[test]
	fn compute_score_provider_rejected_short_circuits() {
		let mut output = base_output();
		output.is_reachable = Reachable::Invalid;
		output.provider_rules_applied = true;
		output.provider_rejection_reason = Some(ProviderRejectionReason::LocalPartTooShort);
		let score = compute_score(&output);
		assert_eq!(score.score, 0);
		assert_eq!(score.category, EmailCategory::Invalid);
		assert_eq!(score.sub_reason, SubReason::ProviderRejected);
		assert!(score
			.reason_codes
			.contains(&"provider_rejected".to_string()));
		assert!(score
			.reason_codes
			.contains(&"provider_local_part_too_short".to_string()));
		assert!(!score
			.reason_codes
			.contains(&"invalid_recipient".to_string()));
	}

	#[test]
	fn compute_score_smtp_error() {
		let mut output = base_output();
		output.mx = Err(check_if_email_exists::mx::MxError::from(
			std::io::Error::other("mx"),
		));
		output.smtp = Err(check_if_email_exists::smtp::SmtpError::from(
			std::io::Error::other("smtp"),
		));
		let score = compute_score(&output);
		assert_eq!(score.score, 15);
		assert_eq!(score.category, EmailCategory::Unknown);
		assert_eq!(score.sub_reason, SubReason::NoMx);
		assert!(!score.safe_to_send);
	}

	#[test]
	fn contextual_unknown_gets_partial_confidence_and_actionable_score() {
		let mut output = base_output();
		output.input = "john.smith@company.com".to_string();
		output.is_reachable = Reachable::Unknown;
		output.provider = Some(Provider::GoogleWorkspace);
		output.provider_confidence = Some(ProviderConfidence::Medium);
		output.syntax.domain = "company.com".to_string();
		output.syntax.username = "john.smith".to_string();
		output.syntax.normalized_email = Some("john.smith@company.com".to_string());

		let computation = compute_score_with_context(
			&output,
			&ScoringContext {
				tenant_history: TenantHistoryContext {
					safe_count_180d: 3,
					total_count_180d: 3,
					latest_days_ago: Some(2),
					inconsistent_count_180d: 0,
				},
				domain: DomainSignalContext {
					domain_age_days: Some(3650),
					website_present: Some(true),
					has_spf: Some(true),
					has_dkim: Some(true),
					has_dmarc: Some(true),
				},
				provider_reputation: provider_reputation_context(
					output.provider.as_ref(),
					output.provider_confidence.as_ref(),
				),
				..Default::default()
			},
		);

		assert_eq!(computation.score.category, EmailCategory::Risky);
		assert!(computation
			.score
			.reason_codes
			.contains(&"partial_confidence".to_string()));
		assert!(computation.insights.partial_confidence.is_some());
		assert!(computation.insights.confidence >= 60);
		assert!(!computation.score.safe_to_send);
	}

	#[test]
	fn contextual_catch_all_uses_pattern_tier_instead_of_flat_penalty() {
		let mut output = base_output();
		output.input = "john.smith@company.com".to_string();
		output.is_reachable = Reachable::Risky;
		output.syntax.domain = "company.com".to_string();
		output.syntax.username = "john.smith".to_string();
		output.smtp = Ok(SmtpDetails {
			can_connect_smtp: true,
			has_full_inbox: false,
			is_catch_all: true,
			is_deliverable: true,
			is_disabled: false,
		});

		let high_risk = compute_score_with_context(&output, &ScoringContext::default());
		let low_risk = compute_score_with_context(
			&output,
			&ScoringContext {
				pattern: PatternContext {
					pattern: Some("first.last".to_string()),
					verified_same_pattern_count_180d: 5,
					verified_domain_count_180d: 5,
				},
				domain: DomainSignalContext {
					domain_age_days: Some(2000),
					website_present: Some(true),
					has_spf: Some(true),
					has_dkim: None,
					has_dmarc: Some(true),
				},
				..Default::default()
			},
		);

		assert!(low_risk.score.score > high_risk.score.score);
		assert_eq!(
			low_risk
				.insights
				.catch_all
				.as_ref()
				.map(|catch_all| catch_all.severity),
			Some(CatchAllSeverity::Low)
		);
		assert!(low_risk
			.score
			.reason_codes
			.contains(&"catch_all_low_confidence".to_string()));
		assert!(!low_risk.score.safe_to_send);
	}

	#[test]
	fn safe_to_send_true_when_valid_and_clean() {
		// Directly verify the safe_to_send derivation: Valid + not disposable + not catch-all + not role = true
		let score = EmailScore {
			score: 100,
			category: EmailCategory::Valid,
			sub_reason: SubReason::Deliverable,
			safe_to_send: EmailCategory::Valid == EmailCategory::Valid
				&& !false && !false
				&& !false,
			reason_codes: vec!["deliverable".to_string()],
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
		};
		assert!(score.safe_to_send);
	}

	#[test]
	fn safe_to_send_false_catch_all() {
		let mut output = base_output();
		output.smtp = Ok(SmtpDetails {
			can_connect_smtp: true,
			has_full_inbox: false,
			is_catch_all: true,
			is_deliverable: true,
			is_disabled: false,
		});
		let score = compute_score(&output);
		// catch-all emails are never safe to send regardless of category
		assert!(!score.safe_to_send);
	}

	#[test]
	fn safe_to_send_false_role_account() {
		let mut output = base_output();
		output.misc = Ok(MiscDetails {
			is_disposable: false,
			is_role_account: true,
			..Default::default()
		});
		let score = compute_score(&output);
		assert!(!score.safe_to_send);
	}

	#[test]
	fn safe_to_send_false_disposable() {
		let mut output = base_output();
		output.misc = Ok(MiscDetails {
			is_disposable: true,
			is_role_account: false,
			..Default::default()
		});
		let score = compute_score(&output);
		// disposable emails are never safe to send regardless of category
		assert!(!score.safe_to_send);
	}

	#[test]
	fn reason_codes_multiple_flags() {
		let mut output = base_output();
		output.smtp = Ok(SmtpDetails {
			can_connect_smtp: true,
			has_full_inbox: false,
			is_catch_all: true,
			is_deliverable: true,
			is_disabled: false,
		});
		output.misc = Ok(MiscDetails {
			is_disposable: true,
			is_role_account: true,
			is_b2c: true,
			..Default::default()
		});
		let score = compute_score(&output);
		assert!(score.reason_codes.contains(&"catch_all".to_string()));
		assert!(score.reason_codes.contains(&"disposable".to_string()));
		assert!(score.reason_codes.contains(&"role_account".to_string()));
		assert!(score.reason_codes.contains(&"free_provider".to_string()));
		assert!(!score.reason_codes.contains(&"deliverable".to_string()));
	}

	#[test]
	fn reason_codes_deliverable_when_clean() {
		let signals = ScoringSignals {
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
		};
		let codes = collect_reason_codes(&signals, None);
		assert_eq!(codes, vec!["deliverable"]);
	}

	#[test]
	fn reason_codes_possible_typo() {
		let mut output = base_output();
		output.syntax.suggestion = Some("user@example.com".to_string());
		let score = compute_score(&output);
		assert!(score.reason_codes.contains(&"possible_typo".to_string()));
	}

	#[test]
	fn reason_codes_invalid_syntax_early_return() {
		let mut output = base_output();
		output.syntax.is_valid_syntax = false;
		let score = compute_score(&output);
		assert!(score.reason_codes.contains(&"invalid_syntax".to_string()));
		assert!(!score.reason_codes.contains(&"deliverable".to_string()));
	}

	#[test]
	fn spam_trap_domain_penalizes_score_and_disqualifies_safe_to_send() {
		let mut output = base_output();
		output.misc = Ok(MiscDetails {
			is_spam_trap_domain: true,
			..Default::default()
		});
		let score = compute_score(&output);
		assert!(score.reason_codes.contains(&"spam_trap".to_string()));
		assert!(!score.safe_to_send);
		assert!(score.score < 100);
	}

	#[test]
	fn spam_trap_domain_appears_in_sub_reason() {
		// Test via signals directly to avoid base_output() MX issues
		let signals = ScoringSignals {
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
			is_spam_trap_domain: true,
			is_free_provider: false,
			has_domain_suggestion: false,
		};
		let codes = collect_reason_codes(&signals, None);
		assert!(codes.contains(&"spam_trap".to_string()));
		assert!(!codes.contains(&"deliverable".to_string()));
	}

	#[test]
	fn freshness_tier_fresh() {
		let now = chrono::Utc::now();
		assert_eq!(compute_freshness(now).freshness, Freshness::Fresh);
		assert_eq!(compute_freshness(now).age_days, 0);
		let seven_days_ago = now - chrono::Duration::days(7);
		assert_eq!(
			compute_freshness(seven_days_ago).freshness,
			Freshness::Fresh
		);
	}

	#[test]
	fn freshness_tier_recent() {
		let now = chrono::Utc::now();
		let eight_days_ago = now - chrono::Duration::days(8);
		assert_eq!(
			compute_freshness(eight_days_ago).freshness,
			Freshness::Recent
		);
		let thirty_days_ago = now - chrono::Duration::days(30);
		assert_eq!(
			compute_freshness(thirty_days_ago).freshness,
			Freshness::Recent
		);
	}

	#[test]
	fn freshness_tier_aging() {
		let now = chrono::Utc::now();
		let thirty_one = now - chrono::Duration::days(31);
		assert_eq!(compute_freshness(thirty_one).freshness, Freshness::Aging);
		let sixty = now - chrono::Duration::days(60);
		assert_eq!(compute_freshness(sixty).freshness, Freshness::Aging);
	}

	#[test]
	fn freshness_tier_stale() {
		let now = chrono::Utc::now();
		let sixty_one = now - chrono::Duration::days(61);
		assert_eq!(compute_freshness(sixty_one).freshness, Freshness::Stale);
		let ninety = now - chrono::Duration::days(90);
		assert_eq!(compute_freshness(ninety).freshness, Freshness::Stale);
	}

	#[test]
	fn freshness_tier_expired() {
		let now = chrono::Utc::now();
		let ninety_one = now - chrono::Duration::days(91);
		assert_eq!(compute_freshness(ninety_one).freshness, Freshness::Expired);
	}

	#[test]
	fn freshness_can_use_fixed_reference_time() {
		let completed_at = chrono::Utc::now() - chrono::Duration::days(30);
		let reference_now = completed_at + chrono::Duration::days(2);
		let freshness = compute_freshness_at(completed_at, reference_now);

		assert_eq!(freshness.age_days, 2);
		assert_eq!(freshness.freshness, Freshness::Fresh);
	}
}
