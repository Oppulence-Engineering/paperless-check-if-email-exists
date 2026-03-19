use crate::config::BackendConfig;
use serde::Serialize;
use std::sync::Arc;
use std::time::Instant;
use warp::http::StatusCode;
use warp::Filter;

#[derive(Serialize)]
struct CheckStatus {
	status: &'static str,
	#[serde(skip_serializing_if = "Option::is_none")]
	latency_ms: Option<u64>,
}

#[derive(Serialize)]
struct Checks {
	postgres: CheckStatus,
	rabbitmq: CheckStatus,
}

#[derive(Serialize)]
struct ReadinessResponse {
	status: &'static str,
	checks: Checks,
}

async fn handler(config: Arc<BackendConfig>) -> Result<impl warp::Reply, warp::Rejection> {
	// Check Postgres
	let pg_check = if let Some(pool) = config.get_pg_pool() {
		let start = Instant::now();
		let result = tokio::time::timeout(
			std::time::Duration::from_secs(2),
			sqlx::query("SELECT 1").execute(&pool),
		)
		.await;
		match result {
			Ok(Ok(_)) => CheckStatus {
				status: "ok",
				latency_ms: Some(start.elapsed().as_millis() as u64),
			},
			_ => CheckStatus {
				status: "unavailable",
				latency_ms: None,
			},
		}
	} else {
		CheckStatus {
			status: "not_configured",
			latency_ms: None,
		}
	};

	// Check RabbitMQ
	let rmq_check = if config.worker.enable {
		match config.must_worker_config() {
			Ok(worker_config) => {
				if worker_config.channel.status().connected() {
					CheckStatus {
						status: "ok",
						latency_ms: None,
					}
				} else {
					CheckStatus {
						status: "unavailable",
						latency_ms: None,
					}
				}
			}
			Err(_) => CheckStatus {
				status: "unavailable",
				latency_ms: None,
			},
		}
	} else {
		CheckStatus {
			status: "not_configured",
			latency_ms: None,
		}
	};

	let overall = match (pg_check.status, rmq_check.status) {
		("ok", "ok") | ("ok", "not_configured") | ("not_configured", "not_configured") => "ok",
		("unavailable", _) | (_, "unavailable") => "unavailable",
		_ => "degraded",
	};

	let status_code = if overall == "ok" {
		StatusCode::OK
	} else {
		StatusCode::SERVICE_UNAVAILABLE
	};

	let response = ReadinessResponse {
		status: overall,
		checks: Checks {
			postgres: pg_check,
			rabbitmq: rmq_check,
		},
	};

	Ok(warp::reply::with_status(
		warp::reply::json(&response),
		status_code,
	))
}

/// GET /readyz
///
/// Performs dependency checks for Postgres and RabbitMQ and returns readiness status.
#[utoipa::path(
	get,
	path = "/readyz",
	tag = "Health",
	responses((status = 200, description = "Service readiness and dependency checks"))
)]
pub fn readyz(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("readyz")
		.and(warp::get())
		.and(warp::any().map(move || Arc::clone(&config)))
		.and_then(handler)
}
