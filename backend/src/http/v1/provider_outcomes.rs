use crate::config::BackendConfig;
use crate::finder::require_tenant_id;
use crate::http::v1::bulk::with_worker_db;
use crate::http::v1::outcomes::{ingest_outcomes, OutcomeIngestRequest, OutcomeInput};
use crate::http::{check_scope, resolve_tenant, ReacherResponseError};
use crate::pipelines::{validate_resolved_webhook_target, validate_webhook_url};
use crate::tenant::context::{scope, TenantContext};
use crate::tenant::webhook::{sign_payload, WEBHOOK_SIGNATURE_HEADER};
use base64::engine::general_purpose::STANDARD as BASE64;
use base64::Engine;
use check_if_email_exists::LOG_TARGET;
use chrono::{DateTime, TimeZone, Utc};
use hmac::{Hmac, Mac};
use openssl::hash::MessageDigest;
use openssl::memcmp;
use openssl::sign::Verifier;
use openssl::x509::X509;
use rand::RngCore;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use sqlx::{PgPool, Row};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tracing::warn;
use uuid::Uuid;
use warp::http::{HeaderMap, StatusCode};
use warp::Filter;

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct CreateProviderEndpointInput {
	pub provider: String,
	pub label: String,
	#[serde(default = "default_active")]
	pub status: String,
	#[serde(default)]
	pub provider_config: Value,
	#[serde(default)]
	pub allowed_ips: Vec<String>,
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct UpdateProviderEndpointInput {
	pub label: Option<String>,
	pub status: Option<String>,
	pub provider_config: Option<Value>,
	pub allowed_ips: Option<Vec<String>>,
	#[serde(default)]
	pub rotate_delivery_token: bool,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ProviderEndpointView {
	pub endpoint_id: Uuid,
	pub provider: String,
	pub label: String,
	pub status: String,
	pub allowed_ips: Vec<String>,
	pub provider_configured: bool,
	pub webhook_path: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub delivery_token: Option<String>,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ProviderEndpointListResponse {
	pub provider_endpoints: Vec<ProviderEndpointView>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ProviderDeleteResponse {
	pub deleted: bool,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct InboundOutcomeResponse {
	pub receipt_id: Uuid,
	pub provider: String,
	pub accepted: i64,
	pub duplicates: i64,
	pub unmatched: i64,
	pub rejected: i64,
}

struct EndpointRow {
	id: Uuid,
	tenant_id: Uuid,
	provider: String,
	label: String,
	status: String,
	delivery_token_hash: String,
	provider_config: Value,
	allowed_ips: Vec<String>,
	created_at: DateTime<Utc>,
	updated_at: DateTime<Utc>,
}

fn default_active() -> String {
	"active".to_string()
}

fn validate_provider(provider: &str) -> Result<String, ReacherResponseError> {
	let provider = provider.trim().to_ascii_lowercase();
	if matches!(
		provider.as_str(),
		"sendgrid" | "ses" | "mailgun" | "postmark"
	) {
		Ok(provider)
	} else {
		Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"provider must be sendgrid, ses, mailgun, or postmark",
		))
	}
}

fn validate_status(status: &str) -> Result<String, ReacherResponseError> {
	let status = status.trim().to_ascii_lowercase();
	if matches!(status.as_str(), "active" | "paused" | "disabled") {
		Ok(status)
	} else {
		Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"status must be active, paused, or disabled",
		))
	}
}

fn generate_delivery_token() -> String {
	let mut bytes = [0u8; 32];
	rand::thread_rng().fill_bytes(&mut bytes);
	format!("rt_{}", hex::encode(bytes))
}

fn token_hash(token: &str) -> String {
	hex::encode(Sha256::digest(token.as_bytes()))
}

fn row_to_endpoint(row: &sqlx::postgres::PgRow) -> EndpointRow {
	EndpointRow {
		id: row.get("id"),
		tenant_id: row.get("tenant_id"),
		provider: row.get("provider"),
		label: row.get("label"),
		status: row.get("status"),
		delivery_token_hash: row.get("delivery_token_hash"),
		provider_config: row.get("provider_config"),
		allowed_ips: row.get("allowed_ips"),
		created_at: row.get("created_at"),
		updated_at: row.get("updated_at"),
	}
}

fn endpoint_view(row: EndpointRow, delivery_token: Option<String>) -> ProviderEndpointView {
	let token_segment = delivery_token.as_deref().unwrap_or("{delivery_token}");
	ProviderEndpointView {
		endpoint_id: row.id,
		provider: row.provider.clone(),
		label: row.label,
		status: row.status,
		allowed_ips: row.allowed_ips,
		provider_configured: row
			.provider_config
			.as_object()
			.map(|value| !value.is_empty())
			.unwrap_or(false),
		webhook_path: format!(
			"/v1/inbound/providers/{}/{}/{}",
			row.provider, row.id, token_segment
		),
		delivery_token,
		created_at: row.created_at,
		updated_at: row.updated_at,
	}
}

async fn list_handler(
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::SETTINGS)?;
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	let rows = sqlx::query(
		"SELECT id, tenant_id, provider, label, status, delivery_token_hash, provider_config, allowed_ips, created_at, updated_at FROM v1_provider_endpoints WHERE tenant_id = $1 AND deleted_at IS NULL ORDER BY created_at DESC",
	)
	.bind(tenant_id)
	.fetch_all(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;
	Ok(warp::reply::json(&ProviderEndpointListResponse {
		provider_endpoints: rows
			.iter()
			.map(row_to_endpoint)
			.map(|row| endpoint_view(row, None))
			.collect(),
	}))
}

async fn create_handler(
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	input: CreateProviderEndpointInput,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::SETTINGS)?;
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	let provider = validate_provider(&input.provider)?;
	let status = validate_status(&input.status)?;
	let label = input.label.trim();
	if label.is_empty() || label.len() > 200 {
		return Err(ReacherResponseError::new(StatusCode::BAD_REQUEST, "label is invalid").into());
	}
	for value in &input.allowed_ips {
		value.parse::<std::net::IpAddr>().map_err(|_| {
			ReacherResponseError::new(
				StatusCode::BAD_REQUEST,
				"allowed_ips contains an invalid IP",
			)
		})?;
	}
	let endpoint_id = Uuid::new_v4();
	let token = generate_delivery_token();
	let row = sqlx::query(
		"INSERT INTO v1_provider_endpoints (id, tenant_id, provider, label, status, delivery_token_hash, provider_config, allowed_ips) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING id, tenant_id, provider, label, status, delivery_token_hash, provider_config, allowed_ips, created_at, updated_at",
	)
	.bind(endpoint_id)
	.bind(tenant_id)
	.bind(provider)
	.bind(label)
	.bind(status)
	.bind(token_hash(&token))
	.bind(input.provider_config)
	.bind(input.allowed_ips)
	.fetch_one(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;
	Ok(warp::reply::with_status(
		warp::reply::json(&endpoint_view(row_to_endpoint(&row), Some(token))),
		StatusCode::CREATED,
	))
}

async fn update_handler(
	endpoint_id: Uuid,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
	input: UpdateProviderEndpointInput,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::SETTINGS)?;
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	let status = input.status.as_deref().map(validate_status).transpose()?;
	let label = input.label.as_deref().map(str::trim);
	if label.is_some_and(|value| value.is_empty() || value.len() > 200) {
		return Err(ReacherResponseError::new(StatusCode::BAD_REQUEST, "label is invalid").into());
	}
	if let Some(values) = &input.allowed_ips {
		for value in values {
			value.parse::<std::net::IpAddr>().map_err(|_| {
				ReacherResponseError::new(
					StatusCode::BAD_REQUEST,
					"allowed_ips contains an invalid IP",
				)
			})?;
		}
	}
	let token = input.rotate_delivery_token.then(generate_delivery_token);
	let row = sqlx::query(
		r#"
		UPDATE v1_provider_endpoints SET
		 label = COALESCE($3, label), status = COALESCE($4, status),
		 provider_config = COALESCE($5, provider_config), allowed_ips = COALESCE($6, allowed_ips),
		 delivery_token_hash = COALESCE($7, delivery_token_hash), updated_at = NOW()
		WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL
		RETURNING id, tenant_id, provider, label, status, delivery_token_hash, provider_config, allowed_ips, created_at, updated_at
		"#,
	)
	.bind(endpoint_id)
	.bind(tenant_id)
	.bind(label)
	.bind(status)
	.bind(input.provider_config)
	.bind(input.allowed_ips)
	.bind(token.as_deref().map(token_hash))
	.fetch_optional(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;
	let Some(row) = row else {
		return Err(ReacherResponseError::new(
			StatusCode::NOT_FOUND,
			"Provider endpoint not found",
		)
		.into());
	};
	Ok(warp::reply::json(&endpoint_view(
		row_to_endpoint(&row),
		token,
	)))
}

async fn delete_handler(
	endpoint_id: Uuid,
	tenant_ctx: TenantContext,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant_ctx, scope::SETTINGS)?;
	let tenant_id = require_tenant_id(tenant_ctx.tenant_id)?;
	let result = sqlx::query(
		"UPDATE v1_provider_endpoints SET status = 'disabled', deleted_at = NOW(), updated_at = NOW() WHERE id = $1 AND tenant_id = $2 AND deleted_at IS NULL",
	)
	.bind(endpoint_id)
	.bind(tenant_id)
	.execute(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;
	if result.rows_affected() == 0 {
		return Err(ReacherResponseError::new(
			StatusCode::NOT_FOUND,
			"Provider endpoint not found",
		)
		.into());
	}
	Ok(warp::reply::json(&ProviderDeleteResponse { deleted: true }))
}

fn header_value(headers: &HeaderMap, name: &str) -> Option<String> {
	headers
		.get(name)
		.and_then(|value| value.to_str().ok())
		.map(ToOwned::to_owned)
}

fn safe_headers(headers: &HeaderMap) -> Value {
	let mut result = Map::new();
	for name in [
		"content-type",
		"user-agent",
		"x-twilio-email-event-webhook-timestamp",
		"x-twilio-email-event-webhook-signature",
	] {
		if let Some(value) = header_value(headers, name) {
			result.insert(name.to_string(), Value::String(value));
		}
	}
	Value::Object(result)
}

fn parse_provider_payload(headers: &HeaderMap, body: &[u8]) -> Option<Value> {
	if let Ok(value) = serde_json::from_slice(body) {
		return Some(value);
	}
	let content_type = header_value(headers, "content-type")?;
	if !content_type
		.to_ascii_lowercase()
		.starts_with("application/x-www-form-urlencoded")
	{
		return None;
	}
	let fields: HashMap<String, String> = serde_urlencoded::from_bytes(body).ok()?;
	let mut event_data = fields
		.get("event-data")
		.and_then(|value| serde_json::from_str::<Value>(value).ok())
		.unwrap_or_else(|| {
			Value::Object(
				fields
					.iter()
					.map(|(key, value)| (key.clone(), Value::String(value.clone())))
					.collect(),
			)
		});
	if !event_data.is_object() {
		event_data = Value::Object(Map::new());
	}
	let signature_value = |name: &str| {
		fields
			.get(&format!("signature[{name}]"))
			.or_else(|| fields.get(name))
			.cloned()
	};
	let signature = ["timestamp", "token", "signature"]
		.iter()
		.filter_map(|name| {
			signature_value(name).map(|value| (name.to_string(), Value::String(value)))
		})
		.collect::<Map<String, Value>>();
	Some(serde_json::json!({
		"signature": signature,
		"event-data": event_data,
	}))
}

fn config_settings(config: &Value) -> &Map<String, Value> {
	config
		.get("settings")
		.and_then(Value::as_object)
		.or_else(|| config.as_object())
		.unwrap_or_else(|| {
			static EMPTY: std::sync::OnceLock<Map<String, Value>> = std::sync::OnceLock::new();
			EMPTY.get_or_init(Map::new)
		})
}

fn verify_mailgun(config: &Value, body: &Value) -> Result<(), String> {
	let settings = config_settings(config);
	let signing_key = settings
		.get("signing_key")
		.and_then(Value::as_str)
		.ok_or("Mailgun signing_key is not configured")?;
	let signature = body
		.get("signature")
		.ok_or("Mailgun signature is missing")?;
	let timestamp = signature
		.get("timestamp")
		.and_then(Value::as_str)
		.ok_or("Mailgun timestamp is missing")?;
	let signed_at = timestamp
		.parse::<i64>()
		.map_err(|_| "Mailgun timestamp is invalid")?;
	let tolerance = settings
		.get("verification_timestamp_tolerance_seconds")
		.and_then(Value::as_i64)
		.unwrap_or(300);
	if (Utc::now().timestamp() - signed_at).abs() > tolerance {
		return Err("Mailgun timestamp is outside the allowed tolerance".to_string());
	}
	let token = signature
		.get("token")
		.and_then(Value::as_str)
		.ok_or("Mailgun token is missing")?;
	let supplied = signature
		.get("signature")
		.and_then(Value::as_str)
		.ok_or("Mailgun signature is missing")?;
	let mut mac = Hmac::<Sha256>::new_from_slice(signing_key.as_bytes())
		.map_err(|_| "Mailgun signing key is invalid")?;
	mac.update(format!("{timestamp}{token}").as_bytes());
	let supplied = hex::decode(supplied).map_err(|_| "Mailgun signature is invalid hex")?;
	mac.verify_slice(&supplied)
		.map_err(|_| "Mailgun signature verification failed".to_string())
}

fn verify_sendgrid(config: &Value, headers: &HeaderMap, raw: &[u8]) -> Result<(), String> {
	let settings = config_settings(config);
	let public_key = settings
		.get("public_key_pem")
		.and_then(Value::as_str)
		.ok_or("SendGrid public_key_pem is not configured")?;
	let timestamp = header_value(headers, "x-twilio-email-event-webhook-timestamp")
		.ok_or("SendGrid timestamp header is missing")?;
	let signature = header_value(headers, "x-twilio-email-event-webhook-signature")
		.ok_or("SendGrid signature header is missing")?;
	let tolerance = settings
		.get("verification_timestamp_tolerance_seconds")
		.and_then(Value::as_i64)
		.unwrap_or(300);
	let signed_at = timestamp
		.parse::<i64>()
		.map_err(|_| "SendGrid timestamp is invalid")?;
	if (Utc::now().timestamp() - signed_at).abs() > tolerance {
		return Err("SendGrid timestamp is outside the allowed tolerance".to_string());
	}
	let key = openssl::pkey::PKey::public_key_from_pem(public_key.as_bytes())
		.map_err(|_| "SendGrid public key is invalid")?;
	let mut verifier = Verifier::new(MessageDigest::sha256(), &key)
		.map_err(|_| "Unable to initialize SendGrid verifier")?;
	verifier
		.update(timestamp.as_bytes())
		.and_then(|_| verifier.update(raw))
		.map_err(|_| "Unable to verify SendGrid payload")?;
	let signature = BASE64
		.decode(signature)
		.map_err(|_| "SendGrid signature is invalid base64")?;
	if !verifier
		.verify(&signature)
		.map_err(|_| "SendGrid verification failed")?
	{
		return Err("SendGrid signature verification failed".to_string());
	}
	Ok(())
}

async fn verify_ses(config: &Value, body: &Value) -> Result<(), String> {
	let settings = config_settings(config);
	let topic = body
		.get("TopicArn")
		.and_then(Value::as_str)
		.ok_or("SES TopicArn is missing")?;
	let topics = settings
		.get("topic_arns")
		.and_then(Value::as_array)
		.ok_or("SES topic_arns is not configured")?;
	if !topics.iter().any(|value| value.as_str() == Some(topic)) {
		return Err("SES TopicArn is not allowlisted".to_string());
	}
	let cert_url = body
		.get("SigningCertURL")
		.and_then(Value::as_str)
		.ok_or("SES SigningCertURL is missing")?;
	let parsed = reqwest::Url::parse(cert_url).map_err(|_| "SES SigningCertURL is invalid")?;
	let host = parsed.host_str().unwrap_or_default();
	if parsed.scheme() != "https"
		|| parsed.port_or_known_default() != Some(443)
		|| !parsed.username().is_empty()
		|| parsed.password().is_some()
		|| !(host == "sns.amazonaws.com"
			|| (host.starts_with("sns.")
				&& (host.ends_with(".amazonaws.com") || host.ends_with(".amazonaws.com.cn"))))
		|| !parsed.path().ends_with(".pem")
	{
		return Err("SES SigningCertURL is not trusted".to_string());
	}
	let client = reqwest::Client::builder()
		.timeout(Duration::from_secs(5))
		.redirect(reqwest::redirect::Policy::none())
		.build()
		.map_err(|_| "Unable to initialize SES certificate client")?;
	let response = client
		.get(parsed)
		.send()
		.await
		.map_err(|_| "Unable to fetch SES signing certificate")?;
	if !response.status().is_success()
		|| response
			.content_length()
			.is_some_and(|size| size > 1_048_576)
	{
		return Err("SES signing certificate response is invalid".to_string());
	}
	let cert = response
		.bytes()
		.await
		.map_err(|_| "Unable to read SES signing certificate")?;
	if cert.len() > 1_048_576 {
		return Err("SES signing certificate is too large".to_string());
	}
	let cert = X509::from_pem(&cert).map_err(|_| "SES signing certificate is invalid")?;
	let key = cert
		.public_key()
		.map_err(|_| "SES certificate has no public key")?;
	let signature_version = body
		.get("SignatureVersion")
		.and_then(Value::as_str)
		.unwrap_or("1");
	let digest = match signature_version {
		"1" => MessageDigest::sha1(),
		"2" => MessageDigest::sha256(),
		_ => return Err("SES SignatureVersion is not supported".to_string()),
	};
	let message_type = body
		.get("Type")
		.and_then(Value::as_str)
		.unwrap_or("Notification");
	let fields: &[&str] = if message_type == "Notification" {
		&[
			"Message",
			"MessageId",
			"Subject",
			"Timestamp",
			"TopicArn",
			"Type",
		]
	} else {
		&[
			"Message",
			"MessageId",
			"SubscribeURL",
			"Timestamp",
			"Token",
			"TopicArn",
			"Type",
		]
	};
	let mut canonical = String::new();
	for field in fields {
		if let Some(value) = body.get(*field).and_then(Value::as_str) {
			canonical.push_str(field);
			canonical.push('\n');
			canonical.push_str(value);
			canonical.push('\n');
		}
	}
	let signature = body
		.get("Signature")
		.and_then(Value::as_str)
		.ok_or("SES Signature is missing")?;
	let signature = BASE64
		.decode(signature)
		.map_err(|_| "SES Signature is invalid base64")?;
	let mut verifier =
		Verifier::new(digest, &key).map_err(|_| "Unable to initialize SES verifier")?;
	verifier
		.update(canonical.as_bytes())
		.map_err(|_| "Unable to verify SES payload")?;
	if !verifier
		.verify(&signature)
		.map_err(|_| "SES verification failed")?
	{
		return Err("SES signature verification failed".to_string());
	}
	Ok(())
}

fn event_family(event_type: &str) -> String {
	match event_type {
		"delivered" | "bounced" => "delivery",
		"opened" | "clicked" => "engagement",
		"complained" | "unsubscribed" => "negative_feedback",
		_ => "routing",
	}
	.to_string()
}

fn normalize_event_type(value: &str) -> Option<String> {
	match value.trim().to_ascii_lowercase().as_str() {
		"delivered" | "delivery" => Some("delivered".to_string()),
		"bounce" | "bounced" | "bounce_hard" | "bounce_soft" => Some("bounced".to_string()),
		"spamreport" | "complaint" | "complained" => Some("complained".to_string()),
		"unsubscribe" | "unsubscribed" => Some("unsubscribed".to_string()),
		"open" | "opened" => Some("opened".to_string()),
		"click" | "clicked" => Some("clicked".to_string()),
		_ => None,
	}
}

fn timestamp_from_value(value: Option<&Value>) -> Option<DateTime<Utc>> {
	match value {
		Some(Value::String(value)) => DateTime::parse_from_rfc3339(value)
			.ok()
			.map(|value| value.with_timezone(&Utc)),
		Some(Value::Number(value)) => value
			.as_i64()
			.and_then(|value| Utc.timestamp_opt(value, 0).single()),
		_ => None,
	}
}

fn make_outcome(
	endpoint: &EndpointRow,
	receipt_id: Uuid,
	email: String,
	event_type: String,
	provider_event_id: Option<String>,
	provider_message_id: Option<String>,
	occurred_at: Option<DateTime<Utc>>,
	metadata: Value,
) -> OutcomeInput {
	let source_key = metadata
		.get("source_key")
		.or_else(|| metadata.get("reacher_source_key"))
		.and_then(Value::as_str)
		.map(ToOwned::to_owned);
	let campaign_id = metadata
		.get("campaign_id")
		.and_then(Value::as_str)
		.map(ToOwned::to_owned);
	OutcomeInput {
		email,
		event_family: Some(event_family(&event_type)),
		event_type,
		source_key,
		campaign_id,
		occurred_at,
		metadata: Some(metadata),
		endpoint_id: Some(endpoint.id),
		receipt_id: Some(receipt_id),
		provider_event_id,
		provider_message_id,
		correlation_status: Some("unmatched".to_string()),
	}
}

async fn forward_outcomes(
	endpoint: &EndpointRow,
	receipt_id: Uuid,
	provider: &str,
	outcomes: &[OutcomeInput],
) -> Result<bool, String> {
	let settings = config_settings(&endpoint.provider_config);
	let Some(url) = settings
		.get("outcome_webhook_url")
		.and_then(Value::as_str)
		.map(str::trim)
		.filter(|value| !value.is_empty())
	else {
		return Ok(false);
	};
	validate_webhook_url(url).map_err(|error| error.to_string())?;
	validate_resolved_webhook_target(url)
		.await
		.map_err(|error| error.to_string())?;
	let source_key = settings
		.get("source_key")
		.and_then(Value::as_str)
		.map(ToOwned::to_owned);
	let payload = serde_json::json!({
		"provider": provider,
		"source_key": source_key,
		"outcomes": outcomes,
	});
	let body = serde_json::to_vec(&payload).map_err(|error| error.to_string())?;
	let client = reqwest::Client::builder()
		.timeout(Duration::from_secs(10))
		.redirect(reqwest::redirect::Policy::none())
		.build()
		.map_err(|error| error.to_string())?;
	let mut request = client
		.post(url)
		.header("content-type", "application/json")
		.header("x-reacher-receipt-id", receipt_id.to_string());
	if let Some(headers) = settings
		.get("outcome_webhook_headers")
		.and_then(Value::as_object)
	{
		for (name, value) in headers {
			if let Some(value) = value.as_str() {
				request = request.header(name, value);
			}
		}
	}
	if let Some(secret) = settings
		.get("outcome_webhook_signing_secret")
		.and_then(Value::as_str)
	{
		request = request.header(WEBHOOK_SIGNATURE_HEADER, sign_payload(secret, &body));
	}
	let response = request
		.body(body)
		.send()
		.await
		.map_err(|error| error.to_string())?;
	if !response.status().is_success() {
		return Err(format!(
			"outcome webhook returned HTTP {}",
			response.status()
		));
	}
	Ok(true)
}

fn normalize_sendgrid(endpoint: &EndpointRow, receipt_id: Uuid, body: &Value) -> Vec<OutcomeInput> {
	body.as_array()
		.into_iter()
		.flatten()
		.filter_map(|event| {
			let event_type = normalize_event_type(event.get("event")?.as_str()?)?;
			Some(make_outcome(
				endpoint,
				receipt_id,
				event.get("email")?.as_str()?.to_string(),
				event_type,
				event
					.get("sg_event_id")
					.and_then(Value::as_str)
					.map(ToOwned::to_owned),
				event
					.get("sg_message_id")
					.and_then(Value::as_str)
					.map(ToOwned::to_owned),
				timestamp_from_value(event.get("timestamp")),
				event.clone(),
			))
		})
		.collect()
}

fn normalize_mailgun(endpoint: &EndpointRow, receipt_id: Uuid, body: &Value) -> Vec<OutcomeInput> {
	let event = body.get("event-data").unwrap_or(body);
	let Some(event_type) = event
		.get("event")
		.and_then(Value::as_str)
		.and_then(normalize_event_type)
	else {
		return Vec::new();
	};
	let recipient = event
		.get("recipient")
		.and_then(Value::as_str)
		.unwrap_or_default();
	if recipient.is_empty() {
		return Vec::new();
	}
	vec![make_outcome(
		endpoint,
		receipt_id,
		recipient.to_string(),
		event_type,
		event
			.get("id")
			.and_then(Value::as_str)
			.map(ToOwned::to_owned),
		event
			.get("message")
			.and_then(|value| value.get("headers"))
			.and_then(|value| value.get("message-id"))
			.and_then(Value::as_str)
			.map(ToOwned::to_owned),
		timestamp_from_value(event.get("timestamp")),
		event.clone(),
	)]
}

fn normalize_postmark(endpoint: &EndpointRow, receipt_id: Uuid, body: &Value) -> Vec<OutcomeInput> {
	let event_name = body
		.get("RecordType")
		.or_else(|| body.get("Type"))
		.and_then(Value::as_str)
		.unwrap_or_default();
	let Some(event_type) = normalize_event_type(event_name) else {
		return Vec::new();
	};
	let email = body
		.get("Email")
		.or_else(|| body.get("Recipient"))
		.and_then(Value::as_str)
		.unwrap_or_default();
	if email.is_empty() {
		return Vec::new();
	}
	vec![make_outcome(
		endpoint,
		receipt_id,
		email.to_string(),
		event_type,
		body.get("ID")
			.and_then(Value::as_str)
			.map(ToOwned::to_owned),
		body.get("MessageID")
			.and_then(Value::as_str)
			.map(ToOwned::to_owned),
		timestamp_from_value(body.get("ReceivedAt").or_else(|| body.get("DeliveredAt"))),
		body.clone(),
	)]
}

fn normalize_ses(endpoint: &EndpointRow, receipt_id: Uuid, body: &Value) -> Vec<OutcomeInput> {
	let message = body
		.get("Message")
		.and_then(Value::as_str)
		.unwrap_or_default();
	let Ok(message): Result<Value, _> = serde_json::from_str(message) else {
		return Vec::new();
	};
	let notification = message
		.get("notificationType")
		.and_then(Value::as_str)
		.unwrap_or_default();
	let Some(event_type) = normalize_event_type(notification) else {
		return Vec::new();
	};
	let email = message
		.get("mail")
		.and_then(|value| value.get("destination"))
		.and_then(Value::as_array)
		.and_then(|value| value.first())
		.and_then(Value::as_str)
		.unwrap_or_default();
	if email.is_empty() {
		return Vec::new();
	}
	vec![make_outcome(
		endpoint,
		receipt_id,
		email.to_string(),
		event_type,
		body.get("MessageId")
			.and_then(Value::as_str)
			.map(ToOwned::to_owned),
		message
			.get("mail")
			.and_then(|value| value.get("messageId"))
			.and_then(Value::as_str)
			.map(ToOwned::to_owned),
		timestamp_from_value(message.get("mail").and_then(|value| value.get("timestamp"))),
		message,
	)]
}

async fn inbound_handler(
	provider: String,
	endpoint_id: Uuid,
	delivery_token: String,
	remote: Option<SocketAddr>,
	headers: HeaderMap,
	body: bytes::Bytes,
	pg_pool: PgPool,
) -> Result<impl warp::Reply, warp::Rejection> {
	let provider = validate_provider(&provider)?;
	let row = sqlx::query(
		"SELECT id, tenant_id, provider, label, status, delivery_token_hash, provider_config, allowed_ips, created_at, updated_at FROM v1_provider_endpoints WHERE id = $1 AND provider = $2 AND deleted_at IS NULL",
	)
	.bind(endpoint_id)
	.bind(&provider)
	.fetch_optional(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;
	let Some(row) = row else {
		return Err(ReacherResponseError::new(
			StatusCode::NOT_FOUND,
			"Provider endpoint not found",
		)
		.into());
	};
	let endpoint = row_to_endpoint(&row);
	if endpoint.status == "disabled"
		|| !memcmp::eq(
			token_hash(&delivery_token).as_bytes(),
			endpoint.delivery_token_hash.as_bytes(),
		) {
		return Err(ReacherResponseError::new(
			StatusCode::NOT_FOUND,
			"Provider endpoint not found",
		)
		.into());
	}
	if !endpoint.allowed_ips.is_empty()
		&& !remote.is_some_and(|addr| {
			endpoint
				.allowed_ips
				.iter()
				.any(|ip| ip == &addr.ip().to_string())
		}) {
		return Err(ReacherResponseError::new(
			StatusCode::FORBIDDEN,
			"Source IP is not allowlisted",
		)
		.into());
	}
	let receipt_id = Uuid::new_v4();
	let payload_hash = hex::encode(Sha256::digest(&body));
	let body_json = parse_provider_payload(&headers, &body).unwrap_or(Value::Null);
	let verification = match provider.as_str() {
		"sendgrid" => verify_sendgrid(&endpoint.provider_config, &headers, &body),
		"mailgun" => verify_mailgun(&endpoint.provider_config, &body_json),
		"ses" => verify_ses(&endpoint.provider_config, &body_json).await,
		"postmark" => Ok(()),
		_ => unreachable!(),
	};
	let validation_error = verification.err();
	let validation_status = if endpoint.status == "paused" {
		"paused"
	} else if validation_error.is_some() {
		"rejected"
	} else if body_json.is_null() {
		"malformed"
	} else {
		"accepted"
	};
	sqlx::query(
		"INSERT INTO v1_provider_outcome_receipts (id, tenant_id, endpoint_id, provider, request_headers, raw_payload, payload_sha256, validation_status, validation_error) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
	)
	.bind(receipt_id)
	.bind(endpoint.tenant_id)
	.bind(endpoint.id)
	.bind(&provider)
	.bind(safe_headers(&headers))
	.bind(&body_json)
	.bind(payload_hash)
	.bind(validation_status)
	.bind(&validation_error)
	.execute(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;
	if endpoint.status == "paused" {
		return Ok(warp::reply::with_status(
			warp::reply::json(&InboundOutcomeResponse {
				receipt_id,
				provider,
				accepted: 0,
				duplicates: 0,
				unmatched: 0,
				rejected: 0,
			}),
			StatusCode::ACCEPTED,
		));
	}
	if let Some(error) = validation_error {
		return Err(ReacherResponseError::new(StatusCode::UNAUTHORIZED, error).into());
	}
	if body_json.is_null() {
		return Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"Provider payload must be valid JSON",
		)
		.into());
	}
	let outcomes = match provider.as_str() {
		"sendgrid" => normalize_sendgrid(&endpoint, receipt_id, &body_json),
		"mailgun" => normalize_mailgun(&endpoint, receipt_id, &body_json),
		"ses" => normalize_ses(&endpoint, receipt_id, &body_json),
		"postmark" => normalize_postmark(&endpoint, receipt_id, &body_json),
		_ => unreachable!(),
	};
	let submitted = match provider.as_str() {
		"sendgrid" => body_json.as_array().map_or(0, |events| events.len() as i64),
		_ => 1,
	};
	if outcomes.is_empty() {
		sqlx::query("UPDATE v1_provider_outcome_receipts SET normalized_count = 0 WHERE id = $1")
			.bind(receipt_id)
			.execute(&pg_pool)
			.await
			.map_err(ReacherResponseError::from)?;
		return Ok(warp::reply::with_status(
			warp::reply::json(&InboundOutcomeResponse {
				receipt_id,
				provider,
				accepted: 0,
				duplicates: 0,
				unmatched: 0,
				rejected: submitted,
			}),
			StatusCode::OK,
		));
	}
	let result = ingest_outcomes(
		&pg_pool,
		endpoint.tenant_id,
		OutcomeIngestRequest {
			provider: provider.clone(),
			outcomes: outcomes.clone(),
			source_key: None,
		},
	)
	.await?;
	let (forward_status, forward_attempts, forward_error, forwarded_at) =
		match forward_outcomes(&endpoint, receipt_id, &provider, &outcomes).await {
			Ok(true) => ("delivered", 1, None, Some(Utc::now())),
			Ok(false) => ("not_configured", 0, None, None),
			Err(error) => {
				warn!(
					target: LOG_TARGET,
					receipt_id = %receipt_id,
					endpoint_id = %endpoint.id,
					error = %error,
					"Provider outcome forwarding failed; receipt remains durable"
				);
				("failed", 1, Some(error), None)
			}
		};
	sqlx::query(
		"UPDATE v1_provider_outcome_receipts SET normalized_count = $2, forward_status = $3, forward_attempts = $4, forward_error = $5, forwarded_at = $6 WHERE id = $1",
	)
		.bind(receipt_id)
		.bind(result.ingested as i32)
		.bind(forward_status)
		.bind(forward_attempts)
		.bind(forward_error)
		.bind(forwarded_at)
		.execute(&pg_pool)
		.await
		.map_err(ReacherResponseError::from)?;
	Ok(warp::reply::with_status(
		warp::reply::json(&InboundOutcomeResponse {
			receipt_id,
			provider,
			accepted: result.ingested,
			duplicates: result.ignored,
			unmatched: result.ingested,
			rejected: submitted - result.ingested - result.ignored,
		}),
		StatusCode::OK,
	))
}

#[utoipa::path(
	get,
	path = "/v1/provider-endpoints",
	tag = "Outcomes",
	responses((status = 200, description = "Configured provider endpoints", body = ProviderEndpointListResponse))
)]
pub fn v1_list_provider_endpoints(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "provider-endpoints")
		.and(warp::get())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and_then(list_handler)
		.with(warp::log(LOG_TARGET))
}

