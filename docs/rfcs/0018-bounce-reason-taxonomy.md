# RFC 0018: Bounce Reason Taxonomy

## Verify disposition — 2026-10-23

**Conditional correctness extension.** Promote the recipient-classification slice only when actual feedback causes incorrect exclusions in customer audits. Existing complaint/suppression safeguards remain mandatory. Sender alerts depend on RFC 0017 and stay deferred; taxonomy is not a blocker for starting rights-cleared paid audits.

[Current build order](README.md) · [Codebase audit](../audits/2026-10-23-verify-north-star.md). This disposition supersedes conflicting historical scope, priority, or baseline statements below; it does not certify every original acceptance criterion.

| | |
|---|---|
| **Status** | Conditional correctness extension; audited 2026-10-23 |
| **Created** | 2026-10-21 |
| **Owners** | To assign: backend lead |
| **Parent** | RFC 0016, item 2 |
| **Depends on** | RFC 0017 (alerts with a non-email subject) for sender alerts |
| **Used by** | RFC 0021 (provider breakdown), RFC 0025 (platform bounce events) |
| **Release gates** | Recorded payload fixtures for each provider; dry-run report reviewed; Definition Of Done met |

## Summary

This RFC gives each bounce a class and a disposition. The class says why the mail failed. The disposition says who must act: the recipient address is bad, the failure is temporary, or the sender has a problem. Only recipient problems suppress an address. Sender problems raise a sender alert.

## Motivation / Problem

- Native provider webhooks keep only permanent bounces. They drop soft bounces, deferrals, and blocks (`provider_outcomes.rs`). We lose the signal that a mailbox is full, or that a provider blocks the sender.
- ESPs mark some sender problems as permanent bounces. An example is a `5.7.1` rejection for spam content or for a blocklisted IP. Today these events suppress a good recipient address.
- The two feedback stores use different rules. Campaign outcomes use the tenant's outcome policy, with `hard_bounce` and `soft_bounce` rules. Provider webhooks and `POST /v1/outcomes` use a fixed map, and they ignore the policy.

## Goals

- Classify every bounce event from every source with one versioned rule set.
- Record temporary and sender-side bounces, and do not suppress for them.
- Apply the tenant's outcome policy to all feedback paths, with one rule for each class.
- Raise a sender alert when many sender-side bounces arrive in a short time.
- Show the tenant which current suppressions the new rules would not create.

## Non-Goals

- We do not retry mail. The ESP retries deferred mail.
- We do not parse full non-delivery report (NDR) messages from a mailbox.
- We do not remove existing suppressions automatically.

## Current State In Reacher

- **Provider webhooks** (`backend/src/http/v1/provider_outcomes.rs`) normalize events into `v1_contact_outcomes` with `event_type = 'bounced'`:
  - SendGrid: `bounce` events become `bounced`. Events with `type: blocked` are dropped. `deferred` and `dropped` events are dropped.
  - Mailgun: `failed` with `severity: permanent` becomes `bounced`. Temporary failures are dropped.
  - Postmark: `HardBounce`, `BadEmailAddress`, or `Inactive: true` become `bounced`. Other bounce types are dropped.
  - SES: `bounceType: Permanent` becomes `bounced`. `Transient` and `Undetermined` are dropped.
- The raw provider event is kept in `v1_contact_outcomes.metadata`. The raw request is kept in `v1_provider_outcome_receipts.raw_payload`. So we can classify past events again.
- **Generic outcomes** (`backend/src/http/v1/outcomes.rs`) accept `bounced` with no detail. `suppression_reason_for_event` maps `bounced` to the `bounce` suppression reason for every row.
- **Campaign outcomes** (`backend/src/outcomes.rs`, `/v1/campaign-outcomes`) store `hard_bounce` and `soft_bounce` in `verification_outcomes`. The policy suppresses a hard bounce now and a soft bounce after 3 in 30 days.

## Proposed Design

### Classes And Dispositions

| Class | Meaning | Disposition | Default policy action |
|---|---|---|---|
| `no_such_user` | The mailbox does not exist. | `recipient_permanent` | Suppress now |
| `mailbox_disabled` | The provider disabled the mailbox. | `recipient_permanent` | Suppress now |
| `domain_not_found` | The domain has no mail server, or DNS fails for a long time. | `recipient_permanent` | Suppress now |
| `mailbox_full` | The mailbox is over quota. | `recipient_transient` | Suppress after 3 in 14 days |
| `transient` | Another temporary failure. | `recipient_transient` | Count only |
| `rate_limited` | The provider slows the sender down. | `sender_side` | Sender alert |
| `reputation_block` | The provider blocks the sender's IP or domain. | `sender_side` | Sender alert |
| `auth_failure` | SPF, DKIM, or DMARC failed. | `sender_side` | Sender alert |
| `policy_block` | The provider rejects the content or the message policy. | `sender_side` | Sender alert |
| `esp_suppressed` | The ESP did not send because of its own suppression list. | `recipient_permanent` | Suppress now |
| `unknown` | No rule matched. | `unknown` | Use the legacy hard or soft rule |

