use crate::config::BackendConfig;
use crate::http::ReacherResponseError;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use std::sync::Arc;
use uuid::Uuid;
use warp::http::StatusCode;
use warp::Filter;
// utoipa derive removed — OpenAPI spec served from static JSON

#[derive(Debug, Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
struct ListJobsQuery {
	status: Option<String>,
	tenant_id: Option<String>,
	limit: Option<i64>,
	offset: Option<i64>,
}

#[derive(Debug, Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
struct ListTenantJobsQuery {
	status: Option<String>,
	limit: Option<i64>,
	offset: Option<i64>,
}

#[derive(Debug, Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
struct ListEventsQuery {
	limit: Option<i64>,
	offset: Option<i64>,
}

#[derive(Debug, Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
struct ListTaskResultsQuery {
	limit: Option<i64>,
	offset: Option<i64>,
	state: Option<String>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
struct JobTaskSummary {
	queued: i64,
	running: i64,
	completed: i64,
	retrying: i64,
	failed: i64,
	cancelled: i64,
	dead_lettered: i64,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
struct JobSummary {
	job_id: i32,
	tenant_id: Option<Uuid>,
	tenant_slug: Option<String>,
	tenant_name: Option<String>,
	status: String,
	total_records: i32,
	request_id: Option<Uuid>,
	correlation_id: Option<String>,
	created_by: Option<String>,
	task_summary: JobTaskSummary,
	created_at: String,
	updated_at: String,
	completed_at: Option<String>,
	cancelled_at: Option<String>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
struct JobDetail {
	job_id: i32,
	status: String,
	tenant_id: Option<Uuid>,
	tenant_slug: Option<String>,
	tenant_name: Option<String>,
	total_records: i32,
	request_id: Option<Uuid>,
	correlation_id: Option<String>,
	created_by: Option<String>,
	task_summary: JobTaskSummary,
	created_at: String,
	updated_at: String,
	completed_at: Option<String>,
	cancelled_at: Option<String>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
struct ListJobsResponse {
	jobs: Vec<JobSummary>,
	total: i64,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
struct JobEvent {
	id: i64,
	job_id: i32,
	task_id: Option<i32>,
	event_type: String,
	event_data: Option<serde_json::Value>,
	actor: Option<String>,
	created_at: String,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
struct ListEventsResponse {
	events: Vec<JobEvent>,
	total: i64,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
struct TaskResult {
	id: i64,
	task_state: String,
	result: Option<serde_json::Value>,
	error: Option<String>,
	retry_count: i32,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
struct ListTaskResultsResponse {
	results: Vec<TaskResult>,
	total: i64,
}

#[derive(Debug, FromRow)]
struct ListJobsRow {
	job_id: i32,
	tenant_id: Option<Uuid>,
	tenant_slug: Option<String>,
	tenant_name: Option<String>,
	status: String,
	total_records: i32,
	request_id: Option<Uuid>,
	correlation_id: Option<String>,
	created_by: Option<String>,
	created_at: sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
	updated_at: sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
	completed_at: Option<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>,
	cancelled_at: Option<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>,
}

#[derive(Debug, FromRow)]
struct TaskSummaryRow {
	queued: Option<i64>,
	running: Option<i64>,
	completed: Option<i64>,
	retrying: Option<i64>,
	failed: Option<i64>,
	cancelled: Option<i64>,
	dead_lettered: Option<i64>,
}

#[derive(Debug, FromRow)]
struct EventRow {
	id: i64,
	job_id: i32,
	task_id: Option<i32>,
	event_type: String,
	event_data: Option<serde_json::Value>,
	actor: Option<String>,
	created_at: sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
}

fn with_pg_pool(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (PgPool,), Error = warp::Rejection> + Clone {
	warp::any()
		.and_then(move || {
			let config = Arc::clone(&config);
			async move {
				config.get_pg_pool().ok_or_else(|| {
					warp::reject::custom(ReacherResponseError::new(
						StatusCode::SERVICE_UNAVAILABLE,
						"Postgres database required for admin endpoints",
					))
				})
			}
		})
		.boxed()
}

fn clamp_page(limit: Option<i64>, offset: Option<i64>) -> (i64, i64) {
	let limit = limit.unwrap_or(50).clamp(1, 200);
	let offset = offset.unwrap_or(0).max(0);
	(limit, offset)
}

fn validate_status(raw: Option<String>) -> Result<Option<String>, warp::Rejection> {
	match raw {
		Some(status) => {
			let normalized = status.to_lowercase();
			match normalized.as_str() {
				"pending" | "running" | "completed" | "cancelling" | "cancelled" | "failed" => {
					Ok(Some(normalized))
				}
				_ => Err(warp::reject::custom(ReacherResponseError::new(
					StatusCode::BAD_REQUEST,
					"invalid status. Expected pending|running|completed|cancelling|cancelled|failed",
				)),
				),
		}
		}
		None => Ok(None),
	}
}

fn validate_task_state(raw: Option<String>) -> Result<Option<String>, warp::Rejection> {
	match raw {
		Some(state) => {
			let normalized = state.to_lowercase();
			match normalized.as_str() {
				"queued" | "running" | "completed" | "retrying" | "failed" | "cancelled" | "dead_lettered" => {
					Ok(Some(normalized))
				}
				_ => Err(warp::reject::custom(ReacherResponseError::new(
					StatusCode::BAD_REQUEST,
					"invalid task state. Expected queued|running|completed|retrying|failed|cancelled|dead_lettered",
				))),
			}
		}
		None => Ok(None),
	}
}

fn row_to_task_summary(row: TaskSummaryRow) -> JobTaskSummary {
	JobTaskSummary {
		queued: row.queued.unwrap_or(0),
		running: row.running.unwrap_or(0),
		completed: row.completed.unwrap_or(0),
		retrying: row.retrying.unwrap_or(0),
		failed: row.failed.unwrap_or(0),
		cancelled: row.cancelled.unwrap_or(0),
		dead_lettered: row.dead_lettered.unwrap_or(0),
	}
}

async fn fetch_task_summary(
	pg_pool: &PgPool,
	job_id: i32,
) -> Result<JobTaskSummary, ReacherResponseError> {
	let summary = sqlx::query_as::<_, TaskSummaryRow>(
		r#"
		SELECT
			COUNT(CASE WHEN task_state = 'queued' THEN 1 END) as queued,
			COUNT(CASE WHEN task_state = 'running' THEN 1 END) as running,
			COUNT(CASE WHEN task_state = 'completed' THEN 1 END) as completed,
			COUNT(CASE WHEN task_state = 'retrying' THEN 1 END) as retrying,
			COUNT(CASE WHEN task_state = 'failed' THEN 1 END) as failed,
			COUNT(CASE WHEN task_state = 'cancelled' THEN 1 END) as cancelled,
			COUNT(CASE WHEN task_state = 'dead_lettered' THEN 1 END) as dead_lettered
		FROM v1_task_result
		WHERE job_id = $1
		"#,
	)
	.bind(job_id)
	.fetch_one(pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	Ok(row_to_task_summary(summary))
}

#[utoipa::path(
	get,
	path = "/v1/admin/jobs",
	tag = "Admin Jobs",
	params(ListJobsQuery),
	responses(
		(status = 200, description = "List all jobs across tenants", body = ListJobsResponse)
	),
)]
async fn list_admin_jobs(
	pg_pool: PgPool,
	query: ListJobsQuery,
) -> Result<impl warp::Reply, warp::Rejection> {
	let status = validate_status(query.status)?;
	let tenant_id = query
		.tenant_id
		.map(|id| {
			Uuid::parse_str(&id).map_err(|_| {
				ReacherResponseError::new(StatusCode::BAD_REQUEST, "Invalid tenant ID format")
			})
		})
		.transpose()?;
	let (limit, offset) = clamp_page(query.limit, query.offset);

	let rows = sqlx::query_as::<_, ListJobsRow>(
		r#"
		SELECT
			j.id as job_id,
			j.tenant_id,
			t.slug as tenant_slug,
			t.name as tenant_name,
			j.status::TEXT as status,
			j.total_records,
			j.request_id,
			j.correlation_id,
			j.created_by,
			j.created_at,
			j.updated_at,
			j.completed_at,
			j.cancelled_at
		FROM v1_bulk_job j
		LEFT JOIN tenants t ON t.id = j.tenant_id
		WHERE ($1::TEXT IS NULL OR j.status::TEXT = $1::TEXT)
		  AND ($2::UUID IS NULL OR j.tenant_id = $2)
		ORDER BY j.created_at DESC
		LIMIT $3 OFFSET $4
		"#,
	)
	.bind(status.as_deref())
	.bind(tenant_id)
	.bind(limit)
	.bind(offset)
	.fetch_all(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let total: i64 = sqlx::query_scalar(
		"SELECT COUNT(*) FROM v1_bulk_job WHERE ($1::TEXT IS NULL OR status::TEXT = $1::TEXT) AND ($2::UUID IS NULL OR tenant_id = $2)",
	)
	.bind(status.as_deref())
	.bind(tenant_id)
	.fetch_one(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let mut jobs = Vec::with_capacity(rows.len());
	for row in rows {
		let summary = fetch_task_summary(&pg_pool, row.job_id).await?;
		jobs.push(JobSummary {
			job_id: row.job_id,
			tenant_id: row.tenant_id,
			tenant_slug: row.tenant_slug,
			tenant_name: row.tenant_name,
			status: row.status,
			total_records: row.total_records,
			request_id: row.request_id,
			correlation_id: row.correlation_id,
			created_by: row.created_by,
			task_summary: summary,
			created_at: row.created_at.to_rfc3339(),
			updated_at: row.updated_at.to_rfc3339(),
			completed_at: row.completed_at.map(|v| v.to_rfc3339()),
			cancelled_at: row.cancelled_at.map(|v| v.to_rfc3339()),
		});
	}

	Ok(warp::reply::json(&ListJobsResponse { jobs, total }))
}

#[utoipa::path(
	get,
	path = "/v1/admin/jobs/{job_id}",
	tag = "Admin Jobs",
	params(
		("job_id" = i32, Path, description = "Job ID"),
	),
	responses(
		(status = 200, description = "Get a job by ID for admin users", body = JobDetail)
	),
)]
async fn get_admin_job(job_id: i32, pg_pool: PgPool) -> Result<impl warp::Reply, warp::Rejection> {
	let row = sqlx::query_as::<_, ListJobsRow>(
		r#"
		SELECT
			j.id as job_id,
			j.tenant_id,
			t.slug as tenant_slug,
			t.name as tenant_name,
			j.status::TEXT as status,
			j.total_records,
			j.request_id,
			j.correlation_id,
			j.created_by,
			j.created_at,
			j.updated_at,
			j.completed_at,
			j.cancelled_at
		FROM v1_bulk_job j
		LEFT JOIN tenants t ON t.id = j.tenant_id
		WHERE j.id = $1
		"#,
	)
	.bind(job_id)
	.fetch_optional(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let row =
		row.ok_or_else(|| ReacherResponseError::new(StatusCode::NOT_FOUND, "Job not found"))?;
	let summary = fetch_task_summary(&pg_pool, job_id).await?;

	Ok(warp::reply::json(&JobDetail {
		job_id: row.job_id,
		status: row.status,
		tenant_id: row.tenant_id,
		tenant_slug: row.tenant_slug,
		tenant_name: row.tenant_name,
		total_records: row.total_records,
		request_id: row.request_id,
		correlation_id: row.correlation_id,
		created_by: row.created_by,
		task_summary: summary,
		created_at: row.created_at.to_rfc3339(),
		updated_at: row.updated_at.to_rfc3339(),
		completed_at: row.completed_at.map(|v| v.to_rfc3339()),
		cancelled_at: row.cancelled_at.map(|v| v.to_rfc3339()),
	}))
}

#[utoipa::path(
	get,
	path = "/v1/admin/jobs/{job_id}/events",
	tag = "Admin Jobs",
	params(("job_id" = i32, Path, description = "Job ID"), ListEventsQuery),
	responses(
		(status = 200, description = "Get job events for admin users", body = ListEventsResponse)
	),
)]
async fn get_admin_job_events(
	job_id: i32,
	pg_pool: PgPool,
	query: ListEventsQuery,
) -> Result<impl warp::Reply, warp::Rejection> {
	let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM v1_bulk_job WHERE id = $1)")
		.bind(job_id)
		.fetch_one(&pg_pool)
		.await
		.map_err(ReacherResponseError::from)?;

