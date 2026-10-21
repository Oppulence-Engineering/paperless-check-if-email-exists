use crate::config::BackendConfig;
use crate::finder::require_tenant_id;
use crate::http::shared::paging::clamp_limit;
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
	let limit = clamp_limit(query.limit);
	let source_key = query
		.source_key
		.as_deref()
		.map(str::trim)
		.filter(|value| !value.is_empty())
		.map(str::to_ascii_lowercase);

	let rows = sqlx::query(
		r#"
		WITH source_rows AS (
            SELECT DISTINCT ON (source_key, identity_key) *
            FROM verification_quality_rows
            WHERE tenant_id=$1 AND source_key IS NOT NULL AND ($2::TEXT IS NULL OR source_key=$2)
            ORDER BY source_key, identity_key, (task_state='completed') DESC, completed_at DESC NULLS LAST, id DESC
        ), task_stats AS (
		SELECT
			source_key,
			COUNT(*) AS total_records,
			COUNT(*) FILTER (
				WHERE task_state = 'completed'
			) AS processed_records,
			COUNT(*) FILTER (WHERE task_state = 'completed' AND score_category = 'valid') AS valid_count,
			COUNT(*) FILTER (WHERE task_state = 'completed' AND score_category = 'risky') AS risky_count,
			COUNT(*) FILTER (WHERE task_state = 'completed' AND score_category = 'unknown') AS unknown_count,
			COUNT(*) FILTER (WHERE task_state = 'completed' AND score_category = 'invalid') AS invalid_count,
			COUNT(*) FILTER (WHERE task_state = 'completed' AND effective_safe_to_send = true) AS safe_to_send_count,
			COUNT(*) FILTER (WHERE task_state = 'completed' AND NOT currently_suppressed AND recommendation_action = 'send') AS send_count,
			COUNT(*) FILTER (WHERE task_state = 'completed' AND NOT currently_suppressed AND recommendation_action = 'send_with_caution') AS send_with_caution_count,
			COUNT(*) FILTER (WHERE task_state = 'completed' AND recommendation_action = 'review') AS review_count,
			COUNT(*) FILTER (WHERE task_state = 'completed' AND (currently_suppressed OR recommendation_action = 'suppress')) AS suppress_count,
			COUNT(*) FILTER (WHERE task_state = 'completed' AND recommendation_action = 'drop') AS drop_count
		FROM source_rows
		WHERE tenant_id = $1
		  AND source_key IS NOT NULL
		  AND is_duplicate IS NOT TRUE
		  AND identity_key IS NOT NULL
		  AND ($2::TEXT IS NULL OR source_key = $2)
		GROUP BY source_key
		HAVING COUNT(*) >= $3
		),
		attributed_outcomes AS (
			SELECT source_key, canonical_email, event_type, occurred_at
			FROM v1_contact_outcomes WHERE tenant_id = $1 AND source_key IS NOT NULL
			UNION
			SELECT attribution.source_key, o.canonical_email,
				CASE o.outcome_type::TEXT
					WHEN 'hard_bounce' THEN 'bounced' WHEN 'soft_bounce' THEN 'bounced'
					WHEN 'complaint' THEN 'complained' WHEN 'unsubscribe' THEN 'unsubscribed'
					WHEN 'open' THEN 'opened' WHEN 'click' THEN 'clicked' ELSE 'delivered' END,
				o.occurred_at
			FROM verification_outcome_evidence o
			JOIN LATERAL (
				SELECT t.source_key FROM verification_quality_rows t
				WHERE t.tenant_id = o.tenant_id AND t.source_key IS NOT NULL
				  AND t.is_duplicate IS NOT TRUE
				  AND t.canonical_email = o.canonical_email
				ORDER BY (t.id = o.task_result_id) DESC NULLS LAST, t.created_at DESC, t.id DESC
				LIMIT 1
			) attribution ON true
			WHERE o.tenant_id = $1
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
			FROM attributed_outcomes
			WHERE source_key IS NOT NULL
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
			let processed_records = row.get::<Option<i64>, _>("processed_records").unwrap_or(0);
			let risky_pct = percent(risky_count, processed_records);
			let invalid_pct = percent(invalid_count, processed_records);
			let unsafe_recommendation_pct =
				percent(review_count + suppress_count + drop_count, processed_records);
			let negative_outcome_pct = percent(
				bounced_count + complained_count + unsubscribed_count,
				outcome_count,
			);
			let quality_grade = super::lists::quality::safety_grade(
                row.get::<Option<i64>, _>("safe_to_send_count").unwrap_or(0), processed_records).to_string();
			let source_key: String = row.get("source_key");
			SourceQualityRow {
				summary: format!(
					"Across lists for this source, of {processed_records} unique checked addresses (latest completed check per address): {risky_pct}% risky and {invalid_pct}% invalid. Negative feedback is {negative_outcome_pct}% of recorded events, not a bounce rate per email sent."
				),
				source_key,
				total_records,
				processed_records,
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
}
