ALTER TABLE v1_task_result
    ADD COLUMN score SMALLINT,
    ADD COLUMN score_category TEXT,
    ADD COLUMN sub_reason TEXT;

CREATE INDEX idx_v1_task_result_score_category ON v1_task_result (job_id, score_category);
CREATE INDEX idx_v1_task_result_score ON v1_task_result (job_id, score DESC);
