use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
	#[error("SQLX error: {0}")]
	SqlxError(#[from] sqlx::error::Error),
	#[error("SQLX migrate error: {0}")]
	MigrateError(#[from] sqlx::migrate::MigrateError),
	#[error("Serde JSON error: {0}")]
	SerdeJsonError(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_from_sqlx_error() {
		let err: StorageError = sqlx::Error::RowNotFound.into();
		assert!(err.to_string().contains("SQLX error"));
	}

	#[test]
	fn test_from_serde_json_error() {
		let json_err = serde_json::from_str::<serde_json::Value>("{bad").unwrap_err();
		let err: StorageError = json_err.into();
		assert!(err.to_string().contains("Serde JSON error"));
	}

	#[test]
	fn test_display_format() {
		let err: StorageError = sqlx::Error::RowNotFound.into();
		let display = format!("{}", err);
		assert!(display.starts_with("SQLX error:"));
	}
}
