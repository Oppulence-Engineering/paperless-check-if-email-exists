# Verify RFCs and delivery sequence

Updated 2026-10-23 from the [company thesis](../../company.md), [vision](../../vision.md), and [codebase audit](../audits/2026-10-23-verify-north-star.md).

**This is the authoritative build order. RFC numbers are stable identifiers, not priority ranks.** Preserve filenames and links. Dated disposition sections take precedence over historical baselines and proposals. RFC 0016 and the old feature backlog no longer define the product sequence.

The destination is a repeatable paid audit of customer-owned prospect data: import → assess identity → selectively repair → verify → review → deliver Ready, Needs Review, and Excluded records → maintain approved data after repeat demand. Optimize total cost per correctly matched, usable, role-relevant contact, including failed research and human review.

Read the [shared technical contracts](verify-technical-contracts.md) before
implementing RFCs 0027–0033. Their technical sections define proposed storage
constraints, typed APIs, state machines, transaction boundaries, failure recovery,
migrations and test gates. They are implementation specifications, not claims that
the new APIs or cross-service contracts already exist.

## Build order

| Order | RFC / increment | Depends on | Customer result and exit gate |
|---|---|---|---|
| 0a | [0027 Data Rights and Product Boundaries](0027-data-rights-and-product-boundaries.md), pilot slice | Existing tenant bridge | Rights-cleared sources, explicit Verify/finance separation, retention and tested deletion procedure. No expanded provider/shared use before this gate. |
| 0b | [0028 Paid Audit Validation and Unit Economics](0028-paid-audit-validation-and-unit-economics.md), protocol and concierge pilot | 0027 pilot gate for real data | Scoped paid audit, baseline, independent adjudication, full-cost log. Interviews and synthetic tooling can start immediately. |
| 1a | [0029 Agency Audits and Import Lineage](0029-agency-audits-and-import-lineage.md) | 0027; 0028 identifiers | Client/campaign context, original IDs and fields, visible duplicates/errors, linked research. Import preview makes no paid calls. |
| 1b | [0030 Evidence Review and Audit Delivery](0030-evidence-review-and-audit-delivery.md) | 0029; permitted evidence | Durable decisions, chosen candidates, claim history, three reconciled export groups. Current policy checked at delivery. |
| 2 | [0031 Budgeted Selective Repair and Batch Execution](0031-budgeted-selective-repair-and-batch-execution.md) | 0027 automated enforcement; 0029/0030; 0028 costs | Approved priced plans, no duplicate purchases, tested batch expansion, and a representative benchmark without weakening quality gates. |
| 3 | [0032 Preview-First HubSpot Writeback](0032-preview-first-hubspot-writeback.md), only when requested | 0027/0029/0030; repeat-audit evidence; 0023 cipher slice or equivalent | Previewed, explicitly applied corrections to matched CRM records. CSV remains available. |
| 4 | [0033 Recurring Contact Quality](0033-recurring-contact-quality.md) | 0027–0031; repeat-payment/economic gates | Targeted refresh, reviewed role changes, bounded recurring spend, and change exports. CRM delivery optionally uses 0032. |

RFC 0028 continues across every stage. Before broader investment require **five paid audits, three repeat-paying customers, positive contribution margin**, and a pre-registered improvement in correctly matched approved contacts or manual cleanup time without sacrificing quality. The approximately 1,200-account, three-segment benchmark is a future target. The $249 audit and $149–$299 recurring offer are experiments, not deployed entitlements.

Each increment must deliver usable value before expansion. No new data platform, broad discovery UI, sender analytics, MCP service, or CRM connector blocks a rights-cleared manual audit using the existing bounded pilot. Current source limits remain ten companies, three candidates per company, and thirty verifications per run.

```mermaid
flowchart LR
  R27[0027 Rights and boundaries] --> R28[0028 Paid audit measurement]
  R28 --> R29[0029 Imports and lineage]
  R29 --> R30[0030 Review and delivery]
  R30 --> R31[0031 Budgeted repair and scale]
  R31 --> G[Paid and repeat-customer gates]
  G --> R32[0032 Requested CRM writeback]
  G --> R33[0033 Recurring quality]
```

The map shows the default product sequence. Requested CRM work can use reviewed CSV-era audits without full automated repair if repeat demand and its own prerequisites are proven. Rights checks and measurement continue throughout.

## Existing foundations: reuse before rebuilding

“Implemented baseline” means inspected code provides the central primitive, not that every original acceptance criterion or live deployment is certified. “Partial” leaves named gaps. The audit links source and validation evidence.

