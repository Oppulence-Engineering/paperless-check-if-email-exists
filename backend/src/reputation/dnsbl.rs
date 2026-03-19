use super::models::BlacklistResult;
use hickory_resolver::system_conf::read_system_conf;
use hickory_resolver::TokioAsyncResolver;
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

pub async fn lookup_dnsbl(domain: &str) -> Result<Vec<BlacklistResult>, anyhow::Error> {
	let (config, opts) = read_system_conf()?;
	let resolver = TokioAsyncResolver::tokio(config, opts);
	let mx_lookup = resolver.mx_lookup(domain).await.ok();
	let mx_hosts: Vec<String> = mx_lookup
		.as_ref()
		.map(|lookup| {
			lookup
				.iter()
				.map(|record| record.exchange().to_string())
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

	let ip_lookup = resolver.ipv4_lookup(first_host).await.ok();
	let ip = ip_lookup.and_then(|lookup| lookup.iter().next().copied());

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
				.lookup_ip(query)
				.await
				.ok()
				.map(|lookup| lookup.iter().next().is_some())
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
