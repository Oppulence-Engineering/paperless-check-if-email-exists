# RFC 0033: Recurring Contact Quality

| Field | Value |
|---|---|
| Status | Proposed; stage 4, after repeat-payment and economic gates |
| Updated | 2026-10-23 |
| Owners | Go research lead, Rust scheduling/policy lead, web lead |
| Depends on | RFCs 0027–0031 and measured repeat demand |
| Reuses | RFC 0003 segment scheduling, existing rechecks/list diffs, RFC 0014 employment observations |
| Optional | RFC 0032 for approved CRM delivery; RFC 0023 for customer event subscriptions |

## Problem and outcome

Existing [reverification](../../backend/src/reverification.rs) refreshes mailbox
results and [list diffing](../../backend/src/http/v1/lists/diff.rs) compares lists.
Neither establishes current employment, changed buying roles, or a trustworthy
replacement person. After customers repeat audits profitably, let them maintain
their approved data without re-researching every row.

## Model and scheduling

Rust owns tenant/audit-linked maintenance policies: approved cohort, target-role
version, mailbox and identity freshness thresholds, schedule/timezone, per-run and
monthly charge caps, approval mode, and pause state. Extend existing schedulers
and RFC 0003's segment work rather than introducing a second scheduling engine.

Store versioned snapshots and change proposals referencing Go observations,
review decisions, and verification operations. Select only stale or changed claims;
honor suppression and deletion before scheduling. Go executes research under the
same authorization and spend ledger as RFC 0031. Monthly caps include overlapping
runs and unresolved provider reservations.

Default recurring behavior is assessment plus a proposed repair plan. A customer
may explicitly authorize recurring bounded execution with an expiry and ceiling;
new providers, broader purposes, or higher limits require new approval. Pause or
cancel stops future work and reconciles in-flight spend.

## Change semantics

- Mailbox invalidation is distinct from person departure or changed responsibility.
- A page disappearing or a failed crawl means unknown/stale evidence, not a
  confirmed job change. Conflicting employment observations enter review.
- A corroborated role/employer change invalidates the relevant prior approval and
  produces a dated explanation. A replacement candidate gets its own identity and
  review; never silently substitute one person for another.
- Retain snapshot history and before/after diffs within source retention rights.
  Active suppression always takes precedence; a new job does not erase it.
- Notifications report actionable changes, exhausted budgets, or failed runs;
  unchanged successful checks do not need repeated alerts.

Propose `/v1/audits/{id}/maintenance-policies`, policy run history, and change-review
routes with tenant-scoped administration and RFC 0029 review/export permissions.
Start with in-app review and CSV; CRM changes require RFC 0032 preview/apply.

## Incremental rollout

1. Offer a second manually initiated audit with reusable source mappings and diffs.
   Measure repeat payment and actual maintenance demand through RFC 0028.
2. Schedule bounded mailbox freshness checks for an approved segment using existing
   infrastructure; deliver a change-review queue.
3. Add employment/role refresh for selected accounts using dated evidence and
   adjudicated precision. Add replacement research only after an approved plan.
4. Offer recurring pricing only after costs, review burden, retention, and customer
   benefit support it. The thesis's $149–$299 range remains an experiment.

## Acceptance and metrics

Tests cover overlapping schedules, daylight-saving transitions, repeated ticks,
deleted/suppressed rows, unchanged evidence, stale pages, conflicting roles,
budget exhaustion, cancellation, and idempotent restarts. Repeated checks cannot
duplicate charges, alerts, or CRM changes. Current policy is rechecked at delivery.

Measure independently adjudicated change precision, stale-contact coverage, time
to reviewed correction, full cost per maintained usable contact, customer review
minutes, repeat payments, retention, and contribution margin. Avoid treating a
mailbox result or notification count as a successful role correction. Broader
monitoring/discovery remains gated on these results.
