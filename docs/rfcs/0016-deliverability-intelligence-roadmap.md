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

We compared our backend with the list of things that a newsletter platform must run to get mail to the inbox. Most of that list is about sending, and we do not send email (see `docs/strategy/company-focus.md`). These ten items are the parts that we can own without sending.

## Out Of Scope

We do not build sending infrastructure, IP or domain warm-up, unsubscribe headers, feedback-loop registration, an editor, email rendering, or a website builder.

## 1. Scheduled Domain And IP Health Monitor

Today, `POST /v1/reputation/check` checks DNS blocklists (`reputation/dnsbl.rs`) and SPF, DKIM, and DMARC records (`reputation/dns_records.rs`), but only on request. This feature runs the same check on a schedule for each domain that a tenant marks for monitoring in `/v1/me/domains`. It also adds sending IPs that the tenant registers and checks them against IP blocklists. It adds a DMARC alignment check and a Gmail and Yahoo bulk-sender readiness check. Today the DKIM check tries five common selectors only, so the tenant must be able to give their own selectors. When a domain or IP goes on a blocklist, or a record breaks, the backend writes a row to `v1_alerts` and sends the tenant webhook. Most parts exist already, so this item has a low cost.

## 2. Bounce Reason Taxonomy

Today, outcomes have two bounce types: `hard_bounce` and `soft_bounce` (`outcomes.rs`). Provider payloads contain more detail: the SMTP enhanced status code, the SES bounce subtype, the Mailgun delivery status, and the Postmark bounce type. This feature maps that detail to a `bounce_class` on each outcome: `no_such_user`, `mailbox_full`, `dns_failure`, `policy_block`, `spam_block`, and `transient`. Each class gets a rule in the outcome policy (`/v1/outcome-policies`). The rule says "suppress now", "suppress after N", or "do not suppress". The `policy_block` and `spam_block` classes are sender problems, not recipient problems. For these classes, the backend must not suppress the recipient. It must raise a sender alert. The change adds a nullable column in a new migration and extends the mappers in `provider_outcomes.rs`.

## 3. Suppression Scopes

Today, a suppression applies to the whole tenant, because `v1_suppression_entries` has one row for each tenant and address. Senders need more layers. An address that unsubscribes from one newsletter can stay on another newsletter. Transactional mail must still reach a person who unsubscribes from marketing. This feature adds a `scope` (`tenant`, `list`, or `stream`) and a `scope_ref` to each entry. The check endpoint gets an optional `scope_ref`, and it applies the tenant scope plus the scope that matches. Hard bounces and complaints always use the tenant scope. Unsubscribes can use a narrower scope, and the transactional stream ignores marketing unsubscribes. The change needs a new migration. Do not edit the applied suppression migrations. A cross-tenant layer is a separate decision (open decision 3 in `company-focus.md`).

## 4. Signup Protection V2

The `signup_protection` policy mode (`decision/policy.rs`) blocks disposable and high-risk addresses. It does not see bot patterns. This feature adds signals for each tenant: signup velocity by domain and by caller IP, random-string local parts, plus-address and dot-variant farming (we already normalize aliases), and bursts from new domains (we already read domain age). It adds a new recommendation action, `confirm`. This action tells the caller to send a double opt-in only when the risk is high, so real users do not get extra friction. The signup path must be fast, so it uses cached results and skips slow SMTP checks when a recent result exists. The caller IP is personal data. Store it as a hash with a short retention period. The new action is an additive API change, so regenerate the SDKs. This item is priority 3 in `company-focus.md`.

## 5. Mailbox Provider Breakdown For Each Campaign

Core already identifies the mailbox provider of each address: Gmail, Yahoo, Outlook, Mimecast, Proofpoint, or generic (`core/src/smtp/verif_method.rs`). Campaign outcomes already carry a `campaign_id` (`/v1/campaign-outcomes`). This feature joins the two data sets and shows delivered, bounce, and complaint rates for each provider in each campaign. The report marks the Gmail complaint-rate limits (0.1% target, 0.3% maximum), so the sender sees a provider problem before the provider blocks them. This is backlog item 81. The data exists, so this item needs one query endpoint and one page.

## 6. Bot Engagement Flag

Security gateways such as Mimecast and Proofpoint open and click links in each email before a person sees it. Mail privacy proxies also load images. These machine events make engagement data wrong and make bot traffic look like real readers. Core already detects Mimecast and Proofpoint gateways. This feature adds a `security_gateway` flag to each result. It also marks each ingested open or click as `is_machine` when it matches a heuristic: a click within seconds of delivery, all links clicked at once, or a known scanner user agent or IP range. Engagement counts in history, segments, and source quality then exclude machine events. The heuristic is not exact, so each flag carries a confidence value. This item can be a strong difference from other verification tools.

## 7. Webhook Event Subscriptions And Delivery Log

Today, each tenant has one webhook URL (`/v1/me/webhook`). The backend signs each payload with HMAC (`tenant/webhook.rs`) and retries on server errors (`worker/do_work.rs`). This feature adds more than one endpoint for each tenant, and a list of event types for each endpoint: `job.completed`, `job.failed`, `list.uploaded`, `address.status_changed`, `suppression.created`, and `alert.raised`. It adds a log of each delivery attempt, a manual retry, a disable switch that keeps the endpoint, secret rotation with two active secrets, and a `version` field on each payload. This item covers backlog items 65, 68, and 87.

## 8. MCP Server

An MCP server lets Claude, ChatGPT, and other AI tools use our product from a chat. This feature is a thin layer over the REST API and the generated TypeScript SDK. Start with read tools: check an email, get the history of an address, check a suppression, get list quality, get source quality, and get the provider report from item 5. Then add write tools: add a suppression, and create a segment from a description. The segment tool turns the description into filter JSON and validates it with `validate_segment_filter`. Write tools must ask the user to confirm. Auth uses a workspace API key, so the feature follows the existing plan rule (Professional and Enterprise). This item is priority 5 in `company-focus.md`.

## 9. beehiiv And Kit Integrations

Newsletter platforms send email on behalf of many creators, and they are in our buyer list. Both beehiiv and Kit send a webhook when a subscriber joins. Both have an API to tag or remove a subscriber. This feature receives the subscriber webhook, checks the address, and applies the result. It tags the subscriber or removes the subscriber when the action is `suppress` or `drop`. It also pushes our suppressions back to the platform. The inbound side reuses the provider endpoint model (`/v1/provider-endpoints`). The outbound side needs encrypted storage for each tenant's platform API key. Confirm the API terms and rate limits of each platform before build. This item adds two platforms to priority 2 in `company-focus.md`.

## 10. Postmaster Tools And SNDS Ingestion

Mailbox providers publish sender data. Google Postmaster Tools reports spam rate, authentication rates, and delivery errors for each verified domain. Microsoft SNDS reports data for each sending IP. This feature imports that data each day and shows it next to our outcomes and the item 1 monitor. Postmaster Tools needs OAuth, and the customer must verify their domain with Google first. SNDS needs an access key from the customer. Google changed the Postmaster Tools dashboards recently, so confirm which metrics the current API exposes before build. This item depends on items 1 and 5, so it comes last.

## Build Order

1. Items 1, 2, and 5. They reuse the most existing code.
2. Items 3 and 7. They extend data that exists today.
3. Items 4 and 6. They add new signals and need tuning on real data.
4. Items 8 and 9. They add new access paths to the product.
5. Item 10. It needs OAuth and work by the customer.
