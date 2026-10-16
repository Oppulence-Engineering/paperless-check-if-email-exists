# RFC 0012: Domain Search

| | |
|---|---|
| **Status** | Draft |
| **Created** | 2026-10-16 |
| **Owners** | To assign: backend lead, web lead |
| **Depends on** | RFC 0015 (Data Platform Foundations), RFC 0011 (domain sourcing, pattern model) |
| **Used by** | RFC 0013 (row action "Open Domain Search"), RFC 0014 (departure signal) |
| **Release gates** | RFC 0015 Legal Review sign-off; Definition Of Done met |

## Summary

Add Domain Search: give a domain or a company name, and get the people and the generic addresses at that company. Each row has:

- a name, a position, a seniority, and a department
- an address, with its method, its calibrated confidence, and its verification status
- source URLs, and the last time we saw the person

Rows can be exported to a list. People who are no longer on the company's pages are marked `possibly_left`, and this signal feeds the job-change feature in RFC 0014.

**Decision (2026-10-16):** Domain Search accepts any domain. It is not limited to domains in the tenant's lists.

## Motivation / Problem

- We cannot show who works at a company. A sender who loses a contact at an account has no way to find a replacement.
- RFC 0011 sources the published addresses of a domain, but it shows them one person at a time.
- Hunter's Domain Search is one of its core features. Customers who compare tools expect it.

## Buyer Fit

Our buyer already has a list and sends email to it. Domain Search serves this buyer as an **account coverage** tool:

1. **Replace lost contacts.** A contact at a customer account bounces or leaves. Domain Search shows other people at the same account.
2. **Find gaps in a list.** Show the accounts in a list that have only generic addresses, such as `info@`.
3. **Mark generic addresses before a send.** Show which addresses at a domain are generic, so the sender can segment or suppress them.
4. **Find departures early.** `possibly_left` people are likely to bounce soon.

The main entry point is "Find more contacts at this account" from a list. A standalone search page also exists.

## How We Beat Hunter

| Area | Hunter.io | This RFC |
|---|---|---|
| Freshness | Re-verifies its database | Shows `last_seen_at` for each person, and marks people who left |
| Confidence | A score per address | A calibrated score per address (RFC 0011) |
| Titles outside English | Not documented | Seniority and department rules in 7 languages |
| Large companies | First 10,000 rows | Department-by-department sourcing for companies with more than 500 people |
| Use with lists | Save leads | `in_your_lists` on each row, and export into the tenant's list and verification flow |

## Goals

- Return up to 100 rows a page, with filters for type, seniority, department, verification status, and departed people.
- Return a free count of what we know for a domain.
- Serve from the store when it is fresh. Source the domain again only when the data is old.
- Find people without a published address, and give them an address from the dominant pattern.
- Export selected rows to a list.
- Charge 1 credit for each 5 returned addresses, to keep an 85% gross margin (RFC 0015 section 16).

## Non-Goals

- Search across many domains in one call. RFC 0013 covers discovery.
- Person enrichment beyond name and position. RFC 0014 covers it.
- Phone numbers and social profiles.

## Current State In Reacher

- RFC 0011 adds domain sourcing (Stage A known pages, Stage B roster task), the pattern model, and calibrated confidence.
- RFC 0015 adds the store (`v1_web_domain`, `v1_web_email`, `v1_web_source`), the job runner, credits, and single-flight.
- `is_role_account` in the check result marks generic local parts.
- `v1_lists` and the bulk job system verify addresses at low priority.

## Proposed Design

### 1. Request Flow

```text
POST /v1/domain_search
  │
  ├─ resolve domain (RFC 0015 normalization; company → domain as in RFC 0011)
  ├─ domain opted out? ─────────────────────────────────────────► 451
  ├─ sourced in the last 30 days and refresh = false?
  │     └─ yes ─► read store ─► filter ─► sort ─► page ─► 200
  └─ no ─► start or join job "domain_search:{domain}" (single-flight)
             ├─ Stage A + B (RFC 0011 domain sourcing)
             ├─ Stage C: entity search for people at the company
             ├─ Stage D: department searches (only for large companies)
             ├─ entity resolution and address assignment
             ├─ verification of new addresses (bulk job, low priority)
             └─ store write ─► 200 inline if wait_seconds allows, else 202 + search_id
```

### 2. Sources

