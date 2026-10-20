use super::models::DnsRecordResults;
use anyhow::Context;
use check_if_email_exists::mx::MxLookup;
use hickory_resolver::lookup::Lookup;
use hickory_resolver::net::NetError;
use hickory_resolver::proto::rr::rdata::TXT;
use hickory_resolver::proto::rr::RData;
use hickory_resolver::TokioResolver;

/// Splits a lookup result into "found", "the name has no such records" and a
/// real failure. Only NXDOMAIN or an empty answer means a record is absent; a
/// SERVFAIL, refusal or timeout says nothing about the domain, so callers must
/// not score it as missing or cache it.
pub(super) fn found_or_absent<T>(result: Result<T, NetError>) -> Result<Option<T>, NetError> {
	match result {
		Ok(found) => Ok(Some(found)),
		Err(err) if err.is_no_records_found() => Ok(None),
		Err(err) => Err(err),
	}
}

/// The text of one TXT record. A record holds one or more character-strings of
/// at most 255 bytes each; long SPF and DMARC records are split across several,
/// and RFC 7208 3.3 / RFC 7489 6.3 say to join them without a separator.
fn record_text(txt: &TXT) -> Option<String> {
	let joined: Vec<u8> = txt
		.txt_data
		.iter()
		.flat_map(|part| part.iter().copied())
		.collect();
	String::from_utf8(joined).ok()
}

/// The text of every TXT record in a lookup, one entry per record.
fn txt_records(lookup: &Lookup) -> Vec<String> {
	lookup
		.answers()
		.iter()
		.filter_map(|record| match &record.data {
			RData::TXT(txt) => record_text(txt),
			_ => None,
		})
		.collect()
}

/// TXT records at `name`, or none when the name has no TXT records.
async fn txt_lookup_records(resolver: &TokioResolver, name: &str) -> anyhow::Result<Vec<String>> {
	let lookup = found_or_absent(resolver.txt_lookup(name).await)
		.with_context(|| format!("TXT lookup for {} failed", name))?;
	Ok(lookup.as_ref().map(txt_records).unwrap_or_default())
}

fn is_spf_record(record: &str) -> bool {
	record
		.split_whitespace()
		.next()
		.is_some_and(|version| version.eq_ignore_ascii_case("v=spf1"))
}

/// An SPF modifier is `name=value`; mechanisms use `:` or `/` for their argument.
fn is_spf_modifier(term: &str) -> bool {
	term.split_once('=').is_some_and(|(name, _)| {
		!name.is_empty()
			&& name
				.chars()
				.all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
	})
}

fn is_all_mechanism(term: &str) -> bool {
	let term = term.trim_start_matches(['+', '-', '~', '?']);
	term.eq_ignore_ascii_case("all")
}

/// Returns `(has_spf, spf_valid)` for the TXT records of a domain. A domain
/// with two SPF records gets a permerror from receivers (RFC 7208 4.5), and a
/// record must end its mechanisms with `all` or hand off with `redirect=`, or it
/// never states a policy for unlisted senders.
fn assess_spf(txt_records: &[String]) -> (bool, bool) {
	let spf_records: Vec<&String> = txt_records
		.iter()
		.filter(|record| is_spf_record(record))
		.collect();
	let [record] = spf_records.as_slice() else {
		return (!spf_records.is_empty(), false);
	};
	let (modifiers, mechanisms): (Vec<&str>, Vec<&str>) = record
		.split_whitespace()
		.skip(1)
		.partition(|term| is_spf_modifier(term));
	let ends_with_all = mechanisms.last().is_some_and(|term| is_all_mechanism(term));
	let has_all = mechanisms.iter().any(|term| is_all_mechanism(term));
	// RFC 7208 6.1: redirect is ignored when the record has an "all" mechanism.
	let redirects = modifiers.iter().any(|term| {
		term.split_once('=')
			.is_some_and(|(name, _)| name.eq_ignore_ascii_case("redirect"))
	});
	(true, ends_with_all || (!has_all && redirects))
}

