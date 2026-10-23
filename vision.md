# Verify vision

**Updated October 23, 2026 · Product of PlaybookMedia**

Verify becomes the trusted contact quality layer for businesses whose sales work
depends on accurate information about companies and the people working there.
It earns that position through reliable, repeatable audits of customer-owned
prospect lists, then maintains customer-approved records over time.

The [company thesis](company.md) defines the buyer and business model. The
[audit](docs/audits/2026-10-23-verify-north-star.md) records today's implementation;
the [RFC index](docs/rfcs/README.md) is the authoritative delivery sequence.

## North star

**Total cost per correctly matched, usable, role-relevant B2B contact.**

For a fixed audit cohort:

```text
total delivery cost = research + verification + allocated infrastructure
                    + human research/review + variable delivery/support labor

cost per usable contact = total delivery cost / unique usable contacts
```

Include unsuccessful lookups, no-match accounts, retries, and rejected candidates
in costs. Allocate shared research once. Record the currency, labor rate, time
window, allocation method, and missing measurements. A zero-contact denominator
is undefined, never zero cost; an unmeasured expense is unknown, never free.

A usable contact must pass five independent questions:

1. Is this the correct company?
2. Is this the correct individual?
3. Does the person currently work there?
4. Is the person relevant to the specified role or purchasing decision?
5. Does the mailbox meet the customer's verification and sending requirements,
   including active suppression and complaint rules?

Operational Ready counts are policy decisions. An independently adjudicated
sample is required to establish matching accuracy; a product approval or a title
match alone is not ground truth. Deduplicate contacts within the agreed cohort
before computing yield. The current benchmark script measures provider cost only
and is not yet the complete north-star calculation.

Customer economics are a separate view: fees paid to Verify and other tools plus
remaining customer labor. Do not add Verify's internal delivery costs again to
that customer view. Contribution margin is audit revenue less variable delivery
costs; report fixed overhead separately.

Guardrails are false company/person/role matches, stale employment, suppression
violations, uncertainty rate, coverage, manual review time, maximum charge
adherence, and time to export. Lower cost is useful only while quality is upheld.

## Complete customer journey

| Step | Product behavior |
|---|---|
| Import and organize | Identify client/campaign and target roles; map fields; preserve original IDs, source values, duplicates, and errors. |
| Assess | Evaluate company/domain, person, employer, and role separately, with dated sources and visible contradictions. |
| Plan repair | Select uncertain or incomplete records; explain proposed work and approve a maximum charge before paid calls. |
| Research and verify | Reuse permitted evidence; research selectively; verify mailboxes; retain failures and uncertain results. |
| Review | Persist approve, hold, reject, or request-research decisions with actor, reason, evidence version, and time. |
| Deliver | Produce Ready, Needs Review, and Excluded files with original IDs, corrections, reasons, and evidence; recheck policy at export. |
| Maintain | After repeat demand, detect stale evidence and possible role changes, propose corrections, and obtain approval before writeback. |

A generic company inbox remains a generic inbox. A catch-all or unknown mailbox
remains uncertain. A missing source does not prove departure. Review cannot
override an active suppression or manufacture missing evidence.

## Incremental destination

| Stage | Customer value | Exit evidence |
|---|---|---|
| 0. Measure the existing pilot | A bounded, rights-cleared audit with an agreed scope and baseline. | Ownership and source-use decisions recorded; paid design partners recruited; evaluation protocol and cost log ready. |
| 1. Deliver a complete audit | Traceable imports, durable review, and reconciled correction files. | Every source row accounted for; reload/retry-safe decisions; current policy enforced at delivery. |
| 2. Improve yield and economics | Selective repair within approved spend and measured batch capacity. | Representative three-segment benchmark; no duplicate purchases; full-cost and quality comparisons. |
| 3. Earn repeat workflow | Second audits and optional, previewed CRM updates. | Five paid audits, three repeat payers, positive contribution margin, measured quality or time advantage. |
| 4. Maintain approved data | Targeted freshness checks, role-change review, and recurring repair. | Retained paying cohorts; bounded recurring spend; accurate change alerts and safe updates. |

Stages express dependencies, not promises of launch dates. Commercial interviews,
manual audits, and baseline collection can proceed with the current pilot while
engineering closes the documented gaps. Do not sell an agency-scale automated
workflow before it passes the corresponding acceptance gates.

## Architecture and trust principles

- Reuse the Go research backend and Rust verification platform. Avoid a second
  crawler, research runner, or shared contact database in this repository.
- Keep Next.js as the customer surface, with same-origin requests, server-side
  tenant mapping, and generated API contracts. Auth identity remains separate
  from product records.
- Preserve source observations, inferred claims, reviewer decisions, and mailbox
  results as distinct, versioned records. Re-evaluate when facts or policy change.
- Keep paid operations durable and bounded; retries reuse prior work and unresolved
  provider outcomes require reconciliation before another purchase.
- Keep customer-private evidence out of other tenants and the internal finance
  graph. Enforce source rights, retention, and deletion across both services.
- Let customers inspect proposed exports and CRM changes. Never silently replace
  source data or automatically broaden approved spend.

## What waits

Broad company/people discovery, a shared commercial contact database, newsletter
integrations, signup protection, sender analytics, and MCP distribution are not
prerequisites for proving the agency audit. Existing verification and list hygiene
remain supported. Domain search and richer enrichment advance when they solve an
observed audit gap; CRM and monitoring advance after customer evidence justifies
their costs.

Potential defensibility comes from reliable workflows and authorized evidence:
reviewed company mappings, dated role observations, correction patterns,
verification history, and better matching rules. Reuse beyond a customer's own
workspace requires explicit rights. We make no guaranteed response-rate, inbox
placement, or revenue claim.
