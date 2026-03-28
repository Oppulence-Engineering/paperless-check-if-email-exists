use crate::mx::{is_gmail, is_hotmail_b2b, is_hotmail_b2c, is_yahoo};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::RwLock;
use std::time::{Duration, SystemTime};

const PROVIDER_CACHE_TTL: Duration = Duration::from_secs(60 * 60);
const RULES_JSON: &str = include_str!("rules.json");
#[cfg(test)]
const GOLDEN_FIXTURES: &str = include_str!("golden.csv");

static PROVIDER_RULES: Lazy<ProviderCatalog> =
	Lazy::new(|| ProviderCatalog::from_json(RULES_JSON).expect("provider rules should be valid"));
static PROVIDER_CACHE: Lazy<RwLock<HashMap<String, CachedProvider>>> =
	Lazy::new(|| RwLock::new(HashMap::new()));

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Provider {
	Gmail,
	GoogleWorkspace,
	OutlookConsumer,
	Microsoft365,
	Yahoo,
	AppleIcloud,
	Proton,
	Zoho,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProviderConfidence {
	High,
	Medium,
	Low,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DetectedProvider {
	pub provider: Provider,
	pub confidence: ProviderConfidence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProviderRejectionReason {
	#[serde(rename = "provider_local_part_too_short")]
	LocalPartTooShort,
	#[serde(rename = "provider_local_part_too_long")]
	LocalPartTooLong,
	#[serde(rename = "provider_invalid_character")]
	InvalidCharacter,
	#[serde(rename = "provider_consecutive_special_characters")]
	ConsecutiveSpecialCharacters,
	#[serde(rename = "provider_invalid_start_character")]
	InvalidStartCharacter,
	#[serde(rename = "provider_invalid_end_character")]
	InvalidEndCharacter,
	#[serde(rename = "provider_plus_addressing_not_supported")]
	PlusAddressingNotSupported,
	#[serde(rename = "provider_reserved_word")]
	ReservedWord,
	#[serde(rename = "provider_format_violation")]
	ProviderFormatViolation,
}

impl ProviderRejectionReason {
	pub fn code(&self) -> &'static str {
		match self {
			Self::LocalPartTooShort => "provider_local_part_too_short",
			Self::LocalPartTooLong => "provider_local_part_too_long",
			Self::InvalidCharacter => "provider_invalid_character",
			Self::ConsecutiveSpecialCharacters => "provider_consecutive_special_characters",
			Self::InvalidStartCharacter => "provider_invalid_start_character",
			Self::InvalidEndCharacter => "provider_invalid_end_character",
			Self::PlusAddressingNotSupported => "provider_plus_addressing_not_supported",
			Self::ReservedWord => "provider_reserved_word",
			Self::ProviderFormatViolation => "provider_format_violation",
		}
	}
}

#[derive(Debug, Clone)]
pub struct ProviderRule {
	pub provider: Provider,
	pub min_local_length: usize,
	pub max_local_length: usize,
	pub allowed_special_chars: Vec<char>,
	pub must_start_with_alphanumeric: bool,
	pub consecutive_special_allowed: bool,
	pub leading_trailing_special_allowed: bool,
	pub plus_addressing: bool,
	pub reserved_words: Vec<String>,
}

#[derive(Debug)]
struct CachedProvider {
	detected: DetectedProvider,
	expires_at: SystemTime,
}

#[derive(Debug, Deserialize)]
struct ProviderCatalogJson {
	version: String,
	providers: Vec<ProviderRuleJson>,
}

#[derive(Debug, Deserialize)]
struct ProviderRuleJson {
	provider: Provider,
	domains: Vec<String>,
	min_local_length: usize,
	max_local_length: usize,
	allowed_special_chars: Vec<String>,
	must_start_with_alphanumeric: bool,
	consecutive_special_allowed: bool,
	leading_trailing_special_allowed: bool,
	plus_addressing: bool,
	reserved_words: Vec<String>,
}

#[derive(Debug)]
struct ProviderCatalog {
	version: String,
	by_domain: HashMap<String, ProviderRule>,
}

impl ProviderCatalog {
	fn from_json(raw: &str) -> Result<Self, serde_json::Error> {
		let parsed: ProviderCatalogJson = serde_json::from_str(raw)?;
		let mut by_domain = HashMap::new();
		for rule in parsed.providers {
			let provider = rule.provider;
			let provider_rule = ProviderRule {
				provider: provider.clone(),
				min_local_length: rule.min_local_length,
				max_local_length: rule.max_local_length,
				allowed_special_chars: validate_allowed_special_chars(
					rule.allowed_special_chars,
					&provider,
				)?,
				must_start_with_alphanumeric: rule.must_start_with_alphanumeric,
				consecutive_special_allowed: rule.consecutive_special_allowed,
				leading_trailing_special_allowed: rule.leading_trailing_special_allowed,
				plus_addressing: rule.plus_addressing,
				reserved_words: rule.reserved_words,
			};
			for domain in rule.domains {
				by_domain.insert(domain.to_ascii_lowercase(), provider_rule.clone());
			}
		}
		Ok(Self {
			version: parsed.version,
			by_domain,
		})
	}
}

fn validate_allowed_special_chars(
	allowed_special_chars: Vec<String>,
	provider: &Provider,
) -> Result<Vec<char>, serde_json::Error> {
	allowed_special_chars
		.into_iter()
		.map(|entry| {
			let mut chars = entry.chars();
			match (chars.next(), chars.next()) {
				(Some(ch), None) => Ok(ch),
				_ => Err(serde_json::Error::io(std::io::Error::new(
					std::io::ErrorKind::InvalidData,
					format!(
						"provider {:?} has invalid rule.allowed_special_chars entry {:?}; each allowed_special_chars value must contain exactly one character instead of relying on chars().next() truncation",
						provider, entry
					),
				))),
			}
		})
		.collect()
}

pub fn canonical_domain(domain: &str) -> String {
	match domain.to_ascii_lowercase().as_str() {
		"googlemail.com" => "gmail.com".to_string(),
		other => other.to_string(),
	}
}

pub fn provider_rules_version() -> &'static str {
	&PROVIDER_RULES.version
}

pub fn detect_provider(domain: &str, mx_host: Option<&str>) -> Option<DetectedProvider> {
	let canonical_domain = canonical_domain(domain);
	if let Some(rule) = PROVIDER_RULES.by_domain.get(&canonical_domain) {
		return Some(DetectedProvider {
			provider: rule.provider.clone(),
			confidence: ProviderConfidence::High,
		});
	}

	if let Some(detected) = get_cached_provider(&canonical_domain) {
		return Some(detected);
	}

	let Some(mx_host) = mx_host else {
		return None;
	};

	let detected = detect_provider_from_mx_host(&canonical_domain, mx_host)?;
	cache_provider(&canonical_domain, detected.clone());
	Some(detected)
}

pub fn validate_provider_email(
	provider: &DetectedProvider,
	local_part: &str,
) -> Result<(), ProviderRejectionReason> {
	let Some(rule) = provider_rule(&provider.provider) else {
		return Ok(());
	};

	// Provider-specific rules in this catalog only model ASCII local parts.
	// Preserve RFC-level behavior for EAI addresses until providers are
	// documented with explicit Unicode constraints.
	if !local_part.is_ascii() {
		return Ok(());
	}

	let local_len = local_part.chars().count();
	if local_len < rule.min_local_length {
		return Err(ProviderRejectionReason::LocalPartTooShort);
	}
	if local_len > rule.max_local_length {
		return Err(ProviderRejectionReason::LocalPartTooLong);
	}

	let normalized_local = local_part.to_ascii_lowercase();
	if rule
		.reserved_words
		.iter()
		.any(|word| word == &normalized_local)
	{
		return Err(ProviderRejectionReason::ReservedWord);
	}

	let Some(first_char) = local_part.chars().next() else {
		return Err(ProviderRejectionReason::ProviderFormatViolation);
	};

	if rule.must_start_with_alphanumeric && !first_char.is_ascii_alphanumeric() {
		return Err(ProviderRejectionReason::InvalidStartCharacter);
	}

	let last_char = local_part
		.chars()
		.last()
		.expect("local part cannot be empty after earlier length check");
	if !rule.leading_trailing_special_allowed && !last_char.is_ascii_alphanumeric() {
		return Err(ProviderRejectionReason::InvalidEndCharacter);
	}

	let mut previous_special = !first_char.is_ascii_alphanumeric();
	for current_char in local_part.chars() {
		if current_char == '+' && !rule.plus_addressing {
			return Err(ProviderRejectionReason::PlusAddressingNotSupported);
		}

		let is_alphanumeric = current_char.is_ascii_alphanumeric();
		let is_allowed_special = rule.allowed_special_chars.contains(&current_char);

		if !is_alphanumeric && !is_allowed_special {
			return Err(ProviderRejectionReason::InvalidCharacter);
		}

		let is_special = !is_alphanumeric;
		if is_special && previous_special && !rule.consecutive_special_allowed {
			return Err(ProviderRejectionReason::ConsecutiveSpecialCharacters);
		}
		previous_special = is_special;
	}

	Ok(())
}

pub fn has_provider_rule(provider: &Provider) -> bool {
	provider_rule(provider).is_some()
}

pub fn should_apply_provider_rule(provider: &DetectedProvider) -> bool {
	matches!(provider.confidence, ProviderConfidence::High) && has_provider_rule(&provider.provider)
}

fn provider_rule(provider: &Provider) -> Option<&'static ProviderRule> {
	PROVIDER_RULES
		.by_domain
		.values()
		.find(|rule| &rule.provider == provider)
}

