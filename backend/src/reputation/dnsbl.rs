use super::dns_records::found_or_absent;
use super::models::{BlacklistResult, BlacklistStatus};
use anyhow::Context;
use check_if_email_exists::mx::MxLookup;
use hickory_resolver::lookup::Lookup;
use hickory_resolver::proto::rr::RData;
use hickory_resolver::TokioResolver;
use std::net::Ipv4Addr;
use std::time::Instant;

const PROVIDERS: &[(&str, &str)] = &[
	("Spamhaus ZEN", "zen.spamhaus.org"),
	("SpamCop", "bl.spamcop.net"),
	("Barracuda", "b.barracudacentral.org"),
	("SORBS", "dnsbl.sorbs.net"),
	("UCEPROTECT", "dnsbl-1.uceprotect.net"),
	("PSBL", "psbl.surriel.com"),
	("RATS", "all.rbl.jp"),
	("Invaluement", "dnsbl.invaluement.com"),
];

/// DNSBLs answer a listed IP with an A record in 127.0.0.2-127.0.0.255. Other
/// answers are not listings: Spamhaus returns 127.255.255.252/254/255 when it
/// refuses a query (for example one sent through a public resolver), and an
/// answer outside 127.0.0.0/24 comes from a wildcarding or hijacking resolver.
fn is_listing_answer(answer: Ipv4Addr) -> bool {
	let [a, b, c, d] = answer.octets();
	(a, b, c) == (127, 0, 0) && d >= 2
}

fn ipv4_answers(lookup: &Lookup) -> impl Iterator<Item = Ipv4Addr> + '_ {
	lookup
		.answers()
		.iter()
		.filter_map(|record| match &record.data {
			RData::A(a) => Some(a.0),
			_ => None,
		})
}

/// Checks the preferred MX host of `domain` against the DNSBL zones.
///
/// MX/address failures fail the check. Per-provider failures remain explicit
/// unknown results; only a completed negative DNS answer is "not listed".
pub async fn lookup_dnsbl(domain: &str) -> Result<Vec<BlacklistResult>, anyhow::Error> {
	let resolver = TokioResolver::builder_tokio()?.build()?;
	let mx_lookup = found_or_absent(resolver.mx_lookup(domain).await)
		.with_context(|| format!("MX lookup for {} failed", domain))?;
	let preferred_host = mx_lookup.and_then(|lookup| {
		MxLookup::from(lookup)
			.iter()
			.filter(|record| record.exchange.to_string() != ".")
			.min_by_key(|record| record.preference)
			.map(|record| record.exchange.to_string())
	});
	let first_host = preferred_host.as_ref();
	let ip = if let Some(host) = first_host {
		let lookup = found_or_absent(resolver.ipv4_lookup(host.as_str()).await)
			.with_context(|| format!("A lookup for MX host {} failed", host))?;
		lookup.and_then(|lookup| ipv4_answers(&lookup).next())
	} else {
		None
	};

	let mut results = Vec::new();
	for (provider, zone) in PROVIDERS {
		let started = Instant::now();
		let result = if let Some(ip) = ip {
			let octets = ip.octets();
			let query = format!(
				"{}.{}.{}.{}.{}",
				octets[3], octets[2], octets[1], octets[0], zone
			);
			Some(
				found_or_absent(resolver.ipv4_lookup(query).await)
					.map(|lookup| lookup.map(|lookup| ipv4_answers(&lookup).collect::<Vec<_>>()))
					.map_err(|error| error.to_string()),
			)
		} else {
			None
		};
		results.push(classify_query(
			provider,
			result,
			started.elapsed().as_millis() as u64,
		));
	}

	Ok(results)
}

fn classify_query(
	provider: &str,
	result: Option<Result<Option<Vec<Ipv4Addr>>, String>>,
	lookup_time_ms: u64,
) -> BlacklistResult {
	let (listed, status, answers, error) = match result {
		None => (
			None,
			BlacklistStatus::Skipped,
			Vec::new(),
			Some("No applicable MX IPv4 address".into()),
		),
		Some(Err(error)) => (None, BlacklistStatus::Error, Vec::new(), Some(error)),
		Some(Ok(None)) => (Some(false), BlacklistStatus::NotListed, Vec::new(), None),
		Some(Ok(Some(answers))) => {
			let listed = !answers.is_empty() && answers.iter().copied().all(is_listing_answer);
			let answers = answers.iter().map(ToString::to_string).collect();
			if listed {
				(Some(true), BlacklistStatus::Listed, answers, None)
			} else {
				(
					None,
					BlacklistStatus::Error,
					answers,
					Some("Provider returned an error or unrecognized answer".into()),
				)
			}
		}
	};
	BlacklistResult {
		provider: provider.into(),
		listed,
		status,
		answers,
		error,
		lookup_time_ms,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::net::Ipv4Addr;

	#[test]
	fn failures_and_skips_are_not_negative_evidence() {
		for error in ["timeout", "refused", "servfail"] {
			let result = classify_query("Spamhaus ZEN", Some(Err(error.into())), 1);
			assert_eq!(result.listed, None);
			assert_eq!(result.status, BlacklistStatus::Error);
		}
		for answer in [
			"127.255.255.252",
			"127.255.255.254",
			"127.255.255.255",
			"93.184.216.34",
		] {
			let result = classify_query(
				"Spamhaus ZEN",
				Some(Ok(Some(vec![answer.parse().unwrap()]))),
				1,
			);
			assert_eq!(result.listed, None);
			assert_eq!(result.status, BlacklistStatus::Error);
		}
		assert_eq!(
			classify_query("test", None, 0).status,
			BlacklistStatus::Skipped
		);
		let negative = classify_query("test", Some(Ok(None)), 1);
		assert_eq!(negative.listed, Some(false));
		assert_eq!(negative.status, BlacklistStatus::NotListed);
		assert_eq!(
			classify_query(
				"test",
				Some(Ok(Some(vec!["127.0.0.2".parse().unwrap()]))),
				1
			)
			.listed,
			Some(true)
		);
	}

	#[test]
	fn only_127_0_0_x_return_codes_count_as_listings() {
		let cases = [
			// Spamhaus ZEN listing codes (SBL, XBL, PBL).
			(Ipv4Addr::new(127, 0, 0, 2), true),
			(Ipv4Addr::new(127, 0, 0, 4), true),
			(Ipv4Addr::new(127, 0, 0, 11), true),
			(Ipv4Addr::new(127, 0, 0, 255), true),
			// Spamhaus error codes: typing error, query via public resolver,
			// excessive queries. None of these say anything about the IP.
			(Ipv4Addr::new(127, 255, 255, 252), false),
			(Ipv4Addr::new(127, 255, 255, 254), false),
			(Ipv4Addr::new(127, 255, 255, 255), false),
			// 127.0.0.1 is not a listing code, and a wildcard or hijacked answer
			// outside 127.0.0.0/24 is not one either.
			(Ipv4Addr::new(127, 0, 0, 1), false),
			(Ipv4Addr::new(127, 0, 1, 2), false),
			(Ipv4Addr::new(93, 184, 216, 34), false),
		];
		for (answer, listed) in cases {
			assert_eq!(is_listing_answer(answer), listed, "answer {}", answer);
		}
	}
}
