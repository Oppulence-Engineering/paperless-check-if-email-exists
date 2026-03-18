/// End-to-end tests for job lifecycle SQL operations against a real Postgres
/// database spun up via testcontainers.  These are **direct DB tests** — they
/// exercise the SQL queries that the HTTP handlers would execute, without going
/// through the HTTP layer.

#[cfg(test)]
mod test_helpers;

// ---------------------------------------------------------------------------
// Job Status Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod job_status_tests {
	use super::test_helpers::{self, TestDb};
	use serial_test::serial;
	use sqlx::Row;

	#[tokio::test]
	#[serial]
	async fn test_job_status_with_task_summary() {
		let db = TestDb::start().await;
		let pool = db.pool();

		let tenant_id = test_helpers::insert_tenant(pool, "status-summary", None, 0).await;
		let job_id = test_helpers::insert_job(pool, Some(tenant_id), 5, "running").await;

		// 2 completed, 1 running, 1 queued, 1 failed
		test_helpers::insert_task(
			pool,
			job_id,
			"completed",
			Some(tenant_id),
			Some(test_helpers::safe_result()),
			None,
		)
		.await;
		test_helpers::insert_task(
			pool,
			job_id,
			"completed",
			Some(tenant_id),
			Some(test_helpers::safe_result()),
			None,
		)
		.await;
		test_helpers::insert_task(pool, job_id, "running", Some(tenant_id), None, None).await;
		test_helpers::insert_task(pool, job_id, "queued", Some(tenant_id), None, None).await;
		test_helpers::insert_task(
			pool,
			job_id,
			"failed",
			Some(tenant_id),
			None,
			Some("timeout"),
		)
		.await;

		// Aggregate task states for this job
		let rows = sqlx::query(
			"SELECT task_state::TEXT AS task_state, COUNT(*) AS cnt FROM v1_task_result WHERE job_id = $1 GROUP BY task_state ORDER BY task_state",
		)
		.bind(job_id)
		.fetch_all(pool)
		.await
		.expect("task state aggregation query failed");

		let mut completed: i64 = 0;
		let mut running: i64 = 0;
		let mut queued: i64 = 0;
		let mut failed: i64 = 0;

		for row in &rows {
			let task_state: String = row.get("task_state");
			let cnt: i64 = row.get("cnt");
			match task_state.as_str() {
				"completed" => completed = cnt,
				"running" => running = cnt,
				"queued" => queued = cnt,
				"failed" => failed = cnt,
				_ => {}
			}
		}

		assert_eq!(completed, 2, "expected 2 completed tasks");
		assert_eq!(running, 1, "expected 1 running task");
		assert_eq!(queued, 1, "expected 1 queued task");
		assert_eq!(failed, 1, "expected 1 failed task");
	}

	#[tokio::test]
	#[serial]
	async fn test_job_not_found() {
		let db = TestDb::start().await;
		let pool = db.pool();

		let row = sqlx::query(
			"SELECT id, status::TEXT AS status, total_records FROM v1_bulk_job WHERE id = $1",
		)
		.bind(999_999i32)
		.fetch_optional(pool)
		.await
		.expect("query should not error");

		assert!(row.is_none(), "nonexistent job_id should return no rows");
	}
}

