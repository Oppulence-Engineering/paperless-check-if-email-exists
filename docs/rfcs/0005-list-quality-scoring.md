# RFC 0005: List Quality Scoring

## Status

Draft

## Summary

Formalize list-level quality scorecards on top of the existing `/v1/lists/{list_id}/quality` endpoint. The new scorecard remains deterministic, uses existing verification signals, and adds cached summaries so list surfaces can expose quality without recomputing heavy aggregates on every read.

## Motivation / Problem

- Reacher already exposes list quality metrics, but the current output is a lightweight benchmark rather than a stable scorecard contract.
- Customers need a consistent quality score and grade for comparison across lists, time, and downstream workflows.
- List overview screens and analytics need cached summaries, not repeated full scans of task results.

## Goals

- Define a stable `ListQualityScorecardV1`.
- Preserve the existing `/quality` route and extend it additively.
- Cache list-level summaries once processing is complete.
- Reuse the existing `score`, `safe_to_send`, risk flags, and freshness data.

## Non-Goals

- Changing single-email scoring semantics.
- Adding custom tenant-configurable quality formulas in v1.
- Requiring a new dashboard surface to use the scorecard.

## Current State In Reacher

- `/v1/lists/{list_id}/quality` already returns aggregate score, category distribution, safe-to-send ratio, and a grade.
- The response is computed live from `v1_task_result`.
- List detail and list list endpoints do not expose a durable cached quality summary.

## Proposed Design

Define `ListQualityScorecardV1` with these outputs:

- `quality_score`
- `quality_grade`
- `scorecard_version`
- `component_scores`
- `risk_breakdown`
- `freshness`
- `completeness_pct`
- `is_final`

Scoring formula:

- Compute a base row score across processed rows:
  - `valid = 100`
  - `risky = 60`
  - `unknown = 25`
  - `invalid = 0`
- `base_score` is the processed-row average of those row scores.
- Apply penalty points:
  - `0.20 * catch_all_pct`
  - `0.25 * disposable_pct`
  - `0.15 * role_account_pct`
  - `0.50 * full_inbox_pct`
  - `1.00 * spam_trap_pct`
  - `0.20 * stale_over_30d_pct`
- Cap total penalties at `30`.
- Final `quality_score = clamp(round(base_score - penalties), 0, 100)`.

Grade thresholds:

- `A`: `90-100`
- `B`: `80-89`
- `C`: `65-79`
- `D`: `50-64`
- `F`: `0-49`

If a list is not fully processed:

- `is_final = false`
- `completeness_pct = processed / total_rows * 100`
- `quality_score` still returns, but callers must treat it as provisional

### Scorecard Structure

`ListQualityScorecardV1` is intentionally split into three layers:

- headline summary
  - stable values used by list detail, list index, and downstream automations
- component scores
  - the explainable weighted parts of the final score
- evidence breakdowns
  - raw counts and percentages that let operators understand why a list received its grade

The canonical scorecard object is:

```json
{
  "scorecard_version": "list_quality_v1",
  "quality_score": 84,
  "quality_grade": "B",
  "is_final": true,
  "completeness_pct": 100,
  "component_scores": {
    "deliverability": 88,
    "risk_hygiene": 79,
    "freshness": 82
  },
  "risk_breakdown": {
    "valid_pct": 74.0,
    "risky_pct": 15.0,
    "unknown_pct": 7.0,
    "invalid_pct": 4.0,
    "catch_all_pct": 9.0,
    "disposable_pct": 1.0,
    "role_account_pct": 3.0,
    "full_inbox_pct": 0.2,
    "spam_trap_pct": 0.0,
    "safe_to_send_pct": 71.0
  },
  "freshness": {
    "evaluated_at": "2026-03-30T17:30:00Z",
    "newer_than_14d_pct": 100.0,
    "older_than_30d_pct": 0.0
  }
}
```

### Row Inclusion Rules

Only rows with a durable task result for the effective list job are included. The v1 formula excludes:

- rows still in `queued` or `processing`
- rows with terminal infrastructure failure but no verification result
- duplicate replay artifacts created by retries, with the latest terminal attempt winning

If a list is associated with multiple verification runs over time, the quality score reads from the latest effective run linked to that list unless the caller later requests historical versions. Historical comparison is deferred; this RFC only defines the current snapshot.

### Component Score Semantics

