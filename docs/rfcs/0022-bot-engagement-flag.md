# RFC 0022: Bot Engagement Flag

## Verify disposition — 2026-10-23

**Deferred; sender-demand and fixture gate.** Machine engagement classification needs RFC 0021 gateway attribution and representative provider fixtures. It does not establish identity or role correctness and is outside the initial audit path. Revisit if engagement-driven policies or sender reporting create a measured customer problem.

[Current build order](README.md) · [Codebase audit](../audits/2026-10-23-verify-north-star.md). This disposition supersedes conflicting historical scope, priority, or baseline statements below; it does not certify every original acceptance criterion.

| | |
|---|---|
| **Status** | Deferred; sender-demand and fixture gate; audited 2026-10-23 |
| **Created** | 2026-10-21 |
| **Owners** | To assign: backend lead |
| **Parent** | RFC 0016, item 6 |
| **Depends on** | RFC 0021 (`mail_gateway` on results and the domain cache) |
| **Release gates** | Recorded open and click fixtures for each native provider; Definition Of Done met |

## Summary

This RFC marks opens and clicks that a machine made, not a person. Security gateways scan links before a person sees the email. Privacy proxies load images ahead of time. The backend marks these events with `is_machine`, a rule name, and a confidence. Engagement counts, score boosts, and reports then use human events only.

## Motivation / Problem

- Mail security gateways (for example Mimecast and Proofpoint) open links in each message to scan them. Mail privacy proxies load the tracking image for every message. These events look like real engagement.
- Machine events make a bad source look good in the source quality report.
- The default outcome policy raises the score of an address on each open (+3) and each click (+8) (`default_outcome_policy_rules` in `backend/src/outcomes.rs`). A scanner can raise the score of an address that no person reads.
- Senders build segments of "engaged" readers. Machine events put unread addresses into these segments.

## Goals

- Mark machine opens and clicks at ingestion, with a rule name and a confidence.
- Mark each verified address that sits behind a known security gateway.
- Exclude machine events from score boosts, source quality engagement counts, and RFC 0021 engagement rates.
- Keep all events. Show machine counts next to human counts.

## Non-Goals

- A redirect service for links. We do not own the sender's links.
- Paid IP-reputation feeds for scanners.
- Proof that an event came from a person. The flag is a best guess with a confidence.

## Current State In Reacher

- `v1_contact_outcomes` stores `opened` and `clicked` events from native webhooks and from `POST /v1/outcomes`. The full raw provider event is in `metadata`. Each row has `provider_message_id` when the provider sends it.
- `verification_outcomes` stores `open` and `click` events from campaign outcomes, with `metadata`.
- The source quality report counts `opened` and `clicked` from both stores (`http/v1/source_quality.rs`).
- Core detects Mimecast and Proofpoint MX hosts. RFC 0021 adds `mail_gateway` to each result.

## Proposed Design

### Rules

The classifier `classify_engagement(event, context) -> EngagementClass` runs for each `opened` and `clicked` event. The first matching rule wins.

| Rule | Event | Condition | Confidence |
|---|---|---|---|
| `provider_flag` | Open | The provider marks the open as a machine open. Example: SendGrid `sg_machine_open: true`. | High |
| `link_burst` | Click | 3 or more different links in the same message are clicked within 2 seconds. | High |
| `fast_click` | Click | The click comes less than 10 seconds after the `delivered` event of the same message. | Medium |
| `gateway_fast_click` | Click | The address has a `mail_gateway`, and the click comes less than 60 seconds after delivery. | Medium |
| `scanner_user_agent` | Open or click | The user agent matches the scanner list `scanner_agents_v1`. | Medium |
| `gateway_address` | Open or click | The address has a `mail_gateway`, and no other rule matched. | Low |

- Events with High or Medium confidence count as machine events.
- Low confidence events stay human in counts. They are marked so that reports can show them.
- Confirm each provider field (for example `sg_machine_open`) with a recorded payload in a test fixture. Remove a rule that a provider does not support.
- The rule set has a version, `engagement_rules_v1`. Each classified row stores the version.

### Late Events

Webhooks do not arrive in order. A click can arrive before its `delivered` event. So:

1. At ingestion, the classifier runs with the events it has.
2. A background task runs each 10 minutes. It classifies again each click from the last 2 hours that has a `provider_message_id` and no `delivered` match at first.
3. After 2 hours, the class is final.

### Where Machine Events Are Excluded

| Place | Change |
|---|---|
| Outcome policy score boosts | `open` and `click` boosts apply to human events only. |
| Source quality report | `opened_count` and `clicked_count` count human events. New fields `machine_opened_count` and `machine_clicked_count`. |
| RFC 0021 provider report | `opened` and `clicked` count human events. New fields `machine_opened` and `machine_clicked`. |
| Email history | Machine events stay in the timeline with a "machine" label. |
| Saved segments | A new filter `human_engagement_since_days`. Existing filters do not change. |

### Address Flag

Each verification result shows `mail_gateway` (from RFC 0021). The decision engine adds an info reason `security_gateway_engagement_unreliable` when a gateway exists. It does not change the decision. It tells the sender that opens and clicks for this address are not a reliable signal.