// ---------------------------------------------------------------------------
// Job Cancellation Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod job_cancellation_tests {
	use super::test_helpers::{self, TestDb};
	use serial_test::serial;
	use sqlx::Row;

	#[tokio::test]
	#[serial]
	async fn test_cancel_running_job() {
		let db = TestDb::start().await;
		let pool = db.pool();

		let tenant_id = test_helpers::insert_tenant(pool, "cancel-run", None, 0).await;
		let job_id = test_helpers::insert_job(pool, Some(tenant_id), 4, "running").await;

		// 3 queued + 1 running
		test_helpers::insert_task(pool, job_id, "queued", Some(tenant_id), None, None).await;
		test_helpers::insert_task(pool, job_id, "queued", Some(tenant_id), None, None).await;
		test_helpers::insert_task(pool, job_id, "queued", Some(tenant_id), None, None).await;
		test_helpers::insert_task(pool, job_id, "running", Some(tenant_id), None, None).await;

		// Cancel cancellable tasks
		let row = sqlx::query(
			"WITH updated AS (
				UPDATE v1_task_result
				SET task_state = 'cancelled'::task_state
				WHERE job_id = $1
				  AND task_state IN ('queued', 'retrying')
				RETURNING id
			)
			SELECT COUNT(*) AS cnt FROM updated",
		)
		.bind(job_id)
		.fetch_one(pool)
		.await
		.expect("cancel query failed");

		let cancelled_count: i64 = row.get("cnt");

		assert_eq!(cancelled_count, 3, "3 queued tasks should be cancelled");

		// Verify the running task is still running
		let row = sqlx::query(
			"SELECT COUNT(*) AS cnt FROM v1_task_result WHERE job_id = $1 AND task_state = 'running'::task_state",
		)
		.bind(job_id)
		.fetch_one(pool)
		.await
		.expect("count running query failed");

		let still_running: i64 = row.get("cnt");

		assert_eq!(still_running, 1, "1 running task should remain");
	}

	#[tokio::test]
	#[serial]
	async fn test_cancel_sets_job_status() {
		let db = TestDb::start().await;
		let pool = db.pool();

		let tenant_id = test_helpers::insert_tenant(pool, "cancel-status", None, 0).await;
		let job_id = test_helpers::insert_job(pool, Some(tenant_id), 2, "running").await;

		// 2 queued tasks — cancelling all of them leaves no non-terminal tasks
		test_helpers::insert_task(pool, job_id, "queued", Some(tenant_id), None, None).await;
		test_helpers::insert_task(pool, job_id, "queued", Some(tenant_id), None, None).await;

		// Cancel the tasks
		sqlx::query(
			"UPDATE v1_task_result SET task_state = 'cancelled'::task_state WHERE job_id = $1 AND task_state IN ('queued', 'retrying')",
		)
		.bind(job_id)
		.execute(pool)
		.await
		.expect("cancel tasks failed");

		// Check if any non-terminal tasks remain
		let row = sqlx::query(
			"SELECT COUNT(*) AS cnt FROM v1_task_result WHERE job_id = $1 AND task_state NOT IN ('completed', 'failed', 'cancelled', 'dead_lettered')",
		)
		.bind(job_id)
		.fetch_one(pool)
		.await
		.expect("active count query failed");

		let active_count: i64 = row.get("cnt");

		assert_eq!(active_count, 0, "no active tasks should remain");

		// Since no non-terminal tasks remain, mark job as cancelled
		if active_count == 0 {
			sqlx::query(
				"UPDATE v1_bulk_job SET status = 'cancelled'::job_state, cancelled_at = NOW() WHERE id = $1",
			)
			.bind(job_id)
			.execute(pool)
			.await
			.expect("update job status failed");
		}

		// Verify job status
		let row = sqlx::query("SELECT status::TEXT AS status FROM v1_bulk_job WHERE id = $1")
			.bind(job_id)
			.fetch_one(pool)
			.await
			.expect("fetch job status failed");

		let job_status: String = row.get("status");

		assert_eq!(job_status, "cancelled");
	}

	#[tokio::test]
	#[serial]
	async fn test_cancel_completed_job_rejected() {
		let db = TestDb::start().await;
		let pool = db.pool();

		let tenant_id = test_helpers::insert_tenant(pool, "cancel-done", None, 0).await;
		let job_id = test_helpers::insert_job(pool, Some(tenant_id), 1, "completed").await;

		// Attempt to set a completed job to cancelling — should affect 0 rows
		let result = sqlx::query(
			"UPDATE v1_bulk_job SET status = 'cancelling'::job_state WHERE id = $1 AND status NOT IN ('completed', 'cancelled', 'failed')",
		)
		.bind(job_id)
		.execute(pool)
		.await
		.expect("conditional update query failed");

		assert_eq!(
			result.rows_affected(),
			0,
			"completed job should not transition to cancelling"
		);

		// Confirm status is still completed
		let row = sqlx::query("SELECT status::TEXT AS status FROM v1_bulk_job WHERE id = $1")
			.bind(job_id)
			.fetch_one(pool)
			.await
			.expect("fetch status failed");

		let status: String = row.get("status");

		assert_eq!(status, "completed");
	}
}

