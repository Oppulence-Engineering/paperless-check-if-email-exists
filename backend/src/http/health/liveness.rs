use serde::Serialize;
use warp::Filter;

#[derive(Serialize)]
struct LivenessResponse {
	status: &'static str,
}

async fn handler() -> Result<impl warp::Reply, warp::Rejection> {
	Ok(warp::reply::json(&LivenessResponse { status: "ok" }))
}

/// GET /healthz
///
/// Returns a lightweight readiness signal with no upstream dependency checks.
#[utoipa::path(
	get,
	path = "/healthz",
	tag = "Health",
	responses((status = 200, description = "Service is alive"))
)]
pub fn healthz() -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("healthz")
		.and(warp::get())
		.and_then(handler)
}

#[cfg(test)]
mod tests {
	use super::*;
	use warp::http::StatusCode;
	use warp::test::request;

	#[tokio::test]
	async fn test_healthz_returns_200_ok() {
		let resp = request()
			.path("/healthz")
			.method("GET")
			.reply(&healthz())
			.await;
		assert_eq!(resp.status(), StatusCode::OK);
		let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
		assert_eq!(body["status"], "ok");
	}

	#[tokio::test]
	async fn test_healthz_post_not_allowed() {
		let resp = request()
			.path("/healthz")
			.method("POST")
			.reply(&healthz())
			.await;
		assert_ne!(resp.status(), StatusCode::OK);
	}
}
