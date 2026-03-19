use csv::WriterBuilder;
use serde::Serialize;
use serde_json::{Map, Value};

#[derive(Debug, Clone)]
pub struct TaskResultRecord {
	pub id: i64,
	pub payload: Value,
	pub result: Option<Value>,
	pub error: Option<String>,
	pub score: Option<i16>,
	pub score_category: Option<String>,
	pub sub_reason: Option<String>,
	pub safe_to_send: Option<bool>,
	pub reason_codes: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct CsvDownloadRow {
	pub input: String,
	pub is_reachable: String,
	pub score: Option<i16>,
	pub category: Option<String>,
	pub sub_reason: Option<String>,
	pub safe_to_send: Option<bool>,
	pub reason_codes: Option<String>,
	pub is_disposable: Option<bool>,
	pub is_role_account: Option<bool>,
	pub mx_accepts_mail: Option<bool>,
	pub smtp_can_connect: Option<bool>,
	pub smtp_is_catch_all: Option<bool>,
	pub smtp_is_deliverable: Option<bool>,
	pub error: Option<String>,
}

pub const CSV_HEADER: &str =
	"input,is_reachable,score,category,sub_reason,safe_to_send,reason_codes,is_disposable,is_role_account,mx_accepts_mail,smtp_can_connect,smtp_is_catch_all,smtp_is_deliverable,error\n";

pub fn csv_rows(records: &[TaskResultRecord]) -> Result<Vec<u8>, csv::Error> {
	let mut writer = WriterBuilder::new()
		.has_headers(false)
		.from_writer(Vec::new());
	for record in records {
		writer.serialize(csv_row(record))?;
	}
	writer.into_inner().map_err(|err| err.into_error().into())
}

pub fn ndjson_rows(records: &[TaskResultRecord]) -> Result<Vec<u8>, serde_json::Error> {
	let mut bytes = Vec::new();
	for record in records {
		let line = ndjson_line(record)?;
		bytes.extend_from_slice(&line);
		bytes.push(b'\n');
	}
	Ok(bytes)
}

pub fn csv_row(record: &TaskResultRecord) -> CsvDownloadRow {
	let input = result_value(record, &["input"])
		.and_then(Value::as_str)
		.map(ToOwned::to_owned)
		.or_else(|| payload_input(&record.payload))
		.unwrap_or_default();
	let is_reachable = result_value(record, &["is_reachable"])
		.and_then(Value::as_str)
		.unwrap_or_default()
		.to_string();
	let score = result_value(record, &["score", "score"])
		.and_then(Value::as_i64)
		.map(|value| value as i16)
		.or(record.score);
	let category = result_value(record, &["score", "category"])
		.and_then(Value::as_str)
		.map(ToOwned::to_owned)
		.or_else(|| record.score_category.clone());
	let sub_reason = result_value(record, &["score", "sub_reason"])
		.and_then(Value::as_str)
		.map(ToOwned::to_owned)
		.or_else(|| record.sub_reason.clone());

	let safe_to_send = result_value(record, &["score", "safe_to_send"])
		.and_then(Value::as_bool)
		.or(record.safe_to_send);

	let reason_codes = result_value(record, &["score", "reason_codes"])
		.and_then(Value::as_array)
		.map(|arr| {
			arr.iter()
				.filter_map(Value::as_str)
				.collect::<Vec<_>>()
				.join("|")
		})
		.or_else(|| record.reason_codes.as_ref().map(|codes| codes.join("|")));

	CsvDownloadRow {
		input,
		is_reachable,
		score,
		category,
		sub_reason,
		safe_to_send,
		reason_codes,
		is_disposable: result_value(record, &["misc", "is_disposable"]).and_then(Value::as_bool),
		is_role_account: result_value(record, &["misc", "is_role_account"])
			.and_then(Value::as_bool),
		mx_accepts_mail: result_value(record, &["mx", "accepts_mail"]).and_then(Value::as_bool),
		smtp_can_connect: result_value(record, &["smtp", "can_connect_smtp"])
			.and_then(Value::as_bool),
		smtp_is_catch_all: result_value(record, &["smtp", "is_catch_all"]).and_then(Value::as_bool),
		smtp_is_deliverable: result_value(record, &["smtp", "is_deliverable"])
			.and_then(Value::as_bool),
		error: record.error.clone(),
	}
}

pub fn ndjson_line(record: &TaskResultRecord) -> Result<Vec<u8>, serde_json::Error> {
	let mut line = match &record.result {
		Some(Value::Object(object)) => Value::Object(object.clone()),
		Some(other) => other.clone(),
		None => Value::Object(Map::new()),
	};

	if let Value::Object(map) = &mut line {
		if !map.contains_key("input") {
			if let Some(input) = payload_input(&record.payload) {
				map.insert("input".into(), Value::String(input));
			}
		}
		if let Some(error) = &record.error {
			map.insert("error".into(), Value::String(error.clone()));
		}
		if !map.contains_key("score")
			&& (record.score.is_some()
				|| record.score_category.is_some()
				|| record.sub_reason.is_some()
				|| record.safe_to_send.is_some())
		{
			let mut score = Map::new();
			if let Some(value) = record.score {
				score.insert("score".into(), Value::from(value));
			}
			if let Some(value) = &record.score_category {
				score.insert("category".into(), Value::String(value.clone()));
			}
			if let Some(value) = &record.sub_reason {
				score.insert("sub_reason".into(), Value::String(value.clone()));
			}
			if let Some(value) = record.safe_to_send {
				score.insert("safe_to_send".into(), Value::Bool(value));
			}
			if let Some(codes) = &record.reason_codes {
				score.insert(
					"reason_codes".into(),
					Value::Array(codes.iter().map(|c| Value::String(c.clone())).collect()),
				);
			}
			map.insert("score".into(), Value::Object(score));
		}
	}

	serde_json::to_vec(&line)
}

fn payload_input(payload: &Value) -> Option<String> {
	payload
		.get("input")
		.and_then(|value| value.get("to_email"))
		.and_then(Value::as_str)
		.map(ToOwned::to_owned)
}

fn result_value<'a>(record: &'a TaskResultRecord, path: &[&str]) -> Option<&'a Value> {
	let mut current = record.result.as_ref()?;
	for segment in path {
		current = current.get(*segment)?;
	}
	Some(current)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn csv_row_uses_scored_result_payload() {
		let row = csv_row(&TaskResultRecord {
			id: 1,
			payload: serde_json::json!({"input": {"to_email": "fallback@example.com"}}),
			result: Some(serde_json::json!({
				"input": "user@example.com",
				"is_reachable": "safe",
				"misc": {"is_disposable": false, "is_role_account": true},
				"mx": {"accepts_mail": true},
				"smtp": {"can_connect_smtp": true, "is_catch_all": false, "is_deliverable": true},
				"score": {"score": 95, "category": "valid", "sub_reason": "deliverable", "safe_to_send": true, "reason_codes": ["role_account"]}
			})),
			error: None,
			score: None,
			score_category: None,
			sub_reason: None,
			safe_to_send: None,
			reason_codes: None,
		});

		assert_eq!(row.input, "user@example.com");
		assert_eq!(row.score, Some(95));
		assert_eq!(row.category.as_deref(), Some("valid"));
	}
}
