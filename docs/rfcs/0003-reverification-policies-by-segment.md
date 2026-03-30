# RFC 0003: Reverification Policies by Segment

## Status

Draft

## Summary

Add tenant-scoped reverification policies that target explicit contact segments and run on schedules using the existing worker and pipeline infrastructure. Policies define who to reverify, how stale a result must be before it qualifies, how often the segment runs, and whether a manual trigger is allowed.

## Motivation / Problem

- Reverification exists today, but not as a tenant-managed policy system with reusable segment rules.
- Customers need recurring “recheck this slice of contacts” behavior without building their own schedulers.
- Existing pipelines solve scheduling, but they do not model reverification freshness or segment selection as first-class policy objects.

## Goals

- Create reusable reverification policies per tenant.
- Support list-based, pipeline-based, query-based, and global segments.
- Reuse the current pipeline scheduler and job execution patterns.
- Keep runs auditable and non-destructive.

## Non-Goals

- Building a new automation engine parallel to pipelines.
- Mutating historical task results in place.
- A drag-and-drop UI for segment construction.

## Current State In Reacher

- Reacher already supports scheduled re-verification and pipelines.
- Bulk jobs, lists, email history, and `/v1/query` already expose the contact/result corpus that reverification policies need.
- There is no tenant-visible policy resource that captures segment + schedule + freshness rules together.

## Proposed Design

Add a `reverification policy` resource with these fields:

- `name`
- `enabled`
- `segment_type`
- `segment_filter`
- `freshness_days`
- `schedule_cron`
- `max_records_per_run`
- `policy_mode_override`
- `is_default`

### Policy States

Each policy has an operational state independent of whether it is logically enabled:

- `enabled`
- `paused`
- `invalid_reference`
- `disabled`

Meaning:

- `enabled`: eligible for scheduler pickup
- `paused`: preserved but not scheduled
- `invalid_reference`: blocked because the referenced list or pipeline no longer exists
- `disabled`: intentionally turned off and excluded from scheduling

Supported `segment_type` values are:

- `all_contacts`
- `list`
- `pipeline`
- `query_filter`

### Query Filter Shape

`query_filter` reuses the `/v1/query` selection vocabulary, but the policy stores the filter as a durable JSON object instead of replaying raw query strings. The stored shape in v1 supports:

- `category`
- `safe_to_send`
- `verified_before`
- `verified_after`
- `list_id`
- `job_id`
- `limit`

Unsupported live-query parameters are rejected at policy creation time.

`segment_filter` rules are:

- `list`: requires `list_id`
- `pipeline`: requires `pipeline_id`
- `query_filter`: reuses the `/v1/query` filter shape
- `all_contacts`: empty object

Execution behavior is fixed:

- A policy run selects contacts whose latest result is older than `freshness_days`.
- Every run creates a new reverification job. It does not overwrite older task results.
- If the policy is attached to a pipeline segment, the run is recorded as both a reverification policy run and a pipeline run reference.
- `is_default = true` means the policy applies when a pipeline or manual run requests “use tenant default”.

### Run Selection Algorithm

Each scheduled or manual run executes the same selection algorithm:

1. Resolve the segment to candidate records.
2. Collapse candidates by canonical email, keeping the newest result per email.
3. Exclude contacts with active suppressions when the policy requests `exclude_suppressed = true`.
4. Exclude rows newer than `freshness_days`.
5. Apply `max_records_per_run`.
6. Materialize a new reverification job from the remaining candidates.

`max_records_per_run` defaults to `10_000`. Requests above `100_000` return `400`.

Precedence rules are:

1. Manual run override in `POST /v1/reverification/policies/{policy_id}/run`
2. Pipeline-attached policy
3. Explicit segment policy
4. Tenant default policy
5. No policy

Only one tenant default may exist at a time. Creating or updating a policy with `is_default = true` clears the default flag from the previous tenant default in the same transaction.

## Public API / Interface Changes

Add policy endpoints:

- `GET /v1/reverification/policies`
- `POST /v1/reverification/policies`
- `GET /v1/reverification/policies/{policy_id}`
- `PATCH /v1/reverification/policies/{policy_id}`
- `DELETE /v1/reverification/policies/{policy_id}`
- `POST /v1/reverification/policies/{policy_id}/run`
- `GET /v1/reverification/policies/{policy_id}/runs`

