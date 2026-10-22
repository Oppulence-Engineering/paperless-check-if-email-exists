use crate::config::BackendConfig;
use crate::finder::require_tenant_id;
use crate::http::v1::bulk::with_worker_db;
use crate::http::{check_scope, resolve_tenant, ReacherResponseError};
use crate::tenant::context::{scope, TenantContext};
use check_if_email_exists::LOG_TARGET;
use serde::Serialize;
use sqlx::{PgPool, Row};
use std::sync::Arc;
use warp::http::StatusCode;
use warp::Filter;

#[derive(Debug, Serialize)]
pub struct Summary {
	pub total_valid: i64,
	pub total_risky: i64,
	pub total_unknown: i64,
	pub total_invalid: i64,
	pub total_processed: i64,
}

#[derive(Debug, Serialize)]
struct Response {
	id: i32,
	job_id: i32,
	name: String,
	original_filename: String,
	status: String,
	total_rows: i32,
	rejected_rows: Option<i32>,
	email_column: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	source_key: Option<String>,
	summary: Summary,
	#[serde(skip_serializing_if = "Option::is_none")]
	policy_id: Option<i64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	unique_emails: Option<i32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	deduplicated_count: Option<i32>,
}

async fn http_handler(
	list_id: i32,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::LISTS)?;

	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	let row = sqlx::query(
		r#"
		SELECT id, job_id, name, original_filename, status::TEXT AS status, total_rows, email_column, source_key,
			   unique_emails, deduplicated_count, policy_id
		FROM v1_lists
		WHERE id = $1 AND tenant_id = $2
		"#,
	)
	.bind(list_id)
	.bind(tenant_id)
	.fetch_optional(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;
	let row = row.ok_or_else(|| {
		warp::reject::custom(ReacherResponseError::new(
			StatusCode::NOT_FOUND,
			"List not found",
		))
	})?;

	let summary = list_summary(&pg_pool, list_id)
		.await
		.map_err(warp::reject::custom)?;

	let total_rows = row.get::<i32, _>("total_rows");

	Ok(warp::reply::json(&Response {
		id: row.get("id"),
		job_id: row.get("job_id"),
		name: row.get("name"),
		original_filename: row.get("original_filename"),
		status: row.get("status"),
		total_rows,
		rejected_rows: row.get::<Option<i32>, _>("unique_emails").map(|unique| {
			total_rows - unique - row.get::<Option<i32>, _>("deduplicated_count").unwrap_or(0)
		}),
		email_column: row.get("email_column"),
		source_key: row.get("source_key"),
		summary,
		policy_id: row.get("policy_id"),
		unique_emails: row.get("unique_emails"),
		deduplicated_count: row.get::<Option<i32>, _>("deduplicated_count"),
	}))
}

pub async fn list_summary(pg_pool: &PgPool, list_id: i32) -> Result<Summary, ReacherResponseError> {
	let row = sqlx::query(
		r#"
		SELECT
			COUNT(*) FILTER (WHERE score_category = 'valid') AS total_valid,
			COUNT(*) FILTER (WHERE score_category = 'risky') AS total_risky,
			COUNT(*) FILTER (WHERE score_category = 'unknown') AS total_unknown,
			COUNT(*) FILTER (WHERE score_category = 'invalid') AS total_invalid,
			COUNT(*) FILTER (WHERE task_state IN ('completed','failed','dead_lettered','cancelled') AND (dispatch_id IS NULL OR completion_done) AND (result IS NOT NULL OR error IS NOT NULL OR task_state = 'cancelled')) AS total_processed
		FROM v1_task_result t
        JOIN v1_lists l ON l.job_id=t.job_id AND l.tenant_id=t.tenant_id
        WHERE l.id=$1 AND t.extra->>'list_id'=l.id::text
		"#,
	)
	.bind(list_id)
	.fetch_one(pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	Ok(Summary {
		total_valid: row.get::<Option<i64>, _>("total_valid").unwrap_or(0),
		total_risky: row.get::<Option<i64>, _>("total_risky").unwrap_or(0),
		total_unknown: row.get::<Option<i64>, _>("total_unknown").unwrap_or(0),
		total_invalid: row.get::<Option<i64>, _>("total_invalid").unwrap_or(0),
		total_processed: row.get::<Option<i64>, _>("total_processed").unwrap_or(0),
	})
}

/// GET /v1/lists/{list_id}
#[utoipa::path(
	get,
	path = "/v1/lists/{list_id}",
	tag = "v1",
	params(("list_id" = i32, Path, description = "List identifier")),
	responses((status = 200, description = "List detail"))
)]
pub fn v1_get_list(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "lists" / i32)
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and_then(http_handler)
		.with(warp::log(LOG_TARGET))
}
