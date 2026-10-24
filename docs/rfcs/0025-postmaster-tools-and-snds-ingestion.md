# RFC 0025: Postmaster Tools And SNDS Ingestion

## Verify disposition — 2026-10-23

**Parked; sender analytics outside initial scope.** Postmaster/SNDS data does not establish company/person/role identity. Revisit after sender demand, RFCs 0019/0017/0013, and confirmation of current provider API access. Do not assume historical API descriptions are still valid at implementation time.

[Current build order](README.md) · [Codebase audit](../audits/2026-10-23-verify-north-star.md). This disposition supersedes conflicting historical scope, priority, or baseline statements below; it does not certify every original acceptance criterion.

| | |
|---|---|
| **Status** | Parked; sender analytics outside initial scope; audited 2026-10-23 |
| **Created** | 2026-10-21 |
| **Owners** | To assign: backend lead, web lead |
| **Parent** | RFC 0033, item 10 |
| **Depends on** | RFC 0019 (monitored domains and sending IPs, alerts), RFC 0017 (provider report), RFC 0013 (tenant secret cipher) |
| **Release gates** | Google OAuth app approved for the Postmaster scope; Microsoft API access confirmed; Definition Of Done met |

## Summary

This RFC imports sender data that mailbox providers publish. From Google Postmaster Tools, it imports the daily spam rate, other domain statistics, and the compliance status. It does this for each domain that the tenant has verified with Google. From Microsoft SNDS, it imports daily data for each sending IP. The app shows this data next to the RFC 0019 health state and the RFC 0017 Gmail and Microsoft rows. It writes alerts when a spam rate crosses a limit.

## Motivation / Problem

- Gmail does not send a complaint report for each message. The only Gmail spam rate is the aggregate in Postmaster Tools. Without it, our Gmail rows have no complaint data (RFC 0017).
- Gmail and Yahoo ask bulk senders to keep the spam rate under 0.1% and never reach 0.3%. A sender needs to see this number every day, next to their bounces.
- Microsoft shows IP-level data in SNDS. Senders who use dedicated IPs need it.
- Today a sender must open each provider's site. Few do it every day.

## Goals

- Connect a Google account that has Postmaster access, and import data for its domains each day.
- Connect Microsoft SNDS, and import data for the tenant's sending IPs each day.
- Show the data with the right delay and coverage notes.
- Alert on spam-rate limits and on compliance changes.

## Non-Goals

- Yahoo Sender Hub data. A later RFC can add it if an API exists.
- Domain or IP reputation scores from Google. Postmaster Tools v2 removed these dashboards.
- Writing anything to the providers.

## Current State In Reacher

- No code reads provider sender data.
- Better Auth signs users in with Google and stores OAuth tokens encrypted (`encryptOAuthTokens: true` in `web/lib/auth/auth.ts`). Those tokens are for sign-in, belong to a user, and live in the web database.
- RFC 0019 adds monitored domains and `v1_sending_ips`. RFC 0013 adds the tenant secret cipher.

## What We Know About The Provider APIs

Confirm every item in this section against the official docs before build. Several items changed in 2025 and 2026.

**Google Postmaster Tools API v2**

- Base URL: `https://gmailpostmastertools.googleapis.com`.
- `domains.domainStats.query` takes `startDate` and `endDate`. It replaces the v1 `trafficStats` calls. `domainStats.batchQuery` reads many domains in one call.
- `GET /v2/{name=domains/*/complianceStatus}` returns the compliance status for SPF, DKIM, and DMARC. Third-party sources say the status also covers unsubscribe rules. Confirm the fields.
- OAuth scopes in the discovery document: `postmaster` and `postmaster.traffic.readonly`. Use the narrowest scope that the calls need.
- Google shows data only for domains that send enough daily volume to Gmail. Low-volume domains return no rows.
- The v1 API is deprecated. Build on v2 only.

**Microsoft SNDS**

- SNDS moved to a new portal in June 2026 (third-party reports give June 8, 2026). The old automated-access links stop working.
- The new access path is a REST API with OAuth 2.0. It can filter by date and IPv4 address.
- Some fields were removed in 2026, for example spam-trap hit counts. Read the current field list before mapping.

