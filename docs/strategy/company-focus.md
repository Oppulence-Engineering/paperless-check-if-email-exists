# Company Focus: One Pager

**Product:** Check If Email Exists · **Status:** Draft · **Date:** 2026-10-16

## Focus

We help teams that send email keep their lists clean, so their mail gets to the inbox.

We are the system that decides if an address is safe to send to. We do not send email.

We add finder, search, and enrichment data (RFC 0011–0015). This data serves list repair, account coverage, and segmentation for senders. It is not a cold-prospecting product.

## Our buyer

Our buyer is a team that **already has the addresses** and **sends email to them**.

| | |
|---|---|
| **Who buys** | Lifecycle and email marketing leads, marketing operations, deliverability owners, and engineers who own signup and transactional email. |
| **Company type** | B2C and B2B senders with large lists. SaaS products with signup forms. Platforms and agencies that send on behalf of their customers. |
| **Their pain** | Bounces and spam traps damage their sender reputation. Mail goes to spam. The ESP warns or suspends the account. Old lists decay with no warning. |
| **What they want** | A clear "send / do not send" answer for each address. Proof that the list stays clean over time. |

**Not our buyer:** sales teams that look for new contacts to prospect. Hunter.io, Apollo, and similar tools serve them. We do not compete for that budget.

## Why we win with this buyer

1. **We use real delivery results.** SendGrid, Mailgun, Postmark, and SES webhooks tell us which addresses bounced. Hunter gets this data only from its own sending tool.
2. **We grade catch-all domains.** Other tools mark catch-all as "risky" and stop. We give a severity tier now, and we will give a measured bounce rate.
3. **We keep lists clean over time.** Suppressions, scheduled reverification, pipelines, and list quality scores work after the first check.
4. **We give one decision for each address.** The score, the reason codes, and the policy modes (strict, balanced, aggressive) give a clear "safe to send" result.
5. **Customers can self-host.** Teams with strict data rules keep all data in their own database.

## How we differ from Hunter.io

| | Hunter.io | Us |
|---|---|---|
| Main product | Contact data for prospecting | Send decisions for lists you already have |
| Email check | Add-on, 0.5 credit | Core product |
| Catch-all | Confidence score based on web sources | Severity tier now, measured bounce rate next |
| Delivery feedback | Only from its own sending tool | From the customer's ESP |
| Data location | Hunter cloud only | Our cloud or self-hosted |

## Priorities: next two quarters

1. **Measured catch-all risk.** Calculate a bounce rate for each catch-all domain from ESP delivery results. Show it in each result.
2. **More ESP and marketing platform connections.** We already have SendGrid, Mailgun, Postmark, and SES. Add Klaviyo, Mailchimp, Customer.io, Braze, and HubSpot Marketing. These platforms are where our buyer works.
3. **Signup protection.** Make the real-time API fast and easy to add to signup forms. Stop bad addresses before they enter the list.
4. **Address repair.** When a known contact bounces, use the finder to suggest the new address at the same domain. Teach the finder each domain's pattern from our own results. We use the finder to repair lists, not to prospect.
5. **AI and automation access.** Ship an MCP server and Zapier and Make connectors on top of the current API.
6. **Hunter.io data parity, framed for senders.** Build in this order:
   1. Shared foundations: credits, spend guardrails, privacy, and the Legal Review (RFC 0015).
   2. Finder and list repair with calibrated confidence (RFC 0011).
   3. Enrichment with a job-change warning and monitor (RFC 0014).
   4. Domain Search for account coverage (RFC 0012).
   5. Discover with lookalike accounts (RFC 0013).

   Data comes from Parallel Web Systems. Legal review is a release gate.

## What we do not build

1. Our own web crawler or a bought contact database.
2. Email sequences, inbox warm-up, or domain purchase.
3. Discover or Domain Search as the main entry point of the product. Entry points start from the tenant's lists.

## How we measure success

| Metric | Why it matters |
|---|---|
| Customer bounce rate after cleaning | It shows that our core promise works. |
| Share of tenants with an ESP webhook connected | It shows activation and feeds our data advantage. |
| Share of catch-all results with a measured risk | It shows progress on our main difference. |
| Reverification and pipeline volume per tenant | It shows ongoing use, not a single check. |
| Gross margin on data features (target: 85% or more) | It keeps the data features profitable (RFC 0015 section 16). |

We must set the targets before the start of the quarter.

## Open decisions

1. Do we price per check, per list size, or by a monthly plan with ongoing monitoring?
2. Which marketing platform do we connect first?
3. Can catch-all bounce rates be shared across tenants? Each tenant must opt in, and we must share only domain-level numbers.
