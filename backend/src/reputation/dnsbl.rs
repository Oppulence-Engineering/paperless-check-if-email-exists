use super::dns_records::found_or_absent;
use super::models::BlacklistResult;
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

/// Checks the first MX host of `domain` against the DNSBL zones.
///
/// Fails when the MX or address lookup fails for a reason other than the record
/// not existing, since then no list was actually consulted. A failed query to a
/// single list counts as "not listed": an error must never read as a listing,
/// and some zones are permanently unreachable, which would otherwise make every
/// check fail.
pub async fn lookup_dnsbl(domain: &str) -> Result<Vec<BlacklistResult>, anyhow::Error> {
	let resolver = TokioResolver::builder_tokio()?.build()?;
	let mx_lookup = found_or_absent(resolver.mx_lookup(domain).await)
		.with_context(|| format!("MX lookup for {} failed", domain))?;
	let mx_hosts: Vec<String> = mx_lookup
		.map(|lookup| {
			MxLookup::from(lookup)
				.iter()
				.map(|record| record.exchange.to_string())
				.collect()
		})
		.unwrap_or_default();
	let first_host = match mx_hosts.first() {
		Some(host) => host.clone(),
		None => {
			return Ok(PROVIDERS
				.iter()
				.map(|(provider, _)| BlacklistResult {
					provider: (*provider).to_string(),
					listed: false,
					lookup_time_ms: 0,
				})
				.collect())
		}
	};

	let ip_lookup = found_or_absent(resolver.ipv4_lookup(first_host.as_str()).await)
		.with_context(|| format!("A lookup for MX host {} failed", first_host))?;
	let ip = ip_lookup.and_then(|lookup| ipv4_answers(&lookup).next());

	let mut results = Vec::new();
	for (provider, zone) in PROVIDERS {
		let started = Instant::now();
		let listed = if let Some(ip) = ip {
			let octets = ip.octets();
			let query = format!(
				"{}.{}.{}.{}.{}",
				octets[3], octets[2], octets[1], octets[0], zone
			);
			resolver
				.ipv4_lookup(query)
				.await
				.ok()
				.is_some_and(|lookup| ipv4_answers(&lookup).any(is_listing_answer))
		} else {
			false
		};
		results.push(BlacklistResult {
			provider: (*provider).to_string(),
			listed,
			lookup_time_ms: started.elapsed().as_millis() as u64,
		});
	}

	Ok(results)
}

#[cfg(test)]
mod tests {
	use super::is_listing_answer;
	use std::net::Ipv4Addr;

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