fn matches_domain_or_subdomain(domain: &str, suffix: &str) -> bool {
	domain == suffix || domain.ends_with(&format!(".{suffix}"))
}

fn detect_provider_from_mx_host(domain: &str, mx_host: &str) -> Option<DetectedProvider> {
	if is_gmail(mx_host) {
		let provider = if domain == "gmail.com" {
			Provider::Gmail
		} else {
			Provider::GoogleWorkspace
		};
		let confidence = if matches!(provider, Provider::Gmail) {
			ProviderConfidence::High
		} else {
			ProviderConfidence::Medium
		};
		return Some(DetectedProvider {
			provider,
			confidence,
		});
	}

	if is_hotmail_b2c(mx_host) {
		return Some(DetectedProvider {
			provider: Provider::OutlookConsumer,
			confidence: ProviderConfidence::High,
		});
	}

	if is_hotmail_b2b(mx_host) {
		return Some(DetectedProvider {
			provider: Provider::Microsoft365,
			confidence: ProviderConfidence::Medium,
		});
	}

	if is_yahoo(mx_host) {
		return Some(DetectedProvider {
			provider: Provider::Yahoo,
			confidence: if matches_domain_or_subdomain(domain, "yahoo.com")
				|| matches_domain_or_subdomain(domain, "rocketmail.com")
				|| matches_domain_or_subdomain(domain, "ymail.com")
			{
				ProviderConfidence::High
			} else {
				ProviderConfidence::Medium
			},
		});
	}

	let lower_host = mx_host.to_ascii_lowercase();
	if lower_host.ends_with(".mail.icloud.com.") {
		return Some(DetectedProvider {
			provider: Provider::AppleIcloud,
			confidence: ProviderConfidence::High,
		});
	}
	if lower_host.ends_with(".protonmail.ch.") || lower_host.ends_with(".protonmail.net.") {
		return Some(DetectedProvider {
			provider: Provider::Proton,
			confidence: ProviderConfidence::High,
		});
	}
	if lower_host.ends_with(".zoho.com.") || lower_host.ends_with(".zohomail.com.") {
		return Some(DetectedProvider {
			provider: Provider::Zoho,
			confidence: ProviderConfidence::High,
		});
	}

	None
}

