# RFC 0027: Data Rights and Product Boundaries

| Field | Value |
|---|---|
| Status | Proposed; stage 0 |
| Updated | 2026-10-23 |
| Owners | Product/data-rights owner, Go lead, Rust lead; assign before rollout |
| Depends on | Existing authenticated research bridge; no new RFC dependency |
| Replaces | RFC 0015 shared-store assumptions and unassigned rights/privacy work |
| Enables | RFCs 0028–0033 |

## Problem and outcome

Verify uses a research backend that also serves an internal finance identity
strategy. A server-side tenant mapping does not itself establish ownership,
provider reuse rights, deletion behavior, or separation between those products.
An agency must be able to supply a private list without contributing its contacts,
relationships, or suppressions to another customer or the finance graph.

The [audit](../audits/2026-10-23-verify-north-star.md) inspected the local
[bridge](../../backend/src/research.rs), not the Go implementation or contracts.
Inventory the actual Go stores and provider agreements before defining migrations.

## Decisions

1. Default every imported record, derived contact, reviewer correction, and source
   observation to the originating Verify tenant and purpose. No implicit transfer
   into a shared commercial database or internal finance graph.
2. Maintain a versioned source-use register: provider/source, agreement reference,
   permitted purpose, customer-private vs shared use, export rights, allowed
   retention, deletion obligations, reviewer, review date, and enabled state.
   Unknown or expired rights disable new sourcing and affected exports until resolved.
3. A public URL is evidence of publication, not automatic permission for any resale
   or retention use. Parallel is optional and disabled for uses its contract does
   not cover. Licensed shared data requires its own lineage and access policy.
4. Reuse of tenant evidence is limited to that tenant, purpose, license, and
   freshness window. Shared domain diagnostics must not contain person-level or
   customer relationship data.
5. Record a product ownership decision covering PlaybookMedia/Verify, the Go
   operator, processing responsibilities, and commercial vs finance data flows.
   Engineering records the decision; this RFC does not assert a legal conclusion.

## Storage and service contract

Go remains authoritative for research observations. Extend its existing evidence
metadata with `product_purpose`, `tenant/workspace`, `source_use_version`,
`observed_at`, `retention_until`, and deletion/tombstone state. Rust stores immutable
references and policy versions on audit decisions/exports rather than a second
research corpus. Only licensed source text needed for review is retained.

Propose internal, service-authenticated deletion and retention reconciliation
operations keyed by tenant and stable request ID. Enumerate raw uploads, derived
rows, evidence caches, export artifacts, logs, backups, and any provider-side
copies. Retries must resume without resurrecting data. A deletion receipt reports
completed, pending, and retained-for-defined-obligation resources and deadlines.

Tenant admins configure permitted retention through an additive settings API;
ordinary research callers cannot grant shared-use rights. Retain the minimum
suppression marker needed to honor do-not-contact obligations under an explicitly
documented retention policy; deletion must not silently make an address sendable.
Do not log raw lists, credentials, or unrestricted evidence excerpts.

## Incremental delivery

1. **Rights-cleared concierge pilot:** complete the source register, ownership map,
   retention schedule, and tested operator deletion procedure for pilot sources.
   Block unapproved providers and shared reuse. This enables bounded paid audits.
2. **Automated enforcement:** check source-use versions at scheduling, reuse,
   evidence access, and export; automate cross-service deletion receipts before
   enabling broader self-service research or retention promises.
3. **Optional shared sources:** require an explicit new decision and evidence of
   licensing before any shared dataset. This is not on the initial build path.

## Acceptance and validation

- Two tenants with the same domain cannot see each other's imported people,
  observations, decisions, exports, or suppression metadata. Test both services.
- A Verify record is inaccessible to finance graph consumers without a separately
  authorized and licensed transfer; test the actual Go path, not only the gateway.
- Revoking source rights blocks new calls and disallowed reuse/exports while
  preserving a minimal audit reason. Missing source-use metadata fails closed.
- Deletion interrupts queued work, revokes artifact access, traverses both
  repositories, and remains deleted after replay and cache refresh. Record the
  backup expiry/restoration procedure and any contractual provider limitation.
- Use synthetic fixtures in ordinary CI. Rights-cleared real-source checks require
  a bounded evaluation run and a stored review record.

## Rollout and risks