| Stage | Call | When | Gives |
|---|---|---|---|
| A | Extract known pages (RFC 0011) | Always for a new domain | Published addresses, generic addresses |
| B | Roster task, `base` (RFC 0011) | Always for a new domain | Addresses with names and positions |
| C | Entity search, `entity_type = people`, `match_limit = 100` | Always for a new domain | Names, profile URLs, descriptions |
| D | Entity search for each of 6 departments (executive, sales, marketing, it, finance, operations) | When Stage C returns 100 rows, or the company has more than 500 employees (RFC 0014 cache) | More people at large companies |

Entity search favors recall. We filter its rows:

- We keep a person only when the description or a citation names the company or the domain.
- We drop a person when the description says "former", "ex-", or "previously" next to the company name.

### 3. Entity Resolution

- The person key is `(domain, normalized full name)`, with the RFC 0011 name rules.
- Rows from different stages with the same key merge into one person. The merged person keeps all sources.
- When positions differ, the position with the newest citation wins.
- We link a person to an address when one of these is true:
  - The roster task gives both together.
  - The address matches one of the person's candidates from `generate_candidates`.

### 4. Addresses For People

| Case | Address | `method` |
|---|---|---|
| A published address is linked | That address | `found` |
| No published address, and the domain has a dominant pattern | The pattern address | `pattern` |
| Neither | `null` | — |

People with a `null` address are returned only when `required_field` does not include `email`. The default is `required_field = ["email"]`, as in Hunter.

### 5. Departures

- Each sourcing updates `last_seen_at` for every person it finds.
- A person missing from 2 sourcings in a row that are at least 60 days apart gets `status = possibly_left`.
- `possibly_left` people are hidden by default. `include_departed = true` shows them.
- RFC 0014 uses this status as one input to the job-change signal.

### 6. Seniority And Department

Keyword rules on the position. No LLM. The rules have a version (`taxonomy_version`) and cover English, German, French, Spanish, Portuguese, Dutch, and Italian.

| Field | Values |
|---|---|
| `seniority` | `executive`, `senior`, `junior`, `null` |
| `department` | `executive`, `it`, `finance`, `management`, `sales`, `legal`, `support`, `hr`, `marketing`, `communication`, `education`, `design`, `health`, `operations`, `null` |

Examples:

| Title | Seniority | Department |
|---|---|---|
| "Chief Marketing Officer" | `executive` | `marketing` |
| "Geschäftsführer" | `executive` | `management` |
| "Directeur commercial" | `executive` | `sales` |
| "Senior Software Engineer" | `senior` | `it` |
| "Responsable RH" | `senior` | `hr` |
| "Werkstudent Marketing" | `junior` | `marketing` |

### 7. Generic Addresses

An address is `generic` when its local part is in the role list behind `is_role_account`. Generic rows have `type = "generic"`, no name, and no position.

### 8. Verification

New addresses go into a bulk job at low priority. Each row returns `verification.status`, which is one of:

- `valid`
- `accept_all`
- `unknown`
- `pending`

The row also returns `verification.date`. Addresses that come back `invalid` are removed from the store and never returned.

### 9. Sort And Page

- Rows are sorted by calibrated confidence first, then by seniority (`executive` first).
- `limit` is 1–100, with a default of 25.
- `offset` is at most 10,000.
- `total` counts all matching rows.

### 10. Export

`POST /v1/domain_search/{search_id}/export` adds the selected rows to a new or existing list.

- The rows then go through the normal list verification.
- Export is free, because the rows were already charged when the tenant viewed them.
- Rows already in the target list are skipped.

## Public API / Interface Changes

`POST /v1/domain_search`:

```json
{
  "domain": "acme.com",
  "company": null,
  "type": "personal",
  "seniority": ["executive", "senior"],
  "department": ["marketing"],
  "verification_status": ["valid", "accept_all"],
  "required_field": ["email"],
  "include_departed": false,
  "limit": 25,
  "offset": 0,
  "refresh": false,
  "wait_seconds": 0
}
```

Response (`200`, or `202` with `search_id`; `GET /v1/domain_search/{search_id}` returns the same body when ready):

