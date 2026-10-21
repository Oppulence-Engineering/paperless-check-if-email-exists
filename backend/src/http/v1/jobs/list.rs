use super::super::bulk::with_worker_db;
use crate::{
	config::BackendConfig,
	http::{check_scope, resolve_tenant, ReacherResponseError},
	tenant::context::{scope, TenantContext},
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use std::sync::Arc;
use warp::Filter;
#[derive(Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in=Query)]
struct Query {
	limit: Option<i64>,
	offset: Option<i64>,
}
#[derive(Serialize, utoipa::ToSchema)]
struct Job {
	id: i32,
	status: String,
	total_records: i32,
	created_at: chrono::DateTime<chrono::Utc>,
}
#[derive(Serialize, utoipa::ToSchema)]
struct Response {
	jobs: Vec<Job>,
	total: i64,
}
async fn handler(
	ctx: TenantContext,
	pool: PgPool,
	query: Query,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&ctx, scope::BULK)?;
	let tenant = crate::finder::require_tenant_id(ctx.tenant_id)?;
	let (limit, offset) = crate::http::shared::paging::clamp_page(query.limit, query.offset);
	let total = sqlx::query_scalar("SELECT count(*) FROM v1_bulk_job WHERE tenant_id=$1")
		.bind(tenant)
		.fetch_one(&pool)
		.await
		.map_err(ReacherResponseError::from)?;
	let rows = sqlx::query("SELECT id,status::text,total_records,created_at FROM v1_bulk_job WHERE tenant_id=$1 ORDER BY created_at DESC,id DESC LIMIT $2 OFFSET $3")
        .bind(tenant).bind(limit).bind(offset).fetch_all(&pool).await.map_err(ReacherResponseError::from)?;
	Ok(warp::reply::json(&Response {
		total,
		jobs: rows
			.iter()
			.map(|r| Job {
				id: r.get("id"),
				status: r.get("status"),
				total_records: r.get("total_records"),
				created_at: r.get("created_at"),
			})
			.collect(),
	}))
}
#[utoipa::path(get,path="/v1/jobs",tag="Jobs",params(Query),responses((status=200,description="Recent jobs",body=inline(Response))))]
pub fn v1_list_jobs(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "jobs")
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::query())
		.and_then(handler)
}