	if !exists {
		return Err(ReacherResponseError::new(StatusCode::NOT_FOUND, "Job not found").into());
	}

	let (limit, offset) = clamp_page(query.limit, query.offset);

	let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM job_events WHERE job_id = $1")
		.bind(job_id)
		.fetch_one(&pg_pool)
		.await
		.map_err(ReacherResponseError::from)?;

	let rows = sqlx::query_as::<_, EventRow>(
		r#"
		SELECT id, job_id, task_id, event_type, event_data, actor, created_at
		FROM job_events
		WHERE job_id = $1
		ORDER BY created_at ASC
		LIMIT $2 OFFSET $3
		"#,
	)
	.bind(job_id)
	.bind(limit)
	.bind(offset)
	.fetch_all(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let events = rows
		.into_iter()
		.map(|row| JobEvent {
			id: row.id,
			job_id: row.job_id,
			task_id: row.task_id,
			event_type: row.event_type,
			event_data: row.event_data,
			actor: row.actor,
			created_at: row.created_at.to_rfc3339(),
		})
		.collect();

	Ok(warp::reply::json(&ListEventsResponse { events, total }))
}

#[utoipa::path(
	get,
	path = "/v1/admin/jobs/{job_id}/results",
	tag = "Admin Jobs",
	params(("job_id" = i32, Path, description = "Job ID"), ListTaskResultsQuery),
	responses(
		(status = 200, description = "Get task results for a job for admin users", body = ListTaskResultsResponse)
	),
)]
async fn get_admin_job_results(
	job_id: i32,
	pg_pool: PgPool,
	query: ListTaskResultsQuery,
) -> Result<impl warp::Reply, warp::Rejection> {
	let state = validate_task_state(query.state)?;
	let (limit, offset) = clamp_page(query.limit, query.offset);

	let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM v1_bulk_job WHERE id = $1)")
		.bind(job_id)
		.fetch_one(&pg_pool)
		.await
		.map_err(ReacherResponseError::from)?;

