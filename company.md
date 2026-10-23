# Verify

**A PlaybookMedia product · Business thesis · October 2026**

Better contacts. Less manual research. More confidence before outreach.

## Company and product

PlaybookMedia operates Verify, a B2B contact quality, verification, and data repair
platform. Verify sits between contact discovery and outbound execution: customers
bring their prospect records, and Verify helps determine whether those records
identify the correct company, person, current employer, and relevant professional
role, with an acceptable mailbox under the customer's sending policy.

Finding an email is increasingly inexpensive. Establishing that it belongs to the
right person, at the right company, in the right role remains costly. We compete
on correctly matched, usable contacts and the work required to produce them.

The source of this strategy is the [October 2026 business thesis](docs/strategy/verify-business-thesis-2026-10.md).
The [vision](vision.md) defines the product destination; the [codebase audit](docs/audits/2026-10-23-verify-north-star.md)
and [RFC sequence](docs/rfcs/README.md) distinguish current capabilities from plans.

## Initial customer

Start with small and medium-sized US B2B outbound agencies, appointment-setting
companies, and prospecting operations managing approximately **3–20 active client
lists**. They already use Apollo, Hunter, Clay, spreadsheets, HubSpot, or other CRM
and outbound tools.

- **Buyer:** agency founder, operations manager, or head of client delivery.
- **Operator:** list researcher, sales operations specialist, campaign manager, or
  SDR operations lead.
- **Recurring job:** prepare a reliable prospect list for a new campaign, changed
  target role, or refresh of an existing client list.

Secondary customers are RevOps teams, specialist researchers, founder-led sales
teams, and occasional list-audit buyers. Geographic expansion follows demonstrated
demand and appropriate data protection, marketing, and provider rights.

## Customer outcome

Import customer-owned records and retain their original identifiers. Assess
company and contact identity, selectively repair missing or stale information,
verify professional mailboxes, and present evidence for review. Deliver:

| Group | Meaning |
|---|---|
| Ready | Approved records with sufficient identity, employment, role, mailbox, and sending-policy evidence. |
| Needs Review | Missing, conflicting, stale, ambiguous, or uncertain evidence; further work may be authorized. |
| Excluded | Records that fail requirements, are explicitly rejected, or are suppressed, with reasons retained. |

CSV is the initial handoff. CRM synchronization follows customer demand, with a
change preview and explicit overwrite controls. Recurring repair follows repeat
purchases. This is the intended workflow; the current pilot has only `approved`
and `review` export modes and does not yet implement the complete decision model.

## Positioning

Customers can keep the discovery and sales tools they already trust. Verify
addresses the correction and decision work between acquiring data and using it.
We do not assume that a larger database or a cheaper copy of Hunter, Apollo, Clay,
or Icypeas will differentiate the product.

A mailbox result does not establish identity, employment, buying authority,
consent, inbox placement, or revenue. Show supported facts, inferences, and unknowns
separately. Comparative accuracy, time savings, and price advantages remain
hypotheses until measured against a competently configured customer baseline.

## Commercial model to validate

Test a **$249 fixed-scope agency audit** with a defined account batch, research
allowance, verification, evidence review, and correction file. Agree the scope and
maximum charge before work starts; the price does not imply unlimited research.

After successful audits, test **$149–$299/month** recurring plans, with variable
research charges separately approved. These are experiments, not established
prices or implemented entitlements. Existing verification subscriptions currently
use Starter $49 and Professional $149 in [billing configuration](web/lib/billing/plans.ts).
Changing strategy documents does not change those plans or customer contracts.

Verification-only access, API usage, and selectively managed audits may remain
revenue streams. Include failed lookups, provider fees, infrastructure, review,
and delivery/support labor when assessing economics. Report internal contribution
margin separately from the customer's cost and time savings.

## Assets and ownership

| System | Responsibility |
|---|---|
| This Paperless repository | Next.js customer application and Better Auth; Rust verification, quotas, lists, suppression, outcomes, research gateway, and policy-aware exports; CLI and SDKs. |
| Oppulence Paperless Go backend | Company and person research, source observations, enrichment orchestration, durable research execution, and provider spending controls. This is a separate repository. |
| MillionVerifier | Managed mailbox verification provider. Preserve provider evidence; failures must not become invented SMTP findings. |

Keep research credentials and workspace authority server-side. Customer-private
lists, relationship data, suppression records, and observations stay isolated.
Verify commercial data and the Go repository's internal finance identity graph
require explicit ownership and permitted-use boundaries. A shared commercial
dataset requires independent sourcing or an appropriate license; ordinary Parallel
customer terms must not be treated as permission for database resale.

## Validation and distribution

Founder-led audits use recent customer lists and target roles to measure quality,
cost, and manual work against the customer's existing process. Recruit through
agency communities, RevOps practitioners, prospecting operators, consultants, and
direct outreach. Publish observed results with the cohort and limitations visible.

The following windows begin with the validation program, not repository creation:

| Window | Work and evidence |
|---|---|
| First 14 days | Interview approximately 20 practitioners; inspect real workflows; recruit five unrelated paying design partners; resolve source rights and product ownership. |
| First 30 days | Benchmark approximately 1,200 rights-cleared accounts across three segments against customer baselines; record matching, role relevance, uncertainty, costs, and cleanup time. Expand execution beyond the pilot only through measured capacity gates. |
| First 60 days | Deliver second audits, reduce unnecessary research and review, and test limited CRM writeback only if requested. |
| First 90 days | Evaluate repeat purchasing, measured operational advantage, and contribution margin before broader investment. |

Investment gates are **at least five paid audits, three repeat-paying customers,
positive contribution margin**, and either meaningfully more correctly matched
approved contacts or substantially less manual cleanup without sacrificing
quality. Define comparison thresholds before collecting the evaluation results.
These are targets, not achievements. Failure to meet them calls for revising the
segment, offer, or workflow before expanding the platform.

## Success and stage

The north-star metric is **total cost per correctly matched, usable, role-relevant
B2B contact**, including unsuccessful research and human review. Track coverage,
false matches, stale employment, uncertainty, review time, provider spend, time to
usable export, repeat usage, retention, and margins alongside it.

Verify is technically launched according to the supplied thesis and recorded
release history; commercial product-market fit is unproven. The reported October
20 production catch-all case demonstrated withholding an approved named-contact
export. It did not establish scalable discovery, competitive advantage, profitable
operations, or retention. Current source limits remain ten companies, three
candidates per company, and thirty verifications per research run.

The immediate priority is proving that customers repeatedly pay for better,
more reliable prospect lists.
