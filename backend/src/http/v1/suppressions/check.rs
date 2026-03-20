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
	email: String,
}

#[derive(Debug, Serialize)]
struct Response {
	suppressed: bool,
	reason: Option<String>,
	source: Option<String>,
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
		SELECT reason::TEXT, source, created_at
		FROM v1_suppression_entries
		WHERE tenant_id = $1 AND email = $2
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
			reason: Some(row.get("reason")),
			source: row.get("source"),
			created_at: Some(
				row.get::<chrono::DateTime<chrono::Utc>, _>("created_at")
					.to_rfc3339(),
			),
		},
		None => Response {
			suppressed: false,
			reason: None,
			source: None,
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
