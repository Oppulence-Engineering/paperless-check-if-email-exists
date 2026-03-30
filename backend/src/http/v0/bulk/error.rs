// Reacher - Email Verification
// Copyright (C) 2018-2023 Reacher

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use warp::reject;

#[derive(Debug)]
pub enum CsvError {
	CsvLib(csv::Error),
	CsvLibWriter(Box<csv::IntoInnerError<csv::Writer<Vec<u8>>>>),
	Parse(&'static str),
}

/// Catch all error struct for the bulk endpoints
#[derive(Debug)]
pub enum BulkError {
	EmptyInput,
	JobInProgress,
	Db(sqlx::Error),
	Csv(CsvError),
	Json(serde_json::Error),
}

impl std::fmt::Display for CsvError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::CsvLib(err) => write!(f, "csv error: {err}"),
			Self::CsvLibWriter(err) => write!(f, "csv writer error: {err}"),
			Self::Parse(message) => write!(f, "csv parse error: {message}"),
		}
	}
}

impl std::error::Error for CsvError {}

impl std::fmt::Display for BulkError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::EmptyInput => write!(f, "bulk input is empty"),
			Self::JobInProgress => write!(f, "bulk job is still in progress"),
			Self::Db(err) => write!(f, "database error: {err}"),
			Self::Csv(err) => write!(f, "{err}"),
			Self::Json(err) => write!(f, "json error: {err}"),
		}
	}
}

impl std::error::Error for BulkError {}

// Defaults to Internal server error
impl reject::Reject for BulkError {}

// wrap sql errors as db errors for reacher
impl From<sqlx::Error> for BulkError {
	fn from(e: sqlx::Error) -> Self {
		BulkError::Db(e)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_bulk_error_from_sqlx() {
		let err: BulkError = sqlx::Error::RowNotFound.into();
		assert!(matches!(err, BulkError::Db(_)));
	}

	#[test]
	fn test_bulk_error_empty_input() {
		let err = BulkError::EmptyInput;
		assert!(matches!(err, BulkError::EmptyInput));
	}

	#[test]
	fn test_bulk_error_job_in_progress() {
		let err = BulkError::JobInProgress;
		assert!(matches!(err, BulkError::JobInProgress));
	}

	#[test]
	fn test_csv_error_parse() {
		let err = CsvError::Parse("test error");
		assert!(matches!(err, CsvError::Parse(_)));
	}

	#[test]
	fn test_bulk_error_debug() {
		let err = BulkError::EmptyInput;
		let debug = format!("{:?}", err);
		assert!(debug.contains("EmptyInput"));
	}
}
