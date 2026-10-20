-- Suppressions now share the canonical address that list deduplication,
-- outcomes and scoring use: for Gmail, dots and the +tag in the local part
-- are ignored and googlemail.com is gmail.com. Re-key the Gmail rows written
-- with only lowercasing, so a lookup by the canonical address finds them.

CREATE TEMPORARY TABLE suppression_rekey ON COMMIT DROP AS
SELECT
    id,
    replace(split_part(split_part(canonical_email, '@', 1), '+', 1), '.', '')
        || '@gmail.com' AS new_key
FROM v1_suppression_entries
WHERE canonical_email ~ '^[^@]+@(gmail|googlemail)\.com$'
  AND replace(split_part(split_part(canonical_email, '@', 1), '+', 1), '.', '') <> '';

-- Several active spellings of one mailbox would collide on the unique index
-- of active rows, so one stays active and the rest are merged into it. The
-- one that stays is the one that suppresses longest: a permanent entry, or
-- else the latest expiry; the newest breaks ties. Its expiry is then the
-- combined coverage of the spellings, and its reason, source and other
-- columns stay those of the entry that provides it. Keeping the newest
-- regardless could leave a newer expired or temporary entry in place of an
-- older permanent one. Inactive and merged rows are not part of this.
WITH keyed AS (
    SELECT e.id, e.tenant_id, e.created_at, e.expires_at,
           COALESCE(r.new_key, e.canonical_email) AS key
    FROM v1_suppression_entries e
    LEFT JOIN suppression_rekey r ON r.id = e.id
    WHERE e.status = 'active'
),
ranked AS (
    SELECT id,
           ROW_NUMBER() OVER (
               PARTITION BY tenant_id, key
               ORDER BY expires_at DESC NULLS FIRST, created_at DESC, id DESC
           ) AS row_number
    FROM keyed
)
UPDATE v1_suppression_entries
SET status = 'merged',
    reason_detail = COALESCE(reason_detail, 'Merged during Gmail canonical suppression migration'),
    updated_at = NOW()
FROM ranked
WHERE v1_suppression_entries.id = ranked.id
  AND ranked.row_number > 1;

UPDATE v1_suppression_entries
SET canonical_email = suppression_rekey.new_key
FROM suppression_rekey
WHERE v1_suppression_entries.id = suppression_rekey.id;

UPDATE v1_suppression_events
SET canonical_email = replace(split_part(split_part(canonical_email, '@', 1), '+', 1), '.', '')
    || '@gmail.com'
WHERE canonical_email ~ '^[^@]+@(gmail|googlemail)\.com$'
  AND replace(split_part(split_part(canonical_email, '@', 1), '+', 1), '.', '') <> '';
