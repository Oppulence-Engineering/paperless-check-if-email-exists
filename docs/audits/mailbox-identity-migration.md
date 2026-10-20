# Mailbox identity migration

Migration `20261020000002` and the application revision must be deployed together with verification admission paused and workers drained. Older writers use a different identity, so running them after migration can recreate alias inconsistencies. Take a database backup before migration. The down migration refuses to merge newly distinct mailboxes; roll forward or restore the backup instead.

The shared Rust identity function and PostgreSQL `mailbox_identity` preserve local-part case, dots, and tags except for consumer `gmail.com` and `googlemail.com`. Only those consumer domains fold ASCII case, remove dots and tags, and share the `gmail.com` domain. Workspace domains do not inherit Gmail equivalence. Lists and pipelines retain the first supplied address for SMTP routing and store the identity separately.

The migration:

- Retains every suppression row and existing audit event. Gmail alias collisions keep the active entry with the longest expiry (permanent first); other active aliases become `merged`. New audit events record the old identity, survivor, and status change. Each original supplied address remains on its row.
- Rebuilds contact-outcome identity from its retained address and records the former identity in metadata.
- Retains campaign outcomes that lack an original address, adds an original-address column for new ingestion, and flags legacy non-Gmail events. Those ambiguous events cannot contribute positive engagement evidence; existing negative safety evidence is retained under its recorded identity.
- Rebuilds task identity from the actual checked address, or the supplied address for duplicate rows. Previous canonical identities remain in `extra.identity_v1_canonical`. Duplicate rows remain historical copies, not independent SMTP evidence.
- Invalidates pipeline delta hashes so the next run checks the current input under the new rules. Historical state and change events remain intact.

An older lowercase address cannot reveal whether the caller supplied `Sales` or `sales`. Do not invent that information, rewrite recorded SMTP evidence, or attach it to both mailboxes. Reconcile suppression originals from their authoritative import/provider source where available; otherwise retain the legacy entry and flag the ambiguity for its owner. Historical change events and exports describe the original run and must not be treated as proof of a differently spelled mailbox.

Before resuming automated use of existing lists, identify rows whose supplied mailbox differs from the address actually checked:

```sql
SELECT l.tenant_id, l.id AS list_id, t.id AS task_id,
       row.value ->> l.email_column AS supplied_email,
       COALESCE(t.result->>'input', t.payload->'input'->>'to_email') AS checked_email
FROM v1_lists l
JOIN v1_task_result t ON t.job_id = l.job_id
JOIN LATERAL jsonb_each(l.original_data) row
  ON row.key = t.extra->>'row_index'
WHERE t.result IS NOT NULL
  AND mailbox_identity(row.value ->> l.email_column) IS DISTINCT FROM
      mailbox_identity(COALESCE(t.result->>'input', t.payload->'input'->>'to_email'));
```

Keep affected historical runs intact. Reverify their original source rows in a new run before using them for a send decision; record the new run as the replacement. This is an explicit reconciliation step during deployment, not a background rewrite or a silent quota charge. Do not resume sending from affected historical lists until that reconciliation is complete.

Validation includes Rust/SQL identity parity, a seeded pre-migration upgrade, Gmail alias collisions with expired/finite/permanent entries, preserved original audit events, API/import/worker/outcome suppression consistency, distinct case-sensitive SMTP recipients, and list queue payloads retaining their supplied spelling.
