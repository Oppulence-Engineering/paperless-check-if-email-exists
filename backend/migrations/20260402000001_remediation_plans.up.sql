CREATE TABLE v1_remediation_plans (
    id                  BIGSERIAL PRIMARY KEY,
    tenant_id           UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    list_id             INTEGER NOT NULL REFERENCES v1_lists(id) ON DELETE CASCADE,
    effective_job_id    INTEGER REFERENCES v1_bulk_job(id) ON DELETE SET NULL,
    rule_version        TEXT NOT NULL,
    options             JSONB NOT NULL DEFAULT '{}'::jsonb,
    options_hash        TEXT NOT NULL,
    result_state_digest TEXT NOT NULL,
    status              TEXT NOT NULL CHECK (status IN ('processing', 'completed', 'failed')),
    summary_counts      JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at        TIMESTAMPTZ
);

CREATE UNIQUE INDEX idx_v1_remediation_plan_identity
    ON v1_remediation_plans (
        tenant_id,
        list_id,
        rule_version,
        result_state_digest,
        options_hash
    );

CREATE INDEX idx_v1_remediation_plans_tenant_list_created
    ON v1_remediation_plans (tenant_id, list_id, created_at DESC);

CREATE TABLE v1_remediation_rows (
    id             BIGSERIAL PRIMARY KEY,
    tenant_id      UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    plan_id        BIGINT NOT NULL REFERENCES v1_remediation_plans(id) ON DELETE CASCADE,
    row_number     INTEGER NOT NULL,
    classification TEXT NOT NULL CHECK (classification IN ('fixed', 'safe', 'review', 'drop')),
    rule_id        TEXT NOT NULL,
    confidence     TEXT NOT NULL CHECK (confidence IN ('high', 'medium', 'low')),
    before         JSONB NOT NULL,
    after          JSONB NOT NULL,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX idx_v1_remediation_rows_plan_row
    ON v1_remediation_rows (plan_id, row_number);

CREATE INDEX idx_v1_remediation_rows_tenant_plan_class
    ON v1_remediation_rows (tenant_id, plan_id, classification, row_number);

CREATE TABLE v1_remediation_exports (
    id         BIGSERIAL PRIMARY KEY,
    tenant_id  UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    plan_id    BIGINT NOT NULL REFERENCES v1_remediation_plans(id) ON DELETE CASCADE,
    format     TEXT NOT NULL DEFAULT 'csv' CHECK (format IN ('csv')),
    partitions JSONB NOT NULL DEFAULT '[]'::jsonb,
    storage_key TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_v1_remediation_exports_tenant_plan
    ON v1_remediation_exports (tenant_id, plan_id, created_at DESC);