// ---------------------------------------------------------------------------
// Event Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod event_tests {
	use super::test_helpers::{self, TestDb};
	use serial_test::serial;
	use sqlx::Row;

	#[tokio::test]
	#[serial]
	async fn test_events_ordered_by_created_at() {
		let db = TestDb::start().await;
		let pool = db.pool();

		let tenant_id = test_helpers::insert_tenant(pool, "evt-order", None, 0).await;
		let job_id = test_helpers::insert_job(pool, Some(tenant_id), 1, "running").await;

		let mut event_ids = Vec::new();
		for i in 0..5 {
			let eid = test_helpers::insert_event(pool, job_id, None, &format!("event_{}", i)).await;
			event_ids.push(eid);
		}

		let rows = sqlx::query(
			"SELECT id, event_type, created_at FROM job_events WHERE job_id = $1 ORDER BY created_at ASC, id ASC",
		)
		.bind(job_id)
		.fetch_all(pool)
		.await
		.expect("event ordering query failed");

		assert_eq!(rows.len(), 5);

		// IDs should be monotonically increasing (inserted sequentially)
		for i in 1..rows.len() {
			let prev_ts: chrono::DateTime<chrono::Utc> = rows[i - 1].get("created_at");
			let curr_ts: chrono::DateTime<chrono::Utc> = rows[i].get("created_at");
			assert!(
				curr_ts >= prev_ts,
				"events should be ordered by created_at ASC"
			);
		}

		// Verify all expected event_ids are present
		let fetched_ids: Vec<i64> = rows.iter().map(|r| r.get::<i64, _>("id")).collect();
		assert_eq!(fetched_ids, event_ids);
	}

	#[tokio::test]
	#[serial]
	async fn test_events_pagination() {
		let db = TestDb::start().await;
		let pool = db.pool();

		let tenant_id = test_helpers::insert_tenant(pool, "evt-page", None, 0).await;
		let job_id = test_helpers::insert_job(pool, Some(tenant_id), 1, "running").await;

		for i in 0..10 {
			test_helpers::insert_event(pool, job_id, None, &format!("page_evt_{}", i)).await;
		}

		// First page: LIMIT 3 OFFSET 0
		let page1 = sqlx::query(
			"SELECT id, event_type FROM job_events WHERE job_id = $1 ORDER BY created_at ASC, id ASC LIMIT 3 OFFSET 0",
		)
		.bind(job_id)
		.fetch_all(pool)
		.await
		.expect("page 1 query failed");

		assert_eq!(page1.len(), 3, "first page should have 3 events");

		// Second page: LIMIT 3 OFFSET 3
		let page2 = sqlx::query(
			"SELECT id, event_type FROM job_events WHERE job_id = $1 ORDER BY created_at ASC, id ASC LIMIT 3 OFFSET 3",
		)
		.bind(job_id)
		.fetch_all(pool)
		.await
		.expect("page 2 query failed");

		assert_eq!(page2.len(), 3, "second page should have 3 events");

		// Pages should not overlap
		let page1_ids: Vec<i64> = page1.iter().map(|r| r.get::<i64, _>("id")).collect();
		let page2_ids: Vec<i64> = page2.iter().map(|r| r.get::<i64, _>("id")).collect();

		for id in &page2_ids {
			assert!(
				!page1_ids.contains(id),
				"page 2 should not contain ids from page 1"
			);
		}

		// page2 ids should all be greater than page1 ids (ordered)
		assert!(
			page2_ids[0] > *page1_ids.last().unwrap(),
			"page 2 ids should follow page 1 ids"
		);
	}
}

