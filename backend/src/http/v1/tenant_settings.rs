use crate::config::BackendConfig;
use crate::http::resolve_tenant;
use crate::http::ReacherResponseError;
use crate::tenant::context::TenantContext;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use std::sync::Arc;
use uuid::Uuid;
use warp::http::StatusCode;
use warp::Filter;

#[derive(Debug, Deserialize)]
struct UpdateTenantSettingsRequest {
	pub default_webhook_url: Option<Option<String>>,
	pub webhook_signing_secret: Option<Option<String>>,
	pub result_retention_days: Option<i32>,
}

#[derive(Debug, Deserialize)]
struct UpdateWebhookRequest {
	pub default_webhook_url: Option<Option<String>>,
	pub webhook_signing_secret: Option<Option<String>>,
}

#[derive(Debug, Serialize)]
struct TenantSettingsResponse {
	pub tenant_id: Uuid,
	pub name: String,
	pub slug: String,
	pub monthly_email_limit: Option<i32>,
	pub used_this_period: i32,
	pub period_reset_at: String,
	pub result_retention_days: i32,
	pub default_webhook_url: Option<String>,
}

#[derive(Debug, Serialize)]
struct TenantWebhookResponse {
	pub tenant_id: Uuid,
	pub tenant_name: String,
	pub default_webhook_url: Option<String>,
	pub webhook_signing_secret_configured: bool,
}

#[derive(Debug, Serialize)]
struct TenantUsageResponse {
	pub tenant_id: Uuid,
	pub tenant_name: String,
	pub plan_tier: String,
	pub monthly_email_limit: Option<i32>,
	pub used_this_period: i32,
	pub period_reset_at: String,
	pub quota_unlimited: bool,
	pub quota_remaining: Option<i32>,
}

