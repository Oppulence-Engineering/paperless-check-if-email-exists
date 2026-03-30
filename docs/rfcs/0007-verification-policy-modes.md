# RFC 0007: Verification Policy Modes

## Status

Draft

## Summary

Add explicit verification policy modes that sit above the current scoring model. The modes are `strict`, `balanced`, and `aggressive`, and they produce a new `policy_evaluation` object without changing the underlying `score` or `safe_to_send` semantics.

## Motivation / Problem

- Customers want different send thresholds for different workflows, but today they must build those rules outside Reacher.
- `safe_to_send` is intentionally conservative and global; it cannot represent different risk appetites.
- Pipelines, reverification policies, and bulk exports need a shared policy layer instead of per-feature heuristics.

## Goals

- Define three stable policy modes with explicit behavior.
- Keep `safe_to_send` unchanged for backward compatibility.
- Support tenant defaults plus request-level and workflow-level overrides.
- Return a policy verdict alongside the existing score.

## Non-Goals

- Changing how the current score or category is computed.
- Introducing arbitrary user-defined policy expressions in v1.
- Enforcing send actions outside Reacher.

## Current State In Reacher

- Reacher already computes `score`, `category`, `sub_reason`, `safe_to_send`, and bounce-risk assessments.
- Approval checklist thresholds exist at the job level.
- There is no reusable policy mode surface for single checks, bulk jobs, lists, or pipelines.

## Proposed Design

Add three policy modes:

- `strict`
- `balanced`
- `aggressive`

Evaluation rules are:

- `strict`
  - `send` only when `safe_to_send = true`
  - bounce-risk category must be `safe` or `low`
  - result age must be `<= 14` days
  - address must not be actively suppressed
- `balanced`
  - `send` when `safe_to_send = true`
  - bounce-risk category must not be `high` or `dangerous`
  - result age must be `<= 30` days
  - address must not be actively suppressed
- `aggressive`
  - `send` when score category is `valid` or `risky`
  - address must not be disposable or on a spam-trap domain
  - bounce-risk category must not be `dangerous`
  - result age must be `<= 45` days
  - address must not be actively suppressed

If bounce-risk data is unavailable:

- `strict` -> `review`
- `balanced` -> `review`
- `aggressive` -> continue with score-only evaluation

Override hierarchy is:

1. Per-request `policy_mode`
2. Workflow-level override on a list, bulk job, or pipeline
3. Tenant default
4. System default `balanced`

### Decision Model

Policy evaluation does not replace verification. It is a post-verification decision layer that takes an existing result and converts it into a workflow recommendation. The evaluation inputs are:

- verification category and score
- `safe_to_send`
- bounce-risk category when available
- suppression state
- result age
- selected `policy_mode`

The evaluation output is:

- `mode`
- `decision`
- `reasons[]`
- `evaluated_at`
- `result_age_days`

`decision` semantics:

- `send`
  - the row is acceptable for the current policy
- `review`
  - the row is not automatically blocked, but the evidence is incomplete or borderline
- `suppress`
  - the row should be added to or kept in suppression state if the caller chooses a suppression-aware flow
- `drop`
  - the row should be excluded from downstream send/export flows immediately

The distinction between `suppress` and `drop` matters:

- `suppress`
  - durable operational action
- `drop`
  - workflow-local exclusion that does not itself mutate suppression state

### Mode Matrices

Additional rules for `strict`:

- `catch_all` results become `review`
- disposable domains become `drop`
- role accounts become `review`
- unknown SMTP outcomes become `review`

Additional rules for `balanced`:

- `catch_all` results become `review`
- disposable domains become `suppress` only if tenant settings already enable suppression for disposable
- role accounts may still be `send` when `safe_to_send=true` and bounce risk is not high

Additional rules for `aggressive`:

- `catch_all` may remain `send` if bounce risk is not dangerous
- role accounts remain `review` rather than `drop`
- unknown SMTP outcomes become `review`

### Reason Catalog

`reasons[]` is structured rather than free text. Codes in v1:

- `safe_to_send_false`
- `bounce_risk_missing`
- `bounce_risk_too_high`
- `result_stale`
- `actively_suppressed`
- `disposable_domain`
- `spam_trap_domain`
- `role_account`
- `catch_all_domain`
- `unknown_smtp`

Each reason may optionally include:

- `severity`
- `detail`
- `evidence`

Example:

```json
{
  "mode": "strict",
  "decision": "review",
  "evaluated_at": "2026-03-30T18:20:00Z",
  "result_age_days": 21,
  "reasons": [
    {
      "code": "result_stale",
      "severity": "warning",
      "detail": "Strict mode requires verification freshness <= 14 days",
      "evidence": {
        "max_allowed_days": 14,
        "actual_days": 21
      }
    }
  ]
}
```

### Workflow Integration

The policy layer is consumed by:

- single-check responses
- bulk job row outputs
- list quality and remediation follow-ons
- reverification policies
- pipeline execution decisions

Rules by surface:

- single check
  - returns `policy_evaluation` inline only
- bulk/list processing
  - persists `policy_evaluation` per task result
- pipeline trigger
  - uses the effective policy mode to decide send/review/drop partitioning

This RFC does not add automatic outbound sending or suppression mutations by itself. It only standardizes the decision object so later flows can act on it consistently.

## Public API / Interface Changes

Add `default_policy_mode` to `PATCH /v1/me/settings`.

Add optional `policy_mode` to:

