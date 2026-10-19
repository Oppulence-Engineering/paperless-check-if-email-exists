use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use utoipa::ToSchema;

pub const REMEDIATION_RULE_VERSION: &str = "remediation_v1";

fn default_true() -> bool {
	true
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct RemediationOptions {
	#[serde(default)]
	pub allow_partial: bool,
	#[serde(default = "default_true")]
	pub apply_domain_typos: bool,
	#[serde(default = "default_true")]
	pub drop_suppressed: bool,
	#[serde(default)]
	pub collapse_duplicates: bool,
}

impl Default for RemediationOptions {
	fn default() -> Self {
		Self {
			allow_partial: false,
			apply_domain_typos: true,
			drop_suppressed: true,
			collapse_duplicates: false,
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum RemediationClassification {
	Fixed,
	Safe,
	Review,
	Drop,
}

impl RemediationClassification {
	pub fn parse(value: &str) -> Option<Self> {
		match value {
			"fixed" => Some(Self::Fixed),
			"safe" => Some(Self::Safe),
			"review" => Some(Self::Review),
			"drop" => Some(Self::Drop),
			_ => None,
		}
	}

	pub fn as_str(self) -> &'static str {
		match self {
			Self::Fixed => "fixed",
			Self::Safe => "safe",
			Self::Review => "review",
			Self::Drop => "drop",
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemediationPartition {
	Fixed,
	Safe,
	Review,
	Drop,
	CombinedClean,
}

impl RemediationPartition {
	pub fn parse(value: &str) -> Option<Self> {
		match value {
			"fixed" => Some(Self::Fixed),
			"safe" => Some(Self::Safe),
			"review" => Some(Self::Review),
			"drop" => Some(Self::Drop),
			"combined_clean" => Some(Self::CombinedClean),
			_ => None,
		}
	}

	pub fn as_str(self) -> &'static str {
		match self {
			Self::Fixed => "fixed",
			Self::Safe => "safe",
			Self::Review => "review",
			Self::Drop => "drop",
			Self::CombinedClean => "combined_clean",
		}
	}

	pub fn includes(self, classification: RemediationClassification) -> bool {
		match self {
			Self::Fixed => classification == RemediationClassification::Fixed,
			Self::Safe => classification == RemediationClassification::Safe,
			Self::Review => classification == RemediationClassification::Review,
			Self::Drop => classification == RemediationClassification::Drop,
			Self::CombinedClean => matches!(
				classification,
				RemediationClassification::Fixed | RemediationClassification::Safe
			),
		}
	}
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct RemediationSummaryCounts {
	pub fixed: i64,
	pub safe: i64,
	pub review: i64,
	pub drop: i64,
}

impl RemediationSummaryCounts {
	pub fn add(&mut self, classification: RemediationClassification) {
		match classification {
			RemediationClassification::Fixed => self.fixed += 1,
			RemediationClassification::Safe => self.safe += 1,
			RemediationClassification::Review => self.review += 1,
			RemediationClassification::Drop => self.drop += 1,
		}
	}
}

#[derive(Debug, Clone)]
pub struct RemediationInputRow {
	pub row_number: i32,
	pub original: Map<String, Value>,
	pub email_column: String,
	pub result: Option<Value>,
	pub score_category: Option<String>,
	pub sub_reason: Option<String>,
	pub safe_to_send: Option<bool>,
	pub is_duplicate: bool,
	pub suppressed: bool,
}

#[derive(Debug, Clone)]
pub struct ClassifiedRemediationRow {
	pub row_number: i32,
	pub classification: RemediationClassification,
	pub rule_id: String,
	pub confidence: &'static str,
	pub before: Map<String, Value>,
	pub after: Map<String, Value>,
}

pub fn options_hash(options: &RemediationOptions) -> String {
	hash_json(options)
}

pub fn hash_json<T: Serialize>(value: &T) -> String {
	let bytes = serde_json::to_vec(value).expect("serializing remediation hash input cannot fail");
	let mut hasher = Sha256::new();
	hasher.update(bytes);
	hex::encode(hasher.finalize())
}

pub fn classify_row(
	row: RemediationInputRow,
	options: &RemediationOptions,
) -> ClassifiedRemediationRow {
	let before = row.original.clone();
	let original_email = email_value(&row.original, &row.email_column);
	let (effective_email, fix_rule) =
		fixed_email_candidate(&original_email, row.result.as_ref(), options);
	let mut after = row.original.clone();
	if let Some(email) = &effective_email {
		after.insert(row.email_column.clone(), Value::String(email.clone()));
	}

	let classification = if row.suppressed && options.drop_suppressed {
		(RemediationClassification::Drop, "suppressed", "high")
	} else if row.is_duplicate && options.collapse_duplicates {
		(
			RemediationClassification::Drop,
			"duplicate_collapsed",
			"high",
		)
	} else if row.is_duplicate {
		(
			RemediationClassification::Review,
			"duplicate_collision",
			"medium",
		)
	} else if let Some(rule) = fix_rule {
		(RemediationClassification::Fixed, rule, "high")
	} else if is_hard_drop(&row) {
		(RemediationClassification::Drop, drop_rule_id(&row), "high")
	} else if let Some(rule) = review_rule_id(&row) {
		(RemediationClassification::Review, rule, "medium")
	} else if row.safe_to_send == Some(true) || row.score_category.as_deref() == Some("valid") {
		(RemediationClassification::Safe, "safe_to_send", "high")
	} else {
		(RemediationClassification::Review, "needs_review", "low")
	};

	ClassifiedRemediationRow {
		row_number: row.row_number,
		classification: classification.0,
		rule_id: classification.1.to_string(),
		confidence: classification.2,
		before,
		after,
	}
}

pub fn suppression_lookup_candidates(
	original: &Map<String, Value>,
	email_column: &str,
	result: Option<&Value>,
	options: &RemediationOptions,
) -> Vec<String> {
	let original_email = email_value(original, email_column);
	let mut candidates = vec![original_email.trim().to_lowercase()];
	let (effective_email, _) = fixed_email_candidate(&original_email, result, options);
	if let Some(effective_email) = effective_email {
		candidates.push(effective_email.trim().to_lowercase());
	}
	candidates.sort();
	candidates.dedup();
	candidates
}

pub fn render_remediation_csv(
	headers: &[String],
	email_column: &str,
	rows: &[ClassifiedRemediationRow],
	partition: RemediationPartition,
) -> Result<Vec<u8>, csv::Error> {
	let mut writer = csv::WriterBuilder::new()
		.has_headers(false)
		.from_writer(Vec::new());

	let mut header = headers.to_vec();
	header.extend([
		"_reacher_classification".to_string(),
		"_reacher_rule_id".to_string(),
		"_reacher_confidence".to_string(),
		"_reacher_original_email".to_string(),
		"_reacher_effective_email".to_string(),
	]);
	writer.write_record(&header)?;

	for row in rows
		.iter()
		.filter(|row| partition.includes(row.classification))
	{
		let mut record = Vec::with_capacity(headers.len() + 5);
		for header in headers {
			record.push(
				row.after
					.get(header)
					.and_then(Value::as_str)
					.unwrap_or_default()
					.to_string(),
			);
		}
		record.push(row.classification.as_str().to_string());
		record.push(row.rule_id.clone());
		record.push(row.confidence.to_string());
		record.push(email_value(&row.before, email_column));
		record.push(email_value(&row.after, email_column));
		writer.write_record(&record)?;
	}

	writer.into_inner().map_err(|err| err.into_error().into())
}

fn email_value(row: &Map<String, Value>, email_column: &str) -> String {
	row.get(email_column)
		.and_then(Value::as_str)
		.unwrap_or_default()
		.to_string()
}

fn fixed_email_candidate(
	original_email: &str,
	result: Option<&Value>,
	options: &RemediationOptions,
) -> (Option<String>, Option<&'static str>) {
	let stripped = strip_control_chars(original_email);
	let trimmed = stripped.trim();
	let normalized = lowercase_domain(trimmed);

	if options.apply_domain_typos {
		if let Some(suggestion) = domain_suggestion(result) {
			let suggestion = lowercase_domain(suggestion.trim());
			if is_basic_email(&suggestion) && suggestion != original_email {
				return (Some(suggestion), Some("domain_typo_suggestion"));
			}
		}
	}

	if normalized != original_email && is_basic_email(&normalized) {
		return (Some(normalized), Some("email_format_normalized"));
	}

	(None, None)
}

fn strip_control_chars(value: &str) -> String {
	value
		.chars()
		.filter(|ch| {
			!ch.is_control() && !matches!(*ch as u32, 0x200B | 0x200C | 0x200D | 0x2060 | 0xFEFF)
		})
		.collect()
}

fn lowercase_domain(email: &str) -> String {
	let Some((local, domain)) = email.rsplit_once('@') else {
		return email.to_string();
	};
	format!("{}@{}", local, domain.to_lowercase())
}

fn is_basic_email(email: &str) -> bool {
	let Some((local, domain)) = email.rsplit_once('@') else {
		return false;
	};
	!local.is_empty() && !domain.is_empty() && !local.contains('@') && !domain.contains('@')
}

fn domain_suggestion(result: Option<&Value>) -> Option<&str> {
	result
		.and_then(|value| value.get("syntax"))
		.and_then(|syntax| syntax.get("suggestion"))
		.and_then(Value::as_str)
		.or_else(|| {
			result
				.and_then(|value| value.get("score"))
				.and_then(|score| score.get("domain_suggestion"))
				.and_then(Value::as_str)
		})
}

fn is_hard_drop(row: &RemediationInputRow) -> bool {
	let result = row.result.as_ref();
	let invalid_syntax = result
		.and_then(|value| value.get("syntax"))
		.and_then(|syntax| syntax.get("is_valid_syntax"))
		.and_then(Value::as_bool)
		== Some(false);
	let spam_trap = result
		.and_then(|value| value.get("misc"))
		.and_then(|misc| misc.get("is_spam_trap_domain"))
		.and_then(Value::as_bool)
		== Some(true);
	let invalid_recipient = row.score_category.as_deref() == Some("invalid")
		|| result
			.and_then(|value| value.get("is_reachable"))
			.and_then(Value::as_str)
			== Some("invalid")
		|| matches!(
			row.sub_reason.as_deref(),
			Some("invalid_recipient")
				| Some("mailbox_disabled")
				| Some("provider_rejected")
				| Some("invalid_syntax")
		);

	invalid_syntax || spam_trap || invalid_recipient
}

fn drop_rule_id(row: &RemediationInputRow) -> &'static str {
	let result = row.result.as_ref();
	if result
		.and_then(|value| value.get("misc"))
		.and_then(|misc| misc.get("is_spam_trap_domain"))
		.and_then(Value::as_bool)
		== Some(true)
	{
		return "spam_trap_domain";
	}
	if result
		.and_then(|value| value.get("syntax"))
		.and_then(|syntax| syntax.get("is_valid_syntax"))
		.and_then(Value::as_bool)
		== Some(false)
	{
		return "invalid_syntax";
	}
	"invalid_recipient"
}

fn review_rule_id(row: &RemediationInputRow) -> Option<&'static str> {
	let result = row.result.as_ref()?;
	if result
		.get("smtp")
		.and_then(|smtp| smtp.get("is_catch_all"))
		.and_then(Value::as_bool)
		== Some(true)
	{
		return Some("catch_all");
	}
	if result
		.get("misc")
		.and_then(|misc| misc.get("is_role_account"))
		.and_then(Value::as_bool)
		== Some(true)
	{
		return Some("role_account");
	}
	if result
		.get("misc")
		.and_then(|misc| misc.get("is_disposable"))
		.and_then(Value::as_bool)
		== Some(true)
	{
		return Some("disposable");
	}
	if result
		.get("smtp")
		.and_then(|smtp| smtp.get("has_full_inbox"))
		.and_then(Value::as_bool)
		== Some(true)
	{
		return Some("full_inbox");
	}
	if row.score_category.as_deref() == Some("unknown")
		|| result.get("is_reachable").and_then(Value::as_str) == Some("unknown")
	{
		return Some("unknown_smtp");
	}
	None
}

#[cfg(test)]
mod tests {
	use super::*;

	fn base_row(email: &str) -> RemediationInputRow {
		let mut original = Map::new();
		original.insert("email".to_string(), Value::String(email.to_string()));
		RemediationInputRow {
			row_number: 0,
			original,
			email_column: "email".to_string(),
			result: Some(serde_json::json!({
				"is_reachable": "safe",
				"syntax": {"is_valid_syntax": true},
				"misc": {"is_disposable": false, "is_role_account": false},
				"smtp": {"is_catch_all": false, "has_full_inbox": false}
			})),
			score_category: Some("valid".to_string()),
			sub_reason: Some("deliverable".to_string()),
			safe_to_send: Some(true),
			is_duplicate: false,
			suppressed: false,
		}
	}

	#[test]
	fn classifies_domain_case_fix() {
		let row = classify_row(base_row("user@Example.COM"), &RemediationOptions::default());
		assert_eq!(row.classification, RemediationClassification::Fixed);
		assert_eq!(row.rule_id, "email_format_normalized");
		assert_eq!(
			row.after.get("email").and_then(Value::as_str),
			Some("user@example.com")
		);
	}

	#[test]
	fn classifies_role_account_as_review() {
		let mut row = base_row("billing@example.com");
		row.result = Some(serde_json::json!({
			"is_reachable": "safe",
			"syntax": {"is_valid_syntax": true},
			"misc": {"is_role_account": true},
			"smtp": {"is_catch_all": false}
		}));
		let row = classify_row(row, &RemediationOptions::default());
		assert_eq!(row.classification, RemediationClassification::Review);
		assert_eq!(row.rule_id, "role_account");
	}

	#[test]
	fn classifies_invalid_as_drop() {
		let mut row = base_row("bad");
		row.result = Some(serde_json::json!({
			"is_reachable": "invalid",
			"syntax": {"is_valid_syntax": false}
		}));
		row.score_category = Some("invalid".to_string());
		let row = classify_row(row, &RemediationOptions::default());
		assert_eq!(row.classification, RemediationClassification::Drop);
		assert_eq!(row.rule_id, "invalid_syntax");
	}
}
