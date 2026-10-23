# RFC 0017: Scheduled Domain And IP Health Monitor

## Verify disposition — 2026-10-23

**Deferred; sender-demand gate.** Preserve existing reputation checks. Scheduled domain/IP monitoring is not required to deliver the agency contact audit. Revisit with sender demand, confirmed blocklist terms, and the non-email alert model; it remains the dependency for the sender-alert slice of RFC 0018.

[Current build order](README.md) · [Codebase audit](../audits/2026-10-23-verify-north-star.md). This disposition supersedes conflicting historical scope, priority, or baseline statements below; it does not certify every original acceptance criterion.

| | |
|---|---|
| **Status** | Deferred; sender-demand gate; audited 2026-10-23 |
| **Created** | 2026-10-21 |
| **Owners** | To assign: backend lead, web lead |
| **Parent** | RFC 0016, item 1 |
| **Depends on** | None. RFC 0023 adds webhook delivery for the alerts in this RFC. |
| **Used by** | RFC 0018 (sender alerts), RFC 0026 (provider data next to health state) |
| **Release gates** | Blocklist usage terms confirmed for each list; Definition Of Done met |

## Summary

This RFC runs domain and IP health checks on a schedule for the sending domains and sending IPs that a tenant registers. When a domain or IP goes on a blocklist, or when an SPF, DKIM, or DMARC record breaks, the backend writes an alert. The on-demand check `POST /v1/reputation/check` stays as it is.

## Motivation / Problem

- A sender finds out about a blocklisting when mail starts to bounce. By then, the damage is done.
- Today, a tenant can check a domain only on request. Nobody repeats the check.
- The current blocklist check looks up the IP of the domain's **receiving** MX host. That IP says nothing about the reputation of the IPs that **send** the tenant's mail.
- The DKIM check tries five fixed selectors. Many senders use other selectors, so the check often reports "unknown".

## Goals

- Check each registered sending domain and sending IP on a schedule.
- Check sending IPs against IP blocklists, and sending domains against domain blocklists.
- Let the tenant give their own DKIM selectors.
- Check the parts of the Gmail and Yahoo bulk-sender rules that DNS can show.
- Write one alert when a state changes, not one alert for each run.

## Non-Goals

- We do not send test messages or run a seed-list inbox test.
- We do not parse DMARC aggregate reports (`rua`). A later RFC can add this.
- We do not delist anything for the tenant. We link to each list's removal page.
- We do not check one-click unsubscribe headers or spam rates. RFC 0026 imports spam rates from Google.

## Current State In Reacher

- `backend/src/reputation/checker.rs` (`check_domain`) runs the DNS checks, the blocklist checks, and an RDAP domain-age lookup. It caches the full response in the global table `reputation_cache` for 24 hours.
- `backend/src/reputation/dnsbl.rs` (`lookup_dnsbl`) queries 8 IP lists: Spamhaus ZEN, SpamCop, Barracuda, SORBS, UCEPROTECT, PSBL, RATS, and Invaluement. It queries the first IPv4 address of the lowest-preference MX host. It classifies `127.255.255.x` answers as errors, not as listings.
- `backend/src/reputation/dns_records.rs` reads SPF, DMARC, and MX records. It tries the DKIM selectors `default`, `google`, `selector1`, `selector2`, and `k1`. `has_dkim` is `None` when no selector matches, because a selector search cannot prove absence.
- `tenant_domains` (`/v1/me/domains`) stores domains for each tenant with `is_active` and `is_verified`. No backend code reads this table outside its own handler.
- `v1_alerts` stores alerts. `canonical_email` is `NOT NULL`, so an alert must be about one email address. Only `list_intelligence.rs` writes alerts. No alert sends a webhook.
- Schedulers start in `backend/src/http/mod.rs` when `<feature>.enable` and `worker.enable` are both true. `reverification.rs` claims due rows with one atomic `UPDATE ... RETURNING`.

## Proposed Design

### Monitored Assets

A tenant monitors two kinds of assets:

| Asset | Source | Checks |
|---|---|---|
| Sending domain | A `tenant_domains` row with `monitor_enabled = true` | SPF, DKIM, DMARC, MX, domain blocklists, domain age |
| Sending IP | A new `v1_sending_ips` row | IP blocklists, reverse DNS (PTR), forward-confirmed reverse DNS |

The tenant enters sending IPs by hand. A later version can read them from the ESP. We do not guess sending IPs from SPF records, because SPF often authorizes large shared ranges.

### Checks

| Check key | Asset | Rule |
|---|---|---|
| `spf.present` | Domain | One `v=spf1` TXT record exists. |
| `spf.lookup_limit` | Domain | The SPF record needs 10 or fewer DNS lookups (RFC 7208, section 4.6.4). |
| `dkim.selector` | Domain | Each tenant selector, plus the default selectors, returns a usable key. |
| `dmarc.present` | Domain | One `v=DMARC1` record exists at `_dmarc.<domain>`. |
| `dmarc.policy` | Domain | The policy is `none`, `quarantine`, or `reject`. The report shows which one. |
| `bulk_sender.dns` | Domain | SPF and DKIM exist, and DMARC exists with any policy. This is the DNS part of the Gmail and Yahoo rules. |
| `blocklist.domain` | Domain | Domain lists: Spamhaus DBL, SURBL, and URIBL. |
| `blocklist.ip` | IP | The 8 IP lists in `dnsbl.rs`, after the review in "Blocklist Usage". |
| `ptr.present` | IP | The IP has a PTR record. |
| `ptr.forward_confirmed` | IP | The PTR host name resolves back to the same IP. |

Each check has one of these states: `pass`, `warn`, `fail`, `unknown`, or `error`. A DNS timeout or a list error code gives `error`. An `error` never changes a stored `pass` or `fail` state.

### Blocklist Usage

- Spamhaus allows its public mirrors for small-scale, non-commercial use only. It returns `127.255.255.x` error codes to queries from large public resolvers. The monitor uses the Spamhaus Data Query Service (DQS) with a key from `RCH__REPUTATION__SPAMHAUS_DQS_KEY`. Without a key, the monitor skips Spamhaus and says so in the result.
- Each list has its own terms. Before release, the owner confirms the terms and that each list still runs. Remove a list that has closed.
- The list catalog moves from a constant into a versioned table in code (`blocklists_v1`). Each entry has the zone, the type (`ip` or `domain`), the removal URL, and the license note.

### Schedule

- A new scheduler `spawn_health_monitor_scheduler` runs when `health_monitor.enable` and `worker.enable` are true.
- Each asset has `next_check_at`. The scheduler claims due assets with `UPDATE ... SET next_check_at = NOW() + interval ... RETURNING`, as `reverification.rs` does.
- The default interval is 6 hours. The minimum is 1 hour.
- A short global cache (1 hour) keyed by list and subject stops two tenants that monitor the same domain from querying a list twice.

### State Changes And Alerts

- The monitor stores the current state of each check in `v1_asset_health_state`.
- It writes an alert only when a state changes:

| Alert type | When |
|---|---|
| `ip.blocklisted` | A sending IP moves to `fail` on a list. |
| `ip.delisted` | The IP shows `pass` on that list for 2 runs in a row. |
| `domain.blocklisted` | A sending domain moves to `fail` on a list. |
| `domain.delisted` | The domain shows `pass` on that list for 2 runs in a row. |
| `dns.record_failed` | An SPF, DKIM, or DMARC check moves from `pass` to `fail`. |
| `dns.record_fixed` | That check moves back to `pass`. |

- A listing alerts on the first run, because speed matters. A delisting waits for 2 clean runs, because some lists flap.
- When RFC 0023 ships, each alert also sends an `alert.raised` webhook event.

## Public API / Interface Changes