- `POST /v1/check_email`
- `POST /v1/bulk`
- `POST /v1/lists`
- `POST /v1/pipelines`
- `PATCH /v1/pipelines/{pipeline_id}`

Add `policy_evaluation` to result-bearing responses:

- `mode`
- `decision`
- `reasons`
- `evaluated_at`

`decision` values are:

- `send`
- `review`
- `suppress`
- `drop`

Example single-check request:

```json
{
  "to_email": "lead@example.com",
  "policy_mode": "strict",
  "sandbox": true
}
```

Example result fragment:

```json
{
  "email": "lead@example.com",
  "score": 83,
  "safe_to_send": true,
  "policy_evaluation": {
    "mode": "strict",
    "decision": "review",
    "evaluated_at": "2026-03-30T18:20:00Z",
    "result_age_days": 21,
    "reasons": [
      {
        "code": "result_stale",
        "severity": "warning"
      }
    ]
  }
}
```

Workflow-bearing create surfaces return the effective mode used:

- `effective_policy_mode`

This makes inheritance visible even when the caller did not pass an override explicitly.

## Data Model / Storage Changes

- Add `default_policy_mode` to `tenants`.
- Add nullable `policy_mode` to bulk jobs, lists, and pipelines.
- Add `policy_evaluation JSONB` to `v1_task_result` so historical reads preserve the original verdict.

Additional fields:

- `policy_evaluated_at TIMESTAMPTZ` on `v1_task_result`
- optional `policy_mode` on reverification policies introduced by RFC 0003

Storage rules:

- persisted `policy_evaluation` represents the decision at processing time
- later changes to tenant defaults do not rewrite historical rows
- reevaluation only occurs if a workflow explicitly reruns verification or a future dedicated reevaluate operation is added

Indexes:

- btree `(tenant_id, policy_mode)` on workflow tables where useful
- GIN or JSON path indexing is not required initially for `policy_evaluation`; reads mostly happen by row lookup or export

## Auth, Permissions, And Tenant Isolation

- Tenant default changes require the existing `settings` scope.
- Bulk and list overrides require existing `bulk` and `lists` scopes.
- Pipeline overrides require `pipelines.write`.
- Policy evaluation always uses same-tenant suppression state and result history only.

Additional rules:

- callers without write access to a workflow cannot override its policy mode
- pipeline executions triggered by background schedules use the persisted workflow mode or tenant default, never an external caller override
- policy evaluation must never consult cross-tenant suppression, reputation, or outcome evidence

## Failure Modes And Edge Cases

- Unsupported mode values return `400`.
- If a result is older than the mode threshold, policy evaluation returns `review` even if `safe_to_send = true`.
- A missing suppression table or lookup failure must fail closed for policy evaluation and return `review`.
- Historical rows do not get reinterpreted when tenant defaults change later.

Additional cases:

- if bounce-risk computation times out, strict and balanced degrade to `review`, aggressive continues with score-based rules and adds `bounce_risk_missing`
- if suppression lookup fails transiently in a bulk workflow, row-level evaluation returns `review` rather than aborting the entire job
- if a tenant later disables disposable suppression in settings, historical policy decisions still show the original evaluation they were computed with
- if a row is both actively suppressed and otherwise valid, `actively_suppressed` wins and the decision cannot be `send`

## Alternatives Considered

- Replacing `safe_to_send` with the new mode verdict. Rejected because it would break existing integrations.
- Letting every tenant define arbitrary rule expressions. Rejected for v1 because it creates support and testing complexity.
- Applying policy mode only at export time. Rejected because it needs to be visible on single-check and history surfaces too.

## Rollout / Migration / Compatibility

- Additively introduce `policy_mode` and `policy_evaluation`.
- Default all existing tenants to `balanced`.
- Keep all existing score fields and boolean `safe_to_send` untouched.

Rollout plan:

1. add tenant default setting and processing-side evaluator
2. surface `policy_evaluation` on single-check and bulk result reads
3. allow workflow-level overrides on lists and pipelines
4. document export and automation implications

Compatibility guarantees:

- clients depending only on `safe_to_send` keep working
- `policy_evaluation` is additive in all responses
- `balanced` is chosen specifically to remain closest to current platform behavior

## Metrics And Success Criteria

- Distribution of policy mode usage by endpoint.
- Rate of disagreement between `safe_to_send` and `policy_evaluation.decision`.
- Tenant adoption of non-default modes.
- Rate of stale-result policy blocks.

Additional success criteria:

- support requests asking how to treat `risky` or `catch_all` rows should decrease because the system now returns an explicit workflow verdict
- downstream automation can consume a single decision field instead of rebuilding the same logic externally

## Test Strategy

- Golden tests for all three modes.
- Bounce-risk-unavailable tests.
- Override precedence tests.
- Persistence tests for `policy_evaluation` in history and downloads.
- Suppression-interaction tests.

Add explicit cases for:

- catch-all behavior in each mode
- result age threshold boundaries at 14, 30, and 45 days
- tenant-default inheritance vs per-request override
- pipeline background execution using persisted mode
- historical result immutability after settings change

## Unresolved Risks

- Some customers may read policy mode verdicts as contractual deliverability guarantees.
- The aggressive mode can be overused if later UX does not explain the tradeoff clearly.

The largest communication risk is semantic overlap with `safe_to_send`. Docs and SDKs need to state clearly that `safe_to_send` is a core deliverability signal, while `policy_evaluation` is a workflow decision layer.
