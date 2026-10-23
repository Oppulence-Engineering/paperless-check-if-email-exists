# RFC 0019: Suppression Scopes

## Verify disposition — 2026-10-23

**Conditional stage 2 agency extension.** Build on RFC 0029 client/campaign context and RFC 0030 export policy if customers need separate client do-not-contact scopes inside one workspace. Labels alone do not provide this behavior or ACL isolation; separate existing workspaces are the initial isolation option. Preserve tenant-wide complaint/hard-bounce protections.

[Current build order](README.md) · [Codebase audit](../audits/2026-10-23-verify-north-star.md). This disposition supersedes conflicting historical scope, priority, or baseline statements below; it does not certify every original acceptance criterion.

| | |
|---|---|
| **Status** | Conditional stage 2 agency extension; audited 2026-10-23 |
| **Created** | 2026-10-21 |
| **Owners** | To assign: backend lead, web lead |
| **Parent** | RFC 0016, item 3 |
| **Builds on** | Existing RFC 0004 suppression; RFC 0029 client context; RFC 0030 delivery |
| **Used by** | RFC 0020 (signup checks for one audience), RFC 0025 (one audience for each platform publication) |
| **Release gates** | Migration tested on a production-size copy; Definition Of Done met |

## Summary

This RFC adds two fields to each suppression entry. The **scope** says where the entry applies: to the whole tenant, or to one audience. The **stream** says which mail it blocks: all mail, or only marketing mail. A check request names its audience and stream, and the backend applies only the entries that match.

## Motivation / Problem

- A tenant can have many newsletters or audiences. A person who unsubscribes from one can still want another. Today an unsubscribe blocks the address for the whole tenant.
- Transactional mail, such as a password reset, must still reach a person who unsubscribed from marketing. Today a check cannot tell the two kinds of mail apart.
- Sending platforms already work this way. For example, they keep unsubscribes for each list, and bounces for the whole account. Our suppression list must match the platform it feeds.

## Goals

- Keep all current behavior for callers that send no new parameters.
- Let unsubscribes and manual entries apply to one audience.
- Let transactional checks ignore marketing unsubscribes.
- Keep bounces, complaints, spam traps, and invalid addresses tenant-wide and for all mail.

## Non-Goals

- A cross-tenant suppression layer. It needs its own RFC and a legal review.
- A stored list of audiences. An audience is a key that the tenant chooses.
- Per-audience quotas or reports.

## Current State In Reacher

- `v1_suppression_entries` holds one active entry for each tenant and mailbox. The partial unique index is `idx_v1_suppression_active_canonical ON (tenant_id, canonical_email) WHERE status = 'active'`. The `canonical_email` column holds the mailbox identity (`mailbox_identity()`, migration `20261020000002`).
- Three upserts target that index with `ON CONFLICT (tenant_id, canonical_email) WHERE status = 'active'`: `backend/src/suppression.rs`, `backend/src/http/v1/outcomes.rs`, and `backend/src/http/v1/suppressions/add.rs`.
- 11 files read or write the table: `suppression.rs`, `reverification.rs`, `http/v1/outcomes.rs`, `http/v1/lists/remediation.rs`, `http/v1/jobs/approval_checklist.rs`, and the six files in `http/v1/suppressions/`.
- `suppression::has_active_suppression` decides if a verification result is suppressed (`scoring/response.rs`). The research flow also calls it.
- `GET /v1/suppressions/check?email=` returns the blocking entry, if one exists.

## Proposed Design

### Fields

| Field | Values | Default |
|---|---|---|
| `scope` | `tenant`, `audience` | `tenant` |
| `audience_key` | A tenant-chosen key, 1 to 128 characters, `[a-z0-9:_.-]` | Null. Required when `scope = 'audience'`. |
| `stream` | `all`, `marketing` | `all`, except `unsubscribe`, which defaults to `marketing` |

Examples of audience keys: `newsletter:weekly`, `beehiiv:pub_123`, `product-updates`.

### Rules For Each Reason

