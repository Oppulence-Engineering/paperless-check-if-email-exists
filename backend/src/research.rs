//! Server-only bridge to the existing Go research service.
use crate::tenant::context::TenantContext;
use reqwest::{Client, Method, StatusCode, Url};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{fmt, sync::OnceLock, time::Duration};

const API_PATH: &str = "/lead-scraper-microservice/api/v1";
const MAX_RESPONSE_BYTES: usize = 8 * 1024 * 1024;

#[derive(Default, Serialize, Deserialize)]
pub struct ResearchConfig {
	pub url: Option<String>,
	#[serde(default, skip_serializing)]
	pub bootstrap_api_key: Option<String>,
	#[serde(skip)]
	client: OnceLock<Client>,
}

impl fmt::Debug for ResearchConfig {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("ResearchConfig")
			.field("url", &self.url)
			.field(
				"bootstrap_api_key",
				&self.bootstrap_api_key.as_ref().map(|_| "<redacted>"),
			)
			.finish()
	}
}

#[derive(Debug, thiserror::Error)]
#[error("{message}")]
pub struct ResearchError {
	pub status: StatusCode,
	pub message: &'static str,
}

fn unavailable() -> ResearchError {
	ResearchError {
		status: StatusCode::SERVICE_UNAVAILABLE,
		message: "Company research is not configured.",
	}
}

fn upstream_error() -> ResearchError {
	ResearchError {
		status: StatusCode::BAD_GATEWAY,
		message: "The research service could not complete this request.",
	}
}

// Credentials and Go tenant authority never enter the public response contract.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResearchWorkspace {
	organization_id: String,
	tenant_id: String,
	account_id: String,
	workspace_id: String,
	workspace_api_key: String,
}

impl ResearchWorkspace {
	fn valid(&self) -> bool {
		[
			&self.organization_id,
			&self.tenant_id,
			&self.account_id,
			&self.workspace_id,
		]
		.iter()
		.all(|id| id.parse::<u64>().is_ok_and(|id| id > 0))
			&& !self.workspace_api_key.trim().is_empty()
	}

	fn scope(&self) -> Value {
		json!({"organizationId": self.organization_id, "tenantId": self.tenant_id,
			"accountId": self.account_id, "workspaceId": self.workspace_id})
	}
}

impl ResearchConfig {
	pub fn validate(&self) -> anyhow::Result<()> {
		if self.url.is_none() && self.bootstrap_api_key.is_none() {
			return Ok(());
		}
		anyhow::ensure!(self.base_url().is_ok(), "RCH__RESEARCH__URL must be an HTTP(S) origin and RCH__RESEARCH__BOOTSTRAP_API_KEY must be configured.");
		Ok(())
	}

	fn base_url(&self) -> Result<Url, ResearchError> {
		let url =
			Url::parse(self.url.as_deref().ok_or_else(unavailable)?).map_err(|_| unavailable())?;
		if !matches!(url.scheme(), "http" | "https")
			|| url.host_str().is_none()
			|| !url.username().is_empty()
			|| url.password().is_some()
			|| url.query().is_some()
			|| url.fragment().is_some()
			|| url.path() != "/"
			|| self
				.bootstrap_api_key
				.as_deref()
				.unwrap_or("")
				.trim()
				.is_empty()
		{
			return Err(unavailable());
		}
		Ok(url)
	}

	fn client(&self) -> &Client {
		check_if_email_exists::initialize_crypto_provider();
		self.client.get_or_init(|| {
			Client::builder()
				.connect_timeout(Duration::from_secs(5))
				.timeout(Duration::from_secs(30))
				.redirect(reqwest::redirect::Policy::none())
				.retry(reqwest::retry::never())
				.build()
				.expect("static research HTTP client configuration")
		})
	}

	pub async fn workspace(
		&self,
		tenant: &TenantContext,
	) -> Result<ResearchWorkspace, ResearchError> {
		let id = tenant
			.tenant_id
			.filter(|id| !tenant.is_legacy && !id.is_nil())
			.ok_or(ResearchError {
				status: StatusCode::FORBIDDEN,
				message: "A customer workspace is required for research.",
			})?;
		let url = self
			.base_url()?
			.join(&format!("{API_PATH}/internal/verifier-workspace"))
			.map_err(|_| unavailable())?;
		let response = self
			.client()
			.post(url)
			.header(
				"X-Api-Key",
				self.bootstrap_api_key.as_deref().ok_or_else(unavailable)?,
			)
			.json(&json!({"verifierTenantId": id, "name": tenant.tenant_name}))
			.send()
			.await
			.map_err(|_| upstream_error())?;
		let workspace: ResearchWorkspace =
			serde_json::from_value(read_response(response).await?).map_err(|_| upstream_error())?;
		if !workspace.valid() {
			return Err(upstream_error());
		}
		Ok(workspace)
	}

