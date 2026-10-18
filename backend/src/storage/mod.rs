pub mod error;
pub mod postgres;

use crate::scoring::response::PreparedCheckEmailSuccess;
use crate::worker::do_work::{CheckEmailTask, TaskError};
use check_if_email_exists::CheckEmailOutput;
use error::StorageError;
use postgres::PostgresStorage;
use std::fmt::Debug;

#[derive(Debug, Default)]
pub enum StorageAdapter {
	Postgres(PostgresStorage),
	#[default]
	Noop,
}

impl StorageAdapter {
	pub async fn store(
		&self,
		task: &CheckEmailTask,
		worker_output: &Result<CheckEmailOutput, TaskError>,
		extra: Option<serde_json::Value>,
	) -> Result<(), StorageError> {
		match self {
			StorageAdapter::Postgres(storage) => storage.store(task, worker_output, extra).await,
			StorageAdapter::Noop => Ok(()),
		}
	}

	pub async fn store_prepared(
		&self,
		task: &CheckEmailTask,
		success: &PreparedCheckEmailSuccess,
		extra: Option<serde_json::Value>,
	) -> Result<(), StorageError> {
		match self {
			StorageAdapter::Postgres(storage) => storage.store_prepared(task, success, extra).await,
			StorageAdapter::Noop => Ok(()),
		}
	}

	pub async fn store_error(
		&self,
		task: &CheckEmailTask,
		error: &TaskError,
		extra: Option<serde_json::Value>,
	) -> Result<(), StorageError> {
		match self {
			StorageAdapter::Postgres(storage) => storage.store_error_only(task, error, extra).await,
			StorageAdapter::Noop => Ok(()),
		}
	}

	pub fn get_extra(&self) -> Option<serde_json::Value> {
		match self {
			StorageAdapter::Postgres(storage) => storage.get_extra().clone(),
			StorageAdapter::Noop => None,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::worker::do_work::{CheckEmailJobId, CheckEmailTask};
	use check_if_email_exists::{CheckEmailOutput, Reachable};

	fn make_task() -> CheckEmailTask {
		CheckEmailTask {
			input: check_if_email_exists::CheckEmailInput {
				to_email: "test@example.com".into(),
				..Default::default()
			},
			job_id: CheckEmailJobId::SingleShot,
			webhook: None,
			metadata: None,
		}
	}

	#[tokio::test]
	async fn test_noop_store_succeeds() {
		let adapter = StorageAdapter::Noop;
		let task = make_task();
		let output = Ok(CheckEmailOutput {
			input: "test@example.com".into(),
			is_reachable: Reachable::Invalid,
			..Default::default()
		});
		let result = adapter.store(&task, &output, None).await;
		assert!(result.is_ok());
	}

	#[tokio::test]
	async fn test_noop_store_with_error_succeeds() {
		let adapter = StorageAdapter::Noop;
		let task = make_task();
		let output: Result<CheckEmailOutput, TaskError> =
			Err(TaskError::Lapin(lapin::Error::InvalidChannel(0)));
		let result = adapter.store(&task, &output, None).await;
		assert!(result.is_ok());
	}

	#[test]
	fn test_noop_get_extra_returns_none() {
		let adapter = StorageAdapter::Noop;
		assert!(adapter.get_extra().is_none());
	}

	#[test]
	fn test_default_is_noop() {
		let adapter = StorageAdapter::default();
		assert!(matches!(adapter, StorageAdapter::Noop));
	}
}
