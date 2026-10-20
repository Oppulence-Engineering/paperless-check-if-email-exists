# Paperless product focus

Updated 2026-10-20. This replaces the sender-only positioning from 2026-10-16.

Paperless helps small B2B agencies, founders, and sales teams turn target-account
lists into relevant, current contacts with verifiable evidence and predictable
costs. Verification and ongoing list hygiene remain supported workflows.

## First connected workflow

Upload company names/domains and optional known people, choose the target role and
spending limit, review the research plan, run research, review evidence and mailbox
status, and export approved contacts. Missing, ambiguous, stale, and conflicting
evidence must remain visible. A deliverable mailbox does not establish a person's
identity, employment, purchasing responsibility, consent, or inbox placement.

The first release combines company-list research, person matching, email finding,
list repair, verification, evidence review, and CSV export. Broad company discovery,
employment-change monitoring, and CRM synchronization follow measured pilot value.

## Existing systems and ownership

- The existing Next.js verification application is the customer entry point.
- The Go Paperless backend owns company resolution, research, source observations,
  and contact discovery. Reuse its research sessions, import parsing, and spend controls.
- The Rust backend owns verification operations, tenant quotas, lists, suppression,
  outcome policies, and clean exports. All deployed mailbox verification uses
  MillionVerifier; errors never fall back to SMTP or a proxy.
- Workspace mappings and service credentials stay server-side. One customer must
  never read another customer's evidence or pay for their provider calls.

Each contact separates mailbox verification, person identity, employment, and role
relevance, with sources and observation dates for the facts actually established.
An inferred address and a published address are different kinds of evidence.

## Spending and rights

Prefer permitted existing observations and first-party website evidence before
paid research. Bound candidate verification and stop once the requested contacts
are found. Show the maximum customer charge before a run; persist provider attempts
and reuse completed operations on retries.

Do not use Parallel output to populate a shared commercial contact database. Its
customer-private integration allowance does not itself establish permission for
all contact-data products. Enable a provider only for the uses covered by its
contract; the initial integration must work without assuming a Parallel waiver.

The proposed $19/$39/$79 plans are hypotheses, not implemented entitlements or
claims of sustainable margins. Set included research usage after measuring cost
per usable contact, including unsuccessful searches, verification, and review.

## Evidence of success

North-star metric: total cost per correctly matched, usable B2B contact. A usable
contact meets the requested role, has sufficient current person/company evidence,
and passes the customer's verification and suppression policy.

Evaluate coverage, false matches, stale roles, uncertain outcomes, cost, and manual
review time on a stratified benchmark and paid pilots. Do not advertise superior
accuracy or cost before the comparison supports it.

Execution and acceptance criteria: [Go integration](../paperless-research-integration.md).
