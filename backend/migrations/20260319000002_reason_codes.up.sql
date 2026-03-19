ALTER TABLE v1_task_result ADD COLUMN reason_codes TEXT[];
CREATE INDEX idx_v1_task_result_reason_codes ON v1_task_result USING GIN (reason_codes) WHERE reason_codes IS NOT NULL;
