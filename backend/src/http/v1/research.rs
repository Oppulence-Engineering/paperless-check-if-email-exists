use crate::config::BackendConfig;
use crate::finder::require_tenant_id;
use crate::http::idempotency::{
	check_idempotency_key, complete_idempotency_key, fail_idempotency_key, hash_request_body,
	IdempotencyCheck,
};
use crate::http::v0::check_email::post::with_config;
use crate::http::{check_scope, resolve_tenant, ReacherResponseError};
use crate::research::{ResearchError, ResearchWorkspace};
use crate::tenant::context::{scope, TenantContext};
use base64::{engine::general_purpose::STANDARD, Engine};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{collections::HashSet, sync::Arc};
use warp::{http::StatusCode, Filter};

const COLUMN_KEY: &str = "prospect_contact";

#[derive(Deserialize, Serialize, utoipa::ToSchema)]
#[serde(deny_unknown_fields)]
pub struct CreateResearchRequest {
	pub name: String,
	pub target_role: String,
	pub import_content: String,
	pub import_format: ResearchImportFormat,
	pub max_candidates: u8,
	pub max_verifications: u16,
	pub idempotency_key: String,
}

#[derive(Deserialize, Serialize, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ResearchImportFormat {
	Csv,
	Json,
}

#[derive(Deserialize, Serialize, utoipa::ToSchema)]
pub struct ResearchCreated {
	pub id: String,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct ResearchSummary {
	pub id: String,
	pub name: String,
	pub created_at: Option<String>,
	pub imported_rows: u32,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct ResearchList {
	pub sessions: Vec<ResearchSummary>,
}

#[derive(Deserialize, Serialize, utoipa::ToSchema)]
pub struct ResearchMailbox {
	pub provider: String,
	pub result: String,
	pub action: String,
	pub catch_all: bool,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct ResearchCandidate {
	pub name: String,
	pub title: String,
	pub email: String,
	pub email_source: String,
	pub evidence_url: Option<String>,
	pub observed_at: Option<String>,
	pub person_status: String,
	pub employment_status: String,
	pub role_status: String,
	pub mailbox: Option<ResearchMailbox>,
	pub verification_error: Option<String>,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct ResearchRow {
	pub row_key: String,
	pub company_name: String,
	pub company_domain: String,
	pub status: String,
	pub reason: String,
	pub candidates: Vec<ResearchCandidate>,
	pub checked_pages: Vec<String>,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct ResearchDetail {
	pub session: ResearchSummary,
	pub status: String,
	pub target_role: String,
	pub max_candidates: u8,
	pub max_verifications: u16,
	pub verification_requests: u32,
	pub rows: Vec<ResearchRow>,
}

#[derive(Deserialize, utoipa::ToSchema)]
#[serde(deny_unknown_fields)]
pub struct ResearchExportRequest {
	pub row_keys: Vec<String>,
	pub mode: ResearchExportMode,
}

#[derive(Deserialize, utoipa::ToSchema, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ResearchExportMode {
	Approved,
	Review,
}

#[derive(Serialize, utoipa::ToSchema)]
pub struct ResearchExportResponse {
	pub filename: String,
	pub csv: String,
	pub exported_rows: u32,
	pub excluded_row_keys: Vec<String>,
}

fn reject(status: StatusCode, message: &'static str) -> warp::Rejection {
	ReacherResponseError::new(status, message).into()
}

fn upstream(error: ResearchError) -> warp::Rejection {
	reject(error.status, error.message)
}

fn validate(input: &CreateResearchRequest) -> Result<(), warp::Rejection> {
	if input.name.trim().is_empty()
		|| input.name.len() > 100
		|| input.target_role.trim().is_empty()
		|| input.target_role.len() > 120
		|| input.import_content.trim().is_empty()
		|| input.import_content.len() > 1_048_576
		|| !(1..=3).contains(&input.max_candidates)
		|| !(1..=30).contains(&input.max_verifications)
		|| !uuid::Uuid::parse_str(&input.idempotency_key).is_ok_and(|id| !id.is_nil())
	{
		return Err(reject(StatusCode::BAD_REQUEST,"Provide a name, target role, CSV or JSON import, and explicit limits of 1–3 candidates per company and 1–30 verifications."));
	}
	Ok(())
}

async fn create(
	tenant: TenantContext,
	config: Arc<BackendConfig>,
	input: CreateResearchRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant, scope::FIND)?;
	validate(&input)?;
	let tenant_id = require_tenant_id(tenant.tenant_id)?;
	let pool = config.get_pg_pool().ok_or_else(|| {
		reject(
			StatusCode::SERVICE_UNAVAILABLE,
			"Research storage is unavailable.",
		)
	})?;
	let hash = hash_request_body(
		&serde_json::to_vec(&input)
			.map_err(|_| reject(StatusCode::BAD_REQUEST, "Invalid research request."))?,
	);
	let record_id = match check_idempotency_key(
		&pool,
		&tenant_id.to_string(),
		&input.idempotency_key,
		"/v1/research",
		&hash,
		&uuid::Uuid::new_v4().to_string(),
	)
	.await?
	{
		IdempotencyCheck::New { record_id } => record_id,
		IdempotencyCheck::Cached(cached) => {
			let response: ResearchCreated = serde_json::from_slice(&cached.body).map_err(|_| {
				reject(
					StatusCode::BAD_GATEWAY,
					"Stored research response is unavailable.",
				)
			})?;
			return Ok(warp::reply::json(&response));
		}
		IdempotencyCheck::InProgress => {
			return Err(reject(
				StatusCode::CONFLICT,
				"Research creation is still in progress. Retry with the same request key.",
			))
		}
		_ => {
			return Err(reject(
				StatusCode::CONFLICT,
				"This request key was already used for a different request.",
			))
		}
	};
	let result = create_in_go(&config, &tenant, &input, &hex::encode(hash)).await;
	match result {
		Ok(response) => {
			let raw = serde_json::to_vec(&response).map_err(|_| {
				reject(
					StatusCode::BAD_GATEWAY,
					"Could not encode research response.",
				)
			})?;
			complete_idempotency_key(&pool, record_id, 200, &raw).await?;
			Ok(warp::reply::json(&response))
		}
		Err(error) => {
			fail_idempotency_key(&pool, record_id).await?;
			Err(error)
		}
	}
}

async fn create_in_go(
	config: &BackendConfig,
	tenant: &TenantContext,
	input: &CreateResearchRequest,
	hash: &str,
) -> Result<ResearchCreated, warp::Rejection> {
	let workspace = config.research.workspace(tenant).await.map_err(upstream)?;
	let key = format!("prospect:{}:{hash}", input.idempotency_key);
	let created = config
		.research
		.create_session(
			&workspace,
			json!({
				"prompt":input.name.trim(), "importedFile":STANDARD.encode(input.import_content.as_bytes()),
				"importFormat":input.import_format,"requestedSources":["user_import"],"maxRecords":10,
				"includeContactData":false,"dryRun":true,"idempotencyKey":key
			}),
		)
		.await
		.map_err(upstream)?;
	let id = text(&created["session"], "id");
	if created["session"]["status"].as_str() == Some("failed") {
		return Err(reject(
			StatusCode::BAD_GATEWAY,
			"The import did not complete. Start a new research run after correcting the import.",
		));
	}
	let session_id = valid_id(&id)?;
	let column_config = json!({"target_role":input.target_role.trim(),"max_candidates":input.max_candidates,"max_verifications":input.max_verifications});
	config.research.add_column(&workspace,session_id,json!({
		"columnKey":COLUMN_KEY,"label":"Contact research","executorKind":COLUMN_KEY,
		"configJson":STANDARD.encode(column_config.to_string()),"dataType":"json","sourceType":COLUMN_KEY,
		"runImmediately":true,"idempotencyKey":key
	})).await.map_err(upstream)?;
	Ok(ResearchCreated { id })
}

async fn list(
	tenant: TenantContext,
	config: Arc<BackendConfig>,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant, scope::FIND)?;
	let workspace = config.research.workspace(&tenant).await.map_err(upstream)?;
	let result = config
		.research
		.list_sessions(&workspace)
		.await
		.map_err(upstream)?;
	let sessions = items(&result, "sessions")
		.iter()
		.filter(|s| text(s, "requestUuid").starts_with("prospect:"))
		.map(summary)
		.collect();
	Ok(warp::reply::json(&ResearchList { sessions }))
}

async fn detail(
	id: u64,
	tenant: TenantContext,
	config: Arc<BackendConfig>,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant, scope::FIND)?;
	let workspace = config.research.workspace(&tenant).await.map_err(upstream)?;
	Ok(warp::reply::json(
		&load_detail(&config, &workspace, id).await?,
	))
}

async fn load_detail(
	config: &BackendConfig,
	workspace: &ResearchWorkspace,
	id: u64,
) -> Result<ResearchDetail, warp::Rejection> {
	let (rows, columns) = tokio::try_join!(
		config.research.rows(workspace, id),
		config.research.columns(workspace, id)
	)
	.map_err(upstream)?;
	normalize_detail(&rows, &columns)
}

fn normalize_detail(raw: &Value, columns: &Value) -> Result<ResearchDetail, warp::Rejection> {
	let column = items(columns, "columns")
		.iter()
		.find(|c| text(c, "columnKey") == COLUMN_KEY && text(c, "executorKind") == COLUMN_KEY)
		.ok_or_else(|| reject(StatusCode::NOT_FOUND, "Contact research not found."))?;
	let config = decode_json(&column["configJson"]).ok_or_else(|| {
		reject(
			StatusCode::BAD_GATEWAY,
			"Research settings are unavailable.",
		)
	})?;
	let column_id = text(column, "id");
	let mut rows = Vec::new();
	for row in items(raw, "rows") {
		let cells = items(row, "cells");
		let cell = cells
			.iter()
			.find(|c| text(c, "columnKey") == COLUMN_KEY && text(c, "provider") == COLUMN_KEY);
		let outcome = cell.and_then(|c| decode_json(&c["evidenceJson"]));
		let run = items(raw, "cellRuns")
			.iter()
			.filter(|r| {
				text(r, "researchColumnDefinitionId") == column_id
					&& text(r, "leadId") == text(row, "leadId")
			})
			.max_by_key(|r| text(r, "id").parse::<u64>().unwrap_or(0));
		let field = |key: &str| {
			cells
				.iter()
				.find(|c| text(c, "columnKey") == key)
				.map(|c| text(c, "value"))
				.unwrap_or_default()
		};
		let mut result = ResearchRow {
			row_key: text(row, "rowKey"),
			company_name: text(row, "companyName"),
			company_domain: field("company_domain"),
			status: "queued".into(),
			reason: "research_pending".into(),
			candidates: vec![],
			checked_pages: vec![],
		};
		if let Some(outcome) = outcome {
			result.status = match outcome["status"].as_str() {
				Some("ready") => "ready",
				Some("no_match") => "no_match",
				_ => "needs_review",
			}
			.into();
			result.reason = text(&outcome, "reason");
			result.company_domain = text(&outcome, "domain");
			result.candidates = items(&outcome, "candidates")
				.iter()
				.map(candidate)
				.collect();
			result.checked_pages = items(&outcome, "pages")
				.iter()
				.filter(|p| text(p, "Status") == "fetched")
				.map(|p| text(p, "URL"))
				.collect();
			if result.status == "ready" && !result.candidates.iter().any(ready_candidate) {
				result.status = "needs_review".into();
				result.reason = "incomplete_verification_evidence".into();
			}
		} else if let Some(run) = run {
			match text(run, "status").as_str() {
				"failed" | "cancelled" | "completed" => {
					result.status = "needs_review".into();
					result.reason = "research_did_not_complete".into();
				}
				"running" => result.status = "running".into(),
				_ => {}
			}
		}
		rows.push(result);
	}
	let status = if rows
		.iter()
		.any(|r| r.status == "running" || r.status == "queued")
	{
		"running"
	} else {
		"completed"
	};
	Ok(ResearchDetail {
		session: summary(&raw["session"]),
		status: status.into(),
		target_role: text(&config, "target_role"),
		max_candidates: config["max_candidates"].as_u64().unwrap_or(0) as u8,
		max_verifications: config["max_verifications"].as_u64().unwrap_or(0) as u16,
		verification_requests: raw["verificationRequests"].as_u64().unwrap_or(0) as u32,
		rows,
	})
}

fn candidate(raw: &Value) -> ResearchCandidate {
	let verification = &raw["verification"];
	let external = &verification["external_verification"];
	let mailbox = if external["service"].as_str() == Some("millionverifier") {
		Some(ResearchMailbox {
			provider: "millionverifier".into(),
			result: text(external, "result"),
			action: text(&verification["recommendation"], "action"),
			catch_all: verification["is_catch_all"].as_bool().unwrap_or(false),
		})
	} else {
		None
	};
	ResearchCandidate {
		name: text(raw, "name"),
		title: text(raw, "title"),
		email: text(raw, "email"),
		email_source: text(raw, "email_source"),
		evidence_url: optional_text(raw, "evidence_url"),
		observed_at: optional_text(raw, "observed_at"),
		person_status: text(raw, "person_status"),
		employment_status: text(raw, "employment_status"),
		role_status: text(raw, "role_status"),
		mailbox,
		verification_error: optional_text(raw, "verification_error"),
	}
}

fn ready_candidate(candidate: &ResearchCandidate) -> bool {
	candidate.email_source == "published"
		&& candidate.person_status == "published"
		&& candidate.employment_status == "published_on_company_site"
		&& candidate.role_status == "title_matches"
		&& candidate.mailbox.as_ref().is_some_and(|m| {
			m.provider == "millionverifier"
				&& m.result == "ok"
				&& m.action == "send"
				&& !m.catch_all
		})
}

fn summary(raw: &Value) -> ResearchSummary {
	ResearchSummary {
		id: text(raw, "id"),
		name: text(raw, "prompt"),
		created_at: optional_text(raw, "createdAt"),
		imported_rows: raw["rowsTotal"].as_u64().unwrap_or(0) as u32,
	}
}
fn text(raw: &Value, key: &str) -> String {
	raw[key].as_str().unwrap_or("").to_owned()
}
fn optional_text(raw: &Value, key: &str) -> Option<String> {
	raw[key]
		.as_str()
		.filter(|s| !s.is_empty())
		.map(str::to_owned)
}
fn items<'a>(raw: &'a Value, key: &str) -> &'a [Value] {
	raw[key].as_array().map(Vec::as_slice).unwrap_or(&[])
}
fn decode_json(raw: &Value) -> Option<Value> {
	serde_json::from_slice(&STANDARD.decode(raw.as_str()?).ok()?).ok()
}
fn valid_id(id: &str) -> Result<u64, warp::Rejection> {
	id.parse::<u64>().ok().filter(|i| *i > 0).ok_or_else(|| {
		reject(
			StatusCode::BAD_GATEWAY,
			"Research identifier is unavailable.",
		)
	})
}

