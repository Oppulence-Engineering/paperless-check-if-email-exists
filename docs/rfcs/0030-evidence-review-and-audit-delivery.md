# RFC 0030: Evidence Review and Audit Delivery

| Field | Value |
|---|---|
| Status | Proposed; stage 1b |
| Updated | 2026-10-23 |
| Owners | Go evidence lead, Rust policy lead, web lead |
| Depends on | RFC 0029 lineage; RFC 0027 permitted evidence access |
| Reuses | RFCs 0002/0004/0007/0008 implemented policy, suppression, and remediation primitives |
| Narrows | RFC 0014 claim-level enrichment; recurring monitoring belongs to RFC 0033 |

## Problem and outcome

The [research panel](../../web/app/(product)/app/research/_components/research-panel/research-panel.tsx)
shows evidence and selects rows for download, but selections disappear on reload.
The [API](../../backend/src/http/v1/research.rs) has no review-decision mutation,
exports only approved/review modes, and selects the first eligible candidate.
An agency audit needs an explicit selected contact, durable decisions, corrected
source records, and a complete explanation for rows that cannot be used.

## Claim and review model

Go owns versioned claim observations for company/domain identity, person identity,
employment, role relevance, and mailbox ownership evidence. Each observation has
claim/subject IDs, source and permitted-use reference, observed/retrieved dates,
evidence reference, assertion vs inference, and supported/conflicting/unknown/stale
state. Rust references these versions and the separate verification operation.

A title match is evidence of title relevance, not proof of purchasing authority.
When the requested criterion is buying responsibility, require evidence specific
to that criterion or hold the record. A generic inbox cannot satisfy a named-person
requirement. Existing status strings become legacy observations with their actual
basis; never backfill invented confidence or independent confirmation.

Rust owns append-only `audit_review_decisions`: tenant, audit/input row, chosen
candidate, action (`approve`, `hold`, `reject`, `request_research`), reason, actor,
time, evidence version, policy version, and expected prior decision version.
Use optimistic concurrency: stale writes return 409 with a refresh instruction.
Research requests create work requests, not paid execution; RFC 0031 authorizes it.

New evidence, expired evidence, a changed role target, changed policy, or a newer
mailbox operation invalidates the old approval until re-evaluated. Preserve the
old decision in history. Reviewer approval cannot bypass suppression, complaints,
missing required evidence, catch-all uncertainty, or failed/unknown verification.

## Proposed interfaces

Under `/v1/audits/{id}`, add paginated rows/claim history, an idempotent
`POST /rows/{row_id}/decisions`, an export preview, and an export-artifact creation
route. Use RFC 0029 read/review/export scopes and tenant ownership checks.
The web presents original vs corrected fields, source dates and contradictions,
candidate selection, decisions, and a remaining-work queue.

An export preview binds row/candidate decisions, current evidence and policy
versions, and exclusions. Creating an export re-evaluates eligibility; a changed
record returns a refreshed preview or is explicitly withheld, never silently
included under an earlier approval. Downloaded snapshots show their evaluation
time and expiry; they do not claim to remain safe indefinitely. Revalidate before
new download access and invalidate stale artifacts when current policy disallows
their Ready contents. CRM writes separately recheck at execution.

## Delivery contract

| Partition | Rule |
|---|---|
| Ready | An explicit current approval plus sufficient company/person/employment/role evidence and fresh acceptable verification, with no active suppression or policy block. |
| Needs Review | Missing, contradictory, stale, failed, unknown, or catch-all evidence; held, unfinished, or research-requested rows. No match alone is not proof that a contact does not exist. |
| Excluded | Explicit rejection, suppression, or a decisive failure of the audit's requirements; include the reason. |

Every committed input row has exactly one terminal delivery disposition. Candidate
evidence can be a separate one-to-many file; the reconciliation file keeps one row
per original input. Default one approved contact per input; multiple target-role
slots require explicit slots and unique contact counting rather than inflating yield.

CSV files include audit/import/input IDs, customer original ID and row number,
original and corrected values, candidate ID, decision/actor/time, reasons, source
references and dates, claim states, verification result/history reference, and
policy/evidence versions. A manifest records all row counts, duplicate groups,
artifact hashes, creation time, and incomplete measurements. Every original row
must reconcile across the three groups, including duplicates and failed research.

Keep `approved`/`review` research exports and email-list `fixed/safe/review/drop`
exports backward compatible. Do not relabel old exports as the complete audit.
Use existing CSV escaping and RFC 0027 retention/access controls for artifacts.

## Acceptance and rollout

1. Ship claim history and persisted decisions behind an audit feature flag.
2. Ship preview plus reconciled three-part delivery for internal fixtures, then a
   paid pilot. Write lineage/decision events for RFC 0028 measurement.
3. Promote after review and export agree across reloads, retries, and concurrent
   reviewers, with every source row accounted for.

Tests must cover wrong company, stale employment, title-only relevance, generic
inbox, catch-all, conflicting sources, a selected alternative candidate, late
complaint/suppression, policy change, newer pending verification, deleted evidence,
formula injection, and cross-tenant artifact access. A low-quality record cannot
become Ready by clicking approve. Production acceptance requires a bounded real
pilot with both a supported Ready case and withheld cases; mocks are insufficient
evidence of contact accuracy.