## Proposed Design

### Connections

| | Google Postmaster | Microsoft SNDS |
|---|---|---|
| Who connects | A tenant user whose Google account has Postmaster access to the domains | A tenant user with SNDS access to the IPs |
| Auth | OAuth 2.0 with the Postmaster scope, offline access | OAuth 2.0, per the new SNDS API |
| What we read | Domains the account can see, filtered to the tenant's monitored domains | Data for the tenant's sending IPs |
| Stored | Refresh token, sealed with the RFC 0013 cipher | Refresh token or key, sealed |

- The web app runs the OAuth flow at `/api/integrations/google-postmaster/callback` and `/api/integrations/snds/callback`. It sends the tokens to the backend over the server-side session path. The browser never holds them.
- The connection shows who connected it. If that person leaves, the tenant can reconnect with another account.
- A tenant can connect one Google account and one SNDS account in v1.

### Daily Sync

- A scheduler `spawn_sender_data_sync` runs when `sender_data.enable` and `worker.enable` are true. It claims due connections with `UPDATE ... RETURNING`, like `reverification.rs`.
- Each connection syncs once each day, at a random minute to spread the load. It also syncs once on connect, for the last 30 days.
- Google data is usually 1 to 2 days late. The sync reads the last 7 days each time, so late or changed rows update.
- The sync stores one row for each subject, date, and metric. It stores the raw response in a separate column, so a new metric needs no new API call.

### Metrics We Store

| Source | Subject | Metrics (confirm names) |
|---|---|---|
| Google | Domain | Spam rate, authentication success rates (SPF, DKIM, DMARC), encryption rate, delivery errors by type |
| Google | Domain | Compliance status for each requirement |
| Microsoft | IP | Message volume, complaint rate, filter result (green, yellow, or red), and the other fields that the new API returns |

### Alerts

They use the RFC 0019 alert shape.

| Alert | Condition |
|---|---|
| `gmail.spam_rate_warning` | The daily spam rate is 0.1% or higher. |
| `gmail.spam_rate_critical` | The daily spam rate is 0.3% or higher. |
| `gmail.compliance_changed` | A compliance requirement moves from pass to fail. |
| `snds.filter_red` | An IP's filter result moves to red. |
| `sender_data.connection_failed` | A token refresh fails, or the API returns `403` for 2 days. |

A spam-rate alert fires once when the rate crosses the limit. It fires again only after the rate goes back under the limit for 3 days.

### Display

- The domain health page (RFC 0019) shows a 30-day spam rate chart with lines at 0.1% and 0.3%.
- The provider report (RFC 0017) fills `complaint_data` for Gmail rows with the Postmaster spam rate for the same days. It labels the source and the delay.
- When Google returns no data for a domain, the page says "Gmail shows no data for this domain. This is normal for low daily volume."

## Public API / Interface Changes

| Method | Path | Purpose |
|---|---|---|
| `GET` | `/v1/sender-data/connections` | List connections and their status. |
| `DELETE` | `/v1/sender-data/connections/{id}` | Disconnect and delete the token. |
| `POST` | `/v1/sender-data/connections/{id}/sync` | Start a sync now. At most once each hour. |
| `GET` | `/v1/sender-data/stats` | Rows by `source`, `subject`, `metric`, `since`, `until`. |
| `GET` | `/v1/sender-data/compliance` | The latest compliance status for each domain. |

Reads accept the `reputation` scope. Connect and disconnect need `settings`. The OAuth start and callback routes are in the web app.

## Data Model / Storage Changes