async fn export(
	id: u64,
	tenant: TenantContext,
	config: Arc<BackendConfig>,
	input: ResearchExportRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
	check_scope(&tenant, scope::FIND)?;
	let tenant_id = require_tenant_id(tenant.tenant_id)?;
	let workspace = config.research.workspace(&tenant).await.map_err(upstream)?;
	let detail = load_detail(&config, &workspace, id).await?;
	let selected: HashSet<_> = input.row_keys.iter().collect();
	if selected.is_empty()
		|| selected.len() > 100
		|| selected
			.iter()
			.any(|key| !detail.rows.iter().any(|row| &row.row_key == *key))
	{
		return Err(reject(
			StatusCode::BAD_REQUEST,
			"Select existing rows from this research session.",
		));
	}
	let mut writer = csv::Writer::from_writer(Vec::new());
	let csv_error = |_| {
		reject(
			StatusCode::INTERNAL_SERVER_ERROR,
			"Could not prepare research CSV.",
		)
	};
	writer
		.write_record([
			"company",
			"domain",
			"name",
			"title",
			"email",
			"status",
			"reason",
			"mailbox_result",
			"sending_action",
			"person_status",
			"employment_status",
			"role_status",
			"email_source",
			"source_url",
			"observed_at",
		])
		.map_err(csv_error)?;
	let mut excluded = Vec::new();
	let mut count = 0;
	for row in &detail.rows {
		if !selected.contains(&row.row_key) {
			continue;
		}
		let candidates: Vec<Option<&ResearchCandidate>> =
			if input.mode == ResearchExportMode::Approved {
				let mut eligible = None;
				for candidate in row
					.candidates
					.iter()
					.filter(|c| row.status == "ready" && ready_candidate(c))
				{
					if current_policy_allows(&config, tenant_id, &candidate.email).await? {
						eligible = Some(candidate);
						break;
					}
				}
				if let Some(candidate) = eligible {
					vec![Some(candidate)]
				} else {
					excluded.push(row.row_key.clone());
					continue;
				}
			} else if row.candidates.is_empty() {
				vec![None]
			} else {
				row.candidates.iter().map(Some).collect()
			};
		for candidate in candidates {
			let mailbox = candidate.and_then(|c| c.mailbox.as_ref());
			writer
				.write_record([
					row.company_name.as_str(),
					row.company_domain.as_str(),
					candidate.map(|c| c.name.as_str()).unwrap_or(""),
					candidate.map(|c| c.title.as_str()).unwrap_or(""),
					candidate.map(|c| c.email.as_str()).unwrap_or(""),
					row.status.as_str(),
					row.reason.as_str(),
					mailbox.map(|m| m.result.as_str()).unwrap_or(""),
					mailbox.map(|m| m.action.as_str()).unwrap_or(""),
					candidate.map(|c| c.person_status.as_str()).unwrap_or(""),
					candidate
						.map(|c| c.employment_status.as_str())
						.unwrap_or(""),
					candidate.map(|c| c.role_status.as_str()).unwrap_or(""),
					candidate.map(|c| c.email_source.as_str()).unwrap_or(""),
					candidate
						.and_then(|c| c.evidence_url.as_deref())
						.unwrap_or(""),
					candidate
						.and_then(|c| c.observed_at.as_deref())
						.unwrap_or(""),
				])
				.map_err(csv_error)?;
			count += 1;
		}
	}
	let raw = writer.into_inner().map_err(|_| {
		reject(
			StatusCode::INTERNAL_SERVER_ERROR,
			"Could not finish research CSV.",
		)
	})?;
	let safe = crate::http::csv_shared::spreadsheet_safe_csv(raw).map_err(csv_error)?;
	let csv = String::from_utf8(safe).map_err(|_| {
		reject(
			StatusCode::INTERNAL_SERVER_ERROR,
			"Could not encode research CSV.",
		)
	})?;
	Ok(warp::reply::json(&ResearchExportResponse {
		filename: format!(
			"research-{id}-{}.csv",
			if input.mode == ResearchExportMode::Approved {
				"approved"
			} else {
				"review"
			}
		),
		csv,
		exported_rows: count,
		excluded_row_keys: excluded,
	}))
}

