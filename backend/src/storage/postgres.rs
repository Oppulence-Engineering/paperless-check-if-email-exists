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

use super::error::StorageError;
use crate::worker::do_work::{CheckEmailJobId, CheckEmailTask, TaskError};
use check_if_email_exists::{CheckEmailOutput, LOG_TARGET};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tracing::{debug, info};
use uuid::Uuid;

#[derive(Debug)]
pub struct PostgresStorage {
	pub pg_pool: PgPool,
	extra: Option<serde_json::Value>,
}

impl PostgresStorage {
	pub async fn new(db_url: &str, extra: Option<serde_json::Value>) -> Result<Self, StorageError> {
		debug!(target: LOG_TARGET, "Connecting to DB: {}", db_url);
		let pg_pool = PgPoolOptions::new().connect(db_url).await?;

		sqlx::migrate!("./migrations").run(&pg_pool).await?;

		info!(target: LOG_TARGET, table="v1_task_result", "Connected to DB, Reacher will write verification results to DB");

		Ok(Self { pg_pool, extra })
	}

	pub async fn store(
		&self,
		task: &CheckEmailTask,
		worker_output: &Result<CheckEmailOutput, TaskError>,
		extra: Option<serde_json::Value>,
	) -> Result<(), StorageError> {
		let payload_json = serde_json::to_value(task)?;

		// Extract tenant_id and task_db_id from metadata if present
		let tenant_id: Option<Uuid> = task
			.metadata
			.as_ref()
			.and_then(|m| m.tenant_id.as_ref())
			.and_then(|id| id.parse().ok());

		let task_db_id: Option<i32> = task
			.metadata
			.as_ref()
			.and_then(|m| m.task_db_id);

		match worker_output {
			Ok(output) => {
				let output_json = serde_json::to_value(output)?;

				if let Some(db_id) = task_db_id {
					// UPDATE the pre-created row instead of inserting a duplicate
					let upd = sqlx::query(
						"UPDATE v1_task_result SET payload = $1, extra = $2, result = $3, tenant_id = $4 WHERE id = $5",
					)
					.bind(&payload_json)
					.bind(&extra)
					.bind(&output_json)
					.bind(tenant_id)
					.bind(db_id)
					.execute(&self.pg_pool)
					.await?;
					// If pre-created row was deleted, fall back to INSERT
					if upd.rows_affected() == 0 {
						sqlx::query(
							"INSERT INTO v1_task_result (payload, job_id, extra, result, tenant_id) VALUES ($1, $2, $3, $4, $5)",
						)
						.bind(&payload_json)
						.bind(match task.job_id { CheckEmailJobId::Bulk(jid) => Some(jid), CheckEmailJobId::SingleShot => None })
						.bind(&extra)
						.bind(&output_json)
						.bind(tenant_id)
						.execute(&self.pg_pool)
						.await?;
					}
				} else {
					sqlx::query!(
						r#"
						INSERT INTO v1_task_result (payload, job_id, extra, result, tenant_id)
						VALUES ($1, $2, $3, $4, $5)
						RETURNING id
						"#,
						payload_json,
						match task.job_id {
							CheckEmailJobId::Bulk(job_id) => Some(job_id),
							CheckEmailJobId::SingleShot => None,
						},
						extra,
						output_json,
						tenant_id,
					)
					.fetch_one(&self.pg_pool)
					.await?;
				}
			}
			Err(err) => {
				if let Some(db_id) = task_db_id {
					let upd = sqlx::query(
						"UPDATE v1_task_result SET payload = $1, extra = $2, error = $3, tenant_id = $4 WHERE id = $5",
					)
					.bind(&payload_json)
					.bind(&extra)
					.bind(err.to_string())
					.bind(tenant_id)
					.bind(db_id)
					.execute(&self.pg_pool)
					.await?;
					if upd.rows_affected() == 0 {
						sqlx::query(
							"INSERT INTO v1_task_result (payload, job_id, extra, error, tenant_id) VALUES ($1, $2, $3, $4, $5)",
						)
						.bind(&payload_json)
						.bind(match task.job_id { CheckEmailJobId::Bulk(jid) => Some(jid), CheckEmailJobId::SingleShot => None })
						.bind(&extra)
						.bind(err.to_string())
						.bind(tenant_id)
						.execute(&self.pg_pool)
						.await?;
					}
				} else {
					sqlx::query!(
						r#"
						INSERT INTO v1_task_result (payload, job_id, extra, error, tenant_id)
						VALUES ($1, $2, $3, $4, $5)
						RETURNING id
						"#,
						payload_json,
						match task.job_id {
							CheckEmailJobId::Bulk(job_id) => Some(job_id),
							CheckEmailJobId::SingleShot => None,
						},
						extra,
						err.to_string(),
						tenant_id,
					)
					.fetch_one(&self.pg_pool)
					.await?;
				}
			}
		}

		debug!(target: LOG_TARGET, email=?task.input.to_email, "Wrote to DB");

		Ok(())
	}

	pub fn get_extra(&self) -> Option<serde_json::Value> {
		self.extra.clone()
	}
}
