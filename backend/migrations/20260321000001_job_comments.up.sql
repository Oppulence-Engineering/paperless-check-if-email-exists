CREATE TABLE IF NOT EXISTS job_comments (
    id          BIGSERIAL PRIMARY KEY,
    tenant_id   UUID REFERENCES tenants(id) ON DELETE CASCADE,
    job_id      INTEGER REFERENCES v1_bulk_job(id) ON DELETE CASCADE,
    list_id     INTEGER REFERENCES v1_lists(id) ON DELETE CASCADE,
    body        TEXT NOT NULL,
    author      TEXT,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_job_comments_tenant ON job_comments(tenant_id);
CREATE INDEX idx_job_comments_job_id ON job_comments(job_id) WHERE job_id IS NOT NULL;
CREATE INDEX idx_job_comments_list_id ON job_comments(list_id) WHERE list_id IS NOT NULL;