/// Looks up the mail authentication records of a domain.
///
/// Fails when any lookup fails for a reason other than the record not existing,
/// so a resolver outage is not reported as a domain without SPF, DKIM, DMARC or
/// MX records.
pub async fn check_dns_records(domain: &str) -> Result<DnsRecordResults, anyhow::Error> {
	let resolver = TokioResolver::builder_tokio()?.build()?;

	let mx_lookup = found_or_absent(resolver.mx_lookup(domain).await)
		.with_context(|| format!("MX lookup for {} failed", domain))?;
	let txt_values = txt_lookup_records(&resolver, domain).await?;
	let dmarc_values = txt_lookup_records(&resolver, &format!("_dmarc.{}", domain)).await?;

	let mx_records = mx_lookup
		.map(|lookup| {
			MxLookup::from(lookup)
				.iter()
				.map(|record| record.exchange.to_string())
				.collect()
		})
		.unwrap_or_else(Vec::new);
	let has_mx = !mx_records.is_empty();

	let (has_spf, spf_valid) = assess_spf(&txt_values);
	let dmarc_value = dmarc_values
		.iter()
		.find(|value| value.to_lowercase().starts_with("v=dmarc1"))
		.cloned();

	let mut has_dkim = false;
	let mut dkim_error = None;
	for selector in ["default", "google", "selector1", "selector2", "k1"] {
		let name = format!("{}._domainkey.{}", selector, domain);
		match txt_lookup_records(&resolver, &name).await {
			Ok(records) if !records.is_empty() => {
				has_dkim = true;
				break;
			}
			Ok(_) => {}
			// Keep probing: another selector may still answer and settle it.
			Err(err) => dkim_error = Some(err),
		}
	}
	if let (false, Some(err)) = (has_dkim, dkim_error) {
		return Err(err);
	}

	Ok(DnsRecordResults {
		has_spf,
		spf_valid,
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

#[cfg(test)]
mod tests {
	use super::{assess_spf, record_text};
	use hickory_resolver::proto::rr::rdata::TXT;

	fn txt(strings: &[&str]) -> TXT {
		TXT::new(strings.iter().map(|value| (*value).to_string()).collect())
	}

	fn records(values: &[&str]) -> Vec<String> {
		values.iter().map(|value| (*value).to_string()).collect()
	}

	#[test]
	fn record_text_joins_the_character_strings_of_one_record() {
		// RFC 7208 3.3: a record longer than 255 bytes is split into several
		// strings that are concatenated without a separator.
		let split = txt(&[
			"v=spf1 include:_spf.example.com ",
			"include:mail.example.net -all",
		]);
		assert_eq!(
			record_text(&split).as_deref(),
			Some("v=spf1 include:_spf.example.com include:mail.example.net -all")
		);
	}

	#[test]
	fn spf_is_valid_only_for_one_record_that_ends_in_all_or_redirects() {
		assert_eq!(
			assess_spf(&records(&["v=spf1 include:_spf.google.com ~all"])),
			(true, true)
		);
		assert_eq!(
			assess_spf(&records(&["v=spf1 redirect=_spf.example.com"])),
			(true, true)
		);
		assert_eq!(
			assess_spf(&records(&["V=SPF1 MX -ALL exp=explain.example.com"])),
			(true, true),
			"version and mechanisms are case-insensitive, and modifiers may follow all"
		);
		// Two SPF records is a permerror (RFC 7208 4.5).
		assert_eq!(
			assess_spf(&records(&[
				"v=spf1 include:a.example.com -all",
				"v=spf1 include:b.example.com -all",
			])),
			(true, false)
		);
		// A record that never says what to do with other senders.
		assert_eq!(
			assess_spf(&records(&["v=spf1 include:_spf.example.com"])),
			(true, false)
		);
		// Mechanisms after "all" are never evaluated, so "all" must come last.
		assert_eq!(
			assess_spf(&records(&["v=spf1 -all include:_spf.example.com"])),
			(true, false)
		);
		// Only the exact version tag counts, and unrelated TXT records are ignored.
		assert_eq!(
			assess_spf(&records(&["v=spf10 -all", "google-site-verification=abc"])),
			(false, false)
		);
	}
}
