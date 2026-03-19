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
	pub listed: bool,
	pub lookup_time_ms: u64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DnsRecordResults {
	pub has_spf: bool,
	pub spf_valid: bool,
	pub has_dkim: bool,
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
