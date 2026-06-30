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
	reason: Option<String>,
	status: Option<String>,
	source_type: Option<String>,
	source_ref: Option<String>,
	include_expired: Option<bool>,
}

#[derive(Debug, Serialize)]
struct SuppressionEntry {
	id: i32,
	email: String,
	canonical_email: String,
	status: String,
	reason: String,
	reason_code: Option<String>,
	reason_detail: Option<String>,
	source: Option<String>,
	source_type: Option<String>,
	source_ref: Option<String>,
	notes: Option<String>,
	created_by: Option<String>,
	expires_at: Option<String>,
	last_seen_at: Option<String>,
	metadata: Value,
	created_at: String,
	updated_at: String,
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
	let status_filter = normalize_status_filter(query.status.as_deref())?;
	let include_expired = query.include_expired.unwrap_or(false);

	let rows = sqlx::query(
		r#"
		SELECT
			id, email, canonical_email, status, reason::TEXT AS reason,
			reason_code, reason_detail, source, source_type, source_ref, notes,
			created_by, expires_at, last_seen_at, metadata, created_at, updated_at
		FROM v1_suppression_entries
		WHERE tenant_id = $1
		  AND ($2::TEXT IS NULL OR status = $2)
		  AND ($3::TEXT IS NULL OR reason::TEXT = $3)
		  AND ($4::TEXT IS NULL OR source_type = $4)
		  AND ($5::TEXT IS NULL OR source_ref = $5)
		  AND ($6::BOOLEAN = true OR expires_at IS NULL OR expires_at > NOW())
		ORDER BY created_at DESC
		LIMIT $7 OFFSET $8
		"#,
	)
	.bind(tenant_id)
	.bind(&status_filter)
	.bind(&query.reason)
	.bind(&query.source_type)
	.bind(&query.source_ref)
	.bind(include_expired)
	.bind(limit)
	.bind(offset)
	.fetch_all(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let total: i64 = sqlx::query_scalar(
		r#"
		SELECT COUNT(*)
		FROM v1_suppression_entries
		WHERE tenant_id = $1
		  AND ($2::TEXT IS NULL OR status = $2)
		  AND ($3::TEXT IS NULL OR reason::TEXT = $3)
		  AND ($4::TEXT IS NULL OR source_type = $4)
		  AND ($5::TEXT IS NULL OR source_ref = $5)
		  AND ($6::BOOLEAN = true OR expires_at IS NULL OR expires_at > NOW())
		"#,
	)
	.bind(tenant_id)
	.bind(&status_filter)
	.bind(&query.reason)
	.bind(&query.source_type)
	.bind(&query.source_ref)
	.bind(include_expired)
	.fetch_one(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let entries: Vec<SuppressionEntry> = rows
		.into_iter()
		.map(|row| SuppressionEntry {
			id: row.get("id"),
			email: row.get("email"),
			canonical_email: row.get("canonical_email"),
			status: row.get("status"),
			reason: row.get("reason"),
			reason_code: row.get("reason_code"),
			reason_detail: row.get("reason_detail"),
			source: row.get("source"),
			source_type: row.get("source_type"),
			source_ref: row.get("source_ref"),
			notes: row.get("notes"),
			created_by: row.get("created_by"),
			expires_at: row
				.get::<Option<chrono::DateTime<chrono::Utc>>, _>("expires_at")
				.map(|ts| ts.to_rfc3339()),
			last_seen_at: row
				.get::<Option<chrono::DateTime<chrono::Utc>>, _>("last_seen_at")
				.map(|ts| ts.to_rfc3339()),
			metadata: row.get("metadata"),
			created_at: row
				.get::<chrono::DateTime<chrono::Utc>, _>("created_at")
				.to_rfc3339(),
			updated_at: row
				.get::<chrono::DateTime<chrono::Utc>, _>("updated_at")
				.to_rfc3339(),
		})
		.collect();

	Ok(warp::reply::json(&Response { entries, total }))
}

fn normalize_status_filter(status: Option<&str>) -> Result<Option<String>, warp::Rejection> {
	let Some(status) = status else {
		return Ok(Some("active".to_string()));
	};
	let status = status.trim().to_lowercase();
	if status == "all" {
		return Ok(None);
	}
	if matches!(status.as_str(), "active" | "inactive" | "merged") {
		return Ok(Some(status));
	}
	Err(ReacherResponseError::new(
		StatusCode::BAD_REQUEST,
		"Invalid status. Must be one of: active, inactive, merged, all",
	)
	.into())
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn defaults_to_active_suppressions() {
		assert_eq!(
			normalize_status_filter(None).expect("status should normalize"),
			Some("active".to_string())
		);
	}

	#[test]
	fn all_status_disables_status_filter() {
		assert_eq!(
			normalize_status_filter(Some("all")).expect("status should normalize"),
			None
		);
	}

	#[test]
	fn rejects_unknown_status_filter() {
		assert!(normalize_status_filter(Some("deleted")).is_err());
	}
}
