use crate::config::BackendConfig;
use crate::finder::require_tenant_id;
use crate::http::v1::bulk::with_worker_db;
use crate::http::{check_scope, resolve_tenant, ReacherResponseError};
use crate::remediation::{
	classify_row, hash_json, options_hash, render_remediation_csv, suppression_lookup_candidates,
	ClassifiedRemediationRow, RemediationClassification, RemediationInputRow, RemediationOptions,
	RemediationPartition, RemediationSummaryCounts, REMEDIATION_RULE_VERSION,
};
use crate::tenant::context::{scope, TenantContext};
use check_if_email_exists::LOG_TARGET;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use sqlx::postgres::PgRow;
use sqlx::{PgPool, Row};
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::sync::Arc;
use utoipa::ToSchema;
use warp::http::{Response, StatusCode};
use warp::hyper::Body;
use warp::Filter;

#[derive(Debug, Serialize, ToSchema)]
pub struct RemediationPlanResponse {
	pub plan_id: i64,
	pub list_id: i32,
	pub status: String,
	pub rule_version: String,
	pub effective_job_id: Option<i32>,
	pub summary_counts: RemediationSummaryCounts,
	pub options: RemediationOptions,
	pub result_state_digest: String,
	pub created_at: String,
	pub completed_at: Option<String>,
}

#[derive(Debug, Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
struct PlanQuery {
	plan_id: Option<i64>,
}

#[derive(Debug, Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
struct DownloadQuery {
	partition: Option<String>,
}

struct ListContext {
	job_id: Option<i32>,
	total_rows: i32,
	email_column: String,
	original_headers: Vec<String>,
	original_rows: BTreeMap<i32, Map<String, Value>>,
}

struct TaskRow {
	task_id: i32,
	row_number: i32,
	result: Option<Value>,
	error: Option<String>,
	score_category: Option<String>,
	sub_reason: Option<String>,
	safe_to_send: Option<bool>,
	reason_codes: Option<Vec<String>>,
	canonical_email: Option<String>,
	is_duplicate: bool,
	task_state: String,
	completed_at: Option<DateTime<Utc>>,
}

async fn create_http_handler(
	list_id: i32,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	options: RemediationOptions,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::LISTS)?;
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;

	let list = load_list_context(&pg_pool, tenant_id, list_id).await?;
	ensure_list_ready(&pg_pool, list_id, list.total_rows, options.allow_partial).await?;
	let task_rows = load_task_rows(&pg_pool, list_id).await?;
	let result_state_digest = compute_result_state_digest(&task_rows);
	let options_hash = options_hash(&options);

	if let Some(existing) = fetch_plan_by_identity(
		&pg_pool,
		tenant_id,
		list_id,
		REMEDIATION_RULE_VERSION,
		&result_state_digest,
		&options_hash,
	)
	.await?
	{
		return Ok(warp::reply::with_status(
			warp::reply::json(&existing),
			StatusCode::OK,
		));
	}

	let suppressed =
		load_matching_suppressions(&pg_pool, tenant_id, &list, &task_rows, &options).await?;
	let (classified_rows, summary_counts) = classify_rows(&list, &task_rows, &suppressed, &options);
	let (plan_id, created) = persist_plan(
		&pg_pool,
		tenant_id,
		list_id,
		list.job_id,
		&options,
		&options_hash,
		&result_state_digest,
		&classified_rows,
		&summary_counts,
	)
	.await?;
	let response = fetch_plan(&pg_pool, tenant_id, list_id, Some(plan_id))
		.await?
		.ok_or_else(|| {
			warp::reject::custom(ReacherResponseError::new(
				StatusCode::INTERNAL_SERVER_ERROR,
				"Remediation plan was not persisted",
			))
		})?;

	Ok(warp::reply::with_status(
		warp::reply::json(&response),
		if created {
			StatusCode::CREATED
		} else {
			StatusCode::OK
		},
	))
}

