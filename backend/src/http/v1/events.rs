use crate::config::BackendConfig;
use crate::http::v1::bulk::with_worker_db;
use crate::http::{check_scope, resolve_tenant, ReacherResponseError};
use crate::tenant::context::{scope, TenantContext};
use check_if_email_exists::LOG_TARGET;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use std::sync::Arc;
use warp::Filter;

#[derive(Debug, Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
struct Query {
	limit: Option<i64>,
	offset: Option<i64>,
	event_type: Option<String>,
	actor: Option<String>,
	job_id: Option<i32>,
	since: Option<String>,
	until: Option<String>,
}

#[derive(Debug, Serialize)]
struct Event {
	id: i64,
	job_id: i32,
	task_id: Option<i32>,
	event_type: String,
	event_data: Option<serde_json::Value>,
	actor: Option<String>,
	created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
struct Response {
	events: Vec<Event>,
	total: i64,
}

async fn http_handler(
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	query: Query,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::BULK)?;

	let limit = query.limit.unwrap_or(50).min(200);
	let offset = query.offset.unwrap_or(0);

	let since = query
		.since
		.as_deref()
		.and_then(|s| DateTime::parse_from_rfc3339(s).ok())
		.map(|dt| dt.with_timezone(&Utc));
	let until = query
		.until
		.as_deref()
		.and_then(|s| DateTime::parse_from_rfc3339(s).ok())
		.map(|dt| dt.with_timezone(&Utc));

	let total: i64 = sqlx::query_scalar(
		r#"
		SELECT COUNT(*) FROM job_events je
		JOIN v1_bulk_job j ON j.id = je.job_id
		WHERE (j.tenant_id = $1 OR $1 IS NULL)
		  AND ($2::TEXT IS NULL OR je.event_type = $2)
		  AND ($3::TEXT IS NULL OR je.actor = $3)
		  AND ($4::INTEGER IS NULL OR je.job_id = $4)
		  AND ($5::TIMESTAMPTZ IS NULL OR je.created_at >= $5)
		  AND ($6::TIMESTAMPTZ IS NULL OR je.created_at <= $6)
		"#,
	)
	.bind(tenant_ctx.tenant_id)
	.bind(&query.event_type)
	.bind(&query.actor)
	.bind(query.job_id)
	.bind(since)
	.bind(until)
	.fetch_one(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let rows = sqlx::query(
		r#"
		SELECT je.id, je.job_id, je.task_id, je.event_type, je.event_data, je.actor, je.created_at
		FROM job_events je
		JOIN v1_bulk_job j ON j.id = je.job_id
		WHERE (j.tenant_id = $1 OR $1 IS NULL)
		  AND ($2::TEXT IS NULL OR je.event_type = $2)
		  AND ($3::TEXT IS NULL OR je.actor = $3)
		  AND ($4::INTEGER IS NULL OR je.job_id = $4)
		  AND ($5::TIMESTAMPTZ IS NULL OR je.created_at >= $5)
		  AND ($6::TIMESTAMPTZ IS NULL OR je.created_at <= $6)
		ORDER BY je.created_at DESC
		LIMIT $7 OFFSET $8
		"#,
	)
	.bind(tenant_ctx.tenant_id)
	.bind(&query.event_type)
	.bind(&query.actor)
	.bind(query.job_id)
	.bind(since)
	.bind(until)
	.bind(limit)
	.bind(offset)
	.fetch_all(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let events: Vec<Event> = rows
		.iter()
		.map(|r| Event {
			id: r.get("id"),
			job_id: r.get("job_id"),
			task_id: r.get("task_id"),
			event_type: r.get("event_type"),
			event_data: r.get("event_data"),
			actor: r.get("actor"),
			created_at: r.get("created_at"),
		})
		.collect();

	Ok(warp::reply::json(&Response { events, total }))
}

/// GET /v1/events
///
/// Returns a paginated, filterable audit log of all job events for the tenant.
#[utoipa::path(
	get,
	path = "/v1/events",
	tag = "Events",
	params(Query),
	responses((status = 200, description = "Audit log events"))
)]
pub fn v1_list_events(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "events")
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::query::<Query>())
		.and_then(http_handler)
		.with(warp::log(LOG_TARGET))
}
