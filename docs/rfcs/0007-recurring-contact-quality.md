# RFC 0007: Recurring Contact Quality

| Field | Value |
|---|---|
| Status | Proposed; stage 4, after repeat-payment and economic gates |
| Updated | 2026-10-24 |
| Owners | Go research lead, Rust scheduling/policy lead, web lead |
| Depends on | RFCs 0001–0005 and measured repeat demand |
| Reuses | RFC 0014 segment scheduling, existing rechecks/list diffs, RFC 0008 employment observations |
| Optional | RFC 0006 for approved CRM delivery; RFC 0013 for customer event subscriptions |

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
and RFC 0014's segment work rather than introducing a second scheduling engine.

Store versioned snapshots and change proposals referencing Go observations,
review decisions, and verification operations. Select only stale or changed claims;
honor suppression and deletion before scheduling. Go executes research under the
same authorization and spend ledger as RFC 0005. Monthly caps include overlapping
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
routes with tenant-scoped administration and RFC 0003 review/export permissions.
Start with in-app review and CSV; CRM changes require RFC 0006 preview/apply.

## Incremental rollout

1. Offer a second manually initiated audit with reusable source mappings and diffs.
   Measure repeat payment and actual maintenance demand through RFC 0002.
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

## Technical specification

Use [Verify technical contracts v1](verify-technical-contracts.md), the RFC 0004
eligibility evaluator, and the RFC 0005 reservation/attempt ledger. Maintenance is
a new scheduling policy over those primitives, not a second research or billing
engine. Extend `backend/src/reverification.rs` with an audit occurrence admission
path; keep existing email-list recheck behavior compatible.

### Policy, cohort, and occurrence schema

| Table | Required fields and constraints |
|---|---|
| `audit_maintenance_policies` | Tenant/policy UUID PK, audit FK, version, cohort snapshot ID/hash, criteria/source/price versions, timezone and local schedule, next UTC due time, freshness thresholds, approval mode, per-run and monthly caps, authorization actor/expiry, paused/deleted times. |
| `audit_maintenance_members` | Tenant/policy/cohort version/input row PK, selected candidate/decision versions, enrollment/removal times. Composite input/candidate FKs enforce same audit. |
| `audit_maintenance_occurrences` | Tenant/occurrence UUID PK, policy/version, scheduled UTC time, local period key, state, operation/reservation references, counters, start/finish times, reason. Unique `(tenant_id,policy_id,scheduled_for)` and `(tenant_id,policy_id,local_period_key)`. |
| `audit_maintenance_snapshots` | Tenant/policy/input row/snapshot UUID, observation/evidence/verification/decision versions, normalized claim values/hash, expiry, evaluation time. Append-only, source retention bounded. |
| `audit_change_proposals` | Tenant/change UUID PK, occurrence/input/candidate refs, claim kind, before/after snapshot refs, fingerprint, state/version, reason, reviewer/time. Unique `(tenant_id,policy_id,fingerprint)`. |
| `audit_maintenance_events` | Tenant/event UUID PK, policy/occurrence/change refs, event type/deduplication key, in-app receipt state/time. Unique `(tenant_id,deduplication_key)`. |

Policies refer to an explicit approved cohort version, not a saved filter whose
membership can silently expand. Enrolling new rows or changing target roles,
providers, purposes, price version or ceilings requires a versioned policy review
and new spend authorization. Removing rows or lowering limits can take effect
immediately but cannot revoke already purchased work. A row's later suppression or
deletion removes it from eligible work regardless of the frozen cohort membership.

Indexes: active policies `(next_run_at,id) WHERE paused_at IS NULL AND deleted_at
IS NULL`; occurrences `(tenant_id,policy_id,scheduled_for DESC)`; pending proposals
`(tenant_id,policy_id,state,created_at,id)`; stale snapshots by expiry. Add a partial
unique index allowing at most one active occurrence per policy in `queued`,
`assessing`, `awaiting_approval`, `running`, or `reconciling`. Include foreign-key
indexes and tenant checks on all scheduling and review reads.

### Explicit recurring authorization

```http
POST /v1/audits/{audit_id}/maintenance-policies
Idempotency-Key: <UUID>
```

