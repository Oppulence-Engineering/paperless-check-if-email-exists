use crate::config::BackendConfig;
use crate::http::v1::bulk::with_worker_db;
use crate::http::{check_scope, resolve_tenant, ReacherResponseError};
use crate::tenant::context::{scope, TenantContext};
use check_if_email_exists::LOG_TARGET;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use std::sync::Arc;
use warp::http::StatusCode;
use warp::Filter;

#[derive(Debug, Deserialize)]
struct CreateCommentRequest {
	job_id: Option<i32>,
	list_id: Option<i32>,
	body: String,
	author: Option<String>,
}

#[derive(Debug, Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
struct ListQuery {
	job_id: Option<i32>,
	list_id: Option<i32>,
	limit: Option<i64>,
	offset: Option<i64>,
}

#[derive(Debug, Serialize)]
struct Comment {
	id: i64,
	job_id: Option<i32>,
	list_id: Option<i32>,
	body: String,
	author: Option<String>,
	created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
struct ListResponse {
	comments: Vec<Comment>,
	total: i64,
}

/// Pick the required scope from the referenced resource type.
fn required_scope(job_id: Option<i32>, list_id: Option<i32>) -> &'static str {
	if list_id.is_some() {
		scope::LISTS
	} else if job_id.is_some() {
		scope::BULK
	} else {
		scope::BULK
	}
}

async fn create_handler(
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	body: CreateCommentRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
	if body.body.trim().is_empty() {
		return Err(
			ReacherResponseError::new(StatusCode::BAD_REQUEST, "Comment body cannot be empty")
				.into(),
		);
	}
	if body.job_id.is_none() && body.list_id.is_none() {
		return Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"Either job_id or list_id is required",
		)
		.into());
	}

	check_scope(&tenant_ctx, required_scope(body.job_id, body.list_id))?;

	// Validate that the referenced job/list belongs to this tenant
	if let Some(jid) = body.job_id {
		let exists: bool = sqlx::query_scalar(
			"SELECT EXISTS(SELECT 1 FROM v1_bulk_job WHERE id = $1 AND (tenant_id = $2 OR $2 IS NULL))",
		)
		.bind(jid)
		.bind(tenant_ctx.tenant_id)
		.fetch_one(&pg_pool)
		.await
		.map_err(ReacherResponseError::from)?;
		if !exists {
			return Err(
				ReacherResponseError::new(StatusCode::NOT_FOUND, "Job not found").into(),
			);
		}
	}
	if let Some(lid) = body.list_id {
		let exists: bool = sqlx::query_scalar(
			"SELECT EXISTS(SELECT 1 FROM v1_lists WHERE id = $1 AND (tenant_id = $2 OR $2 IS NULL))",
		)
		.bind(lid)
		.bind(tenant_ctx.tenant_id)
		.fetch_one(&pg_pool)
		.await
		.map_err(ReacherResponseError::from)?;
		if !exists {
			return Err(
				ReacherResponseError::new(StatusCode::NOT_FOUND, "List not found").into(),
			);
		}
	}

	let row = sqlx::query(
		r#"
		INSERT INTO job_comments (tenant_id, job_id, list_id, body, author)
		VALUES ($1, $2, $3, $4, $5)
		RETURNING id, job_id, list_id, body, author, created_at
		"#,
	)
	.bind(tenant_ctx.tenant_id)
	.bind(body.job_id)
	.bind(body.list_id)
	.bind(body.body.trim())
	.bind(&body.author)
	.fetch_one(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let comment = Comment {
		id: row.get("id"),
		job_id: row.get("job_id"),
		list_id: row.get("list_id"),
		body: row.get("body"),
		author: row.get("author"),
		created_at: row.get("created_at"),
	};

	Ok(warp::reply::with_status(
		warp::reply::json(&comment),
		StatusCode::CREATED,
	))
}

async fn list_handler(
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	query: ListQuery,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, required_scope(query.job_id, query.list_id))?;

	let limit = query.limit.unwrap_or(50).clamp(0, 200);
	let offset = query.offset.unwrap_or(0).max(0);

	let total: i64 = sqlx::query_scalar(
		r#"
		SELECT COUNT(*) FROM job_comments
		WHERE (tenant_id = $1 OR $1 IS NULL)
		  AND ($2::INTEGER IS NULL OR job_id = $2)
		  AND ($3::INTEGER IS NULL OR list_id = $3)
		"#,
	)
	.bind(tenant_ctx.tenant_id)
	.bind(query.job_id)
	.bind(query.list_id)
	.fetch_one(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let rows = sqlx::query(
		r#"
		SELECT id, job_id, list_id, body, author, created_at
		FROM job_comments
		WHERE (tenant_id = $1 OR $1 IS NULL)
		  AND ($2::INTEGER IS NULL OR job_id = $2)
		  AND ($3::INTEGER IS NULL OR list_id = $3)
		ORDER BY created_at DESC
		LIMIT $4 OFFSET $5
		"#,
	)
	.bind(tenant_ctx.tenant_id)
	.bind(query.job_id)
	.bind(query.list_id)
	.bind(limit)
	.bind(offset)
	.fetch_all(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let comments: Vec<Comment> = rows
		.iter()
		.map(|r| Comment {
			id: r.get("id"),
			job_id: r.get("job_id"),
			list_id: r.get("list_id"),
			body: r.get("body"),
			author: r.get("author"),
			created_at: r.get("created_at"),
		})
		.collect();

	Ok(warp::reply::json(&ListResponse { comments, total }))
}

async fn delete_handler(
	comment_id: i64,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	// Allow delete if tenant has either bulk or lists scope
	if !tenant_ctx.has_scope(scope::BULK) && !tenant_ctx.has_scope(scope::LISTS) {
		check_scope(&tenant_ctx, scope::BULK)?;
	}

	let deleted = sqlx::query_scalar::<_, i64>(
		"DELETE FROM job_comments WHERE id = $1 AND (tenant_id = $2 OR $2 IS NULL) RETURNING id",
	)
	.bind(comment_id)
	.bind(tenant_ctx.tenant_id)
	.fetch_optional(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	if deleted.is_none() {
		return Err(
			ReacherResponseError::new(StatusCode::NOT_FOUND, "Comment not found").into(),
		);
	}

	Ok(warp::reply::json(&serde_json::json!({"deleted": true, "id": comment_id})))
}

/// POST /v1/comments
#[utoipa::path(
	post,
	path = "/v1/comments",
	tag = "Comments",
	responses((status = 201, description = "Comment created"))
)]
pub fn v1_create_comment(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "comments")
		.and(warp::post())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::body::json())
		.and_then(create_handler)
		.with(warp::log(LOG_TARGET))
}

/// GET /v1/comments
#[utoipa::path(
	get,
	path = "/v1/comments",
	tag = "Comments",
	params(ListQuery),
	responses((status = 200, description = "Comments list"))
)]
pub fn v1_list_comments(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "comments")
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::query::<ListQuery>())
		.and_then(list_handler)
		.with(warp::log(LOG_TARGET))
}

/// DELETE /v1/comments/{comment_id}
#[utoipa::path(
	delete,
	path = "/v1/comments/{comment_id}",
	tag = "Comments",
	params(("comment_id" = i64, Path, description = "Comment identifier")),
	responses((status = 200, description = "Comment deleted"))
)]
pub fn v1_delete_comment(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "comments" / i64)
		.and(warp::delete())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and_then(delete_handler)
		.with(warp::log(LOG_TARGET))
}