// ---------------------------------------------------------------------------
// Cursor-Based Results Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod cursor_results_tests {
	use super::test_helpers::{self, TestDb};
	use serial_test::serial;
	use sqlx::Row;

	#[tokio::test]
	#[serial]
	async fn test_cursor_pagination_first_page() {
		let db = TestDb::start().await;
		let pool = db.pool();

		let tenant_id = test_helpers::insert_tenant(pool, "cursor-first", None, 0).await;
		let job_id = test_helpers::insert_job(pool, Some(tenant_id), 10, "running").await;

		// Insert 10 completed tasks with results
		for _ in 0..10 {
			test_helpers::insert_task(
				pool,
				job_id,
				"completed",
				Some(tenant_id),
				Some(test_helpers::safe_result()),
				None,
			)
			.await;
		}

		let limit: i64 = 3;

		// First page: no cursor
		let rows = sqlx::query(
			"SELECT id, result, task_state::TEXT AS task_state FROM v1_task_result WHERE job_id = $1 ORDER BY id ASC LIMIT $2",
		)
		.bind(job_id)
		.bind(limit + 1) // fetch one extra to determine has_more
		.fetch_all(pool)
		.await
		.expect("cursor first page query failed");

		let has_more = rows.len() as i64 > limit;
		let results: Vec<_> = rows.into_iter().take(limit as usize).collect();
		let next_cursor = results.last().map(|r| r.get::<i32, _>("id"));

		assert_eq!(results.len(), 3, "first page should have 3 results");
		assert!(has_more, "should indicate more results");
		assert!(next_cursor.is_some(), "next_cursor should be set");
	}

	#[tokio::test]
	#[serial]
	async fn test_cursor_pagination_next_page() {
		let db = TestDb::start().await;
		let pool = db.pool();

		let tenant_id = test_helpers::insert_tenant(pool, "cursor-next", None, 0).await;
		let job_id = test_helpers::insert_job(pool, Some(tenant_id), 10, "running").await;

		let mut task_ids = Vec::new();
		for _ in 0..10 {
			let tid = test_helpers::insert_task(
				pool,
				job_id,
				"completed",
				Some(tenant_id),
				Some(test_helpers::safe_result()),
				None,
			)
			.await;
			task_ids.push(tid);
		}

		let limit: i64 = 3;

		// First page
		let page1 =
			sqlx::query("SELECT id FROM v1_task_result WHERE job_id = $1 ORDER BY id ASC LIMIT $2")
				.bind(job_id)
				.bind(limit)
				.fetch_all(pool)
				.await
				.expect("page1 query failed");

		let cursor: i32 = page1.last().unwrap().get("id");

		// Next page: id > cursor
		let page2 = sqlx::query(
			"SELECT id, result, task_state::TEXT AS task_state FROM v1_task_result WHERE job_id = $1 AND id > $2 ORDER BY id ASC LIMIT $3",
		)
		.bind(job_id)
		.bind(cursor)
		.bind(limit)
		.fetch_all(pool)
		.await
		.expect("page2 query failed");

		assert_eq!(page2.len(), 3, "next page should have 3 results");

		for row in &page2 {
			let row_id: i32 = row.get("id");
			assert!(
				row_id > cursor,
				"all results on page 2 should have id > cursor ({}), got {}",
				cursor,
				row_id
			);
		}
	}

	#[tokio::test]
	#[serial]
	async fn test_cursor_pagination_last_page() {
		let db = TestDb::start().await;
		let pool = db.pool();

		let tenant_id = test_helpers::insert_tenant(pool, "cursor-last", None, 0).await;
		let job_id = test_helpers::insert_job(pool, Some(tenant_id), 5, "running").await;

		let mut task_ids = Vec::new();
		for _ in 0..5 {
			let tid = test_helpers::insert_task(
				pool,
				job_id,
				"completed",
				Some(tenant_id),
				Some(test_helpers::safe_result()),
				None,
			)
			.await;
			task_ids.push(tid);
		}

		let limit: i64 = 3;

		// Use cursor pointing at the 3rd task so only 2 remain
		let cursor = task_ids[2];

		let rows = sqlx::query(
			"SELECT id, result FROM v1_task_result WHERE job_id = $1 AND id > $2 ORDER BY id ASC LIMIT $3",
		)
		.bind(job_id)
		.bind(cursor)
		.bind(limit + 1)
		.fetch_all(pool)
		.await
		.expect("last page query failed");

		let has_more = rows.len() as i64 > limit;
		let results: Vec<_> = rows.into_iter().take(limit as usize).collect();

		assert_eq!(
			results.len(),
			2,
			"last page should have only 2 remaining results"
		);
		assert!(!has_more, "has_more should be false on the last page");
	}

	#[tokio::test]
	#[serial]
	async fn test_cursor_with_state_filter() {
		let db = TestDb::start().await;
		let pool = db.pool();

		let tenant_id = test_helpers::insert_tenant(pool, "cursor-filter", None, 0).await;
		let job_id = test_helpers::insert_job(pool, Some(tenant_id), 6, "running").await;

		// Mix of states: 3 completed, 2 failed, 1 queued
		for _ in 0..3 {
			test_helpers::insert_task(
				pool,
				job_id,
				"completed",
				Some(tenant_id),
				Some(test_helpers::safe_result()),
				None,
			)
			.await;
		}
		for _ in 0..2 {
			test_helpers::insert_task(
				pool,
				job_id,
				"failed",
				Some(tenant_id),
				None,
				Some("smtp error"),
			)
			.await;
		}
		test_helpers::insert_task(pool, job_id, "queued", Some(tenant_id), None, None).await;

		// Query only completed tasks
		let completed_rows = sqlx::query(
			"SELECT id, task_state::TEXT AS task_state FROM v1_task_result WHERE job_id = $1 AND task_state = 'completed'::task_state ORDER BY id ASC",
		)
		.bind(job_id)
		.fetch_all(pool)
		.await
		.expect("state filter query failed");

		assert_eq!(
			completed_rows.len(),
			3,
			"only completed tasks should be returned"
		);
		for row in &completed_rows {
			let task_state: String = row.get("task_state");
			assert_eq!(task_state, "completed");
		}
	}
}

// ---------------------------------------------------------------------------
// Dedupe Key Uniqueness Test
// ---------------------------------------------------------------------------

