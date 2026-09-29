use crate::config::BackendConfig;
use crate::http::v1::bulk::with_worker_db;
use crate::http::{resolve_tenant, ReacherResponseError};
use crate::tenant::context::TenantContext;
use bytes::Bytes;
use check_if_email_exists::LOG_TARGET;
use chrono::{DateTime, Utc};
use futures::stream;
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::{PgPool, Row};
use std::io;
use std::sync::Arc;
use warp::http::{Response, StatusCode};
use warp::hyper::Body;
use warp::Filter;

const BATCH_SIZE: i64 = 500;
const CSV_HEADER: &str =
	"task_id,row_index,input,task_state,error,retry_count,updated_at,payload,extra\n";

#[derive(Debug, Clone, Copy)]
enum ReportFormat {
	Csv,
	Json,
}

#[derive(Debug, Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
struct Query {
	format: Option<String>,
}

#[derive(Clone)]
struct ReportState {
	pg_pool: PgPool,
	job_id: i32,
	last_id: i32,
	format: ReportFormat,
	header_sent: bool,
}

struct FailureReportRow {
	id: i32,
	row_index: Option<i32>,
	input: Option<String>,
	task_state: String,
	error: Option<String>,
	retry_count: i32,
	updated_at: DateTime<Utc>,
	payload: Value,
	extra: Value,
}

async fn http_handler(
	job_id: i32,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	query: Query,
) -> Result<impl warp::Reply, warp::Rejection> {
	let job_exists =
		sqlx::query("SELECT id FROM v1_bulk_job WHERE id = $1 AND (tenant_id = $2 OR $2 IS NULL)")
			.bind(job_id)
			.bind(tenant_ctx.tenant_id)
			.fetch_optional(&pg_pool)
			.await
			.map_err(ReacherResponseError::from)?;
	if job_exists.is_none() {
		return Err(ReacherResponseError::new(StatusCode::NOT_FOUND, "Job not found").into());
	}

	let format = parse_report_format(query.format.as_deref())?;
	let state = ReportState {
		pg_pool,
		job_id,
		last_id: 0,
		format,
		header_sent: false,
	};

	let body = Body::wrap_stream(stream::unfold(state, |mut state| async move {
		if matches!(state.format, ReportFormat::Csv) && !state.header_sent {
			state.header_sent = true;
			return Some((Ok::<Bytes, io::Error>(Bytes::from(CSV_HEADER)), state));
		}

		match fetch_batch(&state.pg_pool, state.job_id, state.last_id).await {
			Ok(rows) if rows.is_empty() => None,
			Ok(rows) => {
				state.last_id = rows.last().map(|row| row.id).unwrap_or(state.last_id);
				let chunk = match state.format {
					ReportFormat::Csv => render_csv_rows(&rows)
						.map(Bytes::from)
						.map_err(|err| io::Error::other(err.to_string())),
					ReportFormat::Json => render_ndjson_rows(&rows)
						.map(Bytes::from)
						.map_err(|err| io::Error::other(err.to_string())),
				};
				Some((chunk, state))
			}
			Err(err) => Some((Err(io::Error::other(err.to_string())), state)),
		}
	}));

	let (content_type, extension) = match format {
		ReportFormat::Csv => ("text/csv", "csv"),
		ReportFormat::Json => ("application/x-ndjson", "ndjson"),
	};
	let response = Response::builder()
		.header("Content-Type", content_type)
		.header(
			"Content-Disposition",
			format!(
				"attachment; filename=\"job_{}_failure_report.{}\"",
				job_id, extension
			),
		)
		.body(body)
		.map_err(|err| ReacherResponseError::new(StatusCode::INTERNAL_SERVER_ERROR, err))?;

	Ok(response)
}

