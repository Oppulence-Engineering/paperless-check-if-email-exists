# RFC 0013: Webhook Event Subscriptions And Delivery Log

## Verify disposition — 2026-10-23

**Conditional; credential slice at stage 3.** RFC 0006 may reuse the tenant secret cipher without waiting for the entire event subscription/outbox product. Full audit/research events and delivery logs follow integration demand. Add audit.completed, repair.completed, and export.ready only with the corresponding durable resources and authorization; do not imply those events already exist.

[Current build order](README.md) · [Codebase audit](../audits/2026-10-23-verify-north-star.md). This disposition supersedes conflicting historical scope, priority, or baseline statements below; it does not certify every original acceptance criterion.

| | |
|---|---|
| **Status** | Conditional; credential slice at stage 3; audited 2026-10-23 |
| **Created** | 2026-10-21 |
| **Owners** | To assign: backend lead, web lead |
| **Parent** | RFC 0033, item 7 (backlog items 65, 68, and 87) |
| **Depends on** | None |
| **Used by** | RFC 0019 and RFC 0012 (`alert.raised`), RFC 0024 and RFC 0025 (the secret cipher) |
| **Release gates** | Security review of signing and SSRF rules; Definition Of Done met |

## Summary

This RFC lets a tenant register more than one webhook endpoint and choose the event types for each one. The backend writes each event to a durable outbox. A dispatcher delivers it with retries for up to about 45 hours. Each attempt is logged. A tenant can retry a delivery by hand, disable an endpoint, and rotate its secret with no downtime. This RFC also adds a small cipher for tenant secrets, which later RFCs reuse.

## Motivation / Problem

- Each tenant has one URL (`tenants.default_webhook_url`) and one secret (`tenants.webhook_signing_secret`). A tenant cannot send job events to one system and alerts to another.
- Per-email webhooks retry 3 times in memory, with 1-second and 2-second waits (`WEBHOOK_MAX_ATTEMPTS` in `worker/do_work.rs`). A restart loses the retries. A receiver that is down for 10 seconds loses the event.
- A failed delivery leaves only a `task.webhook_failed` job event, and only for bulk jobs. The tenant cannot see the request, the response, or retry it.
- The signature covers only the body. A captured request can be sent again later, and the receiver cannot tell.
- The secret is stored in plain text.
- Many backend events have no webhook: job completion, list completion, mailbox status changes, suppressions, and alerts.

## Goals

- Many endpoints for each tenant, each with its own event types and secret.
- Durable delivery that survives restarts, with a fixed retry schedule.
- A log of every attempt, with the status code, the duration, and a short part of the response.
- Manual retry, disable, and test delivery.
- A signature with a timestamp, and secret rotation with two active secrets.
- Versioned payloads.
- Secrets encrypted at rest.

## Non-Goals

- Moving the per-email webhook (`webhook.on_each_email` on bulk requests) to the outbox in v1. It keeps its current behavior. Phase 2 moves it.
- Event streaming (for example a queue or a server-sent events feed).
- Webhooks for each outcome event. The volume is too high. Tenants read outcomes with `GET /v1/outcomes`.

## Current State In Reacher

- `tenant/webhook.rs` signs a body with HMAC-SHA256 and sends `X-Reacher-Signature-256: sha256=<hex>`.
- `pipelines.rs` delivers pipeline webhooks to the pipeline's `delivery_config` URL or to `default_webhook_url`. It checks targets with `validate_webhook_url` and `validate_resolved_webhook_target`, and it pins the resolved IP in `pinned_webhook_client`. This blocks private IP targets and DNS rebinding.
- Provider outcome forwarding keeps `forward_status`, `forward_attempts`, and `forward_error` on each receipt.
- The verification outbox (`worker/dispatch.rs`, `spawn_publisher`) shows the durable outbox pattern: write a row in the same transaction, then a background loop publishes it.
- `job_events` holds job and task events. `GET /v1/events` reads them.
- `worker/job_lifecycle.rs` moves jobs and lists between states.

## Proposed Design

### Event Envelope

```json
{
  "id": "evt_01JB8Z4W6Q6Y3M2T9K7R5N1C0D",
  "type": "job.completed",
  "version": 1,
  "created_at": "2026-10-21T10:15:00Z",
  "data": { "job_id": 4412, "total": 5000, "completed": 4990, "failed": 10 }
}
```

- `id` is unique. Receivers use it to ignore a repeat delivery.
- `version` is the payload version for the event type. A breaking change to `data` needs a new version. An endpoint chooses its version when it is created, and we send that version until the tenant changes it.

### Event Types (v1)

| Type | When | Source |
|---|---|---|
| `job.completed` | A bulk job finishes. | `worker/job_lifecycle.rs` |
| `job.failed` | A bulk job fails or is cancelled. | `worker/job_lifecycle.rs` |
| `list.completed` | A list finishes cleaning. | `worker/job_lifecycle.rs` |
| `pipeline_run.completed` | A pipeline run finishes. | `pipelines.rs` |
| `pipeline_run.failed` | A pipeline run fails. | `pipelines.rs` |
| `address.status_changed` | A re-check changes an address's category. | `verification_change_events` in `list_intelligence.rs` |
| `suppression.created` | An active suppression is created. | `v1_suppression_events` |
| `suppression.revoked` | A suppression is revoked. | `v1_suppression_events` |
| `alert.raised` | Any alert is written. | `v1_alerts` |
| `webhook.test` | The tenant sends a test. | The test endpoint |