#[cfg(test)]
mod dedupe_tests {
	use super::test_helpers::{self, TestDb};
	use serial_test::serial;

	#[tokio::test]
	#[serial]
	async fn test_dedupe_key_uniqueness() {
		let db = TestDb::start().await;
		let pool = db.pool();

		let tenant_id = test_helpers::insert_tenant(pool, "dedupe", None, 0).await;
		let job_id = test_helpers::insert_job(pool, Some(tenant_id), 2, "running").await;

		let dedupe_key = "user@example.com";

		// First insert with dedupe_key should succeed
		sqlx::query(
			"INSERT INTO v1_task_result (job_id, payload, task_state, tenant_id, dedupe_key) VALUES ($1, $2, 'queued'::task_state, $3, $4)",
		)
		.bind(job_id)
		.bind(serde_json::json!({"input": {"to_email": "user@example.com"}, "job_id": {"bulk": job_id}, "webhook": null}))
		.bind(tenant_id)
		.bind(dedupe_key)
		.execute(pool)
		.await
		.expect("first dedupe insert should succeed");

		// Second insert with the same dedupe_key for the same job should fail
		let duplicate_result = sqlx::query(
			"INSERT INTO v1_task_result (job_id, payload, task_state, tenant_id, dedupe_key) VALUES ($1, $2, 'queued'::task_state, $3, $4)",
		)
		.bind(job_id)
		.bind(serde_json::json!({"input": {"to_email": "user@example.com"}, "job_id": {"bulk": job_id}, "webhook": null}))
		.bind(tenant_id)
		.bind(dedupe_key)
		.execute(pool)
		.await;

		assert!(
			duplicate_result.is_err(),
			"inserting a duplicate dedupe_key for the same job_id should violate the unique constraint"
		);
	}
}

// ---------------------------------------------------------------------------
// Job Completion Detection Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod completion_detection_tests {
	use super::test_helpers::{self, TestDb};
	use serial_test::serial;
	use sqlx::Row;

	#[tokio::test]
	#[serial]
	async fn test_job_completion_check() {
		let db = TestDb::start().await;
		let pool = db.pool();

		let tenant_id = test_helpers::insert_tenant(pool, "complete-check", None, 0).await;
		let total_records = 3;
		let job_id =
			test_helpers::insert_job(pool, Some(tenant_id), total_records, "running").await;

		// Insert 3 tasks, all completed with results
		for _ in 0..3 {
			test_helpers::insert_task(
				pool,
				job_id,
				"completed",
				Some(tenant_id),
				Some(test_helpers::safe_result()),
				None,
			)
			.await;
		}

		// Count tasks that have been processed (result IS NOT NULL OR error IS NOT NULL)
		let row = sqlx::query(
			"SELECT COUNT(*) AS cnt FROM v1_task_result WHERE job_id = $1 AND (result IS NOT NULL OR error IS NOT NULL)",
		)
		.bind(job_id)
		.fetch_one(pool)
		.await
		.expect("processed count query failed");

		let processed_count: i64 = row.get("cnt");

		assert_eq!(
			processed_count, total_records as i64,
			"processed count should equal total_records when all tasks are done"
		);

		// Fetch job total_records for comparison
		let row = sqlx::query("SELECT total_records FROM v1_bulk_job WHERE id = $1")
			.bind(job_id)
			.fetch_one(pool)
			.await
			.expect("fetch total_records failed");

		let job_total: Option<i32> = row.get("total_records");

		assert_eq!(processed_count, job_total.unwrap() as i64);
	}

	#[tokio::test]
	#[serial]
	async fn test_pre_created_tasks_not_counted_as_processed() {
		let db = TestDb::start().await;
		let pool = db.pool();

		let tenant_id = test_helpers::insert_tenant(pool, "pre-created", None, 0).await;
		let job_id = test_helpers::insert_job(pool, Some(tenant_id), 3, "pending").await;

		// Insert 3 queued tasks with NO result and NO error
		for _ in 0..3 {
			test_helpers::insert_task(pool, job_id, "queued", Some(tenant_id), None, None).await;
		}

		// Count processed (result IS NOT NULL OR error IS NOT NULL)
		let row = sqlx::query(
			"SELECT COUNT(*) AS cnt FROM v1_task_result WHERE job_id = $1 AND (result IS NOT NULL OR error IS NOT NULL)",
		)
		.bind(job_id)
		.fetch_one(pool)
		.await
		.expect("processed count query failed");

		let processed_count: i64 = row.get("cnt");

		assert_eq!(
			processed_count, 0,
			"pre-created queued tasks with no result should not count as processed"
		);
	}
}
