use super::canonicalize::canonicalize_email;
use super::csv_parse::{parse_csv, ParsedCsv};
use crate::config::BackendConfig;
use crate::finder::require_tenant_id;
use crate::http::v0::check_email::post::with_config;
use crate::http::v1::bulk::with_worker_db;
use crate::http::{check_scope, resolve_tenant, CheckEmailRequest, ReacherResponseError};
use crate::scoring::response::{prepare_verification_response, PreparedVerificationResponse};
use crate::tenant::context::{scope, TenantContext};
use crate::tenant::models::PlanTier;
use crate::tenant::quota::{reserve_quota_in_tx, QuotaCheckResult};
use crate::worker::do_work::{CheckEmailJobId, CheckEmailTask, TaskMetadata};
use bytes::Buf;
use check_if_email_exists::{CheckEmailOutput, Reachable, LOG_TARGET};
use chrono::Utc;
use futures::TryStreamExt;
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
	unique_emails: i32,
	duplicates: i32,
	rejected: Vec<RejectedRow>,
	email_column: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	source_key: Option<String>,
}

async fn http_handler(
	tenant_ctx: TenantContext,
	config: Arc<BackendConfig>,
	pg_pool: PgPool,
	form: FormData,
	idempotency_key: Option<String>,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::LISTS)?;

	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	let upload = read_upload(form).await.map_err(warp::reject::custom)?;
	let parsed = parse_csv(
		&upload.file_bytes,
		upload.email_column.as_deref(),
		Some(plan_row_limit(&tenant_ctx)),
	)
	.map_err(warp::reject::custom)?;
	let source_key = normalize_source_key(upload.source_key.as_deref());

	if let Some(policy_id) = upload.policy_id {
		let exists: bool = sqlx::query_scalar(
			"SELECT EXISTS(SELECT 1 FROM v1_score_policies WHERE id = $1 AND tenant_id = $2)",
		)
		.bind(policy_id)
		.bind(tenant_id)
		.fetch_one(&pg_pool)
		.await
		.map_err(ReacherResponseError::from)?;
		if !exists {
			return Err(ReacherResponseError::new(
				StatusCode::BAD_REQUEST,
				"Referenced score policy does not exist for this tenant",
			)
			.into());
		}
	}

	enforce_row_limit(&tenant_ctx, parsed.rows.len())?;

	let (canonical_groups, blank_indices) = group_rows(&parsed);

	let rejected = blank_indices
		.iter()
		.map(|&index| RejectedRow {
			row_number: index + 1,
			value: parsed.rows[index]
				.get(&parsed.email_column)
				.and_then(Value::as_str)
				.unwrap_or_default()
				.to_string(),
			reason: "Invalid email address".into(),
		})
		.collect();
	let unique_email_count = canonical_groups.len() as i32;
	let deduplicated_count =
		(canonical_groups.values().map(|v| v.len()).sum::<usize>() - canonical_groups.len()) as i32;

	// Reserve quota in the same transaction as every row and dispatch intent.
	crate::smtp_egress::require_smtp_ready(&config)?;
	let mut tx = pg_pool.begin().await.map_err(ReacherResponseError::from)?;
	let request_hash = if idempotency_key.is_some() {
		crate::http::idempotency::hash_request_body(
		&serde_json::to_vec(&serde_json::json!({
			"file_sha256": crate::http::idempotency::hash_request_body(&upload.file_bytes), "filename": upload.filename, "name": upload.name,
			"email_column": upload.email_column, "source_key": source_key, "policy_id": upload.policy_id
		}))
		.map_err(ReacherResponseError::from)?,
	)
	} else {
		Vec::new()
	};
	if let Some(key) = &idempotency_key {
		if key.is_empty() || key.len() > 200 {
			return Err(ReacherResponseError::new(
				StatusCode::BAD_REQUEST,
				"Invalid idempotency key",
			)
			.into());
		}
		sqlx::query("SELECT pg_advisory_xact_lock(hashtextextended($1, 0))")
			.bind(format!("list-upload:{tenant_id}:{key}"))
			.execute(&mut *tx)
			.await
			.map_err(ReacherResponseError::from)?;
		let cached: Option<(Vec<u8>, serde_json::Value)> = sqlx::query_as("SELECT request_hash, response FROM v1_list_upload_requests WHERE tenant_id=$1 AND request_key=$2")
            .bind(tenant_id).bind(key).fetch_optional(&mut *tx).await.map_err(ReacherResponseError::from)?;
		if let Some((hash, response)) = cached {
			if hash != request_hash {
				return Err(ReacherResponseError::new(
					StatusCode::CONFLICT,
					"Upload changed; use a new idempotency key",
				)
				.into());
			}
			return Ok(warp::reply::with_status(
				warp::reply::json(&response),
				StatusCode::ACCEPTED,
			));
		}
	}
	match reserve_quota_in_tx(&mut tx, &tenant_ctx, unique_email_count)
		.await
		.map_err(ReacherResponseError::from)?
	{
		QuotaCheckResult::Allowed => {}
		QuotaCheckResult::Unavailable => {
			return Err(ReacherResponseError::new(
				StatusCode::SERVICE_UNAVAILABLE,
				"Quota service unavailable; no verification was dispatched.",
			)
			.into())
		}
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
		"INSERT INTO v1_bulk_job (total_records, tenant_id, status, source_key) VALUES ($1, $2, 'pending'::job_state, $3) RETURNING id",
	)
	.bind(parsed.rows.len() as i32)
	.bind(tenant_id)
	.bind(&source_key)
	.fetch_one(&mut *tx)
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
			unique_emails, deduplicated_count, source_key, policy_id
		)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, 'uploading'::list_status, $10, $11, $12, $13)
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
	.bind(&source_key)
	.bind(upload.policy_id)
	.fetch_one(&mut *tx)
	.await
	.map_err(ReacherResponseError::from)?;

	// Handle blank emails
	for &index in &blank_indices {
		let extra = serde_json::json!({
			"list_id": list_id,
			"row_index": index as i32,
			"original_email": parsed.rows[index].get(&parsed.email_column).and_then(Value::as_str).unwrap_or_default(),
			"email_column": parsed.email_column,
		});
		let invalid_result = blank_email_result(config.as_ref())
			.await
			.map_err(ReacherResponseError::from)?;
		sqlx::query(
			r#"
			INSERT INTO v1_task_result (
				job_id, payload, extra, result, tenant_id, task_state,
				score, score_category, sub_reason, safe_to_send, reason_codes,
				bounce_risk_score, bounce_risk_category, bounce_risk_confidence,
				bounce_risk_action, bounce_risk_model_version, bounce_risk_signals,
				completed_at, canonical_email, is_duplicate, source_key
			)
				VALUES ($1, $2, $3, $4, $5, 'completed', $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, NOW(), NULL, false, $17)
				"#,
			)
		.bind(job_id)
		.bind(serde_json::json!({
			"input": {"to_email": parsed.rows[index].get(&parsed.email_column).and_then(Value::as_str).unwrap_or_default()},
			"job_id": {"bulk": job_id},
			"webhook": null
		}))
		.bind(extra)
		.bind(invalid_result.json)
		.bind(tenant_id)
		.bind(i32::from(invalid_result.score.score))
		.bind(
			serde_json::to_value(&invalid_result.score.category)
				.ok()
				.and_then(|value| value.as_str().map(ToOwned::to_owned))
				.unwrap_or_else(|| "invalid".to_string()),
		)
		.bind(
			serde_json::to_value(&invalid_result.score.sub_reason)
				.ok()
				.and_then(|value| value.as_str().map(ToOwned::to_owned))
				.unwrap_or_else(|| "invalid_syntax".to_string()),
		)
		.bind(invalid_result.score.safe_to_send)
		.bind(&invalid_result.score.reason_codes)
		.bind(invalid_result.bounce_risk.as_ref().map(|risk| i32::from(risk.score)))
		.bind(
			invalid_result
				.bounce_risk
				.as_ref()
				.and_then(|risk| serde_json::to_value(&risk.category).ok())
				.and_then(|value| value.as_str().map(ToOwned::to_owned)),
		)
		.bind(invalid_result.bounce_risk.as_ref().map(|risk| risk.confidence))
		.bind(
			invalid_result
				.bounce_risk
				.as_ref()
				.and_then(|risk| serde_json::to_value(&risk.action).ok())
				.and_then(|value| value.as_str().map(ToOwned::to_owned)),
		)
			.bind(
				invalid_result
					.bounce_risk
					.as_ref()
					.map(|risk| risk.model_version.clone()),
			)
			.bind(invalid_result.bounce_risk_signals)
			.bind(&source_key)
		.execute(&mut *tx)
		.await
		.map_err(ReacherResponseError::from)?;
	}

	// Handle canonical groups: first occurrence is primary, rest are duplicates.
	// Every row is inserted before any task is published. The worker copies a
	// primary's result only to duplicate rows that already exist, and marks the
	// job finished once no queued rows remain, so a row inserted after a
	// publish could stay queued forever or finish the job early.
	let mut primaries = Vec::with_capacity(canonical_groups.len());
	for (canonical, indices) in &canonical_groups {
		let primary_index = indices[0];
		let routing_email = parsed.rows[primary_index]
			.get(&parsed.email_column)
			.and_then(Value::as_str)
			.unwrap_or_default()
			.trim()
			.to_string();

		// Create primary task (queued, published to RabbitMQ below)
		let primary_extra = serde_json::json!({
			"list_id": list_id,
			"row_index": primary_index as i32,
			"original_email": parsed.rows[primary_index].get(&parsed.email_column).and_then(Value::as_str).unwrap_or_default(),
			"email_column": parsed.email_column,
		});
		// Identity groups equivalent addresses; SMTP receives the supplied spelling.
		let primary_task_id: i32 = sqlx::query_scalar(
			r#"
			INSERT INTO v1_task_result (job_id, payload, extra, task_state, tenant_id, canonical_email, is_duplicate, source_key)
			VALUES ($1, $2, $3, 'queued', $4, $5, false, $6)
			RETURNING id
			"#,
		)
		.bind(job_id)
		.bind(serde_json::json!({
			"input": {"to_email": routing_email},
			"job_id": {"bulk": job_id},
			"webhook": null
		}))
		.bind(primary_extra)
		.bind(tenant_id)
		.bind(canonical)
		.bind(&source_key)
		.fetch_one(&mut *tx)
		.await
		.map_err(ReacherResponseError::from)?;
		primaries.push((routing_email, primary_index, primary_task_id));

		// Create duplicate rows (no RabbitMQ publish, no result yet)
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
			"original_email": parsed.rows[dup_index].get(&parsed.email_column).and_then(Value::as_str).unwrap_or_default(),
				"email_column": parsed.email_column,
			});
			// Mark as 'queued' (not 'completed') so polling consumers don't see
			// rows with NULL result. Propagation UPDATE sets state to 'completed'.
			sqlx::query(
				r#"
				INSERT INTO v1_task_result (
					job_id, payload, extra, task_state, tenant_id,
					canonical_email, is_duplicate, canonical_task_id, source_key
				)
				VALUES ($1, $2, $3, 'queued', $4, $5, true, $6, $7)
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
			.bind(&source_key)
			.execute(&mut *tx)
			.await
			.map_err(ReacherResponseError::from)?;
		}
	}

	for (routing_email, primary_index, primary_task_id) in primaries {
		let task = CheckEmailTask {
			verification_backend: config.verification_backend,
			input: CheckEmailRequest {
				to_email: routing_email,
				..Default::default()
			}
			.to_check_email_input(Arc::clone(&config)),
			job_id: CheckEmailJobId::Bulk(job_id),
			webhook: None,
			metadata: Some(TaskMetadata {
				dispatch_id: None,
				lease_id: None,
				decision_policy: None,
				tenant_id: Some(tenant_id.to_string()),
				request_id: None,
				correlation_id: None,
				created_by: Some("lists".to_string()),
				retry_policy: None,
				dedupe_key: Some(format!("list:{}:row:{}", list_id, primary_index)),
				task_db_id: Some(primary_task_id),
			}),
		};
		crate::worker::dispatch::enqueue(&mut tx, task, 1)
			.await
			.map_err(ReacherResponseError::from)?;
	}

	// The worker may already have finished every task and set a terminal
	// status, or the job may have been cancelled; only move the job and list
	// out of their initial states.
	let job_status = if unique_email_count > 0 {
		"running"
	} else {
		"completed"
	};
	let list_status = if unique_email_count > 0 {
		"processing"
	} else {
		"completed"
	};
	sqlx::query("UPDATE v1_bulk_job SET status = $2::job_state, updated_at = NOW(), completed_at = CASE WHEN $2 = 'completed' THEN NOW() ELSE NULL END WHERE id = $1 AND status = 'pending'")
		.bind(job_id)
		.bind(job_status)
		.execute(&mut *tx)
		.await
		.map_err(ReacherResponseError::from)?;
	sqlx::query("UPDATE v1_lists SET status = $2::list_status, completed_at = CASE WHEN $2 = 'completed' THEN NOW() ELSE NULL END, updated_at = NOW() WHERE id = $1 AND status = 'uploading'")
		.bind(list_id)
		.bind(list_status)
		.execute(&mut *tx)
		.await
		.map_err(ReacherResponseError::from)?;

	let response = Response {
		list_id,
		job_id,
		total_rows: parsed.rows.len() as i32,
		unique_emails: unique_email_count,
		duplicates: deduplicated_count,
		rejected,
		email_column: parsed.email_column,
		source_key,
	};
	if let Some(key) = idempotency_key {
		sqlx::query("INSERT INTO v1_list_upload_requests (tenant_id,request_key,request_hash,response) VALUES ($1,$2,$3,$4)")
            .bind(tenant_id).bind(key).bind(request_hash).bind(serde_json::to_value(&response).map_err(ReacherResponseError::from)?)
            .execute(&mut *tx).await.map_err(ReacherResponseError::from)?;
	}
	tx.commit().await.map_err(ReacherResponseError::from)?;
	Ok(warp::reply::with_status(
		warp::reply::json(&response),
		StatusCode::ACCEPTED,
	))
}

