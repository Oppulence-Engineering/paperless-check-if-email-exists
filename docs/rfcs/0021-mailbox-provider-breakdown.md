# RFC 0021: Mailbox Provider Breakdown For Each Campaign

## Verify disposition — 2026-10-23

**Optional attribution fix; campaign reporting deferred.** The [MillionVerifier adapter](../../backend/src/verification.rs) detects provider before MX lookup without a subsequent MX-based detection pass. That narrow evidence correction can be promoted independently. Full campaign reports require customer demand and outcome data; neither reports nor attribution are a prerequisite for the core paid-audit experiment.

[Current build order](README.md) · [Codebase audit](../audits/2026-10-23-verify-north-star.md). This disposition supersedes conflicting historical scope, priority, or baseline statements below; it does not certify every original acceptance criterion.

| | |
|---|---|
| **Status** | Optional attribution fix; campaign reporting deferred; audited 2026-10-23 |
| **Created** | 2026-10-21 |
| **Owners** | To assign: backend lead, web lead |
| **Parent** | RFC 0016, item 5 (backlog item 81) |
| **Depends on** | None. RFC 0018 adds bounce classes to the report. RFC 0022 adds human-only engagement. |
| **Used by** | RFC 0022 (the `mail_gateway` field), RFC 0024 (an MCP tool), RFC 0026 (Gmail rows) |
| **Release gates** | Provider attribution fixed for the MillionVerifier path; Definition Of Done met |

## Summary

This RFC shows delivery, bounce, complaint, and engagement counts for each mailbox provider, such as Gmail, Microsoft 365, and Yahoo. The report works for one campaign or for a period. It also fixes a gap: in managed deployments, results for company domains have no mailbox provider today.

## Motivation / Problem

- A sender's problem is often with one provider. For example, Microsoft blocks the sender while Gmail accepts the mail. A total bounce rate hides this.
- We already have the data: the provider of each verified address, and the feedback events with a `campaign_id`. Nothing joins them.
- In managed staging and production, verification goes through MillionVerifier. That path calls `detect_provider(domain, None)` (`backend/src/verification.rs`). It finds the provider only for known consumer domains. A company on Google Workspace or Microsoft 365 gets no provider.
- The provider sits inside the `v1_task_result.result` JSON. A report over many rows must not parse JSON for each row.

## Goals

- Detect the mailbox provider from the MX host on both verification paths.
- Store the provider and the mail gateway as columns.
- Attribute each feedback event to a provider, even when we did not verify the address.
- Report counts and rates for each provider, with honest denominators and data coverage.

## Non-Goals

- Inbox placement (inbox or spam folder). We see only delivery events.
- Per-IP reports. RFC 0026 imports provider data by IP.
- Changes to the provider list in core, except the gateway helper.

## Current State In Reacher

- Core detects the provider in two steps (`core/src/provider/mod.rs`, `detect_provider`): first a domain rule (High confidence), then the MX host. The default path (`core/src/lib.rs`) runs the MX step after the MX lookup. The MillionVerifier path runs `check_mx` after the provider step, and never runs the MX step.
- `Provider` values: `gmail`, `google_workspace`, `outlook_consumer`, `microsoft365`, `yahoo`, `apple_icloud`, `proton`, `zoho`.
- Core also detects security gateways from the MX host: `is_mimecast` (`.mimecast.com.`) and `is_proofpoint` (`.pphosted.com.`, `.ppe-hosted.com.`) in `core/src/mx/mod.rs`. Only the SMTP path uses them.
- Feedback lives in two stores:
  - `verification_outcomes`: campaign outcomes (`/v1/campaign-outcomes`), with `campaign_id` and an index on `(tenant_id, campaign_id)`.
  - `v1_contact_outcomes`: `POST /v1/outcomes` and native provider webhooks, with `campaign_id` but no campaign index.
- The source quality report (`http/v1/source_quality.rs`) already joins both stores and removes duplicates by source, mailbox, event type, and time.

## Proposed Design

### Fix Provider Detection

