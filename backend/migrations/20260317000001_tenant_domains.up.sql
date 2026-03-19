CREATE TABLE tenant_domains (
    id                       UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id                UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    domain                   TEXT NOT NULL,
    is_active                BOOLEAN NOT NULL DEFAULT true,
    is_verified              BOOLEAN NOT NULL DEFAULT false,
    notes                    TEXT,
    created_at               TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at               TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

ALTER TABLE tenant_domains
    ADD CONSTRAINT tenant_domains_domain_not_blank
    CHECK (char_length(trim(domain)) > 0);

CREATE INDEX idx_tenant_domains_tenant_id ON tenant_domains (tenant_id);
CREATE INDEX idx_tenant_domains_tenant_id_active ON tenant_domains (tenant_id, is_active);
CREATE UNIQUE INDEX idx_tenant_domains_tenant_domain
    ON tenant_domains (tenant_id, lower(domain));

CREATE TRIGGER set_tenant_domains_updated_at
    BEFORE UPDATE ON tenant_domains
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
