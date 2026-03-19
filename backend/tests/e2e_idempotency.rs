#[cfg(test)]
mod test_helpers;

#[cfg(test)]
mod tests {
	use super::test_helpers::TestDb;
	use reacher_backend::http::idempotency::{
		check_idempotency_key, cleanup_idempotency_keys, complete_idempotency_key,
		fail_idempotency_key, hash_request_body, IdempotencyCheck,
	};
	use serial_test::serial;
	use sqlx::Row;

	const TENANT: &str = "test-tenant";
	const PATH: &str = "/v1/check_email";
	const LOCKED_BY: &str = "test-worker";

	#[tokio::test]
	#[serial]
	async fn test_new_key_returns_new() {
		let db = TestDb::start().await;
		let pool = db.pool();
		let body_hash = hash_request_body(b"hello");

		let result = check_idempotency_key(pool, TENANT, "key-new", PATH, &body_hash, LOCKED_BY)
			.await
			.expect("check_idempotency_key failed");

		match result {
			IdempotencyCheck::New { record_id } => {
				assert!(record_id > 0, "record_id should be positive");
			}
			other => panic!("Expected New, got {:?}", other),
		}
	}

	#[tokio::test]
	#[serial]
	async fn test_duplicate_key_in_progress() {
		let db = TestDb::start().await;
		let pool = db.pool();
		let body_hash = hash_request_body(b"body-in-progress");

		// First insert — should return New.
		let first = check_idempotency_key(pool, TENANT, "key-dup", PATH, &body_hash, LOCKED_BY)
			.await
			.expect("first check failed");
		assert!(matches!(first, IdempotencyCheck::New { .. }));

		// Second check with same key — should return InProgress.
		let second = check_idempotency_key(pool, TENANT, "key-dup", PATH, &body_hash, LOCKED_BY)
			.await
			.expect("second check failed");
		assert!(
			matches!(second, IdempotencyCheck::InProgress),
			"Expected InProgress, got {:?}",
			second
		);
	}

	#[tokio::test]
	#[serial]
	async fn test_duplicate_key_completed() {
		let db = TestDb::start().await;
		let pool = db.pool();
		let body_hash = hash_request_body(b"body-completed");
		let response_body = b"cached-response-body";

		// Insert and complete.
		let record_id =
			match check_idempotency_key(pool, TENANT, "key-comp", PATH, &body_hash, LOCKED_BY)
				.await
				.expect("first check failed")
			{
				IdempotencyCheck::New { record_id } => record_id,
				other => panic!("Expected New, got {:?}", other),
			};

		complete_idempotency_key(pool, record_id, 200, response_body)
			.await
			.expect("complete failed");

		// Second check should return Cached.
		let second = check_idempotency_key(pool, TENANT, "key-comp", PATH, &body_hash, LOCKED_BY)
			.await
			.expect("second check failed");

		match second {
			IdempotencyCheck::Cached(cached) => {
				assert_eq!(cached.status_code, 200);
				assert_eq!(cached.body, response_body.to_vec());
			}
			other => panic!("Expected Cached, got {:?}", other),
		}
	}

	#[tokio::test]
	#[serial]
	async fn test_body_mismatch() {
		let db = TestDb::start().await;
		let pool = db.pool();
		let hash_a = hash_request_body(b"body-a");
		let hash_b = hash_request_body(b"body-b");

		// Insert with hash_a.
		let first = check_idempotency_key(pool, TENANT, "key-mismatch", PATH, &hash_a, LOCKED_BY)
			.await
			.expect("first check failed");
		assert!(matches!(first, IdempotencyCheck::New { .. }));

		// Check with hash_b — should detect mismatch.
		let second = check_idempotency_key(pool, TENANT, "key-mismatch", PATH, &hash_b, LOCKED_BY)
			.await
			.expect("second check failed");
		assert!(
			matches!(second, IdempotencyCheck::BodyMismatch),
			"Expected BodyMismatch, got {:?}",
			second
		);
	}

	#[tokio::test]
	#[serial]
	async fn test_failed_key_allows_retry() {
		let db = TestDb::start().await;
		let pool = db.pool();
		let body_hash = hash_request_body(b"body-fail-retry");

		// Insert and fail.
		let record_id =
			match check_idempotency_key(pool, TENANT, "key-fail", PATH, &body_hash, LOCKED_BY)
				.await
				.expect("first check failed")
			{
				IdempotencyCheck::New { record_id } => record_id,
				other => panic!("Expected New, got {:?}", other),
			};

		fail_idempotency_key(pool, record_id)
			.await
			.expect("fail failed");

		// Second check should return New (a fresh record).
		let second = check_idempotency_key(pool, TENANT, "key-fail", PATH, &body_hash, LOCKED_BY)
			.await
			.expect("second check failed");

		match second {
			IdempotencyCheck::New { record_id: new_id } => {
				assert!(new_id > 0, "new record_id should be positive");
				assert_ne!(new_id, record_id, "should be a different record");
			}
			other => panic!("Expected New, got {:?}", other),
		}
	}

