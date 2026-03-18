CREATE TABLE idempotency_keys (
    id                   BIGSERIAL PRIMARY KEY,
    tenant_id            TEXT NOT NULL,
    idempotency_key      TEXT NOT NULL,
    request_path         TEXT NOT NULL,
    request_body_hash    BYTEA NOT NULL,
    status               TEXT NOT NULL DEFAULT 'processing'
                         CHECK (status IN ('processing', 'completed', 'failed')),
    response_status_code SMALLINT,
    response_body        BYTEA,
    response_headers     JSONB,
    created_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at           TIMESTAMPTZ NOT NULL DEFAULT NOW() + INTERVAL '24 hours',
    locked_at            TIMESTAMPTZ,
    locked_by            TEXT,
    CONSTRAINT uq_tenant_idempotency_key UNIQUE (tenant_id, idempotency_key)
);
CREATE INDEX idx_idempotency_keys_expires ON idempotency_keys (expires_at)
    WHERE status != 'processing';