	if !exists {
		return Err(ReacherResponseError::new(StatusCode::NOT_FOUND, "Job not found").into());
	}

	let total: i64 = sqlx::query_scalar(
		"SELECT COUNT(*) FROM v1_task_result WHERE job_id = $1 AND ($2::task_state IS NULL OR task_state = $2::task_state)",
	)
	.bind(job_id)
	.bind(state.as_deref())
	.fetch_one(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let rows = sqlx::query_as::<_, (i32, String, Option<serde_json::Value>, Option<String>, i32)>(
		r#"
		SELECT id, task_state::TEXT as task_state, result, error, retry_count
		FROM v1_task_result
		WHERE job_id = $1
		  AND ($2::task_state IS NULL OR task_state = $2::task_state)
		ORDER BY id ASC
		LIMIT $3 OFFSET $4
		"#,
	)
	.bind(job_id)
	.bind(state.as_deref())
	.bind(limit)
	.bind(offset)
	.fetch_all(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let results: Vec<TaskResult> = rows
		.into_iter()
		.map(|r| TaskResult {
			id: r.0 as i64,
			task_state: r.1,
			result: r.2,
			error: r.3,
			retry_count: r.4,
		})
		.collect();

	Ok(warp::reply::json(&ListTaskResultsResponse {
		results,
		total,
	}))
}

#[utoipa::path(
	get,
	path = "/v1/admin/tenants/{tenant_id}/jobs",
	tag = "Admin Jobs",
	params(("tenant_id" = String, Path, description = "Tenant ID"), ListTenantJobsQuery),
	responses(
		(status = 200, description = "List jobs for a tenant", body = ListJobsResponse)
	),
)]
async fn list_tenant_jobs_handler(
	tenant_id_str: String,
	pg_pool: PgPool,
	query: ListTenantJobsQuery,
) -> Result<impl warp::Reply, warp::Rejection> {
	let tenant_id = tenant_id_str.parse::<Uuid>().map_err(|_| {
		ReacherResponseError::new(StatusCode::BAD_REQUEST, "Invalid tenant ID format")
	})?;

	let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM tenants WHERE id = $1)")
		.bind(tenant_id)
		.fetch_one(&pg_pool)
		.await
		.map_err(ReacherResponseError::from)?;
	if !exists {
		return Err(ReacherResponseError::new(StatusCode::NOT_FOUND, "Tenant not found").into());
	}