Each source writes the event to the outbox in the same transaction as its own change. So an event exists only when the change exists.

### Endpoints And Fan-Out

- A tenant can have up to 10 endpoints.
- Each endpoint has a URL, a description, a list of event types (or `*` for all), a status, a payload version, and its secrets.
- For each event, the backend creates one delivery for each active endpoint that wants the type.

### Delivery

- A dispatcher loop claims due deliveries with `FOR UPDATE SKIP LOCKED`, like the pipeline scheduler. Many replicas can run it.
- Each request has a timeout of 10 seconds. A `2xx` response is a success. Redirects are not followed.
- Retry schedule after a failure: 1 minute, 5 minutes, 30 minutes, 2 hours, 6 hours, 12 hours, and 24 hours. After 8 attempts in total (about 45 hours), the delivery is `dead`.
- A `410 Gone` response marks the endpoint `disabled` at once.
- An endpoint can have no success for 72 hours and 1 or more dead deliveries. Then the system marks it `disabled` and writes an alert. The tenant turns it on again.
- Targets use the existing SSRF rules: HTTPS only in production, no private IPs, and a pinned resolved IP.

### Signature

Each request has these headers:

| Header | Value |
|---|---|
| `X-Reacher-Event-Id` | The event `id`. |
| `X-Reacher-Timestamp` | Unix seconds when we sent the request. |
| `X-Reacher-Signature` | `t=<timestamp>,v1=<hex HMAC-SHA256(secret, timestamp + "." + body)>`. During rotation, two `v1=` values. |
| `X-Reacher-Signature-256` | `sha256=<hex HMAC-SHA256(secret, body)>`, for compatibility with current receivers. |

Receivers check `X-Reacher-Signature` and reject a timestamp older than 5 minutes. The docs give code for TypeScript and Go.

### Secret Rotation

1. `POST /v1/webhook-endpoints/{id}/rotate-secret` makes a new secret and returns it once.
2. For 24 hours, we sign with both the new and the old secret.
3. After 24 hours, the old secret is deleted. The tenant can end the overlap earlier.

### Tenant Secret Cipher

- New module `backend/src/tenant/secrets.rs` with `seal(plaintext) -> SealedSecret` and `open(&SealedSecret) -> plaintext`.
- It uses AES-256-GCM from the `ring` crate, which the backend already depends on.
- The key comes from `RCH__SECRETS__ENCRYPTION_KEYS`, a list of `key_id:base64key` values. The first key seals. All keys can open. This allows key rotation.
- `SealedSecret` stores `key_id`, `nonce`, and `ciphertext`. The tenant ID is the associated data, so a row copied to another tenant does not open.
- RFC 0024 and RFC 0025 use this module for platform credentials and OAuth tokens.

### Legacy Settings

- `default_webhook_url` and `webhook_signing_secret` keep working for pipelines and per-email webhooks.
- A migration creates one endpoint from each tenant's `default_webhook_url`, with event types `job.completed`, `job.failed`, and `pipeline_run.completed`. It copies the secret into the cipher. New events then reach the same URL.
- `PATCH /v1/me/webhook` keeps working. It updates the migrated endpoint and the legacy columns together until phase 2.

## Public API / Interface Changes

| Method | Path | Purpose |
|---|---|---|
| `GET`, `POST` | `/v1/webhook-endpoints` | List and create endpoints. Create returns the secret once. |
| `GET`, `PATCH`, `DELETE` | `/v1/webhook-endpoints/{id}` | Read, change (URL, event types, status, version), and delete. |
| `POST` | `/v1/webhook-endpoints/{id}/rotate-secret` | Start a rotation. |
| `POST` | `/v1/webhook-endpoints/{id}/test` | Send a `webhook.test` event now. |
| `GET` | `/v1/webhook-deliveries` | List deliveries: `endpoint_id`, `status`, `event_type`, `since`, `limit`, `offset`. |
| `GET` | `/v1/webhook-deliveries/{id}` | One delivery with all its attempts. |
| `POST` | `/v1/webhook-deliveries/{id}/retry` | Retry a `failed` or `dead` delivery now. |
| `GET` | `/v1/webhook-event-types` | The event types, with a sample payload for each version. |

All endpoints need the `settings` scope.

## Data Model / Storage Changes

