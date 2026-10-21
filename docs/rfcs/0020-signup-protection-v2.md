# RFC 0020: Signup Protection V2

| | |
|---|---|
| **Status** | Draft; parked 2026-10-21 (customer signup forms are not in the Paperless focus; see the RFC 0016 build order) |
| **Created** | 2026-10-21 |
| **Owners** | To assign: backend lead, web lead |
| **Parent** | RFC 0016, item 4 |
| **Builds on** | RFC 0007 (Verification Policy Modes) |
| **Depends on** | RFC 0019 for audience-scoped suppression checks (optional) |
| **Release gates** | Privacy review of IP handling; latency targets met in staging; Definition Of Done met |

## Summary

This RFC adds `POST /v1/signup-checks`, an endpoint that a customer's signup backend calls before it accepts a new address. It runs cheap local checks first and the paid mailbox check last. It returns one of three decisions: `allow`, `confirm`, or `block`. `confirm` means "send a double opt-in email first". The endpoint skips the paid check when local signals already decide.

## Motivation / Problem

- Bots fill signup forms with fake, random, or stolen addresses. These addresses bounce, hit spam traps, and damage the sender's reputation.
- The `signup_protection` policy mode (`decision/policy.rs`) looks only at the verification result. It does not see bot patterns, such as many signups from one IP in a few minutes.
- In managed deployments, each new mailbox check is a paid MillionVerifier call (`backend/src/verification.rs`). A bot attack against a customer's form spends the customer's credits and our provider credits.
- A signup form must answer fast. A single check can wait up to 20 seconds today (`HTTP_WAIT` in `http/shared/verification_operation.rs`).
- A hard block loses real users when a signal is wrong. Senders need a soft step between "allow" and "block".

## Goals

- Return a decision in under 300 ms (p95) when local signals decide.
- Never spend a paid check on an address that local signals already block or send to double opt-in.
- Reuse a recent verification of the same mailbox instead of buying a new one.
- Give a clear reason for every decision.
- Handle the end user's IP address as personal data.

## Non-Goals

- A browser script or a publishable key. The customer's server calls the endpoint with its API key. A later RFC can add a browser key.
- CAPTCHA. The customer's form can use one and send the result as a form signal.
- Changes to `RecommendedAction`. The new endpoint has its own `decision` field, so existing SDK enums do not change. This changes the plan in RFC 0016, which proposed a new `confirm` action.

## Current State In Reacher

- `evaluate_signup_protection` suppresses disposable, spam-trap, and high bounce-risk addresses. It sends catch-all, unknown, and SMTP-error results to review.
- `POST /v1/check_email` admits a durable operation (quota, identity, and dispatch in one transaction), then waits up to 20 seconds. After that it returns `202` with a status URL.
- The tenant throttle uses `tenant_rate_windows` (1 second, 1 minute, 1 hour, 1 day windows).
- Core has local disposable and spam-trap lists (`core/src/misc/`), provider syntax rules (`core/src/provider/`), and Gmail alias normalization (`mailbox_identity()` in the database).
- `reputation/checker.rs` reads domain age over RDAP and caches it in `reputation_cache` for 24 hours.
- `tenants.settings` is a JSONB object (migration `20261020000003`).

## Proposed Design

### Request

```json
{
  "email": "alex@example.com",
  "client_ip": "198.51.100.7",
  "user_agent": "Mozilla/5.0 ...",
  "form": { "honeypot_filled": false, "elapsed_ms": 8400 },
  "audience_key": "newsletter:weekly",
  "max_wait_ms": 1500,
  "on_timeout": "allow"
}
```

- Only `email` is required.
- `max_wait_ms` is the time we wait for a paid check. The default is 1,500. The maximum is 5,000.
- `on_timeout` is `allow` (default) or `confirm`. It decides what happens when the paid check does not finish in time.

### Response

```json
{
  "decision": "confirm",
  "reasons": [
    { "code": "ip_velocity", "severity": "warning", "message": "12 signups from this IP in 10 minutes." }
  ],
  "suggested_email": null,
  "verification": { "status": "skipped", "operation_id": null },
  "signals": { "ip_signups_10m": 12, "domain_age_days": 2100 }
}
```