The final score is still a single `0-100` integer, but component scores are defined so the total is explainable:

- `deliverability`
  - reflects valid/risky/unknown/invalid distribution
  - dominated by the base row-score average
- `risk_hygiene`
  - reflects catch-all, disposable, role, spam-trap, and full-inbox penalties
- `freshness`
  - reflects how recently rows were verified and whether the list is materially stale

Component calculation in v1:

- `deliverability = round(base_score)`
- `risk_hygiene = clamp(round(100 - hygiene_penalties), 0, 100)`
- `freshness = clamp(round(100 - freshness_penalties), 0, 100)`

Where:

- `hygiene_penalties = 0.20 * catch_all_pct + 0.25 * disposable_pct + 0.15 * role_account_pct + 0.50 * full_inbox_pct + 1.00 * spam_trap_pct`
- `freshness_penalties = 0.20 * stale_over_30d_pct + 0.40 * stale_over_90d_pct`

The headline `quality_score` remains the final clamped combined score and not a simple mean of component scores.

### Snapshot Lifecycle

Snapshots are produced at three times:

1. when a list job first reaches terminal completion
2. when a previously incomplete list becomes complete because retries finish
3. when an operator explicitly refreshes quality after result replay or remediation-relevant data changes

Snapshot recomputation is idempotent for the same list state. The recompute key is:

- `list_id`
- `effective_job_id`
- `scorecard_version`
- `result_state_digest`

If that tuple has not changed, reads reuse the existing snapshot.

### Exposure Model

The scoring surface is intentionally layered:

- `/v1/lists/{list_id}/quality`
  - full scorecard and evidence breakdown
- `/v1/lists/{list_id}`
  - compact quality summary
- `/v1/lists?include_quality=true`
  - compact summary per listed row

This keeps list index reads cheap while still letting callers fetch a richer explanation when needed.

## Public API / Interface Changes

Extend `GET /v1/lists/{list_id}/quality` with:

- `quality_score`
- `scorecard_version`
- `component_scores`
- `freshness`
- `completeness_pct`
- `is_final`

Extend `GET /v1/lists/{list_id}` with a compact `quality_summary`:

- `quality_score`
- `quality_grade`
- `processed_rows`
- `last_scored_at`

Extend `GET /v1/lists` with `include_quality=true` to optionally include the same compact summary per row.

Example `GET /v1/lists/{list_id}/quality` response:

```json
{
  "list_id": "lst_01HT...",
  "scorecard": {
    "scorecard_version": "list_quality_v1",
    "quality_score": 84,
    "quality_grade": "B",
    "is_final": true,
    "completeness_pct": 100,
    "component_scores": {
      "deliverability": 88,
      "risk_hygiene": 79,
      "freshness": 82
    },
    "risk_breakdown": {
      "valid_pct": 74.0,
      "risky_pct": 15.0,
      "unknown_pct": 7.0,
      "invalid_pct": 4.0,
      "catch_all_pct": 9.0,
      "disposable_pct": 1.0,
      "role_account_pct": 3.0,
      "full_inbox_pct": 0.2,
      "spam_trap_pct": 0.0,
      "safe_to_send_pct": 71.0
    },
    "freshness": {
      "evaluated_at": "2026-03-30T17:30:00Z",
      "newer_than_14d_pct": 100.0,
      "older_than_30d_pct": 0.0
    }
  },
  "counts": {
    "total_rows": 1000,
    "processed_rows": 1000
  }
}
```

Compact `quality_summary` on list detail and list list:

- `scorecard_version`
- `quality_score`
- `quality_grade`
- `is_final`
- `processed_rows`
- `total_rows`
- `last_scored_at`
- `safe_to_send_pct`

Compatibility rules:

- existing fields on `/quality` remain present
- new fields are additive
- callers that ignore the new nested structure still receive the old top-level summary fields until a later deprecation RFC

## Data Model / Storage Changes

Add `v1_list_quality_snapshots` with:

- `list_id`
- `tenant_id`
- `scorecard_version`
- `quality_score`
- `quality_grade`
- `summary`
- `computed_at`

Snapshot behavior:

- Recompute when a list job reaches a terminal state.
- Mark snapshots stale when list-derived task results are retried or replayed.
- Serve cached summaries to list detail/list list endpoints and fall back to live compute if no snapshot exists.

`v1_list_quality_snapshots` full columns:

- `id`
- `tenant_id`
- `list_id`
- `effective_job_id`
- `scorecard_version`
- `quality_score`
- `quality_grade`
- `is_final`
- `processed_rows`
- `total_rows`
- `summary JSONB`
- `result_state_digest`
- `computed_at`
- `stale_at`

Indexes:

- unique `(tenant_id, list_id, scorecard_version, result_state_digest)`
- btree `(tenant_id, list_id, computed_at desc)`
- btree `(tenant_id, quality_grade, computed_at desc)` for future list analytics reuse

Storage rules:

- `summary JSONB` stores the full scorecard payload returned by `/quality`
- compact list-level fields are duplicated into scalar columns for cheap filtering and sorting
- only the latest non-stale snapshot is served by default

This RFC does not require deleting old snapshots. Historical snapshots may be retained for later trend views, but are not yet exposed publicly.

## Auth, Permissions, And Tenant Isolation

- All scorecard reads require the existing `lists` scope.
- Snapshots are stored and queried per tenant.
- Legacy/open mode still behaves as full access when tenant auth is disabled.

Additional rules:

- callers cannot request another tenant’s scorecard via guessed `list_id`
- list index inclusion of `quality_summary` must respect the same list visibility rules as the current list endpoints
- future admin cross-tenant reporting is out of scope and must not reuse tenant routes

## Failure Modes And Edge Cases

- Empty lists return `quality_score = 0`, `quality_grade = F`, and `completeness_pct = 0`.
- Partially processed lists return provisional scores and never mark `is_final = true`.
- Snapshot recomputation failures do not block list processing; callers fall back to live compute.
- Historical scorecard versions are not recomputed after formula changes; only new snapshots use the new version.

Additional cases:

- lists with only infrastructure failures and no durable results return `is_final = false` until the job is terminal and row accounting is known
- if a list has zero processed rows but a terminal completed job due to empty upload, the response is still valid and not treated as an error
- if a retry replaces a prior task result, the old snapshot is marked stale immediately
- if freshness data is partially missing, the freshness component is computed from known rows only and the response includes that same effective denominator in `summary`

## Alternatives Considered

- Keeping the current live-compute response only. Rejected because list overviews and comparisons need cached summaries.
- Basing quality entirely on `safe_to_send_pct`. Rejected because it hides risk composition and freshness.
- Tenant-custom scoring formulas in v1. Rejected because it complicates comparability and support.

## Rollout / Migration / Compatibility

- Keep the existing `/quality` endpoint and response fields valid.
- Add snapshot creation behind a migration and backfill on first read for historical lists.
- Version the formula as `list_quality_v1` from day one.

Rollout plan:

1. add snapshot storage and internal scorer service
2. keep `/quality` functionally unchanged while sourcing from snapshot-or-live fallback
3. add compact `quality_summary` to list detail
4. add `include_quality=true` to list index
5. document formula and thresholds in docs and SDK examples

Backfill policy:

- no eager background migration of all historical lists in v1
- first read computes and stores a snapshot if one does not exist
- large tenants may later opt into background precompute, but that is outside this RFC

## Metrics And Success Criteria

- Coverage rate of lists with a populated quality snapshot.
- Snapshot compute duration and staleness rate.
- Distribution of list grades by tenant.
- Percentage of list reads served from cache vs live compute.

Additional success criteria:

- p95 `/v1/lists/{list_id}/quality` latency drops materially for completed lists relative to live aggregate-only mode
- support questions of the form "why is this list bad?" can be answered from the returned breakdown without manual SQL inspection

## Test Strategy

- Golden tests for quality formula and grade thresholds.
- Partial-processing tests for provisional scorecards.
- Snapshot invalidation and recompute tests.
- Endpoint compatibility tests for `/v1/lists/{id}` and `/v1/lists/{id}/quality`.

Add explicit cases for:

- empty upload list
- stale-over-30d and stale-over-90d penalty application
- live fallback when snapshot read fails
- concurrent recompute attempts resulting in one winning snapshot row
- `include_quality=true` on paginated list responses
- formula version bump preserving prior snapshots

## Unresolved Risks

- Fixed formula weights may not match every customer’s notion of “quality”.
- Snapshot drift can confuse callers if recomputation lag is too long after retries.

The main product risk is over-indexing on one composite number. The detailed breakdown is therefore required, not optional, in the full `/quality` response.
