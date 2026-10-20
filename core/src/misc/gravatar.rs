use crate::LOG_TARGET;
use md5::Digest;

const API_BASE_URL: &str = "https://www.gravatar.com/avatar/";

/// Gravatar keys avatars by the MD5 of the trimmed, lowercased address, so
/// John@Example.com would otherwise never find the avatar of john@example.com.
fn gravatar_url(email: &str) -> String {
	let mail_hash: Digest = md5::compute(email.trim().to_lowercase());
	format!("{API_BASE_URL}{mail_hash:x}")
}

pub async fn check_gravatar(to_email: &str) -> Option<String> {
	let client = reqwest::Client::builder()
		.connect_timeout(std::time::Duration::from_secs(5))
		.timeout(std::time::Duration::from_secs(10))
		.build()
		.ok()?;

	let url = gravatar_url(to_email);

	tracing::debug!(
		target: LOG_TARGET,
		email=to_email,
		url=url,
		"Request Gravatar API"
	);

	let response = client
		.get(&url)
		// This option is necessary to return a NotFound exception instead of the default gravatar
		// image if none for the given email is found.
		.query(&[("d", "404")])
		.send()
		.await;

	tracing::debug!(
		target: LOG_TARGET,
		"[email={}] Gravatar response: {:?}",
		to_email,
		response
	);

	let response = match response {
		Ok(response) => response,
		Err(_) => return None,
	};

	match response.status() {
		reqwest::StatusCode::OK => Some(url),
		_ => None,
	}
}

#[cfg(test)]
mod tests {
	use super::gravatar_url;

	#[test]
	fn hashes_the_trimmed_lowercased_address() {
		// The example from https://docs.gravatar.com/api/avatars/hash/
		assert_eq!(
			gravatar_url(" MyEmailAddress@example.com "),
			"https://www.gravatar.com/avatar/0bc83cb571cd1c50ba6f3e8a78ef1346"
		);
	}
}
