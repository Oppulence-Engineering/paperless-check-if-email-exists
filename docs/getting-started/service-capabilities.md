# Service Capabilities

Reacher is best understood here as a backend/API engine for email verification and list hygiene workflows. This repository contains the multi-tenant API, worker system, persistence, and export paths. The hosted customer dashboard is a separate product surface and is not part of this codebase.

## Supported Core Workflows

The current supported backend story is centered on eight workflows:

- single verification through the check-email endpoints
- bulk jobs and CSV list cleaning
- suppression management
- scheduled re-verification and pipelines
- decision policies and explainable recommendations
- automatic list remediation and partitioned exports
- source quality analytics
- outcome ingestion from ESP, CRM, or campaign systems

The platform support around those workflows is also part of the supported core:

- tenant auth and isolation
- API keys and quota enforcement
- job approval and approval-checklist responses
- job failure-center and failure-report responses
- historical email lookup and job result retrieval
- webhooks, exports, and worker execution

## Single Verification

Reacher can verify a single email address and return a structured result with:

- `is_reachable`: `safe`, `risky`, `invalid`, or `unknown`
- syntax validation
- MX record validation
- SMTP connectivity checks
- deliverability detection
- mailbox disabled detection
- catch-all detection
- full inbox detection
- disposable address detection
- role account detection
- optional enrichment such as Gravatar and Have I Been Pwned data

Every verification response also includes a score object with:

- a `0-100` deliverability score
- a category: `valid`, `risky`, `unknown`, or `invalid`
- a `sub_reason` explaining the dominant result
- scoring signals that summarize why the score was assigned

New verification responses can also include an actionable `recommendation` and `policy_evaluation`. Recommendations use actions such as `send`, `send_with_caution`, `review`, `suppress`, `drop`, and `fix_then_send`, with reason codes that explain the decision.

With Postgres, single checks have durable operation identities. Send an `Idempotency-Key` when creating a check. The endpoint waits up to 20 seconds for completion, then returns HTTP `202` with `operation_id`, `status: "pending"`, and `status_url` if work is still pending. Poll `GET /v1/operations/{operation_id}` using the same tenant credentials: it returns `202` while pending, `200` with the saved verification, or `500` with the saved terminal failure. Polling and replaying the original key do not create another check or charge.

Pending work survives a disconnected request or a restarted process. Idempotency records remain available while work is pending and for at least 24 hours after completion. Worker execution and idempotency require Postgres; configurations without it return `503` before dispatch. The web app polls pending checks automatically. Generated Go and TypeScript SDKs return `VerificationResponse`, which represents either `CheckEmailOutput` or `PendingVerification`; inspect the response status before using the result.

## Bulk Jobs And List Cleaning

Reacher supports asynchronous bulk verification for large email sets.

Bulk workflows provide:

- job creation and lifecycle tracking
- task-level result persistence
- paginated job result retrieval
- streaming downloads for completed jobs
- CSV output for spreadsheet workflows. Cells beginning with formula prefixes (`=`, `+`, `-`, `@`), including after whitespace/control characters, or leading tab/CR/LF are prefixed with an apostrophe. This applies to original columns and headers, errors, suppression exports, and remediation exports. Use JSON or NDJSON for exact machine-readable field values.
- NDJSON output for system-to-system processing
- source attribution for source-quality reporting
- job failure-center summaries and downloadable failure reports

List-cleaning capabilities include:

- multipart CSV upload
- automatic email column detection
- preservation of original row data and column order
- tenant-aware row limits
- asynchronous verification of each row
- cleaned CSV download with original columns preserved
- appended verification fields such as `is_reachable`, `score`, `category`, and `error`
- filtered exports, such as downloading only valid rows
- remediation plans that classify rows as `fixed`, `safe`, `review`, or `drop`
- remediation exports for safe-to-send, changed, review, drop, and all-row partitions

## Suppressions, Reverification, And Pipelines

Reacher includes backend primitives for keeping lists healthy after the first verification pass.

These capabilities include:

- tenant-scoped suppression add, import, check, list, export, event-history, and revoke endpoints
- suppression provenance fields such as reason detail, source type, source reference, creator, expiry, confidence metadata, and audit events
- scheduled re-verification status and configuration
- pipeline creation, triggering, pause/resume, and run history
- webhook-oriented automation for downstream delivery

## Source Quality And Outcomes

Reacher can attach source keys to list uploads and bulk jobs, then report quality by source through `/v1/sources/quality`.

Source quality reporting includes:

- valid, risky, unknown, and invalid counts
- safe-to-send counts
- recommendation-action distribution
- delivered, opened, clicked, bounced, complained, and unsubscribed outcomes
- quality grades and customer-readable summaries

The `/v1/outcomes` endpoint accepts delivered, opened, clicked, bounced, complained, and unsubscribed events. Bounced, complained, and unsubscribed outcomes automatically refresh suppression intelligence.

## Deployment And Consumption Modes

Reacher can be consumed in multiple ways:

- as an HTTP API
- as a self-hosted backend
- as a Rust library
- as generated SDKs for external integrations

That makes it useful for product teams building verification directly into apps, CRMs, enrichment pipelines, and internal operations tooling.

## Adjacent And Experimental Surfaces

Some endpoints are useful but are not the main product story for this repository right now.

- Email finder and domain reputation checks are adjacent capabilities, not the current list-hygiene core.
- Advanced query endpoints are documented as experimental for large reporting workloads.
- Comment endpoints are documented as experimental collaboration helpers.

## In Short

Reacher is not just an email checker binary. In this repository, it is a tenant-aware backend/API engine for verification, list cleaning, decision policies, remediation, suppressions, source quality, outcome feedback, re-verification, and pipelines.

## Reading domain reputation evidence

Reputation results report what each check established. A DKIM key found at a common selector does not verify a message signature. `has_dkim: null` means the check could not establish a usable key; another selector may contain one. `dkim_selectors` reports each attempted selector separately, including missing, revoked, invalid, and failed lookups.

SPF record presence is reported by `has_spf`. The former `spf_valid` field is replaced by `spf_policy_explicit`, which indicates whether one SPF record contains a terminal `all` or a nonempty redirect. This field does not validate SPF syntax or authorize a sender. Use an SPF evaluator for those decisions.

Each blocklist result includes `status`, `answers`, and an optional `error`. `listed: false` means a completed negative lookup. A failed or skipped query has `listed: null`; it does not establish a clean result. A score reflects known evidence, and an otherwise low-risk result with incomplete blocklist coverage is labeled `unknown`.

## Mailbox identity

The service preserves the supplied address for verification. Domains are compared without ASCII case, while local-part case, dots, and tags remain significant. Consumer Gmail and Googlemail are the supported exception: their case, dot, and plus aliases share one deduplication and suppression identity. This rule does not extend to Google Workspace domains.
