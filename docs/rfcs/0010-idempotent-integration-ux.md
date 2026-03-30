# RFC 0010: Idempotent Integration UX

## Status

Draft

## Summary

Standardize idempotency behavior across Reacher’s write-heavy endpoints so integrators get the same semantics, headers, error handling, and replay behavior regardless of which workflow they are calling. The design reuses the current idempotency store, keeps the default TTL at 24 hours, and removes accidental cross-endpoint key collisions.

## Motivation / Problem

- Idempotency exists today, but only part of the write surface uses it consistently.
- The current unique key is tenant plus idempotency key, which makes accidental cross-endpoint collisions possible.
- Integrators need one clear contract for duplicate retries, in-progress collisions, and body mismatches.

## Goals

- Support `Idempotency-Key` consistently on write-heavy endpoints.
- Make replay and error semantics uniform.
- Preserve the current 24-hour default replay window.
- Keep the feature additive for callers that do not send an idempotency key.

## Non-Goals

- Exactly-once guarantees across all external side effects.
- A public idempotency-inspection endpoint.
- Infinite retention of cached responses.

## Current State In Reacher

- Reacher already has an `idempotency_keys` table and cleanup worker.
- `/v0/check_email` and `/v1/check_email` already honor `Idempotency-Key`.
- The current retention window is 24 hours and stale processing rows are failed after 5 minutes.

## Proposed Design

Standardize `Idempotency-Key` across:

- `POST /v0/check_email`
- `POST /v1/check_email`
- `POST /v1/bulk`
- `POST /v1/lists`
- `POST /v1/find_email`
- `POST /v1/pipelines`
- `POST /v1/pipelines/{pipeline_id}/trigger`
- `POST /v1/pipelines/{pipeline_id}/pause`
- `POST /v1/pipelines/{pipeline_id}/resume`
- `POST /v1/comments`
- `POST /v1/suppressions`
- all future write-heavy v1 POST/PATCH/DELETE endpoints that create durable tenant state

Core decisions:

- Uniqueness changes from `(tenant_id, idempotency_key)` to `(tenant_id, request_path, idempotency_key)`.
- Default retention remains `24` hours.
- Default stale-processing timeout remains `5` minutes.
- If an idempotency key is provided and the idempotency store is unavailable, the request fails closed with `503`.

Hashing rules are:

- JSON requests: hash canonical JSON bytes.
- Multipart list uploads: hash normalized form fields plus uploaded file content digest.
- Empty-body mutation routes: hash method, request path, and normalized query string.

Standard response headers are:

- `Idempotency-Status: new|cached|in_progress|mismatch`
- `Idempotency-Replayed: true|false`

Standard behavior is:

- first request -> execute and cache response
- same key, same path, same body -> replay cached response with original status code
- same key, same path, different body -> `409 Conflict`
- same key, same path, in progress -> `409 Conflict`

### Scope Of Coverage

This RFC treats idempotency as a platform contract, not an endpoint-specific optimization. Coverage is mandatory for:

- create endpoints
- trigger endpoints
- pause/resume endpoints
- comment creation
- suppression creation

Coverage is optional and explicitly deferred for:

- pure read endpoints
- bulk file downloads
- long-lived streaming responses

DELETE and PATCH routes that mutate durable state must opt in as they are added or audited, using the same underlying contract.

### Idempotency Namespace

The canonical uniqueness tuple becomes:

- `tenant_id`
- `request_method`
- `request_path`
- `idempotency_key`

`request_path` uses the normalized route template semantics already used by the HTTP layer, not the raw URL string with arbitrary ordering artifacts. Query strings are excluded from namespace identity unless the route is one of the explicit empty-body mutation routes that hashes them into the request fingerprint.

Examples:

- `POST /v1/comments` with key `abc` and `POST /v1/suppressions` with key `abc`
  - no collision
- `POST /v1/pipelines/pipe_1/trigger` with key `abc` and the same route with the same key
  - same namespace, subject to replay logic

