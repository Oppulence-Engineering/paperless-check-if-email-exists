# RFC 0016: Deliverability Intelligence Roadmap

| | |
|---|---|
| **Status** | Draft |
| **Created** | 2026-10-19 |
| **Owners** | To assign: backend lead, web lead |
| **Builds on** | RFC 0001 (Provider Webhooks), RFC 0004 (Suppression Intelligence), RFC 0007 (Policy Modes) |
| **Release gates** | One RFC for each item before build starts |

## Summary

This RFC lists ten features for the roadmap. Each feature helps a sender keep a clean list and a good sender reputation. Each section is one paragraph. Each feature gets its own full RFC before build starts.

We compared our backend with the list of things that a newsletter platform must run to get mail to the inbox. Most of that list is about sending, and we do not send email. These ten items are the parts that we can own without sending.

## Out Of Scope

We do not build sending infrastructure, IP or domain warm-up, unsubscribe headers, feedback-loop registration, an editor, email rendering, or a website builder.

## 1. Scheduled Domain And IP Health Monitor

Today, `POST /v1/reputation/check` checks DNS blocklists (`reputation/dnsbl.rs`) and SPF, DKIM, and DMARC records (`reputation/dns_records.rs`), but only on request. This feature runs the same check on a schedule for each domain that a tenant marks for monitoring in `/v1/me/domains`. It also adds sending IPs that the tenant registers and checks them against IP blocklists. It adds a DMARC alignment check and a Gmail and Yahoo bulk-sender readiness check. Today the DKIM check tries five common selectors only, so the tenant must be able to give their own selectors. When a domain or IP goes on a blocklist, or a record breaks, the backend writes a row to `v1_alerts` and sends the tenant webhook. Most parts exist already, so this item has a low cost.

## 2. Bounce Reason Taxonomy

Today, outcomes have two bounce types: `hard_bounce` and `soft_bounce` (`outcomes.rs`). Provider payloads contain more detail: the SMTP enhanced status code, the SES bounce subtype, the Mailgun delivery status, and the Postmark bounce type. This feature maps that detail to a `bounce_class` on each outcome: `no_such_user`, `mailbox_full`, `dns_failure`, `policy_block`, `spam_block`, and `transient`. Each class gets a rule in the outcome policy (`/v1/outcome-policies`). The rule says "suppress now", "suppress after N", or "do not suppress". The `policy_block` and `spam_block` classes are sender problems, not recipient problems. For these classes, the backend must not suppress the recipient. It must raise a sender alert. The change adds a nullable column in a new migration and extends the mappers in `provider_outcomes.rs`.

## 3. Suppression Scopes

Today, a suppression applies to the whole tenant, because `v1_suppression_entries` has one row for each tenant and address. Senders need more layers. An address that unsubscribes from one newsletter can stay on another newsletter. Transactional mail must still reach a person who unsubscribes from marketing. This feature adds a `scope` (`tenant`, `list`, or `stream`) and a `scope_ref` to each entry. The check endpoint gets an optional `scope_ref`, and it applies the tenant scope plus the scope that matches. Hard bounces and complaints always use the tenant scope. Unsubscribes can use a narrower scope, and the transactional stream ignores marketing unsubscribes. The change needs a new migration. Do not edit the applied suppression migrations. A cross-tenant layer is a separate decision.

## 4. Signup Protection V2

The `signup_protection` policy mode (`decision/policy.rs`) blocks disposable and high-risk addresses. It does not see bot patterns. This feature adds signals for each tenant: signup velocity by domain and by caller IP, random-string local parts, plus-address and dot-variant farming (we already normalize aliases), and bursts from new domains (we already read domain age). A new signup endpoint returns a `confirm` decision. This decision tells the caller to send a double opt-in only when the risk is high, so real users do not get extra friction. The signup path must be fast, so it uses cached results and skips the paid mailbox check when a recent result exists. The caller IP is personal data. Store it as a hash with a short retention period. The existing recommendation actions do not change (see RFC 0020).

## 5. Mailbox Provider Breakdown For Each Campaign

