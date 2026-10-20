# Email verification audit remediation

Scope: every finding B01–B36 in [bugs.md](../../bugs.md), followed by computer-use inspection/provisioning of an Oxylabs SMTP-capable dedicated stable exit, DNS/SMTP identity configuration, platform deployment, and end-to-end proof. A locally passing test does not close a deployment or provider requirement.

Base: `da6741786ebf26e80166dda546f3e8a36b81e4e1`; branch: `fix/email-verification-audit`.

## Work and evidence

| Findings | Required change | Current state |
| --- | --- | --- |
| B01–B03 | Approved working SMTP route, valid identity, maintained capability health | Identity validation, cached periodic capability, readiness/metrics and no-charge admission guards implemented. Unit, HTTP and scheduler regressions pass; provider identity, live monitoring and route proof remain pending |
| B04–B08 | Policy errors, catch-all uncertainty/target probe, MX failover, bounded cleanup | Implemented; core suite 51 passed, 4 existing external-service tests ignored |
| B09–B10 | Restrict/pin SMTP, proxy, and webhook destinations | Code implemented; SMTP negative/SOCKS and webhook pinning tests pass. Helm egress isolation, exact dependency exceptions, DNS and monitoring render tests pass; live policy enforcement remains pending |
| B11–B14 | Durable idempotent single-check operation, persistence before reply, aligned deadlines | Durable admission, HTTP timeout/replay, disconnected-request recovery, terminal failure/refund and direct execution regressions pass. Actual broker-backed HTTP completion returns the saved result. SDK pending/completed response test passes; new operation scope/family mappings are included. A worker throttle regression now verifies eventual delivery instead of permanently dropping durable operations; full validation in progress |
| B15–B17 | Atomic distributed admission and fail-closed quota | Implemented; two-pool rate race and quota failure integration tests passed; backend unit suite passed |
| B18–B22 | Transactional job publication/retry, monotonic finalization, durable follow-up actions | B18 list insertion rollback and B20 retry transaction/publication fault tests pass. B19/B21 tests pass. B22 saved-result recovery, lease/generation fencing, action-failure, worker loop/storage, and all 25 delayed-recheck regressions pass |
| B23–B26 | Suppression reactivation/identity, durable outcome actions, consistent safety reads | B23/B25 rollback/retry, B26 lagging-replica, and B24 writer/migration tests pass |
| B27–B29 | Request settings/policy propagation and conservative mailbox canonicalization | B27 unit tests, B28 policy capture/queue/storage/replay, and B29 case-sensitive SMTP, queued-routing, history/migration tests pass; deployment reconciliation remains required |
| B30–B31 | Shared spreadsheet-safe CSV and secret-free database logs | Implemented across CSV writers; formula/control-character and existing export unit tests pass |
| B32–B34 | Honest DKIM/SPF/DNSBL evidence and error semantics | Backend/UI/schema changes, generated SDKs/contracts, unit/UI tests and web typecheck pass |
| B35–B36 | Transactional legacy pruning and lifecycle-safe list deletion | B35 rollback/retry test passes. B36 transactional deletion and lifecycle race tests pass; worker redelivery fencing still depends on B22 |
| Contracts/fixtures | Regenerate affected contracts and update all consumers | Reputation contracts regenerated. TypeScript SDK build, Go compile, web typecheck, production build and contract drift check pass; repeat after remaining contract changes |
| Validation | Focused regressions, Rust suites, isolated DB/broker fault/concurrency tests, contracts/builds | Pending |
| Oxylabs | Inspect actual account/product support using computer use; provision only a verified suitable exit | Pending |
| Deployment | Deploy validated revision/configuration; verify regional SMTP and controlled end-to-end checks | Pending |

## Compatibility and migration requirements

Identity changes must preserve original addresses and existing suppression/history data. Safety decisions must retain unknown/error states. Database/broker operations need stable identities, atomic ownership, and reconciliation across crashes. Changes to serialized requests/results must reach OpenAPI, generated SDKs, web contracts, CSV, and stored-result compatibility.

