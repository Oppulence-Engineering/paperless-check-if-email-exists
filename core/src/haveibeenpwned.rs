use crate::LOG_TARGET;
use reqwest::Client;

const MAIN_API_URL: &str = "https://haveibeenpwned.com/api/v3/";

/// Check if the email has been found in any breach or paste using the
/// HaveIBeenPwned API.
/// This function will return the number of times the email has been found in
/// any breach.
pub async fn check_haveibeenpwned(to_email: &str, api_key: Option<String>) -> Option<bool> {
	// The lookup runs before SMTP and outside the SMTP timeout, so it needs
	// its own bound.
	let client = Client::builder()
		.connect_timeout(std::time::Duration::from_secs(5))
		.timeout(std::time::Duration::from_secs(10))
		.build()
		.ok()?;
	// The address is one path segment: `/`, `#`, `?` and `%` are valid in a
	// local part and must not change the URL.
	let mut url = reqwest::Url::parse(MAIN_API_URL).ok()?;
	url.path_segments_mut()
		.ok()?
		.pop_if_empty()
		.push("breachedaccount")
		.push(to_email);
	url.set_query(Some("truncateResponse=false"));

	let request = client
		.get(url)
		.header("User-Agent", "reacher")
		.header("hibp-api-key", api_key.unwrap_or_default())
		.send()
		.await;

	match request {
		Ok(response) => {
			if response.status().is_success() {
				let breaches: Vec<serde_json::Value> = response.json().await.unwrap_or_default();
				tracing::debug!(
					target: LOG_TARGET,
					breach_count=breaches.len(),
					"HaveIBeenPwned check completed"
				);
				Some(!breaches.is_empty())
			} else if response.status() == reqwest::StatusCode::NOT_FOUND {
				Some(false)
			} else {
				tracing::error!(
					target: LOG_TARGET,
					status = %response.status(),
					"Error checking HaveIBeenPwned"
				);
				None
			}
		}
		Err(e) => {
			tracing::error!(
				target: LOG_TARGET,
				error=?e,
				"Error checking HaveIBeenPwned"
			);
			None
		}
	}
}
