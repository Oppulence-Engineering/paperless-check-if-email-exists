use crate::config::JwtAuthConfig;
use crate::tenant::auth::resolve_tenant_context_by_id;
use crate::tenant::context::TenantContext;
use anyhow::{bail, Context, Result};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use ring::signature::{UnparsedPublicKey, ED25519};
use serde::Deserialize;
use sqlx::PgPool;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

#[derive(Deserialize)]
struct Header {
	alg: String,
	kid: String,
}

#[derive(Deserialize)]
struct Jwks {
	keys: Vec<Jwk>,
}

#[derive(Deserialize)]
struct Jwk {
	kid: String,
	kty: String,
	crv: String,
	x: String,
	#[serde(default)]
	alg: Option<String>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Audience {
	One(String),
	Many(Vec<String>),
}

impl Audience {
	fn contains(&self, expected: &str) -> bool {
		match self {
			Self::One(value) => value == expected,
			Self::Many(values) => values.iter().any(|value| value == expected),
		}
	}
}

#[derive(Deserialize)]
struct Claims {
	sub: String,
	iss: String,
	aud: Audience,
	exp: u64,
	#[serde(default)]
	nbf: Option<u64>,
	org_id: String,
	org_name: String,
	org_role: String,
	session_id: String,
	email: String,
}

fn verify(token: &str, jwks: &Jwks, config: &JwtAuthConfig) -> Result<Claims> {
	if token.len() > 16_384 {
		bail!("Backend token is too large");
	}
	let mut parts = token.split('.');
	let (encoded_header, encoded_claims, encoded_signature) =
		match (parts.next(), parts.next(), parts.next(), parts.next()) {
			(Some(header), Some(claims), Some(signature), None) => (header, claims, signature),
			_ => bail!("Malformed backend token"),
		};
	let header: Header = serde_json::from_slice(&URL_SAFE_NO_PAD.decode(encoded_header)?)?;
	if header.alg != "EdDSA" || header.kid.is_empty() {
		bail!("Unsupported backend token header");
	}
	let key = jwks
		.keys
		.iter()
		.find(|key| key.kid == header.kid && key.kty == "OKP" && key.crv == "Ed25519")
		.context("Backend signing key not found")?;
	if key.alg.as_deref().is_some_and(|alg| alg != "EdDSA") {
		bail!("Backend signing key algorithm is invalid");
	}
	let public_key = URL_SAFE_NO_PAD.decode(&key.x)?;
	if public_key.len() != 32 {
		bail!("Backend signing key is invalid");
	}
	let signature = URL_SAFE_NO_PAD.decode(encoded_signature)?;
	UnparsedPublicKey::new(&ED25519, public_key)
		.verify(
			format!("{}.{}", encoded_header, encoded_claims).as_bytes(),
			&signature,
		)
		.map_err(|_| anyhow::anyhow!("Invalid backend token signature"))?;
	let claims: Claims = serde_json::from_slice(&URL_SAFE_NO_PAD.decode(encoded_claims)?)?;
	let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
	if claims.iss != config.issuer
		|| !claims.aud.contains(&config.audience)
		|| claims.exp <= now
		|| claims.exp > now + 600
		|| claims.nbf.is_some_and(|nbf| nbf > now + 60)
		|| claims.sub.is_empty()
		|| claims.session_id.is_empty()
		|| claims.org_id.is_empty()
		|| claims.org_name.trim().is_empty()
		|| !claims.email.contains('@')
	{
		bail!("Invalid backend token claims");
	}
	Ok(claims)
}

fn role_is_admin(role: &str) -> bool {
	role.split(',')
		.any(|part| matches!(part.trim(), "owner" | "admin"))
}

pub fn route_allowed(role: &str, method: &str, path: &str) -> bool {
	if path.starts_with("/v1/admin/") {
		return false;
	}
	if role_is_admin(role) {
		return true;
	}
	if !role.split(',').any(|part| part.trim() == "member") {
		return false;
	}
	match (method, path) {
		("GET", "/v1/me" | "/v1/me/usage" | "/v1/lists" | "/v1/query") => true,
		("GET", path) if path.starts_with("/v1/lists/") => true,
		("GET", path) if path.starts_with("/v1/emails/") && path.ends_with("/history") => true,
		("POST", "/v1/check_email" | "/v1/find_email" | "/v1/bulk" | "/v1/lists") => true,
		_ => false,
	}
}

pub async fn resolve_from_jwt(
	pool: &PgPool,
	token: &str,
	config: &JwtAuthConfig,
	method: &str,
	path: &str,
) -> Result<TenantContext> {
	// ponytail: Fetch keys per request; cache them if this becomes a measured latency cost.
	let jwks = reqwest::Client::builder()
		.timeout(std::time::Duration::from_secs(5))
		.redirect(reqwest::redirect::Policy::none())
		.build()?
		.get(&config.jwks_url)
		.send()
		.await?
		.error_for_status()?
		.json::<Jwks>()
		.await?;
	let claims = verify(token, &jwks, config)?;
	if !route_allowed(&claims.org_role, method, path) {
		bail!("Organization role cannot call this operation");
	}
	let tenant_id = Uuid::new_v5(&Uuid::NAMESPACE_OID, claims.org_id.as_bytes());
	sqlx::query(
		"INSERT INTO tenants (id, name, slug, contact_email) VALUES ($1, $2, $3, $4) \
		 ON CONFLICT (id) DO UPDATE SET name = EXCLUDED.name",
	)
	.bind(tenant_id)
	.bind(&claims.org_name)
	.bind(format!("auth-{}", tenant_id))
	.bind(&claims.email)
	.execute(pool)
	.await?;
	let mut context = resolve_tenant_context_by_id(pool, tenant_id).await?;
	context.scopes = if role_is_admin(&claims.org_role) {
		vec!["*".into()]
	} else {
		vec![
			"verify".into(),
			"find".into(),
			"bulk".into(),
			"lists".into(),
		]
	};
	Ok(context)
}

#[cfg(test)]
mod tests {
	use super::{route_allowed, verify, Jwk, Jwks};
	use crate::config::JwtAuthConfig;
	use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
	use ring::rand::SystemRandom;
	use ring::signature::{Ed25519KeyPair, KeyPair};
	use serde_json::json;
	use std::time::{SystemTime, UNIX_EPOCH};

