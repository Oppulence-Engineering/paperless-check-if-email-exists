ALTER TABLE v1_task_result
    ADD COLUMN bounce_risk_score SMALLINT,
    ADD COLUMN bounce_risk_category TEXT,
    ADD COLUMN bounce_risk_confidence DOUBLE PRECISION,
    ADD COLUMN bounce_risk_action TEXT,
    ADD COLUMN bounce_risk_model_version TEXT,
    ADD COLUMN bounce_risk_signals JSONB;

CREATE INDEX idx_v1_task_result_tenant_canonical_completed
    ON v1_task_result (tenant_id, canonical_email, completed_at DESC)
    WHERE canonical_email IS NOT NULL;

CREATE TABLE bounce_risk_domain_cache (
    domain TEXT PRIMARY KEY,
    rdap_payload JSONB,
    infra_payload JSONB,
    rdap_fetched_at TIMESTAMPTZ,
    infra_fetched_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