```sql
CREATE TABLE v1_sender_data_connections (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    source TEXT NOT NULL CHECK (source IN ('google_postmaster', 'microsoft_snds')),
    connected_by TEXT NOT NULL,
    credential_sealed JSONB NOT NULL,
    status TEXT NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'error', 'disconnected')),
    status_detail TEXT,
    next_sync_at TIMESTAMPTZ,
    last_sync_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (tenant_id, source)
);

CREATE TABLE v1_sender_data_stats (
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    source TEXT NOT NULL,
    subject_type TEXT NOT NULL CHECK (subject_type IN ('domain', 'ip')),
    subject TEXT NOT NULL,
    stat_date DATE NOT NULL,
    metric TEXT NOT NULL,
    value DOUBLE PRECISION,
    detail JSONB NOT NULL DEFAULT '{}'::jsonb,
    fetched_at TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (tenant_id, source, subject_type, subject, stat_date, metric)
);

CREATE TABLE v1_sender_data_raw (
    id BIGSERIAL PRIMARY KEY,
    connection_id UUID NOT NULL REFERENCES v1_sender_data_connections(id) ON DELETE CASCADE,
    request TEXT NOT NULL,
    response JSONB NOT NULL,
    fetched_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

- Raw responses are deleted after 30 days. Stats are kept for 13 months.
- Config: `RCH__SENDER_DATA__GOOGLE_CLIENT_ID`, `RCH__SENDER_DATA__GOOGLE_CLIENT_SECRET`, and the matching Microsoft values, through `secretEnv` and `existingSecret` in Helm.

## Auth, Permissions, And Tenant Isolation

- A Google account can see domains that belong to other companies, for example at an agency. The sync stores data only for domains that the tenant monitors (RFC 0019). Other domains are ignored and not stored.
- Tokens are sealed with the tenant ID as associated data, and they never appear in responses or logs.
- Disconnect deletes the sealed token at once. We also revoke the token at the provider when its API allows it.

## Failure Modes And Edge Cases

| Case | Behavior |
|---|---|
| The provider returns no rows for a day | No row is stored. The chart shows a gap, not zero. |
| The user loses Postmaster access to a domain | The API returns no data or `403` for that domain. The page says so. Other domains keep syncing. |
| The token refresh fails | The connection moves to `error`. An alert asks the tenant to reconnect. |
| Google changes a metric name | The raw response is kept. A mapping update can fill the stats again from raw rows of the last 30 days. |
| The tenant monitors a domain that Google does not show | The page says how to add the domain in Postmaster Tools. |

## Alternatives Considered

1. **Ask tenants to upload Postmaster exports.** Rejected. Nobody does it every day.
2. **Use Better Auth sign-in tokens.** Rejected. Those tokens belong to a user session, use other scopes, and live in the web database. The sync runs in the backend.
3. **Scrape the provider sites.** Rejected. It breaks the providers' terms and breaks often.

## Rollout / Migration / Compatibility

1. Confirm the API facts. Apply for Google OAuth approval for the Postmaster scope. This can take weeks, so start early.
2. Ship Google first behind `sender_data.enable`. Test with internal domains.
3. Ship SNDS when Microsoft API access is confirmed.
4. Regenerate the OpenAPI document and the SDKs.

No existing behavior changes.

## Metrics And Success Criteria

- Share of tenants with monitored domains that connect Postmaster Tools.
- Share of syncs that succeed. Target: 99%.
- Count of Gmail rows in the RFC 0017 report with spam-rate data.

## Test Strategy

- Unit tests for the mapping from recorded v2 and SNDS responses to stats rows.
- Unit tests for the alert rules: crossing, quiet period, and reset after 3 days.
- End-to-end tests with stub provider servers: connect, first 30-day sync, daily sync, token refresh failure, and a domain that the tenant does not monitor.
- A test that tokens never appear in logs or API responses.
- A weekly contract test with a test Google account and a test SNDS account.

## Definition Of Done

1. The migrations run up and down. `e2e_migration_upgrade` passes.
2. Every item in "What We Know About The Provider APIs" is confirmed against the official docs.
3. The OpenAPI document, the SDKs, and the web client show the new endpoints. CI finds no diff.
4. All tests pass, including the contract tests.
5. Google approves the OAuth app.
6. The domain health page shows the spam rate chart and the coverage notes.

## Open Questions

1. Is the Postmaster scope "sensitive" or "restricted" for Google OAuth review? This decides the review time and cost. Owner: backend lead.
2. Do we let one tenant connect more than one Google account? Owner: product owner.

## Unresolved Risks

1. **Provider changes.** Both providers changed their tools in 2025 and 2026. More changes are likely. The raw store and the contract tests limit the damage.
2. **Access depends on the customer.** Many customers have not set up Postmaster Tools. The setup guide must walk them through it.
