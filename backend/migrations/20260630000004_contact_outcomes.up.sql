CREATE TABLE v1_contact_outcomes (
    id BIGSERIAL PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    email TEXT NOT NULL,
    canonical_email TEXT NOT NULL,
    provider TEXT NOT NULL,
    event_type TEXT NOT NULL,
    source_key TEXT,
    campaign_id TEXT,
    occurred_at TIMESTAMPTZ NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_v1_contact_outcomes_canonical
    ON v1_contact_outcomes (tenant_id, canonical_email, occurred_at DESC);

CREATE INDEX idx_v1_contact_outcomes_source
    ON v1_contact_outcomes (tenant_id, source_key, event_type)
    WHERE source_key IS NOT NULL;

CREATE INDEX idx_v1_contact_outcomes_event
    ON v1_contact_outcomes (tenant_id, event_type, occurred_at DESC);
