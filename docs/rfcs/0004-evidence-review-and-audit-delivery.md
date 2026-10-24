# RFC 0004: Evidence Review and Audit Delivery

| Field | Value |
|---|---|
| Status | Proposed; stage 1b |
| Updated | 2026-10-24 |
| Owners | Go evidence lead, Rust policy lead, web lead |
| Depends on | RFC 0003 lineage; RFC 0001 permitted evidence access |
| Reuses | RFCs 0030/0027/0026/0028 implemented policy, suppression, and remediation primitives |
| Narrows | RFC 0008 claim-level enrichment; recurring monitoring belongs to RFC 0007 |

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
Research requests create work requests, not paid execution; RFC 0005 authorizes it.

New evidence, expired evidence, a changed role target, changed policy, or a newer
mailbox operation invalidates the old approval until re-evaluated. Preserve the
old decision in history. Reviewer approval cannot bypass suppression, complaints,
missing required evidence, catch-all uncertainty, or failed/unknown verification.

## Proposed interfaces

Under `/v1/audits/{id}`, add paginated rows/claim history, an idempotent
`POST /rows/{row_id}/decisions`, an export preview, and an export-artifact creation
route. Use RFC 0003 read/review/export scopes and tenant ownership checks.
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
Use existing CSV escaping and RFC 0001 retention/access controls for artifacts.

## Acceptance and rollout

1. Ship claim history and persisted decisions behind an audit feature flag.
2. Ship preview plus reconciled three-part delivery for internal fixtures, then a
   paid pilot. Write lineage/decision events for RFC 0002 measurement.
3. Promote after review and export agree across reloads, retries, and concurrent
   reviewers, with every source row accounted for.

Tests must cover wrong company, stale employment, title-only relevance, generic
inbox, catch-all, conflicting sources, a selected alternative candidate, late
complaint/suppression, policy change, newer pending verification, deleted evidence,
formula injection, and cross-tenant artifact access. A low-quality record cannot
become Ready by clicking approve. Production acceptance requires a bounded real
pilot with both a supported Ready case and withheld cases; mocks are insufficient
evidence of contact accuracy.

## Technical specification

Apply [Verify technical contracts v1](verify-technical-contracts.md). The review
projection is Rust-owned; Go remains the source of research observations. Reuse
existing verification history, suppression resolution, and feedback policy rather
than introducing another mailbox verdict store.

### Candidate and evidence contract

Propose Go `claims_v1` responses with this minimum envelope:

```json
{
  "contract_version": 1,
  "external_row_id": "40000000-0000-4000-8000-000000000001",
  "candidate_id": "50000000-0000-4000-8000-000000000001",
  "evidence_version": "7",
  "claims": [{
    "claim_id": "60000000-0000-4000-8000-000000000001",
    "kind": "employment",
    "state": "supported",
    "basis": "assertion",
    "subject_ref": "candidate:50000000-0000-4000-8000-000000000001",
    "source_policy_id": "70000000-0000-4000-8000-000000000001",
    "source_policy_version": "2",
    "source_ref": "opaque-source-reference",
    "observed_at": "2026-10-23T10:00:00Z",
    "retrieved_at": "2026-10-23T10:01:00Z",
    "expires_at": "2026-11-22T10:00:00Z",
    "contradicts": []
  }]
}
```

The complete envelope includes all required claim kinds, provenance and typed
asserted values; the example shows one observation only. `source_ref` resolves
through an authorized evidence endpoint, not an arbitrary URL to fetch server-side.
Kinds are `company_identity`, `person_identity`, `employment`, `role_relevance`,
and `mailbox_ownership`. States are `supported`, `conflicting`, `unknown`, `stale`;
basis is `assertion` or `inference`. Absence is `unknown`, never supported. A role
claim records the exact criteria version and whether it supports title relevance
or the requested responsibility. Mailbox reachability is a separate Rust result.

Version observations immutably. Importing an older Go snapshot cannot replace a
newer projection; duplicate same-version/different-hash responses are integrity
errors. Refetch by candidate/version after a notification gap. Rust stores only
fields needed for review/export, source-policy references, hashes, and Go evidence
references under the same retention deadline; it does not build a second crawler.

Initial configurable maximum ages are company/person identity 90 days, employment
and role 30 days, mailbox verification 7 days. These are conservative product
policy defaults to validate, not measured accuracy claims. Effective expiry is the
minimum of policy age, source expiry, consented retention and observation expiry.
Unknown observation dates are not fresh merely because retrieval happened today.
A newer pending/failed verification cannot be masked by an older successful one.

The first eligibility policy is deterministic and versioned:

| Required gate | Minimum support | Insufficient by itself |
|---|---|---|
| Company identity | Canonical domain and company name corroborated by current first-party company evidence or a permitted authoritative registry; conflicting companies unresolved. | A guessed domain or identical display name. |
| Person identity | Named person tied to the selected company by an attributable permitted source; subject identifiers remain consistent across claims. | Email local-part parsing or a generic inbox. |
| Employment | Explicit current person/company relationship with a dated observation within policy freshness; contradictions resolved. | Mailbox validity or an old cached profile without current employment evidence. |
| Role relevance | Current supported title matched by versioned exact/approved-synonym rules to target role; responsibility criteria require explicit responsibility evidence. | Unbounded substring matching or inferring buying authority from seniority. |
| Mailbox ownership | Published/attributed address linked to that same person and company, with permitted provenance. | A pattern-generated address that happens to verify. |
| Mailbox reachability/policy | Latest completed managed verification is acceptable under current policy, is not catch-all, and no newer pending attempt or suppression/complaint blocks it. | An old successful result, transport failure, or a reviewer override. |

Record the rule version and matched evidence IDs for each gate. Normalize title
case/whitespace for matching while retaining the original; synonyms are explicit
configuration entries, not an unrecorded model judgment. Inferences may propose
candidates or review questions but cannot alone satisfy a required supported claim.
Future scoring/model adapters must preserve these gate explanations and demonstrate
adjudicated quality before replacing a rule.

### Persistence and optimistic concurrency

| Table | Required columns and constraints |
|---|---|
| `audit_candidates` | Tenant/audit/input row/candidate UUID, Go candidate reference, projection version/hash, selected person/company/email fields, deletion deadline. Composite input FK; unique `(tenant_id,audit_id,input_row_id,id)`. |
| `audit_evidence_snapshots` | Tenant/candidate/evidence version PK, immutable envelope/hash, source refs, criteria version, earliest expiry. Composite candidate FK; no overwrite on conflict. |
| `audit_row_review_state` | Tenant/audit/input row PK, version, latest decision UUID, selected candidate, current disposition/reason projection, projection input hashes. Composite input FK; one locked row per input. |
| `audit_review_decisions` | Tenant/decision UUID PK, audit/input/candidate, action, reason, actor, evidence version, policy epoch, criteria version, verification operation UUID, previous decision/version, creation time. Unique `(tenant_id,input_row_id,decision_version)`; candidate FK includes audit and input. |
| `audit_export_previews` | Tenant/preview UUID PK, audit/generation, policy epoch, manifest hash, row count/partition counts, evaluation time, earliest expiry, state/version. Immutable preview rows reference input/candidate/decision/evidence/verification versions. |
| `audit_export_artifacts` | Tenant/artifact UUID PK, preview FK, operation FK, manifest/hash, byte size, private payload reference, state, created/expiry/deleted times. Never public object URLs. |

Create review-state rows with input rows, so two initial decisions lock the same
record. For `approve`, lock audit then review state in a consistent order; verify
expected version, candidate ownership, current evidence/criteria/verification,
source rights, and policy before appending the decision and incrementing state
version and audit generation. `hold`/`reject` require a reason. `request_research`
creates an unapproved repair request; it cannot create provider work. Re-evaluation
updates derived disposition without rewriting historical reviewer intent.

```http
POST /v1/audits/{audit_id}/rows/{row_id}/decisions
Idempotency-Key: <UUID>
```

```json
{
  "expected_version": "3",
  "candidate_id": "50000000-0000-4000-8000-000000000001",
  "action": "approve",
  "reason": "Employment and requested role confirmed from current evidence.",
  "evidence_version": "7",
  "criteria_version": "1"
}
```

Return 201 with decision ID, new review version and effective disposition.
A stale version returns 409 `audit_revision_conflict`; a blocked approval returns
409 `audit_not_eligible` with structured gate codes and makes no new decision.
Rejecting/holding does not require all claims to pass. Go unavailability prevents
approval when a referenced snapshot cannot be validated; it does not prevent
holding a record or reading retained permitted history.

### One eligibility evaluator

Implement `evaluate_candidate(context, snapshot, decision, now)` in
`backend/src/audits/review.rs`; use the same function for row presentation, approval,
export and CRM preview/apply. Its input explicitly includes tenant policy epoch,
source grants, newest verification operation/result, suppression/negative outcomes,
criteria version, and current time. It returns a disposition and sorted reason
codes, not a single confidence score.

Evaluate in this order:

1. Deletion/rights/retention or decisive requirement failure blocks usable output.
   Return a redacted Excluded row where evidence cannot be disclosed.
2. Active suppression, complaint, explicit rejection, wrong-company match or
   confirmed invalid mailbox produces Excluded with a specific reason.
3. Unknown/conflicting/stale claims, catch-all, pending/unknown/failed verification,
   missing responsibility evidence or outdated approval produces Needs Review.
4. Only an explicit current approval with every required gate passing is Ready.

