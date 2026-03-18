use crate::config::BackendConfig;
use crate::http::ReacherResponseError;
use crate::http::resolve_tenant;
use crate::tenant::auth::generate_api_key;
use crate::tenant::context::TenantContext;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use std::sync::Arc;
use uuid::Uuid;
use warp::Filter;
use warp::http::StatusCode;

#[derive(Debug, Deserialize)]
struct CreateApiKeyRequest {
	pub name: Option<String>,
	pub scopes: Option<Vec<String>>,
	pub expires_at: Option<String>,
}

#[derive(Debug, Deserialize)]
struct UpdateApiKeyRequest {
	pub name: Option<String>,
	pub scopes: Option<Vec<String>>,
	pub expires_at: Option<String>,
}

#[derive(Debug, Serialize)]
struct ApiKeyResponse {
	pub id: Uuid,
	pub tenant_id: Uuid,
	pub key_prefix: String,
	pub name: String,
	pub scopes: Vec<String>,
	pub status: String,
	pub last_used_at: Option<String>,
	pub expires_at: Option<String>,
	pub created_at: String,
}

#[derive(Debug, Serialize)]
struct CreateApiKeyResponse {
	pub id: Uuid,
	pub tenant_id: Uuid,
	pub key: String,
	pub key_prefix: String,
	pub name: String,
	pub scopes: Vec<String>,
	pub status: String,
	pub expires_at: Option<String>,
	pub created_at: String,
}

#[derive(Debug, Serialize)]
struct ListApiKeysResponse {
	pub api_keys: Vec<ApiKeyResponse>,
}

