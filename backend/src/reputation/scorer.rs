use super::models::{BlacklistResult, DnsRecordResults, DomainInfo};

pub fn compute_score(
	blacklist_results: &[BlacklistResult],
	dns_records: &DnsRecordResults,
	domain_info: &DomainInfo,
) -> (i16, String) {
	let mut score = 100i16;

	for result in blacklist_results {
		if result.listed {
			score -= if is_major_provider(&result.provider) { 15 } else { 8 };
		}
	}
	if !dns_records.has_spf {
		score -= 10;
	}
	if !dns_records.has_dkim {
		score -= 8;
	}
	if !dns_records.has_dmarc {
		score -= 10;
	}
	if !dns_records.has_mx {
		score -= 20;
	}
	if let Some(age_days) = domain_info.domain_age_days {
		if age_days < 30 {
			score -= 15;
		}
	}

	score = score.clamp(0, 100);
	let risk_level = match score {
		80..=100 => "low",
		50..=79 => "medium",
		1..=49 => "high",
		_ => "critical",
	}
	.to_string();

	(score, risk_level)
}

fn is_major_provider(provider: &str) -> bool {
	matches!(provider, "Spamhaus ZEN" | "SpamCop" | "Barracuda")
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn compute_score_penalizes_missing_dns_and_blacklists() {
		let (score, risk) = compute_score(
			&[
				BlacklistResult {
					provider: "Spamhaus ZEN".into(),
					listed: true,
					lookup_time_ms: 10,
				},
				BlacklistResult {
					provider: "PSBL".into(),
					listed: true,
					lookup_time_ms: 10,
				},
			],
			&DnsRecordResults {
				has_spf: false,
				spf_valid: false,
				has_dkim: false,
				has_dmarc: false,
				dmarc_policy: None,
				has_mx: false,
				mx_records: vec![],
			},
			&DomainInfo {
				domain_age_days: Some(10),
				registrar: None,
				created_at: None,
			},
		);

		assert_eq!(score, 14);
		assert_eq!(risk, "high");
	}
}
