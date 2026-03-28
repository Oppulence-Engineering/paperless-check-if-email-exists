use crate::config::BackendConfig;
use crate::finder::require_tenant_id;
use crate::http::v1::bulk::with_worker_db;
use crate::http::{check_scope, resolve_tenant, ReacherResponseError};
use crate::pipelines::{
	create_manual_pipeline_run, create_pipeline, delete_pipeline, get_pipeline, get_pipeline_run,
	list_pipeline_runs, list_pipelines, set_pipeline_status, update_pipeline, CreatePipelineInput,
	PipelineRequestError, PipelineRunView, PipelineStatus, PipelineView, TriggerPipelineInput,
	TriggerPipelineResponse, UpdatePipelineInput,
};
use crate::tenant::context::{scope, TenantContext};
use check_if_email_exists::LOG_TARGET;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use warp::http::StatusCode;
use warp::Filter;

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ListPipelinesResponse {
	pub pipelines: Vec<PipelineView>,
	pub total: i64,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ListPipelineRunsResponse {
	pub runs: Vec<PipelineRunView>,
	pub total: i64,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct DeletePipelineResponse {
	pub deleted: bool,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ErrorResponse {
	pub error: String,
}

#[derive(Debug, Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
struct PipelineListQuery {
	status: Option<String>,
	limit: Option<i64>,
	offset: Option<i64>,
}

#[derive(Debug, Deserialize, utoipa::IntoParams)]
#[into_params(parameter_in = Query)]
struct PipelineRunListQuery {
	limit: Option<i64>,
	offset: Option<i64>,
}

fn parse_status(value: Option<String>) -> Result<Option<PipelineStatus>, warp::Rejection> {
	match value.as_deref() {
		None => Ok(None),
		Some("active") => Ok(Some(PipelineStatus::Active)),
		Some("paused") => Ok(Some(PipelineStatus::Paused)),
		Some("deleted") => Ok(Some(PipelineStatus::Deleted)),
		Some(other) => Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			format!("Invalid pipeline status: {}", other),
		)
		.into()),
	}
}

fn map_pipeline_request_error(err: anyhow::Error) -> ReacherResponseError {
	if let Some(pipeline_err) = err.downcast_ref::<PipelineRequestError>() {
		let status = match pipeline_err {
			PipelineRequestError::Validation(_) => StatusCode::BAD_REQUEST,
			PipelineRequestError::NotFound(_) => StatusCode::NOT_FOUND,
			PipelineRequestError::Conflict(_) => StatusCode::CONFLICT,
			PipelineRequestError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
		};
		return ReacherResponseError::new(status, pipeline_err.to_string());
	}

	ReacherResponseError::from(err)
}

async fn create_handler(
	tenant_ctx: TenantContext,
	config: Arc<BackendConfig>,
	pg_pool: PgPool,
	body: CreatePipelineInput,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::PIPELINES_WRITE)?;
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	let pipeline = create_pipeline(&pg_pool, tenant_id, body, &config.pipelines)
		.await
		.map_err(map_pipeline_request_error)?;
	Ok(warp::reply::with_status(
		warp::reply::json(&pipeline),
		StatusCode::CREATED,
	))
}

async fn list_handler(
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	query: PipelineListQuery,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::PIPELINES_READ)?;
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	let (pipelines, total) = list_pipelines(
		&pg_pool,
		tenant_id,
		crate::pipelines::PipelineListQuery {
			status: parse_status(query.status)?,
			limit: query.limit.unwrap_or(50).clamp(0, 200),
			offset: query.offset.unwrap_or(0).max(0),
		},
	)
	.await
	.map_err(ReacherResponseError::from)?;
	Ok(warp::reply::json(&ListPipelinesResponse {
		pipelines,
		total,
	}))
}

async fn get_handler(
	pipeline_id: i64,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::PIPELINES_READ)?;
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	let Some(pipeline) = get_pipeline(&pg_pool, tenant_id, pipeline_id)
		.await
		.map_err(ReacherResponseError::from)?
	else {
		return Err(ReacherResponseError::new(StatusCode::NOT_FOUND, "Pipeline not found").into());
	};
	Ok(warp::reply::json(&pipeline))
}

