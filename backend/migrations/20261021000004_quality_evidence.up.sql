CREATE VIEW verification_quality_rows AS
SELECT t.*, COALESCE(t.canonical_email, mailbox_identity(COALESCE(t.result->>'input',t.payload->'input'->>'to_email'))) AS identity_key,
 covered.value AS currently_suppressed,
 (t.task_state = 'completed' AND t.safe_to_send IS TRUE AND NOT covered.value) AS effective_safe_to_send
FROM v1_task_result t
CROSS JOIN LATERAL (
 SELECT EXISTS (
  SELECT 1 FROM v1_suppression_entries s WHERE s.tenant_id=t.tenant_id
  AND lower(s.canonical_email) IN (
   lower(t.canonical_email),
   lower(mailbox_identity(COALESCE(t.result->>'input', t.payload->'input'->>'to_email'))),
   lower(mailbox_identity(COALESCE(t.extra->>'original_email',
    (SELECT l.original_data->(t.extra->>'row_index')->>l.email_column
     FROM v1_lists l WHERE l.id=(t.extra->>'list_id')::integer AND l.tenant_id=t.tenant_id))))
  )
  AND s.status='active' AND (s.expires_at IS NULL OR s.expires_at > NOW())
 ) AS value
) covered
WHERE t.is_duplicate IS NOT TRUE
 AND t.sub_reason IS DISTINCT FROM 'invalid_syntax'
 AND COALESCE(t.canonical_email, mailbox_identity(COALESCE(t.result->>'input',t.payload->'input'->>'to_email'))) IS NOT NULL;
