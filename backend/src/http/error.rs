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

use crate::storage::error::StorageError;
use check_if_email_exists::{CheckEmailInputBuilderError, LOG_TARGET};
use serde::ser::SerializeStruct;
use serde::Serialize;
use std::fmt;
use std::fmt::Debug;
use tracing::error;
use warp::{http::StatusCode, reject};

/// Trait combining Display and Debug.
pub trait DisplayDebug: fmt::Display + Debug + Sync + Send {}
impl<T: fmt::Display + Debug + Sync + Send> DisplayDebug for T {}

/// Struct describing an error response.
#[derive(Debug, thiserror::Error)]
pub struct ReacherResponseError {
	pub code: StatusCode,
	pub error: Box<dyn DisplayDebug>,
}

impl reject::Reject for ReacherResponseError {}

impl Serialize for ReacherResponseError {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		let mut state = serializer.serialize_struct("ReacherResponseError", 1)?;
		state.serialize_field("error", &self.error.to_string())?;
		state.end()
	}
}

impl fmt::Display for ReacherResponseError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.error)
	}
}

impl ReacherResponseError {
	pub fn new<T: DisplayDebug + 'static>(code: StatusCode, error: T) -> Self {
		Self {
			code,
			error: Box::new(error),
		}
	}
}

impl From<CheckEmailInputBuilderError> for ReacherResponseError {
	fn from(e: CheckEmailInputBuilderError) -> Self {
		Self {
			code: StatusCode::BAD_REQUEST,
			error: Box::new(e),
		}
	}
}

impl From<serde_json::Error> for ReacherResponseError {
	fn from(e: serde_json::Error) -> Self {
		ReacherResponseError::new(StatusCode::INTERNAL_SERVER_ERROR, e)
	}
}

impl From<lapin::Error> for ReacherResponseError {
	fn from(e: lapin::Error) -> Self {
		ReacherResponseError::new(StatusCode::INTERNAL_SERVER_ERROR, e)
	}
}

impl From<sqlx::Error> for ReacherResponseError {
	fn from(e: sqlx::Error) -> Self {
		ReacherResponseError::new(StatusCode::INTERNAL_SERVER_ERROR, e)
	}
}

impl From<csv::Error> for ReacherResponseError {
	fn from(e: csv::Error) -> Self {
		ReacherResponseError::new(StatusCode::INTERNAL_SERVER_ERROR, e)
	}
}

impl From<csv::IntoInnerError<csv::Writer<Vec<u8>>>> for ReacherResponseError {
	fn from(e: csv::IntoInnerError<csv::Writer<Vec<u8>>>) -> Self {
		ReacherResponseError::new(StatusCode::INTERNAL_SERVER_ERROR, e)
	}
}

impl From<warp::http::status::InvalidStatusCode> for ReacherResponseError {
	fn from(e: warp::http::status::InvalidStatusCode) -> Self {
		ReacherResponseError::new(StatusCode::INTERNAL_SERVER_ERROR, e)
	}
}

impl From<anyhow::Error> for ReacherResponseError {
	fn from(e: anyhow::Error) -> Self {
		ReacherResponseError::new(StatusCode::INTERNAL_SERVER_ERROR, e)
	}
}

impl From<StorageError> for ReacherResponseError {
	fn from(e: StorageError) -> Self {
		ReacherResponseError::new(StatusCode::INTERNAL_SERVER_ERROR, e)
	}
}

impl From<reqwest::Error> for ReacherResponseError {
	fn from(e: reqwest::Error) -> Self {
		ReacherResponseError::new(
			e.status()
				.map(|s| s.as_u16())
				.map(StatusCode::from_u16)
				.and_then(Result::ok)
				.unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
			e,
		)
	}
}

/// This function receives a `Rejection` and tries to return a custom value,
/// otherwise simply passes the rejection along.
pub async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
	if let Some(err) = err.find::<ReacherResponseError>() {
		error!(target: LOG_TARGET, code=?err.code, message=?err.to_string(), "Request rejected");
		Ok((warp::reply::with_status(warp::reply::json(err), err.code),))
	} else {
		Err(err)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_new_error() {
		let err = ReacherResponseError::new(StatusCode::BAD_REQUEST, "bad input");
		assert_eq!(err.code, StatusCode::BAD_REQUEST);
		assert_eq!(err.to_string(), "bad input");
	}

	#[test]
	fn test_error_serializes_to_json() {
		let err = ReacherResponseError::new(StatusCode::NOT_FOUND, "not found");
		let json = serde_json::to_value(&err).unwrap();
		assert_eq!(json["error"], "not found");
		// code is not serialized — only "error" field
		assert!(json.get("code").is_none());
	}

	#[test]
	fn test_display_shows_message() {
		let err = ReacherResponseError::new(StatusCode::INTERNAL_SERVER_ERROR, "oops");
		assert_eq!(format!("{}", err), "oops");
	}

	#[test]
	fn test_from_serde_json_error() {
		let bad_json: Result<serde_json::Value, _> = serde_json::from_str("{invalid");
		let err: ReacherResponseError = bad_json.unwrap_err().into();
		assert_eq!(err.code, StatusCode::INTERNAL_SERVER_ERROR);
	}

	#[test]
	fn test_from_sqlx_error() {
		let err: ReacherResponseError =
			sqlx::Error::RowNotFound.into();
		assert_eq!(err.code, StatusCode::INTERNAL_SERVER_ERROR);
		assert!(err.to_string().contains("no rows"));
	}

	#[test]
	fn test_from_anyhow_error() {
		let err: ReacherResponseError =
			anyhow::anyhow!("something went wrong").into();
		assert_eq!(err.code, StatusCode::INTERNAL_SERVER_ERROR);
		assert!(err.to_string().contains("something went wrong"));
	}

	#[test]
	fn test_from_storage_error() {
		let storage_err = StorageError::SerdeJsonError(
			serde_json::from_str::<serde_json::Value>("{bad").unwrap_err(),
		);
		let err: ReacherResponseError = storage_err.into();
		assert_eq!(err.code, StatusCode::INTERNAL_SERVER_ERROR);
	}
}