async fn patch_handler(
	pipeline_id: i64,
	tenant_ctx: TenantContext,
	config: Arc<BackendConfig>,
	pg_pool: PgPool,
	body: UpdatePipelineInput,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::PIPELINES_WRITE)?;
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	let Some(pipeline) = update_pipeline(&pg_pool, tenant_id, pipeline_id, body, &config.pipelines)
		.await
		.map_err(map_pipeline_request_error)?
	else {
		return Err(ReacherResponseError::new(StatusCode::NOT_FOUND, "Pipeline not found").into());
	};
	Ok(warp::reply::json(&pipeline))
}

async fn delete_handler(
	pipeline_id: i64,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::PIPELINES_WRITE)?;
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	let deleted = delete_pipeline(&pg_pool, tenant_id, pipeline_id)
		.await
		.map_err(ReacherResponseError::from)?;
	if !deleted {
		return Err(ReacherResponseError::new(StatusCode::NOT_FOUND, "Pipeline not found").into());
	}
	Ok(warp::reply::json(&DeletePipelineResponse { deleted }))
}

async fn pause_handler(
	pipeline_id: i64,
	tenant_ctx: TenantContext,
	config: Arc<BackendConfig>,
	pg_pool: PgPool,
	status: PipelineStatus,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::PIPELINES_WRITE)?;
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	let Some(pipeline) = set_pipeline_status(
		&pg_pool,
		tenant_id,
		pipeline_id,
		status,
		config.pipelines.min_interval_seconds,
	)
	.await
	.map_err(map_pipeline_request_error)?
	else {
		return Err(ReacherResponseError::new(StatusCode::NOT_FOUND, "Pipeline not found").into());
	};
	Ok(warp::reply::json(&pipeline))
}

async fn trigger_handler(
	pipeline_id: i64,
	tenant_ctx: TenantContext,
	config: Arc<BackendConfig>,
	pg_pool: PgPool,
	body: TriggerPipelineInput,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::PIPELINES_TRIGGER)?;
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	let response = create_manual_pipeline_run(config, &pg_pool, tenant_id, pipeline_id, body.force)
		.await
		.map_err(map_pipeline_request_error)?;
	Ok(warp::reply::with_status(
		warp::reply::json(&response),
		StatusCode::ACCEPTED,
	))
}

async fn list_runs_handler(
	pipeline_id: i64,
	tenant_ctx: TenantContext,
	config: Arc<BackendConfig>,
	pg_pool: PgPool,
	query: PipelineRunListQuery,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::PIPELINES_READ)?;
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	let Some((runs, total)) = list_pipeline_runs(
		config,
		&pg_pool,
		tenant_id,
		pipeline_id,
		query.limit.unwrap_or(50).clamp(0, 200),
		query.offset.unwrap_or(0).max(0),
	)
	.await
	.map_err(ReacherResponseError::from)?
	else {
		return Err(ReacherResponseError::new(StatusCode::NOT_FOUND, "Pipeline not found").into());
	};
	Ok(warp::reply::json(&ListPipelineRunsResponse { runs, total }))
}

async fn get_run_handler(
	pipeline_id: i64,
	run_id: i64,
	tenant_ctx: TenantContext,
	config: Arc<BackendConfig>,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::PIPELINES_READ)?;
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	let Some(run) = get_pipeline_run(config, &pg_pool, tenant_id, pipeline_id, run_id)
		.await
		.map_err(ReacherResponseError::from)?
	else {
		return Err(
			ReacherResponseError::new(StatusCode::NOT_FOUND, "Pipeline run not found").into(),
		);
	};
	Ok(warp::reply::json(&run))
}

/// POST /v1/pipelines
#[utoipa::path(
	post,
	path = "/v1/pipelines",
	tag = "Pipelines",
	request_body = CreatePipelineInput,
	responses(
		(status = 201, description = "Pipeline created", body = PipelineView),
		(status = 400, description = "Bad request", body = ErrorResponse),
		(status = 500, description = "Internal server error", body = ErrorResponse)
	)
)]
pub fn v1_create_pipeline(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	let config_for_handler = Arc::clone(&config);
	warp::path!("v1" / "pipelines")
		.and(warp::post())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(warp::any().map(move || Arc::clone(&config_for_handler)))
		.and(with_worker_db(Arc::clone(&config)))
		.and(warp::body::json())
		.and_then(create_handler)
		.with(warp::log(LOG_TARGET))
}

