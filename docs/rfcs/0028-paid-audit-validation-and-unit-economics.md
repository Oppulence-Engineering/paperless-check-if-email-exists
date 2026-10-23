# RFC 0028: Paid Audit Validation and Unit Economics

| Field | Value |
|---|---|
| Status | Proposed; starts at stage 0, evaluated at stages 2–3 |
| Updated | 2026-10-23 |
| Owners | Product/commercial owner and evaluation owner; assign before pilot |
| Depends on | RFC 0027 pilot source/ownership gate for real data |
| Later inputs | RFCs 0029–0031 add automatic lineage, decisions, and cost events |

## Problem and outcome

The [benchmark](../../scripts/benchmark_contacts.py) measures manually adjudicated
matches and provider cost on equal account cohorts. It excludes labor and
infrastructure, relies on input deduplication, and has only a synthetic fixture.
Neither a successful deployment nor withheld catch-all export proves willingness
to pay, useful yield, or profitable delivery.

Establish an auditable paid-pilot record before building a larger platform. A
controlled private ledger and manual delivery are acceptable first increments;
an analytics dashboard or new billing integration is not a prerequisite.

## Experiment and measurement contract

- Recruit approximately 20 interviewees and five unrelated paying design partners.
  Record consent/data rights, buyer segment, current tools, list age, target role,
  batch size, allowed spend, delivery deadline, and baseline process.
- Test the $249 audit as a fixed-scope offer. Record actual cash collected,
  discounts/refunds, invoice reference, first/second audit, and a stable customer
  identifier. An interview, free trial, or internal account is not a paid audit.
- Pre-register comparison thresholds and a quality non-inferiority rule before
  looking at outcomes. Use the customer's competently configured tools on the
  same rights-cleared accounts and target roles, with comparable execution dates
  and budgets. Version configurations and report material differences.
- Build toward approximately 1,200 accounts across three segments. Begin with
  bounded samples; RFC 0031 gates larger automated batches. Do not silently drop
  no-match accounts or remove difficult cases after seeing results.
- Independently adjudicate company/person match, current employment, and role
  relevance as true/false/unknown. Blind reviewers to provider where possible;
  use a second reviewer on a stratified sample, retain disagreements as unknown,
  and record source dates. Include catch-all, stale, failed, and no-match cases.

## Data and calculations

Start with tenant-private evaluation tables/files keyed by `audit_id`, account,
source-row ID, candidate ID, provider/config version, and evaluation window. Keep
identifying evidence private; aggregate reports need no names or email addresses.

Store separate ledgers for:

1. **Outcomes:** independent judgments, evidence references, mailbox outcome,
   active policy/suppression eligibility, reviewer decision, export version, and
   unique contact identity within the cohort. Product approval is not adjudication.
2. **Delivery costs:** provider attempts including failures, refunds/credits,
   shared account costs allocated once, verification, human research/review
   minutes and loaded hourly rate, infrastructure allocation, and variable support
   labor. Currency, measured/estimated/unknown basis, and date are mandatory.
3. **Customer effort and revenue:** fees, remaining cleanup time, time to usable
   export, customer baseline cost, repeat payment, and cash contribution margin.

The [vision](../../vision.md#north-star) defines the full-cost numerator. Divide
by unique independently supported, role-relevant, policy-eligible usable contacts.
Unknown costs make full-cost metrics incomplete; zero yield produces an undefined
ratio plus explicit spend and zero-contact counts. Do not equate quota reservations
with measured USD or add internal delivery costs to customer fees a second time.

Extend the existing script additively: retain its provider-only metric with that
label, add a separately named full-cost metric, explicit contact deduplication,
policy eligibility, false-role-match counts, and measurement completeness. Publish
per-segment coverage, errors, uncertainty, review time, and sample sizes alongside
aggregate cost. Use intervals/disagreement rates where samples permit inference.

## Investment gates and deliverables

| Review | Required evidence |
|---|---|
| Day 14 | Interview synthesis, five unrelated paying design partners targeted, rights review, registered baseline and success thresholds. |
| Day 30 | Three-segment benchmark toward 1,200 accounts with actual costs, matching accuracy, uncertainty, and manual time; disclose incomplete coverage. |
| Day 60 | Second audits, measured changes in spend/review time, and specific customer requests before CRM work. |
| Day 90 | At least five paid audits, three repeat-paying customers, positive contribution margin, and the registered quality/yield or cleanup-time improvement without quality regression. |

The clock starts when the validation program starts. These are future gates.
Failure means revise the offer, segment, or workflow and repeat a bounded test;
it does not unlock discovery, automation, or unsupported marketing claims.

## Acceptance and rollout

- Reproduce each report from versioned private inputs; unequal cohorts, duplicate
  contact IDs, missing costs, zero yield, and disagreement have explicit behavior.
- Reconcile a sample of provider costs to billing evidence and human time logs;
  reconcile paid/repeat counts to payments. Report refunds and unsuccessful work.
- The first report can be manual using the current ten-row pilot. Later ingest
  immutable events from RFCs 0029–0031 without changing metric definitions.
- Do not turn the experimental audit price into existing subscription entitlements
  or deploy new charging behavior as part of evaluation tooling.
- CI fixtures remain synthetic and are labeled accordingly. Live comparisons and
  retention results must never be inferred from passing fixtures.
