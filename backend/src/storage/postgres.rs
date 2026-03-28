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
use crate::http::v1::lists::canonicalize::canonicalize_email;
use crate::scoring::compute_score;
use crate::scoring::response::{
	scored_json, PreparedCheckEmailSuccess, PreparedVerificationResponse,
};
use crate::worker::do_work::{CheckEmailJobId, CheckEmailTask, TaskError};
use check_if_email_exists::{CheckEmailOutput, LOG_TARGET};
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tracing::{debug, info};
use uuid::Uuid;

#[derive(Debug)]
pub struct PostgresStorage {
	pub pg_pool: PgPool,
	pub read_pool: PgPool,
	extra: Option<serde_json::Value>,
}

#[derive(Debug)]
struct SuccessColumns {
	result_json: serde_json::Value,
	score: i32,
	score_category: String,
	sub_reason: String,
	safe_to_send: bool,
	reason_codes: Vec<String>,
	canonical_email: Option<String>,
	bounce_risk_score: Option<i32>,
	bounce_risk_category: Option<String>,
	bounce_risk_confidence: Option<f64>,
	bounce_risk_action: Option<String>,
	bounce_risk_model_version: Option<String>,
	bounce_risk_signals: Option<serde_json::Value>,
}

impl PostgresStorage {
	pub async fn new(
		db_url: &str,
		read_replica_url: Option<&str>,
		extra: Option<serde_json::Value>,
	) -> Result<Self, StorageError> {
		debug!(target: LOG_TARGET, "Connecting to DB: {}", db_url);
		let pg_pool = PgPoolOptions::new().connect(db_url).await?;

		sqlx::migrate!("./migrations").run(&pg_pool).await?;

		info!(target: LOG_TARGET, table="v1_task_result", "Connected to DB, Reacher will write verification results to DB");

		let read_pool = match read_replica_url {
			Some(url) if !url.is_empty() => {
				info!(target: LOG_TARGET, "Connecting to read replica");
				PgPoolOptions::new().connect(url).await?
			}
			_ => pg_pool.clone(),
		};

		Ok(Self {
			pg_pool,
			read_pool,
			extra,
		})
	}

	pub async fn store(
		&self,
		task: &CheckEmailTask,
		worker_output: &Result<CheckEmailOutput, TaskError>,
		extra: Option<serde_json::Value>,
	) -> Result<(), StorageError> {
		let payload_json = serde_json::to_value(task)?;
		let tenant_id = parse_tenant_id(task);
		let task_db_id = task.metadata.as_ref().and_then(|m| m.task_db_id);

		match worker_output {
			Ok(output) => {
				let columns = success_columns_from_output(output)?;
				self.store_success(task, &payload_json, tenant_id, task_db_id, columns, extra)
					.await?;
			}
			Err(err) => {
				self.store_error(task, &payload_json, tenant_id, task_db_id, err, extra)
					.await?;
			}
		}

		debug!(target: LOG_TARGET, email=?task.input.to_email, "Wrote to DB");
		Ok(())
	}

	pub async fn store_prepared(
		&self,
		task: &CheckEmailTask,
		success: &PreparedCheckEmailSuccess,
		extra: Option<serde_json::Value>,
	) -> Result<(), StorageError> {
		let payload_json = serde_json::to_value(task)?;
		let tenant_id = parse_tenant_id(task);
		let task_db_id = task.metadata.as_ref().and_then(|m| m.task_db_id);
		let columns = success_columns_from_prepared(&success.response)?;

		self.store_success(task, &payload_json, tenant_id, task_db_id, columns, extra)
			.await?;

		debug!(target: LOG_TARGET, email=?task.input.to_email, "Wrote prepared result to DB");
		Ok(())
	}

	pub async fn store_error_only(
		&self,
		task: &CheckEmailTask,
		err: &TaskError,
		extra: Option<serde_json::Value>,
	) -> Result<(), StorageError> {
		let payload_json = serde_json::to_value(task)?;
		let tenant_id = parse_tenant_id(task);
		let task_db_id = task.metadata.as_ref().and_then(|m| m.task_db_id);
		self.store_error(task, &payload_json, tenant_id, task_db_id, err, extra)
			.await
	}

