DROP TABLE IF EXISTS job_events;

DROP INDEX IF EXISTS idx_v1_task_result_dedupe;
DROP INDEX IF EXISTS idx_v1_task_result_state;
DROP INDEX IF EXISTS idx_v1_task_result_cursor;

ALTER TABLE v1_task_result
    DROP COLUMN IF EXISTS task_state,
    DROP COLUMN IF EXISTS tenant_id,
    DROP COLUMN IF EXISTS request_id,
    DROP COLUMN IF EXISTS correlation_id,
    DROP COLUMN IF EXISTS dedupe_key,
    DROP COLUMN IF EXISTS retry_count,
    DROP COLUMN IF EXISTS max_retries,
    DROP COLUMN IF EXISTS started_at,
    DROP COLUMN IF EXISTS completed_at,
    DROP COLUMN IF EXISTS updated_at;

DROP INDEX IF EXISTS idx_v1_bulk_job_tenant_status;

ALTER TABLE v1_bulk_job
    DROP COLUMN IF EXISTS tenant_id,
    DROP COLUMN IF EXISTS status,
    DROP COLUMN IF EXISTS request_id,
    DROP COLUMN IF EXISTS correlation_id,
    DROP COLUMN IF EXISTS created_by,
    DROP COLUMN IF EXISTS cancelled_at,
    DROP COLUMN IF EXISTS completed_at,
    DROP COLUMN IF EXISTS updated_at,
    DROP COLUMN IF EXISTS metadata;

DROP TYPE IF EXISTS job_state;
DROP TYPE IF EXISTS task_state;
