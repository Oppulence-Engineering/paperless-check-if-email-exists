-- Stop rather than retarget work that is still completing during the identity change.
LOCK TABLE v1_task_result IN SHARE ROW EXCLUSIVE MODE;
DO $$ BEGIN
 IF EXISTS (
  SELECT 1 FROM v1_task_result
  WHERE job_id IS NOT NULL AND canonical_email IS NOT NULL AND is_duplicate IS NOT TRUE
  GROUP BY tenant_id, job_id, lower(canonical_email)
  HAVING count(*) > 1 AND bool_or(task_state NOT IN ('completed','failed','dead_lettered','cancelled')
   OR (dispatch_id IS NOT NULL AND NOT completion_done))
 ) THEN
  RAISE EXCEPTION 'Identity migration requires colliding jobs to finish; retry after workers drain';
 END IF;
END $$;

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
    RETURN translate(local_part, 'ABCDEFGHIJKLMNOPQRSTUVWXYZ', 'abcdefghijklmnopqrstuvwxyz') || '@' || domain_part;
END;
$$;

-- Keep the original keys and statuses for recovery; never shorten suppression coverage.
CREATE TABLE mailbox_identity_v3_backup (
    table_name TEXT NOT NULL, row_id BIGINT NOT NULL, previous JSONB NOT NULL,
    PRIMARY KEY (table_name, row_id)
);
LOCK TABLE v1_suppression_entries IN SHARE ROW EXCLUSIVE MODE;
INSERT INTO mailbox_identity_v3_backup
SELECT 'v1_suppression_entries', id, to_jsonb(s) FROM v1_suppression_entries s;
WITH ranked AS (
    SELECT id, first_value(id) OVER (
        PARTITION BY tenant_id, mailbox_identity(canonical_email)
        ORDER BY expires_at DESC NULLS FIRST, created_at DESC, id DESC
    ) AS survivor_id FROM v1_suppression_entries WHERE status = 'active'
)
UPDATE v1_suppression_entries s SET status = 'merged',
    metadata = metadata || jsonb_build_object('identity_v3_survivor', ranked.survivor_id)
FROM ranked WHERE s.id = ranked.id AND s.id <> ranked.survivor_id;
UPDATE v1_suppression_entries SET canonical_email = mailbox_identity(canonical_email);
INSERT INTO v1_suppression_events (tenant_id, entry_id, canonical_email, event_type,
    from_status, to_status, actor_type, actor_id, metadata)
SELECT s.tenant_id, s.id, s.canonical_email, 'identity_migrated', b.previous->>'status',
    s.status, 'system', 'mailbox_identity_v3', jsonb_build_object('previous_identity', b.previous->>'canonical_email')
FROM v1_suppression_entries s JOIN mailbox_identity_v3_backup b
ON b.table_name = 'v1_suppression_entries' AND b.row_id = s.id
WHERE s.canonical_email IS DISTINCT FROM b.previous->>'canonical_email'
   OR s.status IS DISTINCT FROM b.previous->>'status';

-- Normalize old application writes throughout the regional rolling deployment.
CREATE FUNCTION normalize_suppression_identity() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
    NEW.canonical_email := mailbox_identity(NEW.canonical_email);
    RETURN NEW;
END;
$$;
CREATE TRIGGER suppression_identity_casefold BEFORE INSERT OR UPDATE OF canonical_email
ON v1_suppression_entries FOR EACH ROW EXECUTE FUNCTION normalize_suppression_identity();

