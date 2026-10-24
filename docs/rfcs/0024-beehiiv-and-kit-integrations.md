# RFC 0024: beehiiv And Kit Integrations

## Verify disposition — 2026-10-23

**Parked; newsletter buyer outside initial scope.** Newsletter synchronization is not the agency audit handoff. Requested CRM correction is specified in RFC 0006. Revisit this proposal only with newsletter demand and its original RFC 0011/0013, platform-license, and authentication prerequisites.

[Current build order](README.md) · [Codebase audit](../audits/2026-10-23-verify-north-star.md). This disposition supersedes conflicting historical scope, priority, or baseline statements below; it does not certify every original acceptance criterion.

| | |
|---|---|
| **Status** | Parked; newsletter buyer outside initial scope; audited 2026-10-23 |
| **Created** | 2026-10-21 |
| **Owners** | To assign: backend lead, web lead |
| **Parent** | RFC 0033, item 9 |
| **Builds on** | RFC 0031 (Provider Webhooks) |
| **Depends on** | RFC 0013 (tenant secret cipher), RFC 0011 (audience scopes) |
| **Release gates** | Platform API terms and rate limits confirmed; contract tests against both platforms; Definition Of Done met |

## Summary

This RFC connects a tenant's beehiiv publication or Kit account to our backend. When a subscriber joins, the platform sends a webhook. We check the address and write the decision back to the platform as a tag, or as an unsubscribe when the tenant chooses. We also import unsubscribes, bounces, and complaints, and we push our suppressions back to the platform.

## Motivation / Problem

- Newsletter platforms send email for many creators. Their users are in our buyer list: teams that already have addresses and send to them.
- Fake and mistyped signups reach these lists every day. A creator finds them only when they bounce or never open.
- Today a creator must export a CSV, clean it with us, and import it again. Nobody repeats this often enough.

## Goals

- One connection flow for each platform, in the app.
- Check each new subscriber automatically, within one minute of signup.
- Write results back without destructive changes by default.
- Import negative feedback and apply it with audience scopes.
- Push our suppressions to the platform.

## Non-Goals

- Sync of posts, campaigns, or engagement data.
- A full import of existing subscribers in v1. The tenant can upload a CSV as today. A later version can add a one-time import.
- Other platforms. The design is generic, so a later RFC can add more.

## Current State In Reacher

- `v1_provider_endpoints` stores inbound webhook endpoints. A `CHECK` limits `provider` to `sendgrid`, `ses`, `mailgun`, and `postmark`. Each endpoint has a hashed delivery token and optional `allowed_ips`.
- `POST /v1/inbound/providers/{provider}/{endpoint_id}/{delivery_token}` receives events. Each request is saved in `v1_provider_outcome_receipts` before it is normalized.
- Endpoint management needs the `settings` scope. Provider webhooks work on every plan (gap G16 in `docs/strategy/marketing-product-gaps.md`).
- Outbound calls to tenant targets use the SSRF rules in `pipelines.rs`.
- There is no store for tenant credentials for outside platforms. RFC 0013 adds the cipher.

## What We Know About The Platforms

Confirm every item in this section against the official API reference before build. The contract tests in "Test Strategy" prove each one.

| | beehiiv | Kit |
|---|---|---|
| Auth | API key (Bearer) | OAuth for API v4. Confirm if API keys also work for the calls we need. |
| Subscriber joins | `subscription.created`, `subscription.confirmed` | `subscriber.subscriber_activate` (v3 name; confirm the v4 name) |
| Unsubscribe | `subscription.deleted` | `subscriber.subscriber_unsubscribe` |
| Bounce and complaint events | Not found in public docs | Confirm (v3 had subscriber bounce and complaint events) |
| Webhook registration by API | `POST /v2/publications/{id}/webhooks` with `event_types` | Confirm the v4 path |
| Webhook plan limit | Webhooks by API need a paid plan (the docs say Scale or higher) | Confirm |
| Signature on deliveries | Confirm | Kit says it signs each delivery |
| Batching | Confirm | Up to 100 events in one delivery for bulk actions |
| Tag a subscriber | Confirm the endpoint | Tag endpoint by subscriber ID |
| Unsubscribe by API | Confirm the endpoint | v4 endpoint by subscriber ID |

## Proposed Design

### Connection

A new resource, `v1_platform_connections`, holds one connection:

| Field | Meaning |
|---|---|
| `platform` | `beehiiv` or `kit`. |
| `account_ref` | The beehiiv publication ID, or the Kit account ID. |
| `credential` | The API key or OAuth tokens, sealed with the RFC 0013 cipher. |
| `audience_key` | The RFC 0011 audience for this connection, for example `beehiiv:pub_123`. |
| `actions` | What to write back for each decision. |
| `status` | `active`, `paused`, `error`, or `disconnected`. |
| `inbound_endpoint_id` | The `v1_provider_endpoints` row that receives this connection's webhooks. |

