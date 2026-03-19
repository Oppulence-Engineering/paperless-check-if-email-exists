use crate::config::BackendConfig;
use crate::http::check_header;
use crate::http::ReacherResponseError;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use std::sync::Arc;
use uuid::Uuid;
use warp::http::StatusCode;
use warp::Filter;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct TenantQuotaResponse {
	pub tenant_id: Uuid,
	pub name: String,
	pub monthly_email_limit: Option<i32>,
	pub used_this_period: i32,
	pub period_reset_at: String,
	pub quota_unlimited: bool,
	pub remaining_quota: Option<i32>,
}

fn deserialize_optional_nullable<'de, D>(deserializer: D) -> Result<Option<Option<i32>>, D::Error>
where
	D: serde::Deserializer<'de>,
{
	// If the field is present, deserialize its value (which may be null → None).
	// Wrap in Some(...) so the outer Option distinguishes present-null from absent.
	Ok(Some(Option::deserialize(deserializer)?))
}

#[derive(Debug, Deserialize)]
struct UpdateTenantQuotaRequest {
	#[serde(default, deserialize_with = "deserialize_optional_nullable")]
	pub monthly_email_limit: Option<Option<i32>>,
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
					"Postgres database required for admin quota endpoint",
				))
			})
		}
	})
}

fn row_to_response(row: &sqlx::postgres::PgRow) -> TenantQuotaResponse {
	let tenant_id: Uuid = row.get("id");
	let name: String = row.get("name");
	let monthly_email_limit: Option<i32> = row.get("monthly_email_limit");
	let used_this_period: i32 = row.get("used_this_period");
	let period_reset_at: DateTime<Utc> = row.get("period_reset_at");
	let quota_unlimited = monthly_email_limit.map_or(true, |limit| limit <= 0);
	let remaining_quota = match monthly_email_limit {
		Some(limit) if limit > 0 => Some((limit - used_this_period).max(0)),
		_ => None,
	};

	TenantQuotaResponse {
		tenant_id,
		name,
		monthly_email_limit,
		used_this_period,
		period_reset_at: period_reset_at.to_rfc3339(),
		quota_unlimited,
		remaining_quota,
	}
}

async fn get_handler(
	tenant_id: String,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_uuid = tenant_id.parse::<Uuid>().map_err(|_| {
		ReacherResponseError::new(StatusCode::BAD_REQUEST, "Invalid tenant ID format")
	})?;

	let row = sqlx::query(
		r#"
		SELECT id, name, monthly_email_limit, used_this_period, period_reset_at
		FROM tenants
		WHERE id = $1
		"#,
	)
	.bind(tenant_uuid)
	.fetch_optional(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let row = match row {
		Some(r) => r,
		None => {
			return Err(ReacherResponseError::new(StatusCode::NOT_FOUND, "Tenant not found").into())
		}
	};

	Ok(warp::reply::json(&row_to_response(&row)))
}

async fn reset_handler(
	tenant_id: String,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_uuid = tenant_id.parse::<Uuid>().map_err(|_| {
		ReacherResponseError::new(StatusCode::BAD_REQUEST, "Invalid tenant ID format")
	})?;

	let row = sqlx::query(
		r#"
		UPDATE tenants
		SET used_this_period = 0,
		    period_reset_at = date_trunc('month', NOW()) + INTERVAL '1 month'
		WHERE id = $1
		RETURNING id, name, monthly_email_limit, used_this_period, period_reset_at
		"#,
	)
	.bind(tenant_uuid)
	.fetch_optional(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let row = match row {
		Some(r) => r,
		None => {
			return Err(ReacherResponseError::new(StatusCode::NOT_FOUND, "Tenant not found").into())
		}
	};

	Ok(warp::reply::json(&row_to_response(&row)))
}

async fn update_handler(
	tenant_id: String,
	pg_pool: PgPool,
	body: UpdateTenantQuotaRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_uuid = tenant_id.parse::<Uuid>().map_err(|_| {
		ReacherResponseError::new(StatusCode::BAD_REQUEST, "Invalid tenant ID format")
	})?;

	let limit = match body.monthly_email_limit {
		Some(limit) => limit,
		None => {
			return Err(ReacherResponseError::new(
				StatusCode::BAD_REQUEST,
				"monthly_email_limit is required",
			)
			.into())
		}
	};

	let row = sqlx::query(
		r#"
		UPDATE tenants
		SET monthly_email_limit = $2
		WHERE id = $1
		RETURNING id, name, monthly_email_limit, used_this_period, period_reset_at
		"#,
	)
	.bind(tenant_uuid)
	.bind(limit)
	.fetch_optional(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let row = match row {
		Some(r) => r,
		None => {
			return Err(ReacherResponseError::new(StatusCode::NOT_FOUND, "Tenant not found").into())
		}
	};

	Ok(warp::reply::json(&row_to_response(&row)))
}

/// GET /v1/admin/tenants/{tenant_id}/quota
///
/// Fetch current tenant quota usage and remaining allowance.
#[utoipa::path(
	get,
	path = "/v1/admin/tenants/{tenant_id}/quota",
	tag = "Admin",
	params(("tenant_id" = String, Path, description = "Tenant identifier")),
	responses((status = 200, description = "Tenant quota details")),
)]
pub fn get_tenant_quota(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "admin" / "tenants" / String / "quota")
		.and(warp::get())
		.and(check_header(Arc::clone(&config)))
		.and(with_pg_pool(config))
		.and_then(get_handler)
		.with(warp::log("reacher_backend::v1::admin::tenant_quota"))
}

/// POST /v1/admin/tenants/{tenant_id}/quota/reset
///
/// Reset tenant quota usage counters.
#[utoipa::path(
	post,
	path = "/v1/admin/tenants/{tenant_id}/quota/reset",
	tag = "Admin",
	params(("tenant_id" = String, Path, description = "Tenant identifier")),
	responses((status = 200, description = "Quota reset")),
)]
pub fn reset_tenant_quota(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "admin" / "tenants" / String / "quota" / "reset")
		.and(warp::post())
		.and(check_header(Arc::clone(&config)))
		.and(with_pg_pool(config))
		.and_then(reset_handler)
		.with(warp::log("reacher_backend::v1::admin::tenant_quota::reset"))
}

/// PATCH /v1/admin/tenants/{tenant_id}/quota
///
/// Update tenant quota limit.
#[utoipa::path(
	patch,
	path = "/v1/admin/tenants/{tenant_id}/quota",
	tag = "Admin",
	params(("tenant_id" = String, Path, description = "Tenant identifier")),
	responses((status = 200, description = "Quota updated")),
)]
pub fn update_tenant_quota(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "admin" / "tenants" / String / "quota")
		.and(warp::patch())
		.and(check_header(Arc::clone(&config)))
		.and(with_pg_pool(config))
		.and(warp::body::json())
		.and_then(update_handler)
		.with(warp::log(
			"reacher_backend::v1::admin::tenant_quota::update",
		))
}
