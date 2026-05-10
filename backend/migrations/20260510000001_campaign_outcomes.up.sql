CREATE TYPE outcome_type AS ENUM (
    'delivered',
    'hard_bounce',
    'soft_bounce',
    'complaint',
    'open',
    'click',
    'unsubscribe'
);

CREATE TABLE verification_outcomes (
    id              BIGSERIAL PRIMARY KEY,
    tenant_id       UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    canonical_email TEXT NOT NULL,
    outcome_type    outcome_type NOT NULL,
    occurred_at     TIMESTAMPTZ NOT NULL,
    source          TEXT NOT NULL DEFAULT '',
    campaign_id     TEXT,
    metadata        JSONB,
    task_result_id  INTEGER REFERENCES v1_task_result(id) ON DELETE SET NULL,
    policy_action   TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (tenant_id, canonical_email, outcome_type, occurred_at, source)
);

CREATE INDEX idx_verification_outcomes_lookup
    ON verification_outcomes (tenant_id, canonical_email, occurred_at DESC);

CREATE INDEX idx_verification_outcomes_recent
    ON verification_outcomes (tenant_id, occurred_at DESC);

CREATE INDEX idx_verification_outcomes_campaign
    ON verification_outcomes (tenant_id, campaign_id)
    WHERE campaign_id IS NOT NULL;

CREATE TABLE v1_outcome_policies (
    id          BIGSERIAL PRIMARY KEY,
    tenant_id   UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name        TEXT NOT NULL,
    is_default  BOOLEAN NOT NULL DEFAULT false,
    rules       JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (tenant_id, name)
);

CREATE UNIQUE INDEX idx_outcome_policies_one_default
    ON v1_outcome_policies (tenant_id)
    WHERE is_default = true;
