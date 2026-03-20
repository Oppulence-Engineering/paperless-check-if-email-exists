use crate::config::BackendConfig;
use crate::finder::require_tenant_id;
use crate::http::v1::bulk::with_worker_db;
use crate::http::{resolve_tenant, ReacherResponseError};
use crate::tenant::context::TenantContext;
use check_if_email_exists::LOG_TARGET;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use std::sync::Arc;
use warp::Filter;

#[derive(Debug, Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
struct Query {
	limit: Option<i64>,
	offset: Option<i64>,
	reason: Option<String>,
}

#[derive(Debug, Serialize)]
struct SuppressionEntry {
	id: i32,
	email: String,
	reason: String,
	source: Option<String>,
	notes: Option<String>,
	created_at: String,
}

#[derive(Debug, Serialize)]
struct Response {
	entries: Vec<SuppressionEntry>,
	total: i64,
}

async fn http_handler(
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	query: Query,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	let limit = query.limit.unwrap_or(50).min(200);
	let offset = query.offset.unwrap_or(0);

	let (entries, total) = if let Some(reason) = &query.reason {
		let rows = sqlx::query(
			r#"
			SELECT id, email, reason::TEXT, source, notes, created_at
			FROM v1_suppression_entries
			WHERE tenant_id = $1 AND reason = $2::suppression_reason
			ORDER BY created_at DESC
			LIMIT $3 OFFSET $4
			"#,
		)
		.bind(tenant_id)
		.bind(reason)
		.bind(limit)
		.bind(offset)
		.fetch_all(&pg_pool)
		.await
		.map_err(ReacherResponseError::from)?;

		let total: i64 = sqlx::query_scalar(
			"SELECT COUNT(*) FROM v1_suppression_entries WHERE tenant_id = $1 AND reason = $2::suppression_reason",
		)
		.bind(tenant_id)
		.bind(reason)
		.fetch_one(&pg_pool)
		.await
		.map_err(ReacherResponseError::from)?;

		(rows, total)
	} else {
		let rows = sqlx::query(
			r#"
			SELECT id, email, reason::TEXT, source, notes, created_at
			FROM v1_suppression_entries
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

		let total: i64 =
			sqlx::query_scalar("SELECT COUNT(*) FROM v1_suppression_entries WHERE tenant_id = $1")
				.bind(tenant_id)
				.fetch_one(&pg_pool)
				.await
				.map_err(ReacherResponseError::from)?;

		(rows, total)
	};

	let entries: Vec<SuppressionEntry> = entries
		.into_iter()
		.map(|row| SuppressionEntry {
			id: row.get("id"),
			email: row.get("email"),
			reason: row.get("reason"),
			source: row.get("source"),
			notes: row.get("notes"),
			created_at: row
				.get::<chrono::DateTime<chrono::Utc>, _>("created_at")
				.to_rfc3339(),
		})
		.collect();

	Ok(warp::reply::json(&Response { entries, total }))
}

/// GET /v1/suppressions
#[utoipa::path(
	get,
	path = "/v1/suppressions",
	tag = "v1",
	params(Query),
	responses((status = 200, description = "Suppression list"))
)]
pub fn v1_list_suppressions(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "suppressions")
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::query::<Query>())
		.and_then(http_handler)
		.with(warp::log(LOG_TARGET))
}
