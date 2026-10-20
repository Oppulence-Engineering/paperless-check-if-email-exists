use crate::config::BackendConfig;
use crate::http::{check_admin_header, ReacherResponseError};
use crate::tenant::auth::generate_api_key;
use crate::tenant::context::scope;
use crate::tenant::jwt::plan_allowance;
use check_if_email_exists::{check_email, LOG_TARGET};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use std::sync::Arc;
use tracing::info;
use uuid::Uuid;
use warp::http::StatusCode;
use warp::Filter;

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct OnboardRequest {
	/// Email address to verify.
	pub email_to_verify: String,
	/// Display name for the new tenant.
	pub tenant_name: String,
	/// Contact email for the tenant account (billing, alerts).
	pub contact_email: String,
	/// URL-safe slug (auto-generated from tenant_name if omitted).
	pub slug: Option<String>,
	/// Optional plan tier (defaults to "free").
	pub plan_tier: Option<String>,
}

#[derive(Debug, Serialize)]
struct TenantInfo {
	id: Uuid,
	name: String,
	slug: String,
	plan_tier: String,
	status: String,
}

#[derive(Debug, Serialize)]
struct ApiKeyInfo {
	id: Uuid,
	key: String,
	key_prefix: String,
	name: String,
}

#[derive(Debug, Serialize)]
struct OnboardResponse {
	tenant: TenantInfo,
	api_key: ApiKeyInfo,
	verification_result: serde_json::Value,
}

/// Scopes of the key created during onboarding. An empty scope list grants
/// every scope, including `admin`, so the key gets an explicit list.
const ONBOARD_KEY_SCOPES: &[&str] = &[
	scope::VERIFY,
	scope::BULK,
	scope::FIND,
	scope::LISTS,
	scope::SUPPRESSIONS,
	scope::REPUTATION,
	scope::SETTINGS,
	scope::PIPELINES_READ,
	scope::PIPELINES_WRITE,
	scope::PIPELINES_TRIGGER,
	scope::OUTCOMES_READ,
	scope::OUTCOMES_WRITE,
];

/// Plan tier and monthly allowance for a new tenant. The allowance must be
/// set here: a NULL `monthly_email_limit` means unlimited checks.
fn onboard_plan(
	plan_tier: Option<&str>,
) -> Result<(&'static str, Option<i32>), ReacherResponseError> {
	let plan_tier = plan_tier.unwrap_or("free");
	plan_allowance(plan_tier).ok_or_else(|| {
		ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			format!(
				"Unknown plan_tier '{}'. Expected free, starter, professional or enterprise.",
				plan_tier
			),
		)
	})
}

fn slugify(name: &str) -> String {
	name.to_lowercase()
		.chars()
		.map(|c| if c.is_alphanumeric() { c } else { '-' })
		.collect::<String>()
		.trim_matches('-')
		.to_string()
}

