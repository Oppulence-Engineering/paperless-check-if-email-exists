CREATE TABLE verification_delayed_rechecks (
    id BIGSERIAL PRIMARY KEY,
    task_result_id INTEGER NOT NULL REFERENCES v1_task_result(id) ON DELETE CASCADE,
    job_id INTEGER NOT NULL REFERENCES v1_bulk_job(id) ON DELETE CASCADE,
    tenant_id UUID REFERENCES tenants(id),
    task JSONB NOT NULL,
    retry_count INTEGER NOT NULL,
    run_at TIMESTAMPTZ NOT NULL,
    status TEXT NOT NULL DEFAULT 'scheduled',
    publish_attempts INTEGER NOT NULL DEFAULT 0,
    last_error TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    published_at TIMESTAMPTZ,
    CONSTRAINT verification_delayed_rechecks_status_check CHECK (
        status IN ('scheduled', 'publishing', 'published', 'failed', 'cancelled')
    )
);

CREATE INDEX idx_verification_delayed_rechecks_due
    ON verification_delayed_rechecks (run_at, id)
    WHERE status = 'scheduled';

CREATE INDEX idx_verification_delayed_rechecks_task
    ON verification_delayed_rechecks (task_result_id, status);

CREATE UNIQUE INDEX idx_verification_delayed_rechecks_one_scheduled
    ON verification_delayed_rechecks (task_result_id)
    WHERE status = 'scheduled';
