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
	outcome_count: i64,
	delivered_count: i64,
	opened_count: i64,
	clicked_count: i64,
	bounced_count: i64,
	complained_count: i64,
	unsubscribed_count: i64,
	risky_pct: f64,
	invalid_pct: f64,
	unsafe_recommendation_pct: f64,
	negative_outcome_pct: f64,
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
		WITH task_stats AS (
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
		),
		outcome_stats AS (
			SELECT
				source_key,
				COUNT(*) AS outcome_count,
				COUNT(*) FILTER (WHERE event_type = 'delivered') AS delivered_count,
				COUNT(*) FILTER (WHERE event_type = 'opened') AS opened_count,
				COUNT(*) FILTER (WHERE event_type = 'clicked') AS clicked_count,
				COUNT(*) FILTER (WHERE event_type = 'bounced') AS bounced_count,
				COUNT(*) FILTER (WHERE event_type = 'complained') AS complained_count,
				COUNT(*) FILTER (WHERE event_type = 'unsubscribed') AS unsubscribed_count
			FROM v1_contact_outcomes
			WHERE tenant_id = $1
			  AND source_key IS NOT NULL
			  AND ($2::TEXT IS NULL OR source_key = $2)
			GROUP BY source_key
		)
		SELECT
			task_stats.*,
			COALESCE(outcome_stats.outcome_count, 0) AS outcome_count,
			COALESCE(outcome_stats.delivered_count, 0) AS delivered_count,
			COALESCE(outcome_stats.opened_count, 0) AS opened_count,
			COALESCE(outcome_stats.clicked_count, 0) AS clicked_count,
			COALESCE(outcome_stats.bounced_count, 0) AS bounced_count,
			COALESCE(outcome_stats.complained_count, 0) AS complained_count,
			COALESCE(outcome_stats.unsubscribed_count, 0) AS unsubscribed_count
		FROM task_stats
		LEFT JOIN outcome_stats USING (source_key)
		ORDER BY
			invalid_count DESC,
			risky_count DESC,
			total_records DESC
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
			let outcome_count = row.get::<Option<i64>, _>("outcome_count").unwrap_or(0);
			let bounced_count = row.get::<Option<i64>, _>("bounced_count").unwrap_or(0);
			let complained_count = row.get::<Option<i64>, _>("complained_count").unwrap_or(0);
			let unsubscribed_count = row.get::<Option<i64>, _>("unsubscribed_count").unwrap_or(0);
			let risky_pct = percent(risky_count, total_records);
			let invalid_pct = percent(invalid_count, total_records);
			let unsafe_recommendation_pct =
				percent(review_count + suppress_count + drop_count, total_records);
			let negative_outcome_pct = percent(
				bounced_count + complained_count + unsubscribed_count,
				outcome_count,
			);
			let quality_grade = quality_grade(
				risky_pct,
				invalid_pct,
				unsafe_recommendation_pct,
				negative_outcome_pct,
			);
			let source_key: String = row.get("source_key");
			SourceQualityRow {
				summary: format!(
					"This source produces {}% risky contacts, {}% invalid contacts, and {}% negative outcomes.",
					risky_pct, invalid_pct, negative_outcome_pct
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
				outcome_count,
				delivered_count: row.get::<Option<i64>, _>("delivered_count").unwrap_or(0),
				opened_count: row.get::<Option<i64>, _>("opened_count").unwrap_or(0),
				clicked_count: row.get::<Option<i64>, _>("clicked_count").unwrap_or(0),
				bounced_count,
				complained_count,
				unsubscribed_count,
				risky_pct,
				invalid_pct,
				unsafe_recommendation_pct,
				negative_outcome_pct,
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

fn quality_grade(
	risky_pct: f64,
	invalid_pct: f64,
	unsafe_recommendation_pct: f64,
	negative_outcome_pct: f64,
) -> String {
	if invalid_pct >= 20.0 || unsafe_recommendation_pct >= 45.0 || negative_outcome_pct >= 25.0 {
		"F"
	} else if invalid_pct >= 10.0
		|| risky_pct >= 35.0
		|| unsafe_recommendation_pct >= 30.0
		|| negative_outcome_pct >= 15.0
	{
		"D"
	} else if invalid_pct >= 5.0
		|| risky_pct >= 20.0
		|| unsafe_recommendation_pct >= 20.0
		|| negative_outcome_pct >= 8.0
	{
		"C"
	} else if invalid_pct >= 2.0
		|| risky_pct >= 10.0
		|| unsafe_recommendation_pct >= 10.0
		|| negative_outcome_pct >= 3.0
	{
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
		assert_eq!(quality_grade(4.0, 1.0, 3.0, 1.0), "A");
		assert_eq!(quality_grade(18.0, 3.0, 12.0, 4.0), "B");
		assert_eq!(quality_grade(25.0, 6.0, 21.0, 9.0), "C");
		assert_eq!(quality_grade(40.0, 12.0, 31.0, 16.0), "D");
		assert_eq!(quality_grade(10.0, 22.0, 50.0, 26.0), "F");
	}
}
