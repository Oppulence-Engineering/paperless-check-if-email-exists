use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReputationCheckRequest {
	pub domain: String,
	#[serde(default)]
	pub force_refresh: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BlacklistResult {
	pub provider: String,
	pub listed: Option<bool>,
	pub status: BlacklistStatus,
	pub answers: Vec<String>,
	pub error: Option<String>,
	pub lookup_time_ms: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BlacklistStatus {
	Listed,
	NotListed,
	Error,
	Skipped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DkimKeyStatus {
	UsableKey,
	Revoked,
	Invalid,
	NotFound,
	Error,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DkimSelectorResult {
	pub selector: String,
	pub status: DkimKeyStatus,
	pub error: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DnsRecordResults {
	pub has_spf: bool,
	/// Heuristic only; does not validate syntax or evaluate sender authorization.
	pub spf_policy_explicit: bool,
	/// None means unknown: a selector search cannot establish domain-wide absence.
	pub has_dkim: Option<bool>,
	pub dkim_selectors: Vec<DkimSelectorResult>,
	pub has_dmarc: bool,
	pub dmarc_policy: Option<String>,
	pub has_mx: bool,
	pub mx_records: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainInfo {
	pub domain_age_days: Option<i64>,
	pub registrar: Option<String>,
	pub created_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReputationCheckResponse {
	pub domain: String,
	pub score: i16,
	pub risk_level: String,
	pub blacklist_results: Vec<BlacklistResult>,
	pub dns_records: DnsRecordResults,
	pub domain_info: DomainInfo,
	pub cached: bool,
}