	pub async fn create_session(
		&self,
		workspace: &ResearchWorkspace,
		input: Value,
	) -> Result<Value, ResearchError> {
		self.session_request(workspace, Method::POST, "", input)
			.await
	}

	pub async fn list_sessions(
		&self,
		workspace: &ResearchWorkspace,
	) -> Result<Value, ResearchError> {
		self.session_request(workspace, Method::GET, "", json!({"pageSize": 50}))
			.await
	}

	pub async fn session(
		&self,
		workspace: &ResearchWorkspace,
		session_id: u64,
	) -> Result<Value, ResearchError> {
		self.session_request(
			workspace,
			Method::GET,
			&format!("/{}", positive_id(session_id)?),
			json!({}),
		)
		.await
	}

	pub async fn rows(
		&self,
		workspace: &ResearchWorkspace,
		session_id: u64,
	) -> Result<Value, ResearchError> {
		self.session_request(
			workspace,
			Method::GET,
			&format!("/{}/rows", positive_id(session_id)?),
			json!({"pageSize": 100}),
		)
		.await
	}

	pub async fn columns(
		&self,
		workspace: &ResearchWorkspace,
		session_id: u64,
	) -> Result<Value, ResearchError> {
		self.session_request(
			workspace,
			Method::GET,
			&format!("/{}/columns", positive_id(session_id)?),
			json!({}),
		)
		.await
	}

	pub async fn add_column(
		&self,
		workspace: &ResearchWorkspace,
		session_id: u64,
		input: Value,
	) -> Result<Value, ResearchError> {
		self.session_request(
			workspace,
			Method::POST,
			&format!("/{}/columns", positive_id(session_id)?),
			input,
		)
		.await
	}

	async fn session_request(
		&self,
		workspace: &ResearchWorkspace,
		method: Method,
		suffix: &str,
		mut input: Value,
	) -> Result<Value, ResearchError> {
		let fields = input.as_object_mut().ok_or_else(upstream_error)?;
		for alias in ["organization_id", "tenant_id", "account_id", "workspace_id"] {
			fields.remove(alias);
		}
		// Overwrite any supplied authority; only the authenticated mapping can set it.
		for (key, value) in workspace.scope().as_object().expect("scope object") {
			fields.insert(key.clone(), value.clone());
		}
		let url = self
			.base_url()?
			.join(&format!(
				"{API_PATH}/workspaces/{}/research-sessions{suffix}",
				workspace.workspace_id
			))
			.map_err(|_| unavailable())?;
		let mut request = self
			.client()
			.request(method.clone(), url)
			.header("X-Api-Key", &workspace.workspace_api_key)
			.header("X-Tenant-Id", &workspace.tenant_id)
			.header("X-Organization-Id", &workspace.organization_id);
		if method == Method::GET {
			let query: Vec<_> = fields
				.iter()
				.map(|(key, value)| {
					(
						key.clone(),
						value
							.as_str()
							.map(str::to_owned)
							.unwrap_or_else(|| value.to_string()),
					)
				})
				.collect();
			request = request.query(&query);
		} else {
			request = request.json(&input);
		}
		read_response(request.send().await.map_err(|_| upstream_error())?).await
	}
}

fn positive_id(id: u64) -> Result<u64, ResearchError> {
	if id == 0 {
		return Err(ResearchError {
			status: StatusCode::BAD_REQUEST,
			message: "A valid research identifier is required.",
		});
	}
	Ok(id)
}