async fn current_policy_allows(
	config: &BackendConfig,
	tenant_id: uuid::Uuid,
	email: &str,
) -> Result<bool, warp::Rejection> {
	let pool = config.get_pg_pool().ok_or_else(|| {
		reject(
			StatusCode::SERVICE_UNAVAILABLE,
			"Verification history is unavailable.",
		)
	})?;
	let canonical = crate::suppression::suppression_key(email);
	let latest=sqlx::query_as::<_,(Option<Value>,Option<chrono::DateTime<chrono::Utc>>,bool)>("SELECT result, completed_at, error IS NULL FROM v1_task_result WHERE tenant_id=$1 AND canonical_email=$2 ORDER BY created_at DESC, id DESC LIMIT 1")
		.bind(tenant_id).bind(canonical).fetch_optional(&pool).await.map_err(ReacherResponseError::from)?;
	let Some((Some(raw), Some(completed_at), true)) = latest else {
		return Ok(false);
	};
	let Ok(score) = serde_json::from_value::<crate::scoring::EmailScore>(raw["score"].clone())
	else {
		return Ok(false);
	};
	if !text(&raw, "input").eq_ignore_ascii_case(email)
		|| raw["external_verification"]["service"].as_str() != Some("millionverifier")
		|| raw["external_verification"]["result"].as_str() != Some("ok")
		|| raw["recommendation"]["action"].as_str() != Some("send")
	{
		return Ok(false);
	}
	// New negative feedback requires another review even if the older mailbox result was safe.
	let new_negative_feedback: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM verification_outcomes WHERE tenant_id=$1 AND canonical_email=$2 AND occurred_at > $3 AND occurred_at <= NOW() AND outcome_type IN ('hard_bounce', 'complaint', 'unsubscribe'))")
		.bind(tenant_id).bind(crate::suppression::suppression_key(email)).bind(completed_at)
		.fetch_one(&pool).await.map_err(ReacherResponseError::from)?;
	if new_negative_feedback {
		return Ok(false);
	}
	let configured: String =
		sqlx::query_scalar("SELECT default_policy_mode FROM tenants WHERE id=$1")
			.bind(tenant_id)
			.fetch_one(&pool)
			.await
			.map_err(ReacherResponseError::from)?;
	let policy_mode =
		serde_json::from_value::<crate::decision::types::PolicyMode>(Value::String(configured))
			.map_err(|_| {
				reject(
					StatusCode::SERVICE_UNAVAILABLE,
					"Sending policy is unavailable.",
				)
			})?;
	let custom_profile = if policy_mode == crate::decision::types::PolicyMode::Custom {
		crate::decision::custom_policy::load_active_profile(&pool, tenant_id)
			.await
			.map_err(|_| {
				reject(
					StatusCode::SERVICE_UNAVAILABLE,
					"Custom sending policy is unavailable.",
				)
			})?
	} else {
		None
	};
	let active_suppression = crate::suppression::has_active_suppression(&pool, tenant_id, email)
		.await
		.map_err(ReacherResponseError::from)?;
	let (recommendation, _) =
		crate::decision::engine::evaluate(&crate::decision::engine::DecisionInput {
			score: &score,
			completed_at,
			evaluated_at: chrono::Utc::now(),
			policy_mode,
			policy_profile_key: custom_profile.as_ref().map(|(key, _)| key.clone()),
			custom_policy: custom_profile.as_ref().map(|(_, rules)| rules),
			domain_suggestion: None,
			suggested_email: None,
			bounce_risk: None,
			active_suppression,
			previous_hard_bounce: false,
		});
	Ok(recommendation.action == crate::decision::types::RecommendedAction::Send)
}

