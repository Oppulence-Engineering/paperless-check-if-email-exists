use crate::config::BackendConfig;
use crate::http::v1::bulk::with_worker_db;
use crate::http::{check_scope, resolve_tenant, ReacherResponseError};
use crate::list_intelligence::{classify_change, VerificationSnapshot};
use crate::tenant::context::{scope, TenantContext};
use check_if_email_exists::LOG_TARGET;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use std::sync::Arc;
use warp::Filter;

#[derive(Debug, Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
struct Query {
	limit: Option<i64>,
}

#[derive(Debug, Serialize)]
struct HistoryEntry {
	task_id: i32,
	job_id: Option<i32>,
	list_id: Option<i32>,
	pipeline_run_id: Option<i64>,
	score: Option<i16>,
	category: Option<String>,
	sub_reason: Option<String>,
	safe_to_send: Option<bool>,
	reason_codes: Option<Vec<String>>,
	is_reachable: Option<String>,
	completed_at: Option<DateTime<Utc>>,
	previous_task_id: Option<i32>,
	previous_list_id: Option<i32>,
	previous_pipeline_run_id: Option<i64>,
	previous_score: Option<i16>,
	previous_category: Option<String>,
	previous_safe_to_send: Option<bool>,
	previous_reason_codes: Option<Vec<String>>,
	change_type: Option<String>,
}

#[derive(Debug, Serialize)]
struct Response {
	email: String,
	history: Vec<HistoryEntry>,
	total: i64,
}