In `verify_task` (`backend/src/verification.rs`), after `check_mx` returns, call `detect_provider(&syntax.domain, Some(lowest_preference_mx_host))` when the first call returned `None`. This matches the default path. It adds no network call, because the MX lookup already ran.

### Mail Gateway

Add `detect_mail_gateway(mx_host) -> Option<MailGateway>` to core next to `is_mimecast` and `is_proofpoint`:

| Gateway | MX host suffix |
|---|---|
| `mimecast` | `.mimecast.com.` |
| `proofpoint` | `.pphosted.com.`, `.ppe-hosted.com.` |
| `barracuda` | `.barracudanetworks.com.` |
| `cisco` | `.iphmx.com.` |

Confirm each suffix with a real MX record in a test fixture before release. A gateway sits in front of the real mailbox provider, so a result can have both a provider and a gateway.

### Store As Columns

- `v1_task_result` gets `mailbox_provider` and `mail_gateway`. The worker sets them when it stores a result.
- A background job fills them for old rows from `result->>'provider'`, in batches. It does not run in a migration, because the table is large.

### Attribute Each Feedback Event

For each event, the report finds the provider in this order:

1. The linked verification (`task_result_id`), when the event has one.
2. The latest verification of the same tenant and mailbox.
3. The domain cache `mailbox_provider_domains` (below).
4. The domain rule only (`detect_provider(domain, None)`).
5. Otherwise `unresolved`.

The domain cache is a global table, like `reputation_cache`. It holds one row for each domain: provider, gateway, MX host, and expiry (7 days). A background task resolves domains that appear in feedback events with no verification. It runs MX lookups at a fixed rate (default 20 each second). The report shows `unresolved` until the task finishes.

### Report Rows

For each provider (and optionally each gateway):

| Field | Meaning |
|---|---|
| `delivered` | Delivery events. |
| `bounced` | Recipient-permanent bounces. |
| `deferred`, `sender_rejected` | After RFC 0018. Null before it. |
| `complained`, `unsubscribed` | Negative feedback events. |
| `opened`, `clicked` | Engagement events. After RFC 0022, human events only, with machine counts in separate fields. |
| `attempted` | `delivered + bounced + sender_rejected`. |
| `bounce_rate` | `bounced / attempted`. Null when `attempted` is under 100. |
| `complaint_rate` | `complained / delivered`. Null when `delivered` is under 100, or when complaint data does not exist for the provider. |
| `complaint_data` | `available`, `partial`, or `not_reported`. |

### Complaint Data Is Not Equal

- Gmail does not send a complaint report for each message to the sender or the ESP. Gmail gives spam rates only in aggregate, in Postmaster Tools. So our Gmail and Google Workspace rows show `complaint_data = "not_reported"` and no `complaint_rate`. RFC 0026 fills this gap with Postmaster data.
- Yahoo and Microsoft run feedback loops for each message. ESPs pass some of these on as complaint events. These rows show `available` when the ESP endpoint sends complaint events, and `partial` otherwise.
- The report shows the Gmail and Yahoo spam-rate limits (0.1% target, 0.3% maximum) only next to data that can measure them.

This corrects RFC 0016 item 5, which said that the report marks the Gmail limits on our own complaint data.

### Duplicates

The report uses the same rule as the source quality report to remove an event that is in both stores. That rule moves into one shared SQL function, so the two reports cannot drift apart.

## Public API / Interface Changes

| Method | Path | Change |
|---|---|---|
| `GET` | `/v1/campaigns` | New. Campaign IDs from both stores, with event counts and the first and last event time. `limit`, `offset`, `since`. |
| `GET` | `/v1/campaigns/{campaign_id}/provider-breakdown` | New. Report rows for one campaign. Optional `group_by_gateway=true`. |
| `GET` | `/v1/reports/provider-breakdown` | New. Report rows for a period: `since`, `until`, optional `source_key` and `endpoint_id`. |
| Verification results | All result endpoints | New fields `mailbox_provider` and `mail_gateway`. |

Scope: `outcomes.read` for the reports. Result endpoints keep their scopes.