Core already identifies the mailbox provider of each address: Gmail, Yahoo, Outlook, Mimecast, Proofpoint, or generic (`core/src/smtp/verif_method.rs`). Campaign outcomes already carry a `campaign_id` (`/v1/campaign-outcomes`). This feature joins the two data sets and shows delivered, bounce, and complaint rates for each provider in each campaign. Gmail does not report complaints for each message, so the Gmail spam rate comes from item 10, not from our complaint events. In managed deployments, results for company domains have no provider today, so this item first fixes provider detection on the MillionVerifier path (see RFC 0021). This is backlog item 81.

## 6. Bot Engagement Flag

Security gateways such as Mimecast and Proofpoint open and click links in each email before a person sees it. Mail privacy proxies also load images. These machine events make engagement data wrong and make bot traffic look like real readers. Core already detects Mimecast and Proofpoint gateways. This feature adds a `security_gateway` flag to each result. It also marks each ingested open or click as `is_machine` when it matches a heuristic: a click within seconds of delivery, all links clicked at once, or a known scanner user agent or IP range. Engagement counts in history, segments, and source quality then exclude machine events. The heuristic is not exact, so each flag carries a confidence value. This item can be a strong difference from other verification tools.

## 7. Webhook Event Subscriptions And Delivery Log

Today, each tenant has one webhook URL (`/v1/me/webhook`). The backend signs each payload with HMAC (`tenant/webhook.rs`) and retries on server errors (`worker/do_work.rs`). This feature adds more than one endpoint for each tenant, and a list of event types for each endpoint: `job.completed`, `job.failed`, `list.uploaded`, `address.status_changed`, `suppression.created`, and `alert.raised`. It adds a log of each delivery attempt, a manual retry, a disable switch that keeps the endpoint, secret rotation with two active secrets, and a `version` field on each payload. This item covers backlog items 65, 68, and 87.

## 8. MCP Server

An MCP server lets Claude, ChatGPT, and other AI tools use our product from a chat. This feature is a thin layer over the REST API and the generated TypeScript SDK. Start with read tools: check an email, get the history of an address, check a suppression, get list quality, get source quality, and get the provider report from item 5. Then add write tools: add a suppression, and create a segment from a description. The segment tool turns the description into filter JSON and validates it with `validate_segment_filter`. Write tools must ask the user to confirm. Auth uses a workspace API key, so the feature follows the existing plan rule (Professional and Enterprise).

## 9. beehiiv And Kit Integrations

Newsletter platforms send email on behalf of many creators. They were in the sender buyer list of the 2026-10-16 focus, but the Paperless focus does not include them (see Build Order). Both beehiiv and Kit send a webhook when a subscriber joins. Both have an API to tag or remove a subscriber. This feature receives the subscriber webhook, checks the address, and applies the result. It tags the subscriber or removes the subscriber when the action is `suppress` or `drop`. It also pushes our suppressions back to the platform. The inbound side reuses the provider endpoint model (`/v1/provider-endpoints`). The outbound side needs encrypted storage for each tenant's platform API key. Confirm the API terms and rate limits of each platform before build.

## 10. Postmaster Tools And SNDS Ingestion

Mailbox providers publish sender data. Google Postmaster Tools reports spam rate, authentication rates, and delivery errors for each verified domain. Microsoft SNDS reports data for each sending IP. This feature imports that data each day and shows it next to our outcomes and the item 1 monitor. Postmaster Tools needs OAuth, and the customer must verify their domain with Google first. SNDS moved to a REST API with OAuth in 2026. Postmaster Tools v2 removed the reputation dashboards, so confirm which metrics the current API exposes before build (see RFC 0026). This item depends on items 1 and 5, so it comes last.

## Full RFCs

| Item | RFC |
|---|---|
| 1 | [RFC 0017: Scheduled Domain And IP Health Monitor](0017-domain-and-ip-health-monitor.md) |
| 2 | [RFC 0018: Bounce Reason Taxonomy](0018-bounce-reason-taxonomy.md) |
| 3 | [RFC 0019: Suppression Scopes](0019-suppression-scopes.md) |
| 4 | [RFC 0020: Signup Protection V2](0020-signup-protection-v2.md) |
| 5 | [RFC 0021: Mailbox Provider Breakdown For Each Campaign](0021-mailbox-provider-breakdown.md) |
| 6 | [RFC 0022: Bot Engagement Flag](0022-bot-engagement-flag.md) |
| 7 | [RFC 0023: Webhook Event Subscriptions And Delivery Log](0023-webhook-event-subscriptions.md) |
| 8 | [RFC 0024: MCP Server](0024-mcp-server.md) |
| 9 | [RFC 0025: beehiiv And Kit Integrations](0025-beehiiv-and-kit-integrations.md) |
| 10 | [RFC 0026: Postmaster Tools And SNDS Ingestion](0026-postmaster-tools-and-snds-ingestion.md) |

