use crate::config::BounceRiskConfig;
use crate::http::v1::lists::canonicalize::canonicalize_email;
use crate::reputation::checker::fetch_domain_info_with_client;
use crate::reputation::dns_records::check_dns_records;
use crate::reputation::models::{DnsRecordResults, DomainInfo};
use crate::scoring::EmailScore;
use anyhow::Context;
use check_if_email_exists::mx::{is_gmail, is_hotmail, is_mimecast, is_proofpoint, is_yahoo};
use check_if_email_exists::{CheckEmailOutput, LOG_TARGET};
use chrono::{DateTime, Duration as ChronoDuration, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use std::cmp::Reverse;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant, SystemTime};
use tracing::warn;
use uuid::Uuid;

const INFRA_TTL_HOURS: i64 = 24;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum BounceRiskCategory {
	Safe,
	Low,
	Medium,
	High,
	Dangerous,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RiskDirection {
	IncreasesRisk,
	DecreasesRisk,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum RecommendedAction {
	Send,
	SendWithCaution,
	VerifyManually,
	DoNotSend,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RiskFactor {
	pub signal: String,
	pub contribution: i16,
	pub description: String,
	pub direction: RiskDirection,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BounceRiskAssessment {
	pub score: i16,
	pub category: BounceRiskCategory,
	pub confidence: f64,
	pub action: RecommendedAction,
	pub model_version: String,
	pub scored_at: String,
	pub risk_factors: Vec<RiskFactor>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum HistoryConsistency {
	Consistent,
	Inconsistent,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SignalBundle {
	pub provider_from_mx: Option<String>,
	pub free_provider: Option<bool>,
	pub mx_count: Option<i32>,
	pub mx_priority_spread: Option<i32>,
	pub domain_age_days: Option<i64>,
	pub website_present: Option<bool>,
	pub has_spf: Option<bool>,
	pub has_dkim: Option<bool>,
	pub has_dmarc: Option<bool>,
	pub historical_verification_count: Option<u32>,
	pub result_consistency: Option<HistoryConsistency>,
	pub days_since_last_verification: Option<i64>,
	pub hard_override_reason: Option<String>,
}

#[derive(Debug, Clone)]
pub struct BounceRiskResult {
	pub assessment: BounceRiskAssessment,
	pub signals: SignalBundle,
}

#[derive(Debug, Clone)]
pub struct BounceRiskRequestContext {
	pub tenant_id: Option<Uuid>,
	pub completed_at: DateTime<Utc>,
	pub allow_external_enrichment: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BounceRiskModelConfig {
	pub model_version: String,
	pub hard_overrides: HardOverrideConfig,
	pub thresholds: ThresholdConfig,
	pub weights: WeightConfig,
	pub confidence_weights: ConfidenceWeightConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HardOverrideConfig {
	pub invalid_syntax: i16,
	pub invalid_recipient: i16,
	pub smtp_undeliverable: i16,
	pub disabled_mailbox: i16,
	pub disposable: i16,
	pub mailbox_full: i16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ThresholdConfig {
	pub safe_max: i16,
	pub low_max: i16,
	pub medium_max: i16,
	pub high_max: i16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WeightConfig {
	pub base_risk: i16,
	pub corporate_catch_all: i16,
	pub free_provider_catch_all: i16,
	pub role_account: i16,
	pub smtp_unknown_error: i16,
	pub missing_spf: i16,
	pub missing_dkim: i16,
	pub missing_dmarc: i16,
	pub missing_website: i16,
	pub consistent_history: i16,
	pub inconsistent_history: i16,
	pub domain_age: DomainAgeWeights,
	pub last_verification: LastVerificationWeights,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DomainAgeWeights {
	pub under_30_days: i16,
	pub under_90_days: i16,
	pub under_365_days: i16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LastVerificationWeights {
	pub within_7_days: i16,
	pub within_30_days: i16,
	pub over_90_days: i16,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConfidenceWeightConfig {
	pub verification_core: f64,
	pub provider_from_mx: f64,
	pub free_provider_flag: f64,
	pub mx_count: f64,
	pub mx_priority_spread: f64,
	pub domain_age: f64,
	pub website_presence: f64,
	pub spf: f64,
	pub dkim: f64,
	pub dmarc: f64,
	pub history_count: f64,
	pub history_consistency: f64,
	pub last_verification: f64,
}

impl Default for BounceRiskModelConfig {
	fn default() -> Self {
		Self {
			model_version: "bounce-risk-phase1-v1".to_string(),
			hard_overrides: HardOverrideConfig {
				invalid_syntax: 98,
				invalid_recipient: 98,
				smtp_undeliverable: 98,
				disabled_mailbox: 98,
				disposable: 90,
				mailbox_full: 75,
			},
			thresholds: ThresholdConfig {
				safe_max: 20,
				low_max: 40,
				medium_max: 60,
				high_max: 80,
			},
			weights: WeightConfig {
				base_risk: 20,
				corporate_catch_all: 20,
				free_provider_catch_all: 12,
				role_account: 10,
				smtp_unknown_error: 20,
				missing_spf: 6,
				missing_dkim: 2,
				missing_dmarc: 8,
				missing_website: 6,
				consistent_history: -10,
				inconsistent_history: 15,
				domain_age: DomainAgeWeights {
					under_30_days: 25,
					under_90_days: 15,
					under_365_days: 5,
				},
				last_verification: LastVerificationWeights {
					within_7_days: -10,
					within_30_days: -5,
					over_90_days: 8,
				},
			},
			confidence_weights: ConfidenceWeightConfig {
				verification_core: 2.0,
				provider_from_mx: 0.5,
				free_provider_flag: 0.5,
				mx_count: 0.5,
				mx_priority_spread: 0.5,
				domain_age: 1.0,
				website_presence: 0.75,
				spf: 0.75,
				dkim: 0.5,
				dmarc: 0.75,
				history_count: 0.5,
				history_consistency: 1.0,
				last_verification: 1.0,
			},
		}
	}
}

#[derive(Debug)]
pub struct BounceRiskService {
	runtime: BounceRiskConfig,
	model_state: Arc<RwLock<ModelState>>,
}

#[derive(Debug)]
struct ModelState {
	model: Arc<BounceRiskModelConfig>,
	last_checked_at: Option<Instant>,
	last_modified_at: Option<SystemTime>,
}

impl Default for BounceRiskService {
	fn default() -> Self {
		Self::new(BounceRiskConfig::default())
	}
}

impl BounceRiskService {
	pub fn new(runtime: BounceRiskConfig) -> Self {
		Self {
			runtime,
			model_state: Arc::new(RwLock::new(ModelState {
				model: Arc::new(BounceRiskModelConfig::default()),
				last_checked_at: None,
				last_modified_at: None,
			})),
		}
	}

	pub async fn assess(
		&self,
		output: &CheckEmailOutput,
		email_score: &EmailScore,
		read_pool: Option<&PgPool>,
		write_pool: Option<&PgPool>,
		context: &BounceRiskRequestContext,
	) -> Result<Option<BounceRiskResult>, anyhow::Error> {
		if !self.runtime.enabled {
			return Ok(None);
		}

		let model = self.current_model()?;
		let signals = collect_signal_bundle(
			output,
			email_score,
			read_pool,
			write_pool,
			&self.runtime,
			context,
		)
		.await?;
		let assessment = compute_assessment(email_score, &signals, &model, context.completed_at);
		Ok(Some(BounceRiskResult {
			assessment,
			signals,
		}))
	}

	fn current_model(&self) -> Result<Arc<BounceRiskModelConfig>, anyhow::Error> {
		let min_reload = self.runtime.reload_interval_seconds.max(30);
		let reload_interval = Duration::from_secs(min_reload);
		let path = resolve_model_path(&self.runtime.config_path);
		let mut state = self.model_state.write().expect("bounce risk model lock");
		if state
			.last_checked_at
			.map(|checked_at| checked_at.elapsed() < reload_interval)
			.unwrap_or(false)
		{
			return Ok(Arc::clone(&state.model));
		}
		state.last_checked_at = Some(Instant::now());

		let modified_at = fs::metadata(&path)
			.ok()
			.and_then(|metadata| metadata.modified().ok());
		if modified_at == state.last_modified_at {
			return Ok(Arc::clone(&state.model));
		}

		if let Some(modified_at) = modified_at {
			let raw = fs::read_to_string(&path).with_context(|| {
				format!("reading bounce-risk model config at {}", path.display())
			})?;
			match toml::from_str::<BounceRiskModelConfig>(&raw) {
				Ok(model) => {
					state.model = Arc::new(model);
					state.last_modified_at = Some(modified_at);
				}
				Err(error) => {
					warn!(
						target: LOG_TARGET,
						path = %path.display(),
						error = ?error,
						"Failed to reload bounce-risk model config, keeping last good config"
					);
				}
			}
		} else {
			warn!(
				target: LOG_TARGET,
				path = %path.display(),
				"Bounce-risk model config file was not found, keeping last good config"
			);
		}

		Ok(Arc::clone(&state.model))
	}
}

fn resolve_model_path(config_path: &str) -> PathBuf {
	let path = Path::new(config_path);
	if path.is_absolute() || path.exists() {
		return path.to_path_buf();
	}

	let manifest_relative = Path::new(env!("CARGO_MANIFEST_DIR")).join(config_path);
	if manifest_relative.exists() {
		return manifest_relative;
	}

	path.to_path_buf()
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct DomainInfraCachePayload {
	dns_records: DnsRecordResults,
	website_present: Option<bool>,
}

#[derive(Debug)]
struct DomainSignals {
	provider_from_mx: Option<String>,
	mx_count: Option<i32>,
	mx_priority_spread: Option<i32>,
	domain_info: Option<DomainInfo>,
	dns_records: Option<DnsRecordResults>,
	website_present: Option<bool>,
}

#[derive(Debug)]
struct HistorySignals {
	count: Option<u32>,
	consistency: Option<HistoryConsistency>,
	days_since_last_verification: Option<i64>,
}

#[derive(Debug)]
struct HistoryRow {
	category: Option<String>,
	completed_at: DateTime<Utc>,
}

async fn collect_signal_bundle(
	output: &CheckEmailOutput,
	email_score: &EmailScore,
	read_pool: Option<&PgPool>,
	write_pool: Option<&PgPool>,
	runtime: &BounceRiskConfig,
	context: &BounceRiskRequestContext,
) -> Result<SignalBundle, anyhow::Error> {
	let hard_override_reason = detect_hard_override_reason(email_score);
	let (provider_from_mx, mx_count, mx_priority_spread) = collect_mx_signals(output);

	let history_signals = if context.allow_external_enrichment {
		collect_history_signals(
			read_pool,
			context.tenant_id,
			&output.input,
			&email_score.category,
			runtime.history_limit,
		)
		.await?
	} else {
		HistorySignals {
			count: None,
			consistency: None,
			days_since_last_verification: None,
		}
	};

	let domain_signals = if context.allow_external_enrichment {
		collect_domain_signals(output, read_pool, write_pool, runtime).await?
	} else {
		DomainSignals {
			provider_from_mx,
			mx_count,
			mx_priority_spread,
			domain_info: None,
			dns_records: None,
			website_present: None,
		}
	};

	Ok(SignalBundle {
		provider_from_mx: domain_signals.provider_from_mx,
		free_provider: output.misc.as_ref().ok().map(|misc| misc.is_b2c),
		mx_count: domain_signals.mx_count,
		mx_priority_spread: domain_signals.mx_priority_spread,
		domain_age_days: domain_signals
			.domain_info
			.as_ref()
			.and_then(|info| info.domain_age_days),
		website_present: domain_signals.website_present,
		has_spf: domain_signals
			.dns_records
			.as_ref()
			.map(|dns_records| dns_records.has_spf),
		has_dkim: domain_signals
			.dns_records
			.as_ref()
			.map(|dns_records| dns_records.has_dkim),
		has_dmarc: domain_signals
			.dns_records
			.as_ref()
			.map(|dns_records| dns_records.has_dmarc),
		historical_verification_count: history_signals.count,
		result_consistency: history_signals.consistency,
		days_since_last_verification: history_signals.days_since_last_verification,
		hard_override_reason,
	})
}

async fn collect_domain_signals(
	output: &CheckEmailOutput,
	read_pool: Option<&PgPool>,
	write_pool: Option<&PgPool>,
	runtime: &BounceRiskConfig,
) -> Result<DomainSignals, anyhow::Error> {
	let (provider_from_mx, mx_count, mx_priority_spread) = collect_mx_signals(output);
	let domain = output.syntax.domain.trim().to_lowercase();
	if domain.is_empty() {
		return Ok(DomainSignals {
			provider_from_mx,
			mx_count,
			mx_priority_spread,
			domain_info: None,
			dns_records: None,
			website_present: None,
		});
	}

	let cached = if let Some(pool) = read_pool {
		load_domain_cache(pool, &domain).await?
	} else {
		None
	};

	let mut domain_info = cached.as_ref().and_then(|entry| entry.domain_info.clone());
	let mut dns_records = cached.as_ref().and_then(|entry| entry.dns_records.clone());
	let mut website_present = cached.as_ref().and_then(|entry| entry.website_present);
	let mut rdap_refreshed = false;
	let mut infra_refreshed = false;

	let rdap_client = reqwest::Client::builder()
		.timeout(Duration::from_millis(runtime.network_timeout_ms))
		.build()?;

	if domain_info.is_none() {
		match fetch_domain_info_with_client(&rdap_client, &domain).await {
			Ok(fetched) => {
				domain_info = Some(fetched);
				rdap_refreshed = true;
			}
			Err(error) => {
				warn!(
					target: LOG_TARGET,
					domain = %domain,
					error = ?error,
					"Failed to fetch RDAP domain information for bounce-risk scoring"
				);
			}
		}
	}

	let infra_stale = cached
		.as_ref()
		.map(|entry| entry.infra_stale)
		.unwrap_or(true);
	if infra_stale || dns_records.is_none() || website_present.is_none() {
		let website_client = reqwest::Client::builder()
			.timeout(Duration::from_millis(runtime.website_probe_timeout_ms))
			.redirect(reqwest::redirect::Policy::limited(5))
			.build()?;
		let fetched_dns = check_dns_records(&domain).await.ok();
		let fetched_website = probe_website_presence(&website_client, &domain).await;
		infra_refreshed = fetched_dns.is_some() || fetched_website.is_some();
		dns_records = fetched_dns.or(dns_records);
		website_present = fetched_website.or(website_present);
	}

	if rdap_refreshed || infra_refreshed {
		if let Some(pool) = write_pool {
			store_domain_cache(
				pool,
				&domain,
				rdap_refreshed.then(|| domain_info.clone()).flatten(),
				infra_refreshed.then(|| dns_records.clone()).flatten(),
				infra_refreshed.then_some(website_present).flatten(),
			)
			.await?;
		}
	}

	Ok(DomainSignals {
		provider_from_mx,
		mx_count,
		mx_priority_spread,
		domain_info,
		dns_records,
		website_present,
	})
}

fn collect_mx_signals(output: &CheckEmailOutput) -> (Option<String>, Option<i32>, Option<i32>) {
	let Some(mx_details) = output.mx.as_ref().ok() else {
		return (None, None, None);
	};

	let Ok(lookup) = mx_details.lookup.as_ref() else {
		return (None, None, None);
	};

	let records: Vec<_> = lookup.iter().collect();
	let provider = records
		.iter()
		.min_by_key(|record| record.preference())
		.map(|record| derive_provider_from_mx_host(record.exchange().to_string().as_str()));

	let mx_count = Some(records.len() as i32);
	let mx_priority_spread = if records.is_empty() {
		Some(0)
	} else {
		let min = records
			.iter()
			.map(|record| i32::from(record.preference()))
			.min();
		let max = records
			.iter()
			.map(|record| i32::from(record.preference()))
			.max();
		match (min, max) {
			(Some(min), Some(max)) => Some(max - min),
			_ => Some(0),
		}
	};

	(provider, mx_count, mx_priority_spread)
}

async fn collect_history_signals(
	read_pool: Option<&PgPool>,
	tenant_id: Option<Uuid>,
	email: &str,
	current_category: &impl Serialize,
	history_limit: u32,
) -> Result<HistorySignals, anyhow::Error> {
	let Some(pool) = read_pool else {
		return Ok(HistorySignals {
			count: None,
			consistency: None,
			days_since_last_verification: None,
		});
	};
	let Some(tenant_id) = tenant_id else {
		return Ok(HistorySignals {
			count: None,
			consistency: None,
			days_since_last_verification: None,
		});
	};

	let canonical_email = canonicalize_email(email);
	let lowered_email = email.trim().to_lowercase();
	let current_category = serde_json::to_string(current_category)?
		.trim_matches('"')
		.to_string();

	let rows = sqlx::query(
		r#"
		SELECT
			COALESCE(score_category, result->'score'->>'category') AS category,
			completed_at
		FROM v1_task_result
		WHERE tenant_id = $1
		  AND completed_at IS NOT NULL
		  AND (
			($2::TEXT IS NOT NULL AND canonical_email = $2)
			OR (
				canonical_email IS NULL
				AND lower(COALESCE(payload->'input'->>'to_email', result->>'input', '')) = $3
			)
		  )
		ORDER BY completed_at DESC
		LIMIT $4
		"#,
	)
	.bind(tenant_id)
	.bind(&canonical_email)
	.bind(&lowered_email)
	.bind(i64::from(history_limit))
	.fetch_all(pool)
	.await?;

	let history_rows: Vec<HistoryRow> = rows
		.into_iter()
		.filter_map(|row| {
			row.get::<Option<DateTime<Utc>>, _>("completed_at")
				.map(|completed_at| HistoryRow {
					category: row.get::<Option<String>, _>("category"),
					completed_at,
				})
		})
		.collect();

	Ok(summarize_history(&history_rows, &current_category))
}

fn summarize_history(rows: &[HistoryRow], current_category: &str) -> HistorySignals {
	if rows.is_empty() {
		return HistorySignals {
			count: Some(0),
			consistency: None,
			days_since_last_verification: None,
		};
	}

	let categories: BTreeSet<String> = rows.iter().filter_map(|row| row.category.clone()).collect();
	let consistency = if categories.is_empty() {
		None
	} else if categories.len() == 1 && categories.contains(current_category) {
		Some(HistoryConsistency::Consistent)
	} else {
		Some(HistoryConsistency::Inconsistent)
	};

	let most_recent = rows.iter().map(|row| row.completed_at).max();
	let days_since_last_verification =
		most_recent.map(|completed_at| (Utc::now() - completed_at).num_days().max(0));

	HistorySignals {
		count: Some(rows.len() as u32),
		consistency,
		days_since_last_verification,
	}
}

#[derive(Debug)]
struct CachedDomainRow {
	domain_info: Option<DomainInfo>,
	dns_records: Option<DnsRecordResults>,
	website_present: Option<bool>,
	infra_stale: bool,
}

async fn load_domain_cache(
	pool: &PgPool,
	domain: &str,
) -> Result<Option<CachedDomainRow>, anyhow::Error> {
	let Some(row) = sqlx::query(
		r#"
		SELECT rdap_payload, infra_payload, rdap_fetched_at, infra_fetched_at
		FROM bounce_risk_domain_cache
		WHERE domain = $1
		"#,
	)
	.bind(domain)
	.fetch_optional(pool)
	.await?
	else {
		return Ok(None);
	};

	let rdap_payload = row.get::<Option<serde_json::Value>, _>("rdap_payload");
	let infra_payload = row.get::<Option<serde_json::Value>, _>("infra_payload");
	let infra_fetched_at = row.get::<Option<DateTime<Utc>>, _>("infra_fetched_at");
	let infra_stale = infra_fetched_at
		.map(|fetched_at| fetched_at + ChronoDuration::hours(INFRA_TTL_HOURS) < Utc::now())
		.unwrap_or(true);

	let domain_info =
		rdap_payload.and_then(|value| serde_json::from_value::<DomainInfo>(value).ok());
	let infra = infra_payload
		.and_then(|value| serde_json::from_value::<DomainInfraCachePayload>(value).ok());

	Ok(Some(CachedDomainRow {
		domain_info,
		dns_records: infra.as_ref().map(|payload| payload.dns_records.clone()),
		website_present: infra.and_then(|payload| payload.website_present),
		infra_stale,
	}))
}

async fn store_domain_cache(
	pool: &PgPool,
	domain: &str,
	domain_info: Option<DomainInfo>,
	dns_records: Option<DnsRecordResults>,
	website_present: Option<bool>,
) -> Result<(), anyhow::Error> {
	let rdap_payload = domain_info.as_ref().map(serde_json::to_value).transpose()?;
	let infra_payload = if let Some(dns_records) = dns_records {
		Some(serde_json::to_value(DomainInfraCachePayload {
			dns_records,
			website_present,
		})?)
	} else {
		None
	};

	sqlx::query(
		r#"
		INSERT INTO bounce_risk_domain_cache (
			domain,
			rdap_payload,
			infra_payload,
			rdap_fetched_at,
			infra_fetched_at,
			updated_at
		)
		VALUES (
			$1,
			$2,
			$3,
			CASE WHEN $2 IS NULL THEN NULL ELSE NOW() END,
			CASE WHEN $3 IS NULL THEN NULL ELSE NOW() END,
			NOW()
		)
		ON CONFLICT (domain) DO UPDATE SET
			rdap_payload = COALESCE(EXCLUDED.rdap_payload, bounce_risk_domain_cache.rdap_payload),
			infra_payload = COALESCE(EXCLUDED.infra_payload, bounce_risk_domain_cache.infra_payload),
			rdap_fetched_at = CASE
				WHEN EXCLUDED.rdap_payload IS NULL THEN bounce_risk_domain_cache.rdap_fetched_at
				ELSE NOW()
			END,
			infra_fetched_at = CASE
				WHEN EXCLUDED.infra_payload IS NULL THEN bounce_risk_domain_cache.infra_fetched_at
				ELSE NOW()
			END,
			updated_at = NOW()
		"#,
	)
	.bind(domain)
	.bind(rdap_payload)
	.bind(infra_payload)
	.execute(pool)
	.await?;

	Ok(())
}

fn compute_assessment(
	email_score: &EmailScore,
	signals: &SignalBundle,
	model: &BounceRiskModelConfig,
	completed_at: DateTime<Utc>,
) -> BounceRiskAssessment {
	if let Some(override_reason) = signals.hard_override_reason.as_deref() {
		let score = match override_reason {
			"invalid_syntax" => model.hard_overrides.invalid_syntax,
			"invalid_recipient" => model.hard_overrides.invalid_recipient,
			"smtp_undeliverable" => model.hard_overrides.smtp_undeliverable,
			"disabled_mailbox" => model.hard_overrides.disabled_mailbox,
			"disposable" => model.hard_overrides.disposable,
			"mailbox_full" => model.hard_overrides.mailbox_full,
			_ => model.hard_overrides.invalid_recipient,
		};
		let factor = RiskFactor {
			signal: override_reason.to_string(),
			contribution: score,
			description: format!("Hard override applied for {override_reason}"),
			direction: RiskDirection::IncreasesRisk,
		};
		let (category, action) = category_and_action(score, &model.thresholds);
		return BounceRiskAssessment {
			score,
			category,
			confidence: compute_confidence(signals, &model.confidence_weights, true),
			action,
			model_version: model.model_version.clone(),
			scored_at: completed_at.to_rfc3339(),
			risk_factors: vec![factor],
		};
	}

	let mut score = model.weights.base_risk;
	let mut factors = Vec::new();

	if email_score.signals.smtp_is_catch_all {
		let is_free_provider = signals.free_provider.unwrap_or(false);
		let contribution = if is_free_provider {
			model.weights.free_provider_catch_all
		} else {
			model.weights.corporate_catch_all
		};
		push_factor(
			&mut factors,
			"smtp_is_catch_all",
			contribution,
			if is_free_provider {
				"Free-provider catch-all mailbox"
			} else {
				"Corporate catch-all mailbox"
			},
		);
		score += contribution;
	}

	if email_score.signals.is_role_account {
		push_factor(
			&mut factors,
			"is_role_account",
			model.weights.role_account,
			"Role-based mailbox",
		);
		score += model.weights.role_account;
	}

	if email_score.signals.smtp_error
		|| matches!(email_score.category, crate::scoring::EmailCategory::Unknown)
	{
		push_factor(
			&mut factors,
			"smtp_unknown_error",
			model.weights.smtp_unknown_error,
			"SMTP deliverability could not be confirmed",
		);
		score += model.weights.smtp_unknown_error;
	}

	if let Some(domain_age_days) = signals.domain_age_days {
		let contribution = if domain_age_days < 30 {
			model.weights.domain_age.under_30_days
		} else if domain_age_days < 90 {
			model.weights.domain_age.under_90_days
		} else if domain_age_days < 365 {
			model.weights.domain_age.under_365_days
		} else {
			0
		};
		push_factor(
			&mut factors,
			"domain_age_days",
			contribution,
			"Young domains bounce more often",
		);
		score += contribution;
	}

	if signals.has_spf == Some(false) {
		push_factor(
			&mut factors,
			"has_spf",
			model.weights.missing_spf,
			"Domain is missing SPF",
		);
		score += model.weights.missing_spf;
	}

	if signals.has_dkim == Some(false) {
		push_factor(
			&mut factors,
			"has_dkim",
			model.weights.missing_dkim,
			"Domain is missing DKIM",
		);
		score += model.weights.missing_dkim;
	}

	if signals.has_dmarc == Some(false) {
		push_factor(
			&mut factors,
			"has_dmarc",
			model.weights.missing_dmarc,
			"Domain is missing DMARC",
		);
		score += model.weights.missing_dmarc;
	}

	if signals.website_present == Some(false) {
		push_factor(
			&mut factors,
			"website_present",
			model.weights.missing_website,
			"Domain website could not be reached",
		);
		score += model.weights.missing_website;
	}

	match signals.result_consistency {
		Some(HistoryConsistency::Consistent) => {
			push_factor(
				&mut factors,
				"result_consistency",
				model.weights.consistent_history,
				"Historical results agree with the current outcome",
			);
			score += model.weights.consistent_history;
		}
		Some(HistoryConsistency::Inconsistent) => {
			push_factor(
				&mut factors,
				"result_consistency",
				model.weights.inconsistent_history,
				"Historical results conflict with the current outcome",
			);
			score += model.weights.inconsistent_history;
		}
		None => {}
	}

	if let Some(days) = signals.days_since_last_verification {
		let contribution = if days <= 7 {
			model.weights.last_verification.within_7_days
		} else if days <= 30 {
			model.weights.last_verification.within_30_days
		} else if days > 90 {
			model.weights.last_verification.over_90_days
		} else {
			0
		};
		push_factor(
			&mut factors,
			"days_since_last_verification",
			contribution,
			"Recent verification history changes bounce risk",
		);
		score += contribution;
	}

	score = score.clamp(0, 100);
	factors.retain(|factor| factor.contribution != 0);
	factors.sort_by_key(|factor| Reverse(i32::from(factor.contribution.abs())));

	let confidence = compute_confidence(signals, &model.confidence_weights, false);
	let (category, action) = category_and_action(score, &model.thresholds);

	BounceRiskAssessment {
		score,
		category,
		confidence,
		action,
		model_version: model.model_version.clone(),
		scored_at: completed_at.to_rfc3339(),
		risk_factors: factors,
	}
}

fn push_factor(factors: &mut Vec<RiskFactor>, signal: &str, contribution: i16, description: &str) {
	if contribution == 0 {
		return;
	}
	factors.push(RiskFactor {
		signal: signal.to_string(),
		contribution,
		description: description.to_string(),
		direction: if contribution >= 0 {
			RiskDirection::IncreasesRisk
		} else {
			RiskDirection::DecreasesRisk
		},
	});
}

fn compute_confidence(
	signals: &SignalBundle,
	weights: &ConfidenceWeightConfig,
	hard_override: bool,
) -> f64 {
	let mut total = 0.0;
	let mut available = 0.0;
	add_confidence_weight(&mut total, &mut available, weights.verification_core, true);
	add_confidence_weight(
		&mut total,
		&mut available,
		weights.provider_from_mx,
		signals.provider_from_mx.is_some(),
	);
	add_confidence_weight(
		&mut total,
		&mut available,
		weights.free_provider_flag,
		signals.free_provider.is_some(),
	);
	add_confidence_weight(
		&mut total,
		&mut available,
		weights.mx_count,
		signals.mx_count.is_some(),
	);
	add_confidence_weight(
		&mut total,
		&mut available,
		weights.mx_priority_spread,
		signals.mx_priority_spread.is_some(),
	);
	add_confidence_weight(
		&mut total,
		&mut available,
		weights.domain_age,
		signals.domain_age_days.is_some(),
	);
	add_confidence_weight(
		&mut total,
		&mut available,
		weights.website_presence,
		signals.website_present.is_some(),
	);
	add_confidence_weight(
		&mut total,
		&mut available,
		weights.spf,
		signals.has_spf.is_some(),
	);
	add_confidence_weight(
		&mut total,
		&mut available,
		weights.dkim,
		signals.has_dkim.is_some(),
	);
	add_confidence_weight(
		&mut total,
		&mut available,
		weights.dmarc,
		signals.has_dmarc.is_some(),
	);
	add_confidence_weight(
		&mut total,
		&mut available,
		weights.history_count,
		signals.historical_verification_count.is_some(),
	);
	add_confidence_weight(
		&mut total,
		&mut available,
		weights.history_consistency,
		signals.result_consistency.is_some(),
	);
	add_confidence_weight(
		&mut total,
		&mut available,
		weights.last_verification,
		signals.days_since_last_verification.is_some(),
	);

	let mut confidence = if total <= f64::EPSILON {
		1.0
	} else {
		available / total
	};

	if hard_override {
		confidence = confidence.max(0.9);
	}

	(confidence.clamp(0.35, 1.0) * 100.0).round() / 100.0
}

fn add_confidence_weight(total: &mut f64, available: &mut f64, weight: f64, present: bool) {
	*total += weight;
	if present {
		*available += weight;
	}
}

fn category_and_action(
	score: i16,
	thresholds: &ThresholdConfig,
) -> (BounceRiskCategory, RecommendedAction) {
	if score <= thresholds.safe_max {
		(BounceRiskCategory::Safe, RecommendedAction::Send)
	} else if score <= thresholds.low_max {
		(BounceRiskCategory::Low, RecommendedAction::SendWithCaution)
	} else if score <= thresholds.medium_max {
		(
			BounceRiskCategory::Medium,
			RecommendedAction::VerifyManually,
		)
	} else if score <= thresholds.high_max {
		(BounceRiskCategory::High, RecommendedAction::DoNotSend)
	} else {
		(BounceRiskCategory::Dangerous, RecommendedAction::DoNotSend)
	}
}

fn detect_hard_override_reason(email_score: &EmailScore) -> Option<String> {
	if !email_score.signals.valid_syntax {
		return Some("invalid_syntax".to_string());
	}
	if matches!(
		email_score.sub_reason,
		crate::scoring::SubReason::InvalidRecipient
	) {
		return Some("invalid_recipient".to_string());
	}
	if email_score.signals.smtp_is_disabled {
		return Some("disabled_mailbox".to_string());
	}
	if email_score.signals.smtp_has_full_inbox {
		return Some("mailbox_full".to_string());
	}
	if !email_score.signals.smtp_is_deliverable {
		return Some("smtp_undeliverable".to_string());
	}
	if email_score.signals.is_disposable {
		return Some("disposable".to_string());
	}
	None
}

pub fn derive_provider_from_mx_host(mx_host: &str) -> String {
	if is_gmail(mx_host) {
		"google_workspace".to_string()
	} else if is_hotmail(mx_host) {
		"microsoft_365".to_string()
	} else if is_mimecast(mx_host) {
		"mimecast".to_string()
	} else if is_proofpoint(mx_host) {
		"proofpoint".to_string()
	} else if is_yahoo(mx_host) {
		"yahoo".to_string()
	} else {
		"other".to_string()
	}
}

pub async fn probe_website_presence(client: &reqwest::Client, domain: &str) -> Option<bool> {
	let mut saw_response = false;
	for url in [format!("https://{domain}"), format!("http://{domain}")] {
		match client.get(&url).send().await {
			Ok(response) => {
				saw_response = true;
				if response.status().is_success() || response.status().is_redirection() {
					return Some(true);
				}
			}
			Err(error) if error.is_timeout() => continue,
			Err(_) => continue,
		}
	}
	saw_response.then_some(false)
}

pub fn parse_domain_info_from_rdap_value(value: &serde_json::Value) -> DomainInfo {
	let events = value.get("events").and_then(|value| value.as_array());
	let created_at = events.and_then(|events| {
		events.iter().find_map(|event| {
			let action = event.get("eventAction").and_then(|value| value.as_str())?;
			if matches!(action, "registration" | "creation") {
				event
					.get("eventDate")
					.and_then(|value| value.as_str())
					.map(ToOwned::to_owned)
			} else {
				None
			}
		})
	});
	let registrar = value
		.get("entities")
		.and_then(|value| value.as_array())
		.and_then(|entities| {
			entities.iter().find_map(|entity| {
				entity
					.get("vcardArray")
					.and_then(|value| value.as_array())
					.and_then(|items| items.get(1))
					.and_then(|value| value.as_array())
					.and_then(|items| {
						items.iter().find_map(|item| {
							let item = item.as_array()?;
							if item.first()?.as_str()? == "fn" {
								item.get(3)?.as_str().map(ToOwned::to_owned)
							} else {
								None
							}
						})
					})
			})
		});
	let domain_age_days = created_at.as_deref().and_then(|created_at| {
		chrono::DateTime::parse_from_rfc3339(created_at)
			.ok()
			.map(|created_at| (Utc::now() - created_at.with_timezone(&Utc)).num_days())
	});

	DomainInfo {
		domain_age_days,
		registrar,
		created_at,
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn hard_override_invalid_syntax_scores_high_risk() {
		let mut output = CheckEmailOutput::default();
		output.syntax.is_valid_syntax = false;
		let score = crate::scoring::compute_score(&output);
		let signals = SignalBundle {
			hard_override_reason: Some("invalid_syntax".to_string()),
			..Default::default()
		};
		let assessment = compute_assessment(
			&score,
			&signals,
			&BounceRiskModelConfig::default(),
			Utc::now(),
		);
		assert_eq!(assessment.score, 98);
		assert_eq!(assessment.category, BounceRiskCategory::Dangerous);
		assert_eq!(assessment.action, RecommendedAction::DoNotSend);
	}

	#[test]
	fn category_mapping_matches_thresholds() {
		let thresholds = BounceRiskModelConfig::default().thresholds;
		assert_eq!(
			category_and_action(20, &thresholds),
			(BounceRiskCategory::Safe, RecommendedAction::Send)
		);
		assert_eq!(
			category_and_action(40, &thresholds),
			(BounceRiskCategory::Low, RecommendedAction::SendWithCaution)
		);
		assert_eq!(
			category_and_action(60, &thresholds),
			(
				BounceRiskCategory::Medium,
				RecommendedAction::VerifyManually
			)
		);
		assert_eq!(
			category_and_action(75, &thresholds),
			(BounceRiskCategory::High, RecommendedAction::DoNotSend)
		);
		assert_eq!(
			category_and_action(90, &thresholds),
			(BounceRiskCategory::Dangerous, RecommendedAction::DoNotSend)
		);
	}

	#[test]
	fn confidence_is_clamped() {
		let confidence = compute_confidence(
			&SignalBundle::default(),
			&BounceRiskModelConfig::default().confidence_weights,
			false,
		);
		assert_eq!(confidence, 0.35);
	}

	#[test]
	fn risk_factors_are_sorted_by_absolute_contribution() {
		let score = crate::scoring::compute_score(&CheckEmailOutput::default());
		let signals = SignalBundle {
			free_provider: Some(false),
			website_present: Some(false),
			has_dmarc: Some(false),
			result_consistency: Some(HistoryConsistency::Consistent),
			..Default::default()
		};
		let assessment = compute_assessment(
			&score,
			&signals,
			&BounceRiskModelConfig::default(),
			Utc::now(),
		);
		let contributions: Vec<i16> = assessment
			.risk_factors
			.iter()
			.map(|factor| factor.contribution.abs())
			.collect();
		assert!(contributions
			.windows(2)
			.all(|window| window[0] >= window[1]));
	}

	#[test]
	fn provider_is_derived_from_mx_host() {
		assert_eq!(
			derive_provider_from_mx_host("aspmx.l.google.com."),
			"google_workspace".to_string()
		);
		assert_eq!(
			derive_provider_from_mx_host("company-com.mail.protection.outlook.com."),
			"microsoft_365".to_string()
		);
	}

	#[test]
	fn full_inbox_override_beats_generic_undeliverable() {
		let mut output = CheckEmailOutput::default();
		output.is_reachable = check_if_email_exists::Reachable::Safe;
		output.syntax.is_valid_syntax = true;
		output.smtp = Ok(check_if_email_exists::smtp::SmtpDetails {
			can_connect_smtp: true,
			has_full_inbox: true,
			is_catch_all: false,
			is_deliverable: false,
			is_disabled: false,
		});

		let score = crate::scoring::compute_score(&output);
		assert_eq!(
			detect_hard_override_reason(&score).as_deref(),
			Some("mailbox_full")
		);
	}

	#[test]
	fn mx_lookup_errors_do_not_claim_signal_availability() {
		let mut output = CheckEmailOutput::default();
		output.mx = Ok(check_if_email_exists::mx::MxDetails::default());
		assert_eq!(collect_mx_signals(&output), (None, None, None));
	}

	#[test]
	fn confidence_only_counts_known_free_provider_state() {
		let weights = BounceRiskModelConfig::default().confidence_weights;
		let known_signals = SignalBundle {
			provider_from_mx: Some("other".to_string()),
			free_provider: Some(false),
			mx_count: Some(1),
			mx_priority_spread: Some(0),
			domain_age_days: Some(400),
			website_present: Some(true),
			has_spf: Some(true),
			has_dkim: Some(true),
			has_dmarc: Some(true),
			historical_verification_count: Some(1),
			result_consistency: Some(HistoryConsistency::Consistent),
			days_since_last_verification: Some(1),
			..Default::default()
		};
		let unknown_free_provider = SignalBundle {
			free_provider: None,
			..known_signals.clone()
		};

		assert!(
			compute_confidence(&known_signals, &weights, false)
				> compute_confidence(&unknown_free_provider, &weights, false)
		);
	}

	#[test]
	fn history_summary_detects_flip_flops() {
		let now = Utc::now();
		let consistent = summarize_history(
			&[
				HistoryRow {
					category: Some("valid".to_string()),
					completed_at: now,
				},
				HistoryRow {
					category: Some("valid".to_string()),
					completed_at: now - ChronoDuration::days(3),
				},
			],
			"valid",
		);
		assert_eq!(consistent.consistency, Some(HistoryConsistency::Consistent));

		let inconsistent = summarize_history(
			&[
				HistoryRow {
					category: Some("valid".to_string()),
					completed_at: now,
				},
				HistoryRow {
					category: Some("invalid".to_string()),
					completed_at: now - ChronoDuration::days(3),
				},
			],
			"valid",
		);
		assert_eq!(
			inconsistent.consistency,
			Some(HistoryConsistency::Inconsistent)
		);
	}

	#[test]
	fn parse_domain_info_tolerates_missing_fields() {
		let value = serde_json::json!({});
		let parsed = parse_domain_info_from_rdap_value(&value);
		assert!(parsed.domain_age_days.is_none());
		assert!(parsed.registrar.is_none());
		assert!(parsed.created_at.is_none());
	}

	#[test]
	fn website_probe_fallback_prefers_any_success() {
		let https_success = Some(true);
		let http_success = Some(false);
		assert_eq!(https_success.or(http_success), Some(true));
	}

	#[test]
	fn model_reload_keeps_last_good_config() {
		let path =
			std::env::temp_dir().join(format!("bounce-risk-model-{}.toml", uuid::Uuid::new_v4()));
		fs::write(&path, "model_version = \"first\"\n[hard_overrides]\ninvalid_syntax = 98\ninvalid_recipient = 98\nsmtp_undeliverable = 98\ndisabled_mailbox = 98\ndisposable = 90\nmailbox_full = 75\n[thresholds]\nsafe_max = 20\nlow_max = 40\nmedium_max = 60\nhigh_max = 80\n[weights]\nbase_risk = 20\ncorporate_catch_all = 20\nfree_provider_catch_all = 12\nrole_account = 10\nsmtp_unknown_error = 20\nmissing_spf = 6\nmissing_dkim = 2\nmissing_dmarc = 8\nmissing_website = 6\nconsistent_history = -10\ninconsistent_history = 15\n[weights.domain_age]\nunder_30_days = 25\nunder_90_days = 15\nunder_365_days = 5\n[weights.last_verification]\nwithin_7_days = -10\nwithin_30_days = -5\nover_90_days = 8\n[confidence_weights]\nverification_core = 2.0\nprovider_from_mx = 0.5\nfree_provider_flag = 0.5\nmx_count = 0.5\nmx_priority_spread = 0.5\ndomain_age = 1.0\nwebsite_presence = 0.75\nspf = 0.75\ndkim = 0.5\ndmarc = 0.75\nhistory_count = 0.5\nhistory_consistency = 1.0\nlast_verification = 1.0\n").unwrap();
		let service = BounceRiskService::new(BounceRiskConfig {
			enabled: true,
			config_path: path.to_string_lossy().to_string(),
			reload_interval_seconds: 0,
			history_limit: 10,
			network_timeout_ms: 1000,
			website_probe_timeout_ms: 1000,
		});
		let first = service.current_model().unwrap();
		assert_eq!(first.model_version, "first");

		fs::write(&path, "not valid toml").unwrap();
		let second = service.current_model().unwrap();
		assert_eq!(second.model_version, "first");

		let _ = fs::remove_file(path);
	}
}
