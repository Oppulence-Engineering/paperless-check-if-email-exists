//! Bounded dashboard reads. Authorization remains fresh; only the browser caches this snapshot.
use crate::config::BackendConfig;
use crate::finder::require_tenant_id;
use crate::http::resolve_tenant;
use crate::http::v1::{
	alerts::{row_to_alert, AlertListResponse},
	bulk::with_worker_db,
	jobs::approval_checklist::{readiness, DEFAULT_THRESHOLDS},
	lists::quality::safety_grade,
	tenant_settings::{usage_summary, TenantUsageResponse},
};
use crate::tenant::context::{scope, TenantContext};
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{PgPool, Row};
use std::{future::Future, sync::Arc};
use utoipa::ToSchema;
use uuid::Uuid;
use warp::Filter;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum SectionStatus {
	Ready,
	Forbidden,
	Unavailable,
}

#[derive(Serialize, ToSchema)]
pub struct OverviewSection<T> {
	status: SectionStatus,
	#[serde(skip_serializing_if = "Option::is_none")]
	data: Option<T>,
}

#[derive(Serialize, ToSchema)]
pub struct OverviewResponse {
	usage: TenantUsageResponse,
	alerts: OverviewSection<AlertListResponse>,
	lists: OverviewSection<Vec<OverviewList>>,
	suppressions: OverviewSection<i64>,
	outcomes: OverviewSection<OverviewOutcomes>,
	domains: OverviewSection<OverviewDomains>,
	reverification: OverviewSection<OverviewReverification>,
	pipelines: OverviewSection<Vec<OverviewPipeline>>,
}

#[derive(Serialize, ToSchema)]
pub struct OverviewList {
	id: i32,
	#[serde(skip_serializing_if = "Option::is_none")]
	job_id: Option<i32>,
	name: String,
	status: String,
	total_rows: i32,
	total_processed: i64,
	safe_to_send_count: Option<i64>,
	ready_to_send: Option<bool>,
	quality_grade: Option<String>,
}
#[derive(Serialize, ToSchema)]
pub struct OverviewOutcomes {
	bounced: i64,
	complained: i64,
}
#[derive(Serialize, ToSchema)]
pub struct OverviewDomains {
	total: i64,
	verified: i64,
	active: i64,
}
#[derive(Serialize, ToSchema)]
pub struct OverviewReverification {
	enabled: bool,
	next_run_at: Option<DateTime<Utc>>,
}
#[derive(Serialize, ToSchema)]
pub struct OverviewPipeline {
	id: i64,
	name: String,
	status: String,
	next_run_at: Option<DateTime<Utc>>,
	last_run_id: Option<i64>,
	last_run_status: Option<String>,
}

fn permitted(ctx: &TenantContext, scope: &str, path: &str) -> bool {
	ctx.has_scope(scope)
		&& (ctx.user_id.is_none()
			|| ctx.has_scope("*")
			|| crate::tenant::jwt::route_allowed("member", "GET", path))
}

async fn section<T>(
	allowed: bool,
	name: &'static str,
	read: impl Future<Output = Result<T, sqlx::Error>>,
) -> OverviewSection<T> {
	if !allowed {
		return OverviewSection {
			status: SectionStatus::Forbidden,
			data: None,
		};
	}
	match read.await {
		Ok(data) => OverviewSection {
			status: SectionStatus::Ready,
			data: Some(data),
		},
		Err(_) => {
			tracing::warn!(target: check_if_email_exists::LOG_TARGET, "overview section unavailable: {}", name);
			OverviewSection {
				status: SectionStatus::Unavailable,
				data: None,
			}
		}
	}
}