A delivery row can retain its original ID and exclusion code while personal fields
are redacted under rights/deletion policy. Required `named_person` criteria reject
generic inboxes. A transport failure is not a confirmed invalid mailbox. Confirmed
wrong employment is a requirement failure for that candidate; an input row with
other unresolved candidates can remain Needs Review. Evaluate row disposition from
the selected candidate and explicit decision, not whichever candidate appears first.

### Snapshot export protocol

`POST /v1/audits/{id}/export-previews` (export scope) takes `expected_version` and
an explicit field profile (`reconciliation_v1` or `reconciliation_with_evidence_v1`).
Return 202 operation; build immutable preview rows in chunks of 200. Capture audit
generation and policy epoch, evaluate every committed input exactly once, then
recheck both counters before publishing. Any drift yields
`preview_invalidated`; never publish mixed generations. Preview lifetime is at most
15 minutes and never past the earliest included claim/verification expiry.

`GET /v1/audits/{id}/export-previews/{preview_id}` returns counts, reasons, manifest
hash and paginated preview rows. `POST /v1/audits/{id}/exports` accepts
`preview_id`, `manifest_hash`, and preview `expected_version`; admit an artifact
operation only after a fresh eligibility pass. A stale preview returns 409 without
silently accepting a different set of contacts. Artifact state transitions:

```text
queued -> building -> ready -> expired -> purged
                   -> invalidated
                   -> failed
ready -> invalidated -> purged
```

Manifest schema 1 contains audit/import IDs, generation, policy epoch, evaluation
and expiry times, total rows, Ready/Needs Review/Excluded counts, distinct-contact
count, duplicate-group count, file names/row counts/SHA-256 hashes, and unresolved
cost/evidence flags. Enforce `ready + needs_review + excluded = committed_inputs`
and unique `(input_row_id)` across reconciliation files before publication.
Candidate/evidence files may have many rows and must not inflate that equation.

Pilot artifacts can use a private PostgreSQL `BYTEA` payload table with an 8 MiB
output cap; fail visibly before publication if exceeded. Stage 2 replaces payload
storage with a configured private chunk/object adapter for larger artifacts;
metadata/authorization stay in Rust. Never proxy an unbounded Go response through
the existing 8 MiB bridge. The download API streams the stored snapshot rather than
recomputing CSV per request. CSV cells beginning with spreadsheet formula prefixes
(`=`, `+`, `-`, `@`, tab or carriage return after leading whitespace) are escaped;
retain exact originals only in the authorized JSON source representation.

`GET /v1/audits/{id}/exports/{artifact_id}/download` checks tenant/export scope,
deletion status, current policy/generation and effective expiry before sending any
bytes. Require `audit_export_artifacts.state = 'ready'` and a present, readable
payload reference; otherwise return a typed not-ready/expired error without streaming. Re-evaluate current Ready gates if freshness cannot be proven from those
versions. Any relevant change invalidates the entire artifact and requires a new
preview. Use `Cache-Control: private, no-store`; no reusable public/presigned URL.
The authorization point is the final check before streaming starts; already sent
bytes cannot be recalled if a suppression arrives mid-download. Record that time
in an access receipt. CRM apply must independently check later policy.

### Migration, failures, and verification

Do not turn transient UI selections or legacy approved exports into persisted
approvals. Existing candidates enter as unreviewed, with known legacy evidence
basis. Release candidate/evidence readers first, then decision writes, then exports.
Add indexes on `(tenant_id,audit_id,disposition,input_row_id)`, candidate snapshot
versions, and artifact expiry/state. Retention workers delete payloads and evidence
while keeping only allowed non-PII receipts. Disable export admission on rollback;
continue invalidation and purge workers.

| Test/fault injection | Required result |
|---|---|
| Two reviewers use version 3 simultaneously | One decision commits; the other receives 409. |
| Selected second candidate is eligible | CSV contains that candidate, never first-list fallback. |
| Complaint/source revocation between preview and build/download | Invalidate; no stale Ready payload starts streaming. |
| Evidence expires without a new event | Clock-based gate invalidates even if epoch is unchanged. |
| Older snapshot arrives after a newer version | Preserve newer projection; no approval resurrection. |
| Duplicate/no-match/failed input rows | Exactly one reconciliation row per original UUID. |
| Cross-audit candidate or artifact ID | 404; database FK also rejects invalid references. |
| Formula cells, multiline CSV and redacted evidence | Safe parseable output, exact manifest counts and hashes. |

Use fake-clock evaluator unit tests, PostgreSQL concurrency tests, Go contract
fixtures, and an end-to-end review/reload/export test. Record preview invalidation
reasons, approval conflicts, export row-count mismatches and oldest artifact purge
lag. Export generation must be restartable without acquiring new evidence or costs.
