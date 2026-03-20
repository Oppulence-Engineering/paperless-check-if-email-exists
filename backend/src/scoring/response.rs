use crate::scoring::{compute_freshness, compute_score};
use check_if_email_exists::CheckEmailOutput;
use chrono::{DateTime, Utc};
use serde_json::{Map, Value};

pub fn scored_json(output: &CheckEmailOutput) -> Result<Value, serde_json::Error> {
	let mut scored = serde_json::to_value(output)?;
	let score = serde_json::to_value(compute_score(output))?;
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
	use check_if_email_exists::CheckEmailOutput;

	#[test]
	fn scored_json_appends_score() {
		let value = scored_json(&CheckEmailOutput::default()).unwrap();
		assert!(value.get("score").is_some());
	}
}