/// GET /v1/pipelines
#[utoipa::path(
	get,
	path = "/v1/pipelines",
	tag = "Pipelines",
	params(PipelineListQuery),
	responses(
		(status = 200, description = "Pipeline list", body = ListPipelinesResponse),
		(status = 400, description = "Bad request", body = ErrorResponse),
		(status = 500, description = "Internal server error", body = ErrorResponse)
	)
)]
pub fn v1_list_pipelines(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "pipelines")
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(Arc::clone(&config)))
		.and(warp::query::<PipelineListQuery>())
		.and_then(list_handler)
		.with(warp::log(LOG_TARGET))
}

/// GET /v1/pipelines/{pipeline_id}
#[utoipa::path(
	get,
	path = "/v1/pipelines/{pipeline_id}",
	tag = "Pipelines",
	params(("pipeline_id" = i64, Path, description = "Pipeline identifier")),
	responses(
		(status = 200, description = "Pipeline detail", body = PipelineView),
		(status = 404, description = "Not found", body = ErrorResponse),
		(status = 500, description = "Internal server error", body = ErrorResponse)
	)
)]
pub fn v1_get_pipeline(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "pipelines" / i64)
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(Arc::clone(&config)))
		.and_then(get_handler)
		.with(warp::log(LOG_TARGET))
}

/// PATCH /v1/pipelines/{pipeline_id}
#[utoipa::path(
	patch,
	path = "/v1/pipelines/{pipeline_id}",
	tag = "Pipelines",
	params(("pipeline_id" = i64, Path, description = "Pipeline identifier")),
	request_body = UpdatePipelineInput,
	responses(
		(status = 200, description = "Pipeline updated", body = PipelineView),
		(status = 400, description = "Bad request", body = ErrorResponse),
		(status = 404, description = "Not found", body = ErrorResponse),
		(status = 500, description = "Internal server error", body = ErrorResponse)
	)
)]
pub fn v1_update_pipeline(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	let config_for_handler = Arc::clone(&config);
	warp::path!("v1" / "pipelines" / i64)
		.and(warp::patch())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(warp::any().map(move || Arc::clone(&config_for_handler)))
		.and(with_worker_db(Arc::clone(&config)))
		.and(warp::body::json())
		.and_then(patch_handler)
		.with(warp::log(LOG_TARGET))
}

/// DELETE /v1/pipelines/{pipeline_id}
#[utoipa::path(
	delete,
	path = "/v1/pipelines/{pipeline_id}",
	tag = "Pipelines",
	params(("pipeline_id" = i64, Path, description = "Pipeline identifier")),
	responses(
		(status = 200, description = "Pipeline deleted", body = DeletePipelineResponse),
		(status = 404, description = "Not found", body = ErrorResponse),
		(status = 500, description = "Internal server error", body = ErrorResponse)
	)
)]
pub fn v1_delete_pipeline(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "pipelines" / i64)
		.and(warp::delete())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(Arc::clone(&config)))
		.and_then(delete_handler)
		.with(warp::log(LOG_TARGET))
}

/// POST /v1/pipelines/{pipeline_id}/pause
#[utoipa::path(
	post,
	path = "/v1/pipelines/{pipeline_id}/pause",
	tag = "Pipelines",
	params(("pipeline_id" = i64, Path, description = "Pipeline identifier")),
	responses(
		(status = 200, description = "Pipeline paused", body = PipelineView),
		(status = 404, description = "Not found", body = ErrorResponse),
		(status = 500, description = "Internal server error", body = ErrorResponse)
	)
)]
pub fn v1_pause_pipeline(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	let config_for_handler = Arc::clone(&config);
	warp::path!("v1" / "pipelines" / i64 / "pause")
		.and(warp::post())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(warp::any().map(move || Arc::clone(&config_for_handler)))
		.and(with_worker_db(Arc::clone(&config)))
		.and_then(move |pipeline_id, tenant_ctx, config, pg_pool| {
			pause_handler(
				pipeline_id,
				tenant_ctx,
				config,
				pg_pool,
				PipelineStatus::Paused,
			)
		})
		.with(warp::log(LOG_TARGET))
}

