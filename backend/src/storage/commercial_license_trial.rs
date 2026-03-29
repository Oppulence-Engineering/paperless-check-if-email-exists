// Reacher - Email Verification
// Copyright (C) 2018-2023 Reacher

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use crate::config::{BackendConfig, CommercialLicenseTrialConfig};
use crate::http::ReacherResponseError;
use crate::worker::do_work::TaskError;
use check_if_email_exists::LOG_TARGET;
use serde::Serialize;
use std::sync::Arc;
use tracing::debug;
use warp::http::StatusCode;

/// If we're in the Commercial License Trial, we also store the
/// result by sending it to back to Reacher.
pub async fn send_to_reacher<T>(
	config: Arc<BackendConfig>,
	email: &str,
	worker_output: Result<&T, &TaskError>,
) -> Result<(), ReacherResponseError>
where
	T: Serialize + ?Sized,
{
	if let Some(CommercialLicenseTrialConfig { api_token, url }) = &config.commercial_license_trial
	{
		let res = reqwest::Client::new()
			.post(url)
			.header("Authorization", api_token)
			.json(&worker_output)
			.send()
			.await?;

		// Error if not 2xx status code
		if !res.status().is_success() {
			let status = StatusCode::from_u16(res.status().as_u16())?;
			let body: serde_json::Value = res.json().await?;

			// Extract error message from the "error" field, if it exists, or
			// else just return the whole body.
			let error_body = body.get("error").unwrap_or(&body).to_owned();

			return Err(ReacherResponseError::new(status, error_body));
		}

		let res = res.text().await?;
		debug!(target: LOG_TARGET, email=email, res=res, "Sent result to Reacher Commercial License Trial");
	}

	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::scoring::compute_score;
	use crate::scoring::response::{PreparedCheckEmailSuccess, PreparedVerificationResponse};
	use check_if_email_exists::{CheckEmailOutput, Reachable};
	use serde_json::json;

	#[tokio::test]
	async fn test_send_to_reacher_no_config_is_noop() {
		let config = Arc::new(BackendConfig::empty());
		let output = CheckEmailOutput {
			input: "test@example.com".into(),
			is_reachable: Reachable::Invalid,
			..Default::default()
		};
		let result = send_to_reacher(config, "test@example.com", Ok(&output)).await;
		assert!(result.is_ok());
	}

	#[tokio::test]
	async fn test_send_to_reacher_with_error_result() {
		let config = Arc::new(BackendConfig::empty());
		let output = TaskError::Lapin(lapin::Error::InvalidChannel(0));
		let result =
			send_to_reacher::<CheckEmailOutput>(config, "test@example.com", Err(&output)).await;
		assert!(result.is_ok());
	}

	#[test]
	fn prepared_success_serializes_with_api_shape() {
		let input = "test@example.com".to_string();
		let output = CheckEmailOutput {
			input: input.clone(),
			is_reachable: Reachable::Safe,
			..Default::default()
		};
		let score = compute_score(&output);
		let prepared = PreparedCheckEmailSuccess {
			output,
			response: PreparedVerificationResponse {
				json: json!({
					"input": input,
					"score": { "score": 42 },
					"prepared_marker": true
				}),
				body: Vec::new(),
				score,
				canonical_email: None,
				bounce_risk: None,
				bounce_risk_signals: None,
			},
		};

		let serialized = serde_json::to_value(Ok::<_, TaskError>(&prepared)).unwrap();
		assert_eq!(
			serialized
				.get("Ok")
				.and_then(|value| value.get("prepared_marker")),
			Some(&json!(true))
		);
	}
}
