use crate::config::BackendConfig;
use crate::http::check_admin_header;
use crate::http::ReacherResponseError;
use check_if_email_exists::LOG_TARGET;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use std::sync::Arc;
use uuid::Uuid;
use warp::http::StatusCode;
use warp::Filter;

// ── Request/Response types ──────────────────────────

#[derive(Debug, Deserialize)]
pub struct CreateTenantRequest {
	pub name: String,
	pub slug: String,
	pub contact_email: String,
	pub plan_tier: Option<String>,
	pub monthly_email_limit: Option<i32>,
	pub max_requests_per_second: Option<i32>,
	pub max_requests_per_minute: Option<i32>,
	pub max_requests_per_hour: Option<i32>,
	pub max_requests_per_day: Option<i32>,
	pub default_webhook_url: Option<String>,
	pub webhook_signing_secret: Option<String>,
	pub result_retention_days: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTenantRequest {
	pub name: Option<String>,
	pub contact_email: Option<String>,
	pub plan_tier: Option<String>,
	pub status: Option<String>,
	// Nullable columns: `null` clears them (no limit, no webhook).
	#[serde(default, deserialize_with = "crate::http::present_nullable")]
	pub monthly_email_limit: Option<Option<i32>>,
	#[serde(default, deserialize_with = "crate::http::present_nullable")]
	pub max_requests_per_second: Option<Option<i32>>,
	#[serde(default, deserialize_with = "crate::http::present_nullable")]
	pub max_requests_per_minute: Option<Option<i32>>,
	#[serde(default, deserialize_with = "crate::http::present_nullable")]
	pub max_requests_per_hour: Option<Option<i32>>,
	#[serde(default, deserialize_with = "crate::http::present_nullable")]
	pub max_requests_per_day: Option<Option<i32>>,
	#[serde(default, deserialize_with = "crate::http::present_nullable")]
	pub default_webhook_url: Option<Option<String>>,
	#[serde(default, deserialize_with = "crate::http::present_nullable")]
	pub webhook_signing_secret: Option<Option<String>>,
	pub result_retention_days: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct TenantResponse {
	pub id: Uuid,
	pub name: String,
	pub slug: String,
	pub contact_email: String,
	pub plan_tier: String,
	pub status: String,
	pub monthly_email_limit: Option<i32>,
	pub max_requests_per_second: Option<i32>,
	pub max_requests_per_minute: Option<i32>,
	pub max_requests_per_hour: Option<i32>,
	pub max_requests_per_day: Option<i32>,
	pub used_this_period: i32,
	pub default_webhook_url: Option<String>,
	pub result_retention_days: i32,
	pub created_at: String,
	pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct ListQuery {
	pub limit: Option<i64>,
	pub offset: Option<i64>,
	pub status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ListResponse {
	pub tenants: Vec<TenantResponse>,
	pub total: i64,
}

// ── Helpers ─────────────────────────────────────────

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

fn row_to_response(row: &sqlx::postgres::PgRow) -> TenantResponse {
	TenantResponse {
		id: row.get("id"),
		name: row.get("name"),
		slug: row.get("slug"),
		contact_email: row.get("contact_email"),
		plan_tier: row.get("plan_tier"),
		status: row.get("status"),
		monthly_email_limit: row.get("monthly_email_limit"),
		max_requests_per_second: row.get("max_requests_per_second"),
		max_requests_per_minute: row.get("max_requests_per_minute"),
		max_requests_per_hour: row.get("max_requests_per_hour"),
		max_requests_per_day: row.get("max_requests_per_day"),
		used_this_period: row.get("used_this_period"),
		default_webhook_url: row.get("default_webhook_url"),
		result_retention_days: row.get("result_retention_days"),
		created_at: row
			.get::<chrono::DateTime<chrono::Utc>, _>("created_at")
			.to_rfc3339(),
		updated_at: row
			.get::<chrono::DateTime<chrono::Utc>, _>("updated_at")
			.to_rfc3339(),
	}
}

// ── Validation ──────────────────────────────────────

const PLAN_TIERS: [&str; 4] = ["free", "starter", "professional", "enterprise"];
const TENANT_STATUSES: [&str; 3] = ["active", "suspended", "deactivated"];

fn bad_request(message: impl Into<String>) -> warp::Rejection {
	ReacherResponseError::new(StatusCode::BAD_REQUEST, message.into()).into()
}

/// Reject values that the database would refuse with a 500, or that would
/// break the tenant: a rate limit below 1 blocks every request (and a
/// negative one wraps to "unlimited" in the throttle), and a retention period
/// outside the shared range either deletes every result at the next prune or
/// overflows the prune's date arithmetic for all tenants.
fn validate_tenant_fields(
	plan_tier: Option<&str>,
	status: Option<&str>,
	rate_limits: [Option<i32>; 4],
	result_retention_days: Option<i32>,
) -> Result<(), warp::Rejection> {
	if let Some(tier) = plan_tier {
		if !PLAN_TIERS.contains(&tier) {
			return Err(bad_request(format!(
				"plan_tier must be one of: {}",
				PLAN_TIERS.join(", ")
			)));
		}
	}
	if let Some(status) = status {
		if !TENANT_STATUSES.contains(&status) {
			return Err(bad_request(format!(
				"status must be one of: {}",
				TENANT_STATUSES.join(", ")
			)));
		}
	}
	if rate_limits.iter().flatten().any(|limit| *limit < 1) {
		return Err(bad_request("max_requests_per_* limits must be at least 1"));
	}
	if let Some(days) = result_retention_days {
		crate::http::v1::tenant_settings::validate_result_retention_days(days)?;
	}
	Ok(())
}

// ── Handlers ────────────────────────────────────────

async fn create_handler(
	pg_pool: PgPool,
	body: CreateTenantRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
	if body.name.is_empty() || body.slug.is_empty() || body.contact_email.is_empty() {
		return Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"name, slug, and contact_email are required",
		)
		.into());
	}
	validate_tenant_fields(
		body.plan_tier.as_deref(),
		None,
		[
			body.max_requests_per_second,
			body.max_requests_per_minute,
			body.max_requests_per_hour,
			body.max_requests_per_day,
		],
		body.result_retention_days,
	)?;

	let plan_tier = body.plan_tier.as_deref().unwrap_or("free");
	let retention = body.result_retention_days.unwrap_or(30);

	let row = sqlx::query(
		"INSERT INTO tenants (name, slug, contact_email, plan_tier, monthly_email_limit, \
		 max_requests_per_second, max_requests_per_minute, max_requests_per_hour, max_requests_per_day, \
		 default_webhook_url, webhook_signing_secret, result_retention_days) \
		 VALUES ($1, $2, $3, $4::plan_tier, $5, $6, $7, $8, $9, $10, $11, $12) \
		 RETURNING id, name, slug, contact_email, plan_tier::TEXT, status::TEXT, monthly_email_limit, \
		 max_requests_per_second, max_requests_per_minute, max_requests_per_hour, max_requests_per_day, \
		 used_this_period, default_webhook_url, result_retention_days, created_at, updated_at",
	)
	.bind(&body.name)
	.bind(&body.slug)
	.bind(&body.contact_email)
	.bind(plan_tier)
	.bind(body.monthly_email_limit)
	.bind(body.max_requests_per_second)
	.bind(body.max_requests_per_minute)
	.bind(body.max_requests_per_hour)
	.bind(body.max_requests_per_day)
	.bind(&body.default_webhook_url)
	.bind(&body.webhook_signing_secret)
	.bind(retention)
	.fetch_one(&pg_pool)
	.await
	.map_err(|e| {
		// Check for PG unique violation error code 23505
		if let sqlx::Error::Database(ref db_err) = e {
			if db_err.code().as_deref() == Some("23505") {
				return ReacherResponseError::new(
					StatusCode::CONFLICT,
					format!("Tenant with slug '{}' already exists", body.slug),
				);
			}
		}
		ReacherResponseError::from(e)
	})?;

	Ok(warp::reply::with_status(
		warp::reply::json(&row_to_response(&row)),
		StatusCode::CREATED,
	))
}

async fn list_handler(
	pg_pool: PgPool,
	query: ListQuery,
) -> Result<impl warp::Reply, warp::Rejection> {
	let limit = query.limit.unwrap_or(50).clamp(1, 200);
	let offset = query.offset.unwrap_or(0).max(0);
	validate_tenant_fields(None, query.status.as_deref(), [None; 4], None)?;

	let (rows, total) = if let Some(ref status) = query.status {
		let rows = sqlx::query(
			"SELECT id, name, slug, contact_email, plan_tier::TEXT, status::TEXT, monthly_email_limit, \
			 max_requests_per_second, max_requests_per_minute, max_requests_per_hour, max_requests_per_day, \
			 used_this_period, default_webhook_url, result_retention_days, created_at, updated_at \
			 FROM tenants WHERE status = $3::tenant_status ORDER BY created_at DESC LIMIT $1 OFFSET $2",
		)
		.bind(limit)
		.bind(offset)
		.bind(status)
		.fetch_all(&pg_pool)
		.await
		.map_err(ReacherResponseError::from)?;

		let total: i64 =
			sqlx::query("SELECT COUNT(*) as count FROM tenants WHERE status = $1::tenant_status")
				.bind(status)
				.fetch_one(&pg_pool)
				.await
				.map_err(ReacherResponseError::from)?
				.get("count");

		(rows, total)
	} else {
		let rows = sqlx::query(
			"SELECT id, name, slug, contact_email, plan_tier::TEXT, status::TEXT, monthly_email_limit, \
			 max_requests_per_second, max_requests_per_minute, max_requests_per_hour, max_requests_per_day, \
			 used_this_period, default_webhook_url, result_retention_days, created_at, updated_at \
			 FROM tenants ORDER BY created_at DESC LIMIT $1 OFFSET $2",
		)
		.bind(limit)
		.bind(offset)
		.fetch_all(&pg_pool)
		.await
		.map_err(ReacherResponseError::from)?;

		let total: i64 = sqlx::query("SELECT COUNT(*) as count FROM tenants")
			.fetch_one(&pg_pool)
			.await
			.map_err(ReacherResponseError::from)?
			.get("count");

		(rows, total)
	};

	let tenants: Vec<TenantResponse> = rows.iter().map(row_to_response).collect();
	Ok(warp::reply::json(&ListResponse { tenants, total }))
}

async fn get_handler(id: String, pg_pool: PgPool) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id: Uuid = id.parse().map_err(|_| {
		ReacherResponseError::new(StatusCode::BAD_REQUEST, "Invalid tenant ID format")
	})?;

	let row = sqlx::query(
		"SELECT id, name, slug, contact_email, plan_tier::TEXT, status::TEXT, monthly_email_limit, \
		 max_requests_per_second, max_requests_per_minute, max_requests_per_hour, max_requests_per_day, \
		 used_this_period, default_webhook_url, result_retention_days, created_at, updated_at \
		 FROM tenants WHERE id = $1",
	)
	.bind(tenant_id)
	.fetch_optional(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	match row {
		Some(r) => Ok(warp::reply::json(&row_to_response(&r))),
		None => Err(ReacherResponseError::new(StatusCode::NOT_FOUND, "Tenant not found").into()),
	}
}

async fn update_handler(
	id: String,
	pg_pool: PgPool,
	body: UpdateTenantRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id: Uuid = id.parse().map_err(|_| {
		ReacherResponseError::new(StatusCode::BAD_REQUEST, "Invalid tenant ID format")
	})?;

	validate_tenant_fields(
		body.plan_tier.as_deref(),
		body.status.as_deref(),
		[
			body.max_requests_per_second.flatten(),
			body.max_requests_per_minute.flatten(),
			body.max_requests_per_hour.flatten(),
			body.max_requests_per_day.flatten(),
		],
		body.result_retention_days,
	)?;

	// Build dynamic UPDATE query
	let mut sets: Vec<String> = vec![];
	let mut param_idx = 2u32; // $1 is tenant_id

	if body.name.is_some() {
		sets.push(format!("name = ${}", param_idx));
		param_idx += 1;
	}
	if body.contact_email.is_some() {
		sets.push(format!("contact_email = ${}", param_idx));
		param_idx += 1;
	}
	if body.plan_tier.is_some() {
		sets.push(format!("plan_tier = ${}::plan_tier", param_idx));
		// An operator's tier is kept when the billing plan later changes
		// (see migration 20261016000002_plan_from_billing).
		sets.push("plan_from_billing = false".to_string());
		param_idx += 1;
	}
	if body.status.is_some() {
		sets.push(format!("status = ${}::tenant_status", param_idx));
		param_idx += 1;
	}
	if body.monthly_email_limit.is_some() {
		sets.push(format!("monthly_email_limit = ${}", param_idx));
		param_idx += 1;
	}
	if body.max_requests_per_second.is_some() {
		sets.push(format!("max_requests_per_second = ${}", param_idx));
		param_idx += 1;
	}
	if body.max_requests_per_minute.is_some() {
		sets.push(format!("max_requests_per_minute = ${}", param_idx));
		param_idx += 1;
	}
	if body.max_requests_per_hour.is_some() {
		sets.push(format!("max_requests_per_hour = ${}", param_idx));
		param_idx += 1;
	}
	if body.max_requests_per_day.is_some() {
		sets.push(format!("max_requests_per_day = ${}", param_idx));
		param_idx += 1;
	}
	if body.default_webhook_url.is_some() {
		sets.push(format!("default_webhook_url = ${}", param_idx));
		param_idx += 1;
	}
	if body.webhook_signing_secret.is_some() {
		sets.push(format!("webhook_signing_secret = ${}", param_idx));
		param_idx += 1;
	}
	if body.result_retention_days.is_some() {
		sets.push(format!("result_retention_days = ${}", param_idx));
	}

	if sets.is_empty() {
		return Err(
			ReacherResponseError::new(StatusCode::BAD_REQUEST, "No fields to update").into(),
		);
	}

	let sql = format!(
		"UPDATE tenants SET {} WHERE id = $1 \
		 RETURNING id, name, slug, contact_email, plan_tier::TEXT, status::TEXT, monthly_email_limit, \
		 max_requests_per_second, max_requests_per_minute, max_requests_per_hour, max_requests_per_day, \
		 used_this_period, default_webhook_url, result_retention_days, created_at, updated_at",
		sets.join(", ")
	);

	let mut query = sqlx::query(&sql).bind(tenant_id);

	if let Some(ref v) = body.name {
		query = query.bind(v);
	}
	if let Some(ref v) = body.contact_email {
		query = query.bind(v);
	}
	if let Some(ref v) = body.plan_tier {
		query = query.bind(v);
	}
	if let Some(ref v) = body.status {
		query = query.bind(v);
	}
	if let Some(v) = body.monthly_email_limit {
		query = query.bind(v);
	}
	if let Some(v) = body.max_requests_per_second {
		query = query.bind(v);
	}
	if let Some(v) = body.max_requests_per_minute {
		query = query.bind(v);
	}
	if let Some(v) = body.max_requests_per_hour {
		query = query.bind(v);
	}
	if let Some(v) = body.max_requests_per_day {
		query = query.bind(v);
	}
	if let Some(ref v) = body.default_webhook_url {
		query = query.bind(v);
	}
	if let Some(ref v) = body.webhook_signing_secret {
		query = query.bind(v);
	}
	if let Some(v) = body.result_retention_days {
		query = query.bind(v);
	}

	let row = query
		.fetch_optional(&pg_pool)
		.await
		.map_err(ReacherResponseError::from)?;

	match row {
		Some(r) => Ok(warp::reply::json(&row_to_response(&r))),
		None => Err(ReacherResponseError::new(StatusCode::NOT_FOUND, "Tenant not found").into()),
	}
}

async fn delete_handler(id: String, pg_pool: PgPool) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id: Uuid = id.parse().map_err(|_| {
		ReacherResponseError::new(StatusCode::BAD_REQUEST, "Invalid tenant ID format")
	})?;

	let result = sqlx::query("DELETE FROM tenants WHERE id = $1")
		.bind(tenant_id)
		.execute(&pg_pool)
		.await
		.map_err(|error| match &error {
			// Jobs, lists and finder jobs reference the tenant without a
			// cascade, so a tenant that has done work cannot be deleted.
			sqlx::Error::Database(db_error) if db_error.code().as_deref() == Some("23503") => {
				ReacherResponseError::new(
					StatusCode::CONFLICT,
					"Tenant still owns jobs, lists or other records; deactivate it instead",
				)
			}
			_ => ReacherResponseError::from(error),
		})?;

	if result.rows_affected() == 0 {
		return Err(ReacherResponseError::new(StatusCode::NOT_FOUND, "Tenant not found").into());
	}

	Ok(warp::reply::with_status(
		warp::reply::json(&serde_json::json!({"deleted": true})),
		StatusCode::OK,
	))
}

// ── Route registration ──────────────────────────────

/// POST /v1/admin/tenants
///
/// Create a new tenant.
#[utoipa::path(
	post,
	path = "/v1/admin/tenants",
	tag = "Admin",
	responses((status = 201, description = "Tenant created")),
)]
pub fn create_tenant(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "admin" / "tenants")
		.and(warp::post())
		.and(check_admin_header(Arc::clone(&config)))
		.and(with_pg_pool(config))
		.and(warp::body::json())
		.and_then(create_handler)
		.with(warp::log(LOG_TARGET))
}

