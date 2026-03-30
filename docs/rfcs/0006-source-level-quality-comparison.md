# RFC 0006: Source-Level Quality Comparison

## Status

Draft

## Summary

Add first-class source attribution for list uploads, bulk jobs, and later delivery outcomes so Reacher can compare quality across acquisition sources. The feature remains analytics-first and backend-only: callers tag work with a stable `source_key`, and Reacher produces source-level quality rollups and comparison endpoints.

## Motivation / Problem

- Reacher can already score emails and lists, but it cannot tell customers which acquisition source produced the best or worst records.
- Source attribution is currently ad hoc metadata at best, which makes analytics inconsistent.
- Customers need source-level quality reporting to make budget and vendor decisions.

## Goals

- Add a stable source dimension to verification workloads.
- Make source attribution explicit and queryable.
- Expose source-level rollups and side-by-side comparisons.
- Reuse existing job, list, score, and suppression data.

## Non-Goals

- Inferring sources from list names or filenames.
- Building warehouse connectors or BI exports in the same feature.
- Making delivery outcomes a hard prerequisite for source analytics.

## Current State In Reacher

- Bulk jobs and list uploads already persist tenant-owned work and quality data.
- The current APIs do not expose a formal `source_key` or `source_type`.
- There is no source dimension table or source analytics endpoint.

## Proposed Design

Introduce a tenant-scoped source model:

- `source_key` is the immutable analytics key and must be lowercase kebab-case.
- `source_display_name` is mutable human-readable text.
- `source_type` is one of:
  - `crm`
  - `form`
  - `csv_upload`
  - `enrichment_vendor`
  - `manual`
  - `api`
  - `provider_webhook`

Attribution rules are fixed:

- `POST /v1/bulk` accepts `source_key`, `source_display_name`, and `source_type`.
- `POST /v1/lists` accepts the same values as multipart form fields.
- Missing values are recorded under the synthetic source key `unknown`.
- Reacher never infers a source from `job_id`, `list name`, or filename.
- If an existing `source_key` is reused with a new display name, the display name is updated and the key stays immutable.

Rollups are generated from verification results, suppressions, and later outcomes when they exist. Outcome metrics are nullable until RFC 0001 is implemented.

### Source Identity Model

Source attribution has three layers:

- `source_key`
  - stable analytics identity
  - immutable once created for a tenant
- `source_display_name`
  - mutable presentation label
  - safe to rename without rewriting historical attribution
- `source_type`
  - controlled enum used for grouping and filtering

This deliberately separates analytical identity from display text. A source renamed from "Apollo CSV" to "Apollo SDR Import" remains the same source if the tenant reuses `source_key=apollo`.

### Attribution Rules By Surface

Bulk verification:

- `POST /v1/bulk` accepts explicit source attributes
- every task result created from that job inherits the effective source

List uploads:

- `POST /v1/lists` accepts the same source attributes
- the linked verification work inherits the list source unless later overridden by a rerun request

Future provider outcome ingestion:

- if a provider event is linked to a prior verification result, it inherits that result’s `source_key`
- if no matching verification record exists, the outcome may still carry a caller-provided source

If both a list and a later job rerun specify sources, precedence is:

1. explicit source on the rerun job
2. stored source on the list
3. synthetic `unknown`

### Normalization Rules

`source_key` format:

- lowercase ASCII letters, digits, and hyphen only
- length `1..64`
- cannot begin or end with `-`
- examples:
  - `hubspot-inbound`
  - `apollo`
  - `vendor-a-q1`

Reserved keys:

- `unknown`
- `system`

Callers may not explicitly create reserved keys; they are system-managed only.

### Rollup Semantics

Source rollups exist at two granularities:

- daily materialized rollups for fast trend reads
- on-demand aggregated reads over the daily table for week, month, or arbitrary range views

The base metric set in v1 includes:

- ingestion volume
- deliverability quality metrics
- suppression interaction
- freshness markers
- outcome metrics where present

Rollups are computed from the latest effective verification result per logical row, not from every retry attempt. This avoids retry noise inflating counts.

### Missing And Partial Metadata

The system accepts missing attribution because backward compatibility matters. However, the behavior is fixed:

- missing all source fields -> attribute to `unknown`
- present `source_key` with missing display name -> reuse existing display name if key exists, else default display name to the key
- present display name without `source_key` -> reject with `400`; callers must provide a stable key
- present key with conflicting type for an existing key -> reject with `409` rather than silently mutating source meaning

This keeps the catalog analytically stable and prevents one source key from drifting across categories.

## Public API / Interface Changes

Add write-time request fields:

- `source_key`
- `source_display_name`
- `source_type`

Add read endpoints:

- `GET /v1/analytics/sources`
- `GET /v1/analytics/sources/{source_key}`
- `GET /v1/analytics/sources/compare?source_key=a&source_key=b`

`GET /v1/analytics/sources` supports:

- `from`
- `to`
- `source_type`
- `include_unknown`
- `group_by=day|week|month`

Rollup fields include:

- `total_records`
- `avg_score`
- `valid_pct`
- `risky_pct`
- `unknown_pct`
- `invalid_pct`
- `safe_to_send_pct`
- `suppressed_pct`
- `outcome_delivered_pct`
- `outcome_bounce_pct`

Example bulk request fragment:

```json
{
  "emails": ["a@example.com", "b@example.com"],
  "source_key": "hubspot-inbound",
  "source_display_name": "HubSpot Inbound",
  "source_type": "crm"
}
```

Example source rollup response:

```json
{
  "source": {
    "source_key": "hubspot-inbound",
    "source_display_name": "HubSpot Inbound",
    "source_type": "crm"
  },
  "window": {
    "from": "2026-03-01",
    "to": "2026-03-30",
    "group_by": "day"
  },
  "totals": {
    "total_records": 4200,
    "avg_score": 86.4,
    "valid_pct": 76.0,
    "risky_pct": 14.0,
    "unknown_pct": 6.0,
    "invalid_pct": 4.0,
    "safe_to_send_pct": 72.0,
    "suppressed_pct": 3.0,
    "outcome_delivered_pct": null,
    "outcome_bounce_pct": null
  },
  "series": [
    {
      "bucket": "2026-03-01",
      "total_records": 120,
      "avg_score": 84.1,
      "safe_to_send_pct": 70.0
    }
  ]
}
```

`GET /v1/analytics/sources/compare` response shape:

- `sources`
  - ordered array of source summaries
- `comparison`
  - metric-by-metric delta block
- `window`

The compare endpoint supports between two and five `source_key` values in a single request to keep payloads bounded and useful.

## Data Model / Storage Changes

Add tables:

- `v1_sources`
- `v1_source_quality_daily`

Add nullable attribution columns to:

- `v1_bulk_job`
- `v1_lists`
- `v1_task_result`

Storage decisions:

- `v1_sources` holds the current source catalog per tenant.
- `v1_source_quality_daily` stores daily rollups for fast analytics reads.
- `v1_task_result` carries the effective `source_key` for downstream joins.

`v1_sources` columns:

- `id`
- `tenant_id`
- `source_key`
- `source_display_name`
- `source_type`
- `status` with values `active|inactive`
- `created_at`
- `updated_at`

Indexes:

- unique `(tenant_id, source_key)`
- btree `(tenant_id, source_type, updated_at desc)`

`v1_source_quality_daily` columns:

- `tenant_id`
- `source_key`
- `bucket_date`
- `total_records`
- `avg_score`
- `valid_count`
- `risky_count`
- `unknown_count`
- `invalid_count`
- `safe_to_send_count`
- `suppressed_count`
- `outcome_delivered_count`
- `outcome_bounce_count`
- `computed_at`

Indexes:

- unique `(tenant_id, source_key, bucket_date)`
- btree `(tenant_id, bucket_date desc)`

Attribution columns added to write-path tables:

- `source_key` on `v1_bulk_job`
- `source_key` on `v1_lists`
- `source_key` on `v1_task_result`

`v1_task_result` gets denormalized `source_key` specifically so analytics queries do not need to traverse jobs or lists for every row.

## Auth, Permissions, And Tenant Isolation

- Bulk and list creation remain gated by existing `bulk` and `lists` scopes.
- Add a new read scope, `analytics.read`, for `/v1/analytics/sources*`.
- Legacy and open-mode access still behaves as unrestricted when tenant auth is disabled.
- Source analytics must never aggregate across tenants.

Additional rules:

- write callers may only upsert sources inside their own tenant catalog
- analytics routes may not accept `tenant_id` query parameters
- identical `source_key` strings in different tenants are unrelated and must never be merged in caches or materializations

## Failure Modes And Edge Cases

- Invalid `source_key` format returns `400`.
- Missing attribution is accepted and rolled up under `unknown`.
- A deleted source catalog row is not allowed while referenced by jobs or lists; callers may deactivate it instead.
- Historical rows keep their original `source_key` even if the display name changes later.

Additional cases:

- if rollup recomputation fails for one source on a day, other sources still compute normally; the failed source is retried on the next run
- if a result has `source_key=unknown` and the tenant later creates a real source named `unknown-import`, historical unknown rows remain on `unknown`
- if a job mixes list-derived rows and direct API rows, all rows inherit the job-level source for that run
- if outcome metrics are absent for a source, the API returns `null` for rate fields rather than `0` to avoid implying a measured zero

## Alternatives Considered

- Keeping source attribution as free-form metadata only. Rejected because analytics queries become inconsistent and expensive.
- Inferring sources from upload filenames. Rejected because it is brittle and not tenant-safe.
- Requiring delivery outcomes before source analytics. Rejected because verification-only quality is already valuable.

## Rollout / Migration / Compatibility

- Add fields additively to list and bulk creation.
- Backfill historical jobs/lists/results to `source_key = unknown`.
- Keep outcome metrics nullable until RFC 0001 is implemented.

Rollout plan:

1. add source catalog table and write-path columns
2. begin storing `unknown` for unattributed new writes
3. backfill historical rows
4. add daily rollup job and analytics endpoints
5. layer in outcome metrics after RFC 0001 ships

Compatibility decisions:

- callers that do not pass source metadata keep working unchanged
- all new write fields are optional
- analytics endpoints tolerate older historical data that predates full attribution

## Metrics And Success Criteria

- Percentage of new jobs and lists with explicit source attribution.
- Source rollup generation latency.
- Share of traffic still attributed to `unknown`.
- Usage of compare endpoint per tenant.

Additional success criteria:

- source analytics read latency remains stable for a 90-day window because queries hit daily rollups rather than raw task results
- at least one clear operational action can be derived from the data, such as dropping a low-performing vendor source or prioritizing a higher-quality inbound channel

## Test Strategy

- Input validation tests for `source_key` and `source_type`.
- Auto-upsert tests for reused keys with renamed display names.
- Aggregation tests by day, week, and month.
- Tenant isolation tests across shared `source_key` values.
- `unknown` fallback tests.

Add explicit cases for:

- conflicting `source_type` reuse on an existing key
- backfill of historical data into `unknown`
- result retry dedupe so rollups are not double-counted
- compare endpoint ordering and delta correctness
- null outcome metrics until downstream event ingestion is available

## Unresolved Risks

- Customers may tag the same real-world source with multiple keys, fragmenting analytics.
- Outcome metrics will be sparse until the webhook ingestion feature lands.

The most likely operational risk is taxonomy sprawl. Documentation and SDK examples need to push customers toward a small, stable source catalog rather than one-off keys per upload.
