use super::get_detail::list_summary;
use crate::config::BackendConfig;
use crate::finder::require_tenant_id;
use crate::http::csv_shared::{csv_row, TaskResultRecord};
use crate::http::v1::bulk::with_worker_db;
use crate::http::{resolve_tenant, ReacherResponseError};
use crate::tenant::context::TenantContext;
use bytes::Bytes;
use check_if_email_exists::LOG_TARGET;
use futures::stream;
use serde::Deserialize;
use serde_json::{Map, Value};
use sqlx::{PgPool, Row};
use std::collections::BTreeMap;
use std::io;
use std::sync::Arc;
use warp::http::{Response, StatusCode};
use warp::hyper::Body;
use warp::Filter;

const BATCH_SIZE: i64 = 500;

#[derive(Debug, Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
struct Query {
	filter: Option<String>,
	format: Option<String>,
}

struct DownloadState {
	pg_pool: PgPool,
	list_id: i32,
	headers: Vec<String>,
	original_rows: BTreeMap<i32, Map<String, Value>>,
	last_row_index: i32,
	filter: Option<String>,
	header_sent: bool,
}

async fn http_handler(
	list_id: i32,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	query: Query,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	if query.format.as_deref().unwrap_or("csv") != "csv" {
		return Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"Only format=csv is supported",
		)
		.into());
	}

	let list = sqlx::query(
		r#"
		SELECT id, job_id, name, original_headers, original_data, status::TEXT AS status
		FROM v1_lists
		WHERE id = $1 AND tenant_id = $2
		"#,
	)
	.bind(list_id)
	.bind(tenant_id)
	.fetch_optional(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;
	let list = list.ok_or_else(|| {
		warp::reject::custom(ReacherResponseError::new(
			StatusCode::NOT_FOUND,
			"List not found",
		))
	})?;

	let summary = list_summary(&pg_pool, list_id)
		.await
		.map_err(warp::reject::custom)?;
	let total_rows: i32 = sqlx::query_scalar("SELECT total_rows FROM v1_lists WHERE id = $1")
		.bind(list_id)
		.fetch_one(&pg_pool)
		.await
		.map_err(ReacherResponseError::from)
		.map_err(warp::reject::custom)?;
	if summary.total_processed < i64::from(total_rows) {
		return Err(
			ReacherResponseError::new(StatusCode::BAD_REQUEST, "List is still processing").into(),
		);
	}

	let headers: Vec<String> = list.get("original_headers");
	let original_data: Value = list.get("original_data");
	let original_map = original_data.as_object().cloned().unwrap_or_default();
	let mut sorted_original: BTreeMap<i32, Map<String, Value>> = BTreeMap::new();
	for (key, value) in original_map {
		if let Ok(index) = key.parse::<i32>() {
			if let Some(object) = value.as_object() {
				sorted_original.insert(index, object.clone());
			}
		}
	}

	let body = Body::wrap_stream(stream::unfold(
		DownloadState {
			pg_pool,
			list_id,
			headers,
			original_rows: sorted_original,
			last_row_index: -1,
			filter: query.filter,
			header_sent: false,
		},
		|mut state| async move {
			if !state.header_sent {
				state.header_sent = true;
				return Some((
					Ok::<Bytes, io::Error>(Bytes::from(render_header(&state.headers))),
					state,
				));
			}

			loop {
				match fetch_batch(&state.pg_pool, state.list_id, state.last_row_index).await {
					Ok(rows) if rows.is_empty() => return None,
					Ok(rows) => {
						state.last_row_index = rows
							.last()
							.map(|(row_index, _)| *row_index)
							.unwrap_or(state.last_row_index);

						let mut chunk = Vec::new();
						for (row_index, task_record) in rows {
							let flat = csv_row(&task_record);
							if let Some(filter) = state.filter.as_deref() {
								if flat.category.as_deref() != Some(filter) {
									continue;
								}
							}

							let original =
								state.original_rows.remove(&row_index).unwrap_or_default();
							chunk.extend_from_slice(&render_row(&state.headers, &original, &flat));
						}

						if !chunk.is_empty() {
							return Some((Ok(Bytes::from(chunk)), state));
						}
					}
					Err(err) => {
						return Some((Err(io::Error::other(err.to_string())), state));
					}
				}
			}
		},
	));

	let response = Response::builder()
		.header("Content-Type", "text/csv")
		.header(
			"Content-Disposition",
			format!("attachment; filename=\"list_{}_cleaned.csv\"", list_id),
		)
		.body(body)
		.map_err(|err| ReacherResponseError::new(StatusCode::INTERNAL_SERVER_ERROR, err))
		.map_err(warp::reject::custom)?;

	Ok(response)
}

