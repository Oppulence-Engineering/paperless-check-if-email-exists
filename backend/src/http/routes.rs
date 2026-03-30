use crate::config::BackendConfig;
use std::sync::Arc;
use warp::filters::BoxedFilter;
use warp::{Filter, Rejection, Reply};

pub type ResponseFilter = BoxedFilter<(warp::reply::Response,)>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ApiOperationKey {
	pub method: &'static str,
	pub path: &'static str,
}

#[derive(Clone, Copy)]
pub struct ApiRouteSpec {
	pub key: ApiOperationKey,
	pub build: fn(Arc<BackendConfig>) -> ResponseFilter,
}

impl ApiRouteSpec {
	const fn new(
		method: &'static str,
		path: &'static str,
		build: fn(Arc<BackendConfig>) -> ResponseFilter,
	) -> Self {
		Self {
			key: ApiOperationKey { method, path },
			build,
		}
	}
}

fn box_route<F, R>(filter: F) -> ResponseFilter
where
	F: Filter<Extract = (R,), Error = Rejection> + Clone + Send + Sync + 'static,
	R: Reply + 'static,
{
	filter.map(|reply: R| reply.into_response()).boxed()
}

macro_rules! api_routes {
	($(($factory:ident, $method:literal, $path:literal, |$config:ident| $builder:expr)),+ $(,)?) => {
		$(
			fn $factory(config: Arc<BackendConfig>) -> ResponseFilter {
				let $config = config;
				box_route($builder)
			}
		)+

		static ALL_ROUTE_SPECS: &[ApiRouteSpec] = &[
			$(ApiRouteSpec::new($method, $path, $factory)),+
		];
	};
}

