use crate::http::ReacherResponseError;
use serde_json::Value;
use utoipa::OpenApi;
use warp::Filter;

use crate::http::v1::account_api_keys;
use crate::http::v1::admin::api_keys as admin_api_keys;
use crate::http::v1::admin::jobs as admin_jobs;
use crate::http::v1::admin::quota as admin_quota;
use crate::http::v1::admin::tenants as admin_tenants;
use crate::http::v1::tenant_domains;
use crate::http::v1::tenant_settings;

const BASE_OPENAPI: &str = include_str!("../../openapi.json");

#[derive(OpenApi)]
#[openapi(
	paths(
		crate::http::health::liveness::healthz,
		crate::http::health::readiness::readyz,
		crate::http::version::get::get_version,
		crate::http::openapi::openapi_spec,
		crate::http::v0::check_email::post::post_check_email,
		crate::http::v0::bulk::post::create_bulk_job,
		crate::http::v0::bulk::get::get_bulk_job_status,
		crate::http::v0::bulk::results::get_bulk_job_result,
		crate::http::v1::check_email::post::v1_check_email,
		crate::http::v1::bulk::post::v1_create_bulk_job,
		crate::http::v1::bulk::get_progress::v1_get_bulk_job_progress,
		crate::http::v1::bulk::get_results::v1_get_bulk_job_results,
		crate::http::v1::jobs::get_status::v1_get_job_status,
		crate::http::v1::jobs::cancel::v1_cancel_job,
		crate::http::v1::jobs::get_events::v1_get_job_events,
		crate::http::v1::jobs::get_results::v1_get_job_results,
		crate::http::v1::me::v1_me,
		account_api_keys::get_api_key,
		account_api_keys::list_api_keys,
		account_api_keys::create_api_key,
		account_api_keys::update_api_key,
		account_api_keys::revoke_api_key,
		admin_tenants::create_tenant,
		admin_tenants::list_tenants,
		admin_tenants::get_tenant,
		admin_tenants::update_tenant,
		admin_tenants::delete_tenant,
		admin_quota::get_tenant_quota,
		admin_quota::reset_tenant_quota,
		admin_quota::update_tenant_quota,
		admin_api_keys::list_all_api_keys,
		admin_api_keys::create_api_key,
		admin_api_keys::list_api_keys,
		admin_api_keys::get_api_key,
		admin_api_keys::update_api_key,
		admin_api_keys::revoke_api_key,
		admin_api_keys::reactivate_api_key,
		tenant_settings::v1_get_tenant_settings,
		tenant_settings::v1_update_tenant_settings,
		tenant_settings::v1_get_tenant_webhook,
		tenant_settings::v1_update_tenant_webhook,
		tenant_settings::v1_clear_tenant_webhook,
		tenant_settings::v1_get_tenant_usage,
		tenant_domains::v1_list_tenant_domains,
		tenant_domains::v1_create_tenant_domain,
		tenant_domains::v1_get_tenant_domain,
		tenant_domains::v1_update_tenant_domain,
		tenant_domains::v1_delete_tenant_domain,
		admin_jobs::list_jobs,
		admin_jobs::get_job,
		admin_jobs::get_job_events,
		admin_jobs::get_job_results,
		admin_jobs::list_tenant_jobs,
	),
	tags(
		(name = "System", description = "System and service metadata endpoints"),
		(name = "Health", description = "Service health endpoints"),
		(name = "v0", description = "Legacy v0 API endpoints"),
		(name = "v1", description = "Primary v1 API endpoints"),
		(name = "Jobs", description = "Job lifecycle and results endpoints"),
		(name = "Account", description = "Account-level endpoints"),
		(name = "Admin", description = "Administrative endpoints"),
		(name = "Admin Jobs", description = "Administrative job endpoints"),
		(name = "Tenant", description = "Tenant-scoped account settings and domain endpoints"),
	)
)]
struct BackendApiDoc;

fn merge_openapi(base: &mut Value, generated: Value) {
	if let (Some(base_paths), Some(generated_paths)) = (
		base.get_mut("paths").and_then(Value::as_object_mut),
		generated.get("paths").and_then(Value::as_object),
	) {
		for (path, value) in generated_paths {
			base_paths.insert(path.clone(), value.clone());
		}
	}

	if let (Some(base_schemas), Some(generated_schemas)) = (
		base.get_mut("components")
			.and_then(|v| v.get_mut("schemas"))
			.and_then(Value::as_object_mut),
		generated
			.get("components")
			.and_then(|v| v.get("schemas"))
			.and_then(Value::as_object),
	) {
		for (name, value) in generated_schemas {
			base_schemas.insert(name.clone(), value.clone());
		}
	}
}

fn build_spec() -> Result<Value, ReacherResponseError> {
	let mut spec: Value = serde_json::from_str(BASE_OPENAPI).map_err(ReacherResponseError::from)?;
	let generated_spec =
		serde_json::to_value(BackendApiDoc::openapi()).map_err(ReacherResponseError::from)?;

	merge_openapi(&mut spec, generated_spec);
	Ok(spec)
}

/// Serve the merged OpenAPI document for all documented REST endpoints.
#[utoipa::path(
	get,
	path = "/openapi.json",
	tag = "System",
	responses(
		(status = 200, description = "Merged OpenAPI specification for all documented REST endpoints")
	)
)]
pub fn openapi_spec() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone
{
	warp::path!("openapi.json")
		.and(warp::get())
		.and_then(|| async move {
			build_spec()
				.map(|v| warp::reply::json(&v))
				.map_err(|e| warp::reject::custom(e))
		})
}
