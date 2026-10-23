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