async fn read_response(mut response: reqwest::Response) -> Result<Value, ResearchError> {
	if !response.status().is_success() {
		let (status, message) = match response.status() {
			StatusCode::NOT_FOUND => (
				StatusCode::NOT_FOUND,
				"Research item not found in this workspace.",
			),
			StatusCode::TOO_MANY_REQUESTS => (
				StatusCode::TOO_MANY_REQUESTS,
				"This research exceeds the workspace budget or rate limit.",
			),
			StatusCode::BAD_REQUEST => {
				(StatusCode::BAD_REQUEST, "The research request is invalid.")
			}
			_ => (
				StatusCode::BAD_GATEWAY,
				"The research service could not complete this request.",
			),
		};
		return Err(ResearchError { status, message });
	}
	if response
		.content_length()
		.is_some_and(|size| size > MAX_RESPONSE_BYTES as u64)
	{
		return Err(upstream_error());
	}
	let mut data = Vec::new();
	while let Some(chunk) = response.chunk().await.map_err(|_| upstream_error())? {
		if data.len() + chunk.len() > MAX_RESPONSE_BYTES {
			return Err(upstream_error());
		}
		data.extend_from_slice(&chunk);
	}
	serde_json::from_slice(&data).map_err(|_| upstream_error())
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::config::ThrottleConfig;
	use warp::Filter;

	#[test]
	fn configuration_rejects_credential_urls_and_redacts_secrets() {
		for url in [
			"file:///tmp/data",
			"https://key@example.com",
			"https://example.com/path",
			"https://example.com?key=value",
		] {
			let config = ResearchConfig {
				url: Some(url.into()),
				bootstrap_api_key: Some("private-secret".into()),
				..Default::default()
			};
			assert!(config.validate().is_err());
		}
		let config = ResearchConfig {
			url: Some("https://research.example.com".into()),
			bootstrap_api_key: Some("private-secret".into()),
			..Default::default()
		};
		assert!(config.validate().is_ok());
		assert!(!format!("{config:?}").contains("private-secret"));
		assert!(!serde_json::to_string(&config)
			.unwrap()
			.contains("private-secret"));
	}

	#[tokio::test]
	async fn bridge_uses_only_mapped_authority_and_never_forwards_bootstrap_key() {
		let tenant_id = uuid::Uuid::parse_str("f365604f-3a62-4142-b44e-cf73d42f322a").unwrap();
		let routes = warp::path::full().and(warp::header::<String>("x-api-key")).and(warp::body::json())
			.map(move |path: warp::path::FullPath, key: String, body: Value| {
				if path.as_str().ends_with("/internal/verifier-workspace") {
					assert_eq!(key, "bootstrap-secret");
					assert_eq!(body["verifierTenantId"], tenant_id.to_string());
					warp::reply::json(&json!({"organizationId":"11", "tenantId":"22", "accountId":"33", "workspaceId":"44", "workspaceApiKey":"tenant-secret"}))
				} else {
					assert_eq!(path.as_str(), format!("{API_PATH}/workspaces/44/research-sessions"));
					assert_eq!(key, "tenant-secret");
					assert_eq!(body["workspaceId"], "44");
					assert_eq!(body["tenantId"], "22");
					warp::reply::json(&json!({"session":{"id":"55"}}))
				}
			});
		let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
		let address = listener.local_addr().unwrap();
		let server = tokio::spawn(warp::serve(routes).incoming(listener).run());
		let config = ResearchConfig {
			url: Some(format!("http://{address}")),
			bootstrap_api_key: Some("bootstrap-secret".into()),
			..Default::default()
		};
		let mut tenant = TenantContext::legacy(ThrottleConfig::new_without_throttle());
		assert!(config.workspace(&tenant).await.is_err());
		tenant.is_legacy = false;
		tenant.tenant_id = Some(tenant_id);
		let workspace = config.workspace(&tenant).await.unwrap();
		let result = config
			.create_session(&workspace, json!({"workspaceId":"999", "tenantId":"999"}))
			.await
			.unwrap();
		assert_eq!(result["session"]["id"], "55");
		assert!(!result.to_string().contains("secret"));
		server.abort();
	}

	#[tokio::test]
	async fn bridge_rejects_redirects_without_leaking_credentials_or_error_bodies() {
		use std::sync::{
			atomic::{AtomicUsize, Ordering},
			Arc,
		};
		let calls = Arc::new(AtomicUsize::new(0));
		let observed = calls.clone();
		let destination = warp::any().map(move || {
			observed.fetch_add(1, Ordering::SeqCst);
			warp::reply::json(&json!({}))
		});
		let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
		let destination_address = listener.local_addr().unwrap();
		let destination_server = tokio::spawn(warp::serve(destination).incoming(listener).run());
		let redirect = warp::any().map(move || {
			warp::http::Response::builder()
				.status(302)
				.header("Location", format!("http://{destination_address}/collect"))
				.body("upstream secret that must not reach the user")
				.unwrap()
		});
		let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
		let address = listener.local_addr().unwrap();
		let server = tokio::spawn(warp::serve(redirect).incoming(listener).run());
		let config = ResearchConfig {
			url: Some(format!("http://{address}")),
			bootstrap_api_key: Some("bootstrap-secret".into()),
			..Default::default()
		};
		let mut tenant = TenantContext::legacy(ThrottleConfig::new_without_throttle());
		tenant.is_legacy = false;
		tenant.tenant_id =
			Some(uuid::Uuid::parse_str("f365604f-3a62-4142-b44e-cf73d42f322a").unwrap());
		let error = config
			.workspace(&tenant)
			.await
			.err()
			.expect("redirect must be refused");
		assert_eq!(error.status, StatusCode::BAD_GATEWAY);
		assert!(!error.to_string().contains("secret"));
		assert_eq!(calls.load(Ordering::SeqCst), 0);
		server.abort();
		destination_server.abort();
	}
}
