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
		let snapshot = config.smtp_health.snapshot(config.smtp_health_enabled);
		warp::reply::with_header(format!(
			"# TYPE reacher_smtp_monitor_enabled gauge\nreacher_smtp_monitor_enabled {}\n\
			 # TYPE reacher_smtp_capability_ready gauge\nreacher_smtp_capability_ready {}\n\
			 # TYPE reacher_smtp_capability_checked_timestamp_seconds gauge\nreacher_smtp_capability_checked_timestamp_seconds {}\n",
			u8::from(config.smtp_health_enabled), u8::from(snapshot.status == "ready"),
			snapshot.checked_at.unwrap_or(0),
		), "content-type", "text/plain; version=0.0.4; charset=utf-8")
	})
}
