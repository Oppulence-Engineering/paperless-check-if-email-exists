# RFC 0009: Tenant-Facing Job Observability

## Status

Draft

## Summary

Add a unified observability layer for bulk jobs so tenants can inspect lifecycle events, retries, failure reasons, latency, and downloadable failure reports through stable APIs. This feature builds on the current progress, approval, and latency endpoints rather than replacing them.

## Motivation / Problem

- Job status, latency, and approval are currently spread across separate endpoints.
- Customers need retry and failure visibility without asking support or reading raw logs.
- The platform already tracks enough state to surface richer operational insight, but it is not exposed coherently.

## Goals

- Provide a single job observability summary endpoint.
- Expose lifecycle events and retry/failure details.
- Offer downloadable failure reports for downstream remediation.
- Keep compatibility with current job progress and latency APIs.

## Non-Goals

- Full distributed tracing.
- Per-worker host diagnostics.
- Replacing the existing progress endpoint contract.

## Current State In Reacher

- Reacher already exposes job progress, approval, and latency endpoints.
- Worker processing persists task results and task state.
- There is no tenant-facing event log or unified retry/failure report surface.

## Proposed Design

Add four observability surfaces:

- `GET /v1/jobs/{job_id}/observability`
- `GET /v1/jobs/{job_id}/events`
- `GET /v1/jobs/{job_id}/failures`
- `GET /v1/jobs/{job_id}/failures/download`

`/observability` returns:

- job status
- created/started/completed timestamps
- task state counts
- retry counts
- dead-letter counts
- processing throughput
- queue age
- estimated completion ETA when still running
- latency summary
- webhook delivery summary
- last failure summary

Event types are normalized as:

- `job_created`
- `task_published`
- `task_started`
- `task_completed`
- `task_failed`
- `task_retried`
- `task_dead_lettered`
- `webhook_sent`
- `webhook_failed`
- `job_cancelled`
- `job_completed`

Failure downloads support:

- `format=csv`
- `format=ndjson`

### Read Model Structure

The observability feature is intentionally read-model-first. It does not expose raw internal queue objects. Instead it presents a normalized tenant-safe job view composed from:

- job row metadata
- task attempt metadata
- worker lifecycle events
- queue delivery summaries
- webhook delivery summaries where applicable

The API is split so consumers can choose cost and detail:

- `/observability`
  - compact dashboard-style summary
- `/events`
  - coarse ordered lifecycle log
- `/failures`
  - task-level failure records
- `/failures/download`
  - machine-readable export for remediation or support workflows

### Summary Semantics

`/observability` returns four blocks:

- `status`
  - current lifecycle state and timestamps
- `throughput`
  - processed counts, rates, queue age, and ETA
- `retries`
  - attempts, retryable failures, dead-letter counts
- `latency`
  - queue wait and processing duration summary

Example summary response:

```json
{
  "job_id": "job_01HT...",
  "status": {
    "state": "running",
    "created_at": "2026-03-30T18:50:00Z",
    "started_at": "2026-03-30T18:50:02Z",
    "completed_at": null
  },
  "counts": {
    "total_tasks": 1000,
    "queued": 120,
    "processing": 24,
    "completed": 810,
    "failed": 31,
    "cancelled": 15
  },
  "retries": {
    "attempted_retries": 48,
    "retryable_failures": 11,
    "dead_lettered": 4
  },
  "throughput": {
    "tasks_per_minute": 420.5,
    "queue_age_seconds_p95": 19,
    "eta_seconds": 37
  },
  "latency": {
    "queue_wait_ms_p50": 420,
    "queue_wait_ms_p95": 1800,
    "processing_ms_p50": 650,
    "processing_ms_p95": 1900
  },
  "last_failure": {
    "task_id": "tsk_01HT...",
    "code": "smtp_timeout",
    "occurred_at": "2026-03-30T18:51:30Z"
  }
}
```

### Event Model

`/events` is a coarse event log, not a trace. It captures material workflow transitions only. The event model is intentionally normalized so tenants are not exposed to backend-specific queue implementation details.

Event payload shape:

- `id`
- `job_id`
- `event_type`
- `occurred_at`
- `task_id` nullable
- `attempt_number` nullable
- `payload` object

Representative payload keys:

- queue name
- retry count
- failure code
- webhook target name
- latency summary block

### Failure Record Semantics

`/failures` is row-oriented and designed to answer "which rows failed, why, and can I retry them?"

Each failure row includes:

- task identifiers
- original input identifier where available
- latest failure code and message
- retry count
- retryable boolean
- last attempt timestamps

The endpoint returns only rows whose latest known state is failure-like. Successful retries do not remain in `/failures`.

### Download Contract

Failure downloads are intended for operational export, not archival completeness. They include:

- original input
- canonical email when available
- last failure code
- last failure message
- retry count
- retryable
- last attempt at

CSV is optimized for spreadsheet workflows. NDJSON is optimized for machine consumers and support tooling.

## Public API / Interface Changes

Keep current endpoints:

- `GET /v1/bulk/{job_id}`
- `GET /v1/jobs/{job_id}/approval`
- `GET /v1/jobs/{job_id}/latency`

Add:

- `GET /v1/jobs/{job_id}/observability`
- `GET /v1/jobs/{job_id}/events`
- `GET /v1/jobs/{job_id}/failures`
- `GET /v1/jobs/{job_id}/failures/download`

`GET /v1/jobs/{job_id}/events` supports:

- `cursor`
- `limit`
- `event_type`

`GET /v1/jobs/{job_id}/failures` supports:

- `cursor`
- `limit`
- `task_state`
- `retryable_only`

Additional query parameters:

- `/events`
  - `from`
  - `to`
- `/observability`
  - no pagination; this is a single summary resource
- `/failures/download`
  - `retryable_only`
  - `task_state`

Compatibility additions to existing job reads:

- `GET /v1/bulk/{job_id}` may later include a compact `observability_summary`, but that is explicitly deferred in this RFC to avoid bloating the current progress surface

`task_state` accepted values for failures:

- `failed`
- `dead_lettered`
- `cancelled`

Example failure row:

```json
{
  "task_id": "tsk_01HT...",
  "row_ref": "line_182",
  "email": "lead@example.com",
  "canonical_email": "lead@example.com",
  "task_state": "failed",
  "retryable": true,
  "retry_count": 2,
  "last_failure": {
    "code": "smtp_timeout",
    "message": "verification attempt exceeded timeout",
    "occurred_at": "2026-03-30T18:52:10Z"
  }
}
```

## Data Model / Storage Changes

Add tables:

- `v1_job_events`
- `v1_task_attempts`

Storage decisions:

- `v1_job_events` stores tenant-visible coarse lifecycle events.
- `v1_task_attempts` stores per-attempt timing and failure metadata for retries.
- Both tables inherit the tenant’s result retention window by default.

`v1_job_events` columns:

- `id`
- `tenant_id`
- `job_id`
- `task_id` nullable
- `event_type`
- `payload JSONB`
- `created_at`

Indexes:

- btree `(tenant_id, job_id, created_at desc, id desc)`
- btree `(tenant_id, job_id, event_type, created_at desc)`

`v1_task_attempts` columns:

- `id`
- `tenant_id`
- `job_id`
- `task_id`
- `attempt_number`
- `state`
- `queued_at`
- `started_at`
- `completed_at`
- `failure_code`
- `failure_message`
- `retryable`
- `worker_id` nullable
- `metadata JSONB`

Indexes:

- unique `(task_id, attempt_number)`
- btree `(tenant_id, job_id, state, completed_at desc)`
- btree `(tenant_id, job_id, retryable, completed_at desc)`

Emission rules:

- every published task creates at least one event row
- every attempt creates one `v1_task_attempts` row
- retry transitions append new attempt rows rather than mutating the old one
- event and attempt writes are best-effort but must be wrapped so failures do not abort the underlying verification workflow

## Auth, Permissions, And Tenant Isolation

- All observability endpoints require the existing `bulk` scope.
- Observability records are only queryable by the owning tenant.
- Admin routes may reuse the same read model in a later follow-on, but that is not part of this RFC.

Additional rules:

- webhook delivery summaries exposed via observability include only tenant-owned webhook targets
- failure downloads must be authorized with the same tenant context as the base job
- pagination cursors are opaque and scoped to the tenant plus job ID so they cannot be replayed across jobs

## Failure Modes And Edge Cases

- A job with no attempts yet still returns `/observability` with zero counts and null latency summary.
- Failure downloads for jobs with no failures return an empty file with a header row.
- If attempt logging fails, the underlying verification work continues and `/observability` degrades gracefully to the data already in `v1_task_result`.
- Event pagination is stable by `(created_at, id)` ordering.

Additional cases:

- if a job is cancelled mid-flight, queued tasks may never create attempt rows; summary counts must still reconcile against task states
- ETA is omitted rather than fabricated when insufficient throughput history exists
- if the failure message exceeds storage limits, a truncated message plus full failure code is stored
- if observability materialization lags briefly behind job execution, `/observability` returns the best-known state and a `generated_at` timestamp so callers know how fresh the view is

## Alternatives Considered

- Extending only `/v1/bulk/{job_id}`. Rejected because it would overload a progress endpoint with too many operational concerns.
- Storing observability only in logs. Rejected because tenants need API access to the data.
- Exposing raw queue internals. Rejected because it leaks implementation detail and is unstable.

## Rollout / Migration / Compatibility

- Add the new endpoints and tables additively.
- Keep existing progress, approval, and latency routes unchanged.
- Backfill is not required; historical jobs simply have partial observability until the feature is enabled.

Rollout plan:

1. start emitting attempt rows and lifecycle events for new jobs
2. ship `/observability`
3. ship `/events` and `/failures`
4. ship failure downloads
5. update docs and SDKs

Compatibility decisions:

- older jobs created before rollout remain readable, but their event history may be incomplete
- the feature is read-only; it does not alter job execution semantics

## Metrics And Success Criteria

- Usage of `/observability` and failure download endpoints.
- Reduction in support tickets about “stuck” or “failed” jobs.
- Coverage rate of jobs with attempt and event records.
- p95 latency of observability reads.

Additional success criteria:

- operators can identify whether a job is blocked on queue backlog, retry churn, or deterministic data failures without inspecting raw infrastructure logs
- failure downloads are sufficient to drive customer remediation workflows and support escalation without custom SQL

## Test Strategy

- Job lifecycle event emission tests.
- Retry and dead-letter attempt logging tests.
- Failure download format tests.
- Authorization and tenant isolation tests.
- Compatibility tests proving existing progress/approval/latency routes still work unchanged.

Add explicit cases for:

- job with zero attempts yet
- retried task that later succeeds disappearing from `/failures`
- stable event pagination with same-timestamp rows
- cancellation reconciliation between task states and summary counts
- large failure downloads streaming correctly in both CSV and NDJSON

## Unresolved Risks

- Event and attempt storage can grow quickly for large tenants.
- ETA accuracy may be poor for very small or highly bursty jobs.

Storage growth is the main operational risk. The read model is only worthwhile if retention and summarization stay bounded enough to avoid turning observability into a cost center.
