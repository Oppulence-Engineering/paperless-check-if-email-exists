# VERIFY
**A PlaybookMedia Product | Business Thesis | October 2026**

*Better contacts. Less manual research. More confidence before outreach.*

## Company overview

Verify is a B2B contact quality, verification, and data repair platform that helps companies and sales teams determine whether their prospect records are accurate, current, relevant, and usable.

The platform sits between contact discovery and outbound execution. It takes information customers already possess, validates the underlying company and individual identities, identifies incorrect or outdated records, repairs missing information, verifies professional email addresses, and prepares approved contacts for use in existing sales systems.

Verify's central thesis is straightforward: **finding an email address is increasingly inexpensive; establishing that it belongs to the right person, at the right company, in the right role, remains a recurring operational problem.**

Rather than compete primarily on database size or email verification volume, Verify intends to compete on the number of correctly matched, usable B2B contacts customers obtain for their money, and the time required to produce those contacts.

The business is operated under PlaybookMedia. Its technical foundation consists of the existing Paperless verification application and the Oppulence Paperless Go research backend.

## The problem

Businesses spend money acquiring leads through platforms such as Apollo, Hunter, Clay, and other prospecting services, but their data often requires substantial additional work before it can be used confidently.

Several failures recur during campaign preparation:

- **Outdated contacts:** Employees change jobs, titles, and responsibilities while their previous records remain in contact databases.
- **Incorrect identity matching:** A company name, website, employee, or professional role is matched to the wrong entity.
- **Missing decision-makers:** A valid email address does not establish that its owner is responsible for the purchasing decision.
- **Fragmented workflows:** Teams combine databases, web research, verification tools, spreadsheets, and CRM applications to prepare one prospect list.
- **Unpredictable research costs:** Repeated enrichment calls and verification credits may produce records that still require manual review.
- **Poorly handled uncertainty:** Catch-all domains, outdated sources, conflicting employment records, and ambiguous role information are sometimes presented with more certainty than the underlying evidence justifies.

For outbound agencies, these failures repeat whenever a client starts a campaign, adds target accounts, changes its target audience, or refreshes existing lists.

Verify addresses the work involved in identifying, correcting, and deciding what to do with these records before outreach begins.

## Target customers

Verify will initially serve small and medium-sized B2B outbound agencies, appointment-setting companies, and prospecting operations that manage multiple client campaigns.

The initial ideal customer is an agency managing approximately 3–20 active client lists, using an existing mixture of Apollo, Hunter, Clay, spreadsheets, HubSpot, or other CRM and outbound platforms.

The economic buyer is usually the agency founder, operations manager, or head of client delivery. The daily user is the list researcher, sales operations specialist, campaign manager, or SDR operations lead.

These buyers have a measurable incentive to improve list quality: poor records consume research time, provider credits, and campaign capacity, while client delivery teams must repeatedly correct mistakes.

Secondary markets include RevOps teams, specialized B2B lead researchers, founder-led sales organizations, and businesses needing occasional contact-list audits.

The initial geographic focus is the United States, with expansion into additional markets subject to appropriate data protection, marketing, and provider-licensing requirements.

## The product

Verify brings together seven connected capabilities.

**1. Import and organize**

Customers upload existing prospect or target-account lists, identify the client or campaign, map company and contact fields, and preserve the original record history.

The system detects duplicates, incomplete records, malformed information, and potential company-domain conflicts without silently discarding source data.

**2. Assess company and contact identity**

Verify evaluates whether the supplied company, domain, individual, employer, and professional role correspond to one another.

Each important claim can be supported by source information and an observation date. A generic mailbox such as `billing@company.com` is distinguished from evidence identifying a named finance director.

**3. Repair incomplete and outdated records**

The system researches missing or questionable information, identifies possible current contacts, corrects mismatched company records, and investigates stale professional roles.

Research is selective rather than automatically applied to every record. Customers can authorize a maximum spend before expensive provider calls begin.

**4. Verify professional email addresses**

Verify uses its verification infrastructure to assess mailbox status and classify valid, invalid, risky, catch-all, unknown, and failed verification outcomes.

