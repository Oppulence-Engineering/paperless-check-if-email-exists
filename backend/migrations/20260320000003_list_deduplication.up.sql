-- Add deduplication tracking columns to v1_lists
ALTER TABLE v1_lists
    ADD COLUMN unique_emails INTEGER,
    ADD COLUMN deduplicated_count INTEGER NOT NULL DEFAULT 0;

-- Add deduplication columns to v1_task_result
ALTER TABLE v1_task_result
    ADD COLUMN canonical_email TEXT,
    ADD COLUMN is_duplicate BOOLEAN NOT NULL DEFAULT false,
    ADD COLUMN canonical_task_id INTEGER REFERENCES v1_task_result(id) ON DELETE SET NULL;

CREATE INDEX idx_v1_task_result_canonical_task_id
    ON v1_task_result (canonical_task_id) WHERE canonical_task_id IS NOT NULL;
