use crate::config::BackendConfig;
use crate::finder::require_tenant_id;
use crate::http::v1::bulk::with_worker_db;
use crate::http::{check_scope, resolve_tenant, ReacherResponseError};
use crate::list_intelligence::validate_segment_filter;
use crate::tenant::context::{scope, TenantContext};
use check_if_email_exists::LOG_TARGET;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{PgPool, Row};
use std::sync::Arc;
use warp::http::StatusCode;
use warp::Filter;

#[derive(Debug, Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
pub struct SegmentListQuery {
	scope: Option<String>,
	limit: Option<i64>,
	offset: Option<i64>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct SavedSegmentView {
	pub id: i64,
	pub name: String,
	pub scope: String,
	pub filter: Value,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct SavedSegmentListResponse {
	pub segments: Vec<SavedSegmentView>,
	pub total: i64,
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct CreateSavedSegmentRequest {
	pub name: String,
	#[serde(default = "default_scope")]
	pub scope: String,
	#[serde(default = "default_filter")]
	pub filter: Value,
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct UpdateSavedSegmentRequest {
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub scope: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub filter: Option<Value>,
}

fn default_scope() -> String {
	"lists".to_string()
}

fn default_filter() -> Value {
	serde_json::json!({})
}

async fn list_handler(
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	query: SegmentListQuery,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::LISTS)?;
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	if let Some(scope) = &query.scope {
		validate_scope(scope)?;
	}
	let limit = query.limit.unwrap_or(50).clamp(0, 200);
	let offset = query.offset.unwrap_or(0).max(0);
	let total: i64 = sqlx::query_scalar(
		r#"
		SELECT COUNT(*)
		FROM v1_saved_segments
		WHERE tenant_id = $1 AND ($2::TEXT IS NULL OR scope = $2)
		"#,
	)
	.bind(tenant_id)
	.bind(&query.scope)
	.fetch_one(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;
	let rows = sqlx::query(
		r#"
		SELECT id, name, scope, filter, created_at, updated_at
		FROM v1_saved_segments
		WHERE tenant_id = $1 AND ($2::TEXT IS NULL OR scope = $2)
		ORDER BY created_at DESC
		LIMIT $3 OFFSET $4
		"#,
	)
	.bind(tenant_id)
	.bind(&query.scope)
	.bind(limit)
	.bind(offset)
	.fetch_all(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;
	Ok(warp::reply::json(&SavedSegmentListResponse {
		segments: rows.iter().map(row_to_segment).collect(),
		total,
	}))
}

async fn create_handler(
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	body: CreateSavedSegmentRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::LISTS)?;
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	validate_name(&body.name)?;
	validate_scope(&body.scope)?;
	validate_segment_filter(&body.filter)
		.map_err(|err| ReacherResponseError::new(StatusCode::BAD_REQUEST, err))?;
	let row = sqlx::query(
		r#"
		INSERT INTO v1_saved_segments (tenant_id, name, scope, filter)
		VALUES ($1, $2, $3, $4)
		RETURNING id, name, scope, filter, created_at, updated_at
		"#,
	)
	.bind(tenant_id)
	.bind(body.name.trim())
	.bind(body.scope.trim())
	.bind(&body.filter)
	.fetch_one(&pg_pool)
	.await
	.map_err(map_segment_sql_error)?;
	Ok(warp::reply::with_status(
		warp::reply::json(&row_to_segment(&row)),
		StatusCode::CREATED,
	))
}

async fn get_handler(
	segment_id: i64,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::LISTS)?;
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	let row = fetch_segment(&pg_pool, tenant_id, segment_id)
		.await
		.map_err(ReacherResponseError::from)?;
	let row = row.ok_or_else(|| not_found("Saved segment not found"))?;
	Ok(warp::reply::json(&row_to_segment(&row)))
}

async fn patch_handler(
	segment_id: i64,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	body: UpdateSavedSegmentRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::LISTS)?;
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	if let Some(name) = &body.name {
		validate_name(name)?;
	}
	if let Some(scope) = &body.scope {
		validate_scope(scope)?;
	}
	if let Some(filter) = &body.filter {
		validate_segment_filter(filter)
			.map_err(|err| ReacherResponseError::new(StatusCode::BAD_REQUEST, err))?;
	}
	let row = sqlx::query(
		r#"
		UPDATE v1_saved_segments
		SET name = COALESCE($3, name),
		    scope = COALESCE($4, scope),
		    filter = COALESCE($5, filter),
		    updated_at = NOW()
		WHERE id = $1 AND tenant_id = $2
		RETURNING id, name, scope, filter, created_at, updated_at
		"#,
	)
	.bind(segment_id)
	.bind(tenant_id)
	.bind(body.name.as_deref().map(str::trim))
	.bind(body.scope.as_deref().map(str::trim))
	.bind(body.filter.as_ref())
	.fetch_optional(&pg_pool)
	.await
	.map_err(map_segment_sql_error)?;
	let row = row.ok_or_else(|| not_found("Saved segment not found"))?;
	Ok(warp::reply::json(&row_to_segment(&row)))
}

async fn delete_handler(
	segment_id: i64,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::LISTS)?;
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	let deleted = sqlx::query("DELETE FROM v1_saved_segments WHERE id = $1 AND tenant_id = $2")
		.bind(segment_id)
		.bind(tenant_id)
		.execute(&pg_pool)
		.await
		.map_err(ReacherResponseError::from)?
		.rows_affected()
		> 0;
	if !deleted {
		return Err(not_found("Saved segment not found"));
	}
	Ok(warp::reply::json(&serde_json::json!({ "deleted": true })))
}

pub async fn load_segment_filter(
	pg_pool: &PgPool,
	tenant_id: uuid::Uuid,
	segment_id: i64,
) -> Result<Option<Value>, sqlx::Error> {
	let row = fetch_segment(pg_pool, tenant_id, segment_id).await?;
	Ok(row.map(|row| row.get("filter")))
}

async fn fetch_segment(
	pg_pool: &PgPool,
	tenant_id: uuid::Uuid,
	segment_id: i64,
) -> Result<Option<sqlx::postgres::PgRow>, sqlx::Error> {
	sqlx::query(
		"SELECT id, name, scope, filter, created_at, updated_at FROM v1_saved_segments WHERE id = $1 AND tenant_id = $2",
	)
	.bind(segment_id)
	.bind(tenant_id)
	.fetch_optional(pg_pool)
	.await
}

fn validate_name(name: &str) -> Result<(), warp::Rejection> {
	if name.trim().is_empty() {
		return Err(ReacherResponseError::new(StatusCode::BAD_REQUEST, "name is required").into());
	}
	Ok(())
}

fn validate_scope(scope: &str) -> Result<(), warp::Rejection> {
	match scope {
		"lists" | "tenant" => Ok(()),
		_ => Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"scope must be lists or tenant",
		)
		.into()),
	}
}

fn not_found(message: &str) -> warp::Rejection {
	warp::reject::custom(ReacherResponseError::new(
		StatusCode::NOT_FOUND,
		message.to_string(),
	))
}

fn map_segment_sql_error(err: sqlx::Error) -> ReacherResponseError {
	if let sqlx::Error::Database(db_err) = &err {
		if db_err.is_unique_violation() {
			return ReacherResponseError::new(
				StatusCode::CONFLICT,
				"A saved segment with this name already exists",
			);
		}
	}
	ReacherResponseError::from(err)
}

fn row_to_segment(row: &sqlx::postgres::PgRow) -> SavedSegmentView {
	SavedSegmentView {
		id: row.get("id"),
		name: row.get("name"),
		scope: row.get("scope"),
		filter: row.get("filter"),
		created_at: row.get("created_at"),
		updated_at: row.get("updated_at"),
	}
}

/// GET /v1/segments
#[utoipa::path(
	get,
	path = "/v1/segments",
	tag = "Lists",
	params(SegmentListQuery),
	responses((status = 200, description = "Saved segments", body = SavedSegmentListResponse))
)]
pub fn v1_list_saved_segments(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "segments")
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::query::<SegmentListQuery>())
		.and_then(list_handler)
		.with(warp::log(LOG_TARGET))
}

/// POST /v1/segments
#[utoipa::path(
	post,
	path = "/v1/segments",
	tag = "Lists",
	request_body = CreateSavedSegmentRequest,
	responses((status = 201, description = "Saved segment created", body = SavedSegmentView))
)]
pub fn v1_create_saved_segment(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "segments")
		.and(warp::post())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::body::json())
		.and_then(create_handler)
		.with(warp::log(LOG_TARGET))
}

/// GET /v1/segments/{segment_id}
#[utoipa::path(
	get,
	path = "/v1/segments/{segment_id}",
	tag = "Lists",
	params(("segment_id" = i64, Path, description = "Saved segment identifier")),
	responses((status = 200, description = "Saved segment", body = SavedSegmentView))
)]
pub fn v1_get_saved_segment(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "segments" / i64)
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and_then(get_handler)
		.with(warp::log(LOG_TARGET))
}

/// PATCH /v1/segments/{segment_id}
#[utoipa::path(
	patch,
	path = "/v1/segments/{segment_id}",
	tag = "Lists",
	params(("segment_id" = i64, Path, description = "Saved segment identifier")),
	request_body = UpdateSavedSegmentRequest,
	responses((status = 200, description = "Saved segment updated", body = SavedSegmentView))
)]
pub fn v1_update_saved_segment(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "segments" / i64)
		.and(warp::patch())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::body::json())
		.and_then(patch_handler)
		.with(warp::log(LOG_TARGET))
}

/// DELETE /v1/segments/{segment_id}
#[utoipa::path(
	delete,
	path = "/v1/segments/{segment_id}",
	tag = "Lists",
	params(("segment_id" = i64, Path, description = "Saved segment identifier")),
	responses((status = 200, description = "Saved segment deleted"))
)]
pub fn v1_delete_saved_segment(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "segments" / i64)
		.and(warp::delete())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and_then(delete_handler)
		.with(warp::log(LOG_TARGET))
}
