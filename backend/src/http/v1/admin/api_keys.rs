use crate::config::BackendConfig;
use crate::http::check_header;
use crate::http::ReacherResponseError;
use crate::tenant::auth::generate_api_key;
use check_if_email_exists::LOG_TARGET;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use std::sync::Arc;
use uuid::Uuid;
use warp::Filter;
use warp::http::StatusCode;

// ── Request/Response types ──────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreateApiKeyRequest {
	pub name: Option<String>,
	pub scopes: Option<Vec<String>>,
	pub expires_at: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ListAllApiKeysQuery {
	pub tenant_id: Option<String>,
	pub status: Option<String>,
	pub limit: Option<i64>,
	pub offset: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateApiKeyRequest {
	pub name: Option<String>,
	pub scopes: Option<Vec<String>>,
	pub expires_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CreateApiKeyResponse {
	pub id: Uuid,
	pub tenant_id: Uuid,
	pub key: String, // Full plaintext key — only returned on creation
	pub key_prefix: String,
	pub name: String,
	pub scopes: Vec<String>,
	pub status: String,
	pub expires_at: Option<String>,
	pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct ApiKeyResponse {
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
pub struct ListApiKeysResponse {
	pub api_keys: Vec<ApiKeyResponse>,
}

#[derive(Debug, Serialize)]
pub struct ListAllApiKeysResponse {
	pub api_keys: Vec<ApiKeyResponse>,
	pub total: i64,
}

// ── Helpers ─────────────────────────────────────────

fn parse_status_filter(raw: Option<String>) -> Result<Option<String>, warp::Rejection> {
	match raw {
		Some(s) => {
			let normalized = s.to_lowercase();
			match normalized.as_str() {
				"active" | "revoked" | "expired" => Ok(Some(normalized)),
				_ => Err(warp::reject::custom(ReacherResponseError::new(
					StatusCode::BAD_REQUEST,
					"Invalid api key status. Expected active|revoked|expired",
				))),
			}
		}
		None => Ok(None),
	}
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
					"Postgres database required for admin endpoints",
				))
			})
		}
	})
}

fn parse_expiry(expires_at: Option<String>) -> Result<Option<chrono::DateTime<chrono::Utc>>, warp::Rejection> {
	if let Some(expires_at) = expires_at {
		chrono::DateTime::parse_from_rfc3339(&expires_at)
			.map(|dt| dt.with_timezone(&chrono::Utc))
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
			.get::<Option<chrono::DateTime<chrono::Utc>>, _>("last_used_at")
			.map(|dt| dt.to_rfc3339()),
		expires_at: row
			.get::<Option<chrono::DateTime<chrono::Utc>>, _>("expires_at")
			.map(|dt| dt.to_rfc3339()),
		created_at: row.get::<chrono::DateTime<chrono::Utc>, _>("created_at").to_rfc3339(),
	}
}

// ── Handlers ────────────────────────────────────────

async fn create_handler(
	tenant_id_str: String,
	pg_pool: PgPool,
	body: CreateApiKeyRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id: Uuid = tenant_id_str.parse().map_err(|_| {
		ReacherResponseError::new(StatusCode::BAD_REQUEST, "Invalid tenant ID")
	})?;

	// Verify tenant exists
	let exists: bool = sqlx::query("SELECT EXISTS(SELECT 1 FROM tenants WHERE id = $1) as exists")
		.bind(tenant_id)
		.fetch_one(&pg_pool)
		.await
		.map_err(ReacherResponseError::from)?
		.get("exists");

	if !exists {
		return Err(ReacherResponseError::new(StatusCode::NOT_FOUND, "Tenant not found").into());
	}

	let (full_key, key_prefix, key_hash) = generate_api_key();
	let name = body.name.unwrap_or_else(|| "Default".to_string());
	let scopes: Vec<String> = body.scopes.unwrap_or_default();
	let expires_at = parse_expiry(body.expires_at)?;

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
			.get::<Option<chrono::DateTime<chrono::Utc>>, _>("expires_at")
			.map(|dt| dt.to_rfc3339()),
		created_at: row
			.get::<chrono::DateTime<chrono::Utc>, _>("created_at")
			.to_rfc3339(),
	};

	Ok(warp::reply::with_status(
		warp::reply::json(&response),
		StatusCode::CREATED,
	))
}