api_routes!(
	(route_healthz, "GET", "/healthz", |_config| crate::http::health::liveness::healthz()),
	(route_readyz, "GET", "/readyz", |config| crate::http::health::readiness::readyz(config)),
	(route_version, "GET", "/version", |_config| crate::http::version::get::get_version()),
	(route_openapi, "GET", "/openapi.json", |_config| crate::http::openapi::openapi_spec()),
	(route_v0_check_email, "POST", "/v0/check_email", |config| crate::http::v0::check_email::post::post_check_email(config)),
	(route_v0_bulk_create, "POST", "/v0/bulk", |config| crate::http::v0::bulk::post::create_bulk_job(config.clone(), config.get_pg_pool())),
	(route_v0_bulk_status, "GET", "/v0/bulk/{job_id}", |config| crate::http::v0::bulk::get::get_bulk_job_status(config.clone(), config.get_pg_pool())),
	(route_v0_bulk_results, "GET", "/v0/bulk/{job_id}/results", |config| crate::http::v0::bulk::results::get_bulk_job_result(config.clone(), config.get_pg_pool())),
	(route_v1_onboard, "POST", "/v1/check-email-with-onboard", |config| crate::http::v1::onboard::v1_check_email_with_onboard(config)),
	(route_v1_check_email, "POST", "/v1/check_email", |config| crate::http::v1::check_email::post::v1_check_email(config)),
	(route_v1_find_email_create, "POST", "/v1/find_email", |config| crate::http::v1::find_email::post::v1_find_email(config)),
	(route_v1_find_email_get, "GET", "/v1/find_email/{job_id}", |config| crate::http::v1::find_email::get::v1_get_find_email(config)),
	(route_v1_lists_create, "POST", "/v1/lists", |config| crate::http::v1::lists::post::v1_create_list(config)),
	(route_v1_lists_list, "GET", "/v1/lists", |config| crate::http::v1::lists::get_list::v1_list_lists(config)),
	(route_v1_lists_get, "GET", "/v1/lists/{list_id}", |config| crate::http::v1::lists::get_detail::v1_get_list(config)),
	(route_v1_lists_quality, "GET", "/v1/lists/{list_id}/quality", |config| crate::http::v1::lists::quality::v1_list_quality(config)),
	(route_v1_lists_download, "GET", "/v1/lists/{list_id}/download", |config| crate::http::v1::lists::download::v1_download_list(config)),
	(route_v1_lists_delete, "DELETE", "/v1/lists/{list_id}", |config| crate::http::v1::lists::delete::v1_delete_list(config)),
	(route_v1_pipelines_create, "POST", "/v1/pipelines", |config| crate::http::v1::pipelines::v1_create_pipeline(config)),
	(route_v1_pipelines_list, "GET", "/v1/pipelines", |config| crate::http::v1::pipelines::v1_list_pipelines(config)),
	(route_v1_pipelines_get, "GET", "/v1/pipelines/{pipeline_id}", |config| crate::http::v1::pipelines::v1_get_pipeline(config)),
	(route_v1_pipelines_update, "PATCH", "/v1/pipelines/{pipeline_id}", |config| crate::http::v1::pipelines::v1_update_pipeline(config)),
	(route_v1_pipelines_delete, "DELETE", "/v1/pipelines/{pipeline_id}", |config| crate::http::v1::pipelines::v1_delete_pipeline(config)),
	(route_v1_pipelines_pause, "POST", "/v1/pipelines/{pipeline_id}/pause", |config| crate::http::v1::pipelines::v1_pause_pipeline(config)),
	(route_v1_pipelines_resume, "POST", "/v1/pipelines/{pipeline_id}/resume", |config| crate::http::v1::pipelines::v1_resume_pipeline(config)),
	(route_v1_pipelines_trigger, "POST", "/v1/pipelines/{pipeline_id}/trigger", |config| crate::http::v1::pipelines::v1_trigger_pipeline(config)),
	(route_v1_pipelines_runs, "GET", "/v1/pipelines/{pipeline_id}/runs", |config| crate::http::v1::pipelines::v1_list_pipeline_runs(config)),
	(route_v1_pipelines_run_get, "GET", "/v1/pipelines/{pipeline_id}/runs/{run_id}", |config| crate::http::v1::pipelines::v1_get_pipeline_run(config)),
	(route_v1_reputation_check, "POST", "/v1/reputation/check", |config| crate::http::v1::reputation::check::v1_check_reputation(config)),
	(route_v1_suppressions_add, "POST", "/v1/suppressions", |config| crate::http::v1::suppressions::add::v1_add_suppressions(config)),
	(route_v1_suppressions_check, "GET", "/v1/suppressions/check", |config| crate::http::v1::suppressions::check::v1_check_suppression(config)),
	(route_v1_suppressions_list, "GET", "/v1/suppressions", |config| crate::http::v1::suppressions::list::v1_list_suppressions(config)),
	(route_v1_suppressions_delete, "DELETE", "/v1/suppressions/{id}", |config| crate::http::v1::suppressions::delete::v1_delete_suppression(config)),
	(route_v1_bulk_create, "POST", "/v1/bulk", |config| crate::http::v1::bulk::post::v1_create_bulk_job(config)),
	(route_v1_bulk_progress, "GET", "/v1/bulk/{job_id}", |config| crate::http::v1::bulk::get_progress::v1_get_bulk_job_progress(config)),
	(route_v1_bulk_results, "GET", "/v1/bulk/{job_id}/results", |config| crate::http::v1::bulk::get_results::v1_get_bulk_job_results(config)),
	(route_v1_reverification_status, "GET", "/v1/reverification/status", |config| crate::http::v1::reverification::status::v1_reverification_status(config)),
	(route_v1_events, "GET", "/v1/events", |config| crate::http::v1::events::v1_list_events(config)),
	(route_v1_email_history, "GET", "/v1/emails/{email}/history", |config| crate::http::v1::email_history::v1_email_history(config)),
	(route_v1_query, "GET", "/v1/query", |config| crate::http::v1::query::v1_query_results(config)),
	(route_v1_comments_create, "POST", "/v1/comments", |config| crate::http::v1::comments::v1_create_comment(config)),
	(route_v1_comments_list, "GET", "/v1/comments", |config| crate::http::v1::comments::v1_list_comments(config)),
	(route_v1_comments_delete, "DELETE", "/v1/comments/{comment_id}", |config| crate::http::v1::comments::v1_delete_comment(config)),
	(route_v1_jobs_get, "GET", "/v1/jobs/{job_id}", |config| crate::http::v1::jobs::get_status::v1_get_job_status(config)),
	(route_v1_jobs_cancel, "POST", "/v1/jobs/{job_id}/cancel", |config| crate::http::v1::jobs::cancel::v1_cancel_job(config)),
	(route_v1_jobs_events, "GET", "/v1/jobs/{job_id}/events", |config| crate::http::v1::jobs::get_events::v1_get_job_events(config)),
	(route_v1_jobs_results, "GET", "/v1/jobs/{job_id}/results", |config| crate::http::v1::jobs::get_results::v1_get_job_results(config)),
	(route_v1_jobs_download, "GET", "/v1/jobs/{job_id}/download", |config| crate::http::v1::jobs::download::v1_download_job_results(config)),
	(route_v1_jobs_retry, "POST", "/v1/jobs/{job_id}/retry", |config| crate::http::v1::jobs::retry::v1_retry_job(config)),
	(route_v1_jobs_approval, "GET", "/v1/jobs/{job_id}/approval", |config| crate::http::v1::jobs::approval_checklist::v1_job_approval_checklist(config)),
	(route_v1_jobs_latency, "GET", "/v1/jobs/{job_id}/latency", |config| crate::http::v1::jobs::latency::v1_job_latency(config)),
	(route_v1_me, "GET", "/v1/me", |config| crate::http::v1::me::v1_me(config)),
	(route_v1_me_settings_get, "GET", "/v1/me/settings", |config| crate::http::v1::tenant_settings::v1_get_tenant_settings(config)),
	(route_v1_me_settings_update, "PATCH", "/v1/me/settings", |config| crate::http::v1::tenant_settings::v1_update_tenant_settings(config)),
	(route_v1_me_webhook_get, "GET", "/v1/me/webhook", |config| crate::http::v1::tenant_settings::v1_get_tenant_webhook(config)),
	(route_v1_me_webhook_update, "PATCH", "/v1/me/webhook", |config| crate::http::v1::tenant_settings::v1_update_tenant_webhook(config)),
	(route_v1_me_webhook_delete, "DELETE", "/v1/me/webhook", |config| crate::http::v1::tenant_settings::v1_clear_tenant_webhook(config)),
	(route_v1_me_usage, "GET", "/v1/me/usage", |config| crate::http::v1::tenant_settings::v1_get_tenant_usage(config)),
	(route_v1_me_domains_list, "GET", "/v1/me/domains", |config| crate::http::v1::tenant_domains::v1_list_tenant_domains(config)),
	(route_v1_me_domains_create, "POST", "/v1/me/domains", |config| crate::http::v1::tenant_domains::v1_create_tenant_domain(config)),
	(route_v1_me_domains_get, "GET", "/v1/me/domains/{domain}", |config| crate::http::v1::tenant_domains::v1_get_tenant_domain(config)),
	(route_v1_me_domains_update, "PATCH", "/v1/me/domains/{domain}", |config| crate::http::v1::tenant_domains::v1_update_tenant_domain(config)),
	(route_v1_me_domains_delete, "DELETE", "/v1/me/domains/{domain}", |config| crate::http::v1::tenant_domains::v1_delete_tenant_domain(config)),
	(route_v1_me_api_keys_list, "GET", "/v1/me/api-keys", |config| crate::http::v1::account_api_keys::list_api_keys(config)),
	(route_v1_me_api_keys_get, "GET", "/v1/me/api-keys/{key_id}", |config| crate::http::v1::account_api_keys::get_api_key(config)),
	(route_v1_me_api_keys_create, "POST", "/v1/me/api-keys", |config| crate::http::v1::account_api_keys::create_api_key(config)),
	(route_v1_me_api_keys_update, "PATCH", "/v1/me/api-keys/{key_id}", |config| crate::http::v1::account_api_keys::update_api_key(config)),
	(route_v1_me_api_keys_delete, "DELETE", "/v1/me/api-keys/{key_id}", |config| crate::http::v1::account_api_keys::revoke_api_key(config)),
	(route_v1_admin_tenants_create, "POST", "/v1/admin/tenants", |config| crate::http::v1::admin::tenants::create_tenant(config)),
	(route_v1_admin_tenants_list, "GET", "/v1/admin/tenants", |config| crate::http::v1::admin::tenants::list_tenants(config)),
	(route_v1_admin_tenants_get, "GET", "/v1/admin/tenants/{tenant_id}", |config| crate::http::v1::admin::tenants::get_tenant(config)),
	(route_v1_admin_tenants_update, "PUT", "/v1/admin/tenants/{tenant_id}", |config| crate::http::v1::admin::tenants::update_tenant(config)),
	(route_v1_admin_tenants_delete, "DELETE", "/v1/admin/tenants/{tenant_id}", |config| crate::http::v1::admin::tenants::delete_tenant(config)),
	(route_v1_admin_quota_get, "GET", "/v1/admin/tenants/{tenant_id}/quota", |config| crate::http::v1::admin::quota::get_tenant_quota(config)),
	(route_v1_admin_quota_update, "PATCH", "/v1/admin/tenants/{tenant_id}/quota", |config| crate::http::v1::admin::quota::update_tenant_quota(config)),
	(route_v1_admin_quota_reset, "POST", "/v1/admin/tenants/{tenant_id}/quota/reset", |config| crate::http::v1::admin::quota::reset_tenant_quota(config)),
	(route_v1_admin_jobs_list, "GET", "/v1/admin/jobs", |config| crate::http::v1::admin::jobs::list_jobs(config)),
	(route_v1_admin_jobs_get, "GET", "/v1/admin/jobs/{job_id}", |config| crate::http::v1::admin::jobs::get_job(config)),
	(route_v1_admin_jobs_events, "GET", "/v1/admin/jobs/{job_id}/events", |config| crate::http::v1::admin::jobs::get_job_events(config)),
	(route_v1_admin_jobs_results, "GET", "/v1/admin/jobs/{job_id}/results", |config| crate::http::v1::admin::jobs::get_job_results(config)),
	(route_v1_admin_tenant_jobs, "GET", "/v1/admin/tenants/{tenant_id}/jobs", |config| crate::http::v1::admin::jobs::list_tenant_jobs(config)),
	(route_v1_admin_api_keys_list_all, "GET", "/v1/admin/api-keys", |config| crate::http::v1::admin::api_keys::list_all_api_keys(config)),
	(route_v1_admin_api_keys_create, "POST", "/v1/admin/tenants/{tenant_id}/api-keys", |config| crate::http::v1::admin::api_keys::create_api_key(config)),
	(route_v1_admin_api_keys_list, "GET", "/v1/admin/tenants/{tenant_id}/api-keys", |config| crate::http::v1::admin::api_keys::list_api_keys(config)),
	(route_v1_admin_api_keys_get, "GET", "/v1/admin/tenants/{tenant_id}/api-keys/{key_id}", |config| crate::http::v1::admin::api_keys::get_api_key(config)),
	(route_v1_admin_api_keys_update, "PATCH", "/v1/admin/tenants/{tenant_id}/api-keys/{key_id}", |config| crate::http::v1::admin::api_keys::update_api_key(config)),
	(route_v1_admin_api_keys_delete, "DELETE", "/v1/admin/tenants/{tenant_id}/api-keys/{key_id}", |config| crate::http::v1::admin::api_keys::revoke_api_key(config)),
	(route_v1_admin_api_keys_reactivate, "POST", "/v1/admin/tenants/{tenant_id}/api-keys/{key_id}/reactivate", |config| crate::http::v1::admin::api_keys::reactivate_api_key(config))
);

pub fn all_route_specs() -> &'static [ApiRouteSpec] {
	ALL_ROUTE_SPECS
}

pub fn build_all_routes(config: Arc<BackendConfig>) -> ResponseFilter {
	let mut routes = ALL_ROUTE_SPECS
		.iter()
		.map(|spec| (spec.build)(Arc::clone(&config)));
	let first = routes.next().expect("http route inventory cannot be empty");
	routes.fold(first, |acc, next| acc.or(next).unify().boxed())
}
