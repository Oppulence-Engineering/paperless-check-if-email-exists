use crate::config::BackendConfig;
use crate::finder::require_tenant_id;
use crate::http::shared::paging::clamp_page;
use crate::http::v1::bulk::with_worker_db;
use crate::http::{check_scope, resolve_tenant, ReacherResponseError};
use crate::tenant::context::{scope, TenantContext};
use check_if_email_exists::LOG_TARGET;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use warp::Filter;

#[derive(Debug, Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
struct Query {
	limit: Option<i64>,
	offset: Option<i64>,
}

#[derive(Debug, Serialize)]
struct ListItem {
	id: i32,
	job_id: i32,
	total_processed: i64,
	name: String,
	original_filename: String,
	status: String,
	total_rows: i32,
	email_column: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	source_key: Option<String>,
	created_at: String,
	completed_at: Option<String>,
}

#[derive(Debug, Serialize)]
struct Response {
	lists: Vec<ListItem>,
	total: i64,
}

async fn http_handler(
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	query: Query,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::LISTS)?;

	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	let (limit, offset) = clamp_page(query.limit, query.offset);
	let rows = sqlx::query(
		r#"
		SELECT
			id,
            job_id,
            (SELECT COUNT(*) FROM v1_task_result t WHERE t.job_id=l.job_id AND t.tenant_id=l.tenant_id AND t.extra->>'list_id'=l.id::text AND t.task_state IN ('completed','failed','dead_lettered','cancelled') AND (t.dispatch_id IS NULL OR t.completion_done) AND (t.result IS NOT NULL OR t.error IS NOT NULL OR t.task_state='cancelled')) AS total_processed,
			name,
			original_filename,
			status::TEXT AS status,
			total_rows,
			email_column,
			source_key,
			created_at,
			completed_at
		FROM v1_lists l
		WHERE tenant_id = $1
		ORDER BY created_at DESC
		LIMIT $2 OFFSET $3
		"#,
	)
	.bind(tenant_id)
	.bind(limit)
	.bind(offset)
	.fetch_all(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;
	let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM v1_lists WHERE tenant_id = $1")
		.bind(tenant_id)
		.fetch_one(&pg_pool)
		.await
		.map_err(ReacherResponseError::from)?;

	Ok(warp::reply::json(&Response {
		lists: rows
			.into_iter()
			.map(|row| ListItem {
				id: sqlx::Row::get(&row, "id"),
				job_id: sqlx::Row::get(&row, "job_id"),
				total_processed: sqlx::Row::get(&row, "total_processed"),
				name: sqlx::Row::get(&row, "name"),
				original_filename: sqlx::Row::get(&row, "original_filename"),
				status: sqlx::Row::get(&row, "status"),
				total_rows: sqlx::Row::get(&row, "total_rows"),
				email_column: sqlx::Row::get(&row, "email_column"),
				source_key: sqlx::Row::get(&row, "source_key"),
				created_at: sqlx::Row::get::<DateTime<Utc>, _>(&row, "created_at").to_rfc3339(),
				completed_at: sqlx::Row::get::<Option<DateTime<Utc>>, _>(&row, "completed_at")
					.map(|value| value.to_rfc3339()),
			})
			.collect(),
		total,
	}))
}

/// GET /v1/lists
#[utoipa::path(
	get,
	path = "/v1/lists",
	tag = "v1",
	params(Query),
	responses((status = 200, description = "List resources"))
)]
pub fn v1_list_lists(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "lists")
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::query::<Query>())
		.and_then(http_handler)
		.with(warp::log(LOG_TARGET))
}