| Reason | Allowed scope | Allowed stream |
|---|---|---|
| `bounce`, `invalid`, `auto_invalid`, `spam_trap` | `tenant` only | `all` only |
| `complaint` | `tenant` only | `all` only |
| `unsubscribe` | `tenant` or `audience` | `marketing` (default) or `all` |
| `manual` | `tenant` or `audience` | `all` (default) or `marketing` |

A request that breaks these rules gets `400`. Bad mailboxes must block all mail, because sending to them hurts the sender's reputation in every stream.

Complaints block all streams in v1. Some senders want transactional mail to reach a person who complained. That is an open question, not a v1 feature.

### Match Rule

A check has a `stream` (`marketing` by default, or `transactional`) and an optional `audience_key`. An active, unexpired entry blocks the check when both are true:

1. `scope = 'tenant'`, or `scope = 'audience'` and `audience_key` matches the check.
2. `stream = 'all'`, or `stream = 'marketing'` and the check stream is `marketing`.

A check with no parameters is a marketing check with no audience. It matches every tenant-scoped entry. This is exactly today's behavior, because all current entries are tenant-scoped.

### One Function

- `has_active_suppression` becomes `find_active_suppressions(pool, tenant_id, email, SuppressionContext { stream, audience_key })`. It returns the matching entries, most severe first.
- Every reader in the 11 files calls this function or uses its SQL predicate `suppression_matches($stream, $audience_key)`. A test lists the files that use the table, so a new reader cannot skip the rule.

### Where The Context Comes From

| Flow | Context |
|---|---|
| `GET /v1/suppressions/check` | New query parameters `stream` and `audience_key`. |
| `/v1/check_email`, `/v1/bulk`, `/v1/lists`, pipelines | New optional request fields `stream` and `audience_key`. Lists and pipelines store them, so re-checks use the same context. |
| Outcome ingestion | An `unsubscribed` event uses `metadata.audience_key` when the sender gives it. Otherwise it creates a tenant-scoped marketing entry. |
| Remediation exports and the approval checklist | They use the context stored on the list. |

### Uniqueness

- One active entry can exist for each tenant, mailbox, audience, and stream.
- A tenant-scoped `all` entry makes narrower entries for the same mailbox useless, but we keep them. Revoking the wide entry must not lose the narrow one.

## Public API / Interface Changes

| Method | Path | Change |
|---|---|---|
| `POST` | `/v1/suppressions` and `/v1/suppressions/import` | New optional fields `scope`, `audience_key`, `stream`. |
| `GET` | `/v1/suppressions` and `/v1/suppressions/export` | New filters `scope`, `audience_key`, `stream`. New response and CSV columns. |
| `GET` | `/v1/suppressions/check` | New query parameters `stream` and `audience_key`. The response lists all matching entries in `matches`. The current single-entry fields stay and show the most severe match. |
| `POST` | `/v1/check_email`, `/v1/bulk`, `/v1/lists`, `/v1/pipelines` | New optional fields `stream` and `audience_key`. |
| `POST` | `/v1/outcomes` | `metadata.audience_key` sets the audience of an unsubscribe. |

Example check:

```http
GET /v1/suppressions/check?email=alex@example.com&stream=transactional
```

```json
{
  "suppressed": false,
  "matches": [],
  "ignored": [
    { "id": 812, "reason": "unsubscribe", "scope": "tenant", "stream": "marketing" }
  ]
}
```

`ignored` lists entries that exist but do not apply to this context. It helps support staff explain a result.

## Data Model / Storage Changes

Migration 1 (in a transaction):

```sql
ALTER TABLE v1_suppression_entries
    ADD COLUMN scope TEXT NOT NULL DEFAULT 'tenant' CHECK (scope IN ('tenant', 'audience')),
    ADD COLUMN audience_key TEXT,
    ADD COLUMN stream TEXT NOT NULL DEFAULT 'all' CHECK (stream IN ('all', 'marketing')),
    ADD CONSTRAINT v1_suppression_audience_key_matches_scope
        CHECK ((scope = 'audience') = (audience_key IS NOT NULL));

UPDATE v1_suppression_entries SET stream = 'marketing' WHERE reason = 'unsubscribe';
```

