use crate::config::BackendConfig;
use crate::http::{resolve_tenant, ReacherResponseError};
use crate::tenant::context::TenantContext;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use std::sync::Arc;
use warp::http::StatusCode;
use warp::Filter;

#[derive(Debug, Serialize)]
struct TenantDomainResponse {
	pub id: uuid::Uuid,
	pub tenant_id: uuid::Uuid,
	pub domain: String,
	pub is_active: bool,
	pub is_verified: bool,
	pub notes: Option<String>,
	pub created_at: String,
	pub updated_at: String,
}

#[derive(Debug, Serialize)]
struct TenantDomainsListResponse {
	pub domains: Vec<TenantDomainResponse>,
}

#[derive(Debug, Deserialize)]
struct CreateTenantDomainRequest {
	pub domain: String,
	pub is_active: Option<bool>,
	pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
struct UpdateTenantDomainRequest {
	pub domain: Option<String>,
	pub is_active: Option<bool>,
	pub is_verified: Option<bool>,
	pub notes: Option<Option<String>>,
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
					"Postgres database required for tenant domain endpoints",
				))
			})
		}
	})
}

fn ensure_tenant_id(tenant_ctx: TenantContext) -> Result<uuid::Uuid, warp::Rejection> {
	tenant_ctx.tenant_id.ok_or_else(|| {
		warp::reject::custom(ReacherResponseError::new(
			StatusCode::UNAUTHORIZED,
			"Tenant context is required for tenant domain endpoints",
		))
	})
}

fn normalize_domain(raw: String) -> Result<String, ReacherResponseError> {
	let mut domain = raw.trim().to_ascii_lowercase();
	if domain.is_empty() {
		return Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"Domain is required",
		));
	}

	if domain.len() > 253 {
		return Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"Domain is too long",
		));
	}

	if !domain
		.chars()
		.all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-')
	{
		return Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"Domain contains invalid characters",
		));
	}

	domain = domain
		.trim_end_matches('.')
		.trim_start_matches('.')
		.to_string();
	if domain.is_empty() || !domain.contains('.') {
		return Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"Domain must include a TLD",
		));
	}

	Ok(domain)
}

fn row_to_domain(row: &sqlx::postgres::PgRow) -> TenantDomainResponse {
	TenantDomainResponse {
		id: row.get("id"),
		tenant_id: row.get("tenant_id"),
		domain: row.get("domain"),
		is_active: row.get("is_active"),
		is_verified: row.get("is_verified"),
		notes: row.get("notes"),
		created_at: row.get::<DateTime<Utc>, _>("created_at").to_rfc3339(),
		updated_at: row.get::<DateTime<Utc>, _>("updated_at").to_rfc3339(),
	}
}

async fn list_domains_handler(
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id = ensure_tenant_id(tenant_ctx)?;

	let rows = sqlx::query(
		"SELECT id, tenant_id, domain, is_active, is_verified, notes, created_at, updated_at \
		 FROM tenant_domains WHERE tenant_id = $1 ORDER BY domain ASC",
	)
	.bind(tenant_id)
	.fetch_all(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let domains = rows.iter().map(row_to_domain).collect();

	Ok(warp::reply::json(&TenantDomainsListResponse { domains }))
}

async fn create_domain_handler(
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	body: CreateTenantDomainRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id = ensure_tenant_id(tenant_ctx)?;
	let domain = normalize_domain(body.domain)?;
	let is_active = body.is_active.unwrap_or(true);

	let row = sqlx::query(
		"INSERT INTO tenant_domains (tenant_id, domain, is_active, notes) \
		 VALUES ($1, $2, $3, $4) \
		 RETURNING id, tenant_id, domain, is_active, is_verified, notes, created_at, updated_at",
	)
	.bind(tenant_id)
	.bind(&domain)
	.bind(is_active)
	.bind(&body.notes)
	.fetch_one(&pg_pool)
	.await
	.map_err(|e| {
		if let sqlx::Error::Database(ref db_err) = e {
			if db_err.code().as_deref() == Some("23505") {
				return ReacherResponseError::new(
					StatusCode::CONFLICT,
					"Domain already exists for this tenant",
				);
			}
		}
		ReacherResponseError::from(e)
	})?;

	Ok(warp::reply::with_status(
		warp::reply::json(&row_to_domain(&row)),
		StatusCode::CREATED,
	))
}

async fn get_domain_handler(
	domain: String,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id = ensure_tenant_id(tenant_ctx)?;
	let domain = normalize_domain(domain)?;

	let row = sqlx::query(
		"SELECT id, tenant_id, domain, is_active, is_verified, notes, created_at, updated_at \
		 FROM tenant_domains WHERE tenant_id = $1 AND lower(domain) = lower($2)",
	)
	.bind(tenant_id)
	.bind(&domain)
	.fetch_optional(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	match row {
		Some(row) => Ok(warp::reply::json(&row_to_domain(&row))),
		None => Err(ReacherResponseError::new(StatusCode::NOT_FOUND, "Domain not found").into()),
	}
}

async fn update_domain_handler(
	domain: String,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	body: UpdateTenantDomainRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id = ensure_tenant_id(tenant_ctx)?;
	let target_domain = normalize_domain(domain)?;

	if body.domain.is_none()
		&& body.is_active.is_none()
		&& body.is_verified.is_none()
		&& body.notes.is_none()
	{
		return Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"No domain fields provided",
		)
		.into());
	}

	let mut sets = Vec::new();
	let mut bind_idx = 3u32;
	let mut next_domain: Option<String> = None;

	if let Some(value) = body.domain {
		let normalized = normalize_domain(value)?;
		sets.push(format!("domain = ${}", bind_idx));
		next_domain = Some(normalized);
		bind_idx += 1;
	}
	if body.is_active.is_some() {
		sets.push(format!("is_active = ${}", bind_idx));
		bind_idx += 1;
	}
	if body.is_verified.is_some() {
		sets.push(format!("is_verified = ${}", bind_idx));
		bind_idx += 1;
	}
	if body.notes.is_some() {
		sets.push(format!("notes = ${}", bind_idx));
	}

	let sql = format!(
		"UPDATE tenant_domains SET {} WHERE tenant_id = $1 AND lower(domain) = lower($2) \
		 RETURNING id, tenant_id, domain, is_active, is_verified, notes, created_at, updated_at",
		sets.join(", ")
	);

	let mut query = sqlx::query(&sql).bind(tenant_id).bind(&target_domain);
	if let Some(value) = next_domain {
		query = query.bind(value);
	}
	if let Some(is_active) = body.is_active {
		query = query.bind(is_active);
	}
	if let Some(is_verified) = body.is_verified {
		query = query.bind(is_verified);
	}
	if let Some(notes) = body.notes {
		query = query.bind(notes);
	}

	let row = query.fetch_optional(&pg_pool).await.map_err(|e| {
		if let sqlx::Error::Database(ref db_err) = e {
			if db_err.code().as_deref() == Some("23505") {
				return ReacherResponseError::new(
					StatusCode::CONFLICT,
					"Domain already exists for this tenant",
				);
			}
		}
		ReacherResponseError::from(e)
	})?;

	match row {
		Some(row) => Ok(warp::reply::json(&row_to_domain(&row))),
		None => Err(ReacherResponseError::new(StatusCode::NOT_FOUND, "Domain not found").into()),
	}
}