```sql
CREATE TABLE v1_webhook_endpoints (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    url TEXT NOT NULL,
    description TEXT,
    event_types TEXT[] NOT NULL,
    payload_version SMALLINT NOT NULL DEFAULT 1,
    status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'disabled')),
    disabled_reason TEXT,
    secret_sealed JSONB NOT NULL,
    previous_secret_sealed JSONB,
    previous_secret_expires_at TIMESTAMPTZ,
    last_success_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE v1_webhook_events (
    id TEXT PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    event_type TEXT NOT NULL,
    payload JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE v1_webhook_deliveries (
    id BIGSERIAL PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    endpoint_id UUID NOT NULL REFERENCES v1_webhook_endpoints(id) ON DELETE CASCADE,
    event_id TEXT NOT NULL REFERENCES v1_webhook_events(id) ON DELETE CASCADE,
    status TEXT NOT NULL DEFAULT 'pending'
        CHECK (status IN ('pending', 'succeeded', 'failed', 'dead')),
    attempt_count SMALLINT NOT NULL DEFAULT 0,
    next_attempt_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (endpoint_id, event_id)
);
CREATE INDEX idx_v1_webhook_deliveries_due
    ON v1_webhook_deliveries (next_attempt_at) WHERE status IN ('pending', 'failed');

CREATE TABLE v1_webhook_attempts (
    id BIGSERIAL PRIMARY KEY,
    delivery_id BIGINT NOT NULL REFERENCES v1_webhook_deliveries(id) ON DELETE CASCADE,
    attempt_no SMALLINT NOT NULL,
    started_at TIMESTAMPTZ NOT NULL,
    duration_ms INTEGER,
    status_code SMALLINT,
    error TEXT,
    response_excerpt TEXT
);
```

- `response_excerpt` holds at most 1 KB. We never store response headers.
- A cleanup task deletes events, deliveries, and attempts older than 30 days.
- Payloads hold at most 256 KB. A job event holds counts and IDs, not the results. The receiver reads results with the API.

## Auth, Permissions, And Tenant Isolation

- Every query filters by `tenant_id`. Delivery and attempt reads join to the tenant's endpoints.
- Secrets are returned only once, at create and at rotation.
- The URL, the description, and the response excerpt are tenant data. Logs never contain the secret or the full body.

## Failure Modes And Edge Cases

| Case | Behavior |
|---|---|
| The receiver times out | The attempt fails. The next retry follows the schedule. |
| The receiver returns `2xx` slowly, after we time out | We retry. The receiver sees the event twice and uses `X-Reacher-Event-Id` to ignore the repeat. |
| The URL resolves to a private IP | The attempt fails with `blocked_target`. No retry for this reason. |
| The cipher key is missing | The backend refuses to start when endpoints exist. This stops silent delivery failures. |
| A tenant has 10,000 pending deliveries | The dispatcher sends at most 10 at a time for one endpoint, so one slow receiver cannot block other tenants. |
| An endpoint is deleted | Its pending deliveries are deleted with it. |

## Alternatives Considered

1. **A third-party webhook service.** Rejected. It adds a vendor and sends tenant data to another company.
2. **RabbitMQ for retries.** Rejected. The log and manual retry need rows in Postgres anyway, and the outbox pattern already exists.
3. **Keep one URL for each tenant.** Rejected. It does not meet backlog item 65.

## Rollout / Migration / Compatibility

1. Ship the cipher, the tables, and the dispatcher behind `webhooks_v2.enable`.
2. Run the migration that copies `default_webhook_url` into an endpoint for each tenant.
3. Turn on new event types for internal tenants, then for all tenants.
4. Release the web pages: an endpoint list (results first), a delivery log, and a sheet for create and edit.
5. Phase 2 (a later release): move pipeline and per-email webhooks to the outbox. Then mark `default_webhook_url` as deprecated.

Current receivers keep working, because they still get `X-Reacher-Signature-256`.

## Metrics And Success Criteria

- Delivery success rate on the first attempt and after all retries.
- p95 delay from event to first attempt. Target: under 5 seconds.
- Count of dead deliveries that a tenant retries by hand with success.

## Test Strategy

- Unit tests for the cipher: seal and open, a wrong tenant fails, an old key still opens.
- Unit tests for the signature, with test vectors in the docs.
- Database tests for fan-out, the retry schedule, the claim with `SKIP LOCKED`, and the auto-disable rule.
- End-to-end tests with a local receiver: success, timeout, `500` then success, `410`, and a private IP target.
- A restart test: stop the dispatcher during a retry wait. The delivery still happens after restart.
- API harness cases for every endpoint, including `403` without `settings` and `404` for another tenant's delivery.

## Definition Of Done

1. All migrations run up and down. `e2e_migration_upgrade` passes.
2. The OpenAPI document, the SDKs, and the web client show the new endpoints. CI finds no diff.
3. All tests pass in CI.
4. Helm values support `RCH__SECRETS__ENCRYPTION_KEYS` through `secretEnv` and `existingSecret`.
5. The docs have a webhook guide with signature code for TypeScript and Go.
6. The security review approves the signature, the SSRF rules, and the cipher.

## Open Questions

1. Do we allow HTTP (not HTTPS) targets in staging only? Owner: backend lead.
2. Do we limit endpoints by plan? Owner: product owner.

## Unresolved Risks

1. **Key loss.** If the cipher key is lost, all secrets are lost. The runbook covers key backup in Infisical.
2. **Outbox growth.** A large tenant can create many events. The 30-day cleanup and the payload limit control the size.