```json
{
  "expected_version": "12",
  "cohort_input_row_ids": ["40000000-0000-4000-8000-000000000001"],
  "schedule": {"cadence": "weekly", "weekday": "monday", "local_time": "09:00", "timezone": "America/New_York"},
  "freshness_days": {"mailbox": 7, "employment": 30, "role_relevance": 30},
  "approval_mode": "review_each_run",
  "limits": {"currency": "USD", "per_run_charge_micros": "2000000", "monthly_charge_micros": "10000000"}
}
```

Creation requires manage scope and returns 201 with policy version, cohort hash,
next due time and state. `review_each_run` is the default: scheduling may assess
cached evidence and create a repair plan, but cannot make a paid call. Zero-price
assessments still obey source rights, provider limits and approved run scope.

`POST /v1/audits/{id}/maintenance-policies/{policy_id}/authorize` requires
manage+spend and policy version/hash, exact ceilings, provider/price versions and
`authorization_expires_at`. Default maximum authorization lifetime is 90 days;
renewal is an explicit action. This changes the mode to `bounded_recurring`.
Approvals are durable policy authority, not borrowed indefinitely from an API key
that might later be revoked. Disabling the policy, revoking that authority, losing
tenant entitlement or deleting the authorizing account pauses future admission.
Every admission rechecks the recorded authority and current tenant state.
Every generated recurring plan must also carry a finite nonnegative
`max_provider_cost_micros`, derived from the approved versioned tariffs and bounded
attempt counts before dispatch. Missing/unknown provider cost blocks admission even
when the customer charge is zero. RFC 0005's Go admission carries this internal
provider-cost ceiling alongside delegated customer-charge caps; it is bound to the
plan hash and cannot be increased by the executor or an automatic policy run.

Recurring authorization reserves nothing until a run is due. Each run then uses
RFC 0005's tenant UTC-month account **and** a policy UTC-month sublimit, locked in
that order, and reserves the same maximum charge against both in one transaction.
Add `audit_maintenance_budget_periods` keyed by tenant/policy/UTC month with
limit/reserved/settled counters and an FK from its reservation. This is a sublimit,
not extra purchasing power. Settlement/refund/release updates both counters in the
same Rust transaction. Unresolved costs count against both until reconciled.

The maximum admitted charge is the lesser of the per-run limit, policy remaining
monthly capacity and tenant remaining capacity. If the complete proposed plan does
not fit, emit `budget_exhausted` and require a smaller previewed plan; do not silently
change its scope. Crossing a calendar month keeps prior reservations in their
original month. Remaining unknown spend is shown in subsequent policy status.
There is no new subscription charge or Stripe entitlement implied by authorization;
recurring commercial billing remains a separately validated product decision.

### Scheduler transaction and time semantics

Support daily/weekly local-time schedules in v1; reject arbitrary cron expressions.
Validate IANA timezone names. Store the timezone, local schedule, timezone database
version used for calculation, next UTC instant and a local period key. On a DST
fold, run only at the earlier matching instant; in a spring-forward gap, run at the
first valid local instant after the requested time. The local period key prevents
double execution if a timezone database update changes the computed UTC instant.
Changing timezone/schedule is a versioned edit with an explicit next-run preview.

The existing scheduler advances `next_run_at` when claiming schedules. For audit
maintenance, do not advance a due time without persisting its occurrence and dispatch:

```text
BEGIN
  lock up to 25 due policy rows FOR UPDATE SKIP LOCKED;
  for each policy:
    verify current policy version, pause/deletion and authorization;
    insert unique occurrence(policy, scheduled UTC instant, local period key);
    insert audit operation and dispatch intent, or a durable skipped reason;
    compute and store next_run_at strictly after now;
COMMIT
```

After downtime, coalesce missed intervals into at most one current assessment and
record the skipped interval range; do not buy one run per missed day. If a previous
occurrence remains active, record `skipped_overlap` for the new due slot and advance
the schedule atomically. A restart after commit sees the same occurrence and outbox;
a restart before commit sees no advanced due time. Use the shared leased workers
for execution; scheduler ticks themselves make no provider calls.

Occurrence states are `queued → assessing → awaiting_approval → running →
reconciling → completed`, with `skipped`, `blocked`, `cancelled` and `failed` as
explicit alternatives. `bounded_recurring` may move directly from assessing to
running after reservation. Unknown paid outcomes stay reconciling; no new overlapping
occurrence starts until they settle or an operator records a permitted resolution.
Unapproved per-run plans expire after their quoted expiry and close as skipped,
allowing a later occurrence to assess again without preserving stale approval.

### Assessment, change detection, and review

