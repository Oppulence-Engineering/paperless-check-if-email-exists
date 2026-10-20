CREATE TABLE tenant_rate_windows (
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    window_seconds INTEGER NOT NULL CHECK (window_seconds IN (1, 60, 3600, 86400)),
    used BIGINT NOT NULL CHECK (used >= 0),
    resets_at TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (tenant_id, window_seconds)
);
