use super::get_detail::list_summary;
use crate::config::BackendConfig;
use crate::finder::require_tenant_id;
use crate::http::v1::bulk::with_worker_db;
use crate::http::{resolve_tenant, ReacherResponseError};
use crate::tenant::context::TenantContext;
use bytes::Bytes;
use check_if_email_exists::LOG_TARGET;
use chrono::{DateTime, Utc};
use futures::stream;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use sqlx::{PgPool, Row};
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use std::io;
use std::sync::Arc;
use warp::http::StatusCode;
use warp::hyper::Body;
use warp::Filter;

const RULE_VERSION: &str = "remediation_v1";
const PREVIEW_ROW_LIMIT: i64 = 100;
const EXPORT_BATCH_SIZE: i64 = 500;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum RemediationClassification {
	Fixed,
	Safe,
	Review,
	Drop,
}

impl RemediationClassification {
	fn as_str(self) -> &'static str {
		match self {
			Self::Fixed => "fixed",
			Self::Safe => "safe",
			Self::Review => "review",
			Self::Drop => "drop",
		}
	}
}

#[derive(Debug, Clone, Serialize)]
struct PlanOptions {
	allow_partial: bool,
	apply_domain_typos: bool,
	normalize_emails: bool,
	deduplicate: bool,
	drop_suppressed: bool,
}

impl Default for PlanOptions {
	fn default() -> Self {
		Self {
			allow_partial: false,
			apply_domain_typos: true,
			normalize_emails: true,
			deduplicate: true,
			drop_suppressed: true,
		}
	}
}

#[derive(Debug, Default, Deserialize)]
struct CreatePlanRequest {
	allow_partial: Option<bool>,
	apply_domain_typos: Option<bool>,
	normalize_emails: Option<bool>,
	deduplicate: Option<bool>,
	drop_suppressed: Option<bool>,
}

#[derive(Debug, Default, Deserialize)]
struct CreateExportRequest {
	plan_id: Option<i64>,
	partitions: Option<Vec<String>>,
	format: Option<String>,
}