#[utoipa::path(post,path="/v1/research",tag="v1",request_body=CreateResearchRequest,responses((status=200,description="Research queued",body=ResearchCreated)))]
pub fn v1_create_research(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "research")
		.and(warp::post())
		.and(resolve_tenant(config.clone()))
		.and(with_config(config))
		.and(warp::body::content_length_limit(1_100_000))
		.and(warp::body::json())
		.and_then(create)
}

#[utoipa::path(get,path="/v1/research",tag="v1",responses((status=200,description="Research sessions",body=ResearchList)))]
pub fn v1_list_research(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "research")
		.and(warp::get())
		.and(resolve_tenant(config.clone()))
		.and(with_config(config))
		.and_then(list)
}

#[utoipa::path(get,path="/v1/research/{session_id}",tag="v1",params(("session_id"=u64,Path,description="Research session")),responses((status=200,description="Research evidence and progress",body=ResearchDetail)))]
pub fn v1_get_research(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "research" / u64)
		.and(warp::get())
		.and(resolve_tenant(config.clone()))
		.and(with_config(config))
		.and_then(detail)
}

#[utoipa::path(post,path="/v1/research/{session_id}/export",tag="v1",params(("session_id"=u64,Path,description="Research session")),request_body=ResearchExportRequest,responses((status=200,description="Selected evidence or currently approved contacts as CSV",body=ResearchExportResponse)))]
pub fn v1_export_research(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	warp::path!("v1" / "research" / u64 / "export")
		.and(warp::post())
		.and(resolve_tenant(config.clone()))
		.and(with_config(config))
		.and(warp::body::content_length_limit(32_768))
		.and(warp::body::json())
		.and_then(export)
}

