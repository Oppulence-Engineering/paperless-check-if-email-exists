ALTER TABLE v1_suppression_entries
    ADD COLUMN canonical_email TEXT,
    ADD COLUMN status TEXT NOT NULL DEFAULT 'active',
    ADD COLUMN reason_code TEXT,
    ADD COLUMN reason_detail TEXT,
    ADD COLUMN source_type TEXT,
    ADD COLUMN source_ref TEXT,
    ADD COLUMN created_by TEXT,
    ADD COLUMN expires_at TIMESTAMPTZ,
    ADD COLUMN last_seen_at TIMESTAMPTZ,
    ADD COLUMN metadata JSONB NOT NULL DEFAULT '{}'::jsonb;

UPDATE v1_suppression_entries
SET canonical_email = lower(trim(email)),
    reason_code = COALESCE(reason::TEXT, 'manual'),
    source_type = COALESCE(source, 'manual'),
    last_seen_at = created_at
WHERE canonical_email IS NULL;

ALTER TABLE v1_suppression_entries
    ALTER COLUMN canonical_email SET NOT NULL;

WITH ranked_entries AS (
    SELECT
        id,
        ROW_NUMBER() OVER (
            PARTITION BY tenant_id, canonical_email
            ORDER BY created_at DESC, id DESC
        ) AS row_number
    FROM v1_suppression_entries
    WHERE status = 'active'
)
UPDATE v1_suppression_entries
SET status = 'merged',
    reason_detail = COALESCE(reason_detail, 'Merged during canonical suppression migration')
FROM ranked_entries
WHERE v1_suppression_entries.id = ranked_entries.id
  AND ranked_entries.row_number > 1;

CREATE UNIQUE INDEX idx_v1_suppression_active_canonical
    ON v1_suppression_entries (tenant_id, canonical_email)
    WHERE status = 'active';

CREATE TABLE v1_suppression_events (
    id BIGSERIAL PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    entry_id INTEGER REFERENCES v1_suppression_entries(id) ON DELETE SET NULL,
    canonical_email TEXT NOT NULL,
    event_type TEXT NOT NULL,
    from_status TEXT,
    to_status TEXT,
    reason_code TEXT,
    reason_detail TEXT,
    source_type TEXT,
    source_ref TEXT,
    actor_type TEXT NOT NULL DEFAULT 'api',
    actor_id TEXT,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_v1_suppression_events_entry
    ON v1_suppression_events (tenant_id, entry_id, created_at DESC);

CREATE TABLE v1_remediation_plans (
    id BIGSERIAL PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    list_id INTEGER NOT NULL REFERENCES v1_lists(id) ON DELETE CASCADE,
    job_id INTEGER REFERENCES v1_bulk_job(id) ON DELETE SET NULL,
    status TEXT NOT NULL DEFAULT 'completed',
    rule_version TEXT NOT NULL,
    options JSONB NOT NULL,
    result_state_digest TEXT NOT NULL,
    summary_counts JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (tenant_id, list_id, rule_version, result_state_digest, options)
);

CREATE TABLE v1_remediation_rows (
    id BIGSERIAL PRIMARY KEY,
    plan_id BIGINT NOT NULL REFERENCES v1_remediation_plans(id) ON DELETE CASCADE,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    list_id INTEGER NOT NULL REFERENCES v1_lists(id) ON DELETE CASCADE,
    task_result_id INTEGER REFERENCES v1_task_result(id) ON DELETE SET NULL,
    row_index INTEGER NOT NULL,
    classification TEXT NOT NULL,
    rule_id TEXT NOT NULL,
    confidence TEXT NOT NULL,
    original_email TEXT NOT NULL,
    effective_email TEXT NOT NULL,
    before JSONB NOT NULL,
    after JSONB NOT NULL,
    reasons JSONB NOT NULL DEFAULT '[]'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (plan_id, row_index)
);

CREATE INDEX idx_v1_remediation_rows_partition
    ON v1_remediation_rows (tenant_id, plan_id, classification, row_index);

CREATE TABLE v1_remediation_exports (
    id BIGSERIAL PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    plan_id BIGINT NOT NULL REFERENCES v1_remediation_plans(id) ON DELETE CASCADE,
    partitions TEXT[] NOT NULL,
    format TEXT NOT NULL DEFAULT 'csv',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
