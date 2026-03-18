CREATE TYPE task_state AS ENUM (
    'queued', 'running', 'completed', 'retrying', 'failed', 'cancelled', 'dead_lettered'
);
CREATE TYPE job_state AS ENUM (
    'pending', 'running', 'completed', 'cancelling', 'cancelled', 'failed'
);

-- Enhance v1_bulk_job
ALTER TABLE v1_bulk_job
    ADD COLUMN tenant_id      UUID REFERENCES tenants(id),
    ADD COLUMN status         job_state NOT NULL DEFAULT 'pending',
    ADD COLUMN request_id     UUID,
    ADD COLUMN correlation_id TEXT,
    ADD COLUMN created_by     TEXT,
    ADD COLUMN cancelled_at   TIMESTAMPTZ,
    ADD COLUMN completed_at   TIMESTAMPTZ,
    ADD COLUMN updated_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ADD COLUMN metadata       JSONB;

-- Backfill existing rows (only count rows that have been actually processed)
UPDATE v1_bulk_job SET status = 'completed'
WHERE (SELECT COUNT(*) FROM v1_task_result WHERE v1_task_result.job_id = v1_bulk_job.id AND (result IS NOT NULL OR error IS NOT NULL))
      >= v1_bulk_job.total_records;
UPDATE v1_bulk_job SET status = 'running'
WHERE status = 'pending'
  AND (SELECT COUNT(*) FROM v1_task_result WHERE v1_task_result.job_id = v1_bulk_job.id AND (result IS NOT NULL OR error IS NOT NULL)) > 0;

CREATE INDEX idx_v1_bulk_job_tenant_status ON v1_bulk_job (tenant_id, status);

-- Enhance v1_task_result
ALTER TABLE v1_task_result
    ADD COLUMN task_state      task_state NOT NULL DEFAULT 'queued',
    ADD COLUMN tenant_id       UUID REFERENCES tenants(id),
    ADD COLUMN request_id      UUID,
    ADD COLUMN correlation_id  TEXT,
    ADD COLUMN dedupe_key      TEXT,
    ADD COLUMN retry_count     INTEGER NOT NULL DEFAULT 0,
    ADD COLUMN max_retries     INTEGER NOT NULL DEFAULT 2,
    ADD COLUMN started_at      TIMESTAMPTZ,
    ADD COLUMN completed_at    TIMESTAMPTZ,
    ADD COLUMN updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW();

-- Backfill existing rows
UPDATE v1_task_result SET task_state = 'completed' WHERE result IS NOT NULL;
UPDATE v1_task_result SET task_state = 'failed' WHERE error IS NOT NULL AND result IS NULL;

CREATE INDEX idx_v1_task_result_cursor ON v1_task_result (job_id, id);
CREATE INDEX idx_v1_task_result_state ON v1_task_result (job_id, task_state);
CREATE UNIQUE INDEX idx_v1_task_result_dedupe ON v1_task_result (job_id, dedupe_key)
    WHERE dedupe_key IS NOT NULL;

-- Event log
CREATE TABLE job_events (
    id         BIGSERIAL PRIMARY KEY,
    job_id     INTEGER NOT NULL REFERENCES v1_bulk_job(id) ON DELETE CASCADE,
    task_id    INTEGER REFERENCES v1_task_result(id) ON DELETE SET NULL,
    event_type TEXT NOT NULL,
    event_data JSONB,
    actor      TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_job_events_job_id ON job_events (job_id, created_at);