async fn lists(
	pool: &PgPool,
	tenant: Uuid,
	can_review: bool,
) -> Result<Vec<OverviewList>, sqlx::Error> {
	let rows = sqlx::query(r#"
        SELECT l.id, l.job_id, l.name, l.status::text, l.total_rows, j.status::text AS job_status,
          t.approval_excellent_threshold, t.approval_good_threshold,
          raw.processed,
          q.completed, q.unsuccessful, q.suppressed, q.unprocessed, q.safe
        FROM (SELECT * FROM v1_lists WHERE tenant_id=$1 AND status <> 'deleted' ORDER BY created_at DESC, id DESC LIMIT 5) l
        JOIN tenants t ON t.id=l.tenant_id
        LEFT JOIN v1_bulk_job j ON j.id=l.job_id AND j.tenant_id=l.tenant_id
        CROSS JOIN LATERAL (
          SELECT COUNT(*) FILTER (WHERE task_state IN ('completed','failed','dead_lettered','cancelled')
            AND (dispatch_id IS NULL OR completion_done) AND (result IS NOT NULL OR error IS NOT NULL OR task_state='cancelled')) AS processed
          FROM v1_task_result WHERE job_id=l.job_id AND tenant_id=$1 AND extra->>'list_id'=l.id::text
        ) raw
        CROSS JOIN LATERAL (
          SELECT COUNT(*) FILTER (WHERE task_state='completed') AS completed,
            COUNT(*) FILTER (WHERE task_state IN ('failed','dead_lettered','cancelled')) AS unsuccessful,
            COUNT(*) FILTER (WHERE currently_suppressed) AS suppressed,
            COUNT(*) FILTER (WHERE task_state NOT IN ('completed','failed','dead_lettered')) AS unprocessed,
            COUNT(*) FILTER (WHERE effective_safe_to_send) AS safe
          FROM verification_quality_rows WHERE job_id=l.job_id AND tenant_id=$1 AND $2
        ) q ORDER BY l.created_at DESC, l.id DESC
    "#).bind(tenant).bind(can_review).fetch_all(pool).await?;
	Ok(rows
		.into_iter()
		.map(|r| {
			let job_id: Option<i32> = r.get("job_id");
			let can_review = can_review && job_id.is_some();
			let completed: i64 = r.get("completed");
			let safe: i64 = r.get("safe");
			let thresholds = r
				.get::<Option<f64>, _>("approval_excellent_threshold")
				.zip(r.get::<Option<f64>, _>("approval_good_threshold"))
				.unwrap_or(DEFAULT_THRESHOLDS);
			let (_, ready) = readiness(
				completed,
				r.get("unprocessed"),
				r.get("unsuccessful"),
				r.get("suppressed"),
				safe as f64 * 100.0 / completed.max(1) as f64,
				matches!(
					r.get::<Option<String>, _>("job_status")
						.as_deref()
						.unwrap_or_default(),
					"completed" | "failed" | "cancelled"
				),
				thresholds,
			);
			OverviewList {
				id: r.get("id"),
				job_id,
				name: r.get("name"),
				status: r.get("status"),
				total_rows: r.get("total_rows"),
				total_processed: r.get("processed"),
				safe_to_send_count: can_review.then_some(safe),
				ready_to_send: can_review.then_some(ready),
				quality_grade: can_review.then(|| safety_grade(safe, completed).to_string()),
			}
		})
		.collect())
}

async fn alerts(pool: &PgPool, tenant: Uuid) -> Result<AlertListResponse, sqlx::Error> {
	let rows = sqlx::query("SELECT *, COUNT(*) OVER() AS total FROM v1_alerts WHERE tenant_id=$1 AND status='unread' ORDER BY created_at DESC,id DESC LIMIT 10")
        .bind(tenant).fetch_all(pool).await?;
	Ok(AlertListResponse {
		total: rows.first().map(|r| r.get("total")).unwrap_or(0),
		alerts: rows.iter().map(row_to_alert).collect(),
	})
}

async fn outcomes(pool: &PgPool, tenant: Uuid) -> Result<OverviewOutcomes, sqlx::Error> {
	let r = sqlx::query("SELECT COUNT(*) FILTER (WHERE event_type='bounced') AS bounced, COUNT(*) FILTER (WHERE event_type='complained') AS complained FROM v1_contact_outcomes WHERE tenant_id=$1 AND occurred_at >= date_trunc('minute',NOW()) - interval '7 days'")
        .bind(tenant).fetch_one(pool).await?;
	Ok(OverviewOutcomes {
		bounced: r.get("bounced"),
		complained: r.get("complained"),
	})
}
async fn domains(pool: &PgPool, tenant: Uuid) -> Result<OverviewDomains, sqlx::Error> {
	let r=sqlx::query("SELECT COUNT(*) AS total, COUNT(*) FILTER (WHERE is_verified) AS verified, COUNT(*) FILTER (WHERE is_active) AS active FROM tenant_domains WHERE tenant_id=$1")
        .bind(tenant).fetch_one(pool).await?;
	Ok(OverviewDomains {
		total: r.get("total"),
		verified: r.get("verified"),
		active: r.get("active"),
	})
}
async fn reverification(
	pool: &PgPool,
	tenant: Uuid,
) -> Result<OverviewReverification, sqlx::Error> {
	let r =
		sqlx::query("SELECT enabled,next_run_at FROM reverification_schedules WHERE tenant_id=$1")
			.bind(tenant)
			.fetch_optional(pool)
			.await?;
	Ok(OverviewReverification {
		enabled: r.as_ref().is_some_and(|r| r.get("enabled")),
		next_run_at: r.and_then(|r| r.get("next_run_at")),
	})
}
async fn pipelines(pool: &PgPool, tenant: Uuid) -> Result<Vec<OverviewPipeline>, sqlx::Error> {
	let rows=sqlx::query("SELECT p.id,p.name,p.status::text,p.next_run_at,p.last_run_id,r.status::text AS last_run_status FROM v1_pipelines p LEFT JOIN v1_pipeline_runs r ON r.id=p.last_run_id AND r.tenant_id=p.tenant_id AND r.pipeline_id=p.id WHERE p.tenant_id=$1 AND p.deleted_at IS NULL ORDER BY p.created_at DESC,p.id DESC LIMIT 5")
        .bind(tenant).fetch_all(pool).await?;
	Ok(rows
		.into_iter()
		.map(|r| OverviewPipeline {
			id: r.get("id"),
			name: r.get("name"),
			status: r.get("status"),
			next_run_at: r.get("next_run_at"),
			last_run_id: r.get("last_run_id"),
			last_run_status: r.get("last_run_status"),
		})
		.collect())
}

async fn handler(ctx: TenantContext, pool: PgPool) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant = require_tenant_id(ctx.tenant_id)?;
	// Sequential reads stay within the existing pool budget and isolate section failures.
	let response=OverviewResponse {
        usage:usage_summary(&ctx)?,
        alerts:section(permitted(&ctx,scope::VERIFY,"/v1/alerts"),"alerts",alerts(&pool,tenant)).await,
        lists:section(permitted(&ctx,scope::LISTS,"/v1/lists"),"lists",lists(&pool,tenant,permitted(&ctx,scope::BULK,"/v1/jobs/1/approval"))).await,
        suppressions:section(permitted(&ctx,scope::SUPPRESSIONS,"/v1/suppressions"),"suppressions",sqlx::query_scalar("SELECT COUNT(*) FROM v1_suppression_entries WHERE tenant_id=$1 AND status='active' AND (expires_at IS NULL OR expires_at>NOW())").bind(tenant).fetch_one(&pool)).await,
        outcomes:section(permitted(&ctx,scope::SUPPRESSIONS,"/v1/outcomes"),"outcomes",outcomes(&pool,tenant)).await,
        domains:section(permitted(&ctx,scope::SETTINGS,"/v1/me/domains"),"domains",domains(&pool,tenant)).await,
        reverification:section(permitted(&ctx,scope::SETTINGS,"/v1/reverification/status"),"reverification",reverification(&pool,tenant)).await,
        pipelines:section(permitted(&ctx,scope::PIPELINES_READ,"/v1/pipelines"),"pipelines",pipelines(&pool,tenant)).await,
    };
	Ok(warp::reply::json(&response))
}

