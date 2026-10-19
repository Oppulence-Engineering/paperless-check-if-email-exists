use crate::config::BackendConfig;
use crate::finder::require_tenant_id;
use crate::http::v1::bulk::with_worker_db;
use crate::http::{check_scope, resolve_tenant, ReacherResponseError};
use crate::list_intelligence::validate_policy_rules;
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
pub struct PolicyListQuery {
	limit: Option<i64>,
	offset: Option<i64>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ScorePolicyView {
	pub id: i64,
	pub name: String,
	pub is_default: bool,
	pub rules: Value,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ScorePolicyListResponse {
	pub policies: Vec<ScorePolicyView>,
	pub total: i64,
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct CreateScorePolicyRequest {
	pub name: String,
	#[serde(default)]
	pub is_default: bool,
	#[serde(default = "default_rules")]
	pub rules: Value,
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct UpdateScorePolicyRequest {
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub is_default: Option<bool>,
	#[serde(default, skip_serializing_if = "Option::is_none")]
	pub rules: Option<Value>,
}

fn default_rules() -> Value {
	serde_json::json!({})
}

async fn list_handler(
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	query: PolicyListQuery,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::SETTINGS)?;
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	let limit = query.limit.unwrap_or(50).clamp(0, 200);
	let offset = query.offset.unwrap_or(0).max(0);

	let total: i64 =
		sqlx::query_scalar("SELECT COUNT(*) FROM v1_score_policies WHERE tenant_id = $1")
			.bind(tenant_id)
			.fetch_one(&pg_pool)
			.await
			.map_err(ReacherResponseError::from)?;
	let rows = sqlx::query(
		r#"
		SELECT id, name, is_default, rules, created_at, updated_at
		FROM v1_score_policies
		WHERE tenant_id = $1
		ORDER BY is_default DESC, created_at DESC
		LIMIT $2 OFFSET $3
		"#,
	)
	.bind(tenant_id)
	.bind(limit)
	.bind(offset)
	.fetch_all(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	Ok(warp::reply::json(&ScorePolicyListResponse {
		policies: rows.iter().map(row_to_policy).collect(),
		total,
	}))
}

async fn create_handler(
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	body: CreateScorePolicyRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::SETTINGS)?;
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	validate_name(&body.name)?;
	validate_policy_rules(&body.rules)
		.map_err(|err| ReacherResponseError::new(StatusCode::BAD_REQUEST, err))?;

	let mut tx = pg_pool.begin().await.map_err(ReacherResponseError::from)?;
	if body.is_default {
		sqlx::query("UPDATE v1_score_policies SET is_default = false WHERE tenant_id = $1")
			.bind(tenant_id)
			.execute(&mut *tx)
			.await
			.map_err(ReacherResponseError::from)?;
	}
	let row = sqlx::query(
		r#"
		INSERT INTO v1_score_policies (tenant_id, name, is_default, rules)
		VALUES ($1, $2, $3, $4)
		RETURNING id, name, is_default, rules, created_at, updated_at
		"#,
	)
	.bind(tenant_id)
	.bind(body.name.trim())
	.bind(body.is_default)
	.bind(&body.rules)
	.fetch_one(&mut *tx)
	.await
	.map_err(map_policy_sql_error)?;
	tx.commit().await.map_err(ReacherResponseError::from)?;

	Ok(warp::reply::with_status(
		warp::reply::json(&row_to_policy(&row)),
		StatusCode::CREATED,
	))
}

async fn get_handler(
	policy_id: i64,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::SETTINGS)?;
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	let row = sqlx::query(
		"SELECT id, name, is_default, rules, created_at, updated_at FROM v1_score_policies WHERE id = $1 AND tenant_id = $2",
	)
	.bind(policy_id)
	.bind(tenant_id)
	.fetch_optional(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;
	let row = row.ok_or_else(|| not_found("Score policy not found"))?;
	Ok(warp::reply::json(&row_to_policy(&row)))
}

async fn patch_handler(
	policy_id: i64,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	body: UpdateScorePolicyRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::SETTINGS)?;
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	if let Some(name) = &body.name {
		validate_name(name)?;
	}
	if let Some(rules) = &body.rules {
		validate_policy_rules(rules)
			.map_err(|err| ReacherResponseError::new(StatusCode::BAD_REQUEST, err))?;
	}

	let mut tx = pg_pool.begin().await.map_err(ReacherResponseError::from)?;
	if body.is_default == Some(true) {
		sqlx::query(
			"UPDATE v1_score_policies SET is_default = false WHERE tenant_id = $1 AND id <> $2",
		)
		.bind(tenant_id)
		.bind(policy_id)
		.execute(&mut *tx)
		.await
		.map_err(ReacherResponseError::from)?;
	}
	let row = sqlx::query(
		r#"
		UPDATE v1_score_policies
		SET name = COALESCE($3, name),
		    is_default = COALESCE($4, is_default),
		    rules = COALESCE($5, rules),
		    updated_at = NOW()
		WHERE id = $1 AND tenant_id = $2
		RETURNING id, name, is_default, rules, created_at, updated_at
		"#,
	)
	.bind(policy_id)
	.bind(tenant_id)
	.bind(body.name.as_deref().map(str::trim))
	.bind(body.is_default)
	.bind(body.rules.as_ref())
	.fetch_optional(&mut *tx)
	.await
	.map_err(map_policy_sql_error)?;
	let row = row.ok_or_else(|| not_found("Score policy not found"))?;
	tx.commit().await.map_err(ReacherResponseError::from)?;
	Ok(warp::reply::json(&row_to_policy(&row)))
}

async fn delete_handler(
	policy_id: i64,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::SETTINGS)?;
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	let deleted = sqlx::query("DELETE FROM v1_score_policies WHERE id = $1 AND tenant_id = $2")
		.bind(policy_id)
		.bind(tenant_id)
		.execute(&pg_pool)
		.await
		.map_err(ReacherResponseError::from)?
		.rows_affected()
		> 0;
	if !deleted {
		return Err(not_found("Score policy not found"));
	}
	Ok(warp::reply::json(&serde_json::json!({ "deleted": true })))
}

fn validate_name(name: &str) -> Result<(), warp::Rejection> {
	if name.trim().is_empty() {
		return Err(ReacherResponseError::new(StatusCode::BAD_REQUEST, "name is required").into());
	}
	Ok(())
}

fn not_found(message: &str) -> warp::Rejection {
	warp::reject::custom(ReacherResponseError::new(
		StatusCode::NOT_FOUND,
		message.to_string(),
	))
}

fn map_policy_sql_error(err: sqlx::Error) -> ReacherResponseError {
	if let sqlx::Error::Database(db_err) = &err {
		if db_err.is_unique_violation() {
			return ReacherResponseError::new(
				StatusCode::CONFLICT,
				"A score policy with this name or default setting already exists",
			);
		}
	}
	ReacherResponseError::from(err)
}

fn row_to_policy(row: &sqlx::postgres::PgRow) -> ScorePolicyView {
	ScorePolicyView {
		id: row.get("id"),
		name: row.get("name"),
		is_default: row.get("is_default"),
		rules: row.get("rules"),
		created_at: row.get("created_at"),
		updated_at: row.get("updated_at"),
	}
}

/// GET /v1/score-policies
#[utoipa::path(
	get,
	path = "/v1/score-policies",
	tag = "Tenant",
	params(PolicyListQuery),
	responses((status = 200, description = "Score policies", body = ScorePolicyListResponse))
)]
pub fn v1_list_score_policies(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "score-policies")
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::query::<PolicyListQuery>())
		.and_then(list_handler)
		.with(warp::log(LOG_TARGET))
}

