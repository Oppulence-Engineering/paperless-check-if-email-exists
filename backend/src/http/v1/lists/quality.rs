use crate::config::BackendConfig;
use crate::finder::require_tenant_id;
use crate::http::v1::bulk::with_worker_db;
use crate::http::{check_scope, resolve_tenant, ReacherResponseError};
use crate::tenant::context::{scope, TenantContext};
use check_if_email_exists::LOG_TARGET;
use serde::Serialize;
use sqlx::{PgPool, Row};
use std::sync::Arc;
use warp::http::StatusCode;
use warp::Filter;

#[derive(Debug, Serialize)]
struct QualityReport {
	list_id: i32,
	total_rows: i32,
	processed: i64,
	avg_score: f64,
	categories: CategoryDistribution,
	safe_to_send_count: i64,
	safe_to_send_pct: f64,
	risk_breakdown: RiskBreakdown,
	quality_grade: String,
}

#[derive(Debug, Serialize)]
struct CategoryDistribution {
	valid: i64,
	valid_pct: f64,
	risky: i64,
	risky_pct: f64,
	unknown: i64,
	unknown_pct: f64,
	invalid: i64,
	invalid_pct: f64,
}

#[derive(Debug, Serialize)]
struct RiskBreakdown {
	disposable: i64,
	catch_all: i64,
	role_account: i64,
	spam_trap: i64,
	full_inbox: i64,
}

async fn http_handler(
	list_id: i32,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::LISTS)?;
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;

	let list_row = sqlx::query("SELECT total_rows FROM v1_lists WHERE id = $1 AND tenant_id = $2")
		.bind(list_id)
		.bind(tenant_id)
		.fetch_optional(&pg_pool)
		.await
		.map_err(ReacherResponseError::from)?;
	let list_row = list_row.ok_or_else(|| {
		warp::reject::custom(ReacherResponseError::new(
			StatusCode::NOT_FOUND,
			"List not found",
		))
	})?;
	let total_rows: i32 = list_row.get("total_rows");

	let row = sqlx::query(
		r#"
		SELECT
			COUNT(*) FILTER (WHERE result IS NOT NULL OR error IS NOT NULL) AS processed,
			COALESCE(AVG(score::FLOAT) FILTER (WHERE score IS NOT NULL), 0)::FLOAT8 AS avg_score,
			COUNT(*) FILTER (WHERE score_category = 'valid') AS valid,
			COUNT(*) FILTER (WHERE score_category = 'risky') AS risky,
			COUNT(*) FILTER (WHERE score_category = 'unknown') AS unknown,
			COUNT(*) FILTER (WHERE score_category = 'invalid') AS invalid,
			COUNT(*) FILTER (WHERE safe_to_send = true) AS safe_to_send,
			COUNT(*) FILTER (WHERE result->'misc'->>'is_disposable' = 'true') AS disposable,
			COUNT(*) FILTER (WHERE result->'smtp'->>'is_catch_all' = 'true') AS catch_all,
			COUNT(*) FILTER (WHERE result->'misc'->>'is_role_account' = 'true') AS role_account,
			COUNT(*) FILTER (WHERE result->'misc'->>'is_spam_trap_domain' = 'true') AS spam_trap,
			COUNT(*) FILTER (WHERE result->'smtp'->>'has_full_inbox' = 'true') AS full_inbox
		FROM v1_task_result
		WHERE (extra->>'list_id')::INTEGER = $1
		"#,
	)
	.bind(list_id)
	.fetch_one(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let processed: i64 = row.get::<Option<i64>, _>("processed").unwrap_or(0);
	let avg_score: f64 = row.get::<Option<f64>, _>("avg_score").unwrap_or(0.0);
	let valid: i64 = row.get::<Option<i64>, _>("valid").unwrap_or(0);
	let risky: i64 = row.get::<Option<i64>, _>("risky").unwrap_or(0);
	let unknown: i64 = row.get::<Option<i64>, _>("unknown").unwrap_or(0);
	let invalid: i64 = row.get::<Option<i64>, _>("invalid").unwrap_or(0);
	let safe_to_send: i64 = row.get::<Option<i64>, _>("safe_to_send").unwrap_or(0);

	let total = processed.max(1) as f64;
	let safe_pct = (safe_to_send as f64 / total * 100.0).round();
	let quality_grade = if safe_pct >= 90.0 {
		"A"
	} else if safe_pct >= 75.0 {
		"B"
	} else if safe_pct >= 50.0 {
		"C"
	} else if safe_pct >= 25.0 {
		"D"
	} else {
		"F"
	}
	.to_string();

	Ok(warp::reply::json(&QualityReport {
		list_id,
		total_rows,
		processed,
		avg_score: (avg_score * 100.0).round() / 100.0,
		categories: CategoryDistribution {
			valid,
			valid_pct: (valid as f64 / total * 100.0).round(),
			risky,
			risky_pct: (risky as f64 / total * 100.0).round(),
			unknown,
			unknown_pct: (unknown as f64 / total * 100.0).round(),
			invalid,
			invalid_pct: (invalid as f64 / total * 100.0).round(),
		},
		safe_to_send_count: safe_to_send,
		safe_to_send_pct: safe_pct,
		risk_breakdown: RiskBreakdown {
			disposable: row.get::<Option<i64>, _>("disposable").unwrap_or(0),
			catch_all: row.get::<Option<i64>, _>("catch_all").unwrap_or(0),
			role_account: row.get::<Option<i64>, _>("role_account").unwrap_or(0),
			spam_trap: row.get::<Option<i64>, _>("spam_trap").unwrap_or(0),
			full_inbox: row.get::<Option<i64>, _>("full_inbox").unwrap_or(0),
		},
		quality_grade,
	}))
}

/// GET /v1/lists/{list_id}/quality
///
/// Returns a quality benchmark report for a list.
#[utoipa::path(
	get,
	path = "/v1/lists/{list_id}/quality",
	tag = "Lists",
	params(("list_id" = i32, Path, description = "List identifier")),
	responses((status = 200, description = "List quality benchmark report"))
)]
pub fn v1_list_quality(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "lists" / i32 / "quality")
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and_then(http_handler)
		.with(warp::log(LOG_TARGET))
}
