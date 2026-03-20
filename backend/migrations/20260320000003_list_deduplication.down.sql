DROP INDEX IF EXISTS idx_v1_task_result_canonical_task_id;
ALTER TABLE v1_task_result
    DROP COLUMN IF EXISTS canonical_email,
    DROP COLUMN IF EXISTS is_duplicate,
    DROP COLUMN IF EXISTS canonical_task_id;
ALTER TABLE v1_lists
    DROP COLUMN IF EXISTS unique_emails,
    DROP COLUMN IF EXISTS deduplicated_count;