impl From<CreatePlanRequest> for PlanOptions {
	fn from(request: CreatePlanRequest) -> Self {
		let defaults = PlanOptions::default();
		Self {
			allow_partial: request.allow_partial.unwrap_or(defaults.allow_partial),
			apply_domain_typos: request
				.apply_domain_typos
				.unwrap_or(defaults.apply_domain_typos),
			normalize_emails: request
				.normalize_emails
				.unwrap_or(defaults.normalize_emails),
			deduplicate: request.deduplicate.unwrap_or(defaults.deduplicate),
			drop_suppressed: request.drop_suppressed.unwrap_or(defaults.drop_suppressed),
		}
	}
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
struct SummaryCounts {
	fixed: i64,
	safe: i64,
	review: i64,
	drop: i64,
}

impl SummaryCounts {
	fn increment(&mut self, classification: RemediationClassification) {
		match classification {
			RemediationClassification::Fixed => self.fixed += 1,
			RemediationClassification::Safe => self.safe += 1,
			RemediationClassification::Review => self.review += 1,
			RemediationClassification::Drop => self.drop += 1,
		}
	}
}

#[derive(Debug, Serialize)]
struct PlanResponse {
	id: i64,
	list_id: i32,
	job_id: Option<i32>,
	status: String,
	rule_version: String,
	options: Value,
	result_state_digest: String,
	summary_counts: SummaryCounts,
	created_at: String,
	completed_at: Option<String>,
	preview_rows: Vec<RemediationRowResponse>,
}

#[derive(Debug, Serialize)]
struct RemediationRowResponse {
	id: i64,
	row_index: i32,
	classification: String,
	rule_id: String,
	confidence: String,
	original_email: String,
	effective_email: String,
	before: Value,
	after: Value,
	reasons: Value,
}

#[derive(Debug, Serialize)]
struct ExportResponse {
	id: i64,
	list_id: i32,
	plan_id: i64,
	partitions: Vec<String>,
	format: String,
	download_url: String,
	created_at: String,
}

struct ListContext {
	id: i32,
	job_id: i32,
	status: String,
	total_rows: i32,
	email_column: String,
	original_data: Value,
}

#[derive(Debug, Clone)]
struct RawTaskRow {
	id: i32,
	row_index: i32,
	payload: Value,
	result: Option<Value>,
	error: Option<String>,
	canonical_email: Option<String>,
	is_duplicate: bool,
	is_suppressed: bool,
	suppression_reason_code: Option<String>,
	recommendation: Option<Value>,
	recommendation_action: Option<String>,
	policy_decision: Option<String>,
	score_category: Option<String>,
	safe_to_send: Option<bool>,
	reason_codes: Option<Vec<String>>,
	completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
struct PlannedRemediationRow {
	row_index: i32,
	classification: RemediationClassification,
	rule_id: String,
	confidence: String,
	original_email: String,
	effective_email: String,
	before: Value,
	after: Value,
	reasons: Value,
	task_result_id: Option<i32>,
}

struct ExportRow {
	row_index: i32,
	classification: String,
	rule_id: String,
	confidence: String,
	original_email: String,
	effective_email: String,
	after: Value,
	reasons: Value,
}

struct ExportDownloadState {
	pg_pool: PgPool,
	tenant_id: uuid::Uuid,
	plan_id: i64,
	partitions: Vec<String>,
	headers: Vec<String>,
	last_row_index: i32,
	header_sent: bool,
}

#[derive(Debug, Clone)]
struct LocalReason {
	code: &'static str,
	message: String,
	severity: &'static str,
}

async fn create_plan_handler(
	list_id: i32,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	body: CreatePlanRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	let options = PlanOptions::from(body);
	let list = load_list_context(&pg_pool, list_id, tenant_id)
		.await
		.map_err(warp::reject::custom)?;
	let summary = list_summary(&pg_pool, list_id)
		.await
		.map_err(warp::reject::custom)?;
	ensure_plan_allowed(
		&list.status,
		list.total_rows,
		summary.total_processed,
		options.allow_partial,
	)?;

	let original_rows = original_rows_by_index(&list.original_data);
	let task_rows = load_task_rows(&pg_pool, list_id, tenant_id)
		.await
		.map_err(warp::reject::custom)?;
	let planned_rows = plan_rows(&task_rows, &original_rows, &list.email_column, &options);
	let summary_counts = summarize_rows(&planned_rows);
	let options_value = serde_json::to_value(&options).map_err(ReacherResponseError::from)?;
	let summary_value =
		serde_json::to_value(&summary_counts).map_err(ReacherResponseError::from)?;
	let result_state_digest = result_state_digest(&task_rows);

	let mut tx = pg_pool.begin().await.map_err(ReacherResponseError::from)?;
	let inserted_plan_id: Option<i64> = sqlx::query_scalar(
		r#"
		INSERT INTO v1_remediation_plans (
			tenant_id, list_id, job_id, status, rule_version, options,
			result_state_digest, summary_counts, completed_at
		)
		VALUES ($1, $2, $3, 'completed', $4, $5, $6, $7, NOW())
		ON CONFLICT (tenant_id, list_id, rule_version, result_state_digest, options)
		DO NOTHING
		RETURNING id
		"#,
	)
	.bind(tenant_id)
	.bind(list.id)
	.bind(list.job_id)
	.bind(RULE_VERSION)
	.bind(&options_value)
	.bind(&result_state_digest)
	.bind(&summary_value)
	.fetch_optional(&mut *tx)
	.await
	.map_err(ReacherResponseError::from)?;

	let plan_id = if let Some(plan_id) = inserted_plan_id {
		insert_plan_rows(&mut tx, plan_id, tenant_id, list.id, &planned_rows)
			.await
			.map_err(warp::reject::custom)?;
		plan_id
	} else {
		sqlx::query_scalar(
			r#"
			SELECT id
			FROM v1_remediation_plans
			WHERE tenant_id = $1
			  AND list_id = $2
			  AND rule_version = $3
			  AND result_state_digest = $4
			  AND options = $5
			"#,
		)
		.bind(tenant_id)
		.bind(list.id)
		.bind(RULE_VERSION)
		.bind(&result_state_digest)
		.bind(&options_value)
		.fetch_one(&mut *tx)
		.await
		.map_err(ReacherResponseError::from)?
	};

	tx.commit().await.map_err(ReacherResponseError::from)?;
	let response = fetch_plan_response(&pg_pool, tenant_id, list.id, Some(plan_id))
		.await
		.map_err(warp::reject::custom)?;

	Ok(warp::reply::with_status(
		warp::reply::json(&response),
		StatusCode::CREATED,
	))
}

async fn get_plan_handler(
	list_id: i32,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	load_list_context(&pg_pool, list_id, tenant_id)
		.await
		.map_err(warp::reject::custom)?;
	let response = fetch_plan_response(&pg_pool, tenant_id, list_id, None)
		.await
		.map_err(warp::reject::custom)?;
	Ok(warp::reply::json(&response))
}

async fn create_export_handler(
	list_id: i32,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	body: CreateExportRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	load_list_context(&pg_pool, list_id, tenant_id)
		.await
		.map_err(warp::reject::custom)?;
	let format = body.format.unwrap_or_else(|| "csv".to_string());
	if format != "csv" {
		return Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"Only format=csv is supported",
		)
		.into());
	}
	let partitions = normalize_export_partitions(body.partitions)?;
	let plan_id = find_plan_id(&pg_pool, tenant_id, list_id, body.plan_id)
		.await
		.map_err(warp::reject::custom)?;

	let row = sqlx::query(
		r#"
		INSERT INTO v1_remediation_exports (tenant_id, plan_id, partitions, format)
		VALUES ($1, $2, $3, $4)
		RETURNING id, created_at
		"#,
	)
	.bind(tenant_id)
	.bind(plan_id)
	.bind(&partitions)
	.bind(&format)
	.fetch_one(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)
	.map_err(warp::reject::custom)?;