## Build Order

Updated 2026-10-21. On 2026-10-20, `docs/strategy/company-focus.md` changed from the sender-only focus to the Paperless prospecting focus. The first Paperless release ships company-list research, person matching, email finding, list repair, verification, evidence review, and CSV export.

None of the ten items blocks that release. So the Paperless launch comes first. After it, the items that raise the count of usable contacts or lower cost come first. The north-star metric is the total cost for each correctly matched, usable B2B contact.

### Order At A Glance

| Phase | Work | Reason for this place |
|---|---|---|
| 0 | Ship Paperless: deploy Go, Rust, and web; run the controlled production workflow; run the benchmark; recruit pilots | This is the real release blocker. RFC work must not delay it. |
| 1 | Add a scope check to `GET /v1/sources/quality` | It closes a small security gap. It is not one of the ten items. |
| 1 | RFC 0021, provider detection only | Paperless verifies company addresses. On the MillionVerifier path, these results have no mailbox provider today. |
| 1 | RFC 0018, without sender alerts | A wrong suppression removes a usable contact from the export. This hurts the north-star metric. |
| 2 | RFC 0023, with Paperless events such as a completed research run | The secret cipher is also the base for CRM credentials later. |
| 2 | RFC 0019 | Build it when agencies ask for a separate do-not-contact list for each client. |
| 2 | RFC 0024, phase 1, with the research, find, and export tools first | The web lead can build it in parallel with RFC 0023 and RFC 0019. |
| 3 | RFC 0017, the RFC 0018 sender alerts, the RFC 0021 report, then RFC 0022 | These serve verification and list-hygiene customers, which stay a supported workflow. |
| Parked | RFC 0020, RFC 0025, RFC 0026 | Their buyers are signup-form owners, newsletter platforms, and senders who need provider analytics. The Paperless focus does not include them. Review them again after the pilots. |

The sections below give the details for each phase.

### Phase 0: Ship Paperless

Finish the open release item in `docs/paperless-research-integration.md`. Deploy compatible Go, Rust, and web revisions, and run a controlled production workflow. Then run the benchmark and recruit the pilot customers. Work on these RFCs must not delay these steps.

### Phase 1: Small fixes that help Paperless now

1. Add an `outcomes.read` scope check to `GET /v1/sources/quality`. Today any API key of the tenant can read it. This fix is not one of the ten items.
2. RFC 0021, provider detection only. Detect the mailbox provider from the MX host on the MillionVerifier path, and store `mailbox_provider` and `mail_gateway`. Paperless verifies company addresses, and today these results have no provider.
3. RFC 0018, without sender alerts. Classify bounces, stop suppressing for sender-side bounces, and accept `smtp_status` in outcome imports. A wrong suppression removes a usable contact from the export.

### Phase 2: After the first paid pilots

4. RFC 0023. Add Paperless event types, for example a research run that completes. The secret cipher is also the base for CRM credentials, which the thesis plans after pilot value.
5. RFC 0019, when agencies ask for a separate do-not-contact list for each client. The audience key can name the client.
6. RFC 0024, phase 1. Put the Paperless research, find, and verify tools first. The web lead can build it in parallel with items 4 and 5.

### Phase 3: Verification and list-hygiene customers

Verification and list hygiene stay supported workflows.

7. The alert changes in RFC 0017, then the monitor. Confirm the blocklist terms first.
8. The sender alerts in RFC 0018.
9. The provider report in RFC 0021.
10. RFC 0022. It needs item 2 and real engagement data to tune its rules.

### Parked

These items serve newsletter senders and signup forms. The Paperless focus does not include these buyers. Review them again after the pilots.

- RFC 0020: signup protection for customer forms.
- RFC 0025: beehiiv and Kit integrations.
- RFC 0026: Postmaster Tools and SNDS. If it comes back, apply for Google OAuth approval early, because the review takes weeks.
