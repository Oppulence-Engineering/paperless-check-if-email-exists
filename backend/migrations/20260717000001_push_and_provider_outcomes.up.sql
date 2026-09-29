CREATE TABLE v1_pipeline_push_batches (
    id BIGSERIAL PRIMARY KEY,
    pipeline_id BIGINT NOT NULL REFERENCES v1_pipelines(id) ON DELETE CASCADE,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    idempotency_key TEXT NOT NULL,
    source_key TEXT,
    email_column TEXT NOT NULL DEFAULT 'email',
    rows JSONB NOT NULL,
    row_count INTEGER NOT NULL,
    run_id BIGINT REFERENCES v1_pipeline_runs(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (pipeline_id, idempotency_key)
);

CREATE INDEX idx_v1_pipeline_push_batches_tenant_created
    ON v1_pipeline_push_batches (tenant_id, created_at DESC);

CREATE TABLE v1_provider_endpoints (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    provider TEXT NOT NULL,
    label TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'active',
    delivery_token_hash TEXT NOT NULL,
    provider_config JSONB NOT NULL DEFAULT '{}'::jsonb,
    allowed_ips TEXT[] NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at TIMESTAMPTZ,
    CHECK (provider IN ('sendgrid', 'ses', 'mailgun', 'postmark')),
    CHECK (status IN ('active', 'paused', 'disabled'))
);

CREATE INDEX idx_v1_provider_endpoints_tenant_status
    ON v1_provider_endpoints (tenant_id, status, created_at DESC)
    WHERE deleted_at IS NULL;

CREATE TABLE v1_provider_outcome_receipts (
    id UUID PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    endpoint_id UUID NOT NULL REFERENCES v1_provider_endpoints(id) ON DELETE CASCADE,
    provider TEXT NOT NULL,
    request_headers JSONB NOT NULL DEFAULT '{}'::jsonb,
    raw_payload JSONB,
    payload_sha256 TEXT NOT NULL,
    validation_status TEXT NOT NULL,
    validation_error TEXT,
    normalized_count INTEGER NOT NULL DEFAULT 0,
    forward_status TEXT NOT NULL DEFAULT 'not_configured',
    forward_attempts INTEGER NOT NULL DEFAULT 0,
    forward_error TEXT,
    forwarded_at TIMESTAMPTZ,
    received_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_v1_provider_receipts_tenant_received
    ON v1_provider_outcome_receipts (tenant_id, received_at DESC);

ALTER TABLE v1_contact_outcomes
    ADD COLUMN endpoint_id UUID REFERENCES v1_provider_endpoints(id) ON DELETE SET NULL,
    ADD COLUMN receipt_id UUID REFERENCES v1_provider_outcome_receipts(id) ON DELETE SET NULL,
    ADD COLUMN provider_event_id TEXT,
    ADD COLUMN provider_message_id TEXT,
    ADD COLUMN event_family TEXT,
    ADD COLUMN correlation_status TEXT NOT NULL DEFAULT 'unmatched';

CREATE UNIQUE INDEX idx_v1_contact_outcomes_provider_event
    ON v1_contact_outcomes (tenant_id, provider, provider_event_id)
    WHERE provider_event_id IS NOT NULL;

CREATE INDEX idx_v1_contact_outcomes_receipt
    ON v1_contact_outcomes (receipt_id);
