use super::models::DnsRecordResults;
use check_if_email_exists::mx::MxLookup;
use hickory_resolver::lookup::Lookup;
use hickory_resolver::proto::rr::RData;
use hickory_resolver::TokioResolver;

/// The value of each TXT record in a lookup. A record longer than 255 bytes
/// arrives as several character-strings; SPF (RFC 7208 section 3.3) and
/// DMARC read the record as their concatenation.
fn txt_strings(lookup: &Lookup) -> Vec<String> {
	lookup
		.answers()
		.iter()
		.filter_map(|record| match &record.data {
			RData::TXT(txt) => Some(txt),
			_ => None,
		})
		.filter_map(|txt| {
			let bytes: Vec<u8> = txt
				.txt_data
				.iter()
				.flat_map(|chunk| chunk.iter().copied())
				.collect();
			String::from_utf8(bytes).ok()
		})
		.collect()
}

pub async fn check_dns_records(domain: &str) -> Result<DnsRecordResults, anyhow::Error> {
	let resolver = TokioResolver::builder_tokio()?.build()?;

	let mx_lookup = resolver.mx_lookup(domain).await.ok();
	let txt_lookup = resolver.txt_lookup(domain).await.ok();
	let dmarc_lookup = resolver.txt_lookup(format!("_dmarc.{}", domain)).await.ok();

	let mx_records = mx_lookup
		.as_ref()
		.map(|lookup| {
			MxLookup::from(lookup.clone())
				.iter()
				.map(|record| record.exchange.to_string())
				.collect()
		})
		.unwrap_or_else(Vec::new);
	let has_mx = !mx_records.is_empty();

	let txt_values: Vec<String> = txt_lookup.as_ref().map(txt_strings).unwrap_or_default();
	let dmarc_values: Vec<String> = dmarc_lookup.as_ref().map(txt_strings).unwrap_or_default();

	let spf_value = txt_values
		.iter()
		.find(|value| value.to_lowercase().starts_with("v=spf1"))
		.cloned();
	let dmarc_value = dmarc_values
		.iter()
		.find(|value| value.to_lowercase().starts_with("v=dmarc1"))
		.cloned();

	let mut has_dkim = false;
	for selector in ["default", "google", "selector1", "selector2", "k1"] {
		let name = format!("{}._domainkey.{}", selector, domain);
		if resolver
			.txt_lookup(name)
			.await
			.ok()
			.as_ref()
			.map(|lookup| !txt_strings(lookup).is_empty())
			.unwrap_or(false)
		{
			has_dkim = true;
			break;
		}
	}

	Ok(DnsRecordResults {
		has_spf: spf_value.is_some(),
		spf_valid: spf_value
			.as_deref()
			.map(|value| value.to_lowercase().starts_with("v=spf1"))
			.unwrap_or(false),
		has_dkim,
		has_dmarc: dmarc_value.is_some(),
		dmarc_policy: dmarc_value.as_deref().and_then(parse_dmarc_policy),
		has_mx,
		mx_records,
	})
}

fn parse_dmarc_policy(value: &str) -> Option<String> {
	value
		.split(';')
		.map(str::trim)
		.find_map(|segment| segment.strip_prefix("p=").map(ToOwned::to_owned))
}
