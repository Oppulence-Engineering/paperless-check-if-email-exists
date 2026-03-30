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

//! This file implements the /bulk/{id}/results endpoints.

use check_if_email_exists::LOG_TARGET;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use std::iter::Iterator;
use std::sync::Arc;
use warp::http::StatusCode;
use warp::Filter;

use super::with_worker_read_db;
use crate::config::BackendConfig;
use crate::http::csv_shared::{csv_rows, TaskResultRecord};
use crate::http::resolve_tenant;
use crate::http::ReacherResponseError;
use crate::tenant::context::TenantContext;
use chrono::{DateTime, Utc};

/// Defines the download format, passed in as a query param.
#[derive(Clone, Copy)]
enum ResponseFormat {
	Json,
	Csv,
}

// limit and offset are optional in the request
// If unspecified, offset will default to 0.
#[derive(Serialize, Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
struct Request {
	/// Supported values: `json`, `csv`
	format: Option<String>,
	limit: Option<u64>,
	offset: Option<u64>,
}

#[derive(Serialize, Deserialize)]
struct Response {
	results: Vec<serde_json::Value>,
}

async fn http_handler(
	job_id: i32,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	req: Request,
) -> Result<impl warp::Reply, warp::Rejection> {
	// Tenant-scoped job lookup
	let total_records = sqlx::query!(
		r#"SELECT total_records FROM v1_bulk_job WHERE id = $1 AND (tenant_id = $2 OR $2 IS NULL);"#,
		job_id,
		tenant_ctx.tenant_id,
	)
	.fetch_one(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?
	.total_records;
	// Count all terminal tasks (completed, failed, cancelled, dead_lettered) — not just those with result/error
	let total_processed = sqlx::query!(
		r#"SELECT COUNT(*) FROM v1_task_result WHERE job_id = $1 AND task_state NOT IN ('queued', 'running', 'retrying');"#,
		job_id
	)
	.fetch_one(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?
	.count
	.unwrap_or(0);

	if total_processed < total_records as i64 {
		return Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			format!("Job {} is still running, please try again later", job_id),
		)
		.into());
	}

	let format = parse_response_format(req.format.as_deref())?;
	match format {
		ResponseFormat::Json => {
			let data = job_result_json(job_id, req.limit, req.offset.unwrap_or(0), pg_pool).await?;

			let reply = serde_json::to_vec(&Response { results: data })
				.map_err(ReacherResponseError::from)?;

			Ok(warp::reply::with_header(
				reply,
				"Content-Type",
				"application/json",
			))
		}
		ResponseFormat::Csv => {
			let data = job_result_csv(job_id, req.limit, req.offset.unwrap_or(0), pg_pool).await?;

			Ok(warp::reply::with_header(data, "Content-Type", "text/csv"))
		}
	}
}

fn parse_response_format(format: Option<&str>) -> Result<ResponseFormat, warp::Rejection> {
	match format.unwrap_or("json").to_ascii_lowercase().as_str() {
		"json" => Ok(ResponseFormat::Json),
		"csv" => Ok(ResponseFormat::Csv),
		_ => Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"Invalid format. Expected one of: json, csv",
		)
		.into()),
	}
}

fn safe_i64(value: u64) -> i64 {
	value.min(i64::MAX as u64) as i64
}

async fn job_result_as_iter(
	job_id: i32,
	limit: Option<u64>,
	offset: u64,
	pg_pool: PgPool,
) -> Result<Box<dyn Iterator<Item = serde_json::Value>>, ReacherResponseError> {
	let rows = sqlx::query(
		r#"
		SELECT result, completed_at FROM v1_task_result
		WHERE job_id = $1
		ORDER BY id
		LIMIT $2 OFFSET $3
		"#,
	)
	.bind(job_id)
	.bind(limit.map(safe_i64))
	.bind(safe_i64(offset))
	.fetch_all(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	Ok(Box::new(rows.into_iter().map(|row| {
		let mut result: serde_json::Value = row.get("result");
		if let Some(completed_at) = row.get::<Option<DateTime<Utc>>, _>("completed_at") {
			crate::scoring::response::inject_freshness_into_result(&mut result, completed_at);
		}
		result
	})))
}

async fn job_result_json(
	job_id: i32,
	limit: Option<u64>,
	offset: u64,
	pg_pool: PgPool,
) -> Result<Vec<serde_json::Value>, warp::Rejection> {
	// For JSON responses, we don't want ot return more than 50 results at a
	// time, to avoid having a too big payload (unless client specifies a limit)

	Ok(
		job_result_as_iter(job_id, limit.or(Some(50)), offset, pg_pool)
			.await?
			.collect(),
	)
}

async fn job_result_csv(
	job_id: i32,
	limit: Option<u64>,
	offset: u64,
	pg_pool: PgPool,
) -> Result<Vec<u8>, warp::Rejection> {
	let rows = sqlx::query(
		r#"
		SELECT id, payload, result, error, score, score_category, sub_reason, safe_to_send, reason_codes, completed_at
		FROM v1_task_result
		WHERE job_id = $1
		ORDER BY id
		LIMIT $2 OFFSET $3
		"#,
	)
	.bind(job_id)
	.bind(limit.map(safe_i64))
	.bind(safe_i64(offset))
	.fetch_all(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let records: Vec<TaskResultRecord> = rows
		.into_iter()
		.map(|row| TaskResultRecord {
			id: row.get::<i32, _>("id") as i64,
			payload: row.get("payload"),
			result: row.get("result"),
			error: row.get("error"),
			score: row.get("score"),
			score_category: row.get("score_category"),
			sub_reason: row.get("sub_reason"),
			safe_to_send: row.get("safe_to_send"),
			reason_codes: row.get("reason_codes"),
			completed_at: row.get::<Option<DateTime<Utc>>, _>("completed_at"),
		})
		.collect();

	let mut data = crate::http::csv_shared::CSV_HEADER.as_bytes().to_vec();
	data.extend(csv_rows(&records).map_err(ReacherResponseError::from)?);
	Ok(data)
}

/// GET /v1/bulk/{job_id}/results
///
/// Returns terminal results for a tenant-scoped v1 bulk job.
#[utoipa::path(
	get,
	path = "/v1/bulk/{job_id}/results",
	tag = "Jobs",
	params(("job_id" = i32, Path, description = "V1 bulk job identifier"), Request),
	responses((status = 200, description = "Bulk job results"))
)]
pub fn v1_get_bulk_job_results(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "bulk" / i32 / "results")
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_read_db(config))
		.and(warp::query::<Request>())
		.and_then(http_handler)
		// View access logs by setting `RUST_LOG=reacher_backend`.
		.with(warp::log(LOG_TARGET))
}
