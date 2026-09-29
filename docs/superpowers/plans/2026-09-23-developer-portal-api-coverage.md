# Developer portal: complete API coverage

Date: 2026-09-23

Baseline: `develop` at `a013bfb8` (PR #51)

## Outcome

Make the app a developer platform around the Rust API. A developer should be able to discover every operation, understand its authentication and scope, call every tenant-supported v1 operation on the app's existing HTTPS host, and use a purpose-built workspace console for the tenant workflows. Platform control-plane operations need a separate, audited admin surface. Machine callbacks, health endpoints, onboarding, and legacy v0 need accurate reference and integration guidance; they do not need misleading buttons in the tenant console.

Completion means every method/path in the backend route registry has an explicit audience, reference page, example or callback contract, and either a usable console workflow or a documented reason that it is server-to-server, system, legacy, or restricted admin. The route-to-portal inventory must fail CI when a new operation is unclassified.

## Current state and gaps

- `backend/openapi.json` and the runtime route inventory contain **106 operations**. `backend/tests/e2e_api_harness.rs` already checks registry/OpenAPI parity and invokes every route.
- The product UI calls **13 operations**: check email; email history; list and create lists; list, create, and revoke workspace API keys; get/update tenant settings; get usage; and get/update/clear the tenant webhook. Its main rail has Check email, Lists, and History. The `/app/admin` page lists Better Auth workspaces; it does not use the 20 Rust admin operations.
- Developer settings link to raw `/api/backend/openapi.json`. The web app has no developer reference route. The existing Fumadocs config defines blog and customer content; the checked-in OpenAPI docs cover only three endpoints.
- Ten OpenAPI operations have no `operationId`: two job failure routes, four list remediation routes, source quality, and three suppression import/export/event routes. The global OpenAPI security declaration makes public health, onboarding, and provider callback routes look like ordinary key-authenticated calls. Scope, error, pagination, upload, and binary response details need a per-operation audit.
- The browser BFF permits 10 MB request bodies, while Rust accepts list multipart uploads up to 50 MB. The capability manifest says uploads are disabled despite the list upload UI. Resolve the actual supported limit and contract before promoting bulk/list workflows.
- Backend admin routes use `x-reacher-secret`; the current `check_header` becomes permissive when that secret is absent. The public `/v1` route strips the header, but the backend itself must fail closed and remain private before an operator console can use these routes.
- The app's public router has `/v1/*`, but no public `/v0/*` compatibility route. Hosted v0 support cannot be implied by backend route existence; keep it marked legacy/self-host-only unless an isolated, tenant-safe compatibility path is built and tested.
- PR #51 merged the one-image app/API route. [Release run 35825020873](https://github.com/Oppulence-Engineering/paperless-check-if-email-exists/actions/runs/35825020873) built, smoke-tested, and published `ghcr.io/oppulence-engineering/check-if-email-exists:sha-a013bfb8f6709777848d5e1fe91c63c648bc86ea`. All three staging jobs then failed at `Connect to Tailscale` (`sudo failed with exit code 1`); deployment and live existing-host API access remain unverified.

## Operation inventory and destination

Counts below are method/path counts from `backend/openapi.json`, not page counts. A slash suffix in a row is relative to the prefix at its start. The detailed reference must include each method, request/response schema, status codes, scope, rate/quota behavior, curl and TypeScript examples, and a testable host URL.

| Family | Operations | Exact route coverage | Portal destination |
| --- | ---: | --- | --- |
| System | 4 | `GET /healthz`, `/readyz`, `/version`, `/openapi.json` | Public status and reference metadata; keep health out of the tenant console. |
| Legacy v0 | 4 | `POST /v0/check_email`, `/v0/bulk`; `GET /v0/bulk/{job_id}`, `/v0/bulk/{job_id}/results` | Mark self-host legacy, provide migration reference and examples, and state that the hosted shared URL supports v1. Do not proxy the legacy global-secret route into tenant traffic. |
| Onboarding | 1 | `POST /v1/check-email-with-onboard` | Explain the server-side signup contract; app sign-up remains the browser entry point. Review abuse controls before external publication. |
| Verification and finder | 6 | `POST /v1/check_email`, `/v1/find_email`, `/v1/reputation/check`; `GET /v1/emails/{email}/history`, `/v1/find_email/{job_id}`, `/v1/reverification/status` | Check playground, finder with asynchronous result, reputation explanation, history and freshness view. Check and history exist in basic form. |
| Bulk and observability | 16 | `POST /v1/bulk`; `GET /v1/bulk/{job_id}`, `/v1/bulk/{job_id}/results`; `GET /v1/jobs/{job_id}` plus `/approval`, `/download`, `/events`, `/failure-center`, `/failure-report`, `/latency`, `/results`; `POST /v1/jobs/{job_id}/cancel`, `/retry`; `GET /v1/events`, `/v1/query`, `/v1/sources/quality` | Bulk creation, job index/detail, progress, failure center, approval, safe retry/cancel, exports, event/query explorer, source quality. |
| Lists and collaboration | 13 | `GET/POST /v1/lists`; `GET/DELETE /v1/lists/{list_id}`; `GET .../download`, `.../quality`, `.../remediation-plan`, `.../remediation-exports/{export_id}/download`; `POST .../remediation-plan`, `.../remediation-exports`; `GET/POST /v1/comments`; `DELETE /v1/comments/{comment_id}` | List detail with original/cleaned quality, remediation plan/export, downloads, comments, and deletion. List/create exist in basic form. |
| Suppressions | 7 | `GET/POST /v1/suppressions`; `GET /v1/suppressions/check`, `/export`, `/{id}/events`; `POST /v1/suppressions/import`; `DELETE /v1/suppressions/{id}` | Suppression search, add/import/export, decision trace, and removal. |
| Feedback and providers | 6 | `GET/POST /v1/outcomes`, `/v1/provider-endpoints`; `PATCH/DELETE /v1/provider-endpoints/{endpoint_id}` | Outcome dashboard and ingest examples; provider endpoint registration, test instructions, update, and removal. |
| Provider callback | 1 | `POST /v1/inbound/providers/{provider}/{endpoint_id}/{delivery_token}` | Server-to-server setup guide covering provider signature, token rotation, retries, and test delivery; never a browser playground action. |
| Pipelines | 11 | `GET/POST /v1/pipelines`; `GET/PATCH/DELETE /v1/pipelines/{pipeline_id}`; `POST .../pause`, `/push`, `/resume`, `/trigger`; `GET .../runs`, `.../runs/{run_id}` | Pipeline builder, schedule/source/delivery configuration, push/trigger, run history, pause/resume, and deletion. |
| Workspace account | 17 | `GET /v1/me`; `GET/POST /v1/me/api-keys`; `GET/PATCH/DELETE /v1/me/api-keys/{key_id}`; `GET/POST /v1/me/domains`; `GET/PATCH/DELETE /v1/me/domains/{domain}`; `GET/PATCH /v1/me/settings`; `GET /v1/me/usage`; `GET/PATCH/DELETE /v1/me/webhook` | Workspace overview, full key lifecycle, domains, verification policy, usage/quota view, webhook configuration and delivery guidance. Nine operations have partial UI. |
| Platform admin | 20 | `GET /v1/admin/api-keys`; `GET /v1/admin/jobs`, `/jobs/{job_id}`, `/jobs/{job_id}/events`, `/jobs/{job_id}/results`; `GET/POST /v1/admin/tenants`; `GET/PUT/DELETE /v1/admin/tenants/{tenant_id}`; `GET/POST .../api-keys`; `GET/PATCH/DELETE .../api-keys/{key_id}`; `POST .../api-keys/{key_id}/reactivate`; `GET .../jobs`; `GET/PATCH .../quota`; `POST .../quota/reset` | Separate operator console with explicit platform authorization, audit, confirmation for mutations, and scoped tenant context. Never expose these through a tenant key or generic API playground. |
| **Total** | **106** | | |

## Product structure

1. **Public developer area:** `/developers` quickstart; authenticated vs API-key request paths; API reference generated from the same OpenAPI artifact as the SDK; guides for checks, bulk jobs, lists, pipelines, suppressions, outcomes, callbacks, errors, retries/idempotency, pagination, quotas, versioning, and migration from v0. Add copyable curl and TypeScript examples against the existing public host. Reuse the installed Fumadocs/MDX tooling.
2. **Workspace console:** add Overview, API keys, Playground, Verify/Finder, Bulk/Jobs, Lists, Pipelines, Suppressions, Outcomes/Providers, Analytics, and Settings to the product navigation. Keep results first and advanced controls in focused sheets. A job or list link must lead to its detail, failure/remediation, and download actions instead of ending at an ID.
3. **Platform admin:** keep the existing allowlist/audit model, but make Rust control-plane operations available only through a dedicated server-side admin adapter with explicit platform credentials and per-action audit. Require a configured secret or stronger operator credential at the Rust boundary and deny `/v1/admin/*` at the public key route. Tenant membership alone must never grant cross-tenant access. Show destructive actions with a confirmation and reason.
4. **Integration endpoints:** document onboarding, provider callbacks, health/status, and v0 without putting secrets or callback tokens into client state. Public `/v1/*` calls use workspace API keys; signed-in console traffic stays on `/api/backend/*` with server-minted JWTs. Preserve the same-host route and forward body, query, content type, downloads, `Range`, request IDs, and accepted idempotency headers.

## Delivery sequence

### 0. Make the contract dependable

- Fix the ten missing operation IDs and regenerate the TypeScript SDK and Zod contracts. Correct per-operation security, required scopes, body/media types, binary downloads, callback auth, and documented error envelopes. Keep the route registry/OpenAPI parity test and add a portal classification check for all 106 operations.
- Make backend admin auth fail closed when unconfigured. Keep `/v1/admin/*` out of the public key proxy and generic browser BFF, then add a separate operator path only after its authorization and audit tests pass. Record hosted v0 as legacy/self-host-only in the reference.
- Resolve the 10 MB BFF vs 50 MB backend list-upload mismatch and capability manifest. Verify upload, CSV/JSONL download, query strings, and safe error propagation through the one-image route.
- Establish the public developer docs route, source OpenAPI link, and example generator; do not hand-edit generated client code. Repair the staging Tailscale connection, confirm the existing HTTPS host and required auth/ingress settings, and make an external key-authenticated request before claiming live API access.

### 1. Deliver first API success

- Build the developer quickstart and reference, full API-key get/update/revoke lifecycle, scope picker that clearly distinguishes tenant key-management scope from platform administration, one-time key reveal, and an authenticated playground that uses the session BFF. Demonstrate a key-based `POST /v1/check_email` request from outside the browser on the existing host.
- Finish Verify, history, finder, reputation, and reverification views. Show reason codes, confidence, policy mode, freshness, and the raw structured result with a matching code snippet.

### 2. Make asynchronous work usable

- Build bulk creation and a Jobs index/detail around status, events, results, approvals, failure center/report, latency, retry/cancel, and downloads. Link a list upload to its job. Add safe polling, bounded pagination, and clear queued/running/failed/completed states.
- Complete list detail, quality, remediation plans and exports, comments, delete, and cleaned CSV download. Surface query/events and source quality as inspectable analytics with filters and empty states.

### 3. Expose automation and feedback

- Build suppression search, check, add/import/export, event history, and delete; pipeline create/edit/run/push/pause/resume/delete and run history; outcomes and provider endpoint setup; and workspace domains, usage, policy, and webhook settings.
- Provide an end-to-end provider callback guide and a signed test delivery path. Separate provider-ingest credentials and tokens from ordinary API-key examples.

### 4. Finish the control plane and rollout

- Implement the 20 backend admin operations in the restricted operator console and retain the Better Auth identity view as a clearly labeled separate data source. Add authorization and audit checks for each read and mutation.
- Publish v0 migration and system/status pages. Verify every operation has the correct reference audience and that no tenant-facing route is stranded behind a raw JSON link.
- Run staging dogfood on the existing authenticated browser session and a separate API-key client: key creation, check, bulk/list job to result and download, pipeline trigger, callback, suppression, tenant switch isolation, forbidden scope, and admin denial. Repeat a production smoke after promotion.

## Gates for each slice

- Route registry = OpenAPI method/path set = classified portal inventory. Each public tenant operation has an operation ID, SDK binding, accurate auth/scope, reference example, and a reachable workflow or explicit integration-only classification.
- Every new screen has a focused query/mutation check and a browser journey through Better Auth, same-origin BFF, Rust API, and tenant data. Assert cross-tenant denial and limited-key `403` for representative read/write paths; never accept browser-supplied tenant or signing credentials.
- Check response and download media types through both the browser BFF and direct key API. Test multipart limits, retry/idempotency behavior as implemented, invalid input, empty states, and callback signature failures.
- Keep `pnpm verify:fast`, `pnpm arch`, `pnpm contracts:check`, `pnpm test`, Rust route/API harness, combined-image smoke, and PR CI green. Confirm GHCR image publication, staging deployment, live HTTPS app/API, and an external key-authenticated request before marking the portal shipped.

This plan covers current backend capabilities. The separate customer feature backlog contains proposed APIs and third-party integrations; those become portal work when they exist in the backend contract.