	let id: i64 = row.get("id");
	let created_at = row.get::<DateTime<Utc>, _>("created_at").to_rfc3339();
	Ok(warp::reply::with_status(
		warp::reply::json(&ExportResponse {
			id,
			list_id,
			plan_id,
			partitions,
			format,
			download_url: format!("/v1/lists/{}/remediation-exports/{}/download", list_id, id),
			created_at,
		}),
		StatusCode::CREATED,
	))
}

async fn download_export_handler(
	list_id: i32,
	export_id: i64,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	let export = sqlx::query(
		r#"
		SELECT e.id, e.plan_id, e.partitions, e.format, l.original_headers
		FROM v1_remediation_exports e
		JOIN v1_remediation_plans p ON p.id = e.plan_id
		JOIN v1_lists l ON l.id = p.list_id
		WHERE e.id = $1
		  AND e.tenant_id = $2
		  AND p.tenant_id = $2
		  AND p.list_id = $3
		"#,
	)
	.bind(export_id)
	.bind(tenant_id)
	.bind(list_id)
	.fetch_optional(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;
	let export = export.ok_or_else(|| {
		warp::reject::custom(ReacherResponseError::new(
			StatusCode::NOT_FOUND,
			"Remediation export not found",
		))
	})?;
	let format: String = export.get("format");
	if format != "csv" {
		return Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"Only format=csv is supported",
		)
		.into());
	}

	let partitions: Vec<String> = export.get("partitions");
	let headers: Vec<String> = export.get("original_headers");
	let plan_id: i64 = export.get("plan_id");
	let body = Body::wrap_stream(stream::unfold(
		ExportDownloadState {
			pg_pool,
			tenant_id,
			plan_id,
			partitions,
			headers,
			last_row_index: -1,
			header_sent: false,
		},
		|mut state| async move {
			if !state.header_sent {
				state.header_sent = true;
				return Some((
					Ok::<Bytes, io::Error>(Bytes::from(render_export_header(&state.headers))),
					state,
				));
			}

			loop {
				match fetch_export_batch(
					&state.pg_pool,
					state.tenant_id,
					state.plan_id,
					state.last_row_index,
				)
				.await
				{
					Ok(rows) if rows.is_empty() => return None,
					Ok(rows) => {
						state.last_row_index = rows
							.last()
							.map(|row| row.row_index)
							.unwrap_or(state.last_row_index);
						let mut chunk = Vec::new();
						for row in rows {
							if export_partition_includes(
								&state.partitions,
								&row.classification,
								&row.original_email,
								&row.effective_email,
							) {
								chunk.extend_from_slice(&render_export_row(&state.headers, &row));
							}
						}
						if !chunk.is_empty() {
							return Some((Ok(Bytes::from(chunk)), state));
						}
					}
					Err(err) => return Some((Err(io::Error::other(err.to_string())), state)),
				}
			}
		},
	));

	let response = warp::http::Response::builder()
		.header("Content-Type", "text/csv")
		.header(
			"Content-Disposition",
			format!(
				"attachment; filename=\"list_{}_remediation_export_{}.csv\"",
				list_id, export_id
			),
		)
		.body(body)
		.map_err(|err| ReacherResponseError::new(StatusCode::INTERNAL_SERVER_ERROR, err))
		.map_err(warp::reject::custom)?;

	Ok(response)
}