async fn http_handler(
	email: String,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	query: Query,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::VERIFY)?;

	let limit = query.limit.unwrap_or(50).clamp(0, 200);
	let email_lower = email.to_lowercase();
	let canonical_email = crate::http::v1::lists::canonicalize::canonicalize_email(&email)
		.unwrap_or_else(|| email_lower.clone());

	let total: i64 = sqlx::query_scalar(
		r#"
		SELECT COUNT(*) FROM v1_task_result
		WHERE (tenant_id = $1 OR $1 IS NULL)
		  AND (
			canonical_email = $2
			OR LOWER(COALESCE(result->>'input', payload->'input'->>'to_email')) = $3
		  )
		  AND task_state IN ('completed', 'failed', 'dead_lettered')
		"#,
	)
	.bind(tenant_ctx.tenant_id)
	.bind(&canonical_email)
	.bind(&email_lower)
	.fetch_one(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let rows = sqlx::query(
		r#"
		SELECT
			r.id,
			r.job_id,
			(r.extra->>'list_id')::INTEGER AS list_id,
			(r.extra->>'pipeline_run_id')::BIGINT AS pipeline_run_id,
			r.canonical_email,
			r.score,
			r.score_category,
			r.sub_reason,
			r.safe_to_send,
			r.reason_codes,
			r.result->>'is_reachable' AS is_reachable,
			r.completed_at,
			prev.id AS previous_task_id,
			(prev.extra->>'list_id')::INTEGER AS previous_list_id,
			(prev.extra->>'pipeline_run_id')::BIGINT AS previous_pipeline_run_id,
			prev.score AS previous_score,
			prev.score_category AS previous_category,
			prev.safe_to_send AS previous_safe_to_send,
			prev.reason_codes AS previous_reason_codes,
			evt.change_type AS persisted_change_type
		FROM v1_task_result r
		LEFT JOIN LATERAL (
			SELECT p.*
			FROM v1_task_result p
			WHERE p.task_state = 'completed'::task_state
			  AND p.result IS NOT NULL
			  AND p.completed_at IS NOT NULL
			  AND p.id <> r.id
			  AND (
				(p.tenant_id = r.tenant_id)
				OR (p.tenant_id IS NULL AND r.tenant_id IS NULL)
			  )
			  AND p.canonical_email IS NOT NULL
			  AND p.canonical_email = r.canonical_email
			  AND r.completed_at IS NOT NULL
			  AND (p.completed_at, p.id) < (r.completed_at, r.id)
			ORDER BY p.completed_at DESC, p.id DESC
			LIMIT 1
		) prev ON true
		LEFT JOIN verification_change_events evt ON evt.current_task_result_id = r.id
		WHERE (r.tenant_id = $1 OR $1 IS NULL)
		  AND (
			r.canonical_email = $2
			OR LOWER(COALESCE(r.result->>'input', r.payload->'input'->>'to_email')) = $3
		  )
		  AND r.task_state IN ('completed', 'failed', 'dead_lettered')
		ORDER BY r.completed_at DESC NULLS LAST, r.id DESC
		LIMIT $4
		"#,
	)
	.bind(tenant_ctx.tenant_id)
	.bind(&canonical_email)
	.bind(&email_lower)
	.bind(limit)
	.fetch_all(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let history: Vec<HistoryEntry> = rows
		.iter()
		.map(|r| {
			let task_id: i32 = r.get("id");
			let completed_at: Option<DateTime<Utc>> = r.get("completed_at");
			let current = completed_at.map(|completed_at| VerificationSnapshot {
				task_id,
				tenant_id: tenant_ctx.tenant_id.unwrap_or_default(),
				canonical_email: r
					.get::<Option<String>, _>("canonical_email")
					.unwrap_or_else(|| canonical_email.clone()),
				list_id: r.get("list_id"),
				pipeline_run_id: r.get("pipeline_run_id"),
				score: r.get("score"),
				category: r.get("score_category"),
				safe_to_send: r.get("safe_to_send"),
				reason_codes: r.get("reason_codes"),
				completed_at,
			});
			let previous = match (
				r.get::<Option<i32>, _>("previous_task_id"),
				r.get::<Option<DateTime<Utc>>, _>("completed_at"),
			) {
				(Some(previous_task_id), Some(completed_at)) => Some(VerificationSnapshot {
					task_id: previous_task_id,
					tenant_id: tenant_ctx.tenant_id.unwrap_or_default(),
					canonical_email: r
						.get::<Option<String>, _>("canonical_email")
						.unwrap_or_else(|| canonical_email.clone()),
					list_id: r.get("previous_list_id"),
					pipeline_run_id: r.get("previous_pipeline_run_id"),
					score: r.get("previous_score"),
					category: r.get("previous_category"),
					safe_to_send: r.get("previous_safe_to_send"),
					reason_codes: r.get("previous_reason_codes"),
					completed_at,
				}),
				_ => None,
			};
			let computed_change_type = current
				.as_ref()
				.map(|current| classify_change(previous.as_ref(), current).to_string());
			HistoryEntry {
				task_id,
				job_id: r.get("job_id"),
				list_id: r.get("list_id"),
				pipeline_run_id: r.get("pipeline_run_id"),
				score: r.get("score"),
				category: r.get("score_category"),
				sub_reason: r.get("sub_reason"),
				safe_to_send: r.get("safe_to_send"),
				reason_codes: r.get("reason_codes"),
				is_reachable: r.get("is_reachable"),
				completed_at,
				previous_task_id: r.get("previous_task_id"),
				previous_list_id: r.get("previous_list_id"),
				previous_pipeline_run_id: r.get("previous_pipeline_run_id"),
				previous_score: r.get("previous_score"),
				previous_category: r.get("previous_category"),
				previous_safe_to_send: r.get("previous_safe_to_send"),
				previous_reason_codes: r.get("previous_reason_codes"),
				change_type: r
					.get::<Option<String>, _>("persisted_change_type")
					.or(computed_change_type),
			}
		})
		.collect();

	Ok(warp::reply::json(&Response {
		email: email_lower,
		history,
		total,
	}))
}

/// GET /v1/emails/{email}/history
///
/// Returns the verification history timeline for a specific email address.
#[utoipa::path(
	get,
	path = "/v1/emails/{email}/history",
	tag = "Verification",
	params(
		("email" = String, Path, description = "Email address to look up"),
		Query
	),
	responses((status = 200, description = "Verification history for the email"))
)]
pub fn v1_email_history(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "emails" / String / "history")
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::query::<Query>())
		.and_then(http_handler)
		.with(warp::log(LOG_TARGET))
}
