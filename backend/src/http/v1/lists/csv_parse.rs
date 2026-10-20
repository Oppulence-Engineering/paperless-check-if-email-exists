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
	// Outlook's contact export.
	"e-mail address",
];

/// The most cells (rows times columns) an upload may build. Every row holds a
/// value for every header, even a row shorter than the header, so a wide
/// header multiplies memory by the row count whatever the file size. This
/// fits the largest plan's 100,000 rows at 50 columns.
const MAX_CSV_CELLS: usize = 5_000_000;

/// Parse an uploaded CSV. With `max_rows`, parsing stops after one row more
/// than the limit, so an oversized upload is rejected without first building
/// every row in memory; the caller sees `rows.len() > max_rows`. A file whose
/// rows times columns would pass [`MAX_CSV_CELLS`] is refused.
pub fn parse_csv(
	bytes: &[u8],
	explicit_email_column: Option<&str>,
	max_rows: Option<usize>,
) -> Result<ParsedCsv, ReacherResponseError> {
	parse_csv_capped(bytes, explicit_email_column, max_rows, MAX_CSV_CELLS)
}

fn parse_csv_capped(
	bytes: &[u8],
	explicit_email_column: Option<&str>,
	max_rows: Option<usize>,
	max_cells: usize,
) -> Result<ParsedCsv, ReacherResponseError> {
	let mut reader = csv::ReaderBuilder::new().flexible(true).from_reader(bytes);

	let headers = reader.headers().map_err(invalid_csv)?.clone();
	if headers.is_empty() {
		return Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"CSV must include headers",
		));
	}

	let headers_vec: Vec<String> = headers.iter().map(ToOwned::to_owned).collect();
	let normalized_headers: Vec<String> = headers_vec
		.iter()
		.map(|header| normalize_header(header))
		.collect();
	let mut seen = std::collections::HashSet::new();
	// Trailing commas give several empty header cells; only named columns
	// must be unique.
	for header in normalized_headers
		.iter()
		.filter(|header| !header.is_empty())
	{
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
				.position(|header| {
					EMAIL_HEADER_CANDIDATES
						.iter()
						.any(|candidate| normalize_header(candidate) == *header)
				})
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
		if max_rows.is_some_and(|max_rows| rows.len() > max_rows) {
			break;
		}
		if (rows.len() + 1).saturating_mul(headers_vec.len()) > max_cells {
			return Err(ReacherResponseError::new(
				StatusCode::BAD_REQUEST,
				format!("CSV is too large: rows times columns may be at most {max_cells}"),
			));
		}
		let record = record.map_err(invalid_csv)?;
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

/// A file that is not valid CSV (for example a Windows-1252 export, which is
/// not UTF-8) is a client error.
fn invalid_csv(error: csv::Error) -> ReacherResponseError {
	ReacherResponseError::new(StatusCode::BAD_REQUEST, format!("Invalid CSV: {error}"))
}

fn normalize_header(header: &str) -> String {
	header.trim().to_lowercase().replace([' ', '-'], "_")
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn parse_csv_detects_email_column() {
		let csv = b"name,email\nAlice,alice@example.com\n";
		let parsed = parse_csv(csv, None, None).unwrap();
		assert_eq!(parsed.email_column, "email");
		assert_eq!(parsed.rows.len(), 1);

		// Outlook exports name the column "E-mail Address", often with
		// trailing empty columns.
		let csv = b"Name,E-mail Address,,\nAlice,alice@example.com,,\n";
		let parsed = parse_csv(csv, None, None).unwrap();
		assert_eq!(parsed.email_column, "E-mail Address");

		let csv = b"email\na@x.test\nb@x.test\nc@x.test\n";
		assert_eq!(parse_csv(csv, None, Some(1)).unwrap().rows.len(), 2);
	}

	#[test]
	fn parse_csv_rejects_more_cells_than_the_cap() {
		// Short rows still get a value for every header, so the cap is on
		// rows times columns, not on the bytes of the file.
		let csv = b"email,a,b\nx@x.test\ny@x.test\n";
		assert!(parse_csv_capped(csv, None, None, 5).is_err());
		assert_eq!(parse_csv_capped(csv, None, None, 6).unwrap().rows.len(), 2);
	}

	#[test]
	fn parse_csv_rejects_duplicate_headers() {
		let csv = b"email,email\none,two\n";
		assert!(parse_csv(csv, None, None).is_err());
	}
}
