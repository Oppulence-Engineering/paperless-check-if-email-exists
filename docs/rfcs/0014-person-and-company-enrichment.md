# RFC 0014: Person And Company Enrichment, And Job-Change Monitoring

| | |
|---|---|
| **Status** | Draft; architecture revised 2026-10-20 |
| **Created** | 2026-10-16 |
| **Owners** | To assign: backend lead, web lead |
| **Depends on** | RFC 0015 (Data Platform Foundations); RFC 0012 (title taxonomy, departure signal) |
| **Connects to** | RFC 0003 (reverification), RFC 0004 (suppressions), RFC 0005 (list quality), RFC 0008 (remediation) |
| **Release gates** | RFC 0015 Legal Review sign-off, including the DPIA (item 2), before any person enrichment; Definition Of Done met |

## Implementation decision — 2026-10-20

Follow the [company-to-contact integration](../paperless-research-integration.md).
Go owns company/person matching, enrichment, provenance, and research state.
Rust owns MillionVerifier mailbox checks and sending decisions. A deliverable
address is not proof of the individual's identity, current employer, or purchasing
role. Keep observations, source dates, and unresolved uncertainty separately.

The separate Rust enrichment store and compulsory Parallel workflow below are
historical proposals. First ship company imports, missing-address repair, evidence
review, and approved exports. Job-change monitoring and CRM synchronization follow
measured results from that workflow.

## Earlier proposal

Add enrichment for work data:

- **Person enrichment:** give a work or consumer address, and get the person's name, title, seniority, department, current company, and country.
- **Company enrichment:** give a domain, and get the company's name, description, industry, size, and location.
- Each field has its citations and a confidence level.

Enrichment also gives a **job-change signal**. When a person no longer works at the company of their address, that address will probably bounce soon. A **job-change monitor** checks a list again on a schedule. It warns the sender before the next send, and it can suggest a re-check or a suppression.

## Motivation / Problem

- Senders segment and personalize by role, company size, and industry. We give none of this data today.
- B2B addresses decay when people change jobs. Our checks find the problem only after the mailbox is closed and the address bounces.
- Hunter gives person, company, and combined enrichment. Customers who compare tools expect it.

## Buyer Fit

Enrichment has the **strongest fit** with our buyer of the data RFCs:

1. **Job-change warning.** Find contacts who left their company before the next send. This protects the sender's reputation.
2. **Segmentation.** Add role, seniority, industry, and company size to list rows, so the sender can target each send.
3. **Signup routing.** Enrich the company of a new signup, and route the signup by size or industry.

## How We Beat Hunter

| Area | Hunter.io | This RFC |
|---|---|---|
| Evidence | Field values | Field values with citations and a confidence level for each field |
| Job changes | Not a product feature | A job-change signal with a strict precision rule, and a scheduled monitor for lists |
| Freshness | Re-verifies leads monthly | Person sources limited to the last 18 months; snapshots show what changed and when |
| Action | Data only | Job changes become remediation rows, re-check suggestions, and optional suppressions |
| Consumer addresses | Supported | Supported, with a stricter identity check and work data only |
| Measured quality | Not published | Fill rate and reviewer-confirmed accuracy, compared with Hunter before release |

## Goals

- Enrich one person by work or consumer address, one company by domain, or both in one call.
- Enrich a full list as a background job, with an estimate and the RFC 0015 approval step.
- Return citations and a confidence level for each field.
- Detect job changes with high precision.
- Monitor a list for job changes on a schedule.
- Connect job changes to remediation, reverification, and suppressions.
- Cache results across tenants, so that the same lookup does not cost us twice.

## Non-Goals

- Personal data beyond work data, for any address type: no age, gender, photos, home address, personal phone numbers, or special category data.
- Lookup by LinkedIn URL.
- Technology stack, funding rounds, and web traffic in v1.
- Automatic suppression by default. The tenant must turn it on.

## Current State In Reacher