#[cfg(test)]
mod tests {
	use super::*;

	fn fixture(provider: &str, action: &str) -> (Value, Value) {
		let outcome = json!({"status":"ready","reason":"matched","domain":"acme.com","pages":[{"URL":"https://acme.com/team","Status":"fetched"}],"candidates":[{"name":"Jane Doe","title":"CEO","email":"jane@acme.com","email_source":"published","person_status":"published","employment_status":"published_on_company_site","role_status":"title_matches","evidence_url":"https://acme.com/team","verification":{"external_verification":{"service":"millionverifier","result":"ok"},"recommendation":{"action":action},"is_catch_all":false}}]});
		let rows = json!({"session":{"id":"42","prompt":"Pilot","rowsTotal":1},"verificationRequests":1,"rows":[{"rowKey":"import:123","leadId":"7","companyName":"Acme","cells":[{"columnKey":COLUMN_KEY,"provider":provider,"evidenceJson":STANDARD.encode(outcome.to_string())}]}],"cellRuns":[{"id":"3","leadId":"7","researchColumnDefinitionId":"9","status":"completed"}]});
		let columns = json!({"columns":[{"id":"9","columnKey":COLUMN_KEY,"executorKind":COLUMN_KEY,"configJson":STANDARD.encode(json!({"target_role":"CEO","max_candidates":3,"max_verifications":5}).to_string())}]});
		(rows, columns)
	}