```json
{
  "search_id": 9012,
  "domain": "acme.com",
  "organization": "Acme Inc.",
  "pattern": "first.last",
  "pattern_share": 0.82,
  "accept_all": false,
  "total": 48,
  "total_is_capped": false,
  "rows": [
    {
      "person_id": 311,
      "email": "jane.doe@acme.com",
      "type": "personal",
      "first_name": "Jane",
      "last_name": "Doe",
      "position": "VP Marketing",
      "seniority": "executive",
      "department": "marketing",
      "status": "current",
      "method": "found",
      "confidence_score": 96,
      "confidence_level": "high",
      "verification": { "status": "valid", "date": "2026-10-12T09:00:00Z" },
      "in_your_lists": true,
      "last_seen_at": "2026-10-12T08:55:00Z",
      "sources": [{ "url": "https://acme.com/team", "title": "Our team", "last_seen_at": "2026-10-10T08:12:00Z" }]
    }
  ],
  "taxonomy_version": "titles_v1",
  "credits_charged": 3,
  "sourced_at": "2026-10-12T08:55:00Z",
  "degraded": false
}
```

Other endpoints:

- `GET /v1/domain_search/count?domain=acme.com`: returns `total`, `personal`, `generic`, `departed`, and counts by department and seniority. It is free and never starts a Parallel call.
- `POST /v1/domain_search/{search_id}/export`: `{ "person_ids": [311, 312], "list_id": null, "new_list_name": "Acme contacts" }`.

`refresh: true` is limited to one refresh for each domain each 24 hours for each tenant.

Scope:

- `domain_search` for search and count.
- `domain_search` and `lists` for export.

Errors (in addition to RFC 0015):

| Status | `error` | When |
|---|---|---|
| `409` | `company_ambiguous` | The company matches more than one domain. |
| `422` | `free_provider_domain` | The domain is a consumer provider. |
| `429` | `refresh_limit` | A second refresh within 24 hours. |

Opted-out addresses are removed from the rows. The call itself does not fail.

Event: `domain_search.completed` (for async searches).

## Data Model / Storage Changes

```sql
CREATE TABLE v1_web_person (
  id               BIGSERIAL PRIMARY KEY,
  domain           TEXT NOT NULL REFERENCES v1_web_domain(domain),
  full_name        TEXT NOT NULL,
  normalized_name  TEXT NOT NULL,
  position         TEXT,
  seniority        TEXT,
  department       TEXT,
  profile_url      TEXT,
  status           TEXT NOT NULL DEFAULT 'current',   -- current | possibly_left
  web_email_id     BIGINT REFERENCES v1_web_email(id) ON DELETE SET NULL,
  missed_sourcings INTEGER NOT NULL DEFAULT 0,
  first_seen_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  last_seen_at     TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  UNIQUE (domain, normalized_name)
);
CREATE INDEX idx_v1_web_person_domain_status ON v1_web_person (domain, status);

ALTER TABLE v1_web_email
  ADD COLUMN seniority  TEXT,
  ADD COLUMN department TEXT;

CREATE TABLE v1_domain_search (
  id               BIGSERIAL PRIMARY KEY,
  tenant_id        UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
  domain           TEXT NOT NULL,
  request          JSONB NOT NULL,
  data_job_id      BIGINT REFERENCES v1_data_job(id) ON DELETE SET NULL,
  status           TEXT NOT NULL,                       -- running | completed | failed
  credits_charged  INTEGER NOT NULL DEFAULT 0,
  created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  completed_at     TIMESTAMPTZ
);
CREATE INDEX idx_v1_domain_search_tenant ON v1_domain_search (tenant_id, created_at DESC);
```

`v1_web_person` is in the store. The RFC 0015 retention rule and the purge job apply to it. Searches are tenant-scoped and are deleted after `result_retention_days`.

## Product Surface (UI)

This follows the RFC 0015 UI rules: results first, and complex controls in sheets.

**Domain Search page:**

1. A compact domain or company input.
2. A domain header: organization, pattern, `accept_all`, and counts (personal, generic, departed).
3. The results table comes first. Its columns are name, position, seniority, department, address, `ConfidenceBadge`, verification, and a sources icon.
4. A "Filters" sheet holds type, seniority, department, verification, and departed people.
5. Row actions:
   - copy the address
   - add the row to a list
   - "Find email" for people without an address (RFC 0011)
6. Bulk select opens an "Export" sheet, with the target list and a count.
7. `CreditCostHint` shows the cost of the page before a new search.

**From a list:** a row's account menu has "Find more contacts at this account". It opens the page with the domain filled in.

## Security And Abuse