async fn get_http_handler(
	list_id: i32,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	query: PlanQuery,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::LISTS)?;
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	load_list_context(&pg_pool, tenant_id, list_id).await?;

	let plan = fetch_plan(&pg_pool, tenant_id, list_id, query.plan_id)
		.await?
		.ok_or_else(|| {
			warp::reject::custom(ReacherResponseError::new(
				StatusCode::NOT_FOUND,
				"Remediation plan not found",
			))
		})?;

	Ok(warp::reply::json(&plan))
}

async fn download_http_handler(
	list_id: i32,
	plan_id: i64,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	query: DownloadQuery,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::LISTS)?;
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	let list = load_list_context(&pg_pool, tenant_id, list_id).await?;
	fetch_plan(&pg_pool, tenant_id, list_id, Some(plan_id))
		.await?
		.ok_or_else(|| {
			warp::reject::custom(ReacherResponseError::new(
				StatusCode::NOT_FOUND,
				"Remediation plan not found",
			))
		})?;

	let partition_name = query.partition.as_deref().unwrap_or("combined_clean");
	let partition = RemediationPartition::parse(partition_name).ok_or_else(|| {
		warp::reject::custom(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"Invalid partition. Must be one of: fixed, safe, review, drop, combined_clean",
		))
	})?;
	let rows = load_plan_rows(&pg_pool, tenant_id, plan_id).await?;
	let csv = render_remediation_csv(&list.original_headers, &list.email_column, &rows, partition)
		.map_err(ReacherResponseError::from)
		.map_err(warp::reject::custom)?;

	let response = Response::builder()
		.status(StatusCode::OK)
		.header("Content-Type", "text/csv")
		.header(
			"Content-Disposition",
			format!(
				"attachment; filename=\"list_{}_remediation_{}.csv\"",
				list_id,
				partition.as_str()
			),
		)
		.body(Body::from(csv))
		.map_err(|err| {
			warp::reject::custom(ReacherResponseError::new(
				StatusCode::INTERNAL_SERVER_ERROR,
				err,
			))
		})?;

	Ok(response)
}

async fn load_list_context(
	pg_pool: &PgPool,
	tenant_id: uuid::Uuid,
	list_id: i32,
) -> Result<ListContext, warp::Rejection> {
	let row = sqlx::query(
		r#"
		SELECT
			job_id,
			total_rows,
			email_column,
			original_headers,
			COALESCE(original_data, '{}'::jsonb) AS original_data
		FROM v1_lists
		WHERE id = $1
		  AND tenant_id = $2
		  AND status <> 'deleted'::list_status
		"#,
	)
	.bind(list_id)
	.bind(tenant_id)
	.fetch_optional(pg_pool)
	.await
	.map_err(ReacherResponseError::from)
	.map_err(warp::reject::custom)?;

	let row = row.ok_or_else(|| {
		warp::reject::custom(ReacherResponseError::new(
			StatusCode::NOT_FOUND,
			"List not found",
		))
	})?;
	let original_data: Value = row.get("original_data");
	let mut original_rows = BTreeMap::new();
	if let Some(object) = original_data.as_object() {
		for (key, value) in object {
			if let (Ok(row_number), Some(row_object)) = (key.parse::<i32>(), value.as_object()) {
				original_rows.insert(row_number, row_object.clone());
			}
		}
	}

	Ok(ListContext {
		job_id: row.get("job_id"),
		total_rows: row.get("total_rows"),
		email_column: row.get("email_column"),
		original_headers: row.get("original_headers"),
		original_rows,
	})
}

async fn ensure_list_ready(
	pg_pool: &PgPool,
	list_id: i32,
	total_rows: i32,
	allow_partial: bool,
) -> Result<(), warp::Rejection> {
	if allow_partial {
		return Ok(());
	}

	let processed: i64 = sqlx::query_scalar(
		r#"
		SELECT COUNT(DISTINCT (extra->>'row_index')::INTEGER)
		FROM v1_task_result
		WHERE (extra->>'list_id')::INTEGER = $1
		  AND (extra->>'row_index') IS NOT NULL
		  AND (
			result IS NOT NULL
			OR error IS NOT NULL
			OR task_state::TEXT IN ('completed', 'failed', 'cancelled', 'dead_lettered')
		  )
		"#,
	)
	.bind(list_id)
	.fetch_one(pg_pool)
	.await
	.map_err(ReacherResponseError::from)
	.map_err(warp::reject::custom)?;

	if processed < i64::from(total_rows) {
		return Err(warp::reject::custom(ReacherResponseError::new(
			StatusCode::CONFLICT,
			"List is still processing",
		)));
	}

	Ok(())
}