Keep existing evidence tenant-private. Mark legacy evidence with unknown source
rights for review; do not retroactively label it licensed. Deploy additive metadata
and enforcement in stages, with a per-source kill switch. A gateway-only fix is
insufficient if Go bypasses the same rule. Contract review and cross-repository
ownership are release dependencies, not proof already supplied by this document.

## Technical specification

Apply the [shared technical contracts](verify-technical-contracts.md). This section
makes the storage, admission, deletion, and rollout decisions above implementable.

### Schema and ownership

The source-use register is Rust-owned control-plane data. Go owns observation
payloads and stores immutable references to approved policy versions. Proposed
minimum Rust DDL (IDs are application-generated):

```sql
CREATE TABLE audit_source_policies (
  id UUID NOT NULL,
  version BIGINT NOT NULL CHECK (version > 0),
  source_key TEXT NOT NULL,
  purpose TEXT NOT NULL CHECK (purpose IN ('verify_private', 'licensed_shared')),
  state TEXT NOT NULL CHECK (state IN ('draft', 'active', 'revoked', 'expired')),
  agreement_ref TEXT NOT NULL,
  allow_export BOOLEAN NOT NULL DEFAULT false,
  allow_derivatives BOOLEAN NOT NULL DEFAULT false,
  retention_days INTEGER NOT NULL CHECK (retention_days BETWEEN 1 AND 365),
  valid_until TIMESTAMPTZ NOT NULL,
  reviewed_by TEXT NOT NULL,
  reviewed_at TIMESTAMPTZ NOT NULL,
  PRIMARY KEY (id, version),
  UNIQUE (source_key, purpose, version)
);
CREATE TABLE audit_source_grants (
  tenant_id UUID NOT NULL REFERENCES tenants(id),
  policy_id UUID NOT NULL,
  policy_version BIGINT NOT NULL,
  enabled BOOLEAN NOT NULL DEFAULT false,
  PRIMARY KEY (tenant_id, policy_id, policy_version),
  FOREIGN KEY (policy_id, policy_version)
    REFERENCES audit_source_policies(id, version)
);
CREATE TABLE audit_deletion_requests (
  tenant_id UUID NOT NULL REFERENCES tenants(id),
  id UUID NOT NULL,
  request_key UUID NOT NULL,
  target_kind TEXT NOT NULL CHECK (target_kind IN ('audit', 'tenant')),
  target_id UUID NOT NULL,
  state TEXT NOT NULL CHECK (state IN
    ('requested', 'tombstoned', 'purging', 'waiting_external', 'completed', 'blocked')),
  version BIGINT NOT NULL DEFAULT 1,
  requested_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  completed_at TIMESTAMPTZ,
  PRIMARY KEY (tenant_id, id),
  UNIQUE (tenant_id, request_key)
);
```

A policy revision is immutable except for state transitions to revoked/expired.
A new agreement creates a new version; it cannot relabel old evidence. Only a
platform data-rights administrator can activate source policies. Tenant admins
can disable grants or select from platform-approved versions, never self-license
shared use. `agreement_ref` points to controlled contract evidence, not a public URL.

Go's proposed observation metadata contains `observation_id`, Rust `tenant_id`,
Go `workspace_id`, `product_purpose=verify_private`, `policy_id/version`,
`subject_id`, `observed_at`, `retrieved_at`, `retention_until`, `artifact_ref`,
`content_sha256`, and `tombstoned_at`. Add indexes on `(workspace_id,subject_id)`
and `(retention_until,observation_id) WHERE tombstoned_at IS NULL`. Keep licensed
shared observations in a distinct namespace; tenant derivatives retain tenant
ownership even if the source itself is shared.

Add `audit_deletion_items` keyed by `(tenant_id,request_id,service,resource_kind,
resource_id)` with state, attempt count, next-attempt time, receipt hash, and deadline.
FK `(tenant_id,request_id)` references the request. Items must enumerate uploads,
rows, observations, caches, exports, CRM previews, evaluation identifiers, and
queued operations. Do not store deleted names/emails in receipt/error text.

### Enforcement interfaces