- Rate limits for each tenant: 15 requests a second and 500 a minute by default.
- At most 2,000 new-domain sourcings for each tenant each day. Store reads have no daily cap.
- The `offset` cap of 10,000 limits bulk extraction of the store.
- Free-provider domains are refused.
- RFC 0015 validation applies to all Parallel output.

## Observability And SLOs

Metrics:

- the share of searches with at least one personal address
- addresses for each domain
- the store hit rate
- the share of searches that start from a list (buyer drift)
- departure rate
- Parallel cost for each new domain

| SLO | Target |
|---|---|
| p95, fresh store | 1 second or less |
| p95 job time, new domain (Stages A–C) | 60 seconds or less |
| p95 job time, new large domain (Stages A–D) | 180 seconds or less |

## Cost Model

| Stage | Price | Calls for a new domain |
|---|---|---|
| A: Extract, 6 URLs | $0.006 | 1 |
| B: Roster task, `base` | $0.010 | 1 |
| C: Entity search | $0.005 | 1 |
| D: Department searches | $0.030 | 0 or 1 set |

A new domain costs about $0.021. A new large domain costs about $0.051. All tenants share this cost for 30 days.

| Event | Credits |
|---|---|
| Each 5 returned rows with an address (rounded up) | 1 |
| Rows without an address | 0 |
| The count endpoint | 0 |
| Export | 0 |
| The same page again in the same period | 0 |

The ledger `resource_key` is `domain_search:{domain}:{filters_hash}:{offset}`.

## Failure Modes And Edge Cases

- **Entity search returns people from another company with the same name:** the company-match rule drops them.
- **Two people with the same name at one company:** they merge into one person. This is a known limit. The sources show both pages.
- **A company with more than 10,000 people:** we store at most 2,000 people for each domain, and the response shows `total_is_capped: true`.
- **A catch-all domain:** pattern addresses get `verification.status = "accept_all"` and a lower confidence.
- **No people and no addresses:** return `200` with no rows and charge 0. Store `sourcing_status = 'empty'`.
- **Parallel fails during a job:** use what the store already has, set `degraded: true`, and charge only for the rows returned.
- **A person appears again after `possibly_left`:** the status goes back to `current`, and `missed_sourcings` resets to 0.

## Alternatives Considered

- **FindAll instead of entity search.** It checks each match, but it costs $0.25 for each run plus $0.03 for each match: $1.75 for 50 people, against $0.005 for entity search. We filter entity search results ourselves.
- **An LLM for seniority and department.** It costs money for each row and is hard to test. Keyword rules cover the common titles.
- **Hiding names until reveal.** Hunter's multi-domain search hides addresses until a paid reveal. For one domain, a clear price for each 5 addresses is simpler.

## Rollout / Migration / Compatibility

1. Ship after RFC 0011 F2 (domain sourcing) is live, and the store has data.
2. Ship the count endpoint first. It is free and shows how much data we have.
3. Ship search and export to beta tenants after the Legal Review sign-off.
4. Add "Find more contacts at this account" to the list views.

This RFC only adds endpoints and tables.

## Milestones

| # | Deliverable | Size |
|---|---|---|
| D1 | Stage C and D sourcing, filters, entity resolution | M |
| D2 | Title taxonomy in 7 languages, with tests | M |
| D3 | Departure tracking | S |
| D4 | API, credits, export, count | M |
| D5 | Domain Search page and list entry point | M |

## Metrics And Success Criteria

- The share of searches with at least one personal address.
- The share of exported rows that later deliver.
- The share of searches that start from a list.
- The precision of `possibly_left` against later bounces.
- Parallel cost for each new domain.

## Test Strategy

- A title table with at least 500 titles in 7 languages, with the expected seniority and department.
- Unit tests for the company-match filter and the "former" filter.
- Unit tests for entity resolution and person-to-address linking.
- Unit tests for departure state changes.
- Unit tests for credit rounding (1–5 rows = 1 credit; 6 rows = 2 credits; 11 rows = 3 credits).
- A tenant isolation test for searches and `in_your_lists`.

## Definition Of Done

This RFC is done only when all items below are true. "Passes" means it passes in CI on the pull request.

### Build

1. RFC 0015 and RFC 0011 F1–F4 are done.
2. The migration runs up and down. `e2e_migration_upgrade` passes.
3. `backend/openapi.json`, the SDKs, and the web clients show the new endpoints. CI finds no diff after generation.
4. The `domain_search` scope exists, and the web app can grant it to API keys.

### Unit And API Tests

