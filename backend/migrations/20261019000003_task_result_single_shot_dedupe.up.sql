-- One row per dedupe key for single-shot tasks. idx_v1_task_result_dedupe
-- is unique on (job_id, dedupe_key), and NULL job_ids never collide, so two
-- SQS deliveries of one message could each insert a row. With this index the
-- second insert fails and SQS redelivers, and that delivery updates the row.
CREATE UNIQUE INDEX IF NOT EXISTS idx_v1_task_result_single_shot_dedupe
    ON v1_task_result (dedupe_key)
    WHERE job_id IS NULL AND dedupe_key IS NOT NULL;