fn with_pg_pool(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (PgPool,), Error = warp::Rejection> + Clone {
	warp::any().and_then(move || {
		let config: Arc<BackendConfig> = Arc::clone(&config);
		async move {
			config.get_pg_pool().ok_or_else(|| {
				warp::reject::custom(ReacherResponseError::new(
					StatusCode::SERVICE_UNAVAILABLE,
					"Postgres database required for tenant settings endpoints",
				))
			})
		}
	})
}

fn ensure_tenant_id(tenant_ctx: TenantContext) -> Result<Uuid, warp::Rejection> {
	tenant_ctx.tenant_id.ok_or_else(|| {
		warp::reject::custom(ReacherResponseError::new(
			StatusCode::UNAUTHORIZED,
			"Tenant context is required for tenant settings endpoints",
		))
	})
}

fn row_to_settings(row: &sqlx::postgres::PgRow) -> TenantSettingsResponse {
	TenantSettingsResponse {
		tenant_id: row.get("id"),
		name: row.get("name"),
		slug: row.get("slug"),
		monthly_email_limit: row.get("monthly_email_limit"),
		used_this_period: row.get("used_this_period"),
		period_reset_at: row.get::<DateTime<Utc>, _>("period_reset_at").to_rfc3339(),
		result_retention_days: row.get("result_retention_days"),
		default_webhook_url: row.get("default_webhook_url"),
	}
}

fn row_to_webhook(row: &sqlx::postgres::PgRow) -> TenantWebhookResponse {
	let configured: Option<String> = row.get("webhook_signing_secret");
	let tenant_id: Uuid = row.get("id");
	let tenant_name: String = row.get("name");
	let default_webhook_url: Option<String> = row.get("default_webhook_url");
	let has_secret = configured.as_ref().map_or(false, |s| !s.is_empty());

	TenantWebhookResponse {
		tenant_id,
		tenant_name,
		default_webhook_url,
		webhook_signing_secret_configured: has_secret,
	}
}

async fn settings_handler(
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id = ensure_tenant_id(tenant_ctx)?;

	let row = sqlx::query(
		"SELECT id, name, slug, monthly_email_limit, used_this_period, period_reset_at, result_retention_days, default_webhook_url \
		 FROM tenants WHERE id = $1",
	)
	.bind(tenant_id)
	.fetch_optional(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	match row {
		Some(r) => Ok(warp::reply::json(&row_to_settings(&r))),
		None => Err(ReacherResponseError::new(StatusCode::NOT_FOUND, "Tenant not found").into()),
	}
}

async fn webhook_handler(
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id = ensure_tenant_id(tenant_ctx)?;

	let row = sqlx::query(
		"SELECT id, name, default_webhook_url, webhook_signing_secret FROM tenants WHERE id = $1",
	)
	.bind(tenant_id)
	.fetch_optional(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	match row {
		Some(r) => Ok(warp::reply::json(&row_to_webhook(&r))),
		None => Err(ReacherResponseError::new(StatusCode::NOT_FOUND, "Tenant not found").into()),
	}
}

async fn usage_handler(tenant_ctx: TenantContext) -> Result<impl warp::Reply, warp::Rejection> {
	let limit = tenant_ctx.monthly_email_limit;
	let quota_unlimited = limit.is_none() || limit.unwrap_or(0) <= 0;
	let quota_remaining = match limit {
		Some(lim) if lim > 0 => Some((lim - tenant_ctx.used_this_period).max(0)),
		_ => None,
	};

	let tenant_id = tenant_ctx.tenant_id.ok_or_else(|| {
		warp::reject::custom(ReacherResponseError::new(
			StatusCode::UNAUTHORIZED,
			"Tenant context required",
		))
	})?;
	let response = TenantUsageResponse {
		tenant_id,
		tenant_name: tenant_ctx.tenant_name,
		plan_tier: format!("{:?}", tenant_ctx.plan_tier).to_lowercase(),
		monthly_email_limit: limit,
		used_this_period: tenant_ctx.used_this_period,
		period_reset_at: tenant_ctx.period_reset_at.to_rfc3339(),
		quota_unlimited,
		quota_remaining,
	};

	Ok(warp::reply::json(&response))
}

async fn update_settings_handler(
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	body: UpdateTenantSettingsRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id = ensure_tenant_id(tenant_ctx)?;

	if body.default_webhook_url.is_none()
		&& body.webhook_signing_secret.is_none()
		&& body.result_retention_days.is_none()
	{
		return Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"No settings fields provided",
		)
		.into());
	}

	if let Some(days) = body.result_retention_days {
		if days <= 0 {
			return Err(ReacherResponseError::new(
				StatusCode::BAD_REQUEST,
				"result_retention_days must be positive",
			)
			.into());
		}
	}

	let mut sets = Vec::new();
	let mut idx = 2u32;

	if body.default_webhook_url.is_some() {
		sets.push(format!("default_webhook_url = ${}", idx));
		idx += 1;
	}
	if body.webhook_signing_secret.is_some() {
		sets.push(format!("webhook_signing_secret = ${}", idx));
		idx += 1;
	}
	if body.result_retention_days.is_some() {
		sets.push(format!("result_retention_days = ${}", idx));
	}

	let sql = format!(
		"UPDATE tenants SET {} WHERE id = $1 \
		 RETURNING id, name, slug, monthly_email_limit, used_this_period, period_reset_at, result_retention_days, default_webhook_url",
		sets.join(", ")
	);

	let mut query = sqlx::query(&sql).bind(tenant_id);
	if let Some(v) = body.default_webhook_url {
		query = query.bind(v);
	}
	if let Some(v) = body.webhook_signing_secret {
		query = query.bind(v);
	}
	if let Some(v) = body.result_retention_days {
		query = query.bind(v);
	}

	let row = query
		.fetch_optional(&pg_pool)
		.await
		.map_err(ReacherResponseError::from)?;

	let row = match row {
		Some(r) => r,
		None => {
			return Err(ReacherResponseError::new(StatusCode::NOT_FOUND, "Tenant not found").into())
		}
	};

	Ok(warp::reply::json(&row_to_settings(&row)))
}

async fn update_webhook_handler(
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	body: UpdateWebhookRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id = ensure_tenant_id(tenant_ctx)?;

	if body.default_webhook_url.is_none() && body.webhook_signing_secret.is_none() {
		return Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"No webhook fields provided",
		)
		.into());
	}

	let row = sqlx::query(
		"UPDATE tenants SET default_webhook_url = $2, webhook_signing_secret = $3 WHERE id = $1 \
		 RETURNING id, name, default_webhook_url, webhook_signing_secret",
	)
	.bind(tenant_id)
	.bind(body.default_webhook_url)
	.bind(body.webhook_signing_secret)
	.fetch_optional(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	match row {
		Some(r) => Ok(warp::reply::json(&row_to_webhook(&r))),
		None => Err(ReacherResponseError::new(StatusCode::NOT_FOUND, "Tenant not found").into()),
	}
}