	#[test]
	fn imported_cells_cannot_forge_completed_research() {
		let (raw, columns) = fixture("user_import", "send");
		let detail = normalize_detail(&raw, &columns).unwrap();
		assert_eq!(detail.rows[0].status, "needs_review");
		assert!(detail.rows[0].candidates.is_empty());
	}

	#[test]
	fn evidence_keeps_person_and_mailbox_claims_separate() {
		let (raw, columns) = fixture(COLUMN_KEY, "send");
		let detail = normalize_detail(&raw, &columns).unwrap();
		assert_eq!(detail.rows[0].status, "ready");
		assert_eq!(detail.verification_requests, 1);
		assert_eq!(detail.max_verifications, 5);
		assert_eq!(detail.rows[0].checked_pages, vec!["https://acme.com/team"]);
		let encoded = serde_json::to_string(&detail).unwrap();
		assert!(!encoded.contains("workspaceApiKey"));
		assert!(!encoded.contains("can_connect_smtp"));
		for action in ["suppress", "drop", "review", ""] {
			let (raw, columns) = fixture(COLUMN_KEY, action);
			assert_eq!(
				normalize_detail(&raw, &columns).unwrap().rows[0].status,
				"needs_review"
			);
		}
	}

	#[test]
	fn request_rejects_unbounded_spend_and_supplied_authority() {
		let raw = json!({"name":"Pilot","target_role":"CEO","import_content":"company,domain\nAcme,acme.com","import_format":"csv","max_candidates":3,"max_verifications":5,"idempotency_key":uuid::Uuid::new_v4().to_string()});
		let mut input: CreateResearchRequest = serde_json::from_value(raw.clone()).unwrap();
		assert!(validate(&input).is_ok());
		input.max_verifications = 0;
		assert!(validate(&input).is_err());
		input.max_verifications = 31;
		assert!(validate(&input).is_err());
		let mut forged = raw;
		forged["workspace_id"] = json!(999);
		assert!(serde_json::from_value::<CreateResearchRequest>(forged).is_err());
	}

