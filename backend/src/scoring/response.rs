use crate::scoring::compute_score;
use check_if_email_exists::CheckEmailOutput;
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