async fn clear_webhook_handler(
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id = ensure_tenant_id(tenant_ctx)?;

	let row = sqlx::query(
		"UPDATE tenants \
		 SET default_webhook_url = NULL, webhook_signing_secret = NULL \
		 WHERE id = $1 \
		 RETURNING id, name, default_webhook_url, webhook_signing_secret",
	)
	.bind(tenant_id)
	.fetch_optional(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let _ = match row {
		Some(_) => (),
		None => {
			return Err(ReacherResponseError::new(StatusCode::NOT_FOUND, "Tenant not found").into())
		}
	};

	Ok(warp::reply::json(&serde_json::json!({
		"webhook_cleared": true,
		"tenant_id": tenant_id,
	})))
}

/// GET /v1/me/settings
///
/// Get tenant runtime settings and default operational behavior.
#[utoipa::path(
	get,
	path = "/v1/me/settings",
	tag = "Tenant",
	responses((status = 200, description = "Tenant settings")),
)]
pub fn v1_get_tenant_settings(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "me" / "settings")
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_pg_pool(config))
		.and_then(settings_handler)
		.with(warp::log("reacher_backend::v1::tenant::settings::get"))
}

/// GET /v1/me/webhook
///
/// Get masked tenant webhook integration state.
#[utoipa::path(
	get,
	path = "/v1/me/webhook",
	tag = "Tenant",
	responses((status = 200, description = "Tenant webhook state")),
)]
pub fn v1_get_tenant_webhook(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "me" / "webhook")
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_pg_pool(config))
		.and_then(webhook_handler)
		.with(warp::log("reacher_backend::v1::tenant::webhook::get"))
}

/// PATCH /v1/me/webhook
///
/// Update tenant webhook URL and signing secret.
#[utoipa::path(
	patch,
	path = "/v1/me/webhook",
	tag = "Tenant",
	responses((status = 200, description = "Tenant webhook updated")),
)]
pub fn v1_update_tenant_webhook(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "me" / "webhook")
		.and(warp::patch())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_pg_pool(config))
		.and(warp::body::json())
		.and_then(update_webhook_handler)
		.with(warp::log("reacher_backend::v1::tenant::webhook::update"))
}

/// DELETE /v1/me/webhook
///
/// Clear tenant webhook URL and signing secret.
#[utoipa::path(
	delete,
	path = "/v1/me/webhook",
	tag = "Tenant",
	responses((status = 200, description = "Tenant webhook cleared")),
)]
pub fn v1_clear_tenant_webhook(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "me" / "webhook")
		.and(warp::delete())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_pg_pool(config))
		.and_then(clear_webhook_handler)
		.with(warp::log("reacher_backend::v1::tenant::webhook::clear"))
}

/// PATCH /v1/me/settings
///
/// Update tenant settings such as retention, default webhook URL, or secret.
#[utoipa::path(
	patch,
	path = "/v1/me/settings",
	tag = "Tenant",
	responses((status = 200, description = "Tenant settings updated")),
)]
pub fn v1_update_tenant_settings(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "me" / "settings")
		.and(warp::patch())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_pg_pool(config))
		.and(warp::body::json())
		.and_then(update_settings_handler)
		.with(warp::log("reacher_backend::v1::tenant::settings::update"))
}

/// GET /v1/me/usage
///
/// Return current tenant usage and quota summary.
#[utoipa::path(
	get,
	path = "/v1/me/usage",
	tag = "Tenant",
	responses((status = 200, description = "Tenant usage summary")),
)]
pub fn v1_get_tenant_usage(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "me" / "usage")
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and_then(usage_handler)
		.with(warp::log("reacher_backend::v1::tenant::usage"))
}