async fn load_list_context(
	pg_pool: &PgPool,
	list_id: i32,
	tenant_id: uuid::Uuid,
) -> Result<ListContext, ReacherResponseError> {
	let row = sqlx::query(
		r#"
		SELECT id, job_id, status::TEXT AS status, total_rows, email_column, original_data
		FROM v1_lists
		WHERE id = $1 AND tenant_id = $2
		"#,
	)
	.bind(list_id)
	.bind(tenant_id)
	.fetch_optional(pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;
	let row =
		row.ok_or_else(|| ReacherResponseError::new(StatusCode::NOT_FOUND, "List not found"))?;

	Ok(ListContext {
		id: row.get("id"),
		job_id: row.get("job_id"),
		status: row.get("status"),
		total_rows: row.get("total_rows"),
		email_column: row.get("email_column"),
		original_data: row.get("original_data"),
	})
}

fn ensure_plan_allowed(
	status: &str,
	total_rows: i32,
	processed_rows: i64,
	allow_partial: bool,
) -> Result<(), warp::Rejection> {
	if allow_partial || status == "completed" || processed_rows >= i64::from(total_rows) {
		return Ok(());
	}

	Err(ReacherResponseError::new(StatusCode::BAD_REQUEST, "List is still processing").into())
}

async fn load_task_rows(
	pg_pool: &PgPool,
	list_id: i32,
	tenant_id: uuid::Uuid,
) -> Result<Vec<RawTaskRow>, ReacherResponseError> {
	let rows = sqlx::query(
		r#"
		SELECT
			tr.id,
			(extra->>'row_index')::INTEGER AS row_index,
			tr.payload,
			tr.result,
			tr.error,
			tr.canonical_email,
			tr.is_duplicate,
			COALESCE(se.id IS NOT NULL, false) AS is_suppressed,
			se.reason_code AS suppression_reason_code,
			tr.recommendation,
			tr.recommendation_action,
			tr.policy_decision,
			tr.score_category,
			tr.safe_to_send,
			tr.reason_codes,
			tr.completed_at
		FROM v1_task_result tr
		LEFT JOIN LATERAL (
			SELECT id, reason_code
			FROM v1_suppression_entries se
			WHERE se.tenant_id = tr.tenant_id
			  AND se.status = 'active'
			  AND se.canonical_email = tr.canonical_email
			ORDER BY se.created_at DESC
			LIMIT 1
		) se ON true
		WHERE tr.tenant_id = $2
		  AND (tr.extra->>'list_id')::INTEGER = $1
		ORDER BY (tr.extra->>'row_index')::INTEGER ASC
		"#,
	)
	.bind(list_id)
	.bind(tenant_id)
	.fetch_all(pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	Ok(rows
		.into_iter()
		.map(|row| RawTaskRow {
			id: row.get("id"),
			row_index: row.get("row_index"),
			payload: row.get("payload"),
			result: row.get("result"),
			error: row.get("error"),
			canonical_email: row.get("canonical_email"),
			is_duplicate: row.get::<Option<bool>, _>("is_duplicate").unwrap_or(false),
			is_suppressed: row.get::<Option<bool>, _>("is_suppressed").unwrap_or(false),
			suppression_reason_code: row.get("suppression_reason_code"),
			recommendation: row.get("recommendation"),
			recommendation_action: row.get("recommendation_action"),
			policy_decision: row.get("policy_decision"),
			score_category: row.get("score_category"),
			safe_to_send: row.get("safe_to_send"),
			reason_codes: row.get("reason_codes"),
			completed_at: row.get("completed_at"),
		})
		.collect())
}

async fn insert_plan_rows(
	tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
	plan_id: i64,
	tenant_id: uuid::Uuid,
	list_id: i32,
	rows: &[PlannedRemediationRow],
) -> Result<(), ReacherResponseError> {
	for row in rows {
		sqlx::query(
			r#"
			INSERT INTO v1_remediation_rows (
				plan_id, tenant_id, list_id, task_result_id, row_index,
				classification, rule_id, confidence, original_email, effective_email,
				before, after, reasons
			)
			VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
			"#,
		)
		.bind(plan_id)
		.bind(tenant_id)
		.bind(list_id)
		.bind(row.task_result_id)
		.bind(row.row_index)
		.bind(row.classification.as_str())
		.bind(&row.rule_id)
		.bind(&row.confidence)
		.bind(&row.original_email)
		.bind(&row.effective_email)
		.bind(&row.before)
		.bind(&row.after)
		.bind(&row.reasons)
		.execute(&mut **tx)
		.await
		.map_err(ReacherResponseError::from)?;
	}

	Ok(())
}

async fn fetch_plan_response(
	pg_pool: &PgPool,
	tenant_id: uuid::Uuid,
	list_id: i32,
	plan_id: Option<i64>,
) -> Result<PlanResponse, ReacherResponseError> {
	let plan = sqlx::query(
		r#"
		SELECT id, list_id, job_id, status, rule_version, options, result_state_digest,
		       summary_counts, created_at, completed_at
		FROM v1_remediation_plans
		WHERE tenant_id = $1
		  AND list_id = $2
		  AND ($3::BIGINT IS NULL OR id = $3)
		ORDER BY created_at DESC
		LIMIT 1
		"#,
	)
	.bind(tenant_id)
	.bind(list_id)
	.bind(plan_id)
	.fetch_optional(pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;
	let plan = plan.ok_or_else(|| {
		ReacherResponseError::new(StatusCode::NOT_FOUND, "Remediation plan not found")
	})?;
	let id: i64 = plan.get("id");

	let row_records = sqlx::query(
		r#"
		SELECT id, row_index, classification, rule_id, confidence, original_email,
		       effective_email, before, after, reasons
		FROM v1_remediation_rows
		WHERE tenant_id = $1 AND plan_id = $2
		ORDER BY
			CASE classification
				WHEN 'fixed' THEN 1
				WHEN 'review' THEN 2
				WHEN 'drop' THEN 3
				ELSE 4
			END,
			row_index ASC
		LIMIT $3
		"#,
	)
	.bind(tenant_id)
	.bind(id)
	.bind(PREVIEW_ROW_LIMIT)
	.fetch_all(pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let summary_counts = plan
		.get::<Value, _>("summary_counts")
		.as_object()
		.cloned()
		.and_then(|object| serde_json::from_value(Value::Object(object)).ok())
		.unwrap_or_default();
	let created_at = plan.get::<DateTime<Utc>, _>("created_at").to_rfc3339();
	let completed_at = plan
		.get::<Option<DateTime<Utc>>, _>("completed_at")
		.map(|ts| ts.to_rfc3339());

	Ok(PlanResponse {
		id,
		list_id: plan.get("list_id"),
		job_id: plan.get("job_id"),
		status: plan.get("status"),
		rule_version: plan.get("rule_version"),
		options: plan.get("options"),
		result_state_digest: plan.get("result_state_digest"),
		summary_counts,
		created_at,
		completed_at,
		preview_rows: row_records
			.into_iter()
			.map(|row| RemediationRowResponse {
				id: row.get("id"),
				row_index: row.get("row_index"),
				classification: row.get("classification"),
				rule_id: row.get("rule_id"),
				confidence: row.get("confidence"),
				original_email: row.get("original_email"),
				effective_email: row.get("effective_email"),
				before: row.get("before"),
				after: row.get("after"),
				reasons: row.get("reasons"),
			})
			.collect(),
	})
}

async fn find_plan_id(
	pg_pool: &PgPool,
	tenant_id: uuid::Uuid,
	list_id: i32,
	plan_id: Option<i64>,
) -> Result<i64, ReacherResponseError> {
	let row = sqlx::query(
		r#"
		SELECT id
		FROM v1_remediation_plans
		WHERE tenant_id = $1
		  AND list_id = $2
		  AND ($3::BIGINT IS NULL OR id = $3)
		ORDER BY created_at DESC
		LIMIT 1
		"#,
	)
	.bind(tenant_id)
	.bind(list_id)
	.bind(plan_id)
	.fetch_optional(pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;
	row.map(|row| row.get("id")).ok_or_else(|| {
		ReacherResponseError::new(StatusCode::NOT_FOUND, "Remediation plan not found")
	})
}

async fn fetch_export_batch(
	pg_pool: &PgPool,
	tenant_id: uuid::Uuid,
	plan_id: i64,
	last_row_index: i32,
) -> Result<Vec<ExportRow>, ReacherResponseError> {
	let rows = sqlx::query(
		r#"
		SELECT row_index, classification, rule_id, confidence, original_email,
		       effective_email, after, reasons
		FROM v1_remediation_rows
		WHERE tenant_id = $1
		  AND plan_id = $2
		  AND row_index > $3
		ORDER BY row_index ASC
		LIMIT $4
		"#,
	)
	.bind(tenant_id)
	.bind(plan_id)
	.bind(last_row_index)
	.bind(EXPORT_BATCH_SIZE)
	.fetch_all(pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	Ok(rows
		.into_iter()
		.map(|row| ExportRow {
			row_index: row.get("row_index"),
			classification: row.get("classification"),
			rule_id: row.get("rule_id"),
			confidence: row.get("confidence"),
			original_email: row.get("original_email"),
			effective_email: row.get("effective_email"),
			after: row.get("after"),
			reasons: row.get("reasons"),
		})
		.collect())
}

fn original_rows_by_index(original_data: &Value) -> BTreeMap<i32, Value> {
	original_data
		.as_object()
		.into_iter()
		.flat_map(|object| object.iter())
		.filter_map(|(key, value)| key.parse::<i32>().ok().map(|index| (index, value.clone())))
		.collect()
}

fn plan_rows(
	task_rows: &[RawTaskRow],
	original_rows: &BTreeMap<i32, Value>,
	email_column: &str,
	options: &PlanOptions,
) -> Vec<PlannedRemediationRow> {
	task_rows
		.iter()
		.map(|row| {
			let before = original_rows
				.get(&row.row_index)
				.cloned()
				.unwrap_or_else(|| Value::Object(Map::new()));
			classify_row(row, before, email_column, options)
		})
		.collect()
}

fn classify_row(
	row: &RawTaskRow,
	before: Value,
	email_column: &str,
	options: &PlanOptions,
) -> PlannedRemediationRow {
	let original_email = original_email(&before, email_column)
		.or_else(|| payload_email(&row.payload).map(ToOwned::to_owned))
		.unwrap_or_default();
	let normalized_email = if options.normalize_emails {
		normalize_email(&original_email)
	} else {
		original_email.trim().to_string()
	};
	let suggested_email = if options.apply_domain_typos {
		recommendation_field(row, "suggested_email")
			.or_else(|| result_domain_suggestion(row.result.as_ref()))
	} else {
		None
	};
	let effective_email = suggested_email
		.clone()
		.or_else(|| {
			row.canonical_email
				.clone()
				.filter(|_| options.normalize_emails)
		})
		.unwrap_or_else(|| normalized_email.clone());
	let after = row_after(&before, email_column, &effective_email);
	let changed = original_email.trim() != effective_email;
	let action = recommendation_action(row);
	let mut local_reasons = Vec::new();
	let (classification, rule_id) = if original_email.trim().is_empty() {
		local_reasons.push(LocalReason {
			code: "blank_email",
			message: "Row does not contain an email address.".to_string(),
			severity: "blocking",
		});
		(RemediationClassification::Drop, "blank_email")
	} else if row.is_duplicate && options.deduplicate {
		local_reasons.push(LocalReason {
			code: "duplicate_email",
			message: "Duplicate email removed from cleaned output.".to_string(),
			severity: "warning",
		});
		(RemediationClassification::Drop, "duplicate_email")
	} else if row.is_suppressed && options.drop_suppressed {
		local_reasons.push(LocalReason {
			code: "active_suppression",
			message: row
				.suppression_reason_code
				.as_ref()
				.map(|reason| format!("Email is actively suppressed: {}", reason))
				.unwrap_or_else(|| "Email is actively suppressed.".to_string()),
			severity: "blocking",
		});
		(RemediationClassification::Drop, "active_suppression")
	} else if row.result.is_none() && row.error.is_none() {
		local_reasons.push(LocalReason {
			code: "not_processed",
			message: "Verification has not completed for this row.".to_string(),
			severity: "warning",
		});
		(RemediationClassification::Review, "not_processed")
	} else {
		classify_processed_row(row, changed, suggested_email.as_deref(), action.as_deref())
	};

	PlannedRemediationRow {
		row_index: row.row_index,
		classification,
		rule_id: rule_id.to_string(),
		confidence: recommendation_field(row, "confidence").unwrap_or_else(|| {
			match classification {
				RemediationClassification::Safe | RemediationClassification::Drop => "high",
				RemediationClassification::Fixed => "medium",
				RemediationClassification::Review => "medium",
			}
			.to_string()
		}),
		original_email,
		effective_email,
		before,
		after,
		reasons: reasons_value(row, &local_reasons),
		task_result_id: Some(row.id),
	}
}

fn classify_processed_row(
	row: &RawTaskRow,
	changed: bool,
	suggested_email: Option<&str>,
	action: Option<&str>,
) -> (RemediationClassification, &'static str) {
	match action {
		Some("fix_then_send") if changed || suggested_email.is_some() => {
			return (RemediationClassification::Fixed, "recommended_fix")
		}
		Some("send") if changed => return (RemediationClassification::Fixed, "normalized_email"),
		Some("send") => return (RemediationClassification::Safe, "recommended_send"),
		Some("send_with_caution") => {
			return (
				RemediationClassification::Review,
				"recommended_send_with_caution",
			)
		}
		Some("review") => return (RemediationClassification::Review, "recommended_review"),
		Some("suppress") => return (RemediationClassification::Drop, "recommended_suppress"),
		Some("drop") => return (RemediationClassification::Drop, "recommended_drop"),
		_ => {}
	}

	if row.error.is_some() {
		return (RemediationClassification::Review, "verification_error");
	}
	if row.score_category.as_deref() == Some("invalid") {
		return (RemediationClassification::Drop, "invalid_email");
	}
	if row.safe_to_send == Some(true) {
		if changed {
			(RemediationClassification::Fixed, "normalized_email")
		} else {
			(RemediationClassification::Safe, "safe_to_send")
		}
	} else {
		(RemediationClassification::Review, "needs_review")
	}
}

fn summarize_rows(rows: &[PlannedRemediationRow]) -> SummaryCounts {
	let mut summary = SummaryCounts::default();
	for row in rows {
		summary.increment(row.classification);
	}
	summary
}

fn result_state_digest(rows: &[RawTaskRow]) -> String {
	let mut hasher = std::collections::hash_map::DefaultHasher::new();
	for row in rows {
		row.id.hash(&mut hasher);
		row.row_index.hash(&mut hasher);
		row.result.as_ref().map(Value::to_string).hash(&mut hasher);
		row.error.hash(&mut hasher);
		row.canonical_email.hash(&mut hasher);
		row.is_duplicate.hash(&mut hasher);
		row.is_suppressed.hash(&mut hasher);
		row.suppression_reason_code.hash(&mut hasher);
		row.recommendation
			.as_ref()
			.map(Value::to_string)
			.hash(&mut hasher);
		row.recommendation_action.hash(&mut hasher);
		row.policy_decision.hash(&mut hasher);
		row.score_category.hash(&mut hasher);
		row.safe_to_send.hash(&mut hasher);
		row.reason_codes.hash(&mut hasher);
		row.completed_at.map(|ts| ts.timestamp()).hash(&mut hasher);
	}
	format!("{:016x}", hasher.finish())
}

fn original_email(row: &Value, email_column: &str) -> Option<String> {
	row.as_object()?
		.get(email_column)
		.and_then(Value::as_str)
		.map(ToOwned::to_owned)
}

fn payload_email(payload: &Value) -> Option<&str> {
	payload
		.get("input")
		.and_then(|value| {
			value
				.get("to_email")
				.and_then(Value::as_str)
				.or_else(|| value.as_str())
		})
		.or_else(|| payload.get("to_email").and_then(Value::as_str))
}

fn normalize_email(email: &str) -> String {
	let cleaned: String = email
		.chars()
		.filter(|c| {
			!c.is_control() && !matches!(c, '\u{200b}' | '\u{200c}' | '\u{200d}' | '\u{feff}')
		})
		.collect();
	let trimmed = cleaned.trim();
	match trimmed.rsplit_once('@') {
		Some((local, domain)) => format!("{}@{}", local.trim(), domain.trim().to_lowercase()),
		None => trimmed.to_string(),
	}
}

fn row_after(before: &Value, email_column: &str, effective_email: &str) -> Value {
	let mut object = before.as_object().cloned().unwrap_or_default();
	object.insert(
		email_column.to_string(),
		Value::String(effective_email.to_string()),
	);
	Value::Object(object)
}

fn recommendation_action(row: &RawTaskRow) -> Option<String> {
	row.recommendation_action
		.clone()
		.or_else(|| recommendation_field(row, "action"))
}

fn recommendation_field(row: &RawTaskRow, field: &str) -> Option<String> {
	row.recommendation
		.as_ref()
		.and_then(|value| value.get(field))
		.and_then(Value::as_str)
		.map(ToOwned::to_owned)
}

fn result_domain_suggestion(result: Option<&Value>) -> Option<String> {
	result
		.and_then(|value| value.get("score"))
		.and_then(|score| score.get("domain_suggestion"))
		.and_then(Value::as_str)
		.map(ToOwned::to_owned)
}

fn reasons_value(row: &RawTaskRow, local_reasons: &[LocalReason]) -> Value {
	let mut reasons = row
		.recommendation
		.as_ref()
		.and_then(|value| value.get("reasons"))
		.and_then(Value::as_array)
		.cloned()
		.unwrap_or_default();

	for reason in local_reasons {
		reasons.push(json!({
			"code": reason.code,
			"message": reason.message,
			"severity": reason.severity
		}));
	}

	Value::Array(reasons)
}

fn normalize_export_partitions(
	partitions: Option<Vec<String>>,
) -> Result<Vec<String>, warp::Rejection> {
	let raw = partitions.unwrap_or_else(|| vec!["safe_to_send".to_string()]);
	if raw.is_empty() {
		return Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"partitions array must not be empty",
		)
		.into());
	}

	let mut normalized = Vec::new();
	for partition in raw {
		let partition = partition.trim().to_lowercase();
		if partition.is_empty() {
			continue;
		}
		if !matches!(
			partition.as_str(),
			"all" | "safe_to_send" | "fixed" | "safe" | "review" | "drop" | "changed"
		) {
			return Err(ReacherResponseError::new(
				StatusCode::BAD_REQUEST,
				format!("Invalid remediation export partition: {}", partition),
			)
			.into());
		}
		if !normalized.contains(&partition) {
			normalized.push(partition);
		}
	}

	if normalized.is_empty() {
		return Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"partitions array must include at least one valid partition",
		)
		.into());
	}
	Ok(normalized)
}

fn export_partition_includes(
	partitions: &[String],
	classification: &str,
	original_email: &str,
	effective_email: &str,
) -> bool {
	partitions.iter().any(|partition| match partition.as_str() {
		"all" => true,
		"safe_to_send" => matches!(classification, "fixed" | "safe"),
		"changed" => original_email.trim() != effective_email,
		other => other == classification,
	})
}

fn render_export_header(headers: &[String]) -> Vec<u8> {
	let mut writer = csv::WriterBuilder::new()
		.has_headers(false)
		.from_writer(Vec::new());
	let mut row = headers.to_vec();
	row.extend([
		"remediation_classification".to_string(),
		"remediation_rule_id".to_string(),
		"remediation_confidence".to_string(),
		"original_email".to_string(),
		"effective_email".to_string(),
		"remediation_changed".to_string(),
		"remediation_reasons".to_string(),
	]);
	writer.write_record(&row).expect("csv header write");
	writer.into_inner().expect("csv header bytes")
}

fn render_export_row(headers: &[String], row: &ExportRow) -> Vec<u8> {
	let mut writer = csv::WriterBuilder::new()
		.has_headers(false)
		.from_writer(Vec::new());
	let object = row.after.as_object();
	let mut record = headers
		.iter()
		.map(|header| {
			object
				.and_then(|object| object.get(header))
				.map(value_to_cell)
				.unwrap_or_default()
		})
		.collect::<Vec<_>>();
	record.push(row.classification.clone());
	record.push(row.rule_id.clone());
	record.push(row.confidence.clone());
	record.push(row.original_email.clone());
	record.push(row.effective_email.clone());
	record.push((row.original_email.trim() != row.effective_email).to_string());
	record.push(compact_export_reason_codes(&row.reasons));
	writer.write_record(&record).expect("csv row write");
	writer.into_inner().expect("csv row bytes")
}

fn value_to_cell(value: &Value) -> String {
	match value {
		Value::Null => String::new(),
		Value::String(value) => value.clone(),
		Value::Bool(value) => value.to_string(),
		Value::Number(value) => value.to_string(),
		other => other.to_string(),
	}
}

fn compact_export_reason_codes(value: &Value) -> String {
	value
		.as_array()
		.into_iter()
		.flat_map(|reasons| reasons.iter())
		.filter_map(|reason| reason.get("code").and_then(Value::as_str))
		.collect::<Vec<_>>()
		.join("|")
}

/// POST /v1/lists/{list_id}/remediation-plan
pub fn v1_create_remediation_plan(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "lists" / i32 / "remediation-plan")
		.and(warp::post())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::body::json())
		.and_then(create_plan_handler)
		.with(warp::log(LOG_TARGET))
}