	async fn store_success(
		&self,
		task: &CheckEmailTask,
		payload_json: &serde_json::Value,
		tenant_id: Option<Uuid>,
		task_db_id: Option<i32>,
		columns: SuccessColumns,
		extra: Option<serde_json::Value>,
	) -> Result<(), StorageError> {
		if let Some(db_id) = task_db_id {
			let upd = sqlx::query(
				r#"
				UPDATE v1_task_result
				SET payload = $1,
				    extra = COALESCE(extra, '{}'::jsonb) || COALESCE($2, '{}'::jsonb),
				    result = $3,
				    error = NULL,
				    tenant_id = $4,
				    score = $5,
				    score_category = $6,
				    sub_reason = $7,
				    safe_to_send = $8,
				    reason_codes = $9,
				    canonical_email = COALESCE($10, canonical_email),
				    bounce_risk_score = $11,
				    bounce_risk_category = $12,
				    bounce_risk_confidence = $13,
				    bounce_risk_action = $14,
				    bounce_risk_model_version = $15,
				    bounce_risk_signals = $16,
				    updated_at = NOW()
				WHERE id = $17
				"#,
			)
			.bind(payload_json)
			.bind(&extra)
			.bind(&columns.result_json)
			.bind(tenant_id)
			.bind(columns.score)
			.bind(&columns.score_category)
			.bind(&columns.sub_reason)
			.bind(columns.safe_to_send)
			.bind(&columns.reason_codes)
			.bind(&columns.canonical_email)
			.bind(columns.bounce_risk_score)
			.bind(&columns.bounce_risk_category)
			.bind(columns.bounce_risk_confidence)
			.bind(&columns.bounce_risk_action)
			.bind(&columns.bounce_risk_model_version)
			.bind(&columns.bounce_risk_signals)
			.bind(db_id)
			.execute(&self.pg_pool)
			.await?;
			if upd.rows_affected() == 0 {
				self.insert_success(task, payload_json, tenant_id, columns, extra)
					.await?;
			}
		} else {
			self.insert_success(task, payload_json, tenant_id, columns, extra)
				.await?;
		}

		Ok(())
	}

	async fn insert_success(
		&self,
		task: &CheckEmailTask,
		payload_json: &serde_json::Value,
		tenant_id: Option<Uuid>,
		columns: SuccessColumns,
		extra: Option<serde_json::Value>,
	) -> Result<(), StorageError> {
		sqlx::query(
			r#"
			INSERT INTO v1_task_result (
				payload, job_id, extra, result, tenant_id,
				score, score_category, sub_reason, safe_to_send, reason_codes,
				canonical_email,
				bounce_risk_score, bounce_risk_category, bounce_risk_confidence,
				bounce_risk_action, bounce_risk_model_version, bounce_risk_signals
			)
			VALUES (
				$1, $2, $3, $4, $5,
				$6, $7, $8, $9, $10,
				$11,
				$12, $13, $14,
				$15, $16, $17
			)
			RETURNING id
			"#,
		)
		.bind(payload_json)
		.bind(match task.job_id {
			CheckEmailJobId::Bulk(job_id) => Some(job_id),
			CheckEmailJobId::SingleShot => None,
		})
		.bind(&extra)
		.bind(&columns.result_json)
		.bind(tenant_id)
		.bind(columns.score)
		.bind(&columns.score_category)
		.bind(&columns.sub_reason)
		.bind(columns.safe_to_send)
		.bind(&columns.reason_codes)
		.bind(&columns.canonical_email)
		.bind(columns.bounce_risk_score)
		.bind(&columns.bounce_risk_category)
		.bind(columns.bounce_risk_confidence)
		.bind(&columns.bounce_risk_action)
		.bind(&columns.bounce_risk_model_version)
		.bind(&columns.bounce_risk_signals)
		.fetch_one(&self.pg_pool)
		.await?;

		Ok(())
	}

	async fn store_error(
		&self,
		task: &CheckEmailTask,
		payload_json: &serde_json::Value,
		tenant_id: Option<Uuid>,
		task_db_id: Option<i32>,
		err: &TaskError,
		extra: Option<serde_json::Value>,
	) -> Result<(), StorageError> {
		let canonical_email = canonicalize_email(&task.input.to_email);

		if let Some(db_id) = task_db_id {
			let upd = sqlx::query(
				r#"
				UPDATE v1_task_result
				SET payload = $1,
				    extra = COALESCE(extra, '{}'::jsonb) || COALESCE($2, '{}'::jsonb),
				    error = $3,
				    tenant_id = $4,
				    canonical_email = COALESCE($5, canonical_email),
				    score = NULL,
				    score_category = NULL,
				    sub_reason = NULL,
				    safe_to_send = NULL,
				    reason_codes = NULL,
				    bounce_risk_score = NULL,
				    bounce_risk_category = NULL,
				    bounce_risk_confidence = NULL,
				    bounce_risk_action = NULL,
				    bounce_risk_model_version = NULL,
				    bounce_risk_signals = NULL
				WHERE id = $6
				"#,
			)
			.bind(payload_json)
			.bind(&extra)
			.bind(err.to_string())
			.bind(tenant_id)
			.bind(&canonical_email)
			.bind(db_id)
			.execute(&self.pg_pool)
			.await?;
			if upd.rows_affected() == 0 {
				self.insert_error(task, payload_json, tenant_id, err, extra, canonical_email)
					.await?;
			}
		} else {
			self.insert_error(task, payload_json, tenant_id, err, extra, canonical_email)
				.await?;
		}

		Ok(())
	}

