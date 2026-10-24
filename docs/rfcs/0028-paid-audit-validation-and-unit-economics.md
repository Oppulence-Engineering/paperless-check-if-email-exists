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

## Technical specification

Use the [shared types](verify-technical-contracts.md) for IDs, timestamps, USD
micro-units, and retention. Stage 0 is an offline, tenant-private evaluation bundle;
it does not depend on building the later audit database or a metrics dashboard.

### Versioned evaluation bundle

Propose `scripts/benchmark_contacts_v2.py <bundle-directory> --output <report.json>`.
Keep the existing script/input format unchanged. Each v2 bundle includes:

| File | Required columns/fields | Unique key |
|---|---|---|
| `manifest.json` | `schema_version=2`, cohort UUID, tenant UUID, frozen UTC window, segment keys, provider/config versions, file SHA-256 hashes, registered success thresholds, labor rates, random seed | Cohort/version |
| `accounts.csv` | account ID, segment, baseline eligibility, exclusion reason registered before execution | account ID |
| `results.csv` | provider, account ID, source-row ID, candidate ID or empty for no-match, tenant contact key, mailbox result, policy state, decision/export version, observed time | provider/account/candidate (one no-match row per account without candidates) |
| `judgments.csv` | provider, candidate ID, blinded reviewer ID, company match, person match, employment, role relevance, evidence snapshot hash, judged time | provider/candidate/reviewer |
| `costs.csv` | event ID, provider, account or run allocation group, category, amount micros or blank, measurement basis, external receipt ID, reversal-of ID | event ID |
| `labor.csv` | event ID, provider, audit/account allocation group, role, active seconds, USD hourly rate micros, measured/estimated/unknown | event ID |
| `payments.csv` | audit UUID, pseudonymous customer ID, cash transaction ID, paid/refunded micros, timestamp, first/repeat purchase relation | cash transaction ID |

Allowed judgment values are `true`, `false`, `unknown`. Policy states are `eligible`,
`blocked`, `unknown`; mailbox outcomes are `ok`, `catch_all`, `invalid`, `unknown`,
`failed`, `not_checked`. Empty numeric cells mean unmeasured, not zero. Reject
negative/overflow/nonfinite amounts, duplicate event IDs, undeclared providers,
unknown accounts, broken reversals, and segment reassignment after cohort freeze.

Stage 0 assigns opaque UUIDs even before RFC 0029 exists. Later import stores
explicit aliases from evaluation source-row IDs to real audit/input UUIDs; it must
not silently remap a published cohort. Raw identities/evidence live in restricted
source records; aggregate bundles use tenant HMAC contact keys with a key version.
Use the same approved contact normalization across providers. Do not deduplicate
by company alone, or collapse different people sharing a generic mailbox.

### Metric algorithm

1. Verify hashes and cohort completeness: every provider has exactly the registered
   account set, including failure/no-match rows. A pre-registered exclusion is
   applied to all providers and reported with counts.
2. Resolve independent judgments. Two agreeing judgments produce true/false;
   disagreement or missing required review remains unknown until a third adjudicator
   records a reasoned resolution. The initial dual-review sample is frozen by segment
   and seed in the manifest. Report full-cohort operational yield separately from
   adjudicated yield; unsampled candidates are unknown for this denominator. Never
   derive labels from the product Ready flag. A fully adjudicated small cohort is
   the initial comparable cost-per-usable benchmark; a sampled result must disclose
   its review coverage and must not masquerade as a full-cohort ground-truth ratio.
3. A candidate is usable only if all four judgments are true, mailbox is `ok`,
   policy is eligible, and current evidence/decision versions agree. Group by
   provider + tenant contact key across the cohort; count each contact once.
   Keep per-account coverage separately so deduplication cannot hide an uncovered
   target. If a dedup group contains conflicting judgments, hold the group unknown.
4. Sum all delivery costs for the provider/cohort, including no-match/failure and
   rejected rows. Link refunds to original costs. An unresolved provider attempt
   remains a reservation/unknown expense and makes actual full cost incomplete.
5. Allocate account/run-level costs once, not once per candidate. For segment
   splits, weight shared run cost by registered target-account count; allocate
   integer remainder micros in stable account-ID order and record the rule.
6. Labor cost = active seconds × hourly-rate micros / 3,600, rounded half-up once
   per labor event using decimal arithmetic. Exclude paused timers; enforce unique
   timer event IDs and retain manual corrections as replacement versions.
7. Full cost per usable contact = complete delivery-cost sum / unique usable count.
   Return null with reason `zero_yield` or `incomplete_cost`, not Infinity or zero.
   Keep provider-only cost explicitly named; do not rename the existing metric.

False-match rates use adjudicated candidate counts as denominators and show
unknown/unreviewed counts separately. Coverage denominator is **all registered
eligible accounts**, including failures. Report segment and aggregate metrics;
do not average segment ratios without their denominators. Cash contribution =
net collected revenue minus measured variable delivery costs; fixed overhead and
customer-side remaining labor are separate views.

### Report contract and reproducible example

```json
{
  "schema_version": 2,
  "cohort_id": "40000000-0000-4000-8000-000000000001",
  "provider": "synthetic",
  "input_sha256": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
  "target_accounts": 4,
  "unique_usable_contacts": 2,
  "accounts_with_usable_contact": 2,
  "delivery_cost_micros": "7500000",
  "full_cost_per_usable_contact_micros": "3750000",
  "measurement_complete": true,
  "missing_measurements": [],
  "method_version": "verify-evaluation-v2"
}
```

The fixture above is a **proposed synthetic** acceptance example: provider cost
$0.50 + 600 seconds review at $36/hour ($6.00) + $1.00 allocated infrastructure,
with two usable contacts and two failures. Expected full cost is $7.50 and $3.75
per usable contact; failures remain in coverage and costs. Blank one cost cell:
full-cost ratio becomes null with `incomplete_cost`. Change both usable contacts
to the same identity: denominator becomes one, cost stays $7.50. Change the role
judgments to unknown: denominator zero, ratio null with `zero_yield`.

Emit `measurement_complete` plus missing-measurement details, cohort/config hashes, denominator definitions,
allocation versions, and per-segment error/unknown rates. Bootstrap paired **account**
clusters with a fixed manifest seed and 2,000 draws for exploratory 95% intervals;
keep all candidates from an account together. Do not treat independent candidate
rows as independent accounts. Samples below 30 adjudicated accounts per segment
remain descriptive and cannot unlock comparative marketing claims.

### Automation, privacy, and acceptance

When RFCs 0029–0031 ship, produce bundle inputs from append-only audit/decision/cost
records using a frozen generation and settlement watermark. Late settlement or
adjudication creates a new report version linked to the old one; never edit a
published report in place. Missing matching events are explicit reconciliation
errors. Full source evidence stays under RFC 0027 retention; deleting a customer
removes identifying keys while preserving only permitted aggregate statistics.

Implementation adds v2 fixtures and golden JSON assertions to
`scripts/test_benchmark_contacts.py` or a dedicated v2 test module. Test duplicate
contacts/events, shared-cost allocation, refunds, zero yield, missing expenses,
reviewer disagreement, unequal cohorts, late settlements, and serialization above
JavaScript's safe integer limit. Preserve all current v1 tests and output keys.

Acceptance requires one independently reviewed real cohort report with sampled
invoice/time-log reconciliation and a second run producing identical output from
identical inputs. No network/provider calls are allowed in the reporting script.
