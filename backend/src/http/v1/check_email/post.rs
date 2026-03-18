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

//! This file implements the `POST /v1/check_email` endpoint.

use check_if_email_exists::LOG_TARGET;
use bytes::Bytes;
use warp::http::header::CONTENT_TYPE;
use serde_json;
use std::sync::Arc;
use warp::Filter;

use crate::config::BackendConfig;
use crate::http::v0::check_email::post::{with_config, CheckEmailRequest};
use crate::http::resolve_tenant;
use crate::http::shared::check_email::handle_check_email;
use crate::http::ReacherResponseError;
use crate::tenant::context::TenantContext;
use warp::Reply;

/// The main endpoint handler — delegates to shared check_email logic.
async fn http_handler(
	tenant_ctx: TenantContext,
	config: Arc<BackendConfig>,
	idempotency_key: Option<String>,
	body: Bytes,
) -> Result<impl warp::Reply, warp::Rejection> {
	let request_body = body.clone();
	let body: CheckEmailRequest = serde_json::from_slice(&request_body).map_err(ReacherResponseError::from)?;
	let result = handle_check_email(
		config,
		&request_body,
		&body,
		&tenant_ctx,
		"/v1/check_email",
		idempotency_key,
	)
	.await?;

	let mut response = warp::reply::with_status(result.body, result.status_code).into_response();
	response
		.headers_mut()
		.insert(CONTENT_TYPE, "application/json".parse().unwrap());
	Ok(response)
}

/// POST /v1/check_email
///
/// Verifies an email address and returns a result.
#[utoipa::path(
	post,
	path = "/v1/check_email",
	tag = "v1",
	params((
		"Idempotency-Key" = Option<String>, Header, description = "Optional idempotency key")),
	responses((status = 200, description = "Email verification result"))
)]
pub fn v1_check_email(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "check_email")
		.and(warp::post())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_config(config.clone()))
		.and(warp::header::optional::<String>("Idempotency-Key"))
		.and(warp::body::content_length_limit(1024 * 16))
		.and(warp::body::bytes())
		.and_then(http_handler)
		.with(warp::log(LOG_TARGET))
}
