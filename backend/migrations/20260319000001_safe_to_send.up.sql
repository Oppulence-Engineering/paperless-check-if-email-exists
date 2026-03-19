ALTER TABLE v1_task_result ADD COLUMN safe_to_send BOOLEAN;
CREATE INDEX idx_v1_task_result_safe_to_send ON v1_task_result (job_id, safe_to_send) WHERE safe_to_send IS NOT NULL;