- `/v1/reputation/check` gives domain reputation (blacklists, DNS records). It gives no company data.
- `misc.is_b2c` and `is_role_account` are the only person or company signals in a check result.
- `GET /v1/jobs/{job_id}/approval` returns a quality checklist. **It does not hold a job.** This RFC uses the hold step from RFC 0015 section 6.
- `suppression_reason` has the values `manual`, `bounce`, `invalid`, `spam_trap`, `unsubscribe`, `complaint`, and `auto_invalid`. There is no value for a job change.
- Remediation rows support `classification` values such as `review` and `suppress`, with a `rule_id` and `reasons`.
- `reverification_schedules` runs scheduled re-checks for each tenant.
- RFC 0012 adds the title taxonomy and the `possibly_left` status for people.

## Proposed Design

### 1. Field Dictionary

**Person fields:**

| Field | Type and format | Source |
|---|---|---|
| `full_name` | Text, at most 200 characters | Parallel |
| `current_title` | Text, at most 200 characters | Parallel |
| `seniority` | RFC 0012 value | Derived from `current_title` |
| `department` | RFC 0012 value | Derived from `current_title` |
| `current_company_domain` | Normalized domain | Parallel |
| `location_country` | ISO 3166-1 alpha-2 | Parallel |
| `employment_status` | `current`, `left`, `unknown`, `not_applicable` | Derived (section 3) |

**Company fields:**

| Field | Type and format | Level |
|---|---|---|
| `name` | Text | `basic` |
| `description` | Text, at most 500 characters | `basic` |
| `industry` | One value from a fixed list of 30 industries | `basic` |
| `employee_range` | `1-10`, `11-50`, `51-200`, `201-1000`, `1001-5000`, `5001+` | `basic` |
| `hq_country` | ISO 3166-1 alpha-2 | `basic` |
| `hq_city` | Text | `full` |
| `founded_year` | Integer, 1800 to the current year | `full` |
| `company_type` | `private`, `public`, `nonprofit`, `government`, `education` | `full` |
| `parent_company_domain` | Normalized domain | `full` |
| `revenue_range` | `<1M`, `1M-10M`, `10M-50M`, `50M-250M`, `250M-1B`, `1B+` (USD) | `full` |

Each output schema uses `enum` for fixed lists. Validation (RFC 0015 section 10) drops values outside the format.

### 2. Person Enrichment

**Input rules:**

- A work address and a consumer address (a free provider, such as Gmail) are both accepted.
- A role address (`info@`) returns `422` with `role_address_not_supported`. A role address does not belong to one person.
- An opted-out address returns `451`.

**Call:** Task `base`, with these settings:

- `source_policy.after_date` = today minus 18 months, so old pages do not count.
- The input has the address and its domain.

The output schema:

```json
{
  "type": "object",
  "properties": {
    "full_name": { "type": "string" },
    "current_title": { "type": "string" },
    "current_company_domain": { "type": "string" },
    "location_country": { "type": "string" },
    "still_at_email_domain": { "type": "boolean" }
  }
}
```

For a consumer address, the schema has no `still_at_email_domain` field, because the domain says nothing about the employer.

**Identity check.** This check stops results about another person with the same name.

| Address type | We keep the result only when |
|---|---|
| Work | A citation excerpt contains the address, or `full_name` matches the local part through one of the RFC 0011 patterns. |
| Consumer | A citation excerpt contains the address. A name match is not enough, because many people share a name and a consumer local part. |

### 3. Employment Status (Job-Change Signal)

`left` must be precise. A false `left` can make a tenant drop a good contact. All four conditions must be true for `left`:

1. `still_at_email_domain` is `false`, with `high` confidence in the basis.
2. `current_company_domain` is present, and it is not the address domain.
3. `current_company_domain` is not the parent or a subsidiary of the address domain (company enrichment `parent_company_domain`).
4. The tenant has no `delivered` outcome for this address in the last 14 days, unless RFC 0012 also marks the person `possibly_left`.

Other cases:

- `current`: `still_at_email_domain` is `true`, with `medium` or `high` confidence.
- `not_applicable`: the address is a consumer address. A job change does not make a consumer address bounce.
- `unknown`: all other cases.

For all address types, a change of `current_company_domain` between two snapshots sends the event `contact.company_changed` (section 7).

The response also gives `employment_evidence`: the conditions that passed, and the citations.

### 4. Company Enrichment

1. Read the cache.
2. On a miss, run Task `base` (`basic`) or Task `core` (`full`).
3. A domain that redirects to a parent company returns `parent_company_domain`, and keeps the requested domain as the key.

### 5. Combined Enrichment

`combined` runs the person task and the company task at the same time. The company domain is the domain of the address. The company result is also used for condition 3 in section 3.

### 6. Field Basis

For each field we store and return the Parallel `output.basis` entry: `confidence` (`low`, `medium`, `high`, or null) and citations.

- By default the API hides fields with `low` or null confidence.
- `include_low_confidence=true` shows them, marked as low.

### 7. Cache And Snapshots

| Item | Key | Fresh for |
|---|---|---|
| Person cache | HMAC of the address (RFC 0015 pepper) | 30 days |
| Company cache | Domain and level | 90 days |

Rules:

- A cache hit costs us nothing. The tenant pays normal credits, unless it already paid for the same resource in this period.
- `v1_enrich_person_snapshot` keeps the last 3 results for each person. We compare each new result with the last one to find changes in title and company.
- An opt-out deletes the cache row and the snapshots.

### 8. Response Time

- `wait_seconds` (0–10, default 5) returns the result inline when it is ready in time. Otherwise the response is `202` with `job_id`, and the event `enrich.completed` follows.
- For signup routing, company enrichment is the right call: the domain cache answers most requests in under 300 milliseconds.

### 9. List Enrichment

`POST /v1/lists/{list_id}/enrich`:

1. **Count.** Count the unique addresses and domains in the list. Skip role addresses. Remove cache hits that this tenant already paid for in this period.
2. **Estimate.** Return the credit estimate. `dry_run: true` stops here.
3. **Approve.** An estimate above the tenant threshold waits in `awaiting_approval` (RFC 0015 section 6).
4. **Run.** Send the missing lookups through the Parallel Task Group API in batches of up to 100.
5. **Write.** Store the fields in `v1_list_row_enrichment`. List downloads include them as columns.
6. **Act.** Each row with `employment_status = left` becomes a remediation row with `classification = review` and `rule_id = contact_left_company`.

### 10. Job-Change Monitor

`POST /v1/lists/{list_id}/job-change-monitor` turns on a monitor:

```json
{ "interval_days": 30, "auto_suppress": false, "recheck_on_left": true }
```

- The runner creates an `enrich_monitor` job at each interval.
- The job enriches again only the rows (except role addresses) whose last enrichment is older than the interval. Cache hits are reused.
- The `left` signal applies to work addresses only. Consumer rows can send `contact.title_changed` and `contact.company_changed`.
- Each estimate above the threshold needs approval, as in section 9.
- For each new `left` row:
  - send the event `contact.left_company`
  - when `recheck_on_left` is true, queue a re-check of the address (RFC 0003)
  - when `auto_suppress` is true, add a suppression with the new reason `left_company` (RFC 0004)
- For each changed title, send the event `contact.title_changed`.
- The list quality summary (RFC 0005) shows the number of `left` rows.

## Public API / Interface Changes

- `GET /v1/enrich/person?email=jane.doe@acme.com&include_low_confidence=false&wait_seconds=5`
- `GET /v1/enrich/company?domain=acme.com&level=basic`
- `GET /v1/enrich/combined?email=jane.doe@acme.com&level=basic`
- `GET /v1/enrich/jobs/{job_id}`
- `POST /v1/lists/{list_id}/enrich` with `{ "fields": ["person", "company"], "level": "basic", "dry_run": false }`
- `POST /v1/lists/{list_id}/job-change-monitor`, `GET` and `DELETE` on the same path

