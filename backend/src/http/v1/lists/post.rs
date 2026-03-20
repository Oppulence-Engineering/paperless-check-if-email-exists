use super::canonicalize::canonicalize_email;
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
use std::collections::HashMap;
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

	// Phase 1: Build canonical groupings for deduplication
	// Maps canonical_email -> vec of row indices that share that canonical form
	let mut canonical_groups: HashMap<String, Vec<usize>> = HashMap::new();
	let mut blank_indices: Vec<usize> = Vec::new();

	for (index, row) in parsed.rows.iter().enumerate() {
		let email = row
			.get(&parsed.email_column)
			.and_then(Value::as_str)
			.unwrap_or_default()
			.trim()
			.to_string();

		if email.is_empty() {
			blank_indices.push(index);
			continue;
		}

		let canonical = canonicalize_email(&email).unwrap_or_else(|| email.to_lowercase());
		canonical_groups.entry(canonical).or_default().push(index);
	}

	let unique_email_count = canonical_groups.len() as i32;
	let deduplicated_count =
		(canonical_groups.values().map(|v| v.len()).sum::<usize>() - canonical_groups.len()) as i32;

	// Phase 2: Charge quota for unique emails only
	match check_and_increment_quota_for_count(Some(&pg_pool), &tenant_ctx, unique_email_count).await
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

	// Phase 3: Store dedup stats on v1_lists
	let list_id: i32 = sqlx::query_scalar(
		r#"
		INSERT INTO v1_lists (
			tenant_id, job_id, name, original_filename, file_size_bytes, total_rows,
			email_column, original_headers, original_data, status,
			unique_emails, deduplicated_count
		)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, 'uploading'::list_status, $10, $11)
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
	.bind(unique_email_count)
	.bind(deduplicated_count)
	.fetch_one(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let properties = BasicProperties::default()
		.with_content_type("application/json".into())
		.with_priority(1);

	// Phase 4: Process rows with deduplication
	let mut published_any = false;

	// Handle blank emails
	for &index in &blank_indices {
		let extra = serde_json::json!({
			"list_id": list_id,
			"row_index": index as i32,
			"email_column": parsed.email_column,
		});
		let invalid_result = blank_email_result();
		sqlx::query(
			r#"
			INSERT INTO v1_task_result (
				job_id, payload, extra, result, tenant_id, task_state,
				score, score_category, sub_reason, safe_to_send, reason_codes,
				completed_at, canonical_email, is_duplicate
			)
			VALUES ($1, $2, $3, $4, $5, 'completed', 0, 'invalid', 'invalid_syntax', false, ARRAY['invalid_syntax'], NOW(), NULL, false)
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
	}

	// Handle canonical groups: first occurrence is primary, rest are duplicates
	for (canonical, indices) in &canonical_groups {
		let primary_index = indices[0];
		let primary_email = parsed.rows[primary_index]
			.get(&parsed.email_column)
			.and_then(Value::as_str)
			.unwrap_or_default()
			.trim()
			.to_string();

		// Create primary task (queued, published to RabbitMQ)
		let primary_extra = serde_json::json!({
			"list_id": list_id,
			"row_index": primary_index as i32,
			"email_column": parsed.email_column,
		});
		let primary_task_id: i32 = sqlx::query_scalar(
			r#"
			INSERT INTO v1_task_result (job_id, payload, extra, task_state, tenant_id, canonical_email, is_duplicate)
			VALUES ($1, $2, $3, 'queued', $4, $5, false)
			RETURNING id
			"#,
		)
		.bind(job_id)
		.bind(serde_json::json!({
			"input": {"to_email": primary_email},
			"job_id": {"bulk": job_id},
			"webhook": null
		}))
		.bind(primary_extra)
		.bind(tenant_id)
		.bind(canonical)
		.fetch_one(&pg_pool)
		.await
		.map_err(ReacherResponseError::from)?;

		let task = CheckEmailTask {
			input: CheckEmailRequest {
				to_email: primary_email.clone(),
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
				dedupe_key: Some(format!("list:{}:row:{}", list_id, primary_index)),
				task_db_id: Some(primary_task_id),
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

		// Create duplicate rows (completed immediately, no RabbitMQ publish, no result yet)
		for &dup_index in &indices[1..] {
			let dup_email = parsed.rows[dup_index]
				.get(&parsed.email_column)
				.and_then(Value::as_str)
				.unwrap_or_default()
				.trim()
				.to_string();
			let dup_extra = serde_json::json!({
				"list_id": list_id,
				"row_index": dup_index as i32,
				"email_column": parsed.email_column,
			});
			sqlx::query(
				r#"
				INSERT INTO v1_task_result (
					job_id, payload, extra, task_state, tenant_id,
					canonical_email, is_duplicate, canonical_task_id
				)
				VALUES ($1, $2, $3, 'completed', $4, $5, true, $6)
				"#,
			)
			.bind(job_id)
			.bind(serde_json::json!({
				"input": {"to_email": dup_email},
				"job_id": {"bulk": job_id},
				"webhook": null
			}))
			.bind(dup_extra)
			.bind(tenant_id)
			.bind(canonical)
			.bind(primary_task_id)
			.execute(&pg_pool)
			.await
			.map_err(ReacherResponseError::from)?;
		}
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
			"score": {"score": 0, "category": "invalid", "sub_reason": "invalid_syntax", "safe_to_send": false, "reason_codes": ["invalid_syntax"]}
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