/// GET /v1/admin/tenants
///
/// List tenants with optional status and pagination filters.
#[utoipa::path(
	get,
	path = "/v1/admin/tenants",
	tag = "Admin",
	responses((status = 200, description = "Tenant list")),
)]
pub fn list_tenants(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "admin" / "tenants")
		.and(warp::get())
		.and(check_admin_header(Arc::clone(&config)))
		.and(with_pg_pool(config))
		.and(warp::query::<ListQuery>())
		.and_then(list_handler)
		.with(warp::log(LOG_TARGET))
}

/// GET /v1/admin/tenants/{tenant_id}
///
/// Fetch tenant details by tenant ID.
#[utoipa::path(
	get,
	path = "/v1/admin/tenants/{tenant_id}",
	tag = "Admin",
	params(("tenant_id" = String, Path, description = "Tenant identifier")),
	responses((status = 200, description = "Tenant details")),
)]
pub fn get_tenant(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "admin" / "tenants" / String)
		.and(warp::get())
		.and(check_admin_header(Arc::clone(&config)))
		.and(with_pg_pool(config))
		.and_then(get_handler)
		.with(warp::log(LOG_TARGET))
}

/// PUT /v1/admin/tenants/{tenant_id}
///
/// Update tenant fields.
#[utoipa::path(
	put,
	path = "/v1/admin/tenants/{tenant_id}",
	tag = "Admin",
	params(("tenant_id" = String, Path, description = "Tenant identifier")),
	responses((status = 200, description = "Tenant updated")),
)]
pub fn update_tenant(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "admin" / "tenants" / String)
		.and(warp::put())
		.and(check_admin_header(Arc::clone(&config)))
		.and(with_pg_pool(config))
		.and(warp::body::json())
		.and_then(update_handler)
		.with(warp::log(LOG_TARGET))
}

/// DELETE /v1/admin/tenants/{tenant_id}
///
/// Delete a tenant.
#[utoipa::path(
	delete,
	path = "/v1/admin/tenants/{tenant_id}",
	tag = "Admin",
	params(("tenant_id" = String, Path, description = "Tenant identifier")),
	responses((status = 200, description = "Tenant deleted")),
)]
pub fn delete_tenant(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "admin" / "tenants" / String)
		.and(warp::delete())
		.and(check_admin_header(Arc::clone(&config)))
		.and(with_pg_pool(config))
		.and_then(delete_handler)
		.with(warp::log(LOG_TARGET))
}