async fn delete_domain_handler(
	domain: String,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id = ensure_tenant_id(tenant_ctx)?;
	let domain = normalize_domain(domain)?;

	let row = sqlx::query(
		"DELETE FROM tenant_domains WHERE tenant_id = $1 AND lower(domain) = lower($2) RETURNING id, domain",
	)
	.bind(tenant_id)
	.bind(domain)
	.fetch_optional(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	match row {
		Some(row) => Ok(warp::reply::json(&serde_json::json!({
			"deleted": true,
			"tenant_domain_id": row.get::<uuid::Uuid, _>("id"),
			"domain": row.get::<String, _>("domain"),
		}))),
		None => Err(ReacherResponseError::new(StatusCode::NOT_FOUND, "Domain not found").into()),
	}
}

/// GET /v1/me/domains
///
/// List all active and inactive domain entries for the authenticated tenant.
#[utoipa::path(
	get,
	path = "/v1/me/domains",
	tag = "Tenant",
	responses((status = 200, description = "Tenant domains list")),
)]
pub fn v1_list_tenant_domains(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "me" / "domains")
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_pg_pool(config))
		.and_then(list_domains_handler)
		.with(warp::log("reacher_backend::v1::tenant_domains::list"))
}

/// POST /v1/me/domains
///
/// Add a domain for the authenticated tenant.
#[utoipa::path(
	post,
	path = "/v1/me/domains",
	tag = "Tenant",
	responses((status = 201, description = "Tenant domain created")),
)]
pub fn v1_create_tenant_domain(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "me" / "domains")
		.and(warp::post())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_pg_pool(config))
		.and(warp::body::json())
		.and_then(create_domain_handler)
		.with(warp::log("reacher_backend::v1::tenant_domains::create"))
}

/// GET /v1/me/domains/{domain}
///
/// Get one domain entry for the authenticated tenant.
#[utoipa::path(
	get,
	path = "/v1/me/domains/{domain}",
	tag = "Tenant",
	params(("domain" = String, Path, description = "Domain identifier")),
	responses((status = 200, description = "Tenant domain details")),
)]
pub fn v1_get_tenant_domain(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "me" / "domains" / String)
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_pg_pool(config))
		.and_then(get_domain_handler)
		.with(warp::log("reacher_backend::v1::tenant_domains::get"))
}

/// PATCH /v1/me/domains/{domain}
///
/// Update the domain value, status, verification state, or metadata notes.
#[utoipa::path(
	patch,
	path = "/v1/me/domains/{domain}",
	tag = "Tenant",
	params(("domain" = String, Path, description = "Domain identifier")),
	responses((status = 200, description = "Tenant domain updated")),
)]
pub fn v1_update_tenant_domain(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "me" / "domains" / String)
		.and(warp::patch())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_pg_pool(config))
		.and(warp::body::json())
		.and_then(update_domain_handler)
		.with(warp::log("reacher_backend::v1::tenant_domains::update"))
}

/// DELETE /v1/me/domains/{domain}
///
/// Remove a domain from the authenticated tenant.
#[utoipa::path(
	delete,
	path = "/v1/me/domains/{domain}",
	tag = "Tenant",
	params(("domain" = String, Path, description = "Domain identifier")),
	responses((status = 200, description = "Tenant domain deleted")),
)]
pub fn v1_delete_tenant_domain(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "me" / "domains" / String)
		.and(warp::delete())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_pg_pool(config))
		.and_then(delete_domain_handler)
		.with(warp::log("reacher_backend::v1::tenant_domains::delete"))
}
