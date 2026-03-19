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

//! This file implements the `POST /v0/check_email` endpoint.

use bytes::Bytes;
use check_if_email_exists::smtp::verif_method::VerifMethod;
use check_if_email_exists::{CheckEmailInput, CheckEmailInputProxy, LOG_TARGET};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use warp::http::header::CONTENT_TYPE;
use warp::Filter;
use warp::Reply;

use super::backwardcompat::{BackwardCompatHotmailB2CVerifMethod, BackwardCompatYahooVerifMethod};
use crate::config::BackendConfig;
use crate::http::deprecation::add_deprecation_headers;
use crate::http::resolve_tenant;
use crate::http::shared::check_email::handle_check_email;
use crate::http::ReacherResponseError;
use crate::tenant::context::TenantContext;

/// The request body for the `POST /v0/check_email` endpoint.
#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CheckEmailRequest {
	pub to_email: String,
	pub from_email: Option<String>,
	pub hello_name: Option<String>,
	pub proxy: Option<CheckEmailInputProxy>,
	pub smtp_timeout: Option<Duration>,
	pub smtp_port: Option<u16>,
	// The following fields are for backward compatibility.
	pub yahoo_verif_method: Option<BackwardCompatYahooVerifMethod>,
	pub hotmailb2c_verif_method: Option<BackwardCompatHotmailB2CVerifMethod>,
}

impl CheckEmailRequest {
	pub fn to_check_email_input(&self, config: Arc<BackendConfig>) -> CheckEmailInput {
		let hello_name = self
			.hello_name
			.clone()
			.unwrap_or_else(|| config.hello_name.clone());
		let from_email = self
			.from_email
			.clone()
			.unwrap_or_else(|| config.from_email.clone());
		let smtp_timeout = self
			.smtp_timeout
			.or_else(|| config.smtp_timeout.map(Duration::from_secs));
		let smtp_port = self.smtp_port.unwrap_or(25);
		let retries = 1;

		let mut verif_method = if let Some(proxy) = &self.proxy {
			VerifMethod::new_with_same_config_for_all(
				Some(proxy.clone()),
				hello_name.clone(),
				from_email.clone(),
				smtp_port,
				smtp_timeout.clone(),
				retries,
			)
		} else {
			config.get_verif_method()
		};

		if let Some(yahoo_verif_method) = &self.yahoo_verif_method {
			verif_method.yahoo = yahoo_verif_method.to_yahoo_verif_method(
				self.proxy.is_some(),
				hello_name.clone(),
				from_email.clone(),
				smtp_timeout.clone(),
				smtp_port,
				retries,
			);
		}
		if let Some(hotmailb2c_verif_method) = &self.hotmailb2c_verif_method {
			verif_method.hotmailb2c = hotmailb2c_verif_method.to_hotmailb2c_verif_method(
				self.proxy.is_some(),
				hello_name,
				from_email,
				smtp_timeout,
				smtp_port,
				retries,
			);
		}

		CheckEmailInput {
			to_email: self.to_email.clone(),
			verif_method,
			sentry_dsn: config.sentry_dsn.clone(),
			backend_name: config.backend_name.clone(),
			webdriver_config: config.webdriver.clone(),
			..Default::default()
		}
	}
}

/// The main endpoint handler — delegates to shared check_email logic
/// and adds deprecation headers.
async fn http_handler(
	tenant_ctx: TenantContext,
	config: Arc<BackendConfig>,
	idempotency_key: Option<String>,
	body: Bytes,
) -> Result<impl warp::Reply, warp::Rejection> {
	let request_body = body.clone();
	let body: CheckEmailRequest =
		serde_json::from_slice(&request_body).map_err(ReacherResponseError::from)?;
	let result = handle_check_email(
		config,
		&request_body,
		&body,
		&tenant_ctx,
		"/v0/check_email",
		idempotency_key,
	)
	.await?;

	let mut response = warp::reply::with_status(result.body, result.status_code).into_response();
	response
		.headers_mut()
		.insert(CONTENT_TYPE, "application/json".parse().unwrap());
	Ok(add_deprecation_headers(
		response,
		"2026-09-16",
		"/v1/check_email",
	))
}

/// POST /v0/check_email
///
/// Legacy email verification endpoint (deprecated, retained for compatibility).
#[utoipa::path(
	post,
	path = "/v0/check_email",
	tag = "v0",
	params(
		("Idempotency-Key" = Option<String>, Header, description = "Optional idempotency key")
	),
	responses((status = 200, description = "Email verification result"),)
)]
pub fn post_check_email<'a>(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone + 'a {
	warp::path!("v0" / "check_email")
		.and(warp::post())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_config(config))
		.and(warp::header::optional::<String>("Idempotency-Key"))
		.and(warp::body::content_length_limit(1024 * 16))
		.and(warp::body::bytes())
		.and_then(http_handler)
		.with(warp::log(LOG_TARGET))
}

/// Warp filter that adds the BackendConfig to the handler.
pub fn with_config(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (Arc<BackendConfig>,), Error = std::convert::Infallible> + Clone {
	warp::any().map(move || Arc::clone(&config))
}