`verification.status` is `complete`, `reused`, `pending`, or `skipped`. When it is `pending`, `operation_id` lets the caller read the result later from `GET /v1/operations/{id}`.

### Steps

The endpoint runs these steps in order. It stops at the first `block`. It skips the paid check after any `confirm`, because the double opt-in email proves the address.

| # | Step | Cost | Possible decision |
|---|---|---|---|
| 1 | Syntax and provider rules | Local | `block` |
| 2 | Suppression check (tenant and audience) | 1 query | `block` |
| 3 | Disposable and spam-trap domain lists | Local | `block` |
| 4 | Form signals: honeypot filled; `elapsed_ms` under 1,500 | Local | `block` (honeypot), `confirm` (too fast) |
| 5 | Velocity: signups from one IP, at one domain, and for one mailbox in 10 minutes and 24 hours | 1 query | `confirm`, or `block` above the hard limit |
| 6 | Pattern: random local part; many aliases of one mailbox | Local and 1 query | `confirm` only with another warning signal |
| 7 | New domain: domain age under 30 days, for non-free domains | Cached RDAP | `confirm` |
| 8 | Recent result: a verification of the same mailbox in the last 30 days | 1 query | Decision from that result |
| 9 | Paid check: admit an operation and wait up to `max_wait_ms` | 1 credit | Decision from the result, or `on_timeout` |

Mapping from a verification result (steps 8 and 9) to a decision:

| `signup_protection` policy result | Decision |
|---|---|
| `send` | `allow` |
| `fix_then_send` | `allow` with `suggested_email` |
| `review` | `confirm` |
| `suppress`, `drop` | `block` |

### Default Thresholds

| Signal | Confirm at | Block at |
|---|---|---|
| Signups from one IP in 10 minutes | 5 | 50 |
| Signups from one IP in 24 hours | 20 | 500 |
| Signups at one non-free domain in 10 minutes | 10 | none |
| Aliases of one mailbox in 24 hours | 3 | none |

A tenant can change these in `tenants.settings.signup_protection`. Free mail domains (for example `gmail.com`) are not counted in the domain signal.

The random local part check uses a simple score: length, digit share, and runs of consonants. It is a weak signal, so it never decides alone. A code comment names this limit.

### Privacy

- We never store `client_ip`, `user_agent`, or `email` in plain text in the signup tables.
- We store `HMAC-SHA256(pepper, value)` for the IP and the mailbox identity. The pepper comes from `RCH__PRIVACY__HASH_PEPPER`. RFC 0015 proposed the same secret, so both features share it.
- Signup attempt rows are deleted after 7 days.
- The response never echoes the IP.

### Credits

- Steps 1 to 8 do not use a credit.
- Step 9 uses one credit, like any check.
- This follows the rule "every check path counts against the plan allowance". A signup check without a mailbox check is not a check.

## Public API / Interface Changes

| Method | Path | Change |
|---|---|---|
| `POST` | `/v1/signup-checks` | New. Needs the `verify` scope. |
| `GET` | `/v1/signup-checks/stats` | New. Counts of each decision and reason for the last 30 days. Needs `verify`. |
| `GET`, `PATCH` | `/v1/me/settings` | New object `signup_protection` with the thresholds and `default_on_timeout`. |

The endpoint supports the `Idempotency-Key` header, as other write endpoints do (`http/idempotency.rs`).

## Data Model / Storage Changes

```sql
CREATE TABLE v1_signup_attempts (
    id BIGSERIAL PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    mailbox_hash TEXT NOT NULL,
    ip_hash TEXT,
    domain TEXT NOT NULL,
    decision TEXT NOT NULL CHECK (decision IN ('allow', 'confirm', 'block')),
    reason_codes TEXT[] NOT NULL DEFAULT '{}',
    verification_status TEXT NOT NULL,
    operation_id UUID
);
CREATE INDEX idx_v1_signup_attempts_ip
    ON v1_signup_attempts (tenant_id, ip_hash, created_at DESC) WHERE ip_hash IS NOT NULL;
CREATE INDEX idx_v1_signup_attempts_domain
    ON v1_signup_attempts (tenant_id, domain, created_at DESC);
CREATE INDEX idx_v1_signup_attempts_mailbox
    ON v1_signup_attempts (tenant_id, mailbox_hash, created_at DESC);
```