/// GET /v1/lists/{list_id}/remediation-plan
pub fn v1_get_remediation_plan(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "lists" / i32 / "remediation-plan")
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and_then(get_plan_handler)
		.with(warp::log(LOG_TARGET))
}

/// POST /v1/lists/{list_id}/remediation-exports
pub fn v1_create_remediation_export(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "lists" / i32 / "remediation-exports")
		.and(warp::post())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::body::json())
		.and_then(create_export_handler)
		.with(warp::log(LOG_TARGET))
}

/// GET /v1/lists/{list_id}/remediation-exports/{export_id}/download
pub fn v1_download_remediation_export(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "lists" / i32 / "remediation-exports" / i64 / "download")
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and_then(download_export_handler)
		.with(warp::log(LOG_TARGET))
}

#[cfg(test)]
mod tests {
	use super::*;

	fn raw_row(original: &str) -> (RawTaskRow, Value) {
		(
			RawTaskRow {
				id: 10,
				row_index: 2,
				payload: json!({"input": {"to_email": original}}),
				result: Some(json!({"score": {"category": "valid", "safe_to_send": true}})),
				error: None,
				canonical_email: None,
				is_duplicate: false,
				is_suppressed: false,
				suppression_reason_code: None,
				recommendation: Some(
					json!({"action": "send", "confidence": "high", "reasons": []}),
				),
				recommendation_action: Some("send".to_string()),
				policy_decision: Some("send".to_string()),
				score_category: Some("valid".to_string()),
				safe_to_send: Some(true),
				reason_codes: None,
				completed_at: None,
			},
			json!({"email": original, "name": "Ada"}),
		)
	}

