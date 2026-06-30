use crate::config::BackendConfig;
use crate::finder::require_tenant_id;
use crate::http::v1::bulk::with_worker_db;
use crate::http::{resolve_tenant, ReacherResponseError};
use crate::tenant::context::TenantContext;
use check_if_email_exists::LOG_TARGET;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{PgPool, Row};
use std::sync::Arc;
use warp::http::StatusCode;
use warp::Filter;

#[derive(Debug, Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
struct Query {
	limit: Option<i64>,
	offset: Option<i64>,
}

#[derive(Debug, Serialize)]
struct SuppressionEvent {
	id: i64,
	entry_id: Option<i32>,
	canonical_email: String,
	event_type: String,
	from_status: Option<String>,
	to_status: Option<String>,
	reason_code: Option<String>,
	reason_detail: Option<String>,
	source_type: Option<String>,
	source_ref: Option<String>,
	actor_type: String,
	actor_id: Option<String>,
	metadata: Value,
	created_at: String,
}

#[derive(Debug, Serialize)]
struct Response {
	events: Vec<SuppressionEvent>,
	total: i64,
}

async fn http_handler(
	id: i32,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	query: Query,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	let exists: Option<i32> = sqlx::query_scalar(
		"SELECT id FROM v1_suppression_entries WHERE id = $1 AND tenant_id = $2",
	)
	.bind(id)
	.bind(tenant_id)
	.fetch_optional(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;
	if exists.is_none() {
		return Err(ReacherResponseError::new(
			StatusCode::NOT_FOUND,
			"Suppression entry not found",
		)
		.into());
	}

	let limit = query.limit.unwrap_or(50).min(200);
	let offset = query.offset.unwrap_or(0);
	let rows = sqlx::query(
		r#"
		SELECT id, entry_id, canonical_email, event_type, from_status, to_status,
		       reason_code, reason_detail, source_type, source_ref, actor_type,
		       actor_id, metadata, created_at
		FROM v1_suppression_events
		WHERE tenant_id = $1 AND entry_id = $2
		ORDER BY created_at DESC
		LIMIT $3 OFFSET $4
		"#,
	)
	.bind(tenant_id)
	.bind(id)
	.bind(limit)
	.bind(offset)
	.fetch_all(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let total: i64 = sqlx::query_scalar(
		"SELECT COUNT(*) FROM v1_suppression_events WHERE tenant_id = $1 AND entry_id = $2",
	)
	.bind(tenant_id)
	.bind(id)
	.fetch_one(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let events = rows
		.into_iter()
		.map(|row| SuppressionEvent {
			id: row.get("id"),
			entry_id: row.get("entry_id"),
			canonical_email: row.get("canonical_email"),
			event_type: row.get("event_type"),
			from_status: row.get("from_status"),
			to_status: row.get("to_status"),
			reason_code: row.get("reason_code"),
			reason_detail: row.get("reason_detail"),
			source_type: row.get("source_type"),
			source_ref: row.get("source_ref"),
			actor_type: row.get("actor_type"),
			actor_id: row.get("actor_id"),
			metadata: row.get("metadata"),
			created_at: row
				.get::<chrono::DateTime<chrono::Utc>, _>("created_at")
				.to_rfc3339(),
		})
		.collect();

	Ok(warp::reply::json(&Response { events, total }))
}

/// GET /v1/suppressions/{id}/events
pub fn v1_list_suppression_events(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "suppressions" / i32 / "events")
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::query::<Query>())
		.and_then(http_handler)
		.with(warp::log(LOG_TARGET))
}
