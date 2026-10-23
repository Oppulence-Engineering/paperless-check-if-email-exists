# RFC 0032: Preview-First HubSpot Writeback

| Field | Value |
|---|---|
| Status | Proposed; demand-gated stage 3 |
| Updated | 2026-10-23 |
| Owners | Rust integration lead and web lead |
| Depends on | RFCs 0027, 0029, 0030; requested workflow from repeat-audit customers |
| Shared prerequisite | RFC 0023 tenant secret cipher only, or an equivalent reviewed credential service |

## Problem and scope

CSV proves audit value first. When repeat buyers ask to avoid manual merging,
support a limited HubSpot correction workflow with the same evidence and policy
gates as CSV. This RFC proposes new functionality; no HubSpot research writeback
was found in the [audit](../audits/2026-10-23-verify-north-star.md).

Start with updates to explicitly matched existing contacts/companies. Creating
contacts, merges, deletion, sequences, campaign enrollment, and general bidirectional
CRM synchronization are outside the first release.

## Connection and change contract

Store each HubSpot connection in the Rust product backend, scoped to tenant and
optionally a client/workspace mapping. Use minimum required OAuth scopes, encrypted
server-side credentials, state validation, rotation, revocation, and explicit
installation authority. Better Auth sign-in tokens are not CRM credentials.
Reuse the cipher slice of RFC 0023; its full webhook outbox is not a prerequisite.

Propose connection routes plus `/v1/audits/{id}/crm-previews` and an explicit apply
operation. A preview includes the portal/connection, source row and CRM object IDs,
current CRM values, proposed fields, before/after changes, evidence references,
conflicts, and eligibility. Resolve mappings by retained CRM IDs and corroborating
identity evidence; do not choose an ambiguous object by email/name alone.

Default to filling approved blank fields. Overwriting a nonblank value requires
an explicit field-level choice. Never overwrite CRM opt-out, subscription, or
consent fields or infer permission to contact from a valid mailbox. Customer do-not-
contact signals block the update or require review according to the agreed contract.

## Apply and recovery

Persist a versioned plan and actor approval. At apply time, reload current CRM
values, connection authority, evidence, and sending/suppression policy. Refuse
stale plans/conflicting fields and show a refreshed diff. HubSpot concurrency
semantics must be verified: if conditional writes are unavailable, expose the
remaining race and limit the first release to a reviewed field allowlist with
immediate read-before-write and read-after-write checks. Do not promise atomicity.

Use a durable operation ledger per tenant/connection/object/plan version, stable
retry keys, rate-limit backoff, and per-object outcomes. An ambiguous timeout must
read/reconcile the remote state before replay. Partial success is visible and
retrying failed rows cannot rewrite successful ones. Stop new work on revocation.
Any reversal is a newly previewed change against current CRM state, not a blind
rollback that erases subsequent customer edits.

## Rollout and acceptance

1. Verify current API terms, scopes, rate limits, sandbox behavior, and field update
   semantics; record them before implementation. Keep credentials out of logs/UI.
2. Ship read-only matching/diff preview for two requesting design partners.
3. Enable a small, approved field allowlist with explicit apply and per-row history;
   expand only with measured reduction in customer merge time.

Test cross-tenant/portal confusion, OAuth state/revocation, ambiguous matches,
missing original CRM IDs, concurrent CRM edits, late suppressions/opt-outs,
permission changes, rate limits, partial failures, and timeout reconciliation.
Validate previews with sandbox contracts before any bounded pilot. Regenerate
API/SDK contracts and preserve CSV as the recovery path. RFC 0028 records customer
time saved and added integration/support cost; a connector alone is not evidence
of retention or margin.
