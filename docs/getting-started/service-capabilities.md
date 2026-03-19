# Service Capabilities

Reacher is an email verification platform with both synchronous and asynchronous workflows. It can validate individual addresses, process large jobs, preserve tenant isolation, and expose results through API responses, downloads, and webhooks.

## Core Email Verification

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

## Bulk Verification

Reacher supports asynchronous bulk verification for large email sets.

Bulk workflows provide:

- job creation and lifecycle tracking
- task-level result persistence
- paginated job result retrieval
- streaming downloads for completed jobs
- CSV output for spreadsheet workflows
- NDJSON output for system-to-system processing

## Email Finder

Reacher can find likely email addresses for a person at a domain.

The finder workflow:

- generates common email address patterns from a first name, last name, and domain
- checks domain readiness before spending credits
- verifies candidates concurrently
- scores and ranks each candidate
- returns the best match when confidence is strong enough

This makes the service useful for lead enrichment and contact discovery use cases, not only deliverability screening.

## Email List Cleaning

Reacher supports CSV-based list cleaning.

List cleaning capabilities include:

- multipart CSV upload
- automatic email column detection
- preservation of original row data and column order
- tenant-aware row limits
- asynchronous verification of each row
- cleaned CSV download with original columns preserved
- appended verification fields such as `is_reachable`, `score`, `category`, and `error`
- filtered exports, such as downloading only valid rows

This makes Reacher suitable for cleaning CRM exports, marketing lists, and imported prospect databases.

## Domain Reputation Checks

Reacher can evaluate the sending reputation of a domain.

Domain reputation checks include:

- DNSBL lookups
- SPF detection
- DKIM detection
- DMARC detection and policy parsing
- MX record validation
- domain age lookups
- an aggregated reputation score and risk level

Results are cached so repeated checks are faster and cheaper.

## Tenant Platform Features

The service is built as a multi-tenant platform, not just a verification endpoint.

Platform capabilities include:

- tenant onboarding and CRUD
- API key authentication
- legacy header-secret compatibility where needed
- quota enforcement
- idempotency support
- tenant-scoped jobs and resources
- admin endpoints for operators

## Eventing and Automation

Reacher can integrate into larger systems through:

- asynchronous worker execution
- RabbitMQ-backed job processing
- job and task lifecycle events
- outbound webhooks for result delivery

This allows Reacher to serve both request-response applications and queued background workflows.

## Deployment and Consumption Modes

Reacher can be consumed in multiple ways:

- as an HTTP API
- as a self-hosted backend
- as a Rust library
- as generated SDKs for external integrations

That makes it useful for product teams building verification directly into apps, data pipelines, CRMs, and back-office tools.

## Best-Fit Use Cases

Reacher is well suited for:

- signup form validation
- bounce prevention before send
- CRM and prospect list cleaning
- lead enrichment and email finding
- tenant-based SaaS verification platforms
- compliance and deliverability monitoring
- domain-level reputation analysis

## In Short

Reacher is not only an email checker. It is a tenant-aware email verification and list processing platform with scoring, finder workflows, bulk processing, streaming exports, and domain reputation analysis.