Mailbox verification remains separate from person identity, employment, and role validation. An uncertain result is not automatically converted into a usable contact.

**5. Review evidence and make decisions**

A review interface presents the evidence supporting each record, including unresolved contradictions and uncertainty.

Users can approve, hold, reject, or request additional research. Suppression records, complaints, and customer-specific sending policies are checked before final approval.

**6. Export and integrate**

Customers receive three primary record groups:

- **Ready:** Records meeting the customer's identity, relevance, verification, and sending-policy requirements.
- **Needs Review:** Records with missing, conflicting, outdated, or uncertain evidence.
- **Excluded:** Records that fail the applicable requirements or have been explicitly suppressed.

Exports preserve original record identifiers, corrections, reasons, sources, and relevant verification history.

Initially, CSV provides the main integration mechanism. HubSpot synchronization and other CRM connections follow demonstrated demand, with preview-first updates and explicit overwrite controls.

**7. Maintain data quality**

After the initial workflow demonstrates repeat purchasing, Verify can expand into targeted re-verification, list comparisons, role-change detection, and recurring contact repair.

The purpose is to maintain accurate customer-owned prospect data without requiring teams to rebuild lists whenever information changes.

## Competitive positioning

Verify enters a market containing established contact discovery, enrichment, and verification providers.

Hunter offers contact discovery, domain search, email finding, and verification. Apollo combines contact data with broader sales execution workflows. Clay supports enrichment waterfalls, research automation, and integrations. Icypeas and other providers offer inexpensive email finding and verification.

Building the same collection of features at a slightly lower price is unlikely to create sufficient differentiation.

Verify's proposed position is complementary: customers continue using the tools they already trust, while Verify addresses the expensive work between data acquisition and campaign execution.

Its distinction is the ability to answer five separate questions:

1. Is this the correct company?
2. Is this the correct individual?
3. Does this person currently work there?
4. Is this person relevant to the target role or purchasing decision?
5. Is the mailbox acceptable under the customer's verification and sending policy?

The product should explain which answers are supported, which are inferred, and which remain unknown.

This differentiation remains a hypothesis until customer benchmarks establish a measurable advantage against competently configured existing tools.

## Business model

Verify will initially test a paid, fixed-scope contact-list audit rather than assume that a conventional SaaS subscription is the correct first purchase.

An initial pricing experiment is a **$249 agency audit**, covering a defined batch of target accounts, an explicit research allowance, verification, evidence review, and a deliverable correction file.

Following successful audits, the company can test recurring plans in the **$149–$299 per month** range for agencies requiring frequent campaign-list quality checks, with separately approved variable research charges.

These figures are proposed validation prices, not established market-clearing prices.

Verification-only tools and API access may support lower-priced self-service tiers, but the economics of research-intensive contact repair must account for unsuccessful lookups, provider fees, infrastructure, and manual review.

Over time, revenue could come from recurring list-quality subscriptions, usage-based research, API verification, and selectively managed audits.

The commercial objective is to obtain recurring revenue from a measurable operational outcome, rather than charge customers repeatedly for uncertain enrichment attempts.

## Technology and existing assets

Verify builds on two existing repositories:

**Paperless Check If Email Exists:** The customer-facing Next.js application and Rust verification infrastructure. Its capabilities include Better Auth identity, workspace management, verification operations, bulk list handling, suppression, usage limits, verification history, API access, and policy-aware exports.

Managed deployments use MillionVerifier as the verification provider, preserving external verification evidence and distinguishing actual provider results from local diagnostics.

**Oppulence Paperless Backend:** The Go-based research infrastructure supporting company discovery, website research, contact extraction, enrichment orchestration, background execution, provider integrations, tenant boundaries, evidence collection, and spending controls.

The two systems communicate through server-side integration contracts so customer credentials and research authority remain outside the browser.

On October 20, 2026, merged release PR #124 documented a controlled production import-to-review workflow. An address returned a catch-all result, and the application correctly withheld an approved named-contact export.

That establishes an important part of the safety model. It does not yet establish successful contact discovery at meaningful volume, comparative accuracy, profitable operating costs, or paying-customer retention.

