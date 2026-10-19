use crate::LOG_TARGET;
use reqwest::{Client, Url};

const MAIN_API_URL: &str = "https://haveibeenpwned.com/api/v3/";

/// The account goes in the path as one percent-encoded segment. A local part
/// may contain "/", "?" or "#", which would otherwise change the request.
fn breached_account_url(to_email: &str) -> Url {
	let mut url = Url::parse(MAIN_API_URL).expect("MAIN_API_URL is a valid URL");
	url.path_segments_mut()
		.expect("an https URL has path segments")
		.pop_if_empty()
		.extend(["breachedaccount", to_email]);
	url.query_pairs_mut()
		.append_pair("truncateResponse", "false");
	url
}

/// Check if the email has been found in any breach or paste using the
/// HaveIBeenPwned API.
/// This function will return the number of times the email has been found in
/// any breach.
pub async fn check_haveibeenpwned(to_email: &str, api_key: Option<String>) -> Option<bool> {
	let client = Client::new();
	let url = breached_account_url(to_email);

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

#[cfg(test)]
mod tests {
	use super::breached_account_url;

	#[test]
	fn encodes_the_account_as_one_path_segment() {
		// "/", "?", "#" and "%" are all legal in a local part.
		assert_eq!(
			breached_account_url("a/b?c#d%e@example.com").as_str(),
			"https://haveibeenpwned.com/api/v3/breachedaccount/a%2Fb%3Fc%23d%25e@example.com?truncateResponse=false"
		);
	}
}