### Fingerprint Rules

The request fingerprint combines:

- normalized route identity
- canonicalized body digest
- selected query digest for empty-body mutation routes

Canonicalization rules:

- JSON
  - parse and reserialize to canonical JSON before hashing
- multipart
  - normalize field ordering
  - hash file content bytes, not filenames alone
  - ignore transport-specific multipart boundaries
- form-urlencoded
  - normalize key ordering and repeated values
- empty body
  - hash normalized query string plus path template and method

This is intentionally stricter than "raw bytes equality" so semantically identical client retries are treated as the same request even if field order differs.

### Response Semantics

Status behavior is fixed:

- `new`
  - execute normally and store final response
- `cached`
  - return the stored status code and body exactly as first observed
- `in_progress`
  - return `409 Conflict`
- `mismatch`
  - return `409 Conflict`

Headers:

- `Idempotency-Status`
- `Idempotency-Replayed`
- optional future `Idempotency-Expires-At` is deferred

The body of cached responses is replayed byte-for-byte from stored normalized response data, including non-JSON responses where supported by the underlying endpoint.

### Failure And Recovery Model

Row states are:

- `processing`
- `completed`
- `failed`

Recovery rules:

- a request that fails before durable side effects are committed may mark the key `failed`
- a subsequent retry with the same key and matching fingerprint may execute again
- a stale `processing` row older than the configured timeout is marked `failed` by the recovery path

If the store is unavailable and the caller sent `Idempotency-Key`, Reacher returns `503` because failing open would create unsafe duplicate state while still advertising idempotent behavior.

### Integrator Guidance

The docs and SDK examples should recommend:

- one key per logical business action
- reuse the same key across client retries of the same action only
- do not recycle keys for different actions within the 24-hour window
- always log the key on the client side for support/debugging

This RFC is intentionally called "integration UX" because the goal is not just backend correctness. It is a clear operator contract.

## Public API / Interface Changes

Document `Idempotency-Key` on the endpoints listed above.

Standard error bodies:

- in-progress:
  - `{"error":"Idempotency key is already in use for this path"}`
- body mismatch:
  - `{"error":"Idempotency key body mismatch"}`

Standard response headers:

- `Idempotency-Status`
- `Idempotency-Replayed`

Add config knobs:

- `idempotency_ttl_hours` default `24`, min `1`, max `168`
- `idempotency_processing_timeout_minutes` default `5`, min `1`, max `60`

Example success response headers:

```text
Idempotency-Status: cached
Idempotency-Replayed: true
```

Example mismatch response:

```json
{
  "error": "Idempotency key body mismatch"
}
```

Example in-progress response:

```json
{
  "error": "Idempotency key is already in use for this path"
}
```

SDK-facing additions:

- OpenAPI examples showing header usage on eligible routes
- generated client documentation noting which methods accept idempotency headers
- integration guide examples for JSON and multipart endpoints

## Data Model / Storage Changes

Modify `idempotency_keys` to:

- add `request_method`
- change the uniqueness constraint to `(tenant_id, request_path, idempotency_key)`

Keep existing fields for:

- body hash
- status
- cached response body
- cached response status code
- lock owner and lock timestamp
- expiry timestamp

Additional clarifications:

- cached response headers may be stored only for a safe allowlist rather than every raw header
- response body storage continues to cap payload size; endpoints exceeding the cap may opt out of replay-body caching and store only status plus metadata in a later follow-on

Proposed columns:

- `tenant_id`
- `request_method`
- `request_path`
- `idempotency_key`
- `request_hash`
- `status`
- `response_status_code`
- `response_body`
- `response_content_type`
- `response_headers JSONB`
- `lock_owner`
- `locked_at`
- `expires_at`
- `created_at`
- `updated_at`

Indexes:

- unique `(tenant_id, request_method, request_path, idempotency_key)`
- btree `(status, expires_at)`
- btree `(tenant_id, expires_at)`

