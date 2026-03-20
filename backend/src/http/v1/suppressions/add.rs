use crate::config::BackendConfig;
use crate::finder::require_tenant_id;
use crate::http::v1::bulk::with_worker_db;
use crate::http::{resolve_tenant, ReacherResponseError};
use crate::tenant::context::TenantContext;
use check_if_email_exists::LOG_TARGET;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use warp::http::StatusCode;
use warp::Filter;

#[derive(Debug, Deserialize)]
struct Request {
	emails: Vec<String>,
	#[serde(default = "default_reason")]
	reason: String,
	source: Option<String>,
	notes: Option<String>,
}

fn default_reason() -> String {
	"manual".to_string()
}

#[derive(Debug, Serialize)]
struct Response {
	added: i64,
	duplicates: i64,
}

async fn http_handler(
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	body: Request,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;

	if body.emails.is_empty() {
		return Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"emails array is required and must not be empty",
		)
		.into());
	}
	if body.emails.len() > 10_000 {
		return Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"Maximum 10,000 emails per request",
		)
		.into());
	}

	let reason = &body.reason;
	let valid_reasons = [
		"manual",
		"bounce",
		"invalid",
		"spam_trap",
		"unsubscribe",
		"complaint",
		"auto_invalid",
	];
	if !valid_reasons.contains(&reason.as_str()) {
		return Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			format!(
				"Invalid reason '{}'. Must be one of: {}",
				reason,
				valid_reasons.join(", ")
			),
		)
		.into());
	}

	let normalized: Vec<String> = body
		.emails
		.iter()
		.map(|e| e.trim().to_lowercase())
		.filter(|e| !e.is_empty())
		.collect();

	let mut added: i64 = 0;
	let mut duplicates: i64 = 0;

	for email in &normalized {
		let result = sqlx::query(
			r#"
			INSERT INTO v1_suppression_entries (tenant_id, email, reason, source, notes)
			VALUES ($1, $2, $3::suppression_reason, $4, $5)
			ON CONFLICT (tenant_id, email) DO NOTHING
			"#,
		)
		.bind(tenant_id)
		.bind(email)
		.bind(reason)
		.bind(&body.source)
		.bind(&body.notes)
		.execute(&pg_pool)
		.await
		.map_err(ReacherResponseError::from)?;

		if result.rows_affected() > 0 {
			added += 1;
		} else {
			duplicates += 1;
		}
	}

	Ok(warp::reply::with_status(
		warp::reply::json(&Response { added, duplicates }),
		StatusCode::OK,
	))
}

/// POST /v1/suppressions
#[utoipa::path(
	post,
	path = "/v1/suppressions",
	tag = "v1",
	responses((status = 200, description = "Suppression entries added"))
)]
pub fn v1_add_suppressions(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "suppressions")
		.and(warp::post())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::body::json())
		.and_then(http_handler)
		.with(warp::log(LOG_TARGET))
}