Connect flow:

1. The tenant opens Settings → Integrations and picks a platform.
2. For beehiiv, the tenant pastes an API key and picks a publication. For Kit, the tenant signs in with OAuth.
3. The backend tests the credential with a read call.
4. The backend creates an inbound endpoint and registers its URL as a webhook on the platform, with the event types it needs.
5. The connection is `active`.

When the platform plan does not allow webhooks by API, step 4 fails. The app then shows the URL and the steps to add it by hand.

### Write-Back Actions

| Decision (`signup_protection` policy) | Default action | Other choices |
|---|---|---|
| `send` | Tag `reacher-valid` | None |
| `fix_then_send` | Tag `reacher-typo`, and set a custom field with the suggested address when the platform supports it | None |
| `review` | Tag `reacher-review` | None |
| `suppress`, `drop` | Tag `reacher-invalid` | Unsubscribe the subscriber |

- The defaults only add tags. They never remove a subscriber. The tenant must turn on unsubscribe by hand, and the app warns that it cannot be undone from our side.
- Tag names are tenant settings.

### Flow For A New Subscriber

1. The platform sends a webhook. The inbound handler saves a receipt and checks the delivery token and the signature.
2. The normalizer creates one row in `v1_platform_subscriber_checks` for each new subscriber, with the platform's subscriber ID.
3. The backend admits a verification operation with `policy_mode = signup_protection` and the connection's `audience_key`. This uses one credit.
4. When the result is ready, a write-back job calls the platform API.
5. The row stores the decision, the action, and the write-back status.

A repeat delivery of the same event creates no new check, because the row is unique for each connection and platform subscriber ID.

### Feedback From The Platform

- An unsubscribe event becomes an `unsubscribed` outcome with `metadata.audience_key` set to the connection's audience. RFC 0011 then creates an audience-scoped marketing suppression.
- Bounce and complaint events, where a platform sends them, become `bounced` and `complained` outcomes. RFC 0012 classifies them.

### Suppression Push-Back

- A push job starts when a tenant-scoped suppression is created, or an audience suppression for the connection's audience.
- The job finds the subscriber on the platform by email address. Then it applies the tenant's action for `suppress`.
- Push-back is off by default. The tenant turns it on for each connection.

### Outbound Calls

- A background worker sends write-back and push calls. It claims due rows with `FOR UPDATE SKIP LOCKED`.
- It follows each platform's rate limit, with a token bucket for each connection.
- It retries on `429` and `5xx` with backoff, for up to 24 hours. After that, the row is `failed` and the connection shows the error.
- A `401` or `403` sets the connection to `error` and writes an alert. The worker stops calling until the tenant fixes the credential.

### Quota

- When the tenant has no quota left, new subscriber rows wait with the status `quota_exhausted`. No write-back happens. We never tag or unsubscribe a subscriber because our quota ran out.
- When quota returns, the rows from the last 7 days are checked. Older rows are skipped and shown in the app.

## Public API / Interface Changes

| Method | Path | Purpose |
|---|---|---|
| `GET`, `POST` | `/v1/platform-connections` | List and create connections. |
| `GET`, `PATCH`, `DELETE` | `/v1/platform-connections/{id}` | Read, change actions and status, and disconnect. Disconnect removes the platform webhook when the API allows it. |
| `POST` | `/v1/platform-connections/{id}/test` | Test the credential and the webhook. |
| `GET` | `/v1/platform-connections/{id}/subscriber-checks` | List checks: `status`, `decision`, `since`, `limit`, `offset`. |
| `POST` | `/v1/platform-connections/{id}/subscriber-checks/{check_id}/retry` | Retry a failed write-back. |
| `GET` | `/api/integrations/kit/callback` | The Kit OAuth callback, in the web app. It passes the tokens to the backend over the server-side session path. |
| `POST` | `/v1/inbound/providers/{provider}/{endpoint_id}/{delivery_token}` | Accepts `beehiiv` and `kit` as providers. |

All management endpoints need the `settings` scope.

## Data Model / Storage Changes

