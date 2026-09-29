use crate::config::BackendConfig;
use crate::finder::require_tenant_id;
use crate::http::v1::bulk::with_worker_db;
use crate::http::{resolve_tenant, ReacherResponseError};
use crate::tenant::context::TenantContext;
use bytes::Bytes;
use check_if_email_exists::LOG_TARGET;
use futures::stream;
use serde::Deserialize;
use serde_json::Value;
use sqlx::{PgPool, Row};
use std::io;
use std::sync::Arc;
use warp::http::StatusCode;
use warp::hyper::Body;
use warp::Filter;

const EXPORT_BATCH_SIZE: i64 = 500;

#[derive(Debug, Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
struct Query {
	format: Option<String>,
	reason: Option<String>,
	status: Option<String>,
	source_type: Option<String>,
	source_ref: Option<String>,
	include_expired: Option<bool>,
}

struct ExportState {
	pg_pool: PgPool,
	tenant_id: uuid::Uuid,
	status_filter: Option<String>,
	reason: Option<String>,
	source_type: Option<String>,
	source_ref: Option<String>,
	include_expired: bool,
	last_id: i32,
	header_sent: bool,
}

struct SuppressionExportRow {
	id: i32,
	email: String,
	canonical_email: String,
	status: String,
	reason: String,
	reason_code: Option<String>,
	reason_detail: Option<String>,
	source: Option<String>,
	source_type: Option<String>,
	source_ref: Option<String>,
	notes: Option<String>,
	created_by: Option<String>,
	expires_at: Option<chrono::DateTime<chrono::Utc>>,
	last_seen_at: Option<chrono::DateTime<chrono::Utc>>,
	metadata: Value,
	created_at: chrono::DateTime<chrono::Utc>,
	updated_at: chrono::DateTime<chrono::Utc>,
}

async fn http_handler(
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	query: Query,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	let format = query.format.as_deref().unwrap_or("csv");
	if format != "csv" {
		return Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"Only format=csv is supported",
		)
		.into());
	}
	let status_filter = normalize_status_filter(query.status.as_deref())?;
	let include_expired = query.include_expired.unwrap_or(false);
	let body = Body::wrap_stream(stream::unfold(
		ExportState {
			pg_pool,
			tenant_id,
			status_filter,
			reason: query.reason,
			source_type: query.source_type,
			source_ref: query.source_ref,
			include_expired,
			last_id: 0,
			header_sent: false,
		},
		|mut state| async move {
			if !state.header_sent {
				state.header_sent = true;
				return Some((Ok::<Bytes, io::Error>(Bytes::from(render_header())), state));
			}

			match fetch_batch(&state).await {
				Ok(rows) if rows.is_empty() => None,
				Ok(rows) => {
					state.last_id = rows.last().map(|row| row.id).unwrap_or(state.last_id);
					let mut chunk = Vec::new();
					for row in rows {
						chunk.extend_from_slice(&render_row(&row));
					}
					Some((Ok(Bytes::from(chunk)), state))
				}
				Err(err) => Some((Err(io::Error::other(err.to_string())), state)),
			}
		},
	));

	let response = warp::http::Response::builder()
		.header("Content-Type", "text/csv")
		.header(
			"Content-Disposition",
			"attachment; filename=\"suppression_export.csv\"",
		)
		.body(body)
		.map_err(|err| ReacherResponseError::new(StatusCode::INTERNAL_SERVER_ERROR, err))
		.map_err(warp::reject::custom)?;

	Ok(response)
}

