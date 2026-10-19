use crate::config::BackendConfig;
use crate::finder::require_tenant_id;
use crate::http::v1::bulk::with_worker_db;
use crate::http::{check_scope, resolve_tenant, ReacherResponseError};
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
pub struct AlertQuery {
	status: Option<String>,
	#[serde(rename = "type")]
	alert_type: Option<String>,
	limit: Option<i64>,
	offset: Option<i64>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct AlertView {
	pub id: i64,
	#[serde(rename = "type")]
	pub alert_type: String,
	pub status: String,
	pub canonical_email: String,
	pub title: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub body: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub change_event_id: Option<i64>,
	pub metadata: Value,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct AlertListResponse {
	pub alerts: Vec<AlertView>,
	pub total: i64,
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct UpdateAlertRequest {
	pub status: String,
}

fn validate_status_filter(status: Option<&str>) -> Result<(), ReacherResponseError> {
	match status {
		None | Some("unread") | Some("read") | Some("dismissed") => Ok(()),
		Some(_) => Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"status must be unread, read, or dismissed",
		)),
	}
}

fn validate_update_status(status: &str) -> Result<(), ReacherResponseError> {
	match status {
		"read" | "dismissed" => Ok(()),
		_ => Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"status must be read or dismissed",
		)),
	}
}

async fn list_handler(
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	query: AlertQuery,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::VERIFY)?;
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	validate_status_filter(query.status.as_deref()).map_err(warp::reject::custom)?;

	let limit = query.limit.unwrap_or(50).clamp(0, 200);
	let offset = query.offset.unwrap_or(0).max(0);
	let total: i64 = sqlx::query_scalar(
		r#"
		SELECT COUNT(*)
		FROM v1_alerts
		WHERE tenant_id = $1
		  AND ($2::TEXT IS NULL OR status = $2)
		  AND ($3::TEXT IS NULL OR type = $3)
		"#,
	)
	.bind(tenant_id)
	.bind(&query.status)
	.bind(&query.alert_type)
	.fetch_one(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let rows = sqlx::query(
		r#"
		SELECT id, type, status, canonical_email, title, body, change_event_id,
		       metadata, created_at, updated_at
		FROM v1_alerts
		WHERE tenant_id = $1
		  AND ($2::TEXT IS NULL OR status = $2)
		  AND ($3::TEXT IS NULL OR type = $3)
		ORDER BY created_at DESC, id DESC
		LIMIT $4 OFFSET $5
		"#,
	)
	.bind(tenant_id)
	.bind(&query.status)
	.bind(&query.alert_type)
	.bind(limit)
	.bind(offset)
	.fetch_all(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	Ok(warp::reply::json(&AlertListResponse {
		alerts: rows.iter().map(row_to_alert).collect(),
		total,
	}))
}

async fn patch_handler(
	alert_id: i64,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	body: UpdateAlertRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::VERIFY)?;
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	validate_update_status(&body.status).map_err(warp::reject::custom)?;

	let row = sqlx::query(
		r#"
		UPDATE v1_alerts
		SET status = $3,
		    updated_at = NOW()
		WHERE id = $1 AND tenant_id = $2
		RETURNING id, type, status, canonical_email, title, body, change_event_id,
		          metadata, created_at, updated_at
		"#,
	)
	.bind(alert_id)
	.bind(tenant_id)
	.bind(&body.status)
	.fetch_optional(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let row = row.ok_or_else(|| {
		warp::reject::custom(ReacherResponseError::new(
			StatusCode::NOT_FOUND,
			"Alert not found",
		))
	})?;

	Ok(warp::reply::json(&row_to_alert(&row)))
}

fn row_to_alert(row: &sqlx::postgres::PgRow) -> AlertView {
	AlertView {
		id: row.get("id"),
		alert_type: row.get("type"),
		status: row.get("status"),
		canonical_email: row.get("canonical_email"),
		title: row.get("title"),
		body: row.get("body"),
		change_event_id: row.get("change_event_id"),
		metadata: row.get("metadata"),
		created_at: row.get("created_at"),
		updated_at: row.get("updated_at"),
	}
}

/// GET /v1/alerts
#[utoipa::path(
	get,
	path = "/v1/alerts",
	tag = "Verification",
	params(AlertQuery),
	responses((status = 200, description = "Tenant alert inbox", body = AlertListResponse))
)]
pub fn v1_list_alerts(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "alerts")
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::query::<AlertQuery>())
		.and_then(list_handler)
		.with(warp::log(LOG_TARGET))
}

/// PATCH /v1/alerts/{alert_id}
#[utoipa::path(
	patch,
	path = "/v1/alerts/{alert_id}",
	tag = "Verification",
	params(("alert_id" = i64, Path, description = "Alert identifier")),
	request_body = UpdateAlertRequest,
	responses((status = 200, description = "Updated alert", body = AlertView))
)]
pub fn v1_update_alert(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "alerts" / i64)
		.and(warp::patch())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::body::json())
		.and_then(patch_handler)
		.with(warp::log(LOG_TARGET))
}