	async fn insert_error(
		&self,
		task: &CheckEmailTask,
		payload_json: &serde_json::Value,
		tenant_id: Option<Uuid>,
		err: &TaskError,
		extra: Option<serde_json::Value>,
		canonical_email: Option<String>,
	) -> Result<(), StorageError> {
		sqlx::query(
			r#"
			INSERT INTO v1_task_result (payload, job_id, extra, error, tenant_id, canonical_email)
			VALUES ($1, $2, $3, $4, $5, $6)
			RETURNING id
			"#,
		)
		.bind(payload_json)
		.bind(match task.job_id {
			CheckEmailJobId::Bulk(job_id) => Some(job_id),
			CheckEmailJobId::SingleShot => None,
		})
		.bind(extra)
		.bind(err.to_string())
		.bind(tenant_id)
		.bind(canonical_email)
		.fetch_one(&self.pg_pool)
		.await?;

		Ok(())
	}

	pub fn get_extra(&self) -> Option<serde_json::Value> {
		self.extra.clone()
	}
}

fn parse_tenant_id(task: &CheckEmailTask) -> Option<Uuid> {
	task.metadata
		.as_ref()
		.and_then(|m| m.tenant_id.as_ref())
		.and_then(|id| id.parse().ok())
}

fn success_columns_from_output(
	output: &CheckEmailOutput,
) -> Result<SuccessColumns, serde_json::Error> {
	let email_score = compute_score(output);
	Ok(SuccessColumns {
		result_json: scored_json(output)?,
		score: i32::from(email_score.score),
		score_category: serde_json::to_string(&email_score.category)?
			.trim_matches('"')
			.to_string(),
		sub_reason: serde_json::to_string(&email_score.sub_reason)?
			.trim_matches('"')
			.to_string(),
		safe_to_send: email_score.safe_to_send,
		reason_codes: email_score.reason_codes,
		canonical_email: canonicalize_email(&output.input),
		bounce_risk_score: None,
		bounce_risk_category: None,
		bounce_risk_confidence: None,
		bounce_risk_action: None,
		bounce_risk_model_version: None,
		bounce_risk_signals: None,
	})
}

fn success_columns_from_prepared(
	response: &PreparedVerificationResponse,
) -> Result<SuccessColumns, serde_json::Error> {
	Ok(SuccessColumns {
		result_json: response.json.clone(),
		score: i32::from(response.score.score),
		score_category: serde_json::to_string(&response.score.category)?
			.trim_matches('"')
			.to_string(),
		sub_reason: serde_json::to_string(&response.score.sub_reason)?
			.trim_matches('"')
			.to_string(),
		safe_to_send: response.score.safe_to_send,
		reason_codes: response.score.reason_codes.clone(),
		canonical_email: response.canonical_email.clone(),
		bounce_risk_score: response
			.bounce_risk
			.as_ref()
			.map(|risk| i32::from(risk.score)),
		bounce_risk_category: response
			.bounce_risk
			.as_ref()
			.map(|risk| serde_json::to_string(&risk.category))
			.transpose()?
			.map(|value| value.trim_matches('"').to_string()),
		bounce_risk_confidence: response.bounce_risk.as_ref().map(|risk| risk.confidence),
		bounce_risk_action: response
			.bounce_risk
			.as_ref()
			.map(|risk| serde_json::to_string(&risk.action))
			.transpose()?
			.map(|value| value.trim_matches('"').to_string()),
		bounce_risk_model_version: response
			.bounce_risk
			.as_ref()
			.map(|risk| risk.model_version.clone()),
		bounce_risk_signals: response.bounce_risk_signals.clone(),
	})
}
