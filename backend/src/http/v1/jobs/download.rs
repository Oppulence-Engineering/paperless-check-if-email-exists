use crate::config::BackendConfig;
use crate::http::csv_shared::{csv_rows, ndjson_rows, TaskResultRecord, CSV_HEADER};
use crate::http::v1::bulk::with_worker_db;
use crate::http::{resolve_tenant, ReacherResponseError};
use crate::tenant::context::TenantContext;
use bytes::Bytes;
use check_if_email_exists::LOG_TARGET;
use futures::stream;
use serde::Deserialize;
use sqlx::{PgPool, Row};
use std::io;
use std::sync::Arc;
use warp::http::{Response, StatusCode};
use warp::hyper::Body;
use warp::Filter;

const BATCH_SIZE: i64 = 500;

#[derive(Debug, Clone, Copy)]
enum ResponseFormat {
	Csv,
	Json,
}

#[derive(Debug, Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
struct Query {
	/// Supported values: `csv`, `json`
	format: Option<String>,
}

#[derive(Clone)]
struct DownloadState {
	pg_pool: PgPool,
	job_id: i32,
	last_id: i64,
	format: ResponseFormat,
	header_sent: bool,
}

async fn http_handler(
	job_id: i32,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	query: Query,
) -> Result<impl warp::Reply, warp::Rejection> {
	let job_exists = sqlx::query(
		"SELECT total_records FROM v1_bulk_job WHERE id = $1 AND (tenant_id = $2 OR $2 IS NULL)",
	)
	.bind(job_id)
	.bind(tenant_ctx.tenant_id)
	.fetch_optional(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	if job_exists.is_none() {
		return Err(ReacherResponseError::new(StatusCode::NOT_FOUND, "Job not found").into());
	}

	let active_tasks: i64 = sqlx::query_scalar::<_, i64>(
		"SELECT COUNT(*) FROM v1_task_result WHERE job_id = $1 AND task_state IN ('queued', 'running', 'retrying')",
	)
	.bind(job_id)
	.fetch_one(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	if active_tasks > 0 {
		return Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			format!("Job {} is still running, please try again later", job_id),
		)
		.into());
	}

	let format = parse_response_format(query.format.as_deref())?;
	let state = DownloadState {
		pg_pool,
		job_id,
		last_id: 0,
		format,
		header_sent: false,
	};

	let content_type = match format {
		ResponseFormat::Csv => "text/csv",
		ResponseFormat::Json => "application/x-ndjson",
	};
	let file_extension = match format {
		ResponseFormat::Csv => "csv",
		ResponseFormat::Json => "ndjson",
	};

	let body = Body::wrap_stream(stream::unfold(state, |mut state| async move {
		if matches!(state.format, ResponseFormat::Csv) && !state.header_sent {
			state.header_sent = true;
			return Some((Ok::<Bytes, io::Error>(Bytes::from(CSV_HEADER)), state));
		}

		match fetch_batch(&state.pg_pool, state.job_id, state.last_id).await {
			Ok(records) if records.is_empty() => None,
			Ok(records) => {
				state.last_id = records
					.last()
					.map(|record| record.id)
					.unwrap_or(state.last_id);

				let chunk = match state.format {
					ResponseFormat::Csv => csv_rows(&records)
						.map(Bytes::from)
						.map_err(|err| io::Error::other(err.to_string())),
					ResponseFormat::Json => ndjson_rows(&records)
						.map(Bytes::from)
						.map_err(|err| io::Error::other(err.to_string())),
				};
				Some((chunk, state))
			}
			Err(err) => Some((Err(io::Error::other(err.to_string())), state)),
		}
	}));

	let response = Response::builder()
		.header("Content-Type", content_type)
		.header(
			"Content-Disposition",
			format!(
				"attachment; filename=\"job_{}_results.{}\"",
				job_id, file_extension
			),
		)
		.body(body)
		.map_err(|err| ReacherResponseError::new(StatusCode::INTERNAL_SERVER_ERROR, err))?;

	Ok(response)
}

fn parse_response_format(format: Option<&str>) -> Result<ResponseFormat, warp::Rejection> {
	match format.unwrap_or("csv").to_ascii_lowercase().as_str() {
		"csv" => Ok(ResponseFormat::Csv),
		"json" => Ok(ResponseFormat::Json),
		_ => Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"Invalid format. Expected one of: csv, json",
		)
		.into()),
	}
}

async fn fetch_batch(
	pg_pool: &PgPool,
	job_id: i32,
	last_id: i64,
) -> Result<Vec<TaskResultRecord>, ReacherResponseError> {
	let rows = sqlx::query(
		r#"
		SELECT id, payload, result, error, score, score_category, sub_reason, safe_to_send, reason_codes
		FROM v1_task_result
		WHERE job_id = $1 AND id > $2
		ORDER BY id ASC
		LIMIT $3
		"#,
	)
	.bind(job_id)
	.bind(last_id as i32)
	.bind(BATCH_SIZE)
	.fetch_all(pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	Ok(rows
		.into_iter()
		.map(|row| TaskResultRecord {
			id: row.get::<i32, _>("id") as i64,
			payload: row.get("payload"),
			result: row.get("result"),
			error: row.get("error"),
			score: row.get::<Option<i16>, _>("score"),
			score_category: row.get("score_category"),
			sub_reason: row.get("sub_reason"),
			safe_to_send: row.get("safe_to_send"),
			reason_codes: row.get("reason_codes"),
		})
		.collect())
}

/// GET /v1/jobs/{job_id}/download
///
/// Streams all terminal task results for a tenant-scoped bulk job.
#[utoipa::path(
	get,
	path = "/v1/jobs/{job_id}/download",
	tag = "Jobs",
	params(("job_id" = i32, Path, description = "Bulk job identifier"), Query),
	responses((status = 200, description = "Job result download stream"))
)]
pub fn v1_download_job_results(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "jobs" / i32 / "download")
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::query::<Query>())
		.and_then(http_handler)
		.with(warp::log(LOG_TARGET))
}
