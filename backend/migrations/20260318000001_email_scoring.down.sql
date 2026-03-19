DROP INDEX IF EXISTS idx_v1_task_result_score;
DROP INDEX IF EXISTS idx_v1_task_result_score_category;

ALTER TABLE v1_task_result
    DROP COLUMN IF EXISTS sub_reason,
    DROP COLUMN IF EXISTS score_category,
    DROP COLUMN IF EXISTS score;
