CREATE TABLE v1_finder_job (
    id                    SERIAL PRIMARY KEY,
    tenant_id             UUID NOT NULL REFERENCES tenants(id),
    bulk_job_id           INTEGER NOT NULL REFERENCES v1_bulk_job(id) ON DELETE CASCADE,
    first_name            TEXT NOT NULL,
    last_name             TEXT NOT NULL,
    domain                TEXT NOT NULL,
    normalized_first_name TEXT NOT NULL,
    normalized_last_name  TEXT NOT NULL,
    status                job_state NOT NULL DEFAULT 'pending',
    domain_has_mx         BOOLEAN NOT NULL DEFAULT false,
    domain_is_catch_all   BOOLEAN NOT NULL DEFAULT false,
    candidates_checked    INTEGER NOT NULL DEFAULT 0,
    best_match_email      TEXT,
    best_match_score      SMALLINT,
    best_match_confidence TEXT,
    created_at            TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at            TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at          TIMESTAMPTZ
);

CREATE INDEX idx_v1_finder_job_tenant_created_at ON v1_finder_job (tenant_id, created_at DESC);
CREATE INDEX idx_v1_finder_job_bulk_job_id ON v1_finder_job (bulk_job_id);

CREATE TABLE v1_finder_result (
    id             SERIAL PRIMARY KEY,
    finder_job_id  INTEGER NOT NULL REFERENCES v1_finder_job(id) ON DELETE CASCADE,
    task_result_id INTEGER REFERENCES v1_task_result(id) ON DELETE SET NULL,
    candidate_email TEXT NOT NULL,
    pattern        TEXT NOT NULL,
    rank_position  INTEGER,
    score          SMALLINT,
    score_category TEXT,
    sub_reason     TEXT,
    result         JSONB,
    created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at     TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_v1_finder_result_job_rank ON v1_finder_result (finder_job_id, rank_position);
CREATE INDEX idx_v1_finder_result_task ON v1_finder_result (task_result_id);