fn with_pg_pool(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (PgPool,), Error = warp::Rejection> + Clone {
	warp::any().and_then(move || {
		let config = Arc::clone(&config);
		async move {
			config.get_pg_pool().ok_or_else(|| {
				warp::reject::custom(ReacherResponseError::new(
					StatusCode::SERVICE_UNAVAILABLE,
					"Postgres database required for account API key endpoints",
				))
			})
		}
	})
}

fn ensure_tenant_id(tenant_ctx: TenantContext) -> Result<Uuid, warp::Rejection> {
	tenant_ctx.tenant_id.ok_or_else(|| {
		warp::reject::custom(ReacherResponseError::new(
			StatusCode::UNAUTHORIZED,
			"API key authentication required for account API key management",
		))
	})
}

fn parse_expiry(expires_at: Option<String>) -> Result<Option<DateTime<Utc>>, warp::Rejection> {
	if let Some(expires_at) = expires_at {
		chrono::DateTime::parse_from_rfc3339(&expires_at)
			.map(|dt| dt.with_timezone(&Utc))
			.map(Some)
			.map_err(|_| {
				warp::reject::custom(ReacherResponseError::new(
					StatusCode::BAD_REQUEST,
					"expires_at must be RFC3339 timestamp",
				))
			})
	} else {
		Ok(None)
	}
}

fn row_to_response(row: &sqlx::postgres::PgRow) -> ApiKeyResponse {
	ApiKeyResponse {
		id: row.get("id"),
		tenant_id: row.get("tenant_id"),
		key_prefix: row.get("key_prefix"),
		name: row.get("name"),
		scopes: row.get("scopes"),
		status: row.get("status"),
		last_used_at: row
			.get::<Option<DateTime<Utc>>, _>("last_used_at")
			.map(|dt| dt.to_rfc3339()),
		expires_at: row
			.get::<Option<DateTime<Utc>>, _>("expires_at")
			.map(|dt| dt.to_rfc3339()),
		created_at: row.get::<DateTime<Utc>, _>("created_at").to_rfc3339(),
	}
}

async fn get_handler(
	key_id_str: String,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id = ensure_tenant_id(tenant_ctx)?;
	let key_id = key_id_str
		.parse::<Uuid>()
		.map_err(|_| ReacherResponseError::new(StatusCode::BAD_REQUEST, "Invalid API key ID format"))?;

	let row = sqlx::query(
		"SELECT id, tenant_id, key_prefix, name, scopes, status::TEXT, last_used_at, expires_at, created_at \
		 FROM api_keys WHERE id = $1 AND tenant_id = $2",
	)
	.bind(key_id)
	.bind(tenant_id)
	.fetch_optional(&pg_pool)
	 .await
	 .map_err(ReacherResponseError::from)?;

	let row = match row {
		Some(r) => r,
		None => return Err(ReacherResponseError::new(StatusCode::NOT_FOUND, "API key not found").into()),
	};

	Ok(warp::reply::json(&row_to_response(&row)))
}

async fn list_handler(
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id = ensure_tenant_id(tenant_ctx)?;

	let rows = sqlx::query(
		"SELECT id, tenant_id, key_prefix, name, scopes, status::TEXT, last_used_at, expires_at, created_at \
		 FROM api_keys WHERE tenant_id = $1 ORDER BY created_at DESC",
	)
	.bind(tenant_id)
	.fetch_all(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let api_keys = rows.iter().map(row_to_response).collect();

	Ok(warp::reply::json(&ListApiKeysResponse { api_keys }))
}

async fn create_handler(
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	body: CreateApiKeyRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id = ensure_tenant_id(tenant_ctx)?;

	let name = body.name.unwrap_or_else(|| "Default".to_string());
	let scopes: Vec<String> = body.scopes.unwrap_or_default();
	let expires_at = parse_expiry(body.expires_at)?;
	let (full_key, key_prefix, key_hash) = generate_api_key();

	let row = sqlx::query(
		"INSERT INTO api_keys (tenant_id, key_prefix, key_hash, name, scopes, expires_at) \
		 VALUES ($1, $2, $3, $4, $5, $6) \
		 RETURNING id, tenant_id, key_prefix, name, scopes, status::TEXT, expires_at, created_at",
	)
	.bind(tenant_id)
	.bind(&key_prefix)
	.bind(&key_hash)
	.bind(&name)
	.bind(&scopes)
	.bind(expires_at)
	.fetch_one(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let response = CreateApiKeyResponse {
		id: row.get("id"),
		tenant_id: row.get("tenant_id"),
		key: full_key,
		key_prefix: row.get("key_prefix"),
		name: row.get("name"),
		scopes: row.get("scopes"),
		status: row.get("status"),
		expires_at: row
			.get::<Option<DateTime<Utc>>, _>("expires_at")
			.map(|dt| dt.to_rfc3339()),
		created_at: row.get::<DateTime<Utc>, _>("created_at").to_rfc3339(),
	};

	Ok(warp::reply::with_status(
		warp::reply::json(&response),
		StatusCode::CREATED,
	))
}

async fn update_handler(
	key_id_str: String,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	body: UpdateApiKeyRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id = ensure_tenant_id(tenant_ctx)?;
	let key_id = key_id_str
		.parse::<Uuid>()
		.map_err(|_| ReacherResponseError::new(StatusCode::BAD_REQUEST, "Invalid API key ID format"))?;

	let mut sets = Vec::new();
	let mut param_idx = 3u32;

	if body.name.is_some() {
		sets.push(format!("name = ${}", param_idx));
		param_idx += 1;
	}
	if body.scopes.is_some() {
		sets.push(format!("scopes = ${}", param_idx));
		param_idx += 1;
	}
	if body.expires_at.is_some() {
		sets.push(format!("expires_at = ${}", param_idx));
	}

	if sets.is_empty() {
		return Err(ReacherResponseError::new(StatusCode::BAD_REQUEST, "No fields to update").into());
	}

	let sql = format!(
		"UPDATE api_keys SET {} WHERE id = $1 AND tenant_id = $2 RETURNING id, tenant_id, key_prefix, name, scopes, status::TEXT, last_used_at, expires_at, created_at",
		sets.join(", ")
	);

	let mut query = sqlx::query(&sql).bind(key_id).bind(tenant_id);

	if let Some(name) = body.name {
		query = query.bind(name);
	}
	if let Some(scopes) = body.scopes {
		query = query.bind(scopes);
	}
	if let Some(expires_at) = body.expires_at {
		let parsed = chrono::DateTime::parse_from_rfc3339(&expires_at)
			.map(|dt| dt.with_timezone(&Utc))
			.map_err(|_| {
				warp::reject::custom(ReacherResponseError::new(
					StatusCode::BAD_REQUEST,
					"expires_at must be RFC3339 timestamp",
				))
			})?;
		query = query.bind(parsed);
	}

	let row = query
		.fetch_optional(&pg_pool)
		.await
		.map_err(ReacherResponseError::from)?;

	let row = match row {
		Some(r) => r,
		None => return Err(ReacherResponseError::new(StatusCode::NOT_FOUND, "API key not found").into()),
	};

	Ok(warp::reply::json(&row_to_response(&row)))
}

async fn revoke_handler(
	key_id_str: String,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id = ensure_tenant_id(tenant_ctx)?;
	let key_id: Uuid = key_id_str.parse().map_err(|_| {
		ReacherResponseError::new(StatusCode::BAD_REQUEST, "Invalid API key ID format")
	})?;

	let result = sqlx::query(
		"UPDATE api_keys SET status = 'revoked'::api_key_status WHERE id = $1 AND tenant_id = $2",
	)
	.bind(key_id)
	.bind(tenant_id)
	.execute(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	if result.rows_affected() == 0 {
		return Err(ReacherResponseError::new(StatusCode::NOT_FOUND, "API key not found").into());
	}

	Ok(warp::reply::json(&serde_json::json!({ "revoked": true, "key_id": key_id })))
}

/// GET /v1/me/api-keys/{key_id}
///
/// Return a single API key for the authenticated tenant.
#[utoipa::path(
	get,
	path = "/v1/me/api-keys/{key_id}",
	tag = "Account",
	params(("key_id" = Uuid, Path, description = "API key identifier")),
	responses((status = 200, description = "API key details")),
)]
pub fn get_api_key(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "me" / "api-keys" / String)
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_pg_pool(config))
		.and_then(get_handler)
		.with(warp::log("reacher_backend::v1::account_api_keys::get"))
}