### Rule Order

The classifier `classify_bounce(provider, event) -> BounceClassification` uses the first rule that matches:

1. **Structured provider fields.** Examples: SES `bounceType` and `bounceSubType`; Postmark `Type`; SendGrid `type`; Mailgun `severity` and `reason`.
2. **The enhanced status code** (RFC 3463), from SES `diagnosticCode`, SendGrid `status`, Mailgun `delivery-status`, or Postmark `Details`.
3. **Text patterns** in the diagnostic message. The pattern list is versioned with the rule set.

Starting map for enhanced codes:

| Code | Class |
|---|---|
| `5.1.1`, `5.1.10` | `no_such_user` |
| `5.2.1` | `mailbox_disabled` |
| `4.2.2`, `5.2.2` | `mailbox_full` |
| `5.1.2`, `5.4.4` | `domain_not_found` |
| `4.7.28`, `4.7.0` with a rate text | `rate_limited` |
| `5.7.26`, `5.7.27` | `auth_failure` |
| `5.7.1` with a blocklist text (for example `spamhaus`, `blocklist`, `blacklist`) | `reputation_block` |
| Other `5.7.x` | `policy_block` |
| Other `4.x.x` | `transient` |

Starting map for structured fields (confirm each with a recorded fixture):

| Provider | Field value | Class |
|---|---|---|
| SES | `Permanent` / `General` or `NoEmail` | `no_such_user` (then refine with the code) |
| SES | `Permanent` / `Suppressed` or `OnAccountSuppressionList` | `esp_suppressed` |
| SES | `Transient` / `MailboxFull` | `mailbox_full` |
| SES | `Transient` / `ContentRejected` or `AttachmentRejected` | `policy_block` |
| SES | `Transient` / other, or `Undetermined` | `transient` |
| SendGrid | `bounce` with `type: blocked` | `reputation_block` or `policy_block`, from the code |
| SendGrid | `deferred` | `transient`, then refine with the code |
| Mailgun | `failed`, `severity: temporary` | `transient`, then refine with the code |
| Mailgun | `failed`, `reason: suppress-bounce` | `esp_suppressed` |
| Postmark | `HardBounce`, `BadEmailAddress` | `no_such_user` |
| Postmark | `SoftBounce`, `Transient` | `transient` |
| Postmark | `DnsError` | `transient` (a long DNS failure needs a code to become `domain_not_found`) |
| Postmark | `Blocked` | `reputation_block` |
| Postmark | `DMARCPolicy` | `auth_failure` |
| Postmark | `ManuallyDeactivated` | `esp_suppressed` |

The rule set has a version, `bounce_rules_v1`. Each classified row stores the version.

### Recording

- `bounced` keeps its meaning: a recipient-permanent bounce. Existing consumers see no change.
- Two new event types record what we drop today:
  - `deferred`: a `recipient_transient` event.
  - `sender_rejected`: a `sender_side` event.
- `event_family` stays `delivery` for all three types.

### Policy

- `OutcomePolicyRules` gains a `bounce_classes` map from class to `RuleAction`. The defaults are in the table above.
- The `hard_bounce` and `soft_bounce` rules stay. They apply only when the class is `unknown`.
- `POST /v1/outcomes` and provider webhooks start to use the tenant's policy, as campaign outcomes do. The default policy gives the same result as today for permanent bounces.
- `SuppressAfter` counts events of the same class for the same mailbox in the window.

### Sender Alerts

- A rolling check runs each 15 minutes. It counts `sender_rejected` events for each tenant, provider endpoint, and class in the last hour.
- It raises a `sender.blocked` alert (RFC 0017 alert shape, `subject_type = 'sender'`) when both are true:
  - 20 or more events.
  - 2% or more of the delivery events for that endpoint in the same hour.
- It raises the alert again only after 6 hours with no alert for the same key.
- The alert lists the top diagnostic messages and the recipient mailbox providers.

### Dry-Run Report

- A one-time job classifies all past `bounced` rows with the new rules.
- It lists active suppressions that came from a row that is now `sender_side`.
- The tenant reviews the list and revokes entries in bulk with the existing revoke endpoint. Nothing changes without the tenant.

## Public API / Interface Changes

| Method | Path | Change |
|---|---|---|
| `POST` | `/v1/outcomes` | Accepts `deferred` and `sender_rejected`. Accepts optional `bounce_class`, `smtp_status` (for example `5.1.1`), and `diagnostic` for each row. |
| `POST` | `/v1/campaign-outcomes` and `/upload` | Accept the same optional fields. |
| `GET` | `/v1/outcomes` | New filters `bounce_class` and `bounce_disposition`. New response fields `bounce_class`, `bounce_disposition`, `smtp_status`, and `bounce_rule_version`. |
| `GET` | `/v1/outcomes/bounce-summary` | New. Counts by class and disposition, with `since`, `until`, `source_key`, `campaign_id`, and `endpoint_id`. |
| `GET`, `PATCH` | `/v1/outcome-policies/{id}` | Rules accept `bounce_classes`. |
| `GET` | `/v1/suppressions/reclassification-report` | New. The dry-run list, with `limit` and `offset`. |