| Proposed interface | Auth | Contract |
|---|---|---|
| `GET /v1/audit-source-grants` | `audits.read` | Enabled source keys and permitted purposes; no confidential agreement contents. |
| `PUT /v1/audit-settings/retention` | `audits.manage` | `expected_version`, retention days 1–365; actual expiry is the minimum of tenant and source retention. |
| `DELETE /v1/audits/{audit_id}` | `audits.manage` | Idempotent 202 deletion operation; stop visibility/admission before acknowledging. |
| `GET /v1/audit-deletions/{id}` | `audits.manage` | Per-service counts, state, unresolved obligations, deadline; no deleted PII. |
| Internal Rust `POST /internal/verify/source-authorizations` | Trusted Go service | Tenant/workspace binding, source policy, purpose, operation ID and evidence-use kind; authorize one operation or deny. |
| Go `POST .../internal/verifier-deletions` | Trusted Rust service | Deletion ID, tenant/workspace, target IDs, contract version; durable tombstone before response. |
| Go `GET .../internal/verifier-deletions/{id}` | Trusted Rust service | Receipt version and pending/completed item counts; repeat lookup is side-effect free. |

The Go `...` prefix is `/lead-scraper-microservice/api/v1`. New Rust internal
routes are not public gateway routes and require a separate scoped service
credential; neither `rch_live_` customer keys nor web JWTs may call them.

Source authorization checks active version, enabled tenant grant, purpose, permitted
export/derivative flags, expiry, deletion state, and matching tenant/workspace.
Authorization is a one-operation durable record, expires after 60 seconds if not
used, and must be revalidated immediately before provider dispatch. Revocation is
linearized at admission: it prevents authorizations after its transaction commits.
Already authorized in-flight calls may finish, but revoked evidence cannot be reused
or exported. Rust export/preview reads current policy directly, without a stale
cache. Go outages or unknown policy versions deny new work with a typed reason.

### Deletion algorithm and restoration

1. Lock the tenant/audit admission fence. Persist the request, mark the audit
   deleting, increment generation/policy epoch, invalidate artifact handles, and
   insert deletion dispatch in one transaction. Return 202 only after commit.
2. Workers stop new dispatch; revoke unused authorizations. An in-flight paid call
   is reconciled for cost, but its result cannot repopulate a tombstoned subject.
3. Send the Go tombstone with the stable deletion ID. Go uses an insert-or-read
   receipt and fences executors by subject/tombstone version before any result write.
4. Purge enumerated objects in batches of 100. Retry transient failures with full
   jitter, 1 second to 5 minutes. After 24 hours unresolved, move to `blocked`, alert
   the operator, and continue recovery; never mark the entire deletion complete
   merely because Rust succeeded.
5. Finish when all live stores are purged and external obligations are recorded.
   Return `completed` with a separately visible `backup_expiry_at` when backups
   cannot be selectively erased. An unknown backup deadline prevents completion.

Restore procedures load deletion tombstones **before** restoring application
readability and replay them before enabling workers. Backup expiry follows actual
infrastructure policy; the initial target is at most 30 days, and rollout must fail
its retention gate if operations cannot meet the agreed deadline. Deletion receipts
retain opaque IDs and counts for 365 days unless an approved policy requires less;
no deleted record payload is retained for convenience.

Suppression retention is separate: retain only the tenant-scoped mailbox identity
or approved keyed marker necessary to prevent recontact, with documented lawful
retention. Do not claim that a bare hash is anonymous. Any identity/marker migration
must update all suppression checks together and preserve existing blocks.

### Migration, observability, and release cases

Migrate metadata nullable first, inventory existing Go evidence, then mark legacy
unknown-rights observations `restricted`. Enable enforcement per source after
verified backfill; never turn null into an active license. Add retention/deletion
sweepers with leased work and a five-minute tick. Rollback disables new admissions
but leaves deletion, revocation, and settlement workers running.

Metrics: authorization denials by reason, stale policy versions, oldest pending
deletion, rows pending per store, retention lag, and restore replay failures.
Page when deletion remains blocked for 24 hours or an active observation exceeds
its permitted retention by more than one sweep interval.

| Test | Required observable result |
|---|---|
| Other-tenant subject or finance-purpose reader | 404/403 before artifact read; zero provider calls. |
| Revocation racing dispatch/export | New authorization denied; in-flight result retained only as permitted cost metadata; export withheld. |
| Worker finishes after tombstone | Conditional evidence insert fails; no resurrected row/artifact. |
| Go down after Rust deletion commit | Request remains pending/blocked; retries reuse deletion ID. |
| Restore backup containing deleted evidence | Tombstone replay removes visibility before app/worker readiness. |
| Source retention shorter than tenant default | Earlier deadline wins in both stores and exports. |

Implement a Go/Rust contract fixture for each case and a staging deletion drill
covering one complete imported audit before enabling self-service retention claims.