INSERT INTO mailbox_identity_v3_backup
SELECT 'v1_task_result', id, to_jsonb(t)
FROM v1_task_result t WHERE canonical_email IS NOT NULL;
UPDATE v1_task_result SET canonical_email = mailbox_identity(canonical_email)
WHERE canonical_email IS NOT NULL;
-- Only deduplicate within the same list/job; separate checks remain historical evidence.
WITH ranked AS (
 SELECT id, first_value(id) OVER (PARTITION BY tenant_id, job_id, canonical_email
 ORDER BY is_duplicate ASC NULLS FIRST, id) AS original_id
 FROM v1_task_result WHERE job_id IS NOT NULL AND canonical_email IS NOT NULL
)
UPDATE v1_task_result t SET is_duplicate = true, canonical_task_id = ranked.original_id
FROM ranked WHERE t.id = ranked.id AND t.id <> ranked.original_id;
UPDATE v1_lists l SET unique_emails = x.unique_emails, deduplicated_count = x.duplicates
FROM (SELECT job_id, count(*) FILTER (WHERE canonical_email IS NOT NULL AND is_duplicate IS NOT TRUE)::int AS unique_emails,
 count(*) FILTER (WHERE is_duplicate)::int AS duplicates FROM v1_task_result GROUP BY job_id) x
WHERE l.job_id = x.job_id;

UPDATE v1_contact_outcomes SET metadata = metadata || jsonb_build_object('identity_v2_canonical', canonical_email),
 canonical_email = mailbox_identity(canonical_email);
-- Retain colliding outcome rows but exclude duplicate events from aggregate evidence.
ALTER TABLE verification_outcomes ADD COLUMN identity_duplicate_of BIGINT REFERENCES verification_outcomes(id);
INSERT INTO mailbox_identity_v3_backup
SELECT 'verification_outcomes', id, jsonb_build_object('canonical_email', canonical_email) FROM verification_outcomes;
WITH ranked AS (
 SELECT id, first_value(id) OVER (PARTITION BY tenant_id, mailbox_identity(canonical_email), outcome_type, occurred_at, source
 ORDER BY (canonical_email = mailbox_identity(canonical_email)) DESC, id) AS original_id
 FROM verification_outcomes
)
UPDATE verification_outcomes o SET identity_duplicate_of = ranked.original_id
FROM ranked WHERE o.id = ranked.id AND o.id <> ranked.original_id;
-- Keep the original uniqueness constraint compatible with old writers. Colliding
-- historical rows retain their original spelling and remain available for audit.
UPDATE verification_outcomes SET canonical_email = mailbox_identity(canonical_email)
WHERE identity_duplicate_of IS NULL;
CREATE VIEW verification_outcome_evidence AS SELECT * FROM verification_outcomes WHERE identity_duplicate_of IS NULL;
CREATE TABLE mailbox_identity_v3_pipeline_backup AS TABLE v1_pipeline_contact_state;
-- State is a cache; keep the newest row, with every original retained for recovery.
DELETE FROM v1_pipeline_contact_state p USING (
 SELECT pipeline_id, canonical_email, row_number() OVER (
  PARTITION BY pipeline_id, mailbox_identity(canonical_email)
  ORDER BY last_verified_at DESC NULLS LAST, updated_at DESC, canonical_email
 ) AS rank FROM v1_pipeline_contact_state
) ranked WHERE p.pipeline_id=ranked.pipeline_id AND p.canonical_email=ranked.canonical_email AND ranked.rank>1;
UPDATE v1_pipeline_contact_state SET canonical_email=mailbox_identity(canonical_email);
CREATE TRIGGER pipeline_identity_casefold BEFORE INSERT OR UPDATE OF canonical_email
ON v1_pipeline_contact_state FOR EACH ROW EXECUTE FUNCTION normalize_suppression_identity();
CREATE INDEX idx_results_casefold ON v1_task_result (tenant_id, lower(canonical_email));

CREATE TRIGGER task_identity_casefold BEFORE INSERT OR UPDATE OF canonical_email
ON v1_task_result FOR EACH ROW EXECUTE FUNCTION normalize_suppression_identity();
CREATE TRIGGER outcome_identity_casefold BEFORE INSERT OR UPDATE OF canonical_email
ON verification_outcomes FOR EACH ROW EXECUTE FUNCTION normalize_suppression_identity();
