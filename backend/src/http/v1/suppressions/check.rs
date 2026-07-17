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
use warp::Filter;

#[derive(Debug, Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
struct Query {
	email: String,
}

#[derive(Debug, Serialize)]
struct Response {
	suppressed: bool,
	id: Option<i32>,
	canonical_email: Option<String>,
	status: Option<String>,
	reason: Option<String>,
	reason_code: Option<String>,
	reason_detail: Option<String>,
	source: Option<String>,
	source_type: Option<String>,
	source_ref: Option<String>,
	created_by: Option<String>,
	expires_at: Option<String>,
	metadata: Option<Value>,
	created_at: Option<String>,
}

async fn http_handler(
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	query: Query,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	let email = query.email.trim().to_lowercase();

	let row = sqlx::query(
		r#"
		UPDATE v1_suppression_entries
		SET last_seen_at = NOW(), updated_at = NOW()
		WHERE id = (
			SELECT id
			FROM v1_suppression_entries
			WHERE tenant_id = $1
			  AND canonical_email = $2
			  AND status = 'active'
			  AND (expires_at IS NULL OR expires_at > NOW())
			ORDER BY created_at DESC
			LIMIT 1
		)
		RETURNING id, canonical_email, status, reason::TEXT AS reason, reason_code,
		          reason_detail, source, source_type, source_ref, created_by, expires_at,
		          metadata, created_at
		"#,
	)
	.bind(tenant_id)
	.bind(&email)
	.fetch_optional(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let response = match row {
		Some(row) => Response {
			suppressed: true,
			id: Some(row.get("id")),
			canonical_email: Some(row.get("canonical_email")),
			status: Some(row.get("status")),
			reason: Some(row.get("reason")),
			reason_code: row.get("reason_code"),
			reason_detail: row.get("reason_detail"),
			source: row.get("source"),
			source_type: row.get("source_type"),
			source_ref: row.get("source_ref"),
			created_by: row.get("created_by"),
			expires_at: row
				.get::<Option<chrono::DateTime<chrono::Utc>>, _>("expires_at")
				.map(|ts| ts.to_rfc3339()),
			metadata: row.get("metadata"),
			created_at: Some(
				row.get::<chrono::DateTime<chrono::Utc>, _>("created_at")
					.to_rfc3339(),
			),
		},
		None => Response {
			suppressed: false,
			id: None,
			canonical_email: None,
			status: None,
			reason: None,
			reason_code: None,
			reason_detail: None,
			source: None,
			source_type: None,
			source_ref: None,
			created_by: None,
			expires_at: None,
			metadata: None,
			created_at: None,
		},
	};

	Ok(warp::reply::json(&response))
}

/// GET /v1/suppressions/check
#[utoipa::path(
	get,
	path = "/v1/suppressions/check",
	tag = "v1",
	params(Query),
	responses((status = 200, description = "Suppression check result"))
)]
pub fn v1_check_suppression(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "suppressions" / "check")
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::query::<Query>())
		.and_then(http_handler)
		.with(warp::log(LOG_TARGET))
}
