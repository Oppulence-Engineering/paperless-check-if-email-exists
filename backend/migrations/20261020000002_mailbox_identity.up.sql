-- Match lists::canonicalize::canonicalize_email; only consumer Gmail folds local parts.
CREATE FUNCTION mailbox_identity(email TEXT) RETURNS TEXT
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

-- Preserve every alias row and audit event. Prefer the longest-lived active
-- suppression, so merging aliases never shortens an existing protection.
LOCK TABLE v1_suppression_entries IN SHARE ROW EXCLUSIVE MODE;
CREATE TEMP TABLE suppression_identity_migration ON COMMIT DROP AS
SELECT id, tenant_id, canonical_email AS old_identity, status AS old_status,
       COALESCE(mailbox_identity(email), canonical_email) AS new_identity,
       CASE WHEN status = 'active' THEN
           first_value(id) OVER (
               PARTITION BY tenant_id, COALESCE(mailbox_identity(email), canonical_email), status
               ORDER BY expires_at DESC NULLS FIRST, created_at DESC, id DESC
           )
       ELSE id END AS survivor_id
FROM v1_suppression_entries;

UPDATE v1_suppression_entries s
SET status = 'merged'
FROM suppression_identity_migration m
WHERE s.id = m.id AND m.id <> m.survivor_id;

UPDATE v1_suppression_entries s
SET canonical_email = m.new_identity,
    metadata = s.metadata || jsonb_build_object(
        'identity_migration', jsonb_build_object(
            'version', 2, 'previous_identity', m.old_identity,
            'survivor_id', m.survivor_id,
            'legacy_case_folding_possible', split_part(m.new_identity, '@', 2) <> 'gmail.com'
        )
    )
FROM suppression_identity_migration m
WHERE s.id = m.id;

INSERT INTO v1_suppression_events (
    tenant_id, entry_id, canonical_email, event_type, from_status, to_status,
    actor_type, actor_id, metadata
)
SELECT m.tenant_id, m.id, m.new_identity, 'identity_migrated', m.old_status, s.status,
       'system', 'mailbox_identity_v2', s.metadata->'identity_migration'
FROM suppression_identity_migration m
JOIN v1_suppression_entries s ON s.id = m.id
WHERE m.old_identity IS DISTINCT FROM m.new_identity OR m.id <> m.survivor_id;

-- Contact outcomes retain their supplied address. Campaign outcomes did not;
-- keep that history intact and retain originals for every future write.
UPDATE v1_contact_outcomes
SET canonical_email = COALESCE(mailbox_identity(email), canonical_email),
    metadata = metadata || jsonb_build_object('identity_v1_canonical', canonical_email);
ALTER TABLE verification_outcomes ADD COLUMN email TEXT;
UPDATE verification_outcomes
SET metadata = COALESCE(metadata, '{}'::jsonb) || '{"identity_original_unavailable":true}'::jsonb
WHERE canonical_email NOT LIKE '%@gmail.com';

-- A result belongs to the actual checked address. Duplicate rows keep their
-- supplied identity but are not independent verification evidence.
UPDATE v1_task_result
SET extra = COALESCE(extra, '{}'::jsonb) ||
        jsonb_build_object('identity_v1_canonical', canonical_email),
    canonical_email = CASE WHEN is_duplicate
        THEN mailbox_identity(COALESCE(payload->'input'->>'to_email', result->>'input'))
        ELSE mailbox_identity(COALESCE(result->>'input', payload->'input'->>'to_email'))
    END;

-- Preserve the historical snapshot while forcing the next delta run to verify.
UPDATE v1_pipeline_contact_state SET source_hash = 'identity-v1:' || source_hash;