- The domain is stored in plain text. It is not personal data for a company domain, and the domain signal needs it. For free mail domains, the domain is stored but not counted.
- A cleanup task, like `spawn_idempotency_cleanup`, deletes rows older than 7 days.

## Auth, Permissions, And Tenant Isolation

- The endpoint needs an API key with the `verify` scope. Today API keys are for the Professional and Enterprise plans only (gap G5 in `docs/strategy/marketing-product-gaps.md`).
- Velocity counts are for each tenant only. One tenant's traffic never changes another tenant's decisions.
- The tenant throttle (`tenant_rate_windows`) applies before step 1. A flood that passes the throttle still cannot spend credits beyond the hard velocity limits.

## Failure Modes And Edge Cases

| Case | Behavior |
|---|---|
| The database is down | `503`. The customer's form decides what to do. We do not guess `allow`. |
| The paid check fails | `verification.status = "pending"` or an error status. The decision follows `on_timeout`. |
| The tenant has no quota left | Step 9 is skipped. The decision follows `on_timeout`, and the reason `quota_exhausted` is added. |
| A shared office IP sends many real signups | The tenant raises the IP thresholds, or the decisions stay `confirm`, not `block`, below the hard limit. |
| The same signup is retried | The `Idempotency-Key` returns the first answer. Without it, the retry counts as a new attempt. |
| No `client_ip` | The IP signals are skipped. The response has the reason `ip_not_provided` with severity `info`. |

## Alternatives Considered

1. **Add `confirm` to `RecommendedAction`.** Rejected. Generated SDK clients can fail on an unknown enum value in existing endpoints.
2. **Count velocity in memory.** Rejected. Many backend replicas run, and counts must be shared.
3. **Always run the paid check.** Rejected. It lets a bot spend credits, and the double opt-in already proves the address.

## Rollout / Migration / Compatibility

1. Ship the endpoint behind `signup_checks.enable`. Turn it on in staging.
2. Load test: 200 requests each second for one tenant. Measure p95 latency without paid checks.
3. Turn it on for production tenants on plans with API access.
4. Publish a guide: "Protect your signup form", with server examples in TypeScript and Go from the generated SDKs.

No existing endpoint changes.

## Metrics And Success Criteria

- p95 latency without a paid check. Target: 300 ms or less.
- Share of signup checks that skip the paid check.
- Bounce rate of addresses that got `allow`, from feedback events. Target: lower than for addresses that did not use the endpoint.
- Count of `block` decisions that a tenant reverses by hand. A high count means a bad signal.

## Test Strategy

- Unit tests for each step and for the order: a `block` stops the chain; a `confirm` skips the paid check.
- Unit tests for the random local part score with a fixed list of real and random examples.
- Database tests for the velocity counts across the 10-minute and 24-hour windows.
- An end-to-end test with the MillionVerifier stub: the paid check runs once for a clean address, and zero times for a blocked or confirmed address.
- A test that no plain IP or email address is written to `v1_signup_attempts`.
- A load test script in `backend/tests/` for the latency target.

## Definition Of Done

1. The migration runs up and down. `e2e_migration_upgrade` passes.
2. The OpenAPI document and the SDKs show the endpoint. CI finds no diff.
3. All tests pass. The load test meets the latency target in staging.
4. The privacy review approves the hash and retention design.
5. The settings page has controls for the thresholds.

## Open Questions

1. Does the Starter plan get this endpoint without full API access? Owner: product owner.
2. Do we need a browser-safe key in v2? Owner: product owner.

## Unresolved Risks

1. **Shared IPs.** Mobile networks and offices put many people behind one IP. The defaults use `confirm`, not `block`, for most signals.
2. **Attackers adapt.** Bots can slow down and spread across IPs. The paid check and the double opt-in still apply to the addresses that pass.
