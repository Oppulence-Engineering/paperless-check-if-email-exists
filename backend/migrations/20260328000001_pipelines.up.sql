CREATE TYPE pipeline_status AS ENUM ('active', 'paused', 'deleted');
CREATE TYPE pipeline_source_type AS ENUM ('list_snapshot', 'integration', 'push', 'bucket');
CREATE TYPE pipeline_run_status AS ENUM (
    'queued',
    'preparing',
    'fetching_source',
    'publishing',
    'running',
    'delivering',
    'completed',
    'failed',
    'cancelled',
    'skipped'
);

CREATE TABLE v1_pipelines (
    id BIGSERIAL PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    status pipeline_status NOT NULL DEFAULT 'active',
    source_type pipeline_source_type NOT NULL,
    source_config JSONB NOT NULL,
    schedule_cron TEXT NOT NULL,
    schedule_timezone TEXT NOT NULL DEFAULT 'UTC',
    verification_settings JSONB NOT NULL DEFAULT '{}'::jsonb,
    delivery_config JSONB NOT NULL DEFAULT '{}'::jsonb,
    next_run_at TIMESTAMPTZ,
    last_scheduled_at TIMESTAMPTZ,
    last_run_id BIGINT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ
);

CREATE INDEX idx_v1_pipelines_tenant_status
    ON v1_pipelines (tenant_id, status, created_at DESC);
CREATE INDEX idx_v1_pipelines_due
    ON v1_pipelines (status, next_run_at)
    WHERE deleted_at IS NULL;

CREATE TABLE v1_pipeline_runs (
    id BIGSERIAL PRIMARY KEY,
    pipeline_id BIGINT NOT NULL REFERENCES v1_pipelines(id) ON DELETE CASCADE,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    trigger_type TEXT NOT NULL,
    status pipeline_run_status NOT NULL DEFAULT 'queued',
    scheduled_for TIMESTAMPTZ,
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    job_id INTEGER REFERENCES v1_bulk_job(id) ON DELETE SET NULL,
    list_id INTEGER REFERENCES v1_lists(id) ON DELETE SET NULL,
    source_snapshot JSONB NOT NULL,
    stats JSONB NOT NULL DEFAULT '{}'::jsonb,
    billed_emails INTEGER NOT NULL DEFAULT 0,
    result_location JSONB,
    error_code TEXT,
    error_message TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_v1_pipeline_runs_pipeline_created
    ON v1_pipeline_runs (pipeline_id, created_at DESC);
CREATE INDEX idx_v1_pipeline_runs_tenant_status
    ON v1_pipeline_runs (tenant_id, status, created_at DESC);
CREATE UNIQUE INDEX idx_v1_pipeline_runs_no_overlap
    ON v1_pipeline_runs (pipeline_id)
    WHERE status IN ('preparing', 'fetching_source', 'publishing', 'running', 'delivering');

ALTER TABLE v1_lists
    ADD COLUMN source_list_id INTEGER REFERENCES v1_lists(id) ON DELETE SET NULL,
    ADD COLUMN pipeline_id BIGINT REFERENCES v1_pipelines(id) ON DELETE SET NULL,
    ADD COLUMN pipeline_run_id BIGINT REFERENCES v1_pipeline_runs(id) ON DELETE SET NULL;

CREATE INDEX idx_v1_lists_pipeline_id ON v1_lists (pipeline_id);
CREATE INDEX idx_v1_lists_pipeline_run_id ON v1_lists (pipeline_run_id);

CREATE TABLE v1_usage_events (
    id BIGSERIAL PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    pipeline_id BIGINT REFERENCES v1_pipelines(id) ON DELETE SET NULL,
    pipeline_run_id BIGINT REFERENCES v1_pipeline_runs(id) ON DELETE SET NULL,
    job_id INTEGER REFERENCES v1_bulk_job(id) ON DELETE SET NULL,
    source TEXT NOT NULL,
    reserved_emails INTEGER NOT NULL DEFAULT 0,
    committed_emails INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_v1_usage_events_tenant_created
    ON v1_usage_events (tenant_id, created_at DESC);
CREATE INDEX idx_v1_usage_events_pipeline_run
    ON v1_usage_events (pipeline_run_id);

CREATE TRIGGER set_v1_pipelines_updated_at
    BEFORE UPDATE ON v1_pipelines
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER set_v1_pipeline_runs_updated_at
    BEFORE UPDATE ON v1_pipeline_runs
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER set_v1_usage_events_updated_at
    BEFORE UPDATE ON v1_usage_events
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
