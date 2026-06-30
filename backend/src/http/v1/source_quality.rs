use crate::config::BackendConfig;
use crate::finder::require_tenant_id;
use crate::http::v1::bulk::with_worker_db;
use crate::http::{resolve_tenant, ReacherResponseError};
use crate::tenant::context::TenantContext;
use check_if_email_exists::LOG_TARGET;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use std::sync::Arc;
use warp::Filter;

#[derive(Debug, Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
struct Query {
	source_key: Option<String>,
	min_records: Option<i64>,
	limit: Option<i64>,
}

#[derive(Debug, Serialize)]
struct SourceQualityRow {
	source_key: String,
	total_records: i64,
	processed_records: i64,
	valid_count: i64,
	risky_count: i64,
	unknown_count: i64,
	invalid_count: i64,
	safe_to_send_count: i64,
	send_count: i64,
	send_with_caution_count: i64,
	review_count: i64,
	suppress_count: i64,
	drop_count: i64,
	risky_pct: f64,
	invalid_pct: f64,
	unsafe_recommendation_pct: f64,
	quality_grade: String,
	summary: String,
}

#[derive(Debug, Serialize)]
struct Response {
	sources: Vec<SourceQualityRow>,
}

async fn http_handler(
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	query: Query,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	let min_records = query.min_records.unwrap_or(1).max(1);
	let limit = query.limit.unwrap_or(50).min(200);
	let source_key = query
		.source_key
		.as_deref()
		.map(str::trim)
		.filter(|value| !value.is_empty())
		.map(str::to_ascii_lowercase);

	let rows = sqlx::query(
		r#"
		SELECT
			source_key,
			COUNT(*) AS total_records,
			COUNT(*) FILTER (
				WHERE result IS NOT NULL
				   OR error IS NOT NULL
				   OR task_state IN ('completed', 'failed', 'dead_lettered', 'cancelled')
			) AS processed_records,
			COUNT(*) FILTER (WHERE score_category = 'valid') AS valid_count,
			COUNT(*) FILTER (WHERE score_category = 'risky') AS risky_count,
			COUNT(*) FILTER (WHERE score_category = 'unknown') AS unknown_count,
			COUNT(*) FILTER (WHERE score_category = 'invalid') AS invalid_count,
			COUNT(*) FILTER (WHERE safe_to_send = true) AS safe_to_send_count,
			COUNT(*) FILTER (WHERE recommendation_action = 'send') AS send_count,
			COUNT(*) FILTER (WHERE recommendation_action = 'send_with_caution') AS send_with_caution_count,
			COUNT(*) FILTER (WHERE recommendation_action = 'review') AS review_count,
			COUNT(*) FILTER (WHERE recommendation_action = 'suppress') AS suppress_count,
			COUNT(*) FILTER (WHERE recommendation_action = 'drop') AS drop_count
		FROM v1_task_result
		WHERE tenant_id = $1
		  AND source_key IS NOT NULL
		  AND ($2::TEXT IS NULL OR source_key = $2)
		GROUP BY source_key
		HAVING COUNT(*) >= $3
		ORDER BY
			COUNT(*) FILTER (WHERE score_category = 'invalid') DESC,
			COUNT(*) FILTER (WHERE score_category = 'risky') DESC,
			COUNT(*) DESC
		LIMIT $4
		"#,
	)
	.bind(tenant_id)
	.bind(&source_key)
	.bind(min_records)
	.bind(limit)
	.fetch_all(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let sources = rows
		.into_iter()
		.map(|row| {
			let total_records = row.get::<Option<i64>, _>("total_records").unwrap_or(0);
			let risky_count = row.get::<Option<i64>, _>("risky_count").unwrap_or(0);
			let invalid_count = row.get::<Option<i64>, _>("invalid_count").unwrap_or(0);
			let review_count = row.get::<Option<i64>, _>("review_count").unwrap_or(0);
			let suppress_count = row.get::<Option<i64>, _>("suppress_count").unwrap_or(0);
			let drop_count = row.get::<Option<i64>, _>("drop_count").unwrap_or(0);
			let risky_pct = percent(risky_count, total_records);
			let invalid_pct = percent(invalid_count, total_records);
			let unsafe_recommendation_pct =
				percent(review_count + suppress_count + drop_count, total_records);
			let quality_grade = quality_grade(risky_pct, invalid_pct, unsafe_recommendation_pct);
			let source_key: String = row.get("source_key");
			SourceQualityRow {
				summary: format!(
					"This source produces {}% risky contacts and {}% invalid contacts.",
					risky_pct, invalid_pct
				),
				source_key,
				total_records,
				processed_records: row.get::<Option<i64>, _>("processed_records").unwrap_or(0),
				valid_count: row.get::<Option<i64>, _>("valid_count").unwrap_or(0),
				risky_count,
				unknown_count: row.get::<Option<i64>, _>("unknown_count").unwrap_or(0),
				invalid_count,
				safe_to_send_count: row.get::<Option<i64>, _>("safe_to_send_count").unwrap_or(0),
				send_count: row.get::<Option<i64>, _>("send_count").unwrap_or(0),
				send_with_caution_count: row
					.get::<Option<i64>, _>("send_with_caution_count")
					.unwrap_or(0),
				review_count,
				suppress_count,
				drop_count,
				risky_pct,
				invalid_pct,
				unsafe_recommendation_pct,
				quality_grade,
			}
		})
		.collect();

	Ok(warp::reply::json(&Response { sources }))
}

fn percent(count: i64, total: i64) -> f64 {
	if total <= 0 {
		return 0.0;
	}
	((count as f64 / total as f64) * 1000.0).round() / 10.0
}

fn quality_grade(risky_pct: f64, invalid_pct: f64, unsafe_recommendation_pct: f64) -> String {
	if invalid_pct >= 20.0 || unsafe_recommendation_pct >= 45.0 {
		"F"
	} else if invalid_pct >= 10.0 || risky_pct >= 35.0 || unsafe_recommendation_pct >= 30.0 {
		"D"
	} else if invalid_pct >= 5.0 || risky_pct >= 20.0 || unsafe_recommendation_pct >= 20.0 {
		"C"
	} else if invalid_pct >= 2.0 || risky_pct >= 10.0 || unsafe_recommendation_pct >= 10.0 {
		"B"
	} else {
		"A"
	}
	.to_string()
}

/// GET /v1/sources/quality
pub fn v1_source_quality(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "sources" / "quality")
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

	#[test]
	fn percent_rounds_to_one_decimal_place() {
		assert_eq!(percent(18, 100), 18.0);
		assert_eq!(percent(1, 6), 16.7);
		assert_eq!(percent(1, 0), 0.0);
	}

	#[test]
	fn quality_grade_penalizes_bad_sources() {
		assert_eq!(quality_grade(4.0, 1.0, 3.0), "A");
		assert_eq!(quality_grade(18.0, 3.0, 12.0), "B");
		assert_eq!(quality_grade(25.0, 6.0, 21.0), "C");
		assert_eq!(quality_grade(40.0, 12.0, 31.0), "D");
		assert_eq!(quality_grade(10.0, 22.0, 50.0), "F");
	}
}
