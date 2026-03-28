DROP TRIGGER IF EXISTS set_v1_pipeline_contact_state_updated_at ON v1_pipeline_contact_state;

DROP TABLE IF EXISTS v1_pipeline_contact_state;

DROP INDEX IF EXISTS idx_v1_pipeline_runs_delivery_retry;

ALTER TABLE v1_pipeline_runs
    DROP COLUMN IF EXISTS delivery_error,
    DROP COLUMN IF EXISTS next_delivery_attempt_at,
    DROP COLUMN IF EXISTS last_delivery_attempt_at,
    DROP COLUMN IF EXISTS delivery_attempts,
    DROP COLUMN IF EXISTS delivery_status;

ALTER TABLE v1_pipelines
    DROP COLUMN IF EXISTS policy_config;

DROP TYPE IF EXISTS pipeline_delivery_status;
