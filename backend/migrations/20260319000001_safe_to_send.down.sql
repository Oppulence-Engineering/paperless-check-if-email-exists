DROP INDEX IF EXISTS idx_v1_task_result_safe_to_send;
ALTER TABLE v1_task_result DROP COLUMN IF EXISTS safe_to_send;