async fn load_task_rows(
	pg_pool: &PgPool,
	list_id: i32,
) -> Result<Vec<TaskRow>, ReacherResponseError> {
	let rows = sqlx::query(
		r#"
		SELECT
			id,
			(extra->>'row_index')::INTEGER AS row_number,
			result,
			error,
			score_category,
			sub_reason,
			safe_to_send,
			reason_codes,
			canonical_email,
			is_duplicate,
			task_state::TEXT AS task_state,
			completed_at
		FROM v1_task_result
		WHERE (extra->>'list_id')::INTEGER = $1
		  AND (extra->>'row_index') IS NOT NULL
		ORDER BY (extra->>'row_index')::INTEGER ASC, is_duplicate ASC, id ASC
		"#,
	)
	.bind(list_id)
	.fetch_all(pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	Ok(rows
		.into_iter()
		.map(|row| TaskRow {
			task_id: row.get("id"),
			row_number: row.get("row_number"),
			result: row.get("result"),
			error: row.get("error"),
			score_category: row.get("score_category"),
			sub_reason: row.get("sub_reason"),
			safe_to_send: row.get("safe_to_send"),
			reason_codes: row.get("reason_codes"),
			canonical_email: row.get("canonical_email"),
			is_duplicate: row.get("is_duplicate"),
			task_state: row.get("task_state"),
			completed_at: row.get("completed_at"),
		})
		.collect())
}

fn compute_result_state_digest(rows: &[TaskRow]) -> String {
	let digest_input: Vec<Value> = rows
		.iter()
		.map(|row| {
			json!({
				"task_id": row.task_id,
				"row_number": row.row_number,
				"result": row.result,
				"error": row.error,
				"score_category": row.score_category,
				"sub_reason": row.sub_reason,
				"safe_to_send": row.safe_to_send,
				"reason_codes": row.reason_codes,
				"canonical_email": row.canonical_email,
				"is_duplicate": row.is_duplicate,
				"task_state": row.task_state,
				"completed_at": row.completed_at.as_ref().map(|value| value.to_rfc3339()),
			})
		})
		.collect();
	hash_json(&digest_input)
}

async fn load_matching_suppressions(
	pg_pool: &PgPool,
	tenant_id: uuid::Uuid,
	list: &ListContext,
	task_rows: &[TaskRow],
	options: &RemediationOptions,
) -> Result<HashSet<String>, ReacherResponseError> {
	let task_by_row = first_task_by_row(task_rows);
	let mut row_numbers: BTreeSet<i32> = list.original_rows.keys().copied().collect();
	row_numbers.extend(task_by_row.keys().copied());

	let mut candidates = HashSet::new();
	for row_number in row_numbers {
		let original = list
			.original_rows
			.get(&row_number)
			.cloned()
			.unwrap_or_default();
		let result = task_by_row
			.get(&row_number)
			.and_then(|task| task.result.as_ref());
		for candidate in
			suppression_lookup_candidates(&original, &list.email_column, result, options)
		{
			if !candidate.is_empty() {
				candidates.insert(candidate);
			}
		}
	}

	if candidates.is_empty() {
		return Ok(HashSet::new());
	}

	let candidate_values: Vec<String> = candidates.into_iter().collect();
	let rows = sqlx::query(
		r#"
		SELECT email
		FROM v1_suppression_entries
		WHERE tenant_id = $1
		  AND email = ANY($2::TEXT[])
		"#,
	)
	.bind(tenant_id)
	.bind(&candidate_values)
	.fetch_all(pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	Ok(rows
		.into_iter()
		.map(|row| row.get::<String, _>("email"))
		.collect())
}

fn classify_rows(
	list: &ListContext,
	task_rows: &[TaskRow],
	suppressed: &HashSet<String>,
	options: &RemediationOptions,
) -> (Vec<ClassifiedRemediationRow>, RemediationSummaryCounts) {
	let task_by_row = first_task_by_row(task_rows);
	let mut row_numbers: BTreeSet<i32> = list.original_rows.keys().copied().collect();
	row_numbers.extend(task_by_row.keys().copied());

	let mut rows = Vec::with_capacity(row_numbers.len());
	let mut summary_counts = RemediationSummaryCounts::default();
	for row_number in row_numbers {
		let task = task_by_row.get(&row_number).copied();
		let original = list
			.original_rows
			.get(&row_number)
			.cloned()
			.unwrap_or_default();
		let candidates = suppression_lookup_candidates(
			&original,
			&list.email_column,
			task.and_then(|task| task.result.as_ref()),
			options,
		);
		let row = classify_row(
			RemediationInputRow {
				row_number,
				original,
				email_column: list.email_column.clone(),
				result: task.and_then(|task| task.result.clone()),
				score_category: task.and_then(|task| task.score_category.clone()),
				sub_reason: task.and_then(|task| task.sub_reason.clone()),
				safe_to_send: task.and_then(|task| task.safe_to_send),
				is_duplicate: task.map(|task| task.is_duplicate).unwrap_or(false),
				suppressed: candidates
					.iter()
					.any(|candidate| suppressed.contains(candidate)),
			},
			options,
		);
		summary_counts.add(row.classification);
		rows.push(row);
	}

	(rows, summary_counts)
}

fn first_task_by_row(task_rows: &[TaskRow]) -> BTreeMap<i32, &TaskRow> {
	let mut task_by_row = BTreeMap::new();
	for task in task_rows {
		task_by_row.entry(task.row_number).or_insert(task);
	}
	task_by_row
}

async fn persist_plan(
	pg_pool: &PgPool,
	tenant_id: uuid::Uuid,
	list_id: i32,
	effective_job_id: Option<i32>,
	options: &RemediationOptions,
	options_hash: &str,
	result_state_digest: &str,
	rows: &[ClassifiedRemediationRow],
	summary_counts: &RemediationSummaryCounts,
) -> Result<(i64, bool), ReacherResponseError> {
	let options_json = serde_json::to_value(options).map_err(ReacherResponseError::from)?;
	let summary_json = serde_json::to_value(summary_counts).map_err(ReacherResponseError::from)?;
	let mut tx = pg_pool.begin().await.map_err(ReacherResponseError::from)?;

	let inserted_plan_id: Option<i64> = sqlx::query_scalar(
		r#"
		INSERT INTO v1_remediation_plans (
			tenant_id,
			list_id,
			effective_job_id,
			rule_version,
			options,
			options_hash,
			result_state_digest,
			status,
			summary_counts
		)
		VALUES ($1, $2, $3, $4, $5, $6, $7, 'processing', '{}'::jsonb)
		ON CONFLICT (tenant_id, list_id, rule_version, result_state_digest, options_hash)
		DO NOTHING
		RETURNING id
		"#,
	)
	.bind(tenant_id)
	.bind(list_id)
	.bind(effective_job_id)
	.bind(REMEDIATION_RULE_VERSION)
	.bind(&options_json)
	.bind(options_hash)
	.bind(result_state_digest)
	.fetch_optional(&mut *tx)
	.await
	.map_err(ReacherResponseError::from)?;

	let Some(plan_id) = inserted_plan_id else {
		let existing_id: i64 = sqlx::query_scalar(
			r#"
			SELECT id
			FROM v1_remediation_plans
			WHERE tenant_id = $1
			  AND list_id = $2
			  AND rule_version = $3
			  AND result_state_digest = $4
			  AND options_hash = $5
			"#,
		)
		.bind(tenant_id)
		.bind(list_id)
		.bind(REMEDIATION_RULE_VERSION)
		.bind(result_state_digest)
		.bind(options_hash)
		.fetch_one(&mut *tx)
		.await
		.map_err(ReacherResponseError::from)?;
		tx.commit().await.map_err(ReacherResponseError::from)?;
		return Ok((existing_id, false));
	};

	for row in rows {
		sqlx::query(
			r#"
			INSERT INTO v1_remediation_rows (
				tenant_id,
				plan_id,
				row_number,
				classification,
				rule_id,
				confidence,
				before,
				after
			)
			VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
			"#,
		)
		.bind(tenant_id)
		.bind(plan_id)
		.bind(row.row_number)
		.bind(row.classification.as_str())
		.bind(&row.rule_id)
		.bind(row.confidence)
		.bind(Value::Object(row.before.clone()))
		.bind(Value::Object(row.after.clone()))
		.execute(&mut *tx)
		.await
		.map_err(ReacherResponseError::from)?;
	}

	sqlx::query(
		r#"
		UPDATE v1_remediation_plans
		SET status = 'completed',
		    options = $2,
		    summary_counts = $3,
		    completed_at = NOW()
		WHERE id = $1
		"#,
	)
	.bind(plan_id)
	.bind(&options_json)
	.bind(&summary_json)
	.execute(&mut *tx)
	.await
	.map_err(ReacherResponseError::from)?;

	tx.commit().await.map_err(ReacherResponseError::from)?;
	Ok((plan_id, true))
}

async fn fetch_plan_by_identity(
	pg_pool: &PgPool,
	tenant_id: uuid::Uuid,
	list_id: i32,
	rule_version: &str,
	result_state_digest: &str,
	options_hash: &str,
) -> Result<Option<RemediationPlanResponse>, ReacherResponseError> {
	let sql = plan_select_sql(
		r#"
		WHERE tenant_id = $1
		  AND list_id = $2
		  AND rule_version = $3
		  AND result_state_digest = $4
		  AND options_hash = $5
		"#,
	);
	let row = sqlx::query(&sql)
		.bind(tenant_id)
		.bind(list_id)
		.bind(rule_version)
		.bind(result_state_digest)
		.bind(options_hash)
		.fetch_optional(pg_pool)
		.await
		.map_err(ReacherResponseError::from)?;

	row.map(plan_response_from_row).transpose()
}

async fn fetch_plan(
	pg_pool: &PgPool,
	tenant_id: uuid::Uuid,
	list_id: i32,
	plan_id: Option<i64>,
) -> Result<Option<RemediationPlanResponse>, ReacherResponseError> {
	let row = if let Some(plan_id) = plan_id {
		let sql = plan_select_sql(
			r#"
			WHERE tenant_id = $1
			  AND list_id = $2
			  AND id = $3
			"#,
		);
		sqlx::query(&sql)
			.bind(tenant_id)
			.bind(list_id)
			.bind(plan_id)
			.fetch_optional(pg_pool)
			.await
			.map_err(ReacherResponseError::from)?
	} else {
		let sql = plan_select_sql(
			r#"
			WHERE tenant_id = $1
			  AND list_id = $2
			ORDER BY created_at DESC
			LIMIT 1
			"#,
		);
		sqlx::query(&sql)
			.bind(tenant_id)
			.bind(list_id)
			.fetch_optional(pg_pool)
			.await
			.map_err(ReacherResponseError::from)?
	};

	row.map(plan_response_from_row).transpose()
}

fn plan_select_sql(where_clause: &str) -> String {
	format!(
		r#"
		SELECT
			id,
			list_id,
			status,
			rule_version,
			effective_job_id,
			summary_counts,
			options,
			result_state_digest,
			created_at,
			completed_at
		FROM v1_remediation_plans
		{where_clause}
		"#
	)
}

fn plan_response_from_row(row: PgRow) -> Result<RemediationPlanResponse, ReacherResponseError> {
	let options: Value = row.get("options");
	let summary_counts: Value = row.get("summary_counts");
	let created_at: DateTime<Utc> = row.get("created_at");
	let completed_at: Option<DateTime<Utc>> = row.get("completed_at");
	Ok(RemediationPlanResponse {
		plan_id: row.get("id"),
		list_id: row.get("list_id"),
		status: row.get("status"),
		rule_version: row.get("rule_version"),
		effective_job_id: row.get("effective_job_id"),
		summary_counts: serde_json::from_value(summary_counts)
			.map_err(ReacherResponseError::from)?,
		options: serde_json::from_value(options).map_err(ReacherResponseError::from)?,
		result_state_digest: row.get("result_state_digest"),
		created_at: created_at.to_rfc3339(),
		completed_at: completed_at.map(|value| value.to_rfc3339()),
	})
}

async fn load_plan_rows(
	pg_pool: &PgPool,
	tenant_id: uuid::Uuid,
	plan_id: i64,
) -> Result<Vec<ClassifiedRemediationRow>, warp::Rejection> {
	let rows = sqlx::query(
		r#"
		SELECT row_number, classification, rule_id, confidence, before, after
		FROM v1_remediation_rows
		WHERE tenant_id = $1
		  AND plan_id = $2
		ORDER BY row_number ASC
		"#,
	)
	.bind(tenant_id)
	.bind(plan_id)
	.fetch_all(pg_pool)
	.await
	.map_err(ReacherResponseError::from)
	.map_err(warp::reject::custom)?;

	let mut parsed_rows = Vec::with_capacity(rows.len());
	for row in rows {
		let classification: String = row.get("classification");
		let classification =
			RemediationClassification::parse(&classification).ok_or_else(|| {
				warp::reject::custom(ReacherResponseError::new(
					StatusCode::INTERNAL_SERVER_ERROR,
					format!("Unknown remediation classification: {classification}"),
				))
			})?;
		let confidence: String = row.get("confidence");
		let confidence = match confidence.as_str() {
			"high" => "high",
			"medium" => "medium",
			"low" => "low",
			_ => {
				return Err(warp::reject::custom(ReacherResponseError::new(
					StatusCode::INTERNAL_SERVER_ERROR,
					format!("Unknown remediation confidence: {confidence}"),
				)))
			}
		};
		let before: Value = row.get("before");
		let after: Value = row.get("after");
		parsed_rows.push(ClassifiedRemediationRow {
			row_number: row.get("row_number"),
			classification,
			rule_id: row.get("rule_id"),
			confidence,
			before: before.as_object().cloned().unwrap_or_default(),
			after: after.as_object().cloned().unwrap_or_default(),
		});
	}

	Ok(parsed_rows)
}

/// POST /v1/lists/{list_id}/remediation-plan
#[utoipa::path(
	post,
	path = "/v1/lists/{list_id}/remediation-plan",
	tag = "Lists",
	params(("list_id" = i32, Path, description = "List identifier")),
	request_body = RemediationOptions,
	responses(
		(status = 201, description = "Remediation plan created", body = RemediationPlanResponse),
		(status = 200, description = "Existing remediation plan returned", body = RemediationPlanResponse),
		(status = 409, description = "List is still processing")
	)
)]
pub fn v1_create_remediation_plan(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "lists" / i32 / "remediation-plan")
		.and(warp::post())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::body::json())
		.and_then(create_http_handler)
		.with(warp::log(LOG_TARGET))
}

/// GET /v1/lists/{list_id}/remediation-plan
#[utoipa::path(
	get,
	path = "/v1/lists/{list_id}/remediation-plan",
	tag = "Lists",
	params(("list_id" = i32, Path, description = "List identifier"), PlanQuery),
	responses((status = 200, description = "Remediation plan", body = RemediationPlanResponse))
)]
pub fn v1_get_remediation_plan(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "lists" / i32 / "remediation-plan")
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::query::<PlanQuery>())
		.and_then(get_http_handler)
		.with(warp::log(LOG_TARGET))
}

/// GET /v1/lists/{list_id}/remediation-plan/{plan_id}/download
#[utoipa::path(
	get,
	path = "/v1/lists/{list_id}/remediation-plan/{plan_id}/download",
	tag = "Lists",
	params(
		("list_id" = i32, Path, description = "List identifier"),
		("plan_id" = i64, Path, description = "Remediation plan identifier"),
		DownloadQuery
	),
	responses((status = 200, description = "Remediation CSV download"))
)]
pub fn v1_download_remediation_plan(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "lists" / i32 / "remediation-plan" / i64 / "download")
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::query::<DownloadQuery>())
		.and_then(download_http_handler)
		.with(warp::log(LOG_TARGET))
}
