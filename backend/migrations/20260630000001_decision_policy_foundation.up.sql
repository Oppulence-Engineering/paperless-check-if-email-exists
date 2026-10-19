ALTER TABLE tenants
    ADD COLUMN default_policy_mode TEXT NOT NULL DEFAULT 'deliverability';

CREATE TABLE tenant_policy_profiles (
    id SERIAL PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    profile_key TEXT NOT NULL,
    display_name TEXT NOT NULL,
    rules JSONB NOT NULL,
    status TEXT NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (tenant_id, profile_key)
);

ALTER TABLE v1_bulk_job
    ADD COLUMN policy_mode TEXT,
    ADD COLUMN policy_profile_key TEXT,
    ADD COLUMN source_key TEXT;

ALTER TABLE v1_lists
    ADD COLUMN policy_mode TEXT,
    ADD COLUMN policy_profile_key TEXT,
    ADD COLUMN source_key TEXT;

ALTER TABLE v1_task_result
    ADD COLUMN recommendation JSONB,
    ADD COLUMN recommendation_action TEXT,
    ADD COLUMN recommendation_confidence TEXT,
    ADD COLUMN recommendation_priority TEXT,
    ADD COLUMN policy_mode TEXT,
    ADD COLUMN policy_profile_key TEXT,
    ADD COLUMN policy_evaluation JSONB,
    ADD COLUMN policy_decision TEXT,
    ADD COLUMN policy_evaluated_at TIMESTAMPTZ,
    ADD COLUMN source_key TEXT;

CREATE INDEX idx_v1_task_result_recommendation_action
    ON v1_task_result (tenant_id, recommendation_action)
    WHERE recommendation_action IS NOT NULL;

CREATE INDEX idx_v1_task_result_policy_decision
    ON v1_task_result (tenant_id, policy_decision)
    WHERE policy_decision IS NOT NULL;

CREATE INDEX idx_v1_task_result_source_key
    ON v1_task_result (tenant_id, source_key)
    WHERE source_key IS NOT NULL;