fn parse_report_format(format: Option<&str>) -> Result<ReportFormat, warp::Rejection> {
	match format.unwrap_or("csv").to_ascii_lowercase().as_str() {
		"csv" => Ok(ReportFormat::Csv),
		"json" => Ok(ReportFormat::Json),
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
	last_id: i32,
) -> Result<Vec<FailureReportRow>, ReacherResponseError> {
	let rows = sqlx::query(
		r#"
		SELECT id, extra, payload, task_state::TEXT AS task_state, error, retry_count, updated_at
		FROM v1_task_result
		WHERE job_id = $1
		  AND id > $2
		  AND task_state IN ('failed', 'dead_lettered', 'cancelled')
		ORDER BY id ASC
		LIMIT $3
		"#,
	)
	.bind(job_id)
	.bind(last_id)
	.bind(BATCH_SIZE)
	.fetch_all(pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	Ok(rows
		.into_iter()
		.map(|row| {
			let extra: Value = row.get("extra");
			let payload: Value = row.get("payload");
			FailureReportRow {
				id: row.get("id"),
				row_index: extra
					.get("row_index")
					.and_then(Value::as_i64)
					.map(|value| value as i32),
				input: payload_input(&payload),
				task_state: row.get("task_state"),
				error: row.get("error"),
				retry_count: row.get("retry_count"),
				updated_at: row.get("updated_at"),
				payload,
				extra,
			}
		})
		.collect())
}

fn render_csv_rows(rows: &[FailureReportRow]) -> Result<Vec<u8>, csv::Error> {
	let mut writer = csv::WriterBuilder::new()
		.has_headers(false)
		.from_writer(Vec::new());
	for row in rows {
		writer.write_record([
			row.id.to_string(),
			row.row_index
				.map(|value| value.to_string())
				.unwrap_or_default(),
			row.input.clone().unwrap_or_default(),
			row.task_state.clone(),
			row.error.clone().unwrap_or_default(),
			row.retry_count.to_string(),
			row.updated_at.to_rfc3339(),
			row.payload.to_string(),
			row.extra.to_string(),
		])?;
	}
	writer.into_inner().map_err(|err| err.into_error().into())
}

fn render_ndjson_rows(rows: &[FailureReportRow]) -> Result<Vec<u8>, serde_json::Error> {
	let mut bytes = Vec::new();
	for row in rows {
		let line = json!({
			"task_id": row.id,
			"row_index": row.row_index,
			"input": row.input,
			"task_state": row.task_state,
			"error": row.error,
			"retry_count": row.retry_count,
			"updated_at": row.updated_at,
			"payload": row.payload,
			"extra": row.extra
		});
		bytes.extend_from_slice(&serde_json::to_vec(&line)?);
		bytes.push(b'\n');
	}
	Ok(bytes)
}

fn payload_input(payload: &Value) -> Option<String> {
	payload
		.get("input")
		.and_then(|value| {
			value
				.get("to_email")
				.and_then(Value::as_str)
				.or_else(|| value.as_str())
		})
		.or_else(|| payload.get("to_email").and_then(Value::as_str))
		.map(ToOwned::to_owned)
}

/// GET /v1/jobs/{job_id}/failure-report
pub fn v1_job_failure_report(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "jobs" / i32 / "failure-report")
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::query::<Query>())
		.and_then(http_handler)
		.with(warp::log(LOG_TARGET))
}

#[cfg(test)]
mod tests {
	use super::*;
	use chrono::TimeZone;

	fn row() -> FailureReportRow {
		FailureReportRow {
			id: 42,
			row_index: Some(3),
			input: Some("user@example.com".to_string()),
			task_state: "failed".to_string(),
			error: Some("smtp_timeout".to_string()),
			retry_count: 2,
			updated_at: Utc.with_ymd_and_hms(2026, 6, 30, 12, 0, 0).unwrap(),
			payload: json!({"input": {"to_email": "user@example.com"}}),
			extra: json!({"row_index": 3}),
		}
	}

	#[test]
	fn parses_failure_report_format() {
		assert!(matches!(
			parse_report_format(Some("csv")).expect("csv format"),
			ReportFormat::Csv
		));
		assert!(parse_report_format(Some("xml")).is_err());
	}

	#[test]
	fn renders_failure_report_csv_row() {
		let bytes = render_csv_rows(&[row()]).expect("csv should render");
		let mut reader = csv::ReaderBuilder::new()
			.has_headers(false)
			.from_reader(bytes.as_slice());
		let record = reader
			.records()
			.next()
			.expect("record should exist")
			.expect("record should parse");

		assert_eq!(record.get(0), Some("42"));
		assert_eq!(record.get(2), Some("user@example.com"));
		assert_eq!(record.get(4), Some("smtp_timeout"));
	}

	#[test]
	fn renders_failure_report_ndjson_row() {
		let bytes = render_ndjson_rows(&[row()]).expect("json should render");
		let line: Value = serde_json::from_slice(bytes.split(|b| *b == b'\n').next().unwrap())
			.expect("json should parse");

		assert_eq!(line["task_id"], 42);
		assert_eq!(line["error"], "smtp_timeout");
	}
}
