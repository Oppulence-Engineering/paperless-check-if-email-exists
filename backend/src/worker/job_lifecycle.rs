//! Status transitions shared by everything that creates bulk jobs (bulk API,
//! list uploads, pipelines, reverification, the email finder) and by the
//! worker that completes their tasks.
//!
//! A creator inserts the job as `pending`, inserts and publishes its tasks,
//! then calls [`start_bulk_job`]. (Pipeline runs insert every task row first
//! and move the job to `running` before publishing, which is equally safe.)
//! The worker only finishes jobs that are `running` or `cancelling`. While
//! tasks are still being added the worker can finish every task published so
//! far, and a `pending` job must not be finished then. Every write checks the current status, so a late
//! write never moves a finished job or list back to an open status.

use sqlx::PgPool;

/// Mark a job `running` once its creator has published every task, then
/// finish it if its tasks already finished while it was being published.
/// A job that was cancelled in the meantime keeps its status.
pub async fn start_bulk_job(pool: &PgPool, job_id: i32) -> Result<(), sqlx::Error> {
	sqlx::query(
		"UPDATE v1_bulk_job SET status = 'running'::job_state, updated_at = NOW() WHERE id = $1 AND status = 'pending'",
	)
	.bind(job_id)
	.execute(pool)
	.await?;
	finalize_bulk_job_if_done(pool, job_id).await
}

/// Move a bulk job to its terminal status once none of its tasks can still
/// run. The check and the write are one statement, so a job that a retry has
/// just reopened, or that another worker has already finished, keeps its
/// status. A cancelled job ends as `cancelled`; otherwise one failed or
/// dead-lettered task fails the job.
pub async fn finalize_bulk_job_if_done(pool: &PgPool, job_id: i32) -> Result<(), sqlx::Error> {
	sqlx::query(
		r#"
		UPDATE v1_bulk_job AS job
		SET status = CASE
				WHEN job.status = 'cancelling' THEN 'cancelled'::job_state
				WHEN EXISTS (
					SELECT 1 FROM v1_task_result
					WHERE job_id = $1 AND task_state IN ('failed', 'dead_lettered')
				) THEN 'failed'::job_state
				ELSE 'completed'::job_state
			END,
			completed_at = CASE WHEN job.status = 'cancelling'
				THEN job.completed_at ELSE COALESCE(job.completed_at, NOW()) END,
			cancelled_at = CASE WHEN job.status = 'cancelling'
				THEN COALESCE(job.cancelled_at, NOW()) ELSE job.cancelled_at END,
			updated_at = NOW()
		WHERE job.id = $1
			AND job.status IN ('running', 'cancelling')
			AND NOT EXISTS (
				SELECT 1 FROM v1_task_result
				WHERE job_id = $1 AND task_state IN ('queued', 'running', 'retrying')
			)
		"#,
	)
	.bind(job_id)
	.execute(pool)
	.await?;
	Ok(())
}

/// Finish a list once every row has a result, an error, or was cancelled.
/// A row waiting for a delayed recheck already holds a result but is not done
/// until it leaves 'retrying'. Rows are matched through the list's job, whose
/// index keeps this from scanning every task in the table. A deleted or
/// already finished list keeps its status.
pub async fn finalize_list_if_done(pool: &PgPool, list_id: i32) -> Result<(), sqlx::Error> {
	sqlx::query(
		r#"
		WITH counts AS (
			SELECT
				COUNT(*) FILTER (
					WHERE (t.result IS NOT NULL OR t.error IS NOT NULL OR t.task_state = 'cancelled')
						AND t.task_state NOT IN ('queued', 'running', 'retrying')
				) AS processed,
				COUNT(*) FILTER (WHERE t.task_state IN ('failed', 'dead_lettered')) AS failed
			FROM v1_lists l
			JOIN v1_task_result t
				ON t.job_id = l.job_id AND (t.extra->>'list_id')::INTEGER = l.id
			WHERE l.id = $1
		)
		UPDATE v1_lists AS l
		SET status = CASE WHEN counts.failed > 0
				THEN 'failed'::list_status ELSE 'completed'::list_status END,
			error_message = CASE WHEN counts.failed > 0
				THEN COALESCE(l.error_message, 'task_failed') ELSE l.error_message END,
			completed_at = COALESCE(l.completed_at, NOW()),
			updated_at = NOW()
		FROM counts
		WHERE l.id = $1
			AND l.status IN ('uploading', 'processing')
			AND counts.processed >= l.total_rows
		"#,
	)
	.bind(list_id)
	.execute(pool)
	.await?;
	Ok(())
}

/// Move a list to `processing` after its upload published every row, unless
/// the worker finished it first, then finish it if every row is done.
pub async fn start_list(pool: &PgPool, list_id: i32) -> Result<(), sqlx::Error> {
	sqlx::query(
		"UPDATE v1_lists SET status = 'processing'::list_status, updated_at = NOW() WHERE id = $1 AND status = 'uploading'",
	)
	.bind(list_id)
	.execute(pool)
	.await?;
	finalize_list_if_done(pool, list_id).await
}