If a caller sends `bounce_class`, we trust it. If the caller sends only `smtp_status` or `diagnostic`, we classify. If the caller sends nothing, the class is `unknown`.

## Data Model / Storage Changes

```sql
ALTER TABLE v1_contact_outcomes
    ADD COLUMN bounce_class TEXT,
    ADD COLUMN bounce_disposition TEXT
        CHECK (bounce_disposition IN ('recipient_permanent', 'recipient_transient', 'sender_side', 'unknown')),
    ADD COLUMN smtp_status TEXT,
    ADD COLUMN bounce_rule_version TEXT;

ALTER TABLE verification_outcomes
    ADD COLUMN bounce_class TEXT,
    ADD COLUMN bounce_disposition TEXT,
    ADD COLUMN smtp_status TEXT,
    ADD COLUMN bounce_rule_version TEXT;

CREATE INDEX idx_v1_contact_outcomes_sender_side
    ON v1_contact_outcomes (tenant_id, endpoint_id, occurred_at DESC)
    WHERE bounce_disposition = 'sender_side';
```

- `outcome_type` is a Postgres enum. We do not add values to it. Campaign rows store the class in the new column, and `hard_bounce` or `soft_bounce` stays in `outcome_type`.
- A background job fills the new columns for past rows from `metadata`, in batches of 5,000.

## Auth, Permissions, And Tenant Isolation

- Each existing endpoint keeps its scope. `/v1/outcomes` uses `suppressions` today. Campaign outcomes use `outcomes.read` and `outcomes.write`. Outcome policies use `settings`.
- The new `bounce-summary` endpoint needs `outcomes.read`. The reclassification report needs `suppressions`.
- The classifier is a pure function. It reads only the event it gets.
- The sender alert counts only the tenant's own events.

## Failure Modes And Edge Cases

| Case | Behavior |
|---|---|
| A provider changes its payload shape | The class becomes `unknown`. The legacy rule applies. A metric counts `unknown` rates for each provider. |
| A `5.7.1` message names both a bad user and a block | Structured fields win. If none exist, `reputation_block` wins over `no_such_user`, because a false suppression costs more. |
| Event order: a `deferred` after a `bounced` | Both are kept. The suppression from `bounced` stays. |
| A burst of `sender_rejected` from one bad campaign | One alert for each key every 6 hours. |
| The caller sends a bad `bounce_class` | `400` with the list of valid classes. |

## Alternatives Considered

1. **Add enum values to `outcome_type`.** Rejected. Enum changes need care in migrations, and campaign rows already carry hard or soft.
2. **Suppress on any permanent bounce, as today.** Rejected. It suppresses good addresses when the sender has a reputation problem.
3. **Machine learning on diagnostic text.** Rejected for v1. Codes and fields cover most events, and rules are easy to audit.

## Rollout / Migration / Compatibility

1. Ship the classifier and the columns. Classify and store, but keep the old suppression map (`bounce_rules.enforce = false`).
2. Run the backfill and the dry-run report in staging and production. Review the share of each class.
3. Turn on enforcement for internal tenants, then for all tenants.
4. Regenerate the OpenAPI document and the SDKs.

Existing `GET /v1/outcomes` clients that filter on `event_type = 'bounced'` see the same rows as before. Clients that list all events see two new event types. The changelog says this.

## Metrics And Success Criteria

- Share of bounce events with a known class. Target: 95% or more for each native provider.
- Count of suppressions avoided because the bounce was `sender_side`.
- Time from the first `sender_rejected` burst to the alert. Target: under 20 minutes.

## Test Strategy

- Table-driven unit tests for `classify_bounce`, with one recorded fixture for each row of the structured-field table. Fixtures live in `backend/tests/fixtures/bounces/`.
- Unit tests for the policy: each class gives the expected action. `unknown` falls back to the legacy rules.
- End-to-end tests for each provider webhook:
  - A hard bounce suppresses.
  - A block does not suppress. It is stored as `sender_rejected`.
  - A full mailbox suppresses after 3 events.
- A test for the sender alert threshold and the 6-hour quiet period.
- A test that the backfill job is safe to run twice.

## Definition Of Done

1. All migrations run up and down. `e2e_migration_upgrade` passes.
2. The classifier has a fixture for every row in the starting maps.
3. The OpenAPI document, the SDKs, and the web client show the new fields. CI finds no diff.
4. The dry-run report runs on a copy of production data. The owner reviews the result before enforcement starts.
5. The outcome page in the web app shows the class and disposition for each bounce.

## Open Questions

1. Do we show sender alerts to every tenant member, or only to admins? Owner: product owner.
2. Is 2% of deliveries the right sender alert threshold? Owner: backend lead, after the dry run.

## Unresolved Risks

1. **Text patterns age.** Providers change their messages. The `unknown` metric shows when the rules need an update.
2. **Missing delivery events.** Without `delivered` events, the 2% threshold cannot work. Then only the count of 20 applies, and the alert says so.