async fn list_handler(
	tenant_id_str: String,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id: Uuid = tenant_id_str.parse().map_err(|_| {
		ReacherResponseError::new(StatusCode::BAD_REQUEST, "Invalid tenant ID")
	})?;

	let rows = sqlx::query(
		"SELECT id, tenant_id, key_prefix, name, scopes, status::TEXT, last_used_at, expires_at, created_at \
		 FROM api_keys WHERE tenant_id = $1 ORDER BY created_at DESC",
	)
	.bind(tenant_id)
	.fetch_all(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let api_keys: Vec<ApiKeyResponse> = rows.iter().map(row_to_response).collect();

	Ok(warp::reply::json(&ListApiKeysResponse { api_keys }))
}

async fn list_all_handler(
	pg_pool: PgPool,
	query: ListAllApiKeysQuery,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id = match query.tenant_id {
		Some(t) => Some(Uuid::parse_str(&t).map_err(|_| {
			ReacherResponseError::new(StatusCode::BAD_REQUEST, "Invalid tenant ID")
		})?),
		None => None,
	};

	let status = parse_status_filter(query.status)?;
	let limit = query.limit.unwrap_or(50).clamp(1, 200);
	let offset = query.offset.unwrap_or(0).max(0);

	let rows = sqlx::query(
		"SELECT id, tenant_id, key_prefix, name, scopes, status::TEXT, last_used_at, expires_at, created_at \
		 FROM api_keys \
		 WHERE ($1::UUID IS NULL OR tenant_id = $1::UUID)
             AND ($2::TEXT IS NULL OR status::TEXT = $2::TEXT) \
		 ORDER BY created_at DESC LIMIT $3 OFFSET $4",
	)
	.bind(tenant_id)
	.bind(status.as_deref())
	.bind(limit)
	.bind(offset)
	.fetch_all(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let total: i64 = sqlx::query(
		"SELECT COUNT(*) AS count \
		 FROM api_keys \
		 WHERE ($1::UUID IS NULL OR tenant_id = $1::UUID)
             AND ($2::TEXT IS NULL OR status::TEXT = $2::TEXT)",
	)
	.bind(tenant_id)
	.bind(status.as_deref())
	.fetch_one(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?
	.get("count");

	let api_keys: Vec<ApiKeyResponse> = rows.iter().map(row_to_response).collect();
	Ok(warp::reply::json(&ListAllApiKeysResponse { api_keys, total }))
}

async fn revoke_handler(
	tenant_id_str: String,
	key_id_str: String,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id: Uuid = tenant_id_str.parse().map_err(|_| {
		ReacherResponseError::new(StatusCode::BAD_REQUEST, "Invalid tenant ID")
	})?;
	let key_id: Uuid = key_id_str.parse().map_err(|_| {
		ReacherResponseError::new(StatusCode::BAD_REQUEST, "Invalid API key ID")
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

	Ok(warp::reply::json(&serde_json::json!({"revoked": true, "key_id": key_id})))
}

async fn reactivate_handler(
	tenant_id_str: String,
	key_id_str: String,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id: Uuid = tenant_id_str.parse().map_err(|_| {
		ReacherResponseError::new(StatusCode::BAD_REQUEST, "Invalid tenant ID")
	})?;
	let key_id: Uuid = key_id_str.parse().map_err(|_| {
		ReacherResponseError::new(StatusCode::BAD_REQUEST, "Invalid API key ID")
	})?;

	let result = sqlx::query(
		"UPDATE api_keys SET status = 'active'::api_key_status WHERE id = $1 AND tenant_id = $2",
	)
	.bind(key_id)
	.bind(tenant_id)
	.execute(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	if result.rows_affected() == 0 {
		return Err(ReacherResponseError::new(StatusCode::NOT_FOUND, "API key not found").into());
	}

	Ok(warp::reply::json(&serde_json::json!({"reactivated": true, "key_id": key_id})))
}

async fn get_handler(
	tenant_id_str: String,
	key_id_str: String,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id = Uuid::parse_str(&tenant_id_str)
		.map_err(|_| ReacherResponseError::new(StatusCode::BAD_REQUEST, "Invalid tenant ID"))?;
	let key_id = Uuid::parse_str(&key_id_str)
		.map_err(|_| ReacherResponseError::new(StatusCode::BAD_REQUEST, "Invalid API key ID"))?;

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

async fn update_handler(
	tenant_id_str: String,
	key_id_str: String,
	pg_pool: PgPool,
	body: UpdateApiKeyRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id = Uuid::parse_str(&tenant_id_str)
		.map_err(|_| ReacherResponseError::new(StatusCode::BAD_REQUEST, "Invalid tenant ID"))?;
	let key_id = Uuid::parse_str(&key_id_str)
		.map_err(|_| ReacherResponseError::new(StatusCode::BAD_REQUEST, "Invalid API key ID"))?;

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
			.map(|dt| dt.with_timezone(&chrono::Utc))
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

// ── Route registration ──────────────────────────────

/// POST /v1/admin/tenants/{tenant_id}/api-keys
///
/// Create a new API key for a tenant.
#[utoipa::path(
	post,
	path = "/v1/admin/tenants/{tenant_id}/api-keys",
	tag = "Admin",
	params(("tenant_id" = Uuid, Path, description = "Tenant identifier")),
	responses((status = 201, description = "API key created")),
)]
pub fn create_api_key(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "admin" / "tenants" / String / "api-keys")
		.and(warp::post())
		.and(check_header(Arc::clone(&config)))
		.and(with_pg_pool(config))
		.and(warp::body::json())
		.and_then(create_handler)
		.with(warp::log(LOG_TARGET))
	}

/// GET /v1/admin/api-keys
///
/// List all API keys across tenants with optional filtering.
#[utoipa::path(
	get,
	path = "/v1/admin/api-keys",
	tag = "Admin",
	responses((status = 200, description = "All API keys")),
)]
pub fn list_all_api_keys(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "admin" / "api-keys")
		.and(warp::get())
		.and(check_header(Arc::clone(&config)))
		.and(with_pg_pool(config))
		.and(warp::query::<ListAllApiKeysQuery>())
		.and_then(list_all_handler)
		.with(warp::log("reacher_backend::v1::admin::api_keys::list_all"))
	}

/// GET /v1/admin/tenants/{tenant_id}/api-keys
///
/// List API keys for a tenant.
#[utoipa::path(
	get,
	path = "/v1/admin/tenants/{tenant_id}/api-keys",
	tag = "Admin",
	params(("tenant_id" = String, Path, description = "Tenant identifier")),
	responses((status = 200, description = "Tenant API keys")),
)]
pub fn list_api_keys(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "admin" / "tenants" / String / "api-keys")
		.and(warp::get())
		.and(check_header(Arc::clone(&config)))
		.and(with_pg_pool(config))
		.and_then(list_handler)
		.with(warp::log(LOG_TARGET))
	}

