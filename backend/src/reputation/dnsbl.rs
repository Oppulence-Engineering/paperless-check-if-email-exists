use super::models::BlacklistResult;
use check_if_email_exists::mx::MxLookup;
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

/// Whether a DNSBL answer means that the address is listed. Lists answer
/// with an address in 127.0.0.0/8 for a listing. Spamhaus answers
/// 127.255.255.0/24 when it refuses the query, for example for a query through
/// a public resolver or above the rate limit, and that is not a listing. Some
/// resolvers answer a missing name with an address outside 127.0.0.0/8; that
/// is not a listing either.
fn is_listing_answer(answer: Ipv4Addr) -> bool {
	let [first, second, third, _] = answer.octets();
	first == 127 && !(second == 255 && third == 255)
}

pub async fn lookup_dnsbl(domain: &str) -> Result<Vec<BlacklistResult>, anyhow::Error> {
	let resolver = TokioResolver::builder_tokio()?.build()?;
	let mx_lookup = resolver.mx_lookup(domain).await.ok();
	// Senders deliver to the MX host with the lowest preference, so that host
	// is the one to look up. The DNS answer order is not the preference order.
	let preferred_host = mx_lookup.as_ref().and_then(|lookup| {
		MxLookup::from(lookup.clone())
			.iter()
			.min_by_key(|record| record.preference)
			.map(|record| record.exchange.to_string())
	});
	let first_host = match preferred_host {
		Some(host) => host,
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

	let ip_lookup = resolver.ipv4_lookup(first_host).await.ok();
	let ip = ip_lookup.and_then(|lookup| {
		lookup
			.answers()
			.iter()
			.find_map(|record| match &record.data {
				RData::A(a) => Some(a.0),
				_ => None,
			})
	});

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
				.map(|lookup| {
					lookup.answers().iter().any(|record| match &record.data {
						RData::A(a) => is_listing_answer(a.0),
						_ => false,
					})
				})
				.unwrap_or(false)
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
	fn only_loopback_codes_outside_the_refusal_range_are_listings() {
		assert!(is_listing_answer(Ipv4Addr::new(127, 0, 0, 2)));
		assert!(is_listing_answer(Ipv4Addr::new(127, 0, 0, 10)));
		// Spamhaus refusals: public resolver, rate limit, bad zone name.
		assert!(!is_listing_answer(Ipv4Addr::new(127, 255, 255, 254)));
		assert!(!is_listing_answer(Ipv4Addr::new(127, 255, 255, 255)));
		assert!(!is_listing_answer(Ipv4Addr::new(127, 255, 255, 252)));
		// A resolver that rewrites NXDOMAIN to its own search page.
		assert!(!is_listing_answer(Ipv4Addr::new(198, 51, 100, 7)));
	}
}