fn get_cached_provider(domain: &str) -> Option<DetectedProvider> {
	let now = SystemTime::now();
	{
		let cache = PROVIDER_CACHE.read().ok()?;
		match cache.get(domain) {
			Some(cached) if cached.expires_at > now => return Some(cached.detected.clone()),
			Some(_) => {}
			None => return None,
		}
	}
	if let Ok(mut cache) = PROVIDER_CACHE.write() {
		if cache
			.get(domain)
			.map(|cached| cached.expires_at <= now)
			.unwrap_or(false)
		{
			cache.remove(domain);
		}
	}
	None
}

fn cache_provider(domain: &str, detected: DetectedProvider) {
	if let Ok(mut cache) = PROVIDER_CACHE.write() {
		cache.insert(
			domain.to_string(),
			CachedProvider {
				detected,
				expires_at: SystemTime::now() + PROVIDER_CACHE_TTL,
			},
		);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[derive(Debug, PartialEq, Eq)]
	struct GoldenFixture {
		email: String,
		provider: Option<Provider>,
		valid: bool,
		reason: Option<ProviderRejectionReason>,
		confidence: Option<ProviderConfidence>,
	}

	impl GoldenFixture {
		fn parse(line: &str) -> Self {
			let columns: Vec<&str> = line.split(',').collect();
			let provider = match columns[1] {
				"" => None,
				"gmail" => Some(Provider::Gmail),
				"outlook_consumer" => Some(Provider::OutlookConsumer),
				"yahoo" => Some(Provider::Yahoo),
				other => panic!("unexpected provider fixture {}", other),
			};
			let reason = match columns[3] {
				"" => None,
				"provider_local_part_too_short" => Some(ProviderRejectionReason::LocalPartTooShort),
				"provider_local_part_too_long" => Some(ProviderRejectionReason::LocalPartTooLong),
				"provider_consecutive_special_characters" => {
					Some(ProviderRejectionReason::ConsecutiveSpecialCharacters)
				}
				"provider_invalid_start_character" => {
					Some(ProviderRejectionReason::InvalidStartCharacter)
				}
				"provider_plus_addressing_not_supported" => {
					Some(ProviderRejectionReason::PlusAddressingNotSupported)
				}
				"provider_reserved_word" => Some(ProviderRejectionReason::ReservedWord),
				other => panic!("unexpected reason fixture {}", other),
			};
			let confidence = match columns[4] {
				"" => None,
				"high" => Some(ProviderConfidence::High),
				"medium" => Some(ProviderConfidence::Medium),
				other => panic!("unexpected confidence fixture {}", other),
			};

			Self {
				email: columns[0].to_string(),
				provider,
				valid: columns[2] == "true",
				reason,
				confidence,
			}
		}
	}

	fn reset_provider_cache() {
		PROVIDER_CACHE.write().unwrap().clear();
	}

	#[test]
	fn domain_aliases_detect_provider() {
		assert_eq!(
			detect_provider("googlemail.com", None),
			Some(DetectedProvider {
				provider: Provider::Gmail,
				confidence: ProviderConfidence::High,
			})
		);
		assert_eq!(
			detect_provider("live.com", None),
			Some(DetectedProvider {
				provider: Provider::OutlookConsumer,
				confidence: ProviderConfidence::High,
			})
		);
	}

	#[test]
	fn validation_rules_allow_expected_variants() {
		let gmail = detect_provider("gmail.com", None).unwrap();
		assert!(validate_provider_email(&gmail, "valid.user+tag").is_ok());

		let yahoo = detect_provider("yahoo.com", None).unwrap();
		assert!(validate_provider_email(&yahoo, "valid_user").is_ok());
	}

	#[test]
	fn validation_rules_reject_expected_variants() {
		let gmail = detect_provider("gmail.com", None).unwrap();
		assert_eq!(
			validate_provider_email(&gmail, "abc"),
			Err(ProviderRejectionReason::LocalPartTooShort)
		);
		assert_eq!(
			validate_provider_email(&gmail, "a..bcdef"),
			Err(ProviderRejectionReason::ConsecutiveSpecialCharacters)
		);

		let yahoo = detect_provider("rocketmail.com", None).unwrap();
		assert_eq!(
			validate_provider_email(&yahoo, "valid+tag"),
			Err(ProviderRejectionReason::PlusAddressingNotSupported)
		);
	}

	#[test]
	fn eai_local_parts_pass_through_provider_validation() {
		let gmail = detect_provider("gmail.com", None).unwrap();
		assert!(validate_provider_email(&gmail, "tést.user").is_ok());
	}

	#[test]
	fn mx_detection_caches_results() {
		reset_provider_cache();
		let detected = detect_provider("custom.example", Some("aspmx.l.google.com.")).unwrap();
		assert_eq!(detected.provider, Provider::GoogleWorkspace);

		let cached = get_cached_provider("custom.example").unwrap();
		assert_eq!(cached.provider, Provider::GoogleWorkspace);
		assert_eq!(cached.confidence, ProviderConfidence::Medium);
	}

	#[test]
	fn cache_expiry_is_honored() {
		reset_provider_cache();
		PROVIDER_CACHE.write().unwrap().insert(
			"expired.example".to_string(),
			CachedProvider {
				detected: DetectedProvider {
					provider: Provider::GoogleWorkspace,
					confidence: ProviderConfidence::Medium,
				},
				expires_at: SystemTime::now() - Duration::from_secs(1),
			},
		);
		assert!(get_cached_provider("expired.example").is_none());
		assert!(PROVIDER_CACHE
			.read()
			.unwrap()
			.get("expired.example")
			.is_none());
	}

	#[test]
	fn provider_rules_have_version() {
		assert!(!provider_rules_version().is_empty());
	}

	#[test]
	fn yahoo_custom_domains_stay_medium_confidence() {
		let detected = detect_provider("notyahoo.com", Some("mta7.am0.yahoodns.net.")).unwrap();
		assert_eq!(detected.provider, Provider::Yahoo);
		assert_eq!(detected.confidence, ProviderConfidence::Medium);
	}

	#[test]
	fn provider_rules_only_apply_for_high_confidence_matches() {
		let detected = DetectedProvider {
			provider: Provider::Yahoo,
			confidence: ProviderConfidence::Medium,
		};
		assert!(!should_apply_provider_rule(&detected));
	}

	#[test]
	fn golden_fixtures_cover_expected_provider_validation() {
		for line in GOLDEN_FIXTURES.lines().skip(1) {
			let fixture = GoldenFixture::parse(line);
			let (local_part, domain) = fixture
				.email
				.split_once('@')
				.expect("fixture emails should be valid addresses");
			let detected = detect_provider(domain, None);
			assert_eq!(
				detected.as_ref().map(|value| value.provider.clone()),
				fixture.provider
			);
			assert_eq!(
				detected.as_ref().map(|value| value.confidence.clone()),
				fixture.confidence
			);
			let actual_reason = detected
				.as_ref()
				.and_then(|value| validate_provider_email(value, local_part).err());
			assert_eq!(actual_reason.is_none(), fixture.valid);
			assert_eq!(actual_reason, fixture.reason);
		}
	}
}
