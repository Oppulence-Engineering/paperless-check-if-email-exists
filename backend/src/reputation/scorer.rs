use super::models::{BlacklistResult, DnsRecordResults, DomainInfo};

pub fn compute_score(
	blacklist_results: &[BlacklistResult],
	dns_records: &DnsRecordResults,
	domain_info: &DomainInfo,
) -> (i16, String) {
	let mut score = 100i16;

	for result in blacklist_results {
		if result.listed == Some(true) {
			score -= if is_major_provider(&result.provider) {
				15
			} else {
				8
			};
		}
	}
	if !dns_records.has_spf {
		score -= 10;
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

	score = score.clamp(0, if dns_records.has_mx { 100 } else { 49 });
	let risk_level = match score {
		80..=100
			if blacklist_results
				.iter()
				.any(|result| result.listed.is_none()) =>
		{
			"unknown"
		}
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
	fn unknown_coverage_cannot_be_presented_as_a_clean_low_risk_result() {
		let dns = DnsRecordResults {
			has_spf: true,
			has_dmarc: true,
			has_mx: true,
			..Default::default()
		};
		let (score, risk) = compute_score(
			&[BlacklistResult {
				provider: "test".into(),
				listed: None,
				status: super::super::models::BlacklistStatus::Error,
				answers: vec![],
				error: Some("timeout".into()),
				lookup_time_ms: 1,
			}],
			&dns,
			&DomainInfo::default(),
		);
		assert_eq!(score, 100);
		assert_eq!(risk, "unknown");
	}

	#[test]
	fn compute_score_penalizes_missing_dns_and_blacklists() {
		assert_eq!(
			compute_score(&[], &DnsRecordResults::default(), &DomainInfo::default()),
			(49, "high".into())
		);
		let (score, risk) = compute_score(
			&[
				BlacklistResult {
					provider: "Spamhaus ZEN".into(),
					listed: Some(true),
					status: super::super::models::BlacklistStatus::Listed,
					answers: vec!["127.0.0.2".into()],
					error: None,
					lookup_time_ms: 10,
				},
				BlacklistResult {
					provider: "PSBL".into(),
					listed: Some(true),
					status: super::super::models::BlacklistStatus::Listed,
					answers: vec!["127.0.0.2".into()],
					error: None,
					lookup_time_ms: 10,
				},
			],
			&DnsRecordResults {
				has_spf: false,
				spf_policy_explicit: false,
				has_dkim: None,
				dkim_selectors: Vec::new(),
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

		assert_eq!(score, 22);
		assert_eq!(risk, "high");
	}
}
