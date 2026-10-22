CREATE INDEX idx_suppressions_active_casefold ON v1_suppression_entries (tenant_id, lower(canonical_email)) WHERE status = 'active';
