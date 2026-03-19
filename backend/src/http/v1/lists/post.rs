use super::csv_parse::{parse_csv, ParsedCsv};
use crate::config::BackendConfig;
use crate::finder::require_tenant_id;
use crate::http::v0::check_email::post::with_config;
use crate::http::v1::bulk::post::publish_task;
use crate::http::v1::bulk::with_worker_db;
use crate::http::{resolve_tenant, CheckEmailRequest, ReacherResponseError};
use crate::scoring::response::scored_json;
use crate::tenant::context::TenantContext;
use crate::tenant::models::PlanTier;
use crate::tenant::quota::{check_and_increment_quota_for_count, QuotaCheckResult};
use crate::worker::do_work::{CheckEmailJobId, CheckEmailTask, TaskMetadata};
use bytes::Buf;
use check_if_email_exists::{CheckEmailOutput, Reachable, LOG_TARGET};
use futures::TryStreamExt;
use lapin::BasicProperties;
use serde::Serialize;
use serde_json::Value;
use sqlx::PgPool;
use std::sync::Arc;
use warp::http::StatusCode;
use warp::multipart::FormData;
use warp::Filter;

#[derive(Debug, Serialize)]
struct Response {
	list_id: i32,
	job_id: i32,
	total_rows: i32,
	email_column: String,
}