struct UploadData {
	file_bytes: Vec<u8>,
	filename: String,
	name: Option<String>,
	email_column: Option<String>,
	source_key: Option<String>,
	policy_id: Option<i64>,
}

async fn read_upload(mut form: FormData) -> Result<UploadData, ReacherResponseError> {
	let mut file_bytes = None;
	let mut filename = None;
	let mut name = None;
	let mut email_column = None;
	let mut source_key = None;
	let mut policy_id = None;

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
			// An empty field (as HTML forms send) means "auto-detect".
			"email_column" => {
				email_column = Some(String::from_utf8_lossy(&collected).trim().to_string())
					.filter(|column| !column.is_empty())
			}
			"source_key" | "source" => {
				source_key = Some(String::from_utf8_lossy(&collected).trim().to_string())
			}
			"policy_id" => {
				let raw = String::from_utf8_lossy(&collected).trim().to_string();
				if !raw.is_empty() {
					policy_id = Some(raw.parse::<i64>().map_err(|_| {
						ReacherResponseError::new(
							StatusCode::BAD_REQUEST,
							"policy_id must be an integer",
						)
					})?);
				}
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
		source_key,
		policy_id,
	})
}

fn normalize_source_key(source_key: Option<&str>) -> Option<String> {
	let source_key = source_key?.trim().to_ascii_lowercase();
	if source_key.is_empty() {
		None
	} else {
		Some(source_key)
	}
}