#[utoipa::path(get, path="/v1/overview", tag="v1", responses((status=200, description="Tenant dashboard snapshot; unavailable and forbidden sections contain no data", body=OverviewResponse)))]
pub fn v1_overview(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "overview")
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and_then(handler)
}

#[cfg(test)]
mod tests {
	use super::*;
	#[tokio::test]
	async fn section_failures_are_isolated_and_forbidden_reads_never_run() {
		let forbidden =
			section::<i64>(false, "alerts", async { panic!("forbidden query ran") }).await;
		assert_eq!(
			serde_json::to_value(forbidden).unwrap(),
			serde_json::json!({"status":"forbidden"})
		);
		let failed = section::<i64>(true, "alerts", async { Err(sqlx::Error::PoolTimedOut) }).await;
		assert_eq!(
			serde_json::to_value(failed).unwrap(),
			serde_json::json!({"status":"unavailable"})
		);
		let ready = section(true, "lists", async { Ok(5) }).await;
		assert_eq!(
			serde_json::to_value(ready).unwrap(),
			serde_json::json!({"status":"ready","data":5})
		);
	}
	#[test]
	fn member_permissions_match_existing_routes() {
		let mut ctx = TenantContext::legacy(crate::config::ThrottleConfig::new_without_throttle());
		ctx.is_legacy = false;
		ctx.scopes = vec![
			scope::VERIFY.into(),
			scope::LISTS.into(),
			scope::BULK.into(),
		];
		ctx.user_id = Some("member".into());
		assert!(permitted(&ctx, scope::LISTS, "/v1/lists"));
		assert!(!permitted(&ctx, scope::VERIFY, "/v1/alerts"));
		assert!(!permitted(&ctx, scope::SETTINGS, "/v1/me/domains"));
		ctx.user_id = None;
		assert!(permitted(&ctx, scope::VERIFY, "/v1/alerts"));
	}
}
