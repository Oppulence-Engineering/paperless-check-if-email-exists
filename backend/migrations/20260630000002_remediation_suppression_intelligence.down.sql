DROP TABLE IF EXISTS v1_remediation_exports;
DROP TABLE IF EXISTS v1_remediation_rows;
DROP TABLE IF EXISTS v1_remediation_plans;

DROP INDEX IF EXISTS idx_v1_suppression_events_entry;
DROP TABLE IF EXISTS v1_suppression_events;

DROP INDEX IF EXISTS idx_v1_suppression_active_canonical;

ALTER TABLE v1_suppression_entries
    DROP COLUMN IF EXISTS metadata,
    DROP COLUMN IF EXISTS last_seen_at,
    DROP COLUMN IF EXISTS expires_at,
    DROP COLUMN IF EXISTS created_by,
    DROP COLUMN IF EXISTS source_ref,
    DROP COLUMN IF EXISTS source_type,
    DROP COLUMN IF EXISTS reason_detail,
    DROP COLUMN IF EXISTS reason_code,
    DROP COLUMN IF EXISTS status,
    DROP COLUMN IF EXISTS canonical_email;
