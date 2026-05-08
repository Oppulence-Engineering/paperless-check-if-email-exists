use super::get_detail::list_summary;
use crate::config::BackendConfig;
use crate::finder::require_tenant_id;
use crate::http::csv_shared::{csv_row, TaskResultRecord};
use crate::http::v1::bulk::with_worker_db;
use crate::http::{check_scope, resolve_tenant, ReacherResponseError};
use crate::list_intelligence::{
	classify_change, evaluate_policy, load_effective_policy_rules_for_list, segment_matches,
	PolicyDecision, PolicyEvaluationInput, SegmentRowContext, VerificationSnapshot,
};
use crate::tenant::context::{scope, TenantContext};
use bytes::Bytes;
use check_if_email_exists::LOG_TARGET;
use chrono::{DateTime, Utc};
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
	segment_id: Option<i64>,
	changed_since_list_id: Option<i32>,
}

struct DownloadState {
	pg_pool: PgPool,
	list_id: i32,
	headers: Vec<String>,
	original_rows: BTreeMap<i32, Map<String, Value>>,
	last_row_index: i32,
	filter: Option<String>,
	segment_filter: Option<Value>,
	policy_rules: Value,
	comparison_baseline: BTreeMap<String, VerificationSnapshot>,
	header_sent: bool,
}

struct DownloadTaskRow {
	row_index: i32,
	comparison_key: Option<String>,
	record: TaskResultRecord,
}

async fn http_handler(
	list_id: i32,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	query: Query,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::LISTS)?;

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

	ensure_list_completed(&pg_pool, list_id).await?;

	let segment_filter = match query.segment_id {
		Some(segment_id) => {
			let filter = crate::http::v1::saved_segments::load_segment_filter(
				&pg_pool, tenant_id, segment_id,
			)
			.await
			.map_err(ReacherResponseError::from)?;
			Some(filter.ok_or_else(|| {
				warp::reject::custom(ReacherResponseError::new(
					StatusCode::NOT_FOUND,
					"Saved segment not found",
				))
			})?)
		}
		None => None,
	};

	let comparison_baseline = match query.changed_since_list_id {
		Some(base_list_id) => {
			ensure_owned_list_completed(&pg_pool, tenant_id, base_list_id).await?;
			load_comparison_baseline(&pg_pool, base_list_id).await?
		}
		None => BTreeMap::new(),
	};
	let policy_rules = load_effective_policy_rules_for_list(&pg_pool, tenant_id, list_id)
		.await
		.map_err(ReacherResponseError::from)?;

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
			segment_filter,
			policy_rules,
			comparison_baseline,
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
							.map(|row| row.row_index)
							.unwrap_or(state.last_row_index);

						let mut chunk = Vec::new();
						for row in rows {
							let row_index = row.row_index;
							let task_record = row.record;
							let flat = csv_row(&task_record);
							let previous = row
								.comparison_key
								.as_ref()
								.and_then(|key| state.comparison_baseline.get(key));
							let current =
								snapshot_from_record(&task_record, row.comparison_key.clone());
							let change_type = current
								.as_ref()
								.map(|current| classify_change(previous, current));

							if state.filter.as_deref() == Some("newly_invalid")
								&& change_type != Some("became_invalid")
							{
								continue;
							}
							if !state.comparison_baseline.is_empty()
								&& change_type == Some("unchanged")
							{
								continue;
							}
							if let Some(filter) = state.filter.as_deref() {
								if filter != "newly_invalid"
									&& flat.category.as_deref() != Some(filter)
								{
									continue;
								}
							}

							let reason_codes = task_record.reason_codes.clone().unwrap_or_default();
							let policy_decision = evaluate_policy(
								&state.policy_rules,
								&PolicyEvaluationInput {
									score: flat.score,
									category: flat.category.as_deref(),
									safe_to_send: flat.safe_to_send,
									reason_codes: &reason_codes,
								},
							)
							.unwrap_or(PolicyDecision::Review);
							if let Some(segment_filter) = &state.segment_filter {
								let ctx = SegmentRowContext {
									score: flat.score,
									category: flat.category.as_deref(),
									safe_to_send: flat.safe_to_send,
									reason_codes: &reason_codes,
									is_role_account: flat.is_role_account,
									is_disposable: flat.is_disposable,
									is_catch_all: flat.smtp_is_catch_all,
									age_days: flat.age_days,
									change_type,
									policy_decision: Some(policy_decision),
								};
								if !segment_matches(segment_filter, &ctx) {
									continue;
								}
							}

							let original =
								state.original_rows.remove(&row_index).unwrap_or_default();
							chunk.extend_from_slice(&render_row(
								&state.headers,
								&original,
								&flat,
								Some(policy_decision.as_str()),
								change_type,
							));
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
) -> Result<Vec<DownloadTaskRow>, ReacherResponseError> {
	let rows = sqlx::query(
		r#"
		SELECT
			id,
			(extra->>'row_index')::INTEGER AS row_index,
			COALESCE(canonical_email, LOWER(COALESCE(result->>'input', payload->'input'->>'to_email'))) AS comparison_key,
			payload,
			result,
			error,
			score,
			score_category,
			sub_reason,
			safe_to_send,
			reason_codes,
			completed_at
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
			DownloadTaskRow {
				row_index,
				comparison_key: row.get("comparison_key"),
				record: TaskResultRecord {
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
				},
			}
		})
		.collect())
}

