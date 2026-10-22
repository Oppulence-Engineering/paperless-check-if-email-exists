# Verifier request and cache performance

The Overview uses one authenticated `GET /v1/overview` snapshot, with independent
ready, forbidden and unavailable sections. It counts outcomes in Postgres and
returns five lists and five pipelines, including progress and the existing
readiness decision. Optional setup reads start only when expanded. Existing API
routes remain available; authorization, quota admission and export eligibility
still run on the server for every action.

Rust caches public signing keys for five minutes. Refreshes coalesce, unknown
keys can trigger at most one refresh per ten seconds, and expired keys cannot
verify requests after a failed refresh. Fresh known keys continue during an
unknown-key refresh. Tenant claim synchronization skips unchanged records and
preserves billing precedence.

The browser retains inactive queries for five minutes, with 30-second display
freshness and 60 seconds for configuration and completed job metadata. Active
job status has one polling owner at five seconds; hidden and terminal views do
not poll. Successful mutations invalidate Overview and dependent safety views.
Workspace changes cancel requests and clear the isolated QueryClient. HTTP
responses stay `private, no-store`; this is an in-memory browser cache.

## Measurement

The BFF emits `Server-Timing` for `auth`, `token`, `upstream`, `pool_wait` and
`total`. Pool acquisition time is included in its parent authorization/token
phase, so the values must not be summed. `upstream` ends when response headers
arrive. Set `PERFORMANCE_TIMING_LOGS=1` to also log a fixed metric name, allowlisted
route family, method, status and durations. Logs exclude query strings, resource
IDs, email addresses and credentials. Neither database pool size nor deployment
topology changed.

On 2026-10-21, five authenticated reloads and five immediate client-side returns
were measured in each region, through a local TLS proxy and Kubernetes port
forward. Reloads clear the in-memory query cache but retain the browser HTTP
cache after the first sample. These are low-volume navigation measurements, not
concurrent load tests. Fixtures used an empty workspace and no billable checks;
fixture sessions were revoked. Populated lists are covered by the component and
Postgres parity tests.

| Environment | Region | Initial product reads | Successful endpoint p50 / p95 | HTTP 503s |
| --- | --- | --- | --- | --- |
| Production before | US East | 9 | 1,072 / 1,493 ms | 0 |
| Production before | Europe | 9 | 11,443 / 14,823 ms | 0 |
| Production before | US West | 9–18 including retries | 9,395 / 17,264 ms | 40 |
| Staging before | US East | 9 | 1,051 / 1,635 ms | 0 |
| Staging before | Europe | 9 | 11,859 / 15,266 ms | 0 |
| Staging before | US West | 9–18 including retries | 11,356 / 16,772 ms | 36 |

All immediate return visits reused the browser cache. Failed requests are
excluded from successful latency percentiles and reported separately. US West's
503s occurred with ready pods and no restarts; their cause remains unproven.
Compare staging before/after in the same environment, including failures and
cold navigation completion, before claiming the 50% p95 target.

## Bundle and query checks

The production build's Overview initial references fell from approximately
1.63 MiB JavaScript / 411 KiB CSS to 945 KiB / 303 KiB. Its largest initial chunk
fell from 768 KiB to 256 KiB. Development tools are deferred, inactive list tabs
and the saved-segments drawer load on demand, and marketing CSS is scoped to
marketing routes. These are uncompressed file sizes, not transferred bytes.
`pnpm bundle:check` enforces measured budgets plus 5% for Overview, lists, list
detail, analytics, research and settings. Use `pnpm analyze` for module attribution.

A local `EXPLAIN (ANALYZE, BUFFERS)` over five lists with 2,000 rows each completed
the list summary query in 24.1 ms and used existing tenant/list, job/task and
active-suppression indexes. This does not establish production query latency.
No migration, new index, Redis, replica or connection-budget increase is needed.

## Validation and release

Focused coverage checks signing-key rotation/outages, unchanged tenant writes,
tenant and member permissions, partial failures, summary parity, exact counts
over 500 outcomes, worker-owned list completion, delayed rechecks, cache reuse,
invalidation, hidden/terminal polling and request cancellation. The local
browser regression asserts one initial Overview request, no duplicate return
read, useful outage recovery, private navigation and the 390px suppression flow.

Staging must pass the existing full-stack image smoke test and sequential
regional workflow, followed by authenticated regional navigation measurements.
Production rollout is a separate future step; no production latency improvement
is claimed from local build or unit-test results.
