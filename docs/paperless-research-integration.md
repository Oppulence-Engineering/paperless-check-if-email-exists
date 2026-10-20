# Paperless company-to-contact integration

Objective: connect the Go Paperless research backend to the current signed-in
Paperless application and MillionVerifier-backed Rust service. This checklist
tracks the complete first connected workflow, not just a compatible API client.

This implementation record captures the checks completed before production
promotion. Final deployed revisions and live workflow evidence are recorded in
[release PR #124](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/pull/124).

## Implementation workspaces (local only)

- Verification app: `paperless-check-if-email-exists`, branch
  `feat/paperless-contact-research`.
- Go implementation: isolated worktree
  `/Users/dyomba/.codex/worktrees/paperless-research-integration`, branch
  `feat/verifier-research-integration`, based on `origin/develop`.
- Preserve the unrelated dependency edits in the original Go checkout.

## Acceptance checklist

- [x] Align the product focus with the supplied prospecting thesis.
- [x] Reconcile RFCs 0011–0015 with API verification, evidence ownership, and the
  existing Go implementation rather than implement duplicate research services.
- [x] Establish authenticated server-side tenant/workspace mapping; reject guessed
  workspace, account, session, and export identifiers before provider work.
- [x] Adapt the Go verifier client to durable pending operations, stable retry keys,
  external-provider evidence, and policy decisions without invented SMTP findings.
- [x] Connect company CSV/JSON import and target-role selection to Go research
  sessions. Support supplied people for missing-address repair.
- [x] Resolve company identity; preserve ambiguous/no-match rows for review.
- [x] Research people and published contact evidence; distinguish person, employer,
  role, and mailbox claims. Generic company mailboxes are not named decision-makers.
- [x] Enforce approved per-run spend and candidate limits, stop early, and make
  polling/retries restart-safe without duplicate provider purchases.
- [x] Add the generated API/SDK contracts and signed-in import, progress, evidence,
  review, and export UI, preserving full width and desktop scaling.
- [x] Export approved contacts through the current suppression and sending-policy
  checks, with provenance and unresolved uncertainty in review exports.
- [x] Add reproducible benchmark fixtures/reporting for coverage, matching accuracy,
  verification outcomes, provider cost, and review workload.
- [x] Validate cross-tenant refusals, provider failures, budget exhaustion, duplicate
  imports, catch-all results, retries, and the complete import-to-export journey.
- [ ] Merge, deploy compatible Go/Rust/web revisions, and verify deployed revisions,
  regional readiness, and a controlled real workflow. Keep local/mock evidence
  separate from deployed/provider evidence.

## Findings and decisions

- Existing Go research-session APIs accept company imports, persist source records
  and evidence, and expose planning/approval, progress, results, and cost breakdowns.
- The Go verification client now polls accepted operations, reuses one retry key,
  validates completed responses, and preserves MillionVerifier and sending-policy
  evidence. Research uses the workspace verifier and stable task-derived keys.
- The trusted `EnsureVerifierWorkspace` Go RPC now creates an isolated mapping,
  organization, tenant, account, workspace, bounded trial research subscription,
  and encrypted workspace API key in one transaction. Concurrent retries reuse
  the same mapping. Customer API keys cannot call the provisioning RPC.
- Reverse verification resolves the existing Rust tenant rather than provisioning
  a second billing tenant. New service keys use the actual named Rust scopes.
- Rust's `ResearchConfig` bridges to Go with server-only authority, bounded
  responses, sanitized errors, and no redirects or automatic HTTP retries. Its
  public `/v1/research` routes use the existing `find` scope and generated SDKs.
- The Go contact pipeline has useful source/role gates; some older research paths
  accept the first email extracted from a website. Such a result must not be labeled
  a confirmed person or decision-maker.
- The initial integration must not depend on a Parallel data-resale exception.
  Existing permitted site research and properly licensed sources can be used while
  provider-specific rights remain explicit.
- The business thesis's paid pilot recruitment and competitor benchmark collection
  remain commercial follow-up; implementation supplies the measurable workflow and
  evaluation tooling. Expanded discovery, job-change monitors, and CRM automation
  remain the subsequent product phases described in the thesis.
- Research imports now reject overflow before session creation or persistence.
  The pilot accepts at most ten rows, three candidates per company, and thirty
  verification requests per run. Missing domains stay in review; name-only
  company resolution is not claimed.
- Older `company_website`, `contact_waterfall`, and `reacher_verification` research
  sources are derived steps, not independent executable jobs. Dynamic research
  columns already supply durable per-row execution and evidence; use that path
  for the connected contact workflow rather than queueing an inert source plan.
- The new contact column reserves verification requests transactionally in existing
  usage events before calling the workspace verifier. Completed results are reused;
  unresolved outcomes are retained for review and never automatically repurchased.
  Website research uses the existing bounded crawler without paid research providers.
  Credit reservations are not reported as measured dollar expenditure.
- Initial read-only check on 2026-10-20: the Go gateway and lead-scraper deployments
  are at zero replicas in both `paperless-staging` and `paperless-production` on
  `oppulence-us-east`. Verification app deployments remain at one replica. Do not
  call the integration deployed until compatible Go services are restored and
  their database/queue capacity is verified. The release work below restores
  those dependencies rather than relying on the retired database/Redis hosts.

## Validation evidence

Local checks completed:

- Go `make grpc` in `api-definitions/api/lead-scraper-service`: lint and generation
  passed using pinned plugins. Unrelated GORM preload-order churn was removed.
- `go test ./services/lead-scraper-service/pkg/reacher -count=1`: passed.
- `go test ./services/lead-scraper-service/internal/emailquality -count=1`: passed,
  including use of the existing verification tenant and exact service-key scopes.
- `go test ./services/lead-scraper-service/internal/database -run TestVerifierWorkspace -count=1 -v`:
  passed, including concurrency on a real disposable PostgreSQL 17 instance.
- Focused research-contact, qualified-lead, email-quality, and bootstrap middleware
  tests passed; provider-failure propagation has a regression test that failed
  before the fix.
- `SQLX_OFFLINE=true CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 cargo test -p reacher_backend --lib research::tests -- --nocapture`:
  seven passed (authority, redaction, redirect refusal, request caps, import-only
  creation, and separate person/mailbox evidence).

- Go cap concurrency test on PostgreSQL: three of ten requests admitted at cap three.
- Rust `e2e_research`: passed import creation/retry, cross-tenant refusals, approved
  export, CSV formula escaping, fresh suppression, stale history, new complaint
  feedback, newer pending verification, and an eligible alternative when the
  first candidate is no longer allowed. No export/provider repurchase.
- Rust complete API harness: seven tests passed, including every live route,
  runtime OpenAPI inventory, auth, validation, missing resources and conflicts.
- Generated TypeScript and Go SDKs, copied web OpenAPI and Orval runtime contracts.
- Web research panel: five tests passed; retry key reuse, evidence distinctions,
  selection, fresh export exclusions and terminal polling. Typecheck passed.
- Benchmark script: three tests passed. Synthetic comparison fixture is explicitly
  not evidence of provider pricing or competitive performance.

- Production web build passed; the research and developer-portal suites passed all
  13 tests. The generated OpenAPI snapshot preserves repository formatting.
- Actual local browser -> Rust -> Go -> Redis worker -> review CSV passed using an
  isolated PostgreSQL 18 database. A missing-domain row remains Needs review,
  cannot enter approved export, and consumes zero verification requests. Local-only proof:
  `/Users/dyomba/Documents/Codex/proofs/paperless-research-20261020/local-review-export.png`.
- The browser check exposed a fresh-schema migration bug: the webhook tombstone
  model replaced the full table model. The fix retains the generated columns;
  a regression check failed before the fix and passed afterwards with race checks.
- Research-only workers register just research-column tasks and skip legacy
  background operations. Focused worker tests and Go vet passed. Helm assertions
  passed for both environments in all three regions.
- Existing managed Redis DNS no longer resolves. Deployment now includes an
  authenticated private research queue per region/environment with AOF persistence,
  a 10 GiB volume, no eviction, and a same-release ingress network policy.
- Each Go process is capped at one Postgres connection; Rust and auth pools are
  capped at two and one respectively. Local import/provisioning/worker/export
  passed with the one-connection Go pools. Live capacity must be checked at rollout.
- Infisical now contains the trusted Rust admin credential, an environment-specific
  credential encryption key, and separate research queue credentials. No key is
  included in the repository or browser response.

## Release evidence

- Go PR #1191 merged as `94d7a598c9474489fb541e5cfe7f930c5aeff559`.
  The application image was built from that exact source tree and pushed with
  digest `sha256:ff8fe4d9ad875cd5d24ae4d4265ec53f402c152794b2638c3f70e6cd64951fcc`.
- Go PR #1192 merged as `b584adce16836e95530836cbf950375c2851129a`.
  Its chart-only correction gives production the same 15-minute migration/startup
  allowance as staging; no application binary changed.
- Go PR #1193 merged as `9a97d58891cf95247f0640bed864b02e91b94bbc`.
  Unicode role limits passed focused race tests. Its release image digest is
  `sha256:a5279464042aeba30354abee80d10af62f26b3a4c12b7139910237c766fdb385`.
- Verifier PR #123 merged as `c9219dbf28afa6dcc8bdc9d5e6f55faf378da2cb` after
  all eight CI jobs passed on `da7a14ff4e1808fc469598114655be0254491fde`.
  The merge has the identical source tree. Gates included the full Rust suite,
  HTTP API harness, migration upgrade, complete web suite/build, and security scans.
- New, separate Go staging/production databases and scoped users were provisioned
  in the existing managed PostgreSQL cluster. Infisical is the credential source.
  Encryption keys use the base64 encoding accepted by the Go credential cipher.
- The verifier's network policy permits only same-namespace Go HTTP pods on TCP
  8080. The rendered isolation checks pass; a live authenticated service path is
  reachable and an unauthenticated provisioning request returns 401.
- All six Go staging and production deployments completed their rollouts. A deployed staging import
  provisioned its workspace, ran through the persistent queue and worker, and
  returned `needs_review / company_domain_required` with zero provider requests.
  Test tenant: `99a0b1d7-a0b5-48f0-a3d3-136da5c0ba30`, research session: `1`.
- The merged verifier staging image `c9219dbf28afa6dcc8bdc9d5e6f55faf378da2cb`
  passed authenticated research detail and export checks: completed review row,
  zero verification requests, zero approved exports, one review export. The
  temporary API key was revoked and the isolated test tenant's plan restored.
- GitHub Actions remains disabled in the Go repository. The attempted staging
  dispatch `39504112233` has no jobs and never executed; GitHub rejects both its
  cancellation (409, not yet queued) and deletion (403). Clear that pending
  dispatch before re-enabling repository Actions. The Go release uses the verified
  Depot image and direct Helm deployment instead.

Merged-image deployment and the controlled production browser/provider workflow
remain outstanding. Full Go hooks were blocked by a MinIO image pull and then local
disk exhaustion; focused relevant race tests and service vet passed. Only
regenerable task build artifacts were cleared. No provider performance or
gross-margin claim has been made.

Production review follow-up aligns published request limits with runtime checks,
preserves decimal session IDs through SDK callers, counts international names and
roles by Unicode characters in both services, and stops polling rows without an
active cell run or result. Import-session completion is deliberately not used to
stop active research-column jobs, whose lifecycle is separate.
