use crate::config::BackendConfig;
use crate::finder::{sync_finder_results, FinderBestMatch, FinderCandidateResult};
use crate::http::v1::bulk::with_worker_db;
use crate::http::{resolve_tenant, ReacherResponseError};
use crate::tenant::context::TenantContext;
use check_if_email_exists::LOG_TARGET;
use serde::Serialize;
use sqlx::{PgPool, Row};
use std::sync::Arc;
use warp::http::StatusCode;
use warp::Filter;

#[derive(Debug, Serialize)]
struct Response {
	job_id: i32,
	bulk_job_id: i32,
	status: String,
	domain_has_mx: bool,
	domain_is_catch_all: bool,
	candidates_checked: i32,
	results: Vec<FinderCandidateResult>,
	best_match: Option<FinderBestMatch>,
}

async fn http_handler(
	job_id: i32,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	let row = sqlx::query(
		r#"
		SELECT id, bulk_job_id, status::TEXT AS status, domain_has_mx, domain_is_catch_all, candidates_checked
		FROM v1_finder_job
		WHERE id = $1 AND (tenant_id = $2 OR $2 IS NULL)
		"#,
	)
	.bind(job_id)
	.bind(tenant_ctx.tenant_id)
	.fetch_optional(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let row = row.ok_or_else(|| {
		warp::reject::custom(ReacherResponseError::new(
			StatusCode::NOT_FOUND,
			"Finder job not found",
		))
	})?;

	let (results, best_match, all_terminal) = sync_finder_results(&pg_pool, job_id)
		.await
		.map_err(warp::reject::custom)?;

	let status = if all_terminal {
		"completed".to_string()
	} else {
		row.get::<String, _>("status")
	};

	Ok(warp::reply::json(&Response {
		job_id: row.get("id"),
		bulk_job_id: row.get("bulk_job_id"),
		status,
		domain_has_mx: row.get("domain_has_mx"),
		domain_is_catch_all: row.get("domain_is_catch_all"),
		candidates_checked: row.get("candidates_checked"),
		results,
		best_match,
	}))
}

/// GET /v1/find_email/{job_id}
#[utoipa::path(
	get,
	path = "/v1/find_email/{job_id}",
	tag = "v1",
	params(("job_id" = i32, Path, description = "Finder job identifier")),
	responses((status = 200, description = "Finder job result"))
)]
pub fn v1_get_find_email(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "find_email" / i32)
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and_then(http_handler)
		.with(warp::log(LOG_TARGET))
}
