CREATE TYPE suppression_reason AS ENUM (
    'manual',
    'bounce',
    'invalid',
    'spam_trap',
    'unsubscribe',
    'complaint',
    'auto_invalid'
);

CREATE TABLE v1_suppression_entries (
    id            SERIAL PRIMARY KEY,
    tenant_id     UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    email         TEXT NOT NULL,
    reason        suppression_reason NOT NULL DEFAULT 'manual',
    source        TEXT,
    notes         TEXT,
    created_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (tenant_id, email)
);

CREATE INDEX idx_v1_suppression_tenant_email ON v1_suppression_entries (tenant_id, email);
CREATE INDEX idx_v1_suppression_tenant_created ON v1_suppression_entries (tenant_id, created_at DESC);
CREATE INDEX idx_v1_suppression_tenant_reason ON v1_suppression_entries (tenant_id, reason);