fn render_row(
	headers: &[String],
	original: &Map<String, Value>,
	flat: &crate::http::csv_shared::CsvDownloadRow,
	policy_decision: Option<&str>,
	change_type: Option<&str>,
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
	row.push(flat.reason_codes.clone().unwrap_or_default());
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
	row.push(flat.verified_at.clone().unwrap_or_default());
	row.push(flat.age_days.map(|v| v.to_string()).unwrap_or_default());
	row.push(flat.freshness.clone().unwrap_or_default());
	row.push(policy_decision.unwrap_or_default().to_string());
	row.push(change_type.unwrap_or_default().to_string());
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
		"reason_codes".to_string(),
		"is_disposable".to_string(),
		"smtp_is_deliverable".to_string(),
		"error".to_string(),
		"verified_at".to_string(),
		"age_days".to_string(),
		"freshness".to_string(),
		"policy_decision".to_string(),
		"change_type".to_string(),
	]);
	writer.write_record(&row).expect("csv header write");
	writer.into_inner().expect("csv header bytes")
}

async fn ensure_list_completed(pg_pool: &PgPool, list_id: i32) -> Result<(), warp::Rejection> {
	let summary = list_summary(pg_pool, list_id)
		.await
		.map_err(warp::reject::custom)?;
	let total_rows: i32 = sqlx::query_scalar("SELECT total_rows FROM v1_lists WHERE id = $1")
		.bind(list_id)
		.fetch_one(pg_pool)
		.await
		.map_err(ReacherResponseError::from)
		.map_err(warp::reject::custom)?;
	if summary.total_processed < i64::from(total_rows) {
		return Err(
			ReacherResponseError::new(StatusCode::BAD_REQUEST, "List is still processing").into(),
		);
	}
	Ok(())
}

async fn ensure_owned_list_completed(
	pg_pool: &PgPool,
	tenant_id: uuid::Uuid,
	list_id: i32,
) -> Result<(), warp::Rejection> {
	let exists: bool = sqlx::query_scalar(
		"SELECT EXISTS(SELECT 1 FROM v1_lists WHERE id = $1 AND tenant_id = $2 AND status <> 'deleted'::list_status)",
	)
	.bind(list_id)
	.bind(tenant_id)
	.fetch_one(pg_pool)
	.await
	.map_err(ReacherResponseError::from)
	.map_err(warp::reject::custom)?;
	if !exists {
		return Err(ReacherResponseError::new(StatusCode::NOT_FOUND, "List not found").into());
	}
	ensure_list_completed(pg_pool, list_id).await
}

async fn load_comparison_baseline(
	pg_pool: &PgPool,
	list_id: i32,
) -> Result<BTreeMap<String, VerificationSnapshot>, warp::Rejection> {
	let rows = sqlx::query(
		r#"
		SELECT DISTINCT ON (comparison_key)
			id,
			tenant_id,
			comparison_key,
			(extra->>'list_id')::INTEGER AS list_id,
			(extra->>'pipeline_run_id')::BIGINT AS pipeline_run_id,
			score,
			score_category,
			safe_to_send,
			reason_codes,
			completed_at
		FROM (
			SELECT *,
			       COALESCE(canonical_email, LOWER(COALESCE(result->>'input', payload->'input'->>'to_email'))) AS comparison_key
			FROM v1_task_result
			WHERE (extra->>'list_id')::INTEGER = $1
			  AND task_state = 'completed'::task_state
			  AND result IS NOT NULL
			  AND completed_at IS NOT NULL
		) rows
		WHERE comparison_key IS NOT NULL AND comparison_key <> ''
		ORDER BY comparison_key, is_duplicate ASC, id ASC
		"#,
	)
	.bind(list_id)
	.fetch_all(pg_pool)
	.await
	.map_err(ReacherResponseError::from)
	.map_err(warp::reject::custom)?;

	let mut snapshots = BTreeMap::new();
	for row in rows {
		let key: String = row.get("comparison_key");
		snapshots.insert(
			key.clone(),
			VerificationSnapshot {
				task_id: row.get("id"),
				tenant_id: row.get("tenant_id"),
				canonical_email: key,
				list_id: row.get("list_id"),
				pipeline_run_id: row.get("pipeline_run_id"),
				score: row.get("score"),
				category: row.get("score_category"),
				safe_to_send: row.get("safe_to_send"),
				reason_codes: row.get("reason_codes"),
				completed_at: row.get("completed_at"),
			},
		);
	}
	Ok(snapshots)
}

fn snapshot_from_record(
	record: &TaskResultRecord,
	comparison_key: Option<String>,
) -> Option<VerificationSnapshot> {
	let comparison_key = comparison_key.filter(|key| !key.is_empty())?;
	Some(VerificationSnapshot {
		task_id: record.id as i32,
		tenant_id: uuid::Uuid::nil(),
		canonical_email: comparison_key,
		list_id: None,
		pipeline_run_id: None,
		score: record.score,
		category: record.score_category.clone(),
		safe_to_send: record.safe_to_send,
		reason_codes: record.reason_codes.clone(),
		completed_at: record.completed_at.unwrap_or_else(Utc::now),
	})
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