	#[tokio::test]
	async fn creation_uses_import_only_plan_then_one_bounded_column() {
		use std::sync::Mutex;
		let requests = Arc::new(Mutex::new(Vec::new()));
		let seen = requests.clone();
		let mock=warp::path::full().and(warp::body::json()).map(move|path:warp::path::FullPath,body:Value|{
			seen.lock().unwrap().push((path.as_str().to_owned(),body.clone()));
			warp::reply::json(&if path.as_str().ends_with("/internal/verifier-workspace") {json!({"organizationId":"1","tenantId":"2","accountId":"3","workspaceId":"4","workspaceApiKey":"private"})}else if path.as_str().ends_with("/columns"){json!({"column":{"id":"9"}})}else{json!({"session":{"id":"42","status":"planned"}})})
		});
		let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
		let address = listener.local_addr().unwrap();
		let server = tokio::spawn(warp::serve(mock).incoming(listener).run());
		let mut config = BackendConfig::empty();
		config.research = serde_json::from_value(
			json!({"url":format!("http://{address}"),"bootstrap_api_key":"bootstrap-private"}),
		)
		.unwrap();
		let mut tenant =
			TenantContext::legacy(crate::config::ThrottleConfig::new_without_throttle());
		tenant.is_legacy = false;
		tenant.tenant_id = Some(uuid::Uuid::new_v4());
		let input:CreateResearchRequest=serde_json::from_value(json!({"name":"Pilot","target_role":"CEO","import_content":"Company,Domain\nAcme,acme.com","import_format":"csv","max_candidates":2,"max_verifications":4,"idempotency_key":uuid::Uuid::new_v4().to_string()})).unwrap();
		let result = create_in_go(&config, &tenant, &input, "stablehash")
			.await
			.unwrap();
		assert_eq!(result.id, "42");
		let calls = requests.lock().unwrap();
		assert_eq!(calls.len(), 3);
		assert_eq!(calls[1].1["requestedSources"], json!(["user_import"]));
		assert_eq!(calls[1].1["dryRun"], true);
		assert_eq!(calls[2].1["runImmediately"], true);
		assert_eq!(calls[2].1["workspaceId"], "4");
		assert_eq!(calls[1].1["idempotencyKey"], calls[2].1["idempotencyKey"]);
		let limits = decode_json(&calls[2].1["configJson"]).unwrap();
		assert_eq!(limits["max_verifications"], 4);
		server.abort();
	}
}
