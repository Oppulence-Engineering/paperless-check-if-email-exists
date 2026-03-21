use crate::config::BackendConfig;
use crate::http::v1::bulk::with_worker_db;
use crate::http::{resolve_tenant, ReacherResponseError};
use crate::tenant::context::TenantContext;
use check_if_email_exists::LOG_TARGET;
use serde::Serialize;
use sqlx::{PgPool, Row};
use std::sync::Arc;
use warp::http::StatusCode;
use warp::Filter;

#[derive(Debug, Serialize)]
struct CategoryBreakdown {
	valid: i64,
	risky: i64,
	unknown: i64,
	invalid: i64,
	unprocessed: i64,
}

#[derive(Debug, Serialize)]
struct RiskFlags {
	disposable_count: i64,
	catch_all_count: i64,
	role_account_count: i64,
	spam_trap_count: i64,
	suppressed_count: i64,
}

#[derive(Debug, Serialize)]
struct Response {
	job_id: i32,
	total_records: i32,
	categories: CategoryBreakdown,
	risk_flags: RiskFlags,
	safe_to_send_count: i64,
	safe_to_send_pct: f64,
	recommendation: String,
	ready_to_send: bool,
}

async fn http_handler(
	job_id: i32,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	let job = sqlx::query(
		"SELECT id, total_records, status::TEXT as status FROM v1_bulk_job WHERE id = $1 AND (tenant_id = $2 OR $2 IS NULL)",
	)
	.bind(job_id)
	.bind(tenant_ctx.tenant_id)
	.fetch_optional(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let job = match job {
		Some(j) => j,
		None => {
			return Err(ReacherResponseError::new(StatusCode::NOT_FOUND, "Job not found").into())
		}
	};

	let total_records: i32 = job.get("total_records");

	let row = sqlx::query(
		r#"
		SELECT
			COUNT(CASE WHEN score_category = 'valid' THEN 1 END) AS valid,
			COUNT(CASE WHEN score_category = 'risky' THEN 1 END) AS risky,
			COUNT(CASE WHEN score_category = 'unknown' THEN 1 END) AS unknown,
			COUNT(CASE WHEN score_category = 'invalid' THEN 1 END) AS invalid,
			COUNT(CASE WHEN task_state NOT IN ('completed', 'failed', 'dead_lettered') THEN 1 END) AS unprocessed,
			COUNT(CASE WHEN safe_to_send = true THEN 1 END) AS safe_to_send_count
		FROM v1_task_result WHERE job_id = $1
		"#,
	)
	.bind(job_id)
	.fetch_one(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let valid: i64 = row.get("valid");
	let risky: i64 = row.get("risky");
	let unknown: i64 = row.get("unknown");
	let invalid: i64 = row.get("invalid");
	let unprocessed: i64 = row.get("unprocessed");
	let safe_to_send_count: i64 = row.get("safe_to_send_count");

	let risk_row = sqlx::query(
		r#"
		SELECT
			COUNT(CASE WHEN result->'misc'->>'is_disposable' = 'true' THEN 1 END) AS disposable,
			COUNT(CASE WHEN result->'smtp'->>'is_catch_all' = 'true' THEN 1 END) AS catch_all,
			COUNT(CASE WHEN result->'misc'->>'is_role_account' = 'true' THEN 1 END) AS role_account,
			COUNT(CASE WHEN result->'misc'->>'is_spam_trap_domain' = 'true' THEN 1 END) AS spam_trap
		FROM v1_task_result WHERE job_id = $1 AND result IS NOT NULL
		"#,
	)
	.bind(job_id)
	.fetch_one(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	// Count suppressed emails in this job (fall back to payload email if result is NULL)
	let suppressed: i64 = sqlx::query_scalar(
		r#"
		SELECT COUNT(*) FROM v1_task_result t
		JOIN v1_suppression_entries s
		  ON s.email = LOWER(COALESCE(t.result->>'input', t.payload->'input'->>'to_email'))
		  AND s.tenant_id = t.tenant_id
		WHERE t.job_id = $1
		"#,
	)
	.bind(job_id)
	.fetch_one(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let total = total_records.max(1) as f64;
	// Subtract suppressed from safe count for accurate readiness assessment
	let effective_safe = (safe_to_send_count - suppressed).max(0);
	let safe_ratio = effective_safe as f64 / total * 100.0;
	let safe_pct = (safe_to_send_count as f64 / total * 100.0).round();

	// Distinguish cancelled (terminal) from actively processing
	let job_status: String = job.get("status");
	let is_terminal = matches!(job_status.as_str(), "completed" | "failed" | "cancelled");

	let (recommendation, ready) = if unprocessed > 0 && !is_terminal {
		(
			"Job is still processing. Wait for completion before sending.".to_string(),
			false,
		)
	} else if unprocessed > 0 && is_terminal {
		(
			format!(
				"Job has {} unprocessed rows (cancelled/pending). Review before sending.",
				unprocessed
			),
			false,
		)
	} else if suppressed > 0 && safe_ratio < 90.0 {
		(
			format!(
				"List has {} suppressed addresses. Remove suppressed recipients before sending.",
				suppressed
			),
			false,
		)
	} else if safe_ratio >= 90.0 {
		("List quality is excellent. Safe to send.".to_string(), true)
	} else if safe_ratio >= 70.0 {
		(
			"List quality is good. Consider removing risky and invalid addresses before sending."
				.to_string(),
			true,
		)
	} else if safe_ratio >= 50.0 {
		(
			"List quality is moderate. Remove invalid and risky addresses before sending."
				.to_string(),
			false,
		)
	} else {
		(
			"List quality is poor. Clean the list thoroughly before sending.".to_string(),
			false,
		)
	};

	Ok(warp::reply::json(&Response {
		job_id,
		total_records,
		categories: CategoryBreakdown {
			valid,
			risky,
			unknown,
			invalid,
			unprocessed,
		},
		risk_flags: RiskFlags {
			disposable_count: risk_row.get("disposable"),
			catch_all_count: risk_row.get("catch_all"),
			role_account_count: risk_row.get("role_account"),
			spam_trap_count: risk_row.get("spam_trap"),
			suppressed_count: suppressed,
		},
		safe_to_send_count,
		safe_to_send_pct: safe_pct,
		recommendation,
		ready_to_send: ready,
	}))
}

/// GET /v1/jobs/{job_id}/approval
///
/// Returns a pre-send approval checklist summarizing list quality.
#[utoipa::path(
	get,
	path = "/v1/jobs/{job_id}/approval",
	tag = "Jobs",
	params(("job_id" = i32, Path, description = "Bulk job identifier")),
	responses((status = 200, description = "Pre-send approval checklist"))
)]
pub fn v1_job_approval_checklist(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "jobs" / i32 / "approval")
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and_then(http_handler)
		.with(warp::log(LOG_TARGET))
}