/// GET /v1/me/api-keys
///
/// List API keys for the authenticated tenant.
#[utoipa::path(
	get,
	path = "/v1/me/api-keys",
	tag = "Account",
	responses((status = 200, description = "API key list")),
)]
pub fn list_api_keys(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "me" / "api-keys")
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_pg_pool(config))
		.and_then(list_handler)
		.with(warp::log("reacher_backend::v1::account_api_keys::list"))
}

/// POST /v1/me/api-keys
///
/// Create a new API key for the authenticated tenant.
#[utoipa::path(
	post,
	path = "/v1/me/api-keys",
	tag = "Account",
	responses((status = 201, description = "API key created")),
)]
pub fn create_api_key(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "me" / "api-keys")
		.and(warp::post())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_pg_pool(config))
		.and(warp::body::json())
		.and_then(create_handler)
		.with(warp::log("reacher_backend::v1::account_api_keys::create"))
}

/// PATCH /v1/me/api-keys/{key_id}
///
/// Update metadata for an existing API key.
#[utoipa::path(
	patch,
	path = "/v1/me/api-keys/{key_id}",
	tag = "Account",
	params(("key_id" = Uuid, Path, description = "API key identifier")),
	responses((status = 200, description = "API key updated")),
)]
pub fn update_api_key(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "me" / "api-keys" / String)
		.and(warp::patch())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_pg_pool(config))
		.and(warp::body::json())
		.and_then(update_handler)
		.with(warp::log("reacher_backend::v1::account_api_keys::update"))
}

/// DELETE /v1/me/api-keys/{key_id}
///
/// Revoke an API key.
#[utoipa::path(
	delete,
	path = "/v1/me/api-keys/{key_id}",
	tag = "Account",
	params(("key_id" = Uuid, Path, description = "API key identifier")),
	responses((status = 200, description = "API key revoked")),
)]
pub fn revoke_api_key(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "me" / "api-keys" / String)
		.and(warp::delete())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_pg_pool(config))
		.and_then(revoke_handler)
		.with(warp::log("reacher_backend::v1::account_api_keys::revoke"))
}