async fn fetch_batch(
	state: &ExportState,
) -> Result<Vec<SuppressionExportRow>, ReacherResponseError> {
	let rows = sqlx::query(
		r#"
		SELECT
			id, email, canonical_email, status, reason::TEXT AS reason,
			reason_code, reason_detail, source, source_type, source_ref, notes,
			created_by, expires_at, last_seen_at, metadata, created_at, updated_at
		FROM v1_suppression_entries
		WHERE tenant_id = $1
		  AND id > $2
		  AND ($3::TEXT IS NULL OR status = $3)
		  AND ($4::TEXT IS NULL OR reason::TEXT = $4)
		  AND ($5::TEXT IS NULL OR source_type = $5)
		  AND ($6::TEXT IS NULL OR source_ref = $6)
		  AND ($7::BOOLEAN = true OR expires_at IS NULL OR expires_at > NOW())
		ORDER BY id ASC
		LIMIT $8
		"#,
	)
	.bind(state.tenant_id)
	.bind(state.last_id)
	.bind(&state.status_filter)
	.bind(&state.reason)
	.bind(&state.source_type)
	.bind(&state.source_ref)
	.bind(state.include_expired)
	.bind(EXPORT_BATCH_SIZE)
	.fetch_all(&state.pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	Ok(rows
		.into_iter()
		.map(|row| SuppressionExportRow {
			id: row.get("id"),
			email: row.get("email"),
			canonical_email: row.get("canonical_email"),
			status: row.get("status"),
			reason: row.get("reason"),
			reason_code: row.get("reason_code"),
			reason_detail: row.get("reason_detail"),
			source: row.get("source"),
			source_type: row.get("source_type"),
			source_ref: row.get("source_ref"),
			notes: row.get("notes"),
			created_by: row.get("created_by"),
			expires_at: row.get("expires_at"),
			last_seen_at: row.get("last_seen_at"),
			metadata: row.get("metadata"),
			created_at: row.get("created_at"),
			updated_at: row.get("updated_at"),
		})
		.collect())
}

fn normalize_status_filter(status: Option<&str>) -> Result<Option<String>, warp::Rejection> {
	let Some(status) = status else {
		return Ok(Some("active".to_string()));
	};
	let status = status.trim().to_lowercase();
	if status == "all" {
		return Ok(None);
	}
	if matches!(status.as_str(), "active" | "inactive" | "merged") {
		return Ok(Some(status));
	}
	Err(ReacherResponseError::new(
		StatusCode::BAD_REQUEST,
		"Invalid status. Must be one of: active, inactive, merged, all",
	)
	.into())
}

fn render_header() -> Vec<u8> {
	let mut writer = csv::WriterBuilder::new()
		.has_headers(false)
		.from_writer(Vec::new());
	writer
		.write_record([
			"id",
			"email",
			"canonical_email",
			"status",
			"reason",
			"reason_code",
			"reason_detail",
			"source",
			"source_type",
			"source_ref",
			"notes",
			"created_by",
			"expires_at",
			"last_seen_at",
			"metadata",
			"created_at",
			"updated_at",
		])
		.expect("csv header write");
	writer.into_inner().expect("csv header bytes")
}

fn render_row(row: &SuppressionExportRow) -> Vec<u8> {
	let mut writer = csv::WriterBuilder::new()
		.has_headers(false)
		.from_writer(Vec::new());
	writer
		.write_record([
			row.id.to_string(),
			row.email.clone(),
			row.canonical_email.clone(),
			row.status.clone(),
			row.reason.clone(),
			row.reason_code.clone().unwrap_or_default(),
			row.reason_detail.clone().unwrap_or_default(),
			row.source.clone().unwrap_or_default(),
			row.source_type.clone().unwrap_or_default(),
			row.source_ref.clone().unwrap_or_default(),
			row.notes.clone().unwrap_or_default(),
			row.created_by.clone().unwrap_or_default(),
			row.expires_at.map(|ts| ts.to_rfc3339()).unwrap_or_default(),
			row.last_seen_at
				.map(|ts| ts.to_rfc3339())
				.unwrap_or_default(),
			row.metadata.to_string(),
			row.created_at.to_rfc3339(),
			row.updated_at.to_rfc3339(),
		])
		.expect("csv row write");
	writer.into_inner().expect("csv row bytes")
}

/// GET /v1/suppressions/export
pub fn v1_export_suppressions(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "suppressions" / "export")
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::query::<Query>())
		.and_then(http_handler)
		.with(warp::log(LOG_TARGET))
}

#[cfg(test)]
mod tests {
	use super::*;
	use chrono::TimeZone;
	use serde_json::json;

	#[test]
	fn export_status_filter_defaults_to_active() {
		assert_eq!(
			normalize_status_filter(None).expect("status should normalize"),
			Some("active".to_string())
		);
	}

	#[test]
	fn export_status_all_disables_filter() {
		assert_eq!(
			normalize_status_filter(Some("all")).expect("status should normalize"),
			None
		);
	}

	#[test]
	fn export_row_includes_metadata_json() {
		let now = chrono::Utc.with_ymd_and_hms(2026, 6, 30, 12, 0, 0).unwrap();
		let row = SuppressionExportRow {
			id: 7,
			email: "user@example.com".to_string(),
			canonical_email: "user@example.com".to_string(),
			status: "active".to_string(),
			reason: "complaint".to_string(),
			reason_code: Some("complaint".to_string()),
			reason_detail: Some("ESP complaint".to_string()),
			source: Some("hubspot".to_string()),
			source_type: Some("crm".to_string()),
			source_ref: Some("list-1".to_string()),
			notes: None,
			created_by: Some("revops".to_string()),
			expires_at: None,
			last_seen_at: Some(now),
			metadata: json!({"confidence": "high"}),
			created_at: now,
			updated_at: now,
		};

		let rendered = render_row(&row);
		let mut reader = csv::ReaderBuilder::new()
			.has_headers(false)
			.from_reader(rendered.as_slice());
		let record = reader
			.records()
			.next()
			.expect("record should exist")
			.expect("record should parse");

		assert_eq!(record.get(1), Some("user@example.com"));
		assert_eq!(record.get(3), Some("active"));
		assert_eq!(record.get(4), Some("complaint"));
		assert_eq!(record.get(14), Some("{\"confidence\":\"high\"}"));
	}
}
