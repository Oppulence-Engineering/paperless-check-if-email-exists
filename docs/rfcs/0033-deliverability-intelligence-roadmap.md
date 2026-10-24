# RFC 0033: Deliverability Intelligence Roadmap

## Verify disposition — 2026-10-23

**Superseded as product build order; historical feature inventory.** The Verify audit roadmap is now authoritative. The feature descriptions below remain an inventory for verification/sender customers, not an alternative launch sequence. The old source-quality scope task is already enforced in shared auth; deployment and commercial validation are different evidence gates.

[Current build order](README.md) · [Codebase audit](../audits/2026-10-23-verify-north-star.md). This disposition supersedes conflicting historical scope, priority, or baseline statements below; it does not certify every original acceptance criterion.

| | |
|---|---|
| **Status** | Superseded as product build order; historical feature inventory; audited 2026-10-23 |
| **Created** | 2026-10-19 |
| **Owners** | To assign: backend lead, web lead |
| **Builds on** | RFC 0031 (Provider Webhooks), RFC 0027 (Suppression Intelligence), RFC 0026 (Policy Modes) |
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

The `signup_protection` policy mode (`decision/policy.rs`) blocks disposable and high-risk addresses. It does not see bot patterns. This feature adds signals for each tenant: signup velocity by domain and by caller IP, random-string local parts, plus-address and dot-variant farming (we already normalize aliases), and bursts from new domains (we already read domain age). A new signup endpoint returns a `confirm` decision. This decision tells the caller to send a double opt-in only when the risk is high, so real users do not get extra friction. The signup path must be fast, so it uses cached results and skips the paid mailbox check when a recent result exists. The caller IP is personal data. Store it as a hash with a short retention period. The existing recommendation actions do not change (see RFC 0023).

## 5. Mailbox Provider Breakdown For Each Campaign

Core already identifies the mailbox provider of each address: Gmail, Yahoo, Outlook, Mimecast, Proofpoint, or generic (`core/src/smtp/verif_method.rs`). Campaign outcomes already carry a `campaign_id` (`/v1/campaign-outcomes`). This feature joins the two data sets and shows delivered, bounce, and complaint rates for each provider in each campaign. Gmail does not report complaints for each message, so the Gmail spam rate comes from item 10, not from our complaint events. In managed deployments, results for company domains have no provider today, so this item first fixes provider detection on the MillionVerifier path (see RFC 0017). This is backlog item 81.

## 6. Bot Engagement Flag

Security gateways such as Mimecast and Proofpoint open and click links in each email before a person sees it. Mail privacy proxies also load images. These machine events make engagement data wrong and make bot traffic look like real readers. Core already detects Mimecast and Proofpoint gateways. This feature adds a `security_gateway` flag to each result. It also marks each ingested open or click as `is_machine` when it matches a heuristic: a click within seconds of delivery, all links clicked at once, or a known scanner user agent or IP range. Engagement counts in history, segments, and source quality then exclude machine events. The heuristic is not exact, so each flag carries a confidence value. This item can be a strong difference from other verification tools.

## 7. Webhook Event Subscriptions And Delivery Log

Today, each tenant has one webhook URL (`/v1/me/webhook`). The backend signs each payload with HMAC (`tenant/webhook.rs`) and retries on server errors (`worker/do_work.rs`). This feature adds more than one endpoint for each tenant, and a list of event types for each endpoint: `job.completed`, `job.failed`, `list.uploaded`, `address.status_changed`, `suppression.created`, and `alert.raised`. It adds a log of each delivery attempt, a manual retry, a disable switch that keeps the endpoint, secret rotation with two active secrets, and a `version` field on each payload. This item covers backlog items 65, 68, and 87.

## 8. MCP Server

An MCP server lets Claude, ChatGPT, and other AI tools use our product from a chat. This feature is a thin layer over the REST API and the generated TypeScript SDK. Start with read tools: check an email, get the history of an address, check a suppression, get list quality, get source quality, and get the provider report from item 5. Then add write tools: add a suppression, and create a segment from a description. The segment tool turns the description into filter JSON and validates it with `validate_segment_filter`. Write tools must ask the user to confirm. Auth uses a workspace API key, so the feature follows the existing plan rule (Professional and Enterprise).

## 9. beehiiv And Kit Integrations

Newsletter platforms send email on behalf of many creators. They were in the sender buyer list of the 2026-10-16 focus, but the Paperless focus does not include them (see Build Order). Both beehiiv and Kit send a webhook when a subscriber joins. Both have an API to tag or remove a subscriber. This feature receives the subscriber webhook, checks the address, and applies the result. It tags the subscriber or removes the subscriber when the action is `suppress` or `drop`. It also pushes our suppressions back to the platform. The inbound side reuses the provider endpoint model (`/v1/provider-endpoints`). The outbound side needs encrypted storage for each tenant's platform API key. Confirm the API terms and rate limits of each platform before build.

## 10. Postmaster Tools And SNDS Ingestion

Mailbox providers publish sender data. Google Postmaster Tools reports spam rate, authentication rates, and delivery errors for each verified domain. Microsoft SNDS reports data for each sending IP. This feature imports that data each day and shows it next to our outcomes and the item 1 monitor. Postmaster Tools needs OAuth, and the customer must verify their domain with Google first. SNDS moved to a REST API with OAuth in 2026. Postmaster Tools v2 removed the reputation dashboards, so confirm which metrics the current API exposes before build (see RFC 0025). This item depends on items 1 and 5, so it comes last.

## Full RFCs

| Item | RFC |
|---|---|
| 1 | [RFC 0019: Scheduled Domain And IP Health Monitor](0019-domain-and-ip-health-monitor.md) |
| 2 | [RFC 0012: Bounce Reason Taxonomy](0012-bounce-reason-taxonomy.md) |
| 3 | [RFC 0011: Suppression Scopes](0011-suppression-scopes.md) |
| 4 | [RFC 0023: Signup Protection V2](0023-signup-protection-v2.md) |
| 5 | [RFC 0017: Mailbox Provider Breakdown For Each Campaign](0017-mailbox-provider-breakdown.md) |
| 6 | [RFC 0020: Bot Engagement Flag](0020-bot-engagement-flag.md) |
| 7 | [RFC 0013: Webhook Event Subscriptions And Delivery Log](0013-webhook-event-subscriptions.md) |
| 8 | [RFC 0021: MCP Server](0021-mcp-server.md) |
| 9 | [RFC 0024: beehiiv And Kit Integrations](0024-beehiiv-and-kit-integrations.md) |
| 10 | [RFC 0025: Postmaster Tools And SNDS Ingestion](0025-postmaster-tools-and-snds-ingestion.md) |

## Build Order

Superseded on 2026-10-23 by the [Verify RFC delivery sequence](README.md#build-order).

Start with rights and paid validation (0001/0002), then import lineage and durable
review/delivery (0003/0004), then budgeted selective repair and measured capacity
(0005). Requested CRM updates (0006) and recurring maintenance (0007) follow
repeat-customer and economic evidence. The source-quality scope requirement is
already enforced by shared auth middleware; it is not an open launch task.

The inventory above remains useful for supported verification/list-hygiene
customers. Narrow 0012 correctness, 0011 client suppression, and 0017 attribution
work can be promoted for measured audit needs. Extract 0013 credential storage
when needed for CRM. Broad sender reports, health monitoring, MCP, signup tools,
newsletter connectors, and Postmaster data do not precede the audit workflow.