#[utoipa::path(
	post,
	path = "/v1/provider-endpoints",
	tag = "Outcomes",
	request_body = CreateProviderEndpointInput,
	responses(
		(status = 201, description = "Provider endpoint created; delivery token is returned once", body = ProviderEndpointView),
		(status = 400, description = "Invalid provider endpoint")
	)
)]
pub fn v1_create_provider_endpoint(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "provider-endpoints")
		.and(warp::post())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::body::json())
		.and_then(create_handler)
		.with(warp::log(LOG_TARGET))
}

#[utoipa::path(
	patch,
	path = "/v1/provider-endpoints/{endpoint_id}",
	tag = "Outcomes",
	params(("endpoint_id" = Uuid, Path, description = "Provider endpoint identifier")),
	request_body = UpdateProviderEndpointInput,
	responses(
		(status = 200, description = "Provider endpoint updated", body = ProviderEndpointView),
		(status = 404, description = "Provider endpoint not found")
	)
)]
pub fn v1_update_provider_endpoint(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "provider-endpoints" / Uuid)
		.and(warp::patch())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and(warp::body::json())
		.and_then(update_handler)
		.with(warp::log(LOG_TARGET))
}

#[utoipa::path(
	delete,
	path = "/v1/provider-endpoints/{endpoint_id}",
	tag = "Outcomes",
	params(("endpoint_id" = Uuid, Path, description = "Provider endpoint identifier")),
	responses(
		(status = 200, description = "Provider endpoint disabled and deleted", body = ProviderDeleteResponse),
		(status = 404, description = "Provider endpoint not found")
	)
)]
pub fn v1_delete_provider_endpoint(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "provider-endpoints" / Uuid)
		.and(warp::delete())
		.and(resolve_tenant(Arc::clone(&config)))
		.and(with_worker_db(config))
		.and_then(delete_handler)
		.with(warp::log(LOG_TARGET))
}

#[utoipa::path(
	post,
	path = "/v1/inbound/providers/{provider}/{endpoint_id}/{delivery_token}",
	tag = "Outcomes",
	params(
		("provider" = String, Path, description = "sendgrid, ses, mailgun, or postmark"),
		("endpoint_id" = Uuid, Path, description = "Provider endpoint identifier"),
		("delivery_token" = String, Path, description = "Secret endpoint delivery token")
	),
	request_body(content = Value, content_type = "application/json"),
	responses(
		(status = 200, description = "Provider events authenticated and ingested", body = InboundOutcomeResponse),
		(status = 202, description = "Receipt retained while endpoint is paused", body = InboundOutcomeResponse),
		(status = 401, description = "Provider signature rejected")
	)
)]
pub fn v1_ingest_provider_outcomes(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "inbound" / "providers" / String / Uuid / String)
		.and(warp::post())
		.and(warp::addr::remote())
		.and(warp::header::headers_cloned())
		.and(warp::body::content_length_limit(10 * 1024 * 1024))
		.and(warp::body::bytes())
		.and(with_worker_db(config))
		.and_then(inbound_handler)
		.with(warp::log(LOG_TARGET))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn normalizes_provider_event_names() {
		assert_eq!(normalize_event_type("bounce").as_deref(), Some("bounced"));
		assert_eq!(
			normalize_event_type("spamreport").as_deref(),
			Some("complained")
		);
		assert!(normalize_event_type("processed").is_none());
	}

	#[test]
	fn validates_provider_and_status_values() {
		assert_eq!(validate_provider("SendGrid").unwrap(), "sendgrid");
		assert!(validate_provider("unknown").is_err());
		assert!(validate_status("paused").is_ok());
		assert!(validate_status("deleted").is_err());
	}

	#[test]
	fn parses_mailgun_form_payloads() {
		let mut headers = HeaderMap::new();
		headers.insert(
			"content-type",
			"application/x-www-form-urlencoded"
				.parse()
				.expect("valid content type"),
		);
		let fields = [
			("timestamp", "1700000000"),
			("token", "mailgun-token"),
			("signature", "0123456789abcdef"),
			(
				"event-data",
				r#"{"event":"delivered","recipient":"person@example.com"}"#,
			),
		];
		let encoded = serde_urlencoded::to_string(fields).expect("form payload");
		let parsed = parse_provider_payload(&headers, encoded.as_bytes()).expect("parsed payload");

		assert_eq!(parsed["signature"]["token"].as_str(), Some("mailgun-token"));
		assert_eq!(
			parsed["event-data"]["recipient"].as_str(),
			Some("person@example.com")
		);
	}
}