Person response:

```json
{
  "email": "jane.doe@acme.com",
  "person": {
    "full_name": "Jane Doe",
    "current_title": "VP Marketing",
    "seniority": "executive",
    "department": "marketing",
    "current_company_domain": "acme.com",
    "location_country": "DE",
    "employment_status": "current"
  },
  "basis": {
    "current_title": {
      "confidence": "high",
      "citations": [{ "url": "https://acme.com/team", "title": "Our team" }]
    }
  },
  "employment_evidence": { "conditions_passed": [], "citations": [] },
  "changes": [{ "field": "current_title", "from": "Marketing Director", "to": "VP Marketing", "detected_at": "2026-10-12T09:00:00Z" }],
  "schema_version": "person_v1",
  "enriched_at": "2026-10-12T09:00:00Z",
  "credits_charged": 1
}
```

Status codes:

| Status | When |
|---|---|
| `200` | Data returned |
| `202` | The run takes longer than `wait_seconds` |
| `404` | Parallel found no data (no charge) |
| `422` | `role_address_not_supported` |
| `502` | The Parallel run failed (no charge) |

`402`, `429`, `451`, `501`, and `503` work as in RFC 0015.

Scope:

- `enrich` for single lookups.
- `enrich` and `lists` for list enrichment and monitors.

Events:

- `enrich.completed`
- `enrich.list.completed`
- `contact.left_company` (work addresses only)
- `contact.title_changed`
- `contact.company_changed`

## Data Model / Storage Changes

```sql
CREATE TABLE v1_enrich_person_cache (
  email_hmac     BYTEA PRIMARY KEY,
  domain         TEXT NOT NULL,
  data           JSONB NOT NULL,
  basis          JSONB NOT NULL,
  schema_version TEXT NOT NULL,
  enriched_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE v1_enrich_person_snapshot (
  id             BIGSERIAL PRIMARY KEY,
  email_hmac     BYTEA NOT NULL,
  data           JSONB NOT NULL,
  enriched_at    TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_v1_enrich_person_snapshot_key ON v1_enrich_person_snapshot (email_hmac, enriched_at DESC);

CREATE TABLE v1_enrich_company_cache (
  domain         TEXT NOT NULL,
  level          TEXT NOT NULL,               -- basic | full
  data           JSONB NOT NULL,
  basis          JSONB NOT NULL,
  schema_version TEXT NOT NULL,
  enriched_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY (domain, level)
);

CREATE TABLE v1_list_row_enrichment (
  tenant_id      UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
  list_id        INTEGER NOT NULL REFERENCES v1_lists(id) ON DELETE CASCADE,
  row_index      INTEGER NOT NULL,
  person         JSONB,
  company        JSONB,
  employment_status TEXT,
  enriched_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY (list_id, row_index)
);
CREATE INDEX idx_v1_list_row_enrichment_tenant_status
  ON v1_list_row_enrichment (tenant_id, employment_status);

CREATE TABLE v1_job_change_monitor (
  id             BIGSERIAL PRIMARY KEY,
  tenant_id      UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
  list_id        INTEGER NOT NULL REFERENCES v1_lists(id) ON DELETE CASCADE,
  interval_days  INTEGER NOT NULL DEFAULT 30,
  auto_suppress  BOOLEAN NOT NULL DEFAULT false,
  recheck_on_left BOOLEAN NOT NULL DEFAULT true,
  next_run_at    TIMESTAMPTZ NOT NULL,
  last_run_at    TIMESTAMPTZ,
  last_data_job_id BIGINT REFERENCES v1_data_job(id) ON DELETE SET NULL,
  created_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  UNIQUE (list_id)
);

ALTER TYPE suppression_reason ADD VALUE 'left_company';
```

`ALTER TYPE ... ADD VALUE` cannot run inside a transaction block on older Postgres versions. It goes in its own migration file.

## Product Surface (UI)

This follows the RFC 0015 UI rules: results first, and complex controls in sheets.