Create policy request example:

```json
{
  "name": "recheck-safe-leads-every-30d",
  "enabled": true,
  "segment_type": "query_filter",
  "segment_filter": {
    "category": "valid",
    "safe_to_send": true
  },
  "freshness_days": 30,
  "schedule_cron": "0 2 * * *",
  "max_records_per_run": 10000,
  "policy_mode_override": "balanced",
  "is_default": false
}
```

`POST /v1/reverification/policies` request fields:

- `name`
- `enabled`
- `segment_type`
- `segment_filter`
- `freshness_days`
- `schedule_cron`
- `max_records_per_run`
- `policy_mode_override`
- `is_default`

Run responses include:

- `run_id`
- `policy_id`
- `job_id`
- `selected_count`
- `started_at`
- `status`

Policy status responses should also include:

- `last_run_at`
- `next_run_at`
- `last_run_status`
- `resolved_reference_status`

## Data Model / Storage Changes

Add tables:

- `v1_reverification_policies`
- `v1_reverification_policy_runs`

`v1_reverification_policies` columns:

- `id`
- `tenant_id`
- `name`
- `status`
- `segment_type`
- `segment_filter`
- `freshness_days`
- `schedule_cron`
- `max_records_per_run`
- `policy_mode_override`
- `is_default`
- `created_at`
- `updated_at`

`v1_reverification_policy_runs` columns:

- `id`
- `policy_id`
- `tenant_id`
- `job_id`
- `trigger_type`
- `selected_count`
- `status`
- `started_at`
- `completed_at`
- `error`

Storage decisions:

- `segment_filter` is stored as `JSONB`.
- `schedule_cron` is stored as text and validated on write.
- `policy_mode_override` integrates with RFC 0007 and is nullable until that feature ships.
- indexes required in v1:
  - `(tenant_id, is_default)` partial index where `is_default = true`
  - `(tenant_id, status, next_run_at)`
  - `(policy_id, started_at DESC)` on runs

## Auth, Permissions, And Tenant Isolation

- Read endpoints require `pipelines.read`.
- Create/update/delete requires `pipelines.write`.
- Manual runs require `pipelines.trigger`.
- A policy may only select contacts from resources owned by the same tenant.

## Failure Modes And Edge Cases

- Invalid or unsupported cron expressions return `400`.
- A policy that resolves zero contacts records a successful no-op run.
- A policy with a deleted `list_id` or `pipeline_id` becomes `invalid_reference` and does not schedule until fixed.
- Concurrent manual and scheduled runs for the same policy are serialized per tenant and policy.

Additional edge rules:

- If a manual run is requested while a prior run is still `running`, return `409`.
- If a policy resolves more candidates than `max_records_per_run`, excess candidates are deferred to the next run and reported as `deferred_count`.
- Policies referencing query filters with deleted fields after schema evolution should transition to `invalid_reference` and require explicit update.

## Alternatives Considered

- Encoding reverification rules only in pipelines. Rejected because segment freshness and policy defaults would stay implicit.
- Global tenant reverification settings only. Rejected because customers need multiple segment-specific cadences.
- In-place result mutation. Rejected because it breaks history and job observability.

## Rollout / Migration / Compatibility

- Additively introduce policy APIs and tables.
- Keep existing reverification behavior working unchanged for tenants that do not create policies.
- Default schedule timezone is UTC. No per-policy timezone support in v1.

Implementation phases:

1. CRUD and storage
2. run materialization API
3. scheduler integration
4. default-policy and pipeline-reference wiring

## Metrics And Success Criteria

- Number of active reverification policies per tenant.
- Scheduled run success rate.
- Percentage of selected contacts that were truly stale at run time.
- Reduction in stale-result usage across pipelines and exports.

## Test Strategy

- Policy CRUD tests.
- Segment resolution tests for all four segment types.
- Precedence tests across default, segment, and manual overrides.
- Scheduler tests for duplicate-run prevention.
- Tenant isolation tests across shared numeric IDs and query filters.

Also require:

- canonical-email collapse tests
- invalid-reference transition tests
- max-records truncation tests
- policy-default uniqueness tests

## Unresolved Risks

- Query-based segments can become expensive if tenants define overly broad filters.
- Policy sprawl can create duplicate reverification work without later guardrails or reporting.