fn with_pg_pool(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (PgPool,), Error = warp::Rejection> + Clone {
	warp::any().and_then(move || {
		let config = Arc::clone(&config);
		async move {
			config.get_pg_pool().ok_or_else(|| {
				warp::reject::custom(ReacherResponseError::new(
					StatusCode::SERVICE_UNAVAILABLE,
					"Postgres database required for onboarding",
				))
			})
		}
	})
}

async fn onboard_handler(
	config: Arc<BackendConfig>,
	pg_pool: PgPool,
	body: OnboardRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
	crate::smtp_egress::require_smtp_ready(&config)?;
	// ── Validate ────────────────────────────────────
	if body.email_to_verify.is_empty() {
		return Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"email_to_verify is required",
		)
		.into());
	}
	if body.tenant_name.is_empty() {
		return Err(
			ReacherResponseError::new(StatusCode::BAD_REQUEST, "tenant_name is required").into(),
		);
	}
	if body.contact_email.is_empty() {
		return Err(ReacherResponseError::new(
			StatusCode::BAD_REQUEST,
			"contact_email is required",
		)
		.into());
	}

	let slug = match body.slug {
		Some(ref s) => s.clone(),
		None => slugify(&body.tenant_name),
	};
	let (plan_tier, monthly_email_limit) = onboard_plan(body.plan_tier.as_deref())?;

	// ── 1. Create tenant ────────────────────────────
	let tenant_row = sqlx::query(
		"INSERT INTO tenants (name, slug, contact_email, plan_tier, monthly_email_limit) \
		 VALUES ($1, $2, $3, $4::plan_tier, $5) \
		 RETURNING id, name, slug, plan_tier::TEXT, status::TEXT",
	)
	.bind(&body.tenant_name)
	.bind(&slug)
	.bind(&body.contact_email)
	.bind(plan_tier)
	.bind(monthly_email_limit)
	.fetch_one(&pg_pool)
	.await
	.map_err(|e| {
		if let sqlx::Error::Database(ref db_err) = e {
			if db_err.code().as_deref() == Some("23505") {
				return ReacherResponseError::new(
					StatusCode::CONFLICT,
					format!("Tenant with slug '{}' already exists", slug),
				);
			}
		}
		ReacherResponseError::from(e)
	})?;

	let tenant_id: Uuid = tenant_row.get("id");
	let tenant = TenantInfo {
		id: tenant_id,
		name: tenant_row.get("name"),
		slug: tenant_row.get("slug"),
		plan_tier: tenant_row.get("plan_tier"),
		status: tenant_row.get("status"),
	};

	info!(target: LOG_TARGET, tenant_id=?tenant_id, slug=?tenant.slug, "Onboard: created tenant");

	// ── 2. Generate API key ─────────────────────────
	let (full_key, key_prefix, key_hash) = generate_api_key();

	let key_row = sqlx::query(
		"INSERT INTO api_keys (tenant_id, key_prefix, key_hash, name, scopes) \
		 VALUES ($1, $2, $3, 'Default', $4) \
		 RETURNING id",
	)
	.bind(tenant_id)
	.bind(&key_prefix)
	.bind(&key_hash)
	.bind(ONBOARD_KEY_SCOPES)
	.fetch_one(&pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let api_key = ApiKeyInfo {
		id: key_row.get("id"),
		key: full_key,
		key_prefix,
		name: "Default".to_string(),
	};

	info!(target: LOG_TARGET, tenant_id=?tenant_id, "Onboard: created API key");

	// ── 3. Run email verification ───────────────────
	let input = crate::http::v0::check_email::post::CheckEmailRequest {
		to_email: body.email_to_verify.clone(),
		..Default::default()
	}
	.to_check_email_input(Arc::clone(&config));

	let output = check_email(&input).await;
	let result_json = serde_json::to_value(&output).map_err(ReacherResponseError::from)?;

	info!(
		target: LOG_TARGET,
		tenant_id=?tenant_id,
		email=body.email_to_verify,
		is_reachable=?output.is_reachable,
		"Onboard: verification complete"
	);

	// ── 4. Store result ─────────────────────────────
	let _ =
		sqlx::query("INSERT INTO v1_task_result (payload, result, tenant_id) VALUES ($1, $2, $3)")
			.bind(serde_json::json!({"to_email": body.email_to_verify}))
			.bind(&result_json)
			.bind(tenant_id)
			.execute(&pg_pool)
			.await;

	// ── 5. Return everything ────────────────────────
	Ok(warp::reply::with_status(
		warp::reply::json(&OnboardResponse {
			tenant,
			api_key,
			verification_result: result_json,
		}),
		StatusCode::CREATED,
	))
}

/// POST /v1/check-email-with-onboard — Operator onboarding + email verification in one call.
/// Requires the backend admin secret: it creates a tenant with the requested
/// plan, generates a non-admin API key, verifies the email, and returns all three.
/// Workspaces sign up through the web app instead.
#[utoipa::path(
	post,
	path = "/v1/check-email-with-onboard",
	tag = "v1",
	request_body = OnboardRequest,
	responses((status = 201, description = "Tenant onboarded and email verified"))
)]
pub fn v1_check_email_with_onboard(
	config: Arc<BackendConfig>,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
	let config2 = Arc::clone(&config);
	warp::path!("v1" / "check-email-with-onboard")
		.and(warp::post())
		.and(check_admin_header(Arc::clone(&config)))
		.and(warp::any().map(move || Arc::clone(&config2)))
		.and(with_pg_pool(config))
		.and(warp::body::json())
		.and_then(onboard_handler)
		.with(warp::log(LOG_TARGET))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_slugify() {
		assert_eq!(slugify("Acme Corp"), "acme-corp");
		assert_eq!(slugify("My Company!"), "my-company");
		assert_eq!(slugify("hello"), "hello");
		assert_eq!(slugify("A B C"), "a-b-c");
	}

	#[test]
	fn onboarded_tenant_gets_the_plan_allowance() {
		assert_eq!(onboard_plan(None).unwrap(), ("free", Some(100)));
		assert_eq!(
			onboard_plan(Some("professional")).unwrap(),
			("professional", Some(12_000))
		);
		// A NULL limit means unlimited; an unknown name must not reach the INSERT.
		assert_eq!(
			onboard_plan(Some("pro")).unwrap_err().code,
			StatusCode::BAD_REQUEST
		);
	}

	#[test]
	fn onboarding_key_cannot_manage_keys() {
		use crate::config::ThrottleConfig;
		use crate::tenant::context::{scope, TenantContext};

		let mut context = TenantContext::legacy(ThrottleConfig::new_without_throttle());
		context.is_legacy = false;
		context.scopes = ONBOARD_KEY_SCOPES.iter().map(|s| s.to_string()).collect();
		assert!(context.has_scope(scope::VERIFY));
		assert!(context.has_scope(scope::BULK));
		assert!(!context.has_scope(scope::ADMIN));
	}

	#[test]
	fn test_slugify_special_chars() {
		assert_eq!(slugify("test@company.com"), "test-company-com");
		assert_eq!(slugify("  spaces  "), "spaces");
	}
}