| RFC | Disposition | Next action |
|---|---|---|
| [0001 Native Provider Webhooks](0001-native-provider-webhooks-and-outcome-adapters.md) | Implemented baseline | Preserve SendGrid/SES/Mailgun/Postmark adapters and receipts; reconcile remaining gaps when needed. |
| [0002 Recommendations](0002-deliverability-feedback-and-recommendations.md) | Implemented baseline | Reuse current decisions in 0030; sending advice does not establish person identity. |
| [0003 Segment Reverification](0003-reverification-policies-by-segment.md) | Partial | Workspace scheduling exists; finish segment policies with 0033 after repeat demand. |
| [0004 Suppression Intelligence](0004-suppression-intelligence.md) | Implemented baseline | Reuse reasons, provenance, expiry, and events through review/export. |
| [0005 List Quality](0005-list-quality-scoring.md) | Partial | Existing aggregates/quality view are reusable; cached scorecards do not precede full-cost evaluation. |
| [0006 Source Quality](0006-source-level-quality-comparison.md) | Partial | Existing attribution/reporting is reusable; contact-audit comparisons belong to 0028. |
| [0007 Verification Policies](0007-verification-policy-modes.md) | Implemented baseline; enum changed | Preserve growth/deliverability/signup_protection/enterprise_strict/custom; old mode names are historical. |
| [0008 Bulk Remediation](0008-bulk-remediation-workflows.md) | Implemented baseline | Reuse original rows and partition exports; extend contact review through 0029/0030. |
| [0009 Job Observability](0009-tenant-facing-job-observability.md) | Implemented baseline | Reuse events, retries, latency/failure views; extend research progress for 0031. |
| [0010 Idempotent Integration UX](0010-idempotent-integration-ux.md) | Partial | Reuse keys/replay/conflicts and durable operations; validate new cross-service behavior in 0031. |

The old source-quality scope-check task is already covered by shared auth middleware (`sources → bulk`). It is not outstanding work.

## Research and adjacent RFC dispositions

| RFC | Disposition and sequence | Dependency / promotion gate |
|---|---|---|
| [0011 Email Finder and Selective List Repair](0011-email-finder-parity.md) | Partial; narrow repair slices in stage 2 | 0027 rights + 0031 budgets. Existing finder/pilot are foundations; calibrated identity confidence is unproven. |
| [0014 Person and Company Enrichment](0014-person-and-company-enrichment.md) | Split: evidence in stage 1b, repair in 2, monitoring in 4 | 0030/0031/0033. A standalone Domain Search UI is not required for role evidence. |
| [0015 Data Platform Foundations](0015-data-platform-foundations.md) | Superseded architecture; bridge exists | Go owns research. Remaining rights/privacy and spend/scale work moves to 0027/0031. |
| [0019 Suppression Scopes](0019-suppression-scopes.md) | Conditional stage 2 extension | 0029 client context + 0030 delivery. Build before promising client-specific DNC in one workspace; use separate workspaces where isolation is needed. |
| [0018 Bounce Taxonomy](0018-bounce-reason-taxonomy.md) | Conditional correctness work | Promote recipient classification for demonstrated wrong exclusions; sender alerts wait for 0017. Do not weaken complaints/suppressions to raise yield. |
| [0021 Mailbox Provider Breakdown](0021-mailbox-provider-breakdown.md) | Optional attribution fix; reports deferred | MX detection can improve evidence; reports require customer demand and suitable outcome data. |
| [0023 Webhook Subscriptions](0023-webhook-event-subscriptions.md) | Cipher slice at stage 3; delivery log conditional | Extract credential storage for 0032 if needed; full audit/research event delivery follows integration demand. |
| [0012 Domain Search](0012-domain-search.md) | Deferred | Promote narrow coverage after 0028 finds an audit gap; reuse 0031 rights/spend controls. |
| [0013 Discover](0013-discover-company-and-people-search.md) | Parked | Broad acquisition requires repeat audit demand and independent economics/rights evidence. |
| [0016 Deliverability Roadmap](0016-deliverability-intelligence-roadmap.md) | Superseded as build order | Retained feature inventory; this index controls delivery. |
| [0017 Domain/IP Health Monitor](0017-domain-and-ip-health-monitor.md) | Deferred | Sender demand, blocklist terms, non-email alerts. No audit dependency. |
| [0022 Bot Engagement Flag](0022-bot-engagement-flag.md) | Deferred | 0021 gateway attribution, real engagement fixtures, sender demand. Engagement does not establish identity. |
| [0024 MCP Server](0024-mcp-server.md) | Deferred | Agent-channel demand, stable audit APIs, scoped auth, explicit paid-action authorization. |
| [0020 Signup Protection V2](0020-signup-protection-v2.md) | Parked | Different buyer; revisit with evidence beyond the agency audit. |
| [0025 beehiiv and Kit](0025-beehiiv-and-kit-integrations.md) | Parked | Newsletter demand, 0019/0023, platform rights/auth. Not a substitute for requested CRM handoff. |
| [0026 Postmaster/SNDS](0026-postmaster-tools-and-snds-ingestion.md) | Parked | Sender demand, 0017/0021/0023, confirmed provider access. |

## Completion rules

For each promoted RFC, name an owner and exact incremental scope. Record API/SDK compatibility, tenant/role authorization, retry and budget failure behavior, representative workflow tests, and customer/metric outcomes. Use additive migrations and feature flags where appropriate. Distinguish source inspection, mock tests, deployed checks, and customer evidence. Update this index and the RFC disposition together; a related primitive does not mean the complete proposal shipped.