5. All unit tests in Test Strategy pass in the `test` CI job.
6. The API harness has canonical cases for search, get, count, and export. Each case covers success, `403` without the scope, and `404` for another tenant's search.

### End-To-End Tests (Backend)

7. `backend/tests/e2e_domain_search.rs` runs HTTP, Postgres and RabbitMQ (with `testcontainers`), the worker, the data job runner, the Parallel stub, and sandbox SMTP. It passes these scenarios:

| # | Scenario | Expected result |
|---|---|---|
| a | Fresh store | `200`, rows, 0 stub calls |
| b | Empty or stale store | `202`, then `completed`, with rows and verification statuses from the sandbox domains |
| c | A domain that is in no tenant list | Accepted |
| d | Filters, sort, limit, and offset | Correct rows, order, and `total` |
| e | 11 rows with an address | 3 credits; rows without an address cost 0 |
| f | The same page again in the same period | 0 credits, 0 stub calls |
| g | Count endpoint | Correct counts, 0 credits, 0 stub calls |
| h | Two `refresh: true` calls within 24 hours | The second returns `429` |
| i | Free-provider domain | `422` |
| j | Stage C returns 100 rows | Stage D runs; more people stored |
| k | Entity rows with another company, or with "former" | Dropped |
| l | The same person from Stages B and C | One merged person with both sources |
| m | A person missing from 2 sourcings 60 days apart | `possibly_left`; hidden by default; shown with `include_departed` |
| n | One address in the result is opted out | Row removed; call succeeds |
| o | Stub fails during the job | `degraded = true`; charge only for returned rows |
| p | Export to a new list | List created; rows queued for verification; 0 credits |
| q | Two tenants search the same new domain at once | One sourcing job |

### End-To-End Tests (Web)

8. Playwright specs cover these journeys:
   - From a list, choose "Find more contacts at this account", filter by department in the sheet, and export two rows to a list.
   - On the standalone page, enter any domain, see the domain header and the rows, and open the sources.
   - Show departed people.
9. The specs have the `@smoke` tag, and `test:a11y` passes. Each new component has a story and a passing component test.
10. `ci/smoke_full_stack_image.bash` calls the count endpoint on the built image.

### End-To-End Tests (Live, Staging)

11. A live test runs with the real Parallel API when `PARALLEL_LIVE_E2E=1`.
    - It uses 200 business domains: 50 with fewer than 50 employees, 100 with 50–1,000 employees, and 50 with more than 1,000 employees. At least 40 of them are outside English-speaking countries.
    - Reviewers check 300 rows.
    - The pass targets (set 2026-10-16) are:

    | Metric | Target |
    |---|---|
    | Domains with at least one personal address | 90% or more |
    | People who still work at the company | 95% or more |
    | Positions that a reviewer confirms as correct | 95% or more |
    | Seniority and department that a reviewer confirms as correct | 95% or more |
    | Returned addresses with `invalid` status in a later check | 1% or fewer |
    | p95, fresh store | 1 second or less |
    | p95 job time, new domain | 60 seconds or less |
    | Parallel cost for each new domain | $0.03 or less |

    If one target is not met, the feature does not leave beta.
12. **Hunter benchmark.** We run the same 200 domains through the Hunter Domain Search API, after RFC 0015 Legal Review item 13 has a sign-off. We must match or beat Hunter on these two metrics:
    - the median number of valid personal addresses for each domain
    - the share of people who still work at the company
13. **Load.** The RFC 0015 load test holds the SLOs.
14. The reports (our results, Hunter's results, load, and cost) are attached to the release pull request.

### Operations And Release

15. The dashboards show all metrics in this RFC, including the share of searches that start from a list.
16. The RFC 0015 Legal Review covers `v1_web_person` and has a sign-off.
17. The public API docs and the changelog describe the endpoints, export, the credits, and the errors.

## Open Questions

1. Is 2,000 people for each domain the right cap? Owner: product.
2. Should export add rows to an existing list, or always create a new list in v1? Owner: product.

## Unresolved Risks

1. **Buyer drift.** Domain Search accepts any domain, so customers can use it for cold prospecting. We watch the share of searches that start from a list. If this share falls, we review the product surface.
2. **Legal.** People without an address are still personal data. The RFC 0015 Legal Review covers them.
3. **Data quality.** Entity search can return former employees. The filters and the departure rule reduce this. Delivery outcomes show how often it still happens.
