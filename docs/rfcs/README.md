# RFCs

This directory contains engineering RFCs for the next ten backend/API-first product features under consideration for Reacher. The documents are intentionally implementation-oriented: each RFC makes concrete API, storage, auth, rollout, and testing choices so engineering can estimate and implement without reopening core product decisions.

| RFC | Title | Status | Summary |
|---|---|---|---|
| [0001](0001-native-provider-webhooks-and-outcome-adapters.md) | Native Provider Webhooks and Outcome Adapters | Draft | Add first-wave ESP webhook ingestion for normalized delivery outcomes. |
| [0002](0002-deliverability-feedback-and-recommendations.md) | Deliverability Feedback and Recommendations | Draft | Add deterministic per-email recommendation payloads on top of existing scoring. |
| [0003](0003-reverification-policies-by-segment.md) | Reverification Policies by Segment | Draft | Add tenant-scoped scheduled reverification policies for lists, pipelines, and query filters. |
| [0004](0004-suppression-intelligence.md) | Suppression Intelligence | Draft | Expand suppressions with reason, provenance, expiry, and auditability. |
| [0005](0005-list-quality-scoring.md) | List Quality Scoring | Draft | Formalize list-level quality scorecards and cached summaries. |
| [0006](0006-source-level-quality-comparison.md) | Source-Level Quality Comparison | Draft | Attribute quality to acquisition/import sources and expose comparison analytics. |
| [0007](0007-verification-policy-modes.md) | Verification Policy Modes | Draft | Add explicit `strict`, `balanced`, and `aggressive` policy evaluation modes. |
| [0008](0008-bulk-remediation-workflows.md) | Bulk Remediation Workflows | Draft | Add remediation plans and exportable partitions for cleaned lists. |
| [0009](0009-tenant-facing-job-observability.md) | Tenant-Facing Job Observability | Draft | Add unified operational visibility for jobs, retries, failures, and latencies. |
| [0010](0010-idempotent-integration-ux.md) | Idempotent Integration UX | Draft | Standardize idempotency semantics across write-heavy endpoints. |

## Recommended Build Order

1. [0005 List Quality Scoring](0005-list-quality-scoring.md)
2. [0002 Deliverability Feedback and Recommendations](0002-deliverability-feedback-and-recommendations.md)
3. [0007 Verification Policy Modes](0007-verification-policy-modes.md)
4. [0004 Suppression Intelligence](0004-suppression-intelligence.md)
5. [0003 Reverification Policies by Segment](0003-reverification-policies-by-segment.md)
6. [0009 Tenant-Facing Job Observability](0009-tenant-facing-job-observability.md)
7. [0010 Idempotent Integration UX](0010-idempotent-integration-ux.md)
8. [0008 Bulk Remediation Workflows](0008-bulk-remediation-workflows.md)
9. [0006 Source-Level Quality Comparison](0006-source-level-quality-comparison.md)
10. [0001 Native Provider Webhooks and Outcome Adapters](0001-native-provider-webhooks-and-outcome-adapters.md)

This order favors customer-visible list hygiene value first, operational safety second, and external webhook dependencies last.

## Data RFCs (Hunter.io Parity)

These RFCs add finder, search, and enrichment features on top of the Parallel Web Systems API. They serve our buyer: teams that already have a list and send email to it (see `docs/strategy/company-focus.md`). RFC 0015 defines the shared foundations. The four feature RFCs build on it.

| RFC | Title | Status | Summary |
|---|---|---|---|
| [0015](0015-data-platform-foundations.md) | Data Platform Foundations | Draft | Parallel client, durable data jobs, data credits, spend guardrails, public web store, privacy controls, Legal Review, test and UI foundations. |
| [0011](0011-email-finder-parity.md) | Email Finder Parity And List Repair | Draft | Find addresses from web sources, learned domain patterns, and tenant outcomes, with calibrated confidence. Repair whole lists. |
| [0012](0012-domain-search.md) | Domain Search | Draft | List the people and generic addresses at any domain, with sources, verification, departures, and export. |
| [0013](0013-discover-company-and-people-search.md) | Discover: Company And People Search | Draft | Natural-language and filter search, lookalikes from list data, a sensitive-condition guard, and export. |
| [0014](0014-person-and-company-enrichment.md) | Person And Company Enrichment, And Job-Change Monitoring | Draft | Work data with citations, a high-precision job-change signal, and a scheduled monitor for lists. |

Recommended build order:

1. [0015 Data Platform Foundations](0015-data-platform-foundations.md): all data RFCs need it. Start the Legal Review on day one.
2. [0011 Email Finder Parity And List Repair](0011-email-finder-parity.md): fills the store and gives list repair to our buyer.
3. [0014 Person And Company Enrichment](0014-person-and-company-enrichment.md): the strongest fit with our buyer. Build RFC 0012 milestones D2 (title taxonomy) and D3 (departures) at the start of this step, because RFC 0014 uses them.
4. [0012 Domain Search](0012-domain-search.md): the remaining milestones.
5. [0013 Discover](0013-discover-company-and-people-search.md): the weakest fit with our buyer and the highest cost.

Each data RFC has a Definition Of Done with end-to-end tests, live staging targets, and a Hunter benchmark. The RFC 0015 Legal Review is a release gate for all of them.