/// POST /v1/pipelines/{pipeline_id}/resume
#[utoipa::path(
	post,
	path = "/v1/pipelines/{pipeline_id}/resume",
	tag = "Pipelines",
	params(("pipeline_id" = i64, Path, description = "Pipeline identifier")),
	responses(
		(status = 200, description = "Pipeline resumed", body = PipelineView),
		(status = 404, description = "Not found", body = ErrorResponse),
		(status = 500, description = "Internal server error", body = ErrorResponse)
	)
)]
pub fn v1_resume_pipeline(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	let config_for_handler = Arc::clone(&config);
	warp::path!("v1" / "pipelines" / i64 / "resume")
		.and(warp::post())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(warp::any().map(move || Arc::clone(&config_for_handler)))
		.and(with_worker_db(Arc::clone(&config)))
		.and_then(move |pipeline_id, tenant_ctx, config, pg_pool| {
			pause_handler(
				pipeline_id,
				tenant_ctx,
				config,
				pg_pool,
				PipelineStatus::Active,
			)
		})
		.with(warp::log(LOG_TARGET))
}

/// POST /v1/pipelines/{pipeline_id}/trigger
#[utoipa::path(
	post,
	path = "/v1/pipelines/{pipeline_id}/trigger",
	tag = "Pipelines",
	params(("pipeline_id" = i64, Path, description = "Pipeline identifier")),
	request_body = TriggerPipelineInput,
	responses(
		(status = 202, description = "Pipeline run triggered", body = TriggerPipelineResponse),
		(status = 400, description = "Bad request", body = ErrorResponse),
		(status = 404, description = "Not found", body = ErrorResponse),
		(status = 409, description = "Conflict", body = ErrorResponse),
		(status = 500, description = "Internal server error", body = ErrorResponse)
	)
)]
pub fn v1_trigger_pipeline(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	let config_for_handler = Arc::clone(&config);
	warp::path!("v1" / "pipelines" / i64 / "trigger")
		.and(warp::post())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(warp::any().map(move || Arc::clone(&config_for_handler)))
		.and(with_worker_db(Arc::clone(&config)))
		.and(warp::body::json())
		.and_then(trigger_handler)
		.with(warp::log(LOG_TARGET))
}

/// GET /v1/pipelines/{pipeline_id}/runs
#[utoipa::path(
	get,
	path = "/v1/pipelines/{pipeline_id}/runs",
	tag = "Pipelines",
	params(("pipeline_id" = i64, Path, description = "Pipeline identifier"), PipelineRunListQuery),
	responses(
		(status = 200, description = "Pipeline run history", body = ListPipelineRunsResponse),
		(status = 404, description = "Not found", body = ErrorResponse),
		(status = 500, description = "Internal server error", body = ErrorResponse)
	)
)]
pub fn v1_list_pipeline_runs(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	let config_for_handler = Arc::clone(&config);
	warp::path!("v1" / "pipelines" / i64 / "runs")
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(warp::any().map(move || Arc::clone(&config_for_handler)))
		.and(with_worker_db(Arc::clone(&config)))
		.and(warp::query::<PipelineRunListQuery>())
		.and_then(list_runs_handler)
		.with(warp::log(LOG_TARGET))
}

/// GET /v1/pipelines/{pipeline_id}/runs/{run_id}
#[utoipa::path(
	get,
	path = "/v1/pipelines/{pipeline_id}/runs/{run_id}",
	tag = "Pipelines",
	params(
		("pipeline_id" = i64, Path, description = "Pipeline identifier"),
		("run_id" = i64, Path, description = "Pipeline run identifier")
	),
	responses(
		(status = 200, description = "Pipeline run detail", body = PipelineRunView),
		(status = 404, description = "Not found", body = ErrorResponse),
		(status = 500, description = "Internal server error", body = ErrorResponse)
	)
)]
pub fn v1_get_pipeline_run(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	let config_for_handler = Arc::clone(&config);
	warp::path!("v1" / "pipelines" / i64 / "runs" / i64)
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(warp::any().map(move || Arc::clone(&config_for_handler)))
		.and(with_worker_db(Arc::clone(&config)))
		.and_then(get_run_handler)
		.with(warp::log(LOG_TARGET))
}