	#[test]
	fn classifies_domain_typo_as_fixed() {
		let (mut row, before) = raw_row("user@gmial.com");
		row.recommendation = Some(json!({
			"action": "fix_then_send",
			"confidence": "high",
			"suggested_email": "user@gmail.com",
			"reasons": [{"code": "possible_domain_typo"}]
		}));
		row.recommendation_action = Some("fix_then_send".to_string());

		let plan = classify_row(&row, before, "email", &PlanOptions::default());

		assert_eq!(plan.classification, RemediationClassification::Fixed);
		assert_eq!(plan.effective_email, "user@gmail.com");
		assert_eq!(plan.rule_id, "recommended_fix");
	}

	#[test]
	fn normalizes_safe_email_as_fixed() {
		let (row, before) = raw_row(" User@Example.COM ");

		let plan = classify_row(&row, before, "email", &PlanOptions::default());

		assert_eq!(plan.classification, RemediationClassification::Fixed);
		assert_eq!(plan.effective_email, "User@example.com");
		assert_eq!(plan.after["email"], "User@example.com");
	}

	#[test]
	fn drops_duplicate_rows_by_default() {
		let (mut row, before) = raw_row("dupe@example.com");
		row.is_duplicate = true;

		let plan = classify_row(&row, before, "email", &PlanOptions::default());

		assert_eq!(plan.classification, RemediationClassification::Drop);
		assert_eq!(plan.rule_id, "duplicate_email");
		assert!(plan.reasons.to_string().contains("duplicate_email"));
	}

