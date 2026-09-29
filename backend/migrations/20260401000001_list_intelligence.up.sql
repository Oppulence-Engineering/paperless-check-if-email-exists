CREATE TABLE verification_change_events (
    id                      BIGSERIAL PRIMARY KEY,
    tenant_id               UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    canonical_email         TEXT NOT NULL,
    current_task_result_id  INTEGER NOT NULL REFERENCES v1_task_result(id) ON DELETE CASCADE,
    previous_task_result_id INTEGER REFERENCES v1_task_result(id) ON DELETE SET NULL,
    current_list_id         INTEGER REFERENCES v1_lists(id) ON DELETE SET NULL,
    previous_list_id        INTEGER REFERENCES v1_lists(id) ON DELETE SET NULL,
    current_pipeline_run_id BIGINT REFERENCES v1_pipeline_runs(id) ON DELETE SET NULL,
    previous_pipeline_run_id BIGINT REFERENCES v1_pipeline_runs(id) ON DELETE SET NULL,
    current_score           SMALLINT,
    previous_score          SMALLINT,
    current_category        TEXT,
    previous_category       TEXT,
    current_safe_to_send    BOOLEAN,
    previous_safe_to_send   BOOLEAN,
    current_reason_codes    TEXT[],
    previous_reason_codes   TEXT[],
    change_type             TEXT NOT NULL CHECK (change_type IN (
        'new',
        'improved',
        'degraded',
        'became_invalid',
        'became_risky',
        'became_safe',
        'unchanged'
    )),
    created_at              TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (current_task_result_id)
);

CREATE INDEX idx_verification_change_events_tenant_created
    ON verification_change_events (tenant_id, created_at DESC);
CREATE INDEX idx_verification_change_events_tenant_canonical
    ON verification_change_events (tenant_id, canonical_email, created_at DESC);
CREATE INDEX idx_verification_change_events_type
    ON verification_change_events (tenant_id, change_type, created_at DESC);

CREATE TABLE v1_alerts (
    id              BIGSERIAL PRIMARY KEY,
    tenant_id       UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    type            TEXT NOT NULL,
    status          TEXT NOT NULL DEFAULT 'unread' CHECK (status IN ('unread', 'read', 'dismissed')),
    change_event_id BIGINT UNIQUE REFERENCES verification_change_events(id) ON DELETE CASCADE,
    canonical_email TEXT NOT NULL,
    title           TEXT NOT NULL,
    body            TEXT,
    metadata        JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_v1_alerts_tenant_created
    ON v1_alerts (tenant_id, created_at DESC);
CREATE INDEX idx_v1_alerts_tenant_status
    ON v1_alerts (tenant_id, status, created_at DESC);
CREATE INDEX idx_v1_alerts_tenant_type
    ON v1_alerts (tenant_id, type, created_at DESC);

CREATE TABLE v1_score_policies (
    id          BIGSERIAL PRIMARY KEY,
    tenant_id   UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name        TEXT NOT NULL,
    is_default  BOOLEAN NOT NULL DEFAULT false,
    rules       JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (tenant_id, name)
);

CREATE UNIQUE INDEX idx_v1_score_policies_one_default
    ON v1_score_policies (tenant_id)
    WHERE is_default = true;
CREATE INDEX idx_v1_score_policies_tenant_created
    ON v1_score_policies (tenant_id, created_at DESC);

ALTER TABLE v1_lists
    ADD COLUMN policy_id BIGINT REFERENCES v1_score_policies(id) ON DELETE SET NULL;

CREATE INDEX idx_v1_lists_policy_id ON v1_lists (policy_id);

CREATE TABLE v1_saved_segments (
    id          BIGSERIAL PRIMARY KEY,
    tenant_id   UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    name        TEXT NOT NULL,
    scope       TEXT NOT NULL DEFAULT 'lists',
    filter      JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (tenant_id, name)
);

CREATE INDEX idx_v1_saved_segments_tenant_scope
    ON v1_saved_segments (tenant_id, scope, created_at DESC);

CREATE TRIGGER set_v1_alerts_updated_at
    BEFORE UPDATE ON v1_alerts
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER set_v1_score_policies_updated_at
    BEFORE UPDATE ON v1_score_policies
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER set_v1_saved_segments_updated_at
    BEFORE UPDATE ON v1_saved_segments
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