fn build_original_data(parsed: &ParsedCsv) -> serde_json::Value {
	let mut root = serde_json::Map::new();
	for (index, row) in parsed.rows.iter().enumerate() {
		root.insert(index.to_string(), Value::Object(row.clone()));
	}
	Value::Object(root)
}

async fn blank_email_result(
	config: &BackendConfig,
) -> Result<PreparedVerificationResponse, anyhow::Error> {
	let output = CheckEmailOutput {
		input: "".to_string(),
		is_reachable: Reachable::Invalid,
		..Default::default()
	};
	prepare_verification_response(config, &output, None, Utc::now(), false).await
}

fn plan_row_limit(tenant_ctx: &TenantContext) -> usize {
	match tenant_ctx.plan_tier {
		PlanTier::Free => 1_000usize,
		PlanTier::Starter => 10_000usize,
		PlanTier::Professional => 50_000usize,
		PlanTier::Enterprise => 100_000usize,
	}
}

fn enforce_row_limit(tenant_ctx: &TenantContext, rows: usize) -> Result<(), warp::Rejection> {
	let limit = plan_row_limit(tenant_ctx);
	if rows > limit {
		// Parsing stops one row past the limit, so the real count is unknown.
		return Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			format!(
				"List contains more than {} rows; plan allows {}",
				limit, limit
			),
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
		.and(warp::header::optional::<String>("Idempotency-Key"))
		.and_then(http_handler)
		.with(warp::log(LOG_TARGET))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn source_key_is_normalized_for_grouping() {
		assert_eq!(
			normalize_source_key(Some(" HubSpot Campaign ")),
			Some("hubspot campaign".to_string())
		);
		assert_eq!(normalize_source_key(Some(" ")), None);
		assert_eq!(normalize_source_key(None), None);
	}

	#[test]
	fn row_limit_follows_the_plan() {
		let mut context =
			TenantContext::legacy(crate::config::ThrottleConfig::new_without_throttle());
		context.plan_tier = PlanTier::Free;
		assert!(enforce_row_limit(&context, 1_000).is_ok());
		assert!(enforce_row_limit(&context, 1_001).is_err());
		context.plan_tier = PlanTier::Professional;
		assert!(enforce_row_limit(&context, 50_000).is_ok());
		assert!(enforce_row_limit(&context, 50_001).is_err());
	}
}

fn group_rows(parsed: &ParsedCsv) -> (HashMap<String, Vec<usize>>, Vec<usize>) {
	let mut groups = HashMap::<String, Vec<usize>>::new();
	let mut rejected = Vec::new();
	for (index, row) in parsed.rows.iter().enumerate() {
		let email = row
			.get(&parsed.email_column)
			.and_then(Value::as_str)
			.unwrap_or_default()
			.trim();
		if check_if_email_exists::syntax::check_syntax(email).is_valid_syntax {
			if let Some(key) = canonicalize_email(email) {
				groups.entry(key).or_default().push(index);
				continue;
			}
		}
		rejected.push(index);
	}
	(groups, rejected)
}
#[derive(Debug, Serialize, utoipa::ToSchema)]
struct RejectedRow {
	row_number: usize,
	value: String,
	reason: String,
}
#[derive(Serialize, utoipa::ToSchema)]
struct UploadPreview {
	total_rows: usize,
	unique_emails: usize,
	duplicates: usize,
	estimated_credits: usize,
	rejected: Vec<RejectedRow>,
	email_column: String,
}
async fn preview_handler(
	ctx: TenantContext,
	form: FormData,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&ctx, scope::LISTS)?;
	crate::finder::require_tenant_id(ctx.tenant_id)?;
	let upload = read_upload(form).await?;
	let parsed = parse_csv(
		&upload.file_bytes,
		upload.email_column.as_deref(),
		Some(plan_row_limit(&ctx)),
	)?;
	enforce_row_limit(&ctx, parsed.rows.len())?;
	let (groups, rejected) = group_rows(&parsed);
	let rejected = rejected
		.into_iter()
		.map(|index| RejectedRow {
			row_number: index + 1,
			value: parsed.rows[index]
				.get(&parsed.email_column)
				.and_then(Value::as_str)
				.unwrap_or_default()
				.to_string(),
			reason: "Invalid or blank email address".into(),
		})
		.collect();
	Ok(warp::reply::json(&UploadPreview {
		total_rows: parsed.rows.len(),
		unique_emails: groups.len(),
		estimated_credits: groups.len(),
		duplicates: groups.values().map(|v| v.len() - 1).sum(),
		rejected,
		email_column: parsed.email_column,
	}))
}
#[utoipa::path(post,path="/v1/lists/preview",tag="Lists",responses((status=200,description="Non-billing upload preview",body=inline(UploadPreview))))]
pub fn v1_preview_list(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "lists" / "preview")
		.and(warp::post())
		.and(resolve_tenant(config))
		.and(warp::multipart::form().max_length(50_000_000))
		.and_then(preview_handler)
}
