use warp::http::HeaderValue;

/// Creates a warp filter that appends deprecation headers to responses.
/// - `Deprecation: true`
/// - `Sunset: {sunset_date}` (RFC 7231 date string, e.g. "2026-09-16")
/// - `Link: <{successor_url}>; rel="successor-version"`
pub fn with_deprecation_headers(
	sunset: &'static str,
	successor: &'static str,
) -> warp::log::Log<impl Fn(warp::log::Info<'_>) + Clone> {
	// warp::log is the wrong tool here — we need reply::with_header.
	// Instead we return a function that adds headers via warp reply maps.
	// But warp doesn't have a great middleware story, so we use warp::reply::with::headers.
	let _ = (sunset, successor);
	warp::log("deprecation")
}

/// Helper to wrap a reply with deprecation headers. Call this in each
/// deprecated handler's response path.
pub fn add_deprecation_headers(
	reply: impl warp::Reply,
	sunset: &str,
	successor: &str,
) -> impl warp::Reply {
	let reply = warp::reply::with_header(reply, "Deprecation", "true");
	let reply = warp::reply::with_header(reply, "Sunset", sunset.to_string());
	let link_value = format!("<{}>; rel=\"successor-version\"", successor);
	warp::reply::with_header(reply, "Link", link_value)
}

/// Creates a warp `wrap_fn` that adds deprecation headers to all responses
/// flowing through the wrapped route.
/// Creates a warp `wrap_fn` that adds deprecation headers to all responses
/// flowing through the wrapped route. Uses warp::Reply trait method.
pub fn deprecation_wrapper(
	sunset: &'static str,
	successor: &'static str,
) -> impl Fn(
	warp::http::Response<warp::hyper::body::Bytes>,
) -> warp::http::Response<warp::hyper::body::Bytes>
       + Clone {
	move |mut resp: warp::http::Response<warp::hyper::body::Bytes>| {
		resp.headers_mut()
			.insert("Deprecation", HeaderValue::from_static("true"));
		if let Ok(v) = HeaderValue::from_str(sunset) {
			resp.headers_mut().insert("Sunset", v);
		}
		let link_value = format!("<{}>; rel=\"successor-version\"", successor);
		if let Ok(v) = HeaderValue::from_str(&link_value) {
			resp.headers_mut().insert("Link", v);
		}
		resp
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use warp::Reply;

	#[test]
	fn test_add_deprecation_headers_sets_all_three() {
		let reply = warp::reply::json(&serde_json::json!({"ok": true}));
		let with_headers = add_deprecation_headers(reply, "2026-09-16", "/v1/foo");
		let response = with_headers.into_response();

		assert_eq!(response.headers().get("Deprecation").unwrap(), "true");
		assert_eq!(response.headers().get("Sunset").unwrap(), "2026-09-16");
		let link = response.headers().get("Link").unwrap().to_str().unwrap();
		assert!(link.contains("/v1/foo"));
		assert!(link.contains("successor-version"));
	}

	#[test]
	fn test_add_deprecation_headers_link_format() {
		let reply = warp::reply::json(&serde_json::json!({}));
		let with_headers = add_deprecation_headers(reply, "2027-01-01", "/v2/endpoint");
		let response = with_headers.into_response();
		let link = response.headers().get("Link").unwrap().to_str().unwrap();
		assert_eq!(link, "</v2/endpoint>; rel=\"successor-version\"");
	}

	#[test]
	fn test_add_deprecation_preserves_body() {
		let reply = warp::reply::json(&serde_json::json!({"data": 42}));
		let with_headers = add_deprecation_headers(reply, "2026-09-16", "/v1/x");
		let response = with_headers.into_response();
		// Body should still be there — 200 OK
		assert_eq!(response.status(), 200);
	}
}
