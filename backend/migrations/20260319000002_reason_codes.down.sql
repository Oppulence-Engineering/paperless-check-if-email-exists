DROP INDEX IF EXISTS idx_v1_task_result_reason_codes;
ALTER TABLE v1_task_result DROP COLUMN IF EXISTS reason_codes;