| Method | Path | Change |
|---|---|---|
| `PATCH` | `/v1/me/domains/{domain}` | New fields `monitor_enabled` and `dkim_selectors` (up to 10). |
| `GET` | `/v1/me/domains/{domain}/health` | New. The current state of each check, with the last check time. |
| `GET` | `/v1/me/domains/{domain}/health/history` | New. Past runs, newest first, with `limit` and `offset`. |
| `GET`, `POST` | `/v1/sending-ips` | New. List and add sending IPs. |
| `GET`, `PATCH`, `DELETE` | `/v1/sending-ips/{id}` | New. Read, change, and remove one IP. |
| `GET` | `/v1/sending-ips/{id}/health` | New. The current state of each check. |
| `POST` | `/v1/me/domains/{domain}/health/run` and `/v1/sending-ips/{id}/health/run` | New. Start one check now. One manual run for each asset every 15 minutes. |
| `GET` | `/v1/alerts` | New filter `subject_type` (`email`, `domain`, `ip`, `sender`). New response fields `subject_type` and `subject`. |

Example health response:

```json
{
  "subject_type": "ip",
  "subject": "203.0.113.25",
  "checked_at": "2026-10-21T08:00:00Z",
  "checks": [
    {
      "key": "blocklist.ip",
      "list": "spamcop",
      "state": "fail",
      "since": "2026-10-21T08:00:00Z",
      "answers": ["127.0.0.2"],
      "removal_url": "https://www.spamcop.net/bl.shtml"
    },
    { "key": "ptr.forward_confirmed", "state": "pass", "since": "2026-10-01T00:00:00Z" }
  ]
}
```

`POST /v1/reputation/check` keeps its current response. Its documentation says that `blacklist_results` is about the receiving MX host.

## Data Model / Storage Changes

All changes are in new migrations. Do not edit an applied migration.

```sql
ALTER TABLE tenant_domains
    ADD COLUMN monitor_enabled BOOLEAN NOT NULL DEFAULT false,
    ADD COLUMN dkim_selectors TEXT[] NOT NULL DEFAULT '{}',
    ADD COLUMN next_check_at TIMESTAMPTZ;

CREATE TABLE v1_sending_ips (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    ip INET NOT NULL,
    label TEXT,
    monitor_enabled BOOLEAN NOT NULL DEFAULT true,
    next_check_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (tenant_id, ip),
    CHECK (family(ip) = 4)
);

CREATE TABLE v1_asset_health_state (
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    subject_type TEXT NOT NULL CHECK (subject_type IN ('domain', 'ip')),
    subject TEXT NOT NULL,
    check_key TEXT NOT NULL,
    list_key TEXT NOT NULL DEFAULT '',
    state TEXT NOT NULL CHECK (state IN ('pass', 'warn', 'fail', 'unknown')),
    clean_streak SMALLINT NOT NULL DEFAULT 0,
    detail JSONB NOT NULL DEFAULT '{}'::jsonb,
    since TIMESTAMPTZ NOT NULL,
    checked_at TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (tenant_id, subject_type, subject, check_key, list_key)
);

CREATE TABLE v1_asset_health_runs (
    id BIGSERIAL PRIMARY KEY,
    tenant_id UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
    subject_type TEXT NOT NULL,
    subject TEXT NOT NULL,
    trigger TEXT NOT NULL CHECK (trigger IN ('schedule', 'manual')),
    result JSONB NOT NULL,
    checked_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_v1_asset_health_runs_subject
    ON v1_asset_health_runs (tenant_id, subject_type, subject, checked_at DESC);

ALTER TABLE v1_alerts
    ALTER COLUMN canonical_email DROP NOT NULL,
    ADD COLUMN subject_type TEXT NOT NULL DEFAULT 'email'
        CHECK (subject_type IN ('email', 'domain', 'ip', 'sender')),
    ADD COLUMN subject TEXT;
```

- A cleanup task deletes `v1_asset_health_runs` rows older than 90 days.
- The 1-hour lookup cache is a global table `blocklist_lookup_cache (list_key, subject, answer, expires_at)`, like `reputation_cache`. It has no tenant column and holds only public DNS answers.

## Auth, Permissions, And Tenant Isolation

