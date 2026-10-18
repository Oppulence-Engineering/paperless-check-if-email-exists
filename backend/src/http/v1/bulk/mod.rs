use crate::config::BackendConfig;
use crate::http::ReacherResponseError;
use sqlx::PgPool;
use std::sync::Arc;
use warp::http::StatusCode;
use warp::Filter;

pub mod get_progress;
pub mod get_results;
pub mod post;

/// Warp filter to add the database pool to the handler. This function should
/// only be used for /v1/bulk endpoints, which are only enabled when worker mode
/// is enabled.
pub fn with_worker_db(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (PgPool,), Error = warp::Rejection> + Clone {
	warp::any().and_then(move || {
		let config = Arc::clone(&config);
		let pool = config.get_pg_pool();
		async move {
			if !config.worker.enable {
				return Err(warp::reject::custom(ReacherResponseError::new(
					StatusCode::SERVICE_UNAVAILABLE,
					"Please enable worker mode on Reacher before calling this endpoint",
				)));
			}
			pool.ok_or_else(|| {
				warp::reject::custom(ReacherResponseError::new(
					StatusCode::SERVICE_UNAVAILABLE,
					"Please configure a Postgres database on Reacher before calling this endpoint",
				))
			})
		}
	})
}

/// Like with_worker_db but returns the read replica pool (for SELECT-only endpoints).
pub fn with_worker_read_db(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (PgPool,), Error = warp::Rejection> + Clone {
	warp::any().and_then(move || {
		let config = Arc::clone(&config);
		let pool = config.get_read_pg_pool();
		async move {
			if !config.worker.enable {
				return Err(warp::reject::custom(ReacherResponseError::new(
					StatusCode::SERVICE_UNAVAILABLE,
					"Please enable worker mode on Reacher before calling this endpoint",
				)));
			}
			pool.ok_or_else(|| {
				warp::reject::custom(ReacherResponseError::new(
					StatusCode::SERVICE_UNAVAILABLE,
					"Please configure a Postgres database on Reacher before calling this endpoint",
				))
			})
		}
	})
}