**Enrich page:**

1. A compact input for an address or a domain.
2. The result comes first: a person card and a company card. Each field has a `ConfidenceBadge`, and opens its `CitationList`.
3. An employment status badge shows `current`, `left`, or `unknown`, and opens the evidence.
4. An "Options" sheet holds the level and "Show low-confidence fields".

**Lists:**

1. "Enrich list" opens a sheet. The sheet has the fields, the level, the estimate from `dry_run`, and approval.
2. The new columns appear in the list table.
3. A "Job changes" filter shows `left` rows.
4. "Monitor job changes" opens a sheet with the interval, the re-check option, and the auto-suppress option. The auto-suppress option has a clear warning.
5. The remediation tab shows the `contact_left_company` rows.

## Security And Privacy

- Role addresses are refused.
- Consumer addresses are accepted with three safeguards:
  - the strict identity rule (a citation must contain the address)
  - work data only
  - `employment_status = not_applicable`
- The DPIA (RFC 0015 Legal Review item 2) must cover consumer addresses before person enrichment ships.
- No special category field in any schema. A test checks every schema and every stored row.
- Cache keys use the RFC 0015 HMAC, not a plain hash.
- Rate limits for each tenant: 10 requests a second and 300 a minute by default.
- Auto-suppression is off by default. Turning it on is logged with the user and the time.
- RFC 0015 validation applies to all Parallel output.

## Observability And SLOs

Metrics:

- fill rate for each field
- the confidence mix
- the cache hit rate
- the `left` rate
- job-change precision against later bounces
- monitor runs and their cost
- Parallel cost for each enriched row

| SLO | Target |
|---|---|
| p95, cache hit | 300 milliseconds or less |
| p95, cache miss (person or company `basic`) | 10 seconds or less |
| List enrichment throughput, cache misses | 1,000 rows an hour or more (to confirm against Parallel rate limits) |

## Cost Model

Parallel list prices (October 2026):

| Lookup | Processor | Price |
|---|---|---|
| Person | `base` | $0.010 |
| Company `basic` | `base` | $0.010 |
| Company `full` | `core` | $0.025 |

Our billing:

| Event | Credits |
|---|---|
| Person found | 1 |
| Company `basic` found | 1 |
| Company `full` found | 2 |
| Combined | The sum of the parts |
| Nothing found, refused, or failed | 0 |
| Same tenant, same resource, same period | 0 |

A cache miss on `full` costs us $0.025, so `full` costs 2 credits. This keeps the cost for each credit near the $0.010 planning cost for an 85% gross margin (RFC 0015 section 16). The cache lowers the cost further.

## Failure Modes And Edge Cases

- **Two people with the same name:** the identity check (section 2) drops results that do not match the address.
- **A consumer address that no cited page contains:** return `404` and charge 0.
- **A person changed jobs inside the same group of companies:** condition 3 keeps the status `current` or `unknown`.
- **Old pages say the person is still there:** `after_date` ignores pages older than 18 months.
- **A Task run fails:** a single lookup returns `502` and is not charged. In a list job, the row is marked `enrich_failed`, and the job continues.
- **The tenant runs out of credits during a list job:** the job moves to `awaiting_approval`, and asks for approval to continue.
- **A monitor run finds many `left` rows at once** (for example, after a company closed): events are batched, with at most 1 event each second for each tenant.
- **The tenant deletes the list:** the monitor and the enrichment rows are deleted through `ON DELETE CASCADE`.

## Alternatives Considered

- **The `core` processor for people.** It costs 2.5 times more. Five fields cover the buyer's need.
- **A bought firmographic database.** It is cheaper for each lookup, but it has a licence fee and no citations. We can add it later behind the same API.
- **An LLM for seniority and department.** The RFC 0012 keyword rules are free and easy to test.
- **Parallel Monitor for job changes.** Monitor watches web events, but it costs money on every run for every contact. Re-enrichment on an interval reuses the cache and is easier to budget.

## Rollout / Migration / Compatibility

