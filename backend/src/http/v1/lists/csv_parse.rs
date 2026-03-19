use crate::http::ReacherResponseError;
use serde_json::{Map, Value};
use warp::http::StatusCode;

#[derive(Debug)]
pub struct ParsedCsv {
	pub headers: Vec<String>,
	pub email_column: String,
	pub rows: Vec<Map<String, Value>>,
}

const EMAIL_HEADER_CANDIDATES: &[&str] = &[
	"email",
	"e-mail",
	"email_address",
	"emailaddress",
	"mail",
	"email address",
];

pub fn parse_csv(
	bytes: &[u8],
	explicit_email_column: Option<&str>,
) -> Result<ParsedCsv, ReacherResponseError> {
	let mut reader = csv::ReaderBuilder::new()
		.flexible(true)
		.from_reader(bytes);

	let headers = reader.headers().map_err(ReacherResponseError::from)?.clone();
	if headers.is_empty() {
		return Err(ReacherResponseError::new(StatusCode::BAD_REQUEST, "CSV must include headers"));
	}

	let headers_vec: Vec<String> = headers.iter().map(ToOwned::to_owned).collect();
	let normalized_headers: Vec<String> = headers_vec
		.iter()
		.map(|header| normalize_header(header))
		.collect();
	let mut seen = std::collections::HashSet::new();
	for header in &normalized_headers {
		if !seen.insert(header.clone()) {
			return Err(ReacherResponseError::new(
				StatusCode::BAD_REQUEST,
				"CSV contains duplicate headers",
			));
		}
	}

	let email_column = match explicit_email_column {
		Some(column) => {
			let normalized = normalize_header(column);
			let index = normalized_headers
				.iter()
				.position(|header| *header == normalized)
				.ok_or_else(|| {
					ReacherResponseError::new(
						StatusCode::BAD_REQUEST,
						format!("email_column '{}' was not found", column),
					)
				})?;
			headers_vec[index].clone()
		}
		None => {
			let index = normalized_headers
				.iter()
				.position(|header| EMAIL_HEADER_CANDIDATES.contains(&header.as_str()))
				.ok_or_else(|| {
					ReacherResponseError::new(
						StatusCode::BAD_REQUEST,
						"Unable to auto-detect an email column",
					)
				})?;
			headers_vec[index].clone()
		}
	};

	let mut rows = Vec::new();
	for record in reader.records() {
		let record = record.map_err(ReacherResponseError::from)?;
		let mut row = Map::new();
		for (index, header) in headers_vec.iter().enumerate() {
			row.insert(
				header.clone(),
				Value::String(record.get(index).unwrap_or_default().to_string()),
			);
		}
		rows.push(row);
	}

	Ok(ParsedCsv {
		headers: headers_vec,
		email_column,
		rows,
	})
}

fn normalize_header(header: &str) -> String {
	header
		.trim()
		.to_lowercase()
		.replace([' ', '-'], "_")
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_csv_detects_email_column() {
		let csv = b"name,email\nAlice,alice@example.com\n";
		let parsed = parse_csv(csv, None).unwrap();
		assert_eq!(parsed.email_column, "email");
		assert_eq!(parsed.rows.len(), 1);
	}

	#[test]
	fn parse_csv_rejects_duplicate_headers() {
		let csv = b"email,email\none,two\n";
		assert!(parse_csv(csv, None).is_err());
	}
}