No finding is considered complete until its implementation and appropriate regression evidence are recorded here; B01–B03 and deployment additionally require current runtime proof.

## Network validation

Public destination checks reject private, reserved, documentation and transition ranges, validate every answer, and connect using that resolved address set. SOCKS receives a literal target IP, not a hostname. SMTP mock servers have an explicit test-only bypass that is absent from production builds and serialized configuration. Webhook clients disable environment proxies and redirects and pin the validated addresses while retaining the URL hostname for Host/TLS. Request routing overrides must match an operator-configured proxy and use SMTP port 25; alternate ports require operator configuration.

Special-purpose range references: [IANA IPv4](https://www.iana.org/assignments/iana-ipv4-special-registry/) and [IANA IPv6](https://www.iana.org/assignments/iana-ipv6-special-registry/). This is deliberately conservative for protocol/transition assignments. Deployment-level egress rules remain required.

## Reputation contract changes

`spf_valid` is replaced by `spf_policy_explicit`, a named heuristic for a terminal `all` or nonempty redirect in a single SPF record. It is not syntax validation or sender authorization; malformed records can still contain an explicit policy. SPF validity requires an evaluator, not this field. See [RFC 7208](https://www.rfc-editor.org/rfc/rfc7208.html).

`has_dkim` is nullable: true means usable key material was discovered; null means unknown. Each attempted selector carries its own key/error status. Missing common selectors never prove domain-wide absence and no longer incur a score penalty. Existing OpenSSL/base64 dependencies parse key material, including empty/revoked keys, RSA size, and Ed25519 format; this is not message-signature verification. See [RFC 6376](https://www.rfc-editor.org/rfc/rfc6376.html), [RFC 8301](https://www.rfc-editor.org/rfc/rfc8301.html), and [RFC 8463](https://www.rfc-editor.org/rfc/rfc8463.html).

DNSBL `listed` is nullable with explicit `listed`, `not_listed`, `error`, or `skipped` status and answer/error evidence. Query failures are not cached for 24 hours or presented as clean results. Older cached reputation payloads are bypassed and refreshed because they lack the new evidence fields. Generated SDKs and web contracts must be regenerated together before release.

## Validation checkpoints

- Core: 51 passed, 4 pre-existing external-service tests ignored (`remediation-core-identity.log`), including case-sensitive recipient replies.
- Backend: 366 unit tests, 7 API harness tests, 15 pipeline tests and 4 durable operation tests passed (`remediation-capability-tests.log`). Before the capability additions, 17 audit, 5 dispatch/retention, 19 HTTP handler, 7 HTTP/database and 11 idempotency regressions passed. Test helpers now pin Postgres 16, matching compose/CI, rather than the module's old Postgres 11 default.
- Web: the first full run passed 675 tests but found four suites blocked by a missing operation-family mapping, which is corrected. The next full run passed 681 of 688 tests; seven UI timing failures during excessive compilation load all passed in a single-worker rerun (17 tests). Formatting, lint, types and generation checks pass. Capability/OpenAPI compatibility passes after refreshing the stale version. Production build and remaining checks are running.
- SDKs: regenerated TypeScript build passes; Go's HTTP 202/200 union decoding regression passes (`remediation-final-go-sdk.log`).
- Local tool workaround: use `/opt/homebrew/opt/node@22/bin` in PATH because the default Homebrew Node binary cannot load its simdutf library. Installed existing frozen web dependencies and locked SDK dependencies without changing lockfiles.
- Refreshed the frozen local SDK dependency after generation/build so the web install contains newly generated ESM files.
- Full Rust integration suite, deployment and live SMTP proof remain pending. Logs are in `/tmp/paperless-audit-20261020/`; none of these local checks proves deployed behavior.

## Mailbox identity

B24/B29 implementation uses one provider-aware identity across suppression API/import, worker actions, both outcome paths, history, list comparisons and suppression-aware exports. SMTP routing retains the supplied address. The [migration and reconciliation plan](mailbox-identity-migration.md) preserves old rows/events, avoids merging case-distinct mailboxes, and requires review/reverification of historical lists whose original address differs from the actual checked address before resuming their automated use.

The writer regression also exposed a missing `tenants.settings` column used by the existing auto-suppression hook. Migration `20261020000003` supplies its operator-managed JSON object with an empty default. No tenant automatically gains a new suppression rule. B22 still requires durable hook execution and error propagation.

Added Cargo tracking for the migrations directory: otherwise SQLx's embedded migrator could omit a newly added migration in an incremental build. The identity test suite passed after rebuilding with all three new migrations.

## Durable completion

B21 reconciliation locks the parent before related lists, propagates database errors, and commits duplicate/list/job changes together. It requires all expected task rows before finalizing and preserves terminal parent states. Duplicate results propagate only between equal canonical identities and after the primary becomes terminal. Finder/pipeline failures are returned instead of swallowed.

B18/B20 now commit quota, rows and a private dispatch record together. Bulk creation also commits its idempotent response in that transaction. A publisher retains unconfirmed intent with capped backoff and publishes persistent messages; an uncertain confirmation can publish the same generation again.

Tasks with database identities use short leases renewed during work. Storage checks the lease and dispatch generation, records completion intent with the result, and never inserts a replacement for a missing task ID. Saved results resume required suppression/actions before finalization, without running verification again. A periodic recovery pass republishes interrupted completion. Webhooks carry the stable dispatch ID as an idempotency key; receivers must deduplicate uncertain deliveries. A crash before any result is saved can still require another verification attempt.

The new generation is also used for automatic and delayed retries. All 25 delayed-recheck regressions and the actual worker delivery regressions pass, including preservation of an earlier result when a recheck fails. Existing terminal history is retained as completed during migration; this does not prove that old releases executed every historical action.

Single-check operations now commit quota, task, dispatch and the idempotent pending response atomically. HTTP waits at most 20 seconds after admission, then returns a tenant-scoped retrieval URL. Replaying a key reads the same operation; pending operations are protected from stale-key cleanup and result pruning. Direct execution with Postgres uses the same leases and retains its dispatch until completion, allowing startup recovery. Terminal single-check errors refund once in the original quota period. The API, web polling client and SDKs carry the pending-result contract; storage-free idempotency/worker mode rejects before dispatch.

Migration `20261020000005` adds the operation identity and original quota period. Do not drop operation/idempotency fields during rollback while their work is pending. Client releases must accept HTTP 202 and poll instead of treating it as a verification result.

Before applying migration `20261020000004`, stop admission and drain/stop old workers in every region. Old binaries do not enforce the new lease fence and must not overlap new execution. Deploy the migration, publisher and workers together, then reconcile and resume. Do not roll back by dropping pending dispatch records.

B31 removes the database URL from connection-error context and the startup verification-method dump, which could contain proxy credentials.

## SMTP admission and deployment safeguards

The [SMTP capability contract](../advanced/smtp-capability.md) documents the 60-second monitor, 20-second probe bound, 180-second freshness, service-wide pause and readiness behavior. Startup rejects local/missing SMTP identities; legacy bulk workers now receive the operator configuration rather than synthesizing their own identity. New submissions and scheduled work pause before reserving quota or claiming new work; saved results may still complete their actions. Delayed retries preserve their publish attempt while capability is unavailable.

The Helm egress policy denies private/reserved destinations except exact operator-configured dependency addresses/ports. Rendering tests cover all three value overlays, IPv4/IPv6 negative cases, public routes, DNS scope and exception isolation. Deployment still needs the actual regional dependency inventory, CNI enforcement and additive-policy checks. The US East cluster did not have Prometheus Operator PodMonitor/PrometheusRule CRDs at inspection; live metrics and alert delivery are not yet configured or proven.

Full-workspace validation is limited to two Cargo build jobs with incremental compilation disabled to avoid memory/disk pressure. Disposable repository incremental cache was removed; source and test databases were preserved. The worker failure suite now kills an owned subprocess after durable completion and before acknowledgement; it must pass before claiming crash recovery is verified.
