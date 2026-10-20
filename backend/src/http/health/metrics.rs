use crate::config::BackendConfig;
use std::sync::Arc;
use warp::Filter;

#[utoipa::path(
	get, path = "/metrics", tag = "Health",
	responses((status = 200, description = "Cached SMTP capability gauges", content_type = "text/plain", body = String))
)]
pub fn smtp_metrics(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("metrics").and(warp::get()).map(move || {
		let required = config.verification_backend == crate::verification::VerificationBackend::Reacher;
		let enabled = config.smtp_health_enabled && required;
		let snapshot = config.smtp_health.snapshot(enabled);
		warp::reply::with_header(format!(
			"# TYPE reacher_smtp_required gauge\nreacher_smtp_required {}\n\
			 # TYPE reacher_smtp_monitor_enabled gauge\nreacher_smtp_monitor_enabled {}\n\
			 # TYPE reacher_smtp_capability_ready gauge\nreacher_smtp_capability_ready {}\n\
			 # TYPE reacher_smtp_capability_checked_timestamp_seconds gauge\nreacher_smtp_capability_checked_timestamp_seconds {}\n",
			u8::from(required), u8::from(enabled), u8::from(snapshot.status == "ready"),
			snapshot.checked_at.unwrap_or(0),
		), "content-type", "text/plain; version=0.0.4; charset=utf-8")
	})
}

#[cfg(test)]
mod tests {
	use super::*;

	#[tokio::test]
	async fn external_verification_does_not_claim_smtp_readiness_or_require_smtp_monitoring() {
		let mut config = BackendConfig::empty();
		config.verification_backend = crate::verification::VerificationBackend::Millionverifier;
		config.smtp_health_enabled = true;
		let response = warp::test::request()
			.path("/metrics")
			.reply(&smtp_metrics(Arc::new(config)))
			.await;
		let body = std::str::from_utf8(response.body()).unwrap();
		assert!(body.contains("reacher_smtp_required 0\n"));
		assert!(body.contains("reacher_smtp_monitor_enabled 0\n"));
		assert!(body.contains("reacher_smtp_capability_ready 0\n"));
	}
}