```sql
ALTER TABLE v1_provider_endpoints DROP CONSTRAINT v1_provider_endpoints_provider_check;
ALTER TABLE v1_provider_endpoints ADD CONSTRAINT v1_provider_endpoints_provider_check
    CHECK (provider IN ('sendgrid', 'ses', 'mailgun', 'postmark', 'beehiiv', 'kit'));

CREATE TABLE v1_platform_connections (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    platform TEXT NOT NULL CHECK (platform IN ('beehiiv', 'kit')),
    account_ref TEXT NOT NULL,
    label TEXT NOT NULL,
    credential_sealed JSONB NOT NULL,
    audience_key TEXT NOT NULL,
    actions JSONB NOT NULL,
    push_suppressions BOOLEAN NOT NULL DEFAULT false,
    status TEXT NOT NULL DEFAULT 'active'
        CHECK (status IN ('active', 'paused', 'error', 'disconnected')),
    status_detail TEXT,
    inbound_endpoint_id UUID REFERENCES v1_provider_endpoints(id) ON DELETE SET NULL,
    platform_webhook_ref TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (tenant_id, platform, account_ref)
);

CREATE TABLE v1_platform_subscriber_checks (
    id BIGSERIAL PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    connection_id UUID NOT NULL REFERENCES v1_platform_connections(id) ON DELETE CASCADE,
    platform_subscriber_id TEXT NOT NULL,
    email TEXT NOT NULL,
    status TEXT NOT NULL CHECK (status IN
        ('queued', 'verifying', 'quota_exhausted', 'writing_back', 'done', 'failed', 'skipped')),
    operation_id UUID,
    decision TEXT,
    action TEXT,
    attempt_count SMALLINT NOT NULL DEFAULT 0,
    next_attempt_at TIMESTAMPTZ,
    last_error TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (connection_id, platform_subscriber_id)
);
CREATE INDEX idx_v1_platform_subscriber_checks_due
    ON v1_platform_subscriber_checks (next_attempt_at)
    WHERE status IN ('queued', 'writing_back');
```

- Do not edit the migration that created the provider check. The new migration drops and adds the constraint.
- Rows in `v1_platform_subscriber_checks` follow the tenant's `result_retention_days`.

## Auth, Permissions, And Tenant Isolation

- Credentials are sealed with the tenant ID as associated data. They never appear in API responses, logs, or task payloads.
- The inbound URL has a delivery token, as today. When a platform signs its deliveries, we also check the signature.
- The write-back worker only calls the platform host for the connection (`api.beehiiv.com` or Kit's API host). It does not call tenant-supplied URLs.

## Failure Modes And Edge Cases

| Case | Behavior |
|---|---|
| The platform sends the same event twice | One check row, one credit. |
| A subscriber is deleted on the platform before write-back | The platform returns `404`. The row is `skipped`. |
| The tenant disconnects | Pending rows become `skipped`. The credential is deleted. The audit trail stays. |
| The platform changes its payload | The normalizer finds no subscriber, the receipt shows `validation_failed`, and an alert tells us. |
| A creator's list grows by 50,000 in one day | Each new subscriber uses one credit. The app shows the expected daily use, and the quota limits spending. |
| Kit OAuth refresh fails | The connection moves to `error` with an alert. |

## Alternatives Considered

1. **Polling the subscriber list instead of webhooks.** Kept as a fallback only. It is slower and uses more of the platform's rate limit.
2. **Unsubscribe by default.** Rejected. A false result would remove a real reader with no way back from our side.
3. **Zapier or Make instead of native code.** Rejected as the main path. Those connectors are a separate item. A native flow can retry, scope suppressions, and push back.

## Rollout / Migration / Compatibility

1. Confirm the platform facts table. Record payload fixtures from test accounts.
2. Ship beehiiv first behind `integrations.beehiiv.enable`. Run it with two design partners.
3. Ship Kit after the OAuth app is approved.
4. Publish setup guides for each platform.

No existing behavior changes.

## Metrics And Success Criteria

- Connections created and active after 30 days.
- p95 time from signup to write-back. Target: under 60 seconds.
- Write-back failure rate. Target: under 1%.
- Bounce rate after connection, compared with the 30 days before, from imported feedback.

## Test Strategy

- Unit tests for each normalizer with recorded fixtures: single events, batches, unsubscribes, and unknown types.
- Unit tests for the action map and tag names.
- End-to-end tests with a stub platform server. They connect, receive a signup, verify with the stub verifier, and write back a tag. They also cover `429`, `401`, and `404`.
- A weekly contract test against a test account on each platform.
- A test that no credential appears in a log line or an API response.

## Definition Of Done

1. All migrations run up and down. `e2e_migration_upgrade` passes.
2. Every item in "What We Know About The Platforms" is confirmed and has a fixture.
3. The OpenAPI document, the SDKs, and the web client show the new endpoints. CI finds no diff.
4. All tests pass. The contract tests pass for both platforms.
5. The integrations page in the app supports connect, test, pause, and disconnect.

## Open Questions

1. Do we offer a one-time check of existing subscribers in v1? Owner: product owner.
2. Which plans include platform connections? Owner: product owner.

## Unresolved Risks

1. **Platform terms.** A platform can limit or change its API. The contract tests show changes early.
2. **Plan limits on the platform side.** beehiiv webhooks by API need a paid beehiiv plan. Creators on free plans need the manual steps or polling.