/// GET /v1/admin/tenants/{tenant_id}/api-keys/{key_id}
///
/// Fetch one API key by tenant and key ID.
#[utoipa::path(
	get,
	path = "/v1/admin/tenants/{tenant_id}/api-keys/{key_id}",
	tag = "Admin",
	params(
		("tenant_id" = String, Path, description = "Tenant identifier"),
		("key_id" = Uuid, Path, description = "API key identifier"),
	),
	responses((status = 200, description = "API key details")),
)]
pub fn get_api_key(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "admin" / "tenants" / String / "api-keys" / String)
		.and(warp::get())
		.and(check_header(Arc::clone(&config)))
		.and(with_pg_pool(config))
		.and_then(get_handler)
		.with(warp::log(LOG_TARGET))
	}

/// PATCH /v1/admin/tenants/{tenant_id}/api-keys/{key_id}
///
/// Update metadata for an API key.
#[utoipa::path(
	patch,
	path = "/v1/admin/tenants/{tenant_id}/api-keys/{key_id}",
	tag = "Admin",
	params(
		("tenant_id" = String, Path, description = "Tenant identifier"),
		("key_id" = Uuid, Path, description = "API key identifier"),
	),
	responses((status = 200, description = "API key updated")),
)]
pub fn update_api_key(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "admin" / "tenants" / String / "api-keys" / String)
		.and(warp::patch())
		.and(check_header(Arc::clone(&config)))
		.and(with_pg_pool(config))
		.and(warp::body::json())
		.and_then(update_handler)
		.with(warp::log(LOG_TARGET))
	}

/// DELETE /v1/admin/tenants/{tenant_id}/api-keys/{key_id}
///
/// Revoke an API key.
#[utoipa::path(
	delete,
	path = "/v1/admin/tenants/{tenant_id}/api-keys/{key_id}",
	tag = "Admin",
	params(
		("tenant_id" = String, Path, description = "Tenant identifier"),
		("key_id" = Uuid, Path, description = "API key identifier"),
	),
	responses((status = 200, description = "API key revoked")),
)]
pub fn revoke_api_key(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "admin" / "tenants" / String / "api-keys" / String)
		.and(warp::delete())
		.and(check_header(Arc::clone(&config)))
		.and(with_pg_pool(config))
		.and_then(revoke_handler)
		.with(warp::log(LOG_TARGET))
	}

/// POST /v1/admin/tenants/{tenant_id}/api-keys/{key_id}/reactivate
///
/// Reactivate a previously revoked API key.
#[utoipa::path(
	post,
	path = "/v1/admin/tenants/{tenant_id}/api-keys/{key_id}/reactivate",
	tag = "Admin",
	params(
		("tenant_id" = String, Path, description = "Tenant identifier"),
		("key_id" = Uuid, Path, description = "API key identifier"),
	),
	responses((status = 200, description = "API key reactivated")),
)]
pub fn reactivate_api_key(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "admin" / "tenants" / String / "api-keys" / String / "reactivate")
		.and(warp::post())
		.and(check_header(Arc::clone(&config)))
		.and(with_pg_pool(config))
		.and_then(reactivate_handler)
		.with(warp::log(LOG_TARGET))
}