	#[test]
	fn drops_active_suppression() {
		let (mut row, before) = raw_row("blocked@example.com");
		row.is_suppressed = true;
		row.suppression_reason_code = Some("complaint".to_string());

		let plan = classify_row(&row, before, "email", &PlanOptions::default());

		assert_eq!(plan.classification, RemediationClassification::Drop);
		assert_eq!(plan.rule_id, "active_suppression");
		assert!(plan.reasons.to_string().contains("complaint"));
	}

	#[test]
	fn refuses_incomplete_lists_without_partial_option() {
		let result = ensure_plan_allowed("processing", 10, 4, false);

		assert!(result.is_err());
	}

	#[test]
	fn permits_incomplete_lists_with_partial_option() {
		let result = ensure_plan_allowed("processing", 10, 4, true);

		assert!(result.is_ok());
	}

	#[test]
	fn safe_to_send_export_includes_fixed_and_safe_rows() {
		let partitions = normalize_export_partitions(Some(vec!["safe_to_send".to_string()]))
			.expect("partition should normalize");

		assert!(export_partition_includes(
			&partitions,
			"fixed",
			"user@gmial.com",
			"user@gmail.com"
		));
		assert!(export_partition_includes(
			&partitions,
			"safe",
			"user@example.com",
			"user@example.com"
		));
		assert!(!export_partition_includes(
			&partitions,
			"review",
			"user@example.com",
			"user@example.com"
		));
	}

	#[test]
	fn export_row_renders_repaired_after_values() {
		let row = ExportRow {
			row_index: 1,
			classification: "fixed".to_string(),
			rule_id: "recommended_fix".to_string(),
			confidence: "high".to_string(),
			original_email: "user@gmial.com".to_string(),
			effective_email: "user@gmail.com".to_string(),
			after: json!({"email": "user@gmail.com", "name": "Ada"}),
			reasons: json!([{"code": "possible_domain_typo"}]),
		};

		let rendered = String::from_utf8(render_export_row(
			&["name".to_string(), "email".to_string()],
			&row,
		))
		.expect("csv should be utf8");

		assert!(rendered.contains("Ada,user@gmail.com,fixed"));
		assert!(rendered.contains("user@gmial.com,user@gmail.com,true"));
		assert!(rendered.contains("possible_domain_typo"));
	}
}