Migration 2 (no transaction, so the index builds without a long lock):

```sql
-- no-transaction
CREATE UNIQUE INDEX CONCURRENTLY idx_v1_suppression_active_scoped
    ON v1_suppression_entries (tenant_id, canonical_email, COALESCE(audience_key, ''), stream)
    WHERE status = 'active';
```

Migration 3 drops `idx_v1_suppression_active_canonical`. The code release between migrations 2 and 3 moves the three upserts to the new conflict target.

- Confirm that our `sqlx` version supports the `-- no-transaction` directive. If it does not, build the index in a separate operator step and record it in the runbook.
- The `UPDATE` changes old unsubscribes to `marketing`. Default checks are marketing checks, so they still block. Only new transactional checks see a difference.
- `v1_suppression_events` records the scope, audience, and stream in its metadata for each change.

## Auth, Permissions, And Tenant Isolation

- The `suppressions` scope covers all new fields. No new scope is needed.
- Every query keeps the `tenant_id` filter. An audience key is only a label inside one tenant.

## Failure Modes And Edge Cases

| Case | Behavior |
|---|---|
| A caller sends `audience_key` with `scope = 'tenant'` | `400`. |
| A caller sends a bounce with `scope = 'audience'` | `400`. The message names the rule. |
| A mailbox has a tenant `all` entry and an audience entry | Both are active. The check returns both in `matches`. |
| Gmail alias merge (`mailbox_identity`) creates two active rows with the same key | The merge keeps the longest-lasting row for each audience and stream, as today. |
| An export round trip | Export, then import, gives the same entries. A test checks this. |
| A typo in an audience key | The entry does not match. The list page shows audience keys with their counts, so a tenant can see typos. |

## Alternatives Considered

1. **A foreign key to `v1_lists`.** Rejected. A list in our product is an uploaded file to clean, not a lasting audience.
2. **A stored audience table.** Rejected for v1. Free-form keys are enough and match platform IDs. We can add a table later.
3. **A third stream value, `transactional`, on entries.** Rejected. No reason needs to block only transactional mail.

## Rollout / Migration / Compatibility

1. Release migration 1 and the code that reads both indexes.
2. Run migration 2. Measure the index build time on a production-size copy first.
3. Release the code that uses the new conflict target. Then run migration 3.
4. Regenerate the OpenAPI document, the SDKs, and the web client.
5. Add scope, audience, and stream controls to the suppression form and filters in the web app.

Callers that send no new fields see no change in results.

## Metrics And Success Criteria

- Share of tenants with one or more audience-scoped entries.
- Count of transactional checks that an old marketing unsubscribe did not block.
- No increase in p95 latency for `GET /v1/suppressions/check`. Target: under 20 ms in the database.

## Test Strategy

- Unit tests for the match rule: every combination of entry scope, entry stream, check audience, and check stream.
- Unit tests for the reason rules and the `400` messages.
- A database test that the three upserts create one row for each key and keep narrow entries.
- A migration test: old entries keep blocking default checks, and old unsubscribes do not block transactional checks.
- A test that lists every source file that names `v1_suppression_entries`. It fails when a new file appears that does not use `suppression_matches`.
- API harness cases for the new parameters and the export round trip.

## Definition Of Done

1. All migrations run up and down. `e2e_migration_upgrade` passes.
2. All readers use the match rule. The file-list test passes.
3. The OpenAPI document, the SDKs, and the web client show the new fields. CI finds no diff.
4. The web suppression form and filters support the new fields.
5. The docs page `docs/advanced/suppression-intelligence.md` explains scopes and streams.

## Open Questions

1. Can a tenant let transactional mail reach a person who complained? Owner: product owner.
2. Do we limit the number of audience keys for each tenant? Owner: backend lead.

## Unresolved Risks

1. **Wrong stream from the caller.** A caller who marks marketing mail as transactional skips unsubscribes. The API docs must say that this can break the law in some countries. We log the stream of each check.
2. **Index build time.** A large table needs a concurrent build. The runbook covers a failed build.
