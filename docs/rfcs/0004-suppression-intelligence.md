# RFC 0004: Suppression Intelligence

## Status

Draft

## Summary

Expand suppression handling from simple add/check/list/delete behavior into a richer tenant-safe system with explicit reasons, provenance, expiration, and auditability. The feature remains backend-first and additive to the current suppression endpoints.

## Motivation / Problem

- Current suppressions are useful but too shallow for operational and compliance-heavy workflows.
- Customers need to understand why an address is suppressed, where that suppression came from, whether it expires, and what changed over time.
- Later analytics and recommendation features need better suppression metadata than a flat email entry.

## Goals

- Add reason, source, and expiry metadata to suppressions.
- Distinguish manual and automatic suppressions.
- Preserve audit history without losing current-check performance.
- Support import and export of suppression state.

## Non-Goals

- Native outbound sync back to every ESP or CRM.
- Cross-tenant suppression sharing.
- A UI moderation queue.

## Current State In Reacher

- Reacher already supports tenant-scoped suppressions through add, check, list, and delete routes.
- Worker automation can auto-suppress based on score or category settings.
- There is no first-class audit trail, provenance model, or expiry behavior.

## Proposed Design

Extend the suppression model with these fields:

- `status`: `active`, `expired`, or `revoked`
- `reason_code`
- `reason_detail`
- `source_type`
- `source_ref`
- `created_by`
- `expires_at`
- `last_seen_at`
- `metadata`

Supported `source_type` values are:

- `manual`
- `import`
- `policy`
- `provider_event`
- `worker_rule`
- `api`

Suppression precedence is fixed:

- Manual suppressions always win over automatic suppressions.
- Automatic upserts may refresh `last_seen_at`, `reason_code`, and `source_ref` only when the current active entry is also automatic.
- `revoked` entries stay in history and are never silently reactivated; a new active row must be created.

Expiration behavior is also fixed:

- `expires_at` is optional.
- Reads only treat `active` and unexpired rows as blocking.
- Expired rows are retained for audit and returned only when the caller asks for `status=expired` or `include_expired=true`.

### Resource Model

Suppression Intelligence introduces two related resources:

- `suppression_entry`
  - the current tenant-visible state for one canonical email
  - optimized for fast blocking checks, list views, and export
- `suppression_event`
  - append-only audit history explaining how the current state was reached
  - optimized for traceability, import debugging, and policy review

The contract is intentionally asymmetric:

- `suppression_entry` is the operational record used by `/check`, send-blocking logic, policy modes, and bulk workflows
- `suppression_event` is the evidence trail used by support, compliance, and later analytics

Each canonical email may have many historical events but at most one current active entry per tenant at a time.

### Source And Reason Taxonomy

`reason_code` is a controlled enum in v1:

- `manual_block`
- `unsubscribe`
- `hard_bounce`
- `complaint`
- `spam_trap`
- `invalid_recipient`
- `high_risk_domain`
- `role_account_policy`
- `disposable_policy`
- `job_policy`
- `imported_block`
- `other`

`reason_detail` is optional free text capped at 512 bytes and intended for human context such as:

- "sales ops manual import from CRM suppression export"
- "customer complaint via SendGrid event"
- "blocked by strict outbound policy for campaign launch"

`source_ref` is provider- or workflow-specific and may carry:

- upstream provider event ID
- job ID
- pipeline ID
- import batch ID
- external list ID

### State Machine

State transitions are explicit:

- `active -> active`
  - automatic refresh
  - reason/source metadata update when same precedence band
- `active -> expired`
  - background expiry worker or read-time lazy state transition
- `active -> revoked`
  - explicit user/admin delete or API revoke action
- `expired -> active`
  - not allowed by mutating the old row
  - a brand-new active row must be created with a new `id`
- `revoked -> active`
  - same rule as expired; create a new row

This design avoids reusing old identifiers for materially different blocking decisions.

### Operational Rules

Canonicalization uses the same email normalization path already used by verification and suppression lookup. The canonical email string is the primary join key for:

- suppression check
- policy evaluation
- future provider webhook ingestion
- bulk remediation review classification

Precedence is resolved with this ordering:

1. manual active suppression
2. imported active suppression
3. provider-event active suppression
4. worker-rule or policy active suppression
5. expired and revoked records, which never block

Examples:

- a provider complaint event cannot overwrite a manual block, but it does create an audit event
- a worker rule can refresh another worker-created entry
- a fresh import can upgrade an older automatic suppression if both are non-manual and the new row extends `expires_at` or adds stronger reason evidence

### Import And Export Behavior

Import is synchronous for small payloads and async by job only if later batch size limits require it. RFC v1 chooses synchronous behavior with chunked inserts and a hard request limit of 10,000 rows per call.

Import rules:

- dedupe input rows by canonical email before DB write
- preserve row-level reject reasons for invalid emails or invalid enums
- create one `import_batch_id` for all accepted rows in a request
- write a `suppression_event` for every accepted row, even when the current entry is unchanged

Export rules:

- default format is CSV
- optional `format=ndjson` is supported for machine consumers
- export is filtered server-side and streamed rather than fully materialized in memory
- default export includes only current active entries unless `include_history=true`

## Public API / Interface Changes

Keep existing routes and expand them:

- `POST /v1/suppressions`
- `GET /v1/suppressions`
- `GET /v1/suppressions/check`
- `DELETE /v1/suppressions/{id}`

Add new routes:

- `POST /v1/suppressions/import`
- `GET /v1/suppressions/export`
- `GET /v1/suppressions/{id}/events`

`POST /v1/suppressions` request additions:

- `reason_code`
- `reason_detail`
- `source_type`
- `source_ref`
- `expires_at`
- `metadata`

`GET /v1/suppressions` filter additions:

- `status`
- `reason_code`
- `source_type`
- `q`
- `expires_before`
- `expires_after`

Example create request:

```json
{
  "email": "ops@example.com",
  "reason_code": "manual_block",
  "reason_detail": "Customer requested permanent suppression",
  "source_type": "manual",
  "source_ref": "ticket_1283",
  "expires_at": null,
  "metadata": {
    "channel": "support"
  }
}
```

Example entry response:

```json
{
  "id": "sup_01HTZP3M2D5YF4D4A6E1YJ8Q8R",
  "email": "ops@example.com",
  "canonical_email": "ops@example.com",
  "status": "active",
  "reason_code": "manual_block",
  "reason_detail": "Customer requested permanent suppression",
  "source_type": "manual",
  "source_ref": "ticket_1283",
  "created_by": "user_01HQ...",
  "expires_at": null,
  "last_seen_at": "2026-03-30T17:12:44Z",
  "metadata": {
    "channel": "support"
  },
  "created_at": "2026-03-30T17:12:44Z",
  "updated_at": "2026-03-30T17:12:44Z"
}
```

`GET /v1/suppressions/check` response becomes:

```json
{
  "email": "ops@example.com",
  "canonical_email": "ops@example.com",
  "suppressed": true,
  "entry": {
    "id": "sup_01HTZP3M2D5YF4D4A6E1YJ8Q8R",
    "status": "active",
    "reason_code": "manual_block",
    "source_type": "manual",
    "expires_at": null
  }
}
```

`GET /v1/suppressions/{id}/events` returns reverse-chronological audit records with:

- `event_id`
- `entry_id`
- `event_type`
- `from_status`
- `to_status`
- `reason_code`
- `source_type`
- `source_ref`
- `metadata`
- `actor_type`
- `actor_id`
- `created_at`

`event_type` values in v1:

- `created`
- `refreshed`
- `expired`
- `revoked`
- `imported`
- `read_revalidated`

## Data Model / Storage Changes

- Extend `v1_suppression_entries` with the new metadata fields.
- Add `v1_suppression_events` for append-only audit records.
- Keep the active lookup indexed by tenant and canonical email so `check` performance stays flat.

`v1_suppression_entries` new or clarified columns:

- `id`
- `tenant_id`
- `email`
- `canonical_email`
- `status`
- `reason_code`
- `reason_detail`
- `source_type`
- `source_ref`
- `created_by`
- `expires_at`
- `last_seen_at`
- `metadata JSONB`
- `created_at`
- `updated_at`

Indexes:

- unique partial index on `(tenant_id, canonical_email)` where `status = 'active'`
- btree index on `(tenant_id, status, updated_at desc)`
- btree index on `(tenant_id, source_type, updated_at desc)`
- btree index on `(tenant_id, reason_code, updated_at desc)`

`v1_suppression_events` columns:

- `id`
- `tenant_id`
- `entry_id`
- `canonical_email`
- `event_type`
- `from_status`
- `to_status`
- `reason_code`
- `reason_detail`
- `source_type`
- `source_ref`
- `actor_type`
- `actor_id`
- `metadata JSONB`
- `created_at`

Retention choice:

- entries follow normal tenant data retention because they are part of live product behavior
- events default to 365 days minimum retention, even if entries later expire or are revoked
- a future admin retention override may shorten audit retention, but this RFC does not add per-tenant knobs

## Auth, Permissions, And Tenant Isolation

- All suppression APIs continue to use the existing `suppressions` scope.
- Import/export endpoints also require `suppressions`.
- Tenant isolation is enforced on both entries and audit events.
- Automatic suppressions created by worker rules carry `created_by = system`.

Behavior by caller type:

- tenant scoped API key with `suppressions`
  - full create/list/check/revoke/import/export access
- read-only future analytics key
  - no suppression access in v1
- admin routes
  - unchanged by this RFC; admin reuse of the new read model is a later follow-on

Import and export must never accept a caller-provided `tenant_id`; the tenant is always derived from the authenticated context.

## Failure Modes And Edge Cases

- Duplicate imports are deduped by tenant and canonical email before insert.
- An import row that references an already-active manual suppression becomes an audit event only and does not overwrite the current entry.
- `DELETE` on an already-expired or revoked entry becomes a `revoked` audit event and returns success only once.
- Expired entries are never treated as active even if background cleanup has not yet run.

Additional cases:

- if `expires_at <= now()` at creation time, return `400`; callers must either omit it or provide a future timestamp
- if canonicalization fails because the email is syntactically invalid, return row-level reject detail for imports and `400` for single creates
- if a list page mixes active and expired rows, pagination order remains stable by `(updated_at desc, id desc)`
- if the expiry worker is delayed, read paths may lazily materialize an `expired` event on first access rather than waiting for batch cleanup
- if a revoked entry is re-added later, list and export surfaces show both rows separately with different IDs so the audit story remains intact

## Alternatives Considered

- Overwriting the current suppression row in place with no event history. Rejected because it destroys provenance.
- Separate manual and automatic tables. Rejected because queries and sync semantics become harder to reason about.
- Purging expired suppressions immediately. Rejected because customers need auditability.

## Rollout / Migration / Compatibility

- Extend the current table additively and backfill `source_type = manual` for existing rows.
- Keep existing add/check/list/delete request shapes valid.
- Make import/export optional follow-on surfaces that use the same underlying model.

Planned rollout:

1. add schema fields and backfill defaults
2. add event emission for existing create/delete paths
3. ship expanded list/check responses behind a compatibility-safe additive response shape
4. enable import/export routes
5. update docs and SDKs

Backfill defaults:

- `status = active`
- `reason_code = other`
- `source_type = manual`
- `last_seen_at = created_at`

Compatibility rule:

- existing clients that only send `email` continue to succeed and receive additive response fields
- `DELETE` continues to behave as a success-oriented revoke action rather than a hard physical row delete

## Metrics And Success Criteria

- Percentage of suppressions with non-null reason and source metadata.
- Manual vs automatic suppression share.
- Expired suppression count and expiry processing lag.
- Audit event completeness for create, refresh, revoke, and import actions.

Additional success criteria:

- `GET /v1/suppressions/check` p95 latency remains within 10% of current baseline
- import reject rate is explainable with per-row error codes, not opaque failures
- at least 95% of automatic suppressions generated by worker policies carry a non-null `source_ref` to the originating job or pipeline when available

## Test Strategy

- CRUD compatibility tests for old and new request bodies.
- Expiry behavior tests.
- Manual-over-automatic precedence tests.
- Import dedupe and export filter tests.
- Audit trail tests covering create, refresh, revoke, and expiry transitions.

Add explicit coverage for:

- unique active entry enforcement under concurrent creates
- read-time expiry materialization
- repeated revoke calls on the same row
- import rows mixing valid, invalid, and duplicate emails
- streaming export on large result sets
- tenant isolation across identical canonical emails in different tenants

## Unresolved Risks

- Poor canonicalization can over-collapse distinct addresses into one suppression.
- Large import volumes may need chunking or async job handling in a later iteration.

The largest practical risk is support burden from reason taxonomy drift. If later features create new automatic suppressions without using the shared `reason_code` catalog, the audit model will fragment quickly.