	let status = validate_status(query.status)?;
	let (limit, offset) = clamp_page(query.limit, query.offset);

	let rows = sqlx::query_as::<_, ListJobsRow>(
		r#"
		SELECT
			j.id as job_id,
			j.tenant_id,
			t.slug as tenant_slug,
			t.name as tenant_name,
			j.status::TEXT as status,
			j.total_records,
			j.request_id,
			j.correlation_id,
			j.created_by,
			j.created_at,
			j.updated_at,
			j.completed_at,
			j.cancelled_at
		FROM v1_bulk_job j
		LEFT JOIN tenants t ON t.id = j.tenant_id
		WHERE j.tenant_id = $1
		  AND ($2::TEXT IS NULL OR j.status::TEXT = $2::TEXT)
		ORDER BY j.created_at DESC
		LIMIT $3 OFFSET $4
		"#,
	)
	.bind(tenant_id)
	.bind(status.as_deref())
	.bind(limit)
	.bind(offset)
	.fetch_all(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let total: i64 = sqlx::query_scalar(
		"SELECT COUNT(*) FROM v1_bulk_job WHERE tenant_id = $1 AND ($2::TEXT IS NULL OR status::TEXT = $2::TEXT)",
	)
	.bind(tenant_id)
	.bind(status.as_deref())
	.fetch_one(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let mut jobs = Vec::with_capacity(rows.len());
	for row in rows {
		let summary = fetch_task_summary(&pg_pool, row.job_id).await?;
		jobs.push(JobSummary {
			job_id: row.job_id,
			tenant_id: row.tenant_id,
			tenant_slug: row.tenant_slug,
			tenant_name: row.tenant_name,
			status: row.status,
			total_records: row.total_records,
			request_id: row.request_id,
			correlation_id: row.correlation_id,
			created_by: row.created_by,
			task_summary: summary,
			created_at: row.created_at.to_rfc3339(),
			updated_at: row.updated_at.to_rfc3339(),
			completed_at: row.completed_at.map(|v| v.to_rfc3339()),
			cancelled_at: row.cancelled_at.map(|v| v.to_rfc3339()),
		});
	}

	Ok(warp::reply::json(&ListJobsResponse { jobs, total }))
}

/// GET /v1/admin/jobs
///
/// List jobs across all tenants with optional filters.
#[utoipa::path(
	get,
	path = "/v1/admin/jobs",
	tag = "Admin Jobs",
	responses((status = 200, description = "Admin job list")),
)]
pub fn list_jobs(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "admin" / "jobs")
		.and(warp::get())
		.and(check_admin_header(config.clone()))
		.and(with_pg_pool(config))
		.and(warp::query::<ListJobsQuery>())
		.and_then(list_admin_jobs)
		.with(warp::log("reacher_backend::v1::admin::jobs::list"))
}

/// GET /v1/admin/jobs/{job_id}
///
/// Fetch a single job.
#[utoipa::path(
	get,
	path = "/v1/admin/jobs/{job_id}",
	tag = "Admin Jobs",
	params(("job_id" = i32, Path, description = "Job identifier")),
	responses((status = 200, description = "Admin job details")),
)]
pub fn get_job(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "admin" / "jobs" / i32)
		.and(warp::get())
		.and(check_admin_header(config.clone()))
		.and(with_pg_pool(config))
		.and_then(get_admin_job)
		.with(warp::log("reacher_backend::v1::admin::jobs::get"))
}