async fn fetch_batch(
	pg_pool: &PgPool,
	list_id: i32,
	last_row_index: i32,
) -> Result<Vec<(i32, TaskResultRecord)>, ReacherResponseError> {
	let rows = sqlx::query(
		r#"
		SELECT
			(extra->>'row_index')::INTEGER AS row_index,
			payload,
			result,
			error,
			score,
			score_category,
			sub_reason,
			safe_to_send
		FROM v1_task_result
		WHERE (extra->>'list_id')::INTEGER = $1
		  AND (extra->>'row_index')::INTEGER > $2
		ORDER BY (extra->>'row_index')::INTEGER ASC
		LIMIT $3
		"#,
	)
	.bind(list_id)
	.bind(last_row_index)
	.bind(BATCH_SIZE)
	.fetch_all(pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	Ok(rows
		.into_iter()
		.map(|row| {
			let row_index = row.get::<i32, _>("row_index");
			(
				row_index,
				TaskResultRecord {
					id: row_index as i64,
					payload: row.get("payload"),
					result: row.get("result"),
					error: row.get("error"),
					score: row.get("score"),
					score_category: row.get("score_category"),
					sub_reason: row.get("sub_reason"),
					safe_to_send: row.get("safe_to_send"),
				},
			)
		})
		.collect())
}

fn render_row(
	headers: &[String],
	original: &Map<String, Value>,
	flat: &crate::http::csv_shared::CsvDownloadRow,
) -> Vec<u8> {
	let mut writer = csv::WriterBuilder::new()
		.has_headers(false)
		.from_writer(Vec::new());
	let mut row = Vec::new();
	for header in headers {
		row.push(
			original
				.get(header)
				.and_then(Value::as_str)
				.unwrap_or_default()
				.to_string(),
		);
	}
	row.push(flat.is_reachable.clone());
	row.push(
		flat.score
			.map(|value| value.to_string())
			.unwrap_or_default(),
	);
	row.push(flat.category.clone().unwrap_or_default());
	row.push(
		flat.safe_to_send
			.map(|value| value.to_string())
			.unwrap_or_default(),
	);
	row.push(
		flat.is_disposable
			.map(|value| value.to_string())
			.unwrap_or_default(),
	);
	row.push(
		flat.smtp_is_deliverable
			.map(|value| value.to_string())
			.unwrap_or_default(),
	);
	row.push(flat.error.clone().unwrap_or_default());
	writer.write_record(&row).expect("csv row write");
	writer.into_inner().expect("csv row bytes")
}

fn render_header(headers: &[String]) -> Vec<u8> {
	let mut writer = csv::WriterBuilder::new()
		.has_headers(false)
		.from_writer(Vec::new());
	let mut row = headers.to_vec();
	row.extend([
		"is_reachable".to_string(),
		"score".to_string(),
		"category".to_string(),
		"safe_to_send".to_string(),
		"is_disposable".to_string(),
		"smtp_is_deliverable".to_string(),
		"error".to_string(),
	]);
	writer.write_record(&row).expect("csv header write");
	writer.into_inner().expect("csv header bytes")
}

/// GET /v1/lists/{list_id}/download
#[utoipa::path(
	get,
	path = "/v1/lists/{list_id}/download",
	tag = "v1",
	params(("list_id" = i32, Path, description = "List identifier"), Query),
	responses((status = 200, description = "Cleaned list CSV download"))
)]
pub fn v1_download_list(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "lists" / i32 / "download")
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::query::<Query>())
		.and_then(http_handler)
		.with(warp::log(LOG_TARGET))
}
