use crate::{
	config::BackendConfig,
	http::{
		check_scope, resolve_tenant, shared::verification_operation::read_operation,
		v0::check_email::post::with_config, ReacherResponseError,
	},
	tenant::context::{scope, TenantContext},
};
use std::sync::Arc;
use uuid::Uuid;
use warp::{http::StatusCode, Filter, Reply};

async fn http_handler(
	id: Uuid,
	tenant: TenantContext,
	config: Arc<BackendConfig>,
) -> Result<impl Reply, warp::Rejection> {
	check_scope(&tenant, scope::VERIFY)?;
	let pool = config.get_pg_pool().ok_or_else(|| {
		ReacherResponseError::new(
			StatusCode::SERVICE_UNAVAILABLE,
			"Operation storage unavailable",
		)
	})?;
	let response = read_operation(&pool, id, &tenant).await?;
	Ok(warp::reply::with_header(
		warp::reply::with_header(
			warp::reply::with_status(response.body, response.status_code),
			"content-type",
			"application/json",
		),
		"cache-control",
		"no-store",
	))
}

#[utoipa::path(
	get, path = "/v1/operations/{operation_id}", tag = "v1",
	params(("operation_id" = Uuid, Path, description = "Verification operation identifier")),
	responses(
		(status = 200, description = "Stored verification result"),
		(status = 202, description = "Verification is pending", body = crate::http::shared::verification_operation::PendingVerification)
	)
)]
pub fn v1_get_verification_operation(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "operations" / Uuid)
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_config(config))
		.and_then(http_handler)
}
