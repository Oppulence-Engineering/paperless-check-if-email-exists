# RFC 0001: Native Provider Webhooks and Outcome Adapters

## Status

Draft

## Summary

Add tenant-scoped inbound webhook adapters for SendGrid, Amazon SES, Mailgun, and Postmark. Reacher will remain a backend/API engine, not a sending platform: it will ingest downstream delivery events, normalize them into a stable outcome model, correlate them to known jobs/lists/emails when possible, and persist them for later analytics and suppression decisions.

## Motivation / Problem

- Reacher currently supports outbound tenant webhook configuration, but it does not accept inbound provider delivery events.
- Customers who verify in Reacher and send elsewhere must build their own normalization, signature verification, and deduplication pipeline.
- Later analytics RFCs depend on a normalized outcome substrate instead of one-off provider payloads.

## Goals

- Accept first-wave ESP delivery events without requiring customer-built forwarding middleware.
- Normalize provider payloads into a stable event taxonomy.
- Guarantee tenant isolation, signature validation, and idempotent ingest.
- Preserve raw receipt metadata for audit and debugging.

## Non-Goals

- Sending email through Reacher.
- CRM or sales-engagement adapters in the first wave.
- A no-code setup UI; configuration is API-first.
- Real-time campaign analytics dashboards in the same feature.

## Current State In Reacher

- Tenant settings already expose outbound webhook configuration through `/v1/me/webhook`.
- Verification, bulk jobs, lists, suppressions, email history, and pipelines already exist.
- There is no normalized delivery outcome table, inbound webhook receiver, or provider adapter configuration object.

## Proposed Design

Reacher will add a tenant-scoped `provider endpoint` resource and a single inbound delivery surface:

- Management APIs create an endpoint record with:
  - `endpoint_id` as an opaque ULID
  - `delivery_token` as a generated secret included in the webhook URL
  - `provider` as `sendgrid`, `ses`, `mailgun`, or `postmark`
  - `label`, `status`, `provider_config`, and optional `allowed_ips`
- Incoming deliveries use:
  - `POST /v1/inbound/providers/{provider}/{endpoint_id}/{delivery_token}`
- Every accepted request produces a `receipt` row and zero or more normalized `delivery outcome` rows.

### Resource Model

Each tenant-owned provider endpoint is a long-lived configuration object with:

- one provider
- one generated delivery URL
- one provider-specific config document
- one lifecycle status

Statuses are:

- `active`
- `paused`
- `disabled`

Behavior by status is fixed:

- `active`: accept and process inbound deliveries
- `paused`: return `202` and persist a rejected receipt with `pause_reason = endpoint_paused`, but do not normalize outcomes
- `disabled`: return `404` and do not leak endpoint state

`provider_config` is provider-specific but versioned through a shared envelope:

```json
{
  "version": 1,
  "settings": {}
}
```

Provider-specific `settings` in v1 are:

- SendGrid
  - `verification_timestamp_tolerance_seconds`
- SES
  - `topic_arns`
  - `require_subscription_confirmation`
- Mailgun
  - `signing_key_id`
- Postmark
  - `allowlisted_source_ips`

### Ingest Pipeline

Inbound processing follows this fixed sequence:

1. Resolve `provider`, `endpoint_id`, and `delivery_token`.
2. Load the endpoint and verify that it belongs to an active tenant.
3. Validate provider-specific authenticity checks.
4. Persist a `receipt` row before normalization.
5. Parse provider payloads into one or more provider-native events.
6. Normalize each provider-native event into the shared event taxonomy.
7. Compute per-event idempotency keys.
8. Correlate each event to known Reacher entities.
9. Persist normalized rows.
10. Return an acknowledgment payload that includes accepted, duplicate, rejected, and unmatched counts.

Receipt persistence happens before normalization so support/debugging can inspect malformed or partially failed batches without losing the raw payload.

### Provider Adapter Behavior

First-wave provider verification rules are:

- SendGrid: require provider signature verification and timestamp freshness.
- Amazon SES: require valid SNS signature verification and a configured `TopicArn` allowlist.
- Mailgun: require timestamp/token/signature verification.
- Postmark: require the generated `delivery_token`; provider-native signing is not required in the first wave. Optional IP allowlisting may be configured per endpoint.

Provider adapter responsibilities are explicitly limited to:

- authenticity verification
- payload shape parsing
- provider-event to normalized-event translation
- extraction of provider-native identifiers and message metadata

Provider adapters do not:

- mutate tenant settings
- trigger suppressions directly
- emit customer-facing webhooks in v1

### Normalized Outcome Schema

Normalized event types are:

- `processed`
- `delivered`
- `deferred`
- `bounce_soft`
- `bounce_hard`
- `complaint`
- `unsubscribe`
- `open`
- `click`
- `dropped`

Normalized rows also classify event families:

- `delivery`
- `engagement`
- `negative_feedback`
- `routing`

That family is derived at ingest time and stored explicitly so later analytics queries do not need provider-specific remapping logic.

Correlation rules are deterministic and ordered:

1. Use `metadata.reacher_job_id` when present and valid.
2. Else use `metadata.reacher_list_id` when present and valid.
3. Else use `metadata.reacher_source_key` when present.
4. Else match by tenant plus canonical recipient email to the most recent verification result in the previous 30 days.
5. If no correlation is found, persist the event as `unmatched` rather than rejecting it.

If more than one candidate verification row qualifies in step 4, Reacher chooses the newest row by `completed_at DESC`, then ties by `id DESC`.

Idempotency is enforced by `(tenant_id, provider, provider_event_id)`. If a provider does not supply a stable event identifier, Reacher will compute a fallback digest from `provider`, `event_type`, `occurred_at`, `message_id`, and `recipient_email`.

### Retry Semantics

- `2xx`: provider should treat the request as accepted
- `4xx`: provider should not retry except for its own platform-specific retry policy
- `5xx`: provider should retry

Reacher returns `200` for accepted duplicates because duplicate handling is part of the normal idempotent contract, not an error.

## Public API / Interface Changes

Add tenant management endpoints:

- `GET /v1/provider-endpoints`
- `POST /v1/provider-endpoints`
- `PATCH /v1/provider-endpoints/{endpoint_id}`
- `DELETE /v1/provider-endpoints/{endpoint_id}`

Add inbound receiver:

- `POST /v1/inbound/providers/{provider}/{endpoint_id}/{delivery_token}`

Create endpoint request example:

```json
{
  "provider": "sendgrid",
  "label": "primary-sendgrid-prod",
  "status": "active",
  "provider_config": {
    "version": 1,
    "settings": {
      "verification_timestamp_tolerance_seconds": 300
    }
  },
  "allowed_ips": []
}
```

Create endpoint response example:

```json
{
  "endpoint_id": "01JQWEBHOOK3Y8CK4THATV7",
  "provider": "sendgrid",
  "label": "primary-sendgrid-prod",
  "status": "active",
  "webhook_url": "https://api.reacher.example/v1/inbound/providers/sendgrid/01JQWEBHOOK3Y8CK4THATV7/rt_9X7...",
  "created_at": "2026-03-30T00:00:00Z"
}
```

Inbound acknowledgment example:

```json
{
  "receipt_id": "01JQWEBHOOKRCPTA1WQF8Q",
  "provider": "sendgrid",
  "accepted": 42,
  "duplicates": 3,
  "unmatched": 5,
  "rejected": 0
}
```

`POST /v1/provider-endpoints` request fields:

- `provider`
- `label`
- `provider_config`
- `allowed_ips`
- `status`

Response fields include:

- `endpoint_id`
- `provider`
- `label`
- `status`
- `webhook_url`
- `created_at`

Inbound receiver responses are intentionally compact; normalized outcome rows are not returned inline to avoid leaking correlation detail to the provider caller.

Normalized stored outcome fields include:

- `tenant_id`
- `provider`
- `provider_event_id`
- `endpoint_id`
- `event_type`
- `occurred_at`
- `recipient_email`
- `canonical_email`
- `message_id`
- `job_id`
- `list_id`
- `source_key`
- `correlation_status`
- `metadata`

## Data Model / Storage Changes

Add tables:

- `provider_endpoints`
- `delivery_outcome_receipts`
- `delivery_outcomes`

Concrete `provider_endpoints` columns:

- `id`
- `tenant_id`
- `provider`
- `label`
- `status`
- `delivery_token_hash`
- `provider_config`
- `allowed_ips`
- `created_at`
- `updated_at`

Concrete `delivery_outcome_receipts` columns:

- `id`
- `tenant_id`
- `endpoint_id`
- `provider`
- `request_headers`
- `raw_payload`
- `payload_sha256`
- `received_at`
- `validation_status`
- `validation_error`
- `normalized_count`

Concrete `delivery_outcomes` columns:

- `id`
- `tenant_id`
- `endpoint_id`
- `provider`
- `provider_event_id`
- `provider_message_id`
- `event_type`
- `event_family`
- `occurred_at`
- `recipient_email`
- `canonical_email`
- `job_id`
- `list_id`
- `source_key`
- `correlation_status`
- `metadata`
- `created_at`

Key storage decisions:

- `provider_endpoints` stores provider config, delivery token, status, and tenant ownership.
- `delivery_outcome_receipts` stores raw payload digest, headers, validation result, parse errors, and receipt timestamps.
- `delivery_outcomes` stores normalized per-recipient events and correlation results.
- `delivery_outcomes` keeps a unique index on `(tenant_id, provider, provider_event_id)`.
- `delivery_outcomes` also keeps:
  - `(tenant_id, canonical_email, occurred_at DESC)`
  - `(tenant_id, job_id, occurred_at DESC)`
  - `(tenant_id, source_key, occurred_at DESC)`

## Auth, Permissions, And Tenant Isolation

- Management endpoints require tenant authentication and the existing `settings` scope.
- Inbound receiver endpoints do not use bearer auth. They are tenant-routed by `endpoint_id` and protected by `delivery_token`, plus provider validation where available.
- Legacy or open-mode callers do not receive implicit access to provider endpoint management.
- No inbound request may write across tenants, even if the payload contains foreign `job_id` or `list_id` metadata.

Tenant management scope matrix:

- `GET /v1/provider-endpoints`: `settings`
- `POST /v1/provider-endpoints`: `settings`
- `PATCH /v1/provider-endpoints/{endpoint_id}`: `settings`
- `DELETE /v1/provider-endpoints/{endpoint_id}`: `settings`

Admin APIs are out of scope for v1. If admins later need cross-tenant visibility, that should be implemented as separate `/v1/admin/provider-endpoints` routes, not as a relaxation of tenant routes.

## Failure Modes And Edge Cases

- Unknown `endpoint_id` or bad `delivery_token` returns `404` to avoid leaking endpoint existence.
- Signature validation failures return `401`.
- Valid sender but malformed payload returns `400` and records a failed receipt.
- Internal persistence failures return `500` so providers retry.
- Duplicate events return `200` with a duplicate acknowledgment and do not create a second outcome row.
- Multi-recipient provider batches are split into one normalized outcome per recipient.

Additional edge rules:

- A valid receipt with one bad event in a multi-event batch stores the bad event as rejected and continues processing the remaining events.
- If correlation metadata points to a deleted job or list, the row is stored as `unmatched` with `correlation_error = stale_reference`.
- If the same provider event is replayed against the wrong endpoint for the same tenant, the request is treated as a duplicate only when the normalized provider event key matches exactly.

## Alternatives Considered

- A generic `POST /v1/outcomes` endpoint only. Rejected because it pushes provider verification and normalization back onto customers.
- One global webhook URL with tenant selection in payload metadata. Rejected because tenant routing would be weaker and easier to misconfigure.
- Waiting for CRM adapters first. Rejected because ESP delivery events are the minimum substrate for later analytics features.

## Rollout / Migration / Compatibility

- Roll out as additive v1 APIs and new tables.
- Ship SendGrid, SES, Mailgun, and Postmark together behind provider-specific tests.
- Keep unmatched outcomes queryable for support/debugging, but do not feed them into later aggregate metrics by default.
- No existing verification or bulk API response changes are required in the first release.

Implementation phases:

1. Storage and endpoint management APIs
2. SendGrid and Mailgun adapters
3. SES and Postmark adapters
4. Correlation and queryability
5. Analytics consumers in later RFCs

## Metrics And Success Criteria

- Endpoint creation-to-first-event success rate.
- Webhook receipt acceptance rate per provider.
- Duplicate event discard rate.
- Correlation success rate to known jobs/lists/results.
- p95 ingest latency.

## Test Strategy

- Signature verification fixtures for each provider.
- Duplicate delivery replay tests.
- Tenant isolation tests with conflicting `job_id` and `list_id` values.
- Malformed payload tests and partial-batch parse tests.
- Correlation tests for explicit metadata, email fallback, and unmatched storage.

The minimum automated matrix is:

- unit tests for provider-specific signature validation
- normalization golden tests per provider event type
- integration tests for inbound endpoint routing and receipt persistence
- migration tests for new tables and indexes
- API harness coverage for provider-endpoint CRUD routes

## Unresolved Risks

- Provider payloads can drift over time and require upkeep.
- Postmark’s first-wave security posture is weaker than signed-provider adapters because it relies on the generated delivery token.
