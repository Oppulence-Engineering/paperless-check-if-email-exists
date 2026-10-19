DROP INDEX IF EXISTS idx_v1_suppression_source_type;
DROP INDEX IF EXISTS idx_v1_suppression_status_expiry;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM v1_suppression_entries
        GROUP BY tenant_id, email
        HAVING COUNT(*) > 1
    ) THEN
        ALTER TABLE v1_suppression_entries
            ADD CONSTRAINT v1_suppression_entries_tenant_id_email_key UNIQUE (tenant_id, email);
    END IF;
END $$;
