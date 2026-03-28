CREATE TYPE pipeline_delivery_status AS ENUM (
    'not_requested',
    'pending',
    'delivered',
    'retry_scheduled',
    'failed'
);

ALTER TABLE v1_pipelines
    ADD COLUMN policy_config JSONB NOT NULL DEFAULT '{}'::jsonb;

ALTER TABLE v1_pipeline_runs
    ADD COLUMN delivery_status pipeline_delivery_status NOT NULL DEFAULT 'not_requested',
    ADD COLUMN delivery_attempts INTEGER NOT NULL DEFAULT 0,
    ADD COLUMN last_delivery_attempt_at TIMESTAMPTZ,
    ADD COLUMN next_delivery_attempt_at TIMESTAMPTZ,
    ADD COLUMN delivery_error TEXT;

CREATE INDEX idx_v1_pipeline_runs_delivery_retry
    ON v1_pipeline_runs (delivery_status, next_delivery_attempt_at)
    WHERE delivery_status = 'retry_scheduled';

CREATE TABLE v1_pipeline_contact_state (
    pipeline_id BIGINT NOT NULL REFERENCES v1_pipelines(id) ON DELETE CASCADE,
    canonical_email TEXT NOT NULL,
    source_hash TEXT NOT NULL,
    last_run_id BIGINT REFERENCES v1_pipeline_runs(id) ON DELETE SET NULL,
    last_result_task_id INTEGER REFERENCES v1_task_result(id) ON DELETE SET NULL,
    last_seen_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_verified_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (pipeline_id, canonical_email)
);

CREATE INDEX idx_v1_pipeline_contact_state_last_verified
    ON v1_pipeline_contact_state (pipeline_id, last_verified_at DESC);

CREATE TRIGGER set_v1_pipeline_contact_state_updated_at
    BEFORE UPDATE ON v1_pipeline_contact_state
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