async fn http_handler(
	tenant_ctx: TenantContext,
	config: Arc<BackendConfig>,
	pg_pool: PgPool,
	form: FormData,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	let upload = read_upload(form).await.map_err(warp::reject::custom)?;
	let parsed = parse_csv(&upload.file_bytes, upload.email_column.as_deref())
		.map_err(warp::reject::custom)?;

	enforce_row_limit(&tenant_ctx, parsed.rows.len())?;

	let non_empty_email_count = parsed
		.rows
		.iter()
		.filter(|row| {
			row.get(&parsed.email_column)
				.and_then(Value::as_str)
				.map(|value| !value.trim().is_empty())
				.unwrap_or(false)
		})
		.count() as i32;

	match check_and_increment_quota_for_count(Some(&pg_pool), &tenant_ctx, non_empty_email_count)
		.await
	{
		QuotaCheckResult::Allowed => {}
		QuotaCheckResult::ExceededMonthlyLimit {
			limit,
			used,
			resets_at,
		} => {
			return Err(ReacherResponseError::new(
				StatusCode::TOO_MANY_REQUESTS,
				format!(
					"Monthly email limit of {} reached ({} used). Resets at {}",
					limit,
					used,
					resets_at.format("%Y-%m-%d %H:%M:%S UTC")
				),
			)
			.into())
		}
	}

	let job_id: i32 = sqlx::query_scalar(
		"INSERT INTO v1_bulk_job (total_records, tenant_id, status) VALUES ($1, $2, 'pending'::job_state) RETURNING id",
	)
	.bind(parsed.rows.len() as i32)
	.bind(tenant_id)
	.fetch_one(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let original_data = build_original_data(&parsed);
	let list_name = upload
		.name
		.clone()
		.unwrap_or_else(|| upload.filename.clone());
	let list_id: i32 = sqlx::query_scalar(
		r#"
		INSERT INTO v1_lists (
			tenant_id, job_id, name, original_filename, file_size_bytes, total_rows,
			email_column, original_headers, original_data, status
		)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, 'uploading'::list_status)
		RETURNING id
		"#,
	)
	.bind(tenant_id)
	.bind(job_id)
	.bind(&list_name)
	.bind(&upload.filename)
	.bind(upload.file_bytes.len() as i64)
	.bind(parsed.rows.len() as i32)
	.bind(&parsed.email_column)
	.bind(&parsed.headers)
	.bind(&original_data)
	.fetch_one(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let properties = BasicProperties::default()
		.with_content_type("application/json".into())
		.with_priority(1);

	let mut published_any = false;
	for (index, row) in parsed.rows.iter().enumerate() {
		let email = row
			.get(&parsed.email_column)
			.and_then(Value::as_str)
			.unwrap_or_default()
			.trim()
			.to_string();
		let extra = serde_json::json!({
			"list_id": list_id,
			"row_index": index as i32,
			"email_column": parsed.email_column,
		});

		if email.is_empty() {
			let invalid_result = blank_email_result();
			sqlx::query(
				r#"
				INSERT INTO v1_task_result (
					job_id, payload, extra, result, tenant_id, task_state, score, score_category, sub_reason, completed_at
				)
				VALUES ($1, $2, $3, $4, $5, 'completed', 0, 'invalid', 'invalid_syntax', NOW())
				"#,
			)
			.bind(job_id)
			.bind(serde_json::json!({
				"input": {"to_email": ""},
				"job_id": {"bulk": job_id},
				"webhook": null
			}))
			.bind(extra)
			.bind(invalid_result)
			.bind(tenant_id)
			.execute(&pg_pool)
			.await
			.map_err(ReacherResponseError::from)?;
			continue;
		}

		let task_row_id: i32 = sqlx::query_scalar(
			r#"
			INSERT INTO v1_task_result (job_id, payload, extra, task_state, tenant_id)
			VALUES ($1, $2, $3, 'queued', $4)
			RETURNING id
			"#,
		)
		.bind(job_id)
		.bind(serde_json::json!({
			"input": {"to_email": email},
			"job_id": {"bulk": job_id},
			"webhook": null
		}))
		.bind(extra)
		.bind(tenant_id)
		.fetch_one(&pg_pool)
		.await
		.map_err(ReacherResponseError::from)?;

		let task = CheckEmailTask {
			input: CheckEmailRequest {
				to_email: email.clone(),
				..Default::default()
			}
			.to_check_email_input(Arc::clone(&config)),
			job_id: CheckEmailJobId::Bulk(job_id),
			webhook: None,
			metadata: Some(TaskMetadata {
				tenant_id: Some(tenant_id.to_string()),
				request_id: None,
				correlation_id: None,
				created_by: Some("lists".to_string()),
				retry_policy: None,
				dedupe_key: Some(format!("list:{}:row:{}", list_id, index)),
				task_db_id: Some(task_row_id),
			}),
		};
		publish_task(
			config
				.must_worker_config()
				.map_err(ReacherResponseError::from)?
				.channel,
			task,
			properties.clone(),
		)
		.await?;
		published_any = true;
	}

	let job_status = if published_any {
		"running"
	} else {
		"completed"
	};
	let list_status = if published_any {
		"processing"
	} else {
		"completed"
	};
	sqlx::query("UPDATE v1_bulk_job SET status = $2::job_state, updated_at = NOW(), completed_at = CASE WHEN $2 = 'completed' THEN NOW() ELSE NULL END WHERE id = $1")
		.bind(job_id)
		.bind(job_status)
		.execute(&pg_pool)
		.await
		.map_err(ReacherResponseError::from)?;
	sqlx::query("UPDATE v1_lists SET status = $2::list_status, completed_at = CASE WHEN $2 = 'completed' THEN NOW() ELSE NULL END, updated_at = NOW() WHERE id = $1")
		.bind(list_id)
		.bind(list_status)
		.execute(&pg_pool)
		.await
		.map_err(ReacherResponseError::from)?;

	Ok(warp::reply::with_status(
		warp::reply::json(&Response {
			list_id,
			job_id,
			total_rows: parsed.rows.len() as i32,
			email_column: parsed.email_column,
		}),
		StatusCode::ACCEPTED,
	))
}

struct UploadData {
	file_bytes: Vec<u8>,
	filename: String,
	name: Option<String>,
	email_column: Option<String>,
}

async fn read_upload(mut form: FormData) -> Result<UploadData, ReacherResponseError> {
	let mut file_bytes = None;
	let mut filename = None;
	let mut name = None;
	let mut email_column = None;

	while let Some(part) = form
		.try_next()
		.await
		.map_err(|err| ReacherResponseError::new(StatusCode::BAD_REQUEST, err))?
	{
		let part_name = part.name().to_string();
		let part_filename = part.filename().map(ToOwned::to_owned);
		let collected = part
			.stream()
			.try_fold(Vec::new(), |mut bytes, mut chunk| async move {
				bytes.extend_from_slice(chunk.copy_to_bytes(chunk.remaining()).as_ref());
				Ok(bytes)
			})
			.await
			.map_err(|err| ReacherResponseError::new(StatusCode::BAD_REQUEST, err))?;

		match part_name.as_str() {
			"file" => {
				file_bytes = Some(collected);
				filename = Some(part_filename.unwrap_or_else(|| "upload.csv".to_string()));
			}
			"name" => name = Some(String::from_utf8_lossy(&collected).trim().to_string()),
			"email_column" => {
				email_column = Some(String::from_utf8_lossy(&collected).trim().to_string())
			}
			_ => {}
		}
	}

	Ok(UploadData {
		file_bytes: file_bytes.ok_or_else(|| {
			ReacherResponseError::new(
				StatusCode::BAD_REQUEST,
				"multipart field 'file' is required",
			)
		})?,
		filename: filename.unwrap_or_else(|| "upload.csv".to_string()),
		name,
		email_column,
	})
}

fn build_original_data(parsed: &ParsedCsv) -> serde_json::Value {
	let mut root = serde_json::Map::new();
	for (index, row) in parsed.rows.iter().enumerate() {
		root.insert(index.to_string(), Value::Object(row.clone()));
	}
	Value::Object(root)
}

fn blank_email_result() -> serde_json::Value {
	let output = CheckEmailOutput {
		input: "".to_string(),
		is_reachable: Reachable::Invalid,
		..Default::default()
	};
	scored_json(&output).unwrap_or_else(|_| {
		serde_json::json!({
			"input": "",
			"is_reachable": "invalid",
			"score": {"score": 0, "category": "invalid", "sub_reason": "invalid_syntax"}
		})
	})
}

fn enforce_row_limit(tenant_ctx: &TenantContext, rows: usize) -> Result<(), warp::Rejection> {
	let limit = match tenant_ctx.plan_tier {
		PlanTier::Free => 1_000usize,
		PlanTier::Starter => 10_000usize,
		PlanTier::Professional => 50_000usize,
		PlanTier::Enterprise => 100_000usize,
	};
	if rows > limit {
		return Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			format!("List contains {} rows but plan allows {}", rows, limit),
		)
		.into());
	}
	Ok(())
}

/// POST /v1/lists
#[utoipa::path(
	post,
	path = "/v1/lists",
	tag = "v1",
	responses((status = 202, description = "List upload accepted"))
)]
pub fn v1_create_list(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "lists")
		.and(warp::post())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_config(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::multipart::form().max_length(50_000_000))
		.and_then(http_handler)
		.with(warp::log(LOG_TARGET))
}
