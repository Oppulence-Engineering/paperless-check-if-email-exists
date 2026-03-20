-- Scheduled re-verification schedules per tenant
CREATE TABLE reverification_schedules (
    id              SERIAL PRIMARY KEY,
    tenant_id       UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    enabled         BOOLEAN NOT NULL DEFAULT true,
    staleness_days  INTEGER NOT NULL DEFAULT 30,
    batch_size      INTEGER NOT NULL DEFAULT 100,
    last_run_at     TIMESTAMPTZ,
    next_run_at     TIMESTAMPTZ,
    last_job_id     INTEGER REFERENCES v1_bulk_job(id),
    emails_requeued INTEGER NOT NULL DEFAULT 0,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE UNIQUE INDEX idx_reverification_schedules_tenant ON reverification_schedules (tenant_id);

-- Index to efficiently find stale completed results for a tenant
CREATE INDEX idx_v1_task_result_reverification
    ON v1_task_result (tenant_id, completed_at)
    WHERE task_state = 'completed' AND completed_at IS NOT NULL;