## Auth, Permissions, And Tenant Isolation

- Idempotency behavior inherits the auth rules of the underlying endpoint.
- Cached responses are tenant-scoped and path-scoped.
- Legacy/open mode still uses the synthetic tenant identifier already used by the current idempotency implementation.

Additional rules:

- a key created under one tenant can never replay under another tenant, even if the caller reuses the same literal key string
- auth failures themselves are not cached as successful idempotent results; the request must first pass endpoint auth to enter the idempotency flow
- admin-secret protected routes, if later brought under idempotency, must use a separate synthetic tenant namespace rather than mixing with tenant API keys

## Failure Modes And Edge Cases

- A missing `Idempotency-Key` means current endpoint behavior is unchanged.
- Cached responses replay the original status code and body, including non-`200` success statuses such as `201`.
- Failed attempts mark the key as failed and allow a subsequent retry with the same key and body.
- Cleanup continues to purge expired rows and mark stale `processing` rows as failed.

Additional cases:

- if a client reuses a key on the same path after TTL expiry, the request is treated as new
- if two requests with the same key arrive concurrently, exactly one wins the `processing` lock and the other receives the `in_progress` conflict
- if a route returns `202 Accepted`, the replay behavior still returns `202` with the original body
- if response-body persistence fails after the underlying action succeeds, the request must surface a typed `500` and mark the idempotency row failed so a retry can be safely reasoned about
- if an endpoint changes route templates in a future version, the old and new paths intentionally do not share idempotency history

## Alternatives Considered

- Keeping uniqueness on tenant plus key only. Rejected because it causes avoidable cross-endpoint collisions.
- Supporting idempotency on only a few create endpoints. Rejected because integrators need one rule, not many exceptions.
- Failing open when the idempotency store is unavailable. Rejected because it can create duplicate side effects while still implying safety to callers.

## Rollout / Migration / Compatibility

- Add support endpoint by endpoint, starting with current create/trigger routes.
- Migrate the uniqueness constraint in place and preserve existing rows until they expire.
- Keep old clients working unchanged when they do not send `Idempotency-Key`.
- Update OpenAPI and SDK examples as part of rollout.

Rollout plan:

1. migrate storage schema to the new uniqueness tuple
2. adopt the common middleware on currently supported routes
3. extend coverage to remaining write-heavy endpoints
4. update OpenAPI and SDK docs
5. add cross-endpoint collision regression tests to CI

Compatibility notes:

- existing keys in the old schema may coexist until expiration; no long-term dual-read path is required because the window is short
- clients that already use keys on check-email routes keep working, but gain the stricter namespace semantics automatically

## Metrics And Success Criteria

- Percentage of eligible write requests that include `Idempotency-Key`.
- Replay hit rate.
- Body mismatch rate.
- In-progress collision rate.
- p95 lookup latency for idempotency checks.

Additional success criteria:

- duplicate side-effect incidents attributable to client retries should trend toward zero on covered routes
- support and integration docs can describe one idempotency contract instead of per-endpoint special cases

## Test Strategy

- Golden tests for new, cached, failed, in-progress, and body-mismatch cases.
- Cross-endpoint collision tests proving the same key may be reused safely on different paths.
- Multipart upload hash stability tests.
- Store-unavailable fail-closed tests.
- Cleanup and stale-lock recovery tests.

Add explicit cases for:

- semantically identical JSON with different field order
- multipart requests with stable file bytes but different multipart boundaries
- concurrent same-key same-body requests
- same-key same-path after TTL expiry
- auth failure before idempotency record creation

## Unresolved Risks

- Multipart semantic hashing can still be subtle if the upload surface changes later.
- Strong fail-closed behavior can surprise clients that send idempotency keys before provisioning Postgres correctly.

The main implementation risk is uneven adoption. If some new write routes bypass the shared middleware, the contract becomes confusing again, so coverage should be enforced in route reviews and API harness tests.