	#[tokio::test]
	#[serial]
	async fn test_complete_stores_response() {
		let db = TestDb::start().await;
		let pool = db.pool();
		let body_hash = hash_request_body(b"body-store");
		let response_body = b"{\"result\": \"ok\"}";

		let record_id =
			match check_idempotency_key(pool, TENANT, "key-store", PATH, &body_hash, LOCKED_BY)
				.await
				.expect("check failed")
			{
				IdempotencyCheck::New { record_id } => record_id,
				other => panic!("Expected New, got {:?}", other),
			};

		complete_idempotency_key(pool, record_id, 201, response_body)
			.await
			.expect("complete failed");

		// Verify by checking the same key again.
		let cached = check_idempotency_key(pool, TENANT, "key-store", PATH, &body_hash, LOCKED_BY)
			.await
			.expect("cached check failed");

		match cached {
			IdempotencyCheck::Cached(resp) => {
				assert_eq!(resp.status_code, 201);
				assert_eq!(resp.body, response_body.to_vec());
			}
			other => panic!("Expected Cached, got {:?}", other),
		}
	}

	#[tokio::test]
	#[serial]
	async fn test_cleanup_removes_expired() {
		let db = TestDb::start().await;
		let pool = db.pool();
		let body_hash = hash_request_body(b"body-cleanup-expired");

		let record_id =
			match check_idempotency_key(pool, TENANT, "key-expire", PATH, &body_hash, LOCKED_BY)
				.await
				.expect("check failed")
			{
				IdempotencyCheck::New { record_id } => record_id,
				other => panic!("Expected New, got {:?}", other),
			};

		// Complete the record so it becomes eligible for expiry cleanup.
		complete_idempotency_key(pool, record_id, 200, b"done")
			.await
			.expect("complete failed");

		// Manually set expires_at to the past.
		sqlx::query(
			"UPDATE idempotency_keys SET expires_at = NOW() - INTERVAL '1 hour' WHERE id = $1",
		)
		.bind(record_id)
		.execute(pool)
		.await
		.expect("update expires_at failed");

		// Run cleanup.
		cleanup_idempotency_keys(pool).await;

		// Verify the record was deleted — next check should return New.
		let after = check_idempotency_key(pool, TENANT, "key-expire", PATH, &body_hash, LOCKED_BY)
			.await
			.expect("post-cleanup check failed");

		assert!(
			matches!(after, IdempotencyCheck::New { .. }),
			"Expected New after cleanup, got {:?}",
			after
		);
	}

	#[tokio::test]
	#[serial]
	async fn test_cleanup_marks_stale_processing_as_failed() {
		let db = TestDb::start().await;
		let pool = db.pool();
		let body_hash = hash_request_body(b"body-stale");

		// Insert a key (stays in 'processing' status).
		let _record_id =
			match check_idempotency_key(pool, TENANT, "key-stale", PATH, &body_hash, LOCKED_BY)
				.await
				.expect("check failed")
			{
				IdempotencyCheck::New { record_id } => record_id,
				other => panic!("Expected New, got {:?}", other),
			};

		// Manually set locked_at to 10 minutes ago to simulate a stale lock.
		sqlx::query(
			"UPDATE idempotency_keys SET locked_at = NOW() - INTERVAL '10 minutes' WHERE tenant_id = $1 AND idempotency_key = $2",
		)
		.bind(TENANT)
		.bind("key-stale")
		.execute(pool)
		.await
		.expect("update locked_at failed");

		// Run cleanup — should mark stale processing record as failed.
		cleanup_idempotency_keys(pool).await;

		// Verify status is now 'failed' by querying directly.
		let row = sqlx::query(
			"SELECT status FROM idempotency_keys WHERE tenant_id = $1 AND idempotency_key = $2",
		)
		.bind(TENANT)
		.bind("key-stale")
		.fetch_one(pool)
		.await
		.expect("status query failed");

		let status: String = row.get("status");

		assert_eq!(
			status, "failed",
			"stale processing record should be marked failed"
		);

		// Next check should return New (failed records allow retry).
		let retry = check_idempotency_key(pool, TENANT, "key-stale", PATH, &body_hash, LOCKED_BY)
			.await
			.expect("retry check failed");

		assert!(
			matches!(retry, IdempotencyCheck::New { .. }),
			"Expected New after stale cleanup, got {:?}",
			retry
		);
	}

	#[tokio::test]
	#[serial]
	async fn test_hash_deterministic() {
		let body = b"deterministic-body-content";
		let hash1 = hash_request_body(body);
		let hash2 = hash_request_body(body);
		assert_eq!(hash1, hash2, "same body should produce same hash");
	}

	#[tokio::test]
	#[serial]
	async fn test_hash_different_bodies() {
		let hash_a = hash_request_body(b"alpha");
		let hash_b = hash_request_body(b"beta");
		assert_ne!(
			hash_a, hash_b,
			"different bodies should produce different hashes"
		);
	}

	#[tokio::test]
	#[serial]
	async fn test_concurrent_inserts() {
		let db = TestDb::start().await;
		let pool = db.pool();
		let body_hash = hash_request_body(b"concurrent-body");

		let mut handles = Vec::new();
		for i in 0..5 {
			let pool_clone = db.pool_owned();
			let hash_clone = body_hash.clone();
			let worker = format!("worker-{}", i);
			handles.push(tokio::spawn(async move {
				check_idempotency_key(
					&pool_clone,
					TENANT,
					"key-concurrent",
					PATH,
					&hash_clone,
					&worker,
				)
				.await
			}));
		}

		let mut new_count = 0u32;
		let mut in_progress_count = 0u32;
		for handle in handles {
			let result = handle.await.expect("task panicked").expect("check failed");
			match result {
				IdempotencyCheck::New { .. } => new_count += 1,
				IdempotencyCheck::InProgress => in_progress_count += 1,
				other => panic!("Unexpected result in concurrent test: {:?}", other),
			}
		}

		assert_eq!(new_count, 1, "exactly one task should get New");
		assert_eq!(
			in_progress_count, 4,
			"remaining tasks should get InProgress"
		);
	}
}
