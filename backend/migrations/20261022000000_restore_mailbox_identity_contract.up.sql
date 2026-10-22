-- Match lists::canonicalize::canonicalize_email; only consumer Gmail folds local parts.
CREATE OR REPLACE FUNCTION mailbox_identity(email TEXT) RETURNS TEXT
LANGUAGE plpgsql IMMUTABLE STRICT PARALLEL SAFE AS $$
DECLARE
    address TEXT := btrim(email, E' \t\n\r\x0b\x0c' || U&'\0085\00a0\1680\2000\2001\2002\2003\2004\2005\2006\2007\2008\2009\200a\2028\2029\202f\205f\3000');
    local_part TEXT;
    domain_part TEXT;
BEGIN
    IF length(address) - length(replace(address, '@', '')) <> 1 THEN RETURN NULL; END IF;
    local_part := split_part(address, '@', 1);
    domain_part := translate(split_part(address, '@', 2), 'ABCDEFGHIJKLMNOPQRSTUVWXYZ', 'abcdefghijklmnopqrstuvwxyz');
    IF local_part = '' OR domain_part = '' THEN RETURN NULL; END IF;
    IF domain_part IN ('gmail.com', 'googlemail.com') THEN
        local_part := translate(replace(split_part(local_part, '+', 1), '.', ''), 'ABCDEFGHIJKLMNOPQRSTUVWXYZ', 'abcdefghijklmnopqrstuvwxyz');
        domain_part := 'gmail.com';
        IF local_part = '' THEN RETURN NULL; END IF;
    END IF;
    RETURN local_part || '@' || domain_part;
END;
$$;

-- Recover supplied addresses while retaining the applied v3 migration and its backups.
LOCK TABLE v1_suppression_entries IN SHARE ROW EXCLUSIVE MODE;
CREATE TEMP TABLE identity_v4_suppressions ON COMMIT DROP AS
SELECT s.id, s.tenant_id, s.canonical_email AS previous_identity, s.status AS previous_status,
       COALESCE(mailbox_identity(s.email), s.canonical_email) AS identity,
       CASE WHEN s.status = 'merged' AND s.metadata ? 'identity_v3_survivor'
                 AND b.previous->>'status' = 'active' THEN 'active'
            ELSE s.status END AS status,
       s.expires_at, s.created_at
FROM v1_suppression_entries s
LEFT JOIN mailbox_identity_v3_backup b
  ON b.table_name = 'v1_suppression_entries' AND b.row_id = s.id;

-- Preserve the longest protection if subsequent writes added another active alias.
WITH ranked AS (
    SELECT id, row_number() OVER (
        PARTITION BY tenant_id, identity
        ORDER BY expires_at DESC NULLS FIRST, created_at DESC, id DESC
    ) AS rank FROM identity_v4_suppressions WHERE status = 'active'
)
UPDATE identity_v4_suppressions s SET status = 'merged'
FROM ranked WHERE s.id = ranked.id AND ranked.rank > 1;
UPDATE v1_suppression_entries s SET status = 'merged'
FROM identity_v4_suppressions r WHERE s.id = r.id AND r.status = 'merged';
UPDATE v1_suppression_entries s SET canonical_email = r.identity, status = r.status
FROM identity_v4_suppressions r WHERE s.id = r.id;
INSERT INTO v1_suppression_events (tenant_id, entry_id, canonical_email, event_type,
    from_status, to_status, actor_type, actor_id, metadata)
SELECT tenant_id, id, identity, 'identity_migrated', previous_status, status,
       'system', 'mailbox_identity_v4', jsonb_build_object('previous_identity', previous_identity)
FROM identity_v4_suppressions
WHERE previous_identity IS DISTINCT FROM identity OR previous_status IS DISTINCT FROM status;

UPDATE v1_contact_outcomes
SET metadata = metadata || jsonb_build_object('identity_v3_canonical', canonical_email),
    canonical_email = mailbox_identity(email)
WHERE mailbox_identity(email) IS NOT NULL
  AND canonical_email IS DISTINCT FROM mailbox_identity(email);

-- Stop instead of changing the identity of work still in flight.
LOCK TABLE v1_task_result IN SHARE ROW EXCLUSIVE MODE;
DO $$ BEGIN
 IF EXISTS (
  SELECT 1 FROM v1_task_result
  WHERE canonical_email IS NOT NULL
    AND canonical_email IS DISTINCT FROM CASE WHEN is_duplicate
      THEN mailbox_identity(COALESCE(payload->'input'->>'to_email', result->>'input'))
      ELSE mailbox_identity(COALESCE(result->>'input', payload->'input'->>'to_email')) END
    AND (task_state NOT IN ('completed', 'failed', 'dead_lettered', 'cancelled')
         OR (dispatch_id IS NOT NULL AND NOT COALESCE(completion_done, false)))
 ) THEN RAISE EXCEPTION 'Identity recovery requires affected jobs to finish; retry after workers drain';
 END IF;
END $$;

-- Keep completed results and duplicate provenance; never retarget a live job.
UPDATE v1_task_result
SET extra = COALESCE(extra, '{}'::jsonb) || jsonb_build_object('identity_v3_canonical', canonical_email),
    canonical_email = CASE WHEN is_duplicate
        THEN mailbox_identity(COALESCE(payload->'input'->>'to_email', result->>'input'))
        ELSE mailbox_identity(COALESCE(result->>'input', payload->'input'->>'to_email')) END
WHERE task_state IN ('completed', 'failed', 'dead_lettered', 'cancelled')
  AND (dispatch_id IS NULL OR completion_done)
  AND canonical_email IS NOT NULL;

-- Original-less campaign outcomes remain marked ambiguous; do not invent their case.
WITH restored AS (
 SELECT o.id, COALESCE(mailbox_identity(o.email), b.previous->>'canonical_email', o.canonical_email) AS identity
 FROM verification_outcomes o LEFT JOIN mailbox_identity_v3_backup b
 ON b.table_name = 'verification_outcomes' AND b.row_id = o.id
)
UPDATE verification_outcomes o
SET canonical_email = r.identity,
    metadata = COALESCE(o.metadata, '{}'::jsonb) || jsonb_build_object('identity_v3_canonical', o.canonical_email)
FROM restored r WHERE r.id = o.id AND o.canonical_email IS DISTINCT FROM r.identity;

-- Cached snapshots must be verified again under the restored identity contract.
UPDATE v1_pipeline_contact_state SET source_hash = 'identity-v3:' || source_hash;
