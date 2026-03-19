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

//! This file implements the `GET /v1/bulk/{id}` endpoint.

use std::sync::Arc;

use check_if_email_exists::LOG_TARGET;
use serde::Serialize;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::PgPool;
use warp::http::StatusCode;
use warp::Filter;

use super::with_worker_db;
use crate::config::BackendConfig;
use crate::http::resolve_tenant;
use crate::http::ReacherResponseError;
use crate::tenant::context::TenantContext;

#[derive(Debug, Serialize, PartialEq, Eq)]
enum ValidStatus {
	Running,
	Completed,
}

#[derive(sqlx::FromRow, Debug, Serialize)]
struct JobRecord {
	id: i32,
	created_at: DateTime<Utc>,
	total_records: i32,
}

#[derive(Debug, Serialize)]
struct ResponseSummary {
	total_safe: i32,
	total_risky: i32,
	total_invalid: i32,
	total_unknown: i32,
}

#[derive(Debug, Serialize)]
struct Response {
	job_id: i32,
	created_at: DateTime<Utc>,
	finished_at: Option<DateTime<Utc>>,
	total_records: i32,
	total_processed: i32,
	summary: ResponseSummary,
	job_status: ValidStatus,
}

async fn http_handler(
	job_id: i32,
	tenant_ctx: TenantContext,
	conn_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	let job_rec = sqlx::query_as!(
		JobRecord,
		r#"
		SELECT id, created_at, total_records FROM v1_bulk_job
		WHERE id = $1 AND (tenant_id = $2 OR $2 IS NULL)
		LIMIT 1
		"#,
		job_id,
		tenant_ctx.tenant_id,
	)
	.fetch_one(&conn_pool)
	.await
	.map_err(|e| ReacherResponseError::new(StatusCode::BAD_REQUEST, e))?;

	let agg_info = sqlx::query!(
		r#"
		SELECT
			COUNT(*) as total_processed,
			COUNT(CASE WHEN result ->> 'is_reachable' LIKE 'safe' THEN 1 END) as safe_count,
			COUNT(CASE WHEN result ->> 'is_reachable' LIKE 'risky' THEN 1 END) as risky_count,
			COUNT(CASE WHEN result ->> 'is_reachable' LIKE 'invalid' THEN 1 END) as invalid_count,
			COUNT(CASE WHEN result ->> 'is_reachable' LIKE 'unknown' THEN 1 END) as unknown_count,
			(SELECT created_at FROM v1_task_result WHERE job_id = $1 ORDER BY created_at DESC LIMIT 1) as finished_at
		FROM v1_task_result
		WHERE job_id = $1 AND (result IS NOT NULL OR error IS NOT NULL)
		"#,
		job_id
	)
	.fetch_one(&conn_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let (job_status, finished_at) = if (agg_info
		.total_processed
		.expect("sql COUNT() returns an int. qed.") as i32)
		< job_rec.total_records
	{
		(ValidStatus::Running, None)
	} else {
		(
			ValidStatus::Completed,
			Some(
				agg_info
					.finished_at
					.expect("always at least one task in the job. qed."),
			),
		)
	};

	Ok(warp::reply::json(&Response {
		job_id: job_rec.id,
		created_at: job_rec.created_at,
		finished_at,
		total_records: job_rec.total_records,
		total_processed: agg_info
			.total_processed
			.expect("sql COUNT returns an int. qed.") as i32,
		summary: ResponseSummary {
			total_safe: agg_info.safe_count.expect("sql COUNT returns an int. qed.") as i32,
			total_risky: agg_info
				.risky_count
				.expect("sql COUNT returns an int. qed.") as i32,
			total_invalid: agg_info
				.invalid_count
				.expect("sql COUNT returns an int. qed.") as i32,
			total_unknown: agg_info
				.unknown_count
				.expect("sql COUNT returns an int. qed.") as i32,
		},
		job_status,
	}))
}

/// GET /v1/bulk/{job_id}
///
/// Returns status and current progress for a tenant-scoped v1 bulk job.
#[utoipa::path(
	get,
	path = "/v1/bulk/{job_id}",
	tag = "Jobs",
	params(("job_id" = i32, Path, description = "V1 bulk job identifier")),
	responses((status = 200, description = "Bulk job progress"))
)]
pub fn v1_get_bulk_job_progress(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "bulk" / i32)
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and_then(http_handler)
		.with(warp::log(LOG_TARGET))
}
