use check_if_email_exists::LOG_TARGET;
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use tracing::{debug, warn};
use warp::http::StatusCode;

use crate::http::ReacherResponseError;

/// Represents a cached idempotency response.
#[derive(Debug)]
pub struct CachedResponse {
	pub status_code: u16,
	pub body: Vec<u8>,
	pub headers: Option<serde_json::Value>,
}

/// Result of checking an idempotency key.
#[derive(Debug)]
pub enum IdempotencyCheck {
	/// New key — caller should proceed with handler execution.
	New { record_id: i64 },
	/// Request is currently being processed by another caller.
	InProgress,
	/// Cached response available (body hash matched).
	Cached(CachedResponse),
	/// Body hash mismatch — same key, different request body.
	BodyMismatch,
}

/// Hash a request body using SHA-256.
pub fn hash_request_body(body: &[u8]) -> Vec<u8> {
	let mut hasher = Sha256::new();
	hasher.update(body);
	hasher.finalize().to_vec()
}

/// Check or insert an idempotency key. Returns the check result.
pub async fn check_idempotency_key(
	pg_pool: &PgPool,
	tenant_id: &str,
	idempotency_key: &str,
	request_path: &str,
	request_body_hash: &[u8],
	locked_by: &str,
) -> Result<IdempotencyCheck, ReacherResponseError> {
	// Try to INSERT. If it conflicts, we'll handle the existing record.
	let insert_result = sqlx::query_scalar!(
		r#"
		INSERT INTO idempotency_keys (tenant_id, idempotency_key, request_path, request_body_hash, locked_at, locked_by)
		VALUES ($1, $2, $3, $4, NOW(), $5)
		ON CONFLICT (tenant_id, idempotency_key) DO NOTHING
		RETURNING id
		"#,
		tenant_id,
		idempotency_key,
		request_path,
		request_body_hash,
		locked_by,
	)
	.fetch_optional(pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	if let Some(id) = insert_result {
		return Ok(IdempotencyCheck::New { record_id: id });
	}

	// Conflict — fetch the existing record
	let existing = sqlx::query!(
		r#"
		SELECT id, status, request_body_hash, response_status_code, response_body, response_headers
		FROM idempotency_keys
		WHERE tenant_id = $1 AND idempotency_key = $2
		"#,
		tenant_id,
		idempotency_key,
	)
	.fetch_optional(pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;

	let existing = match existing {
		Some(r) => r,
		None => {
			// Race condition: record was deleted between INSERT and SELECT
			return Err(ReacherResponseError::new(
				StatusCode::INTERNAL_SERVER_ERROR,
				"Idempotency key race condition",
			));
		}
	};

	// Check body hash match
	if existing.request_body_hash != request_body_hash {
		return Ok(IdempotencyCheck::BodyMismatch);
	}

	match existing.status.as_str() {
		"processing" => Ok(IdempotencyCheck::InProgress),
		"completed" => Ok(IdempotencyCheck::Cached(CachedResponse {
			status_code: existing.response_status_code.unwrap_or(200) as u16,
			body: existing.response_body.unwrap_or_default(),
			headers: existing.response_headers,
		})),
		"failed" => {
			// Delete the failed record and re-insert as processing
			sqlx::query!("DELETE FROM idempotency_keys WHERE id = $1", existing.id,)
				.execute(pg_pool)
				.await
				.map_err(ReacherResponseError::from)?;

			let new_id = sqlx::query_scalar!(
				r#"
				INSERT INTO idempotency_keys (tenant_id, idempotency_key, request_path, request_body_hash, locked_at, locked_by)
				VALUES ($1, $2, $3, $4, NOW(), $5)
				RETURNING id
				"#,
				tenant_id,
				idempotency_key,
				request_path,
				request_body_hash,
				locked_by,
			)
			.fetch_one(pg_pool)
			.await
			.map_err(ReacherResponseError::from)?;

			Ok(IdempotencyCheck::New { record_id: new_id })
		}
		_ => Err(ReacherResponseError::new(
			StatusCode::INTERNAL_SERVER_ERROR,
			format!("Unknown idempotency status: {}", existing.status),
		)),
	}
}

/// Mark an idempotency key as completed with the response data.
pub async fn complete_idempotency_key(
	pg_pool: &PgPool,
	record_id: i64,
	status_code: u16,
	response_body: &[u8],
) -> Result<(), ReacherResponseError> {
	sqlx::query!(
		r#"
		UPDATE idempotency_keys
		SET status = 'completed', response_status_code = $2, response_body = $3, updated_at = NOW()
		WHERE id = $1
		"#,
		record_id,
		status_code as i16,
		response_body,
	)
	.execute(pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;
	Ok(())
}

/// Mark an idempotency key as failed.
pub async fn fail_idempotency_key(
	pg_pool: &PgPool,
	record_id: i64,
) -> Result<(), ReacherResponseError> {
	sqlx::query!(
		"UPDATE idempotency_keys SET status = 'failed', updated_at = NOW() WHERE id = $1",
		record_id,
	)
	.execute(pg_pool)
	.await
	.map_err(ReacherResponseError::from)?;
	Ok(())
}

/// Background cleanup task. Call this in a tokio::spawn loop.
/// Deletes expired completed/failed records and marks stale processing records as failed.
pub async fn cleanup_idempotency_keys(pg_pool: &PgPool) {
	let deleted = sqlx::query!(
		"DELETE FROM idempotency_keys WHERE expires_at < NOW() AND status != 'processing'"
	)
	.execute(pg_pool)
	.await;

	match deleted {
		Ok(result) => {
			if result.rows_affected() > 0 {
				debug!(target: LOG_TARGET, rows=result.rows_affected(), "Cleaned up expired idempotency keys");
			}
		}
		Err(e) => warn!(target: LOG_TARGET, error=?e, "Failed to clean up idempotency keys"),
	}

	let stale = sqlx::query!(
		"UPDATE idempotency_keys SET status = 'failed', updated_at = NOW() WHERE status = 'processing' AND locked_at < NOW() - INTERVAL '5 minutes'"
	)
	.execute(pg_pool)
	.await;

	match stale {
		Ok(result) => {
			if result.rows_affected() > 0 {
				debug!(target: LOG_TARGET, rows=result.rows_affected(), "Marked stale processing idempotency keys as failed");
			}
		}
		Err(e) => warn!(target: LOG_TARGET, error=?e, "Failed to clean up stale idempotency keys"),
	}
}

/// Spawn a background task that runs cleanup every 5 minutes.
pub fn spawn_idempotency_cleanup(pg_pool: PgPool) {
	tokio::spawn(async move {
		loop {
			tokio::time::sleep(std::time::Duration::from_secs(300)).await;
			cleanup_idempotency_keys(&pg_pool).await;
		}
	});
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_hash_request_body_deterministic() {
		let body = b"request body content";
		assert_eq!(hash_request_body(body), hash_request_body(body));
	}

	#[test]
	fn test_hash_request_body_different_inputs() {
		assert_ne!(hash_request_body(b"one"), hash_request_body(b"two"));
	}

	#[test]
	fn test_hash_request_body_sha256_length() {
		assert_eq!(hash_request_body(b"anything").len(), 32); // SHA-256 = 32 bytes
	}

	#[test]
	fn test_hash_empty_body() {
		let hash = hash_request_body(b"");
		assert_eq!(hash.len(), 32);
		// Empty string SHA-256 is well-known
		assert_eq!(
			hex::encode(&hash),
			"e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
		);
	}

	#[test]
	fn test_idempotency_check_new_variant() {
		let check = IdempotencyCheck::New { record_id: 42 };
		match check {
			IdempotencyCheck::New { record_id } => assert_eq!(record_id, 42),
			_ => panic!("Expected New variant"),
		}
	}

	#[test]
	fn test_cached_response_fields() {
		let cached = CachedResponse {
			status_code: 201,
			body: b"response body".to_vec(),
			headers: Some(serde_json::json!({"X-Custom": "value"})),
		};
		assert_eq!(cached.status_code, 201);
		assert_eq!(cached.body, b"response body");
		assert!(cached.headers.is_some());
	}
}