	#[test]
	fn member_cannot_manage_credentials_or_cross_into_admin() {
		assert!(route_allowed("member", "POST", "/v1/check_email"));
		assert!(!route_allowed("member", "GET", "/v1/me/api-keys"));
		assert!(!route_allowed("member", "PATCH", "/v1/me/settings"));
		assert!(!route_allowed("owner", "GET", "/v1/admin/tenants"));
		assert!(!route_allowed("unknown", "POST", "/v1/check_email"));
	}

	#[test]
	fn backend_token_requires_valid_signature_issuer_audience_and_expiry() {
		let rng = SystemRandom::new();
		let pkcs8 = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
		let key = Ed25519KeyPair::from_pkcs8(pkcs8.as_ref()).unwrap();
		let jwks = Jwks {
			keys: vec![Jwk {
				kid: "test".into(),
				kty: "OKP".into(),
				crv: "Ed25519".into(),
				x: URL_SAFE_NO_PAD.encode(key.public_key().as_ref()),
				alg: Some("EdDSA".into()),
			}],
		};
		let config = JwtAuthConfig {
			jwks_url: "http://localhost/jwks".into(),
			issuer: "http://localhost".into(),
			audience: "check-if-email-exists-api".into(),
		};
		let now = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.unwrap()
			.as_secs();
		let claims = json!({
			"sub": "user-1", "iss": config.issuer, "aud": config.audience,
			"exp": now + 300, "org_id": "org-1", "org_name": "Workspace 1",
			"org_role": "member", "session_id": "session-1", "email": "user@example.com"
		});
		let sign = |claims: serde_json::Value| {
			let head = URL_SAFE_NO_PAD.encode(br#"{"alg":"EdDSA","kid":"test"}"#);
			let payload = URL_SAFE_NO_PAD.encode(claims.to_string());
			let signed = format!("{}.{}", head, payload);
			format!(
				"{}.{}",
				signed,
				URL_SAFE_NO_PAD.encode(key.sign(signed.as_bytes()).as_ref())
			)
		};
		let token = sign(claims.clone());
		assert_eq!(verify(&token, &jwks, &config).unwrap().org_id, "org-1");
		assert!(verify(&format!("{}x", token), &jwks, &config).is_err());
		let mut other_org = claims.clone();
		other_org["org_id"] = json!("org-2");
		let parts: Vec<&str> = token.split('.').collect();
		let tampered = format!(
			"{}.{}.{}",
			parts[0],
			URL_SAFE_NO_PAD.encode(other_org.to_string()),
			parts[2]
		);
		assert!(verify(&tampered, &jwks, &config).is_err());
		assert_eq!(
			verify(&sign(other_org), &jwks, &config).unwrap().org_id,
			"org-2"
		);
		let mut wrong_issuer = claims.clone();
		wrong_issuer["iss"] = json!("http://other-host");
		assert!(verify(&sign(wrong_issuer), &jwks, &config).is_err());
		let mut expired = claims.clone();
		expired["exp"] = json!(now - 1);
		assert!(verify(&sign(expired), &jwks, &config).is_err());
		let mut wrong_audience = claims;
		wrong_audience["aud"] = json!("other-api");
		assert!(verify(&sign(wrong_audience), &jwks, &config).is_err());
	}
}