The connected research pilot currently limits runs to ten companies, three candidates per company, and thirty verification requests. Those limits provide a bounded initial environment but must be expanded carefully before representative agency-scale evaluation.

The Go repository also contains a separate internal business-identity and intelligence-graph strategy for finance applications. Verify's customer-facing commercial data and the internal finance graph require explicitly defined ownership, licensing, and tenant-isolation boundaries.

## Distribution strategy

Verify's initial growth strategy centers on demonstrating its value using customer-owned data.

The company will recruit boutique outbound agencies, RevOps practitioners, and specialist prospecting operators who already experience repeated list preparation work.

Founder-led audits can establish value before a large marketing or engineering investment. A customer provides a recent list and its target roles; Verify returns a corrected and evidence-backed version, together with the time, cost, and quality differences against the customer's current process.

Distribution can expand through agency communities, operational case studies, direct outreach, integrations, and consultants who already manage prospecting workflows.

Successful customer examples should report observed contact quality and operational savings. Verify should not advertise guaranteed response rates, inbox placement, or revenue increases without separately controlled evidence.

## Long-term defensibility

Verify's potential advantage develops through repeated use and better decisions about contact information.

With appropriate customer authorization and data rights, the platform can accumulate improved identity-resolution rules, reviewed company mappings, source-dated role evidence, correction patterns, and historical verification outcomes.

Over time, this can improve the accuracy and efficiency of future audits while making recurring customer workflows easier to maintain.

The system should remain provider-agnostic, using deterministic methods and permitted first-party evidence where sufficient, while reserving paid research for genuinely difficult cases.

Any shared commercial data must be independently sourced or appropriately licensed. Parallel's ordinary customer terms cannot be assumed to permit building a shared contact database for resale. Customer-private lists, relationships, financial data, and suppression records must remain protected.

The defensible asset is the reliability of the customer workflow and the quality of authorized evidence, rather than the underlying email verification API alone.

## Commercial validation and roadmap

**First 14 days:** Interview approximately twenty practitioners, inspect recent prospecting workflows, identify recurring operational costs, and recruit five unrelated paying design partners. Resolve provider-licensing questions and establish clear product ownership boundaries.

**First 30 days:** Benchmark approximately 1,200 rights-cleared accounts across three customer segments against customers' existing tools. Measure correct-role coverage, independent matching accuracy, uncertain outcomes, research expense, and manual correction time.

**First 60 days:** Run second customer audits, reduce unnecessary research spending, improve the review workflow, and test a limited CRM writeback process if customers request it.

**First 90 days:** Establish whether customers repeat their purchases, whether the workflow produces a measurable advantage, and whether provider and support costs support attractive contribution margins.

Proposed investment gates include at least five paid audits, three repeat-paying customers, positive contribution margin, and either a meaningful improvement in correctly matched approved contacts or a substantial reduction in manual cleanup time without sacrificing quality.

These are future success criteria, not achieved performance figures.

## Measures of success

Verify's primary metric is **total cost per correctly matched, usable, role-relevant B2B contact**, including unsuccessful research and human review.

Supporting metrics include contact coverage per target account, false company/person/role matches, outdated employment records, verification uncertainty, manual review time, research provider cost, time to usable export, repeat customer usage, customer retention, and gross margin.

High email-verification volume alone does not establish success.

## Long-term vision

Verify aims to become the trusted contact quality layer for businesses whose sales activity depends on accurate information about other companies and the people working within them.

It begins with pre-campaign list assessment and targeted repair, then earns the opportunity to maintain customer-approved contact data over time.

The long-term product could support continuous data quality, role-change alerts, evidence-backed CRM updates, and embedded contact intelligence across business workflows.

**The immediate priority is proving that customers will repeatedly pay for better, more reliable prospect lists.** Only after that should Verify expand into a broader contact intelligence platform.

**Company:** PlaybookMedia  
**Product:** Verify  
**Category:** B2B Contact Intelligence, Verification & Data Quality  
**Stage:** Technically launched; commercial product-market fit unproven  
**Initial customer:** Boutique B2B outbound agencies  
**Core promise:** Better business contacts, less manual correction, and predictable research costs.