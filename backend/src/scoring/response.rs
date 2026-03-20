use crate::scoring::{compute_freshness, compute_score};
use check_if_email_exists::CheckEmailOutput;
use chrono::{DateTime, Utc};
use serde_json::{Map, Value};

pub fn scored_json(output: &CheckEmailOutput) -> Result<Value, serde_json::Error> {
	let mut scored = serde_json::to_value(output)?;
	let mut score = serde_json::to_value(compute_score(output))?;

	// Surface domain typo suggestion in score object (#31)
	if let Some(suggestion) = &output.syntax.suggestion {
		if let Some(obj) = score.as_object_mut() {
			obj.insert(
				"domain_suggestion".into(),
				Value::String(suggestion.clone()),
			);
		}
	}

	// Surface canonical (normalized) email in score object (#32)
	let canonical = crate::http::v1::lists::canonicalize::canonicalize_email(&output.input);
	if let Some(ref canon) = canonical {
		if canon != &output.input {
			if let Some(obj) = score.as_object_mut() {
				obj.insert("normalized_email".into(), Value::String(canon.clone()));
			}
		}
	}

	// Add catch-all severity tier (#30)
	if let Some(obj) = score.as_object_mut() {
		if let Some(signals) = obj.get("signals").and_then(|s| s.as_object()) {
			if signals
				.get("smtp_is_catch_all")
				.and_then(|v| v.as_bool())
				.unwrap_or(false)
			{
				let is_free = signals
					.get("is_free_provider")
					.and_then(|v| v.as_bool())
					.unwrap_or(false);
				let tier = if is_free { "low" } else { "high" };
				obj.insert("catch_all_severity".into(), Value::String(tier.to_string()));
			}
		}
	}

	match &mut scored {
		Value::Object(map) => {
			map.insert("score".into(), score);
			Ok(scored)
		}
		_ => {
			let mut map = Map::new();
			map.insert("result".into(), scored);
			map.insert("score".into(), score);
			Ok(Value::Object(map))
		}
	}
}

pub fn scored_response(output: &CheckEmailOutput) -> Result<Vec<u8>, serde_json::Error> {
	serde_json::to_vec(&scored_json(output)?)
}

pub fn scored_json_with_freshness(
	output: &CheckEmailOutput,
	completed_at: Option<DateTime<Utc>>,
) -> Result<Value, serde_json::Error> {
	let mut value = scored_json(output)?;
	if let Some(ts) = completed_at {
		inject_freshness_into_result(&mut value, ts);
	}
	Ok(value)
}

pub fn scored_response_fresh(output: &CheckEmailOutput) -> Result<Vec<u8>, serde_json::Error> {
	serde_json::to_vec(&scored_json_with_freshness(output, Some(Utc::now()))?)
}

pub fn inject_freshness_into_result(result: &mut Value, completed_at: DateTime<Utc>) {
	if let Some(score_obj) = result.get_mut("score").and_then(Value::as_object_mut) {
		let info = compute_freshness(completed_at);
		score_obj.insert("verified_at".into(), Value::String(info.verified_at));
		score_obj.insert("age_days".into(), Value::from(info.age_days));
		score_obj.insert(
			"freshness".into(),
			serde_json::to_value(&info.freshness).unwrap_or(Value::Null),
		);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use check_if_email_exists::{
		smtp::SmtpDetails, syntax::SyntaxDetails, CheckEmailOutput, Reachable,
	};

	#[test]
	fn scored_json_appends_score() {
		let value = scored_json(&CheckEmailOutput::default()).unwrap();
		assert!(value.get("score").is_some());
	}

	#[test]
	fn domain_suggestion_surfaced_in_score() {
		let mut output = CheckEmailOutput::default();
		output.syntax.suggestion = Some("user@gmail.com".to_string());
		let value = scored_json(&output).unwrap();
		let score = value.get("score").unwrap();
		assert_eq!(
			score.get("domain_suggestion").and_then(|v| v.as_str()),
			Some("user@gmail.com")
		);
	}

	#[test]
	fn no_domain_suggestion_when_none() {
		let output = CheckEmailOutput::default();
		let value = scored_json(&output).unwrap();
		let score = value.get("score").unwrap();
		assert!(score.get("domain_suggestion").is_none());
	}

	#[test]
	fn normalized_email_surfaced_when_different() {
		let mut output = CheckEmailOutput::default();
		output.input = "User+tag@Gmail.com".to_string();
		output.syntax = SyntaxDetails {
			address: None,
			domain: "gmail.com".to_string(),
			is_valid_syntax: true,
			username: "user+tag".to_string(),
			normalized_email: Some("user+tag@gmail.com".to_string()),
			suggestion: None,
		};
		let value = scored_json(&output).unwrap();
		let score = value.get("score").unwrap();
		assert_eq!(
			score.get("normalized_email").and_then(|v| v.as_str()),
			Some("user@gmail.com")
		);
	}

	#[test]
	fn catch_all_severity_high_for_corporate() {
		let mut output = CheckEmailOutput::default();
		output.input = "user@company.com".to_string();
		output.is_reachable = Reachable::Risky;
		output.smtp = Ok(SmtpDetails {
			can_connect_smtp: true,
			has_full_inbox: false,
			is_catch_all: true,
			is_deliverable: true,
			is_disabled: false,
		});
		let value = scored_json(&output).unwrap();
		let score = value.get("score").unwrap();
		assert_eq!(
			score.get("catch_all_severity").and_then(|v| v.as_str()),
			Some("high")
		);
	}

	#[test]
	fn no_catch_all_severity_when_not_catch_all() {
		let output = CheckEmailOutput::default();
		let value = scored_json(&output).unwrap();
		let score = value.get("score").unwrap();
		assert!(score.get("catch_all_severity").is_none());
	}
}