## Public API / Interface Changes

| Method | Path | Change |
|---|---|---|
| `GET` | `/v1/outcomes` | New fields `is_machine`, `machine_rule`, `machine_confidence`. New filter `is_machine`. |
| `POST` | `/v1/outcomes` | Optional `is_machine` for each row, for senders that detect machine events themselves. We keep their value and set `machine_rule = 'caller'`. |
| `GET` | `/v1/sources/quality` | New machine count fields. |
| `GET` | `/v1/emails/{email}/history` | Events carry `is_machine`. |
| `POST`, `PATCH` | `/v1/segments` | New filter `human_engagement_since_days`. |

## Data Model / Storage Changes

```sql
ALTER TABLE v1_contact_outcomes
    ADD COLUMN is_machine BOOLEAN,
    ADD COLUMN machine_rule TEXT,
    ADD COLUMN machine_confidence TEXT CHECK (machine_confidence IN ('high', 'medium', 'low')),
    ADD COLUMN engagement_rules_version TEXT,
    ADD COLUMN engagement_final_at TIMESTAMPTZ;

ALTER TABLE verification_outcomes
    ADD COLUMN is_machine BOOLEAN,
    ADD COLUMN machine_rule TEXT,
    ADD COLUMN machine_confidence TEXT CHECK (machine_confidence IN ('high', 'medium', 'low')),
    ADD COLUMN engagement_rules_version TEXT;

-- no-transaction
CREATE INDEX CONCURRENTLY idx_v1_contact_outcomes_message
    ON v1_contact_outcomes (tenant_id, provider, provider_message_id, event_type)
    WHERE provider_message_id IS NOT NULL;
```

- `is_machine` is null for non-engagement events and for engagement events that are not classified yet.
- Campaign outcomes rarely carry message IDs or user agents. For them, only `gateway_address` and `caller` usually apply.
- A backfill job classifies past engagement events from `metadata`, newest first, for the last 90 days.

## Auth, Permissions, And Tenant Isolation

- No new scopes. Each endpoint keeps its current scope. For example, `/v1/outcomes` uses `suppressions`, and campaign outcomes use `outcomes.read`.
- The classifier reads only the tenant's own events.
- The scanner user agent list is global and holds no tenant data.

## Failure Modes And Edge Cases

| Case | Behavior |
|---|---|
| The ESP does not send `delivered` events | The timing rules cannot run. Only `provider_flag`, `link_burst`, `scanner_user_agent`, and `gateway_address` apply. |
| A person clicks fast on a phone | `fast_click` can mark a real click. The 10-second limit keeps this rare. The tenant sees machine counts, not hidden data. |
| A message has one link | `link_burst` cannot apply. Other rules still run. |
| A caller sends `is_machine` | We keep it and do not run our rules on that row. |
| A rule set changes | New rows use the new version. Old rows keep their version unless a backfill runs again. |

## Alternatives Considered

1. **Delete machine events.** Rejected. Senders need to see them, and a rule can be wrong.
2. **Mark every event behind a gateway as machine.** Rejected. People behind gateways also read mail. This is the Low confidence rule only.
3. **Buy an IP list of scanners.** Rejected for v1. It adds cost and license terms. The timing rules cover most cases.

## Rollout / Migration / Compatibility

1. Ship the columns and the classifier. Classify, but keep the old counts (`engagement_rules.enforce = false`).
2. Run the backfill. Compare human and machine counts for internal tenants.
3. Turn on enforcement. Score boosts and reports use human events.
4. Regenerate the OpenAPI document and the SDKs.

When enforcement starts, some engagement counts go down. The changelog and the source quality page say why.

## Metrics And Success Criteria

- Share of engagement events with a class after 2 hours. Target: 99%.
- Share of clicks marked as machine for addresses behind a gateway, compared with other addresses. We expect a clear gap. If there is none, the rules need work.
- Count of tenant reports about wrong marks.

## Test Strategy

- Table-driven unit tests for each rule, with recorded open and click payloads for SendGrid, Mailgun, Postmark, and SES.
- Tests for late events: a click before its `delivered` event gets its final class after the re-run.
- Tests that score boosts skip machine events.
- Tests that the source quality and provider reports split human and machine counts.
- A test that the backfill is safe to run twice.

## Definition Of Done

1. The migrations run up and down. `e2e_migration_upgrade` passes.
2. Each native provider has engagement fixtures, and each rule has a test.
3. The OpenAPI document, the SDKs, and the web client show the new fields. CI finds no diff.
4. The history page labels machine events. The source quality page shows both counts.

## Open Questions

1. Do we let a tenant turn off a rule (for example `fast_click`)? Owner: product owner.
2. Do we treat Apple Mail Privacy Protection opens as machine opens when the provider does not flag them? Owner: backend lead.

## Unresolved Risks

1. **Heuristics.** The rules are not exact. Each mark carries a confidence, and we keep all events.
2. **Provider changes.** A provider can drop a field such as `sg_machine_open`. The fixtures and the class share metric show this.
