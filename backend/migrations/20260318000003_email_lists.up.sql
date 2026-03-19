CREATE TYPE list_status AS ENUM (
    'uploading',
    'processing',
    'completed',
    'failed',
    'deleted'
);

CREATE TABLE v1_lists (
    id                SERIAL PRIMARY KEY,
    tenant_id         UUID NOT NULL REFERENCES tenants(id),
    job_id            INTEGER REFERENCES v1_bulk_job(id),
    name              TEXT NOT NULL,
    original_filename TEXT NOT NULL,
    file_size_bytes   BIGINT NOT NULL,
    total_rows        INTEGER NOT NULL DEFAULT 0,
    email_column      TEXT NOT NULL,
    original_headers  TEXT[] NOT NULL DEFAULT '{}',
    original_data     JSONB,
    status            list_status NOT NULL DEFAULT 'uploading',
    error_message     TEXT,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at      TIMESTAMPTZ,
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_v1_lists_tenant_created_at ON v1_lists (tenant_id, created_at DESC);
CREATE INDEX idx_v1_lists_tenant_status ON v1_lists (tenant_id, status);
CREATE INDEX idx_v1_lists_job_id ON v1_lists (job_id);
