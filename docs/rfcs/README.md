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