Read the frozen cohort in pages of 200. For each row, check suppression/deletion
first, then compare claim expiry and the latest verification operation. Select only
claims whose freshness window is exceeded or that have a new conflicting observation.
A cached fresh claim requires no provider call. Any network refresh that could cost
money must be in an approved RFC 0005 plan, including mailbox-only refreshes.

Compare normalized claim values, not crawl timestamps, ordering or HTML formatting.
Compute a tenant-scoped HMAC fingerprint over policy, input row, claim kind, prior
accepted snapshot ID and new normalized value hash; store the fingerprint algorithm
version. Repeated equivalent observations attach evidence to the same open proposal.
A later return to a prior role after an accepted intervening change is a new event
because the prior accepted snapshot differs.

| Observation | Change classification and action |
|---|---|
| Mailbox becomes definitively invalid | `mailbox_invalid`; invalidate usable status; no claim of job departure. |
| Timeout, missing page or undated new page | `evidence_unknown`; review when freshness expires; no confirmed role change. |
| New source contradicts current employer/role | `employment_conflict` / `role_conflict`; invalidate affected approval and request review. |
| Fresh explicit departure/new-role evidence, independently corroborated | `employment_change_supported` / `role_change_supported`; propose a dated change, still require review. |
| Same supported values, newer valid evidence | Refresh snapshot; no actionable-change notification. |
| Replacement person found | New candidate UUID and claim chain; never overwrite the old person's identity. |

For the initial supported-change classifier, require two distinct source origins
with explicit dated assertions within 30 days, one being first-party company/person
evidence. Syndicated copies count as one origin. Store source-independence reasoning;
uncertain independence stays conflicting/unknown. This is a configurable hypothesis
to test against independently adjudicated changes, not a guaranteed precision claim.
A reviewer can resolve the proposal using the same claim rules as RFC 0004, but
cannot bypass a missing identity/role or mailbox gate.

`GET /v1/audits/{id}/maintenance-policies/{policy_id}/runs` lists occurrences/costs.
`GET /v1/audits/{id}/changes` lists proposals with before/after snapshots and reasons.
`POST /v1/audits/{id}/changes/{change_id}/decisions` requires review scope,
expected version, action (`accept`, `hold`, `reject`) and reason. Accept appends the
new snapshot/decision and invalidates prior approvals; it does not automatically
mark the contact Ready or apply CRM changes. Ready approval and CRM preview/apply
remain separate actions using current policy and verification.

`POST .../maintenance-policies/{policy_id}/pause` and versioned `PATCH` require
manage scope. Pause stops future occurrence admission and sends cancellation fences
to active runs; already dispatched costs reconcile. Resume revalidates authorization
and calculates one next run, without catching up missed paid intervals.

### Notifications, rollout, and tests

Start with durable in-app events: one per actionable change fingerprint, one budget
block per policy/accounting period, and one recovery event when a previously blocked
policy succeeds. Insert the event in the same transaction as the proposal/state
transition. Repeated ticks or worker retries find the deduplication key and do not
alert again. Unchanged successful checks create run history, not alerts. Email or
customer webhooks require their separate delivery contract; they are not hidden
side effects of this RFC.

Roll out manual snapshot/diff first, then scheduled assessment without paid calls,
then mailbox-only bounded authorization, then selected identity/role refresh.
Rollback pauses admission and retains occurrence/settlement readers. Deletion
removes enrolled personal data and suppresses future notifications while allowing
non-PII financial receipts to settle under RFC 0001 retention rules.

Test with a fake clock around both DST transitions, timezone changes, month rollover,
authorization expiry and multiple missed intervals. Database fault injection must
kill the scheduler before/after occurrence/outbox commit and prove one run per local
period. Race manual repair with two recurring policies against one tenant balance;
assert exact reservation/settlement totals. Provider stubs must cover unknown charges,
repeated unchanged observations, syndicated evidence, disappearance, corroborated
change, suppressed rows and a new candidate replacing an old role. No fixture should
label a changed job from mailbox invalidation alone.

At a 400-row cohort, verify bounded paging/memory and fairness with a concurrent
small audit using RFC 0005's capacity gates. Publish metrics for due-to-admitted lag,
missed/overlap slots, stale eligible cohort fraction, proposal duplication, unresolved
cost age and policy budget blocks. Accuracy reporting must include unknown/conflict
rates and adjudication sample sizes; notification volume is not change precision.