1. Ship after RFC 0015 M1–M5.
2. Ship single company enrichment first. It has the lowest privacy risk.
3. Ship person enrichment and the job-change signal after the Legal Review and DPIA sign-off.
4. Ship list enrichment.
5. Ship the job-change monitor and the connections to RFC 0003, RFC 0004, RFC 0005, and RFC 0008.

This RFC adds endpoints, tables, and one enum value. No current behavior changes.

## Milestones

| # | Deliverable | Size |
|---|---|---|
| E1 | Field dictionary, schemas, validation, caches | M |
| E2 | Company enrichment (`basic` and `full`) | S |
| E3 | Person enrichment, identity check, employment status | M |
| E4 | List enrichment with Task Group batches | M |
| E5 | Snapshots, change detection, events | S |
| E6 | Job-change monitor and connections to RFC 0003, 0004, 0005, 0008 | M |
| E7 | Enrich page, list sheets, job-change views | M |

## Metrics And Success Criteria

- Job-change precision and recall (see Definition Of Done).
- The bounce rate of lists after the tenant acts on job-change rows, compared with lists where the tenant did not act.
- Fill rate and confirmed accuracy for each field.
- The number of lists with an active monitor.
- The cache hit rate and Parallel cost for each enriched row.

## Test Strategy

- Unit tests for each `employment_status` rule, including each of the four `left` conditions on its own.
- Unit tests for the identity check, with same-name fixtures.
- Unit tests for field validation: enums, ISO codes, year ranges, and lengths.
- Unit tests for change detection between snapshots.
- Unit tests for credit estimates on list jobs, with cache hits and repeats in a period.
- A test that no schema, request, or stored row contains a special category field.

## Definition Of Done

This RFC is done only when all items below are true. "Passes" means it passes in CI on the pull request.

### Build

1. RFC 0015 M1–M5 and RFC 0012 D2–D3 are done.
2. The migrations run up and down, including the separate enum migration. `e2e_migration_upgrade` passes.
3. `backend/openapi.json`, the SDKs, and the web clients show the new endpoints. CI finds no diff after generation.
4. The `enrich` scope exists.
5. The Parallel stub supports single Task runs, slow runs, failed runs, and Task Group batches.

### Unit And API Tests

6. All unit tests in Test Strategy pass in the `test` CI job.
7. The API harness has a canonical case for each new endpoint. Each case covers:
   - success
   - `403` without the scope
   - `404` for another tenant's job or list
   - for list endpoints, `403` with `enrich` but without `lists`

### End-To-End Tests (Backend)

8. `backend/tests/e2e_enrichment.rs` runs HTTP, Postgres and RabbitMQ (with `testcontainers`), the data job runner, the approval step, and the Parallel stub. It passes these scenarios:

| # | Scenario | Expected result |
|---|---|---|
| a | Person found | Fields and basis; `low` fields hidden; `include_low_confidence=true` shows them; 1 credit |
| b | Each `left` condition fails on its own | Status is not `left` in each case |
| c | All `left` conditions pass | `left`, with `employment_evidence` |
| d | Same-name result that does not match the address | Dropped; `404`; 0 credits |
| e | Company `basic`, company `full`, and `combined` | 1, 2, and the sum of the parts in credits |
| f | Cache hit, same tenant, same period | 0 credits, 0 stub calls |
| g | Cache hit, another tenant | Normal credits, 0 stub calls |
| h | Consumer address with a citation that contains it | Fields returned; `employment_status = not_applicable`; 1 credit |
| h2 | Consumer address with a name match only | Dropped; `404`; 0 credits |
| h3 | Role address | `422` `role_address_not_supported` |
| i | Opted-out address | `451`; cache row and snapshots deleted |
| j | Slow run | `202`, then the job returns the result; `enrich.completed` sent |
| k | Not found, and a failed run | `404` and `502`; 0 credits |
| l | List enrichment with `dry_run: true` | Correct estimate without paid cache hits; no job |
| m | List enrichment above the threshold | `awaiting_approval`; after approval, rows enriched through the Task Group stub; downloads include the columns |
| n | List rows with `left` | Remediation rows with `contact_left_company` |
| o | Credits run out during a list job | Job moves to `awaiting_approval` |
| p | Monitor run with `recheck_on_left` and with `auto_suppress` | `contact.left_company` events; re-check queued; suppression with reason `left_company` only when `auto_suppress` is on |
| q | A title changed between two runs | `changes` in the response; `contact.title_changed` sent |
| r | Schemas and stored rows | No special category field |