/// POST /v1/score-policies
#[utoipa::path(
	post,
	path = "/v1/score-policies",
	tag = "Tenant",
	request_body = CreateScorePolicyRequest,
	responses((status = 201, description = "Score policy created", body = ScorePolicyView))
)]
pub fn v1_create_score_policy(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "score-policies")
		.and(warp::post())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::body::json())
		.and_then(create_handler)
		.with(warp::log(LOG_TARGET))
}

/// GET /v1/score-policies/{policy_id}
#[utoipa::path(
	get,
	path = "/v1/score-policies/{policy_id}",
	tag = "Tenant",
	params(("policy_id" = i64, Path, description = "Score policy identifier")),
	responses((status = 200, description = "Score policy", body = ScorePolicyView))
)]
pub fn v1_get_score_policy(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "score-policies" / i64)
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and_then(get_handler)
		.with(warp::log(LOG_TARGET))
}

/// PATCH /v1/score-policies/{policy_id}
#[utoipa::path(
	patch,
	path = "/v1/score-policies/{policy_id}",
	tag = "Tenant",
	params(("policy_id" = i64, Path, description = "Score policy identifier")),
	request_body = UpdateScorePolicyRequest,
	responses((status = 200, description = "Score policy updated", body = ScorePolicyView))
)]
pub fn v1_update_score_policy(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "score-policies" / i64)
		.and(warp::patch())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::body::json())
		.and_then(patch_handler)
		.with(warp::log(LOG_TARGET))
}

/// DELETE /v1/score-policies/{policy_id}
#[utoipa::path(
	delete,
	path = "/v1/score-policies/{policy_id}",
	tag = "Tenant",
	params(("policy_id" = i64, Path, description = "Score policy identifier")),
	responses((status = 200, description = "Score policy deleted"))
)]
pub fn v1_delete_score_policy(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "score-policies" / i64)
		.and(warp::delete())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and_then(delete_handler)
		.with(warp::log(LOG_TARGET))
}