/// GET /v1/admin/jobs/{job_id}/events
///
/// Fetch events for a job.
#[utoipa::path(
	get,
	path = "/v1/admin/jobs/{job_id}/events",
	tag = "Admin Jobs",
	params(("job_id" = i32, Path, description = "Job identifier")),
	responses((status = 200, description = "Job event list")),
)]
pub fn get_job_events(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "admin" / "jobs" / i32 / "events")
		.and(warp::get())
		.and(check_admin_header(config.clone()))
		.and(with_pg_pool(config))
		.and(warp::query::<ListEventsQuery>())
		.and_then(get_admin_job_events)
		.with(warp::log("reacher_backend::v1::admin::jobs::events"))
}

/// GET /v1/admin/jobs/{job_id}/results
///
/// Fetch task results for a job.
#[utoipa::path(
	get,
	path = "/v1/admin/jobs/{job_id}/results",
	tag = "Admin Jobs",
	params(("job_id" = i32, Path, description = "Job identifier")),
	responses((status = 200, description = "Job result list")),
)]
pub fn get_job_results(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "admin" / "jobs" / i32 / "results")
		.and(warp::get())
		.and(check_admin_header(config.clone()))
		.and(with_pg_pool(config))
		.and(warp::query::<ListTaskResultsQuery>())
		.and_then(get_admin_job_results)
		.with(warp::log("reacher_backend::v1::admin::jobs::results"))
}

/// GET /v1/admin/tenants/{tenant_id}/jobs
///
/// List jobs scoped to one tenant.
#[utoipa::path(
	get,
	path = "/v1/admin/tenants/{tenant_id}/jobs",
	tag = "Admin Jobs",
	params(("tenant_id" = String, Path, description = "Tenant identifier")),
	responses((status = 200, description = "Tenant job list")),
)]
pub fn list_tenant_jobs(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "admin" / "tenants" / String / "jobs")
		.and(warp::get())
		.and(check_admin_header(config.clone()))
		.and(with_pg_pool(config))
		.and(warp::query::<ListTenantJobsQuery>())
		.and_then(list_tenant_jobs_handler)
		.with(warp::log("reacher_backend::v1::admin::jobs::tenant"))
}

fn check_admin_header(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (), Error = warp::Rejection> + Clone {
	crate::http::check_header(config)
}