### End-To-End Tests (Web)

9. Playwright specs cover these journeys:
   - Enrich one address, see the fields with citations, and open the employment evidence.
   - Enrich a list: see the estimate, approve, and see the new columns.
   - Filter "Job changes", and open the remediation tab.
   - Turn on a monitor with the auto-suppress warning.
10. The specs have the `@smoke` tag, and `test:a11y` passes. Each new component has a story and a passing component test.
11. `ci/smoke_full_stack_image.bash` calls company enrichment on the built image and checks the response.

### End-To-End Tests (Live, Staging)

12. A live test runs with the real Parallel API when `PARALLEL_LIVE_E2E=1`.
    - It uses 1,000 work contacts from our internal tenant: 500 with a recent `delivered` outcome and 500 with a recent hard bounce. It also uses 200 consumer contacts and 500 company domains.
    - Reviewers check 300 work person rows, 100 consumer person rows, and 300 company rows.
    - The pass targets (set 2026-10-16) are:

    | Metric | Target |
    |---|---|
    | Job-change precision: `left` flags that are hard bounces, or that a reviewer confirms | 95% or more |
    | Job-change recall: hard-bounced contacts that we flag `left` | 70% or more |
    | Contacts with a recent `delivered` outcome that we flag `left` | 2% or fewer |
    | Person lookups with `current_title` at `medium` or `high` confidence | 85% or more |
    | Person fields that a reviewer confirms as correct | 97% or more |
    | Consumer-address results about the wrong person | 1% or fewer |
    | Company `basic` lookups with all 5 fields | 95% or more |
    | Company fields that a reviewer confirms as correct | 98% or more |
    | p95, cache hit | 300 milliseconds or less |
    | p95, cache miss | 10 seconds or less |
    | Parallel cost for each enriched row | 10% or less of the list price of the credits that we charge |

    If one target is not met, the feature does not leave beta.
13. **Hunter benchmark.** We run the same contacts and domains through the Hunter enrichment API, after RFC 0015 Legal Review item 13 has a sign-off. We must match or beat Hunter on two metrics:
    - the field fill rate
    - the share of fields that reviewers confirm
14. **Monitor soak.** A monitor runs on 3 internal lists for 2 intervals on staging. It shows correct estimates, approvals, events, and costs.
15. **Load.** The RFC 0015 load test holds the SLOs.
16. The reports (our results, Hunter's results, monitor soak, load, and cost) are attached to the release pull request.

### Operations And Release

17. The dashboards show all metrics in this RFC, including job-change precision.
18. The RFC 0015 Legal Review has a sign-off, including the DPIA (item 2). Person enrichment does not ship without it.
19. The public API docs and the changelog describe the endpoints, the fields, the job-change signal, the monitor, and the credits.

## Open Questions

1. Which fixed list of 30 industries do we use? Owner: product.
2. Is 14 days the right window for the `delivered` outcome rule in section 3? Owner: backend lead, after the live test.

## Unresolved Risks

1. **Legal.** Person enrichment has the highest privacy risk of the data RFCs. The RFC 0015 Legal Review and the DPIA must approve it before release.
2. **Job-change accuracy.** Public pages can be late or wrong. The four-condition rule favors precision over recall. If recall stays below 70%, the feature stays in beta.
3. **Cost.** Monitors re-enrich many rows. The margin depends on the cache hit rate and on interval choices. The approval step keeps each run visible.
