CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TYPE tenant_status AS ENUM ('active', 'suspended', 'deactivated');
CREATE TYPE plan_tier AS ENUM ('free', 'starter', 'professional', 'enterprise');
CREATE TYPE api_key_status AS ENUM ('active', 'revoked', 'expired');

CREATE TABLE tenants (
    id                       UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name                     TEXT NOT NULL,
    slug                     TEXT NOT NULL UNIQUE,
    contact_email            TEXT NOT NULL,
    plan_name                TEXT NOT NULL DEFAULT 'free',
    plan_tier                plan_tier NOT NULL DEFAULT 'free',
    max_requests_per_second  INTEGER,
    max_requests_per_minute  INTEGER,
    max_requests_per_hour    INTEGER,
    max_requests_per_day     INTEGER,
    monthly_email_limit      INTEGER,
    used_this_period         INTEGER NOT NULL DEFAULT 0,
    period_reset_at          TIMESTAMPTZ NOT NULL DEFAULT (date_trunc('month', NOW()) + INTERVAL '1 month'),
    status                   tenant_status NOT NULL DEFAULT 'active',
    default_webhook_url      TEXT,
    webhook_signing_secret   TEXT,
    result_retention_days    INTEGER NOT NULL DEFAULT 30,
    created_at               TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at               TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_tenants_slug ON tenants (slug);
CREATE INDEX idx_tenants_status ON tenants (status);

CREATE TABLE api_keys (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id   UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    key_prefix  TEXT NOT NULL,
    key_hash    TEXT NOT NULL UNIQUE,
    name        TEXT NOT NULL DEFAULT 'Default',
    scopes      TEXT[] NOT NULL DEFAULT '{}',
    status      api_key_status NOT NULL DEFAULT 'active',
    last_used_at TIMESTAMPTZ,
    expires_at   TIMESTAMPTZ,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_api_keys_tenant_id ON api_keys (tenant_id);
CREATE INDEX idx_api_keys_key_hash ON api_keys (key_hash);

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN NEW.updated_at = NOW(); RETURN NEW; END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_tenants_updated_at BEFORE UPDATE ON tenants
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER set_api_keys_updated_at BEFORE UPDATE ON api_keys
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