- Writes need the `settings` scope, as the domain endpoints do today. The new health reads accept `settings` or `reputation`.
- Every query filters by `tenant_id`. A tenant cannot read another tenant's assets, health state, or alerts.
- The monitor does not need proof that the tenant owns a domain or IP, because it reads only public DNS data. Plan limits stop abuse:

| Plan | Monitored domains | Sending IPs |
|---|---|---|
| Free | 1 | 0 |
| Starter | 3 | 5 |
| Professional | 20 | 50 |
| Enterprise | Set by contract | Set by contract |

These limits are proposed defaults. The product owner confirms them before release.

## Failure Modes And Edge Cases

| Case | Behavior |
|---|---|
| A list returns an error code or times out | The check state is `error` for that run. The stored state does not change. No alert. |
| All lists fail for 24 hours | One `monitor.degraded` log event and a Sentry error. No tenant alert. |
| The DQS key is missing | Spamhaus checks show `unknown` with the reason `not_configured`. |
| A tenant removes an asset | Its state rows go. Its alerts stay for history. |
| A domain has no MX record | `blocklist.domain` still runs. `bulk_sender.dns` reports MX as `warn`. |
| IPv6 sending IP | Rejected in v1 with `400`. Most IP lists need a different query form for IPv6. |
| Many tenants monitor one popular domain | The 1-hour cache keeps the list queries for that domain at one each hour. |

## Alternatives Considered

1. **Read sending IPs from SPF.** Rejected. SPF `include` ranges often cover thousands of shared IPs.
2. **Extend `reputation_cache` for monitoring.** Rejected. A 24-hour cache is too slow for listing alerts.
3. **Alert on every run with a listing.** Rejected. It floods the tenant and teaches them to ignore alerts.

## Rollout / Migration / Compatibility

1. Ship the migrations and the scheduler behind `health_monitor.enable = false`.
2. Turn it on in staging with the DQS key. Watch query volume for one week.
3. Turn it on in production for internal tenants, then for all tenants.
4. Regenerate `backend/openapi.json`, the SDKs, and `web/lib/api/generated/`.

`v1_alerts.canonical_email` becomes nullable. The generated TypeScript type changes from `string` to `string | null`. Existing alerts keep their email. Tell API users in the changelog.

## Metrics And Success Criteria

- Share of paying tenants with one or more monitored assets.
- Median time from a listing to the alert. Target: under 7 hours with the default interval.
- Share of runs with `error` states. Target: under 2%.
- Count of alerts that a tenant dismisses within 1 minute. A high count means noisy alerts.

## Test Strategy

- Unit tests for each check with a stub resolver: SPF lookup count, DKIM selector states, DMARC policy parsing, PTR forward confirmation, and list answer classes.
- Unit tests for the state machine: first listing alerts, an error does not change state, delisting waits for 2 clean runs.
- An end-to-end test in `backend/tests/` with a stub DNS server and Postgres. It runs two scheduler cycles and checks the alert rows.
- API harness cases for each new endpoint, including `403` without the scope and `404` for another tenant's asset.
- Web tests for the domain health page and the sending IP form.

## Definition Of Done

1. All migrations run up and down. `e2e_migration_upgrade` passes.
2. The OpenAPI document, the SDKs, and the web client show the new endpoints. CI finds no diff after generation.
3. All tests in Test Strategy pass in CI.
4. A staging drill lists a test IP on a test zone. The alert appears within one interval.
5. The owner records the usage terms for each list in `docs/advanced/`.

## Open Questions

1. Which lists does the Spamhaus DQS free tier cover at our expected volume? Owner: backend lead.
2. Do we show health on the domains page or on a new page? Owner: web lead.
3. Do we email the tenant about a listing, or only show it in the app? Owner: product owner.

## Unresolved Risks

1. **List terms.** Some lists forbid commercial use without a license. We remove any list we cannot license.
2. **False confidence.** A clean result does not mean good inbox placement. The page must say this.
3. **Query volume.** Many assets can exceed free list quotas. The cache and the minimum interval limit this.
