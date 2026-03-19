CREATE TABLE reputation_cache (
    id         BIGSERIAL PRIMARY KEY,
    domain     TEXT NOT NULL UNIQUE,
    response   JSONB NOT NULL,
    score      SMALLINT NOT NULL,
    risk_level TEXT NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_reputation_cache_expires_at ON reputation_cache (expires_at);