## Data Model / Storage Changes

```sql
ALTER TABLE v1_task_result
    ADD COLUMN mailbox_provider TEXT,
    ADD COLUMN mail_gateway TEXT;

CREATE TABLE mailbox_provider_domains (
    domain TEXT PRIMARY KEY,
    provider TEXT,
    mail_gateway TEXT,
    mx_host TEXT,
    resolved_at TIMESTAMPTZ NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL
);

-- no-transaction
CREATE INDEX CONCURRENTLY idx_v1_contact_outcomes_campaign
    ON v1_contact_outcomes (tenant_id, campaign_id, occurred_at)
    WHERE campaign_id IS NOT NULL;
```

`mailbox_provider_domains` has no tenant column. It holds only public DNS facts about a domain, like `reputation_cache`.

## Auth, Permissions, And Tenant Isolation

- Reports read only the tenant's own events and results.
- The domain cache is shared, but it holds no tenant data. A tenant cannot learn which domains another tenant mails from the cache, because the API never lists it.

## Failure Modes And Edge Cases

| Case | Behavior |
|---|---|
| The tenant sends no `delivered` events | Rates are null. The report says "No delivery events. Turn on delivered events in your ESP." |
| A campaign has fewer than 100 attempts for a provider | Counts show. Rates are null. |
| The MX lookup fails during verification | Provider stays from the domain rule, or null. The result is not changed in any other way. |
| A domain moves from one provider to another | The domain cache expires after 7 days. Old verifications keep the old provider, which was true at that time. |
| An event has no `campaign_id` | It counts in the period report, not in a campaign report. |

## Alternatives Considered

1. **Parse `result` JSON in the report query.** Rejected. It is slow over millions of rows.
2. **Run an MX lookup for each event at ingestion.** Rejected. A 10,000-row batch would wait on DNS. The background task does the same work without delay.
3. **Show Gmail complaint rates from ESP events.** Rejected. Gmail does not report complaints for each message, so the rate would look good when it is not.

## Rollout / Migration / Compatibility

1. Release the provider detection fix and the gateway helper. New results get the provider.
2. Release the migration and the store change. Start the backfill job.
3. Release the domain resolver and the report endpoints behind `provider_reports.enable`.
4. Release the web page: a campaign picker, then a table with one row for each provider. The table comes first on the page. Filters sit in a sheet.
5. Regenerate the OpenAPI document and the SDKs.

The new result fields are additive.

## Metrics And Success Criteria

- Share of new results with a provider, for company domains. Target: 90% or more on both paths.
- Share of feedback events with a resolved provider after 24 hours. Target: 95% or more.
- p95 time for a campaign report with 1 million events. Target: under 2 seconds.

## Test Strategy

- Unit tests for `detect_mail_gateway` with recorded MX hosts.
- A unit test for the MillionVerifier path: a Google Workspace domain gets `google_workspace` after the MX lookup.
- Database tests for attribution order, duplicate removal across both stores, and the 100-event rate limit.
- An end-to-end test: ingest events for two providers and check the report rows.
- A query plan test on a seeded table with 1 million events.
- Web tests for the campaign picker and the empty states.

## Definition Of Done

1. The migrations run up and down. `e2e_migration_upgrade` passes.
2. The backfill job finishes on a production-size copy.
3. The OpenAPI document, the SDKs, and the web client show the new endpoints and fields. CI finds no diff.
4. All tests pass, and the report meets the latency target.
5. The page explains why Gmail rows have no complaint rate.

## Open Questions

1. Do we show `apple_icloud`, `proton`, and `zoho` as rows, or put them in `other` when they are small? Owner: web lead.
2. Do we add more gateways (for example Microsoft Defender) when we cannot see them in MX records? Owner: backend lead.

## Unresolved Risks

1. **Missing campaign IDs.** Some ESPs do not pass a campaign ID in events. The setup guide shows how to add it as a custom field.
2. **Provider rules age.** MX host patterns change. The domain cache expiry and the rules tests limit the damage.
