ALTER TABLE v1_suppression_entries
    DROP CONSTRAINT IF EXISTS v1_suppression_entries_tenant_id_email_key;

CREATE INDEX idx_v1_suppression_status_expiry
    ON v1_suppression_entries (tenant_id, status, expires_at);

CREATE INDEX idx_v1_suppression_source_type
    ON v1_suppression_entries (tenant_id, source_type, source_ref);
