# RFC 0013: Discover — Company And People Search

| | |
|---|---|
| **Status** | Draft; architecture revised 2026-10-20 |
| **Created** | 2026-10-16 |
| **Owners** | To assign: backend lead, web lead |
| **Depends on** | RFC 0015 (Data Platform Foundations), RFC 0011 (finder, list repair), RFC 0012 (Domain Search, counts) |
| **Release gates** | RFC 0015 Legal Review sign-off; sensitive-condition guard live; Definition Of Done met |

## Implementation decision — 2026-10-20

The [company-to-contact integration](../paperless-research-integration.md) starts
with customer-supplied target accounts and roles. Reuse Go research sessions,
source adapters, import handling, evidence, and spending controls. The existing
signed-in Next.js app remains the customer surface, behind Rust tenant authority.

Broad company discovery, advanced business signals, and recurring saved searches
are subsequent phases. The mandatory Parallel implementation and separate Rust
research runner below are superseded; provider use must fit the actual data
rights for the workflow. Do not populate a shared commercial contact database
from customer-private results or a provider whose license prohibits that use.

## Earlier proposal

Add Discover: search for companies and people with a natural-language query ("AI search") or with structured filters. Discover runs on Parallel Web Systems:

| Parallel API | Use |
|---|---|
| FindAll ingest | Turns a natural-language query into match conditions. The user sees and edits them before a run. |
| Entity search | Fast mode. Results in seconds, recall first. |
| FindAll runs | Checked mode. Each match has a status and citations for each condition. |

Each company row shows how many addresses we know for the domain, and whether the domain is already in the tenant's lists. Rows can open Domain Search (RFC 0012) or be exported to a list. A people list then goes through list repair (RFC 0011) to find addresses.

The default entry point for our buyer is **lookalike search**: find companies that are similar to the domains in a list with the best delivery results.

## Motivation / Problem

- We have no way to find companies or people by criteria such as industry, location, or size.
- Hunter's Discover gives this, with an AI assistant and saved searches that flag new matches. Customers who compare tools expect it.
- Parallel gives entity discovery with citations as a service. We do not need our own company database.

## Buyer Fit

Discover has the **weakest fit** with our buyer of the data RFCs. Our buyer already has a list. We frame Discover for this buyer in three ways:

1. **Lookalike accounts.** Start from the domains in a list with the best delivery results (RFC 0001 outcomes). Find similar companies.
2. **Overlap first.** Each row shows `in_your_lists`, so the user sees what they already have.
3. **Back into the list flow.** Discover ends at "export to list" or "open Domain Search". Then list repair and verification run as usual.

We do not add sequences or sending.

## How We Beat Hunter

| Area | Hunter.io | This RFC |
|---|---|---|
| AI search | An AI assistant fills filters | The parsed conditions are shown and can be edited before any run |
| Proof | Company rows | Checked mode gives a match status and citations for each condition |
| Starting point | A blank search | Lookalikes from the tenant's own delivery results |
| Overlap | Filter out saved companies | `in_your_lists` on each row, from the tenant's lists |
| Next step | Save leads, start a sequence | Export to a list, then list repair and verification |
| Safety | Not documented | Searches for people by sensitive traits are refused |

## Goals

- Search companies and people with a natural-language query or with filters.
- Show the parsed conditions, and let the user edit them before a run.
- Give a fast mode (seconds) and a checked mode (minutes, with citations).
- Give lookalike search from a list.
- Show `emails_count` and `in_your_lists` on each company row.
- Save searches, refresh them on a schedule, and flag new rows.
- Export rows to a list.
- Refuse searches for people by special category traits.
- Limit cost with the AI search cap and with data credits for checked runs.

## Non-Goals

- Our own company database or crawler.
- Filters for funding, technology, or web traffic in v1. Conditions can express them, but we do not build filter controls for them yet.
- Address reveal inside Discover. Addresses come from RFC 0011 and RFC 0012, with their own credits.
- Sending, sequences, or CRM sync.

## Current State In Reacher

- No company or people search exists.
- RFC 0015 adds the Parallel client, the job runner, credits, the AI search counter, and the store.
- RFC 0012 adds Domain Search and the count query that gives `emails_count`.
- RFC 0011 adds list repair, which finds addresses for rows with a name and a domain.
- `v1_contact_outcomes` gives the delivered domains that lookalike search uses as seeds.

## Proposed Design

### 1. Modes

| Mode | Parallel call | Latency | Quality | Billing |
|---|---|---|---|---|
| `fast` | Entity search, up to 100 rows | Seconds | Recall first. Some rows do not match. | 1 AI search |
| `checked` | FindAll run, generator `base` | Minutes, async | Each match is checked, with citations | 1 AI search + 25 credits + 3 credits for each match |

### 2. AI Search

1. The user sends a natural-language query of at most 500 characters.
2. We call FindAll ingest. It returns `objective`, `entity_type`, and `match_conditions`.
3. The sensitive-condition guard (section 4) checks the conditions.
4. We return the conditions as a draft. The user can edit, add, or remove conditions.
5. The user starts a `fast` search or a `checked` run with the final conditions.

We store the final conditions with each search, so each result can be explained and run again.

### 3. Structured Filters

Each filter becomes one match condition:

| Filter | Example | Condition text |
|---|---|---|
| `industry` | `["software"]` | "The company is in the software industry." |
| `location` | `{"country": "DE"}` | "The company has its headquarters in Germany." |
| `headcount` | `{"min": 50, "max": 200}` | "The company has between 50 and 200 employees." |
| `keywords` | `["HR software"]` | "The company describes itself with: HR software." |
| `title` (people) | `["VP Marketing"]` | "The person's current job title is similar to: VP Marketing." |
| `seniority` (people) | `["executive"]` | "The person has an executive role." |

Filters and a natural-language query can be used together.

### 4. Sensitive-Condition Guard

People searches must not select people by special category traits:

- health
- religion
- ethnicity or race
- sexual orientation
- political opinion
- union membership
- biometrics

The guard runs on every condition, both before a run and after an edit:

1. A term list in 7 languages, with a version (`sensitive_terms_v1`).
2. A condition that matches returns `422` with `sensitive_condition`, and names the condition.
3. Company searches allow industry terms that look similar (for example, "health care software"). The guard blocks them only for `entity_type = people`.

The guard has its own test table and its own Legal Review item (RFC 0015 item 10).

### 5. Lookalike Search

`similar_to_list_id`:

1. Take the 20 domains in the list with the most `delivered` outcomes in the last 12 months. At least 5 such domains are required; otherwise return `422` with `not_enough_seeds`.
2. Build the objective: "Companies similar to: {domains}. Similar means the same industry, a similar size, and a similar customer type."
3. Run a `fast` search, or a `checked` run if the caller asks.
4. Domains that are already in the tenant's lists are hidden by default (`exclude_existing = true`).

### 6. Row Post-Processing

For each company row:

1. Take the domain from the entity URL, with the RFC 0015 normalization (the Public Suffix List).
2. Drop rows on directory and social domains (for example LinkedIn, Crunchbase, Wikipedia, Glassdoor, Indeed). The blocklist lives in config.
3. Drop free-provider domains.
4. Keep one row for each domain.
5. Add `emails_count` from the RFC 0012 count query.
6. Add `in_your_lists` from the caller's lists.

For each people row: parse the title from the description, and take the company domain the same way. Rows have no address.

### 7. Result Cache

- A `fast` search with the same normalized `entity_type`, objective, and conditions within 24 hours reuses the cached rows.
- A cache hit is free for the tenant and costs us nothing. It does not count as an AI search.
- The cache is global and holds only public rows. It does not record which tenant asked.

### 8. Saved Searches

- A user can save a search (conditions, mode, and refresh interval). The default interval is weekly; the minimum is daily.
- The RFC 0015 runner runs refreshes as `discover_refresh` jobs.
- We compare domains (companies) or profile URLs (people) with the last run. New rows get `is_new: true`.
- New rows send the event `discover.view.new_matches`, and the page shows a badge.
- A refresh counts as an AI search. It is skipped when the tenant has no AI searches left.

### 9. Export

`POST /v1/discover/searches/{search_id}/export` adds rows to a new or existing list:

| Entity type | List columns | What happens next |
|---|---|---|
| Companies | `domain`, `organization` | Domain Search (RFC 0012) from the list |
| People | `full_name`, `domain`, `organization` | List repair (RFC 0011) to find addresses |

Export is free.

## Public API / Interface Changes

`POST /v1/discover/parse`:

```json
{ "query": "B2B SaaS companies in Germany with 50-200 employees that sell to HR teams" }
```

Response:

```json
{
  "entity_type": "companies",
  "objective": "Find B2B SaaS companies in Germany that sell to HR teams",
  "match_conditions": [
    { "name": "b2b_saas", "description": "The company sells software as a service to businesses." },
    { "name": "germany_hq", "description": "The company has its headquarters in Germany." },
    { "name": "headcount", "description": "The company has between 50 and 200 employees." },
    { "name": "hr_buyers", "description": "The company sells to HR teams." }
  ],
  "guard_version": "sensitive_terms_v1"
}
```

`POST /v1/discover/companies` and `POST /v1/discover/people`:

```json
{
  "objective": "Find B2B SaaS companies in Germany that sell to HR teams",
  "match_conditions": [ "..." ],
  "filters": { "location": { "country": "DE" } },
  "similar_to_list_id": null,
  "exclude_existing": false,
  "mode": "fast",
  "limit": 50
}
```

- `mode = fast` returns `200` with rows.
- `mode = checked` returns `202` with `search_id`. `GET /v1/discover/searches/{search_id}` returns progress and rows.

Company row:

```json
{
  "row_id": 551,
  "name": "Acme HR GmbH",
  "domain": "acmehr.de",
  "url": "https://acmehr.de",
  "description": "HR software for mid-size companies.",
  "emails_count": 12,
  "in_your_lists": false,
  "is_new": false,
  "match": {
    "status": "matched",
    "conditions": { "germany_hq": true, "headcount": true },
    "citations": [{ "url": "https://acmehr.de/about", "title": "About us" }]
  }
}
```

`match` is null in `fast` mode.

Other endpoints:

- `POST /v1/discover/searches/{search_id}/export` with `{ "row_ids": [551], "list_id": null, "new_list_name": "German HR SaaS" }`
- `GET /v1/discover/views`, `POST /v1/discover/views`, `GET /v1/discover/views/{view_id}`, `PATCH /v1/discover/views/{view_id}`, `DELETE /v1/discover/views/{view_id}`

Scope:

- `discover` for searches and views.
- `discover` and `lists` for export and lookalike search.

Errors (in addition to RFC 0015):

| Status | `error` | When |
|---|---|---|
| `422` | `sensitive_condition` | A people condition touches a special category trait. |
| `422` | `not_enough_seeds` | A lookalike list has fewer than 5 delivered domains. |
| `422` | `query_too_long` | The query is longer than 500 characters. |

Events:

- `discover.search.completed` (checked mode)
- `discover.view.new_matches`

## Data Model / Storage Changes

```sql
CREATE TABLE v1_discover_search (
  id               BIGSERIAL PRIMARY KEY,
  tenant_id        UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
  view_id          BIGINT,
  data_job_id      BIGINT REFERENCES v1_data_job(id) ON DELETE SET NULL,
  entity_type      TEXT NOT NULL,          -- companies | people
  mode             TEXT NOT NULL,          -- fast | checked
  query            TEXT,
  objective        TEXT NOT NULL,
  match_conditions JSONB NOT NULL,
  guard_version    TEXT NOT NULL,
  similar_to_list_id INTEGER,
  status           TEXT NOT NULL,          -- running | completed | failed | cancelled
  matches          INTEGER NOT NULL DEFAULT 0,
  credits_charged  INTEGER NOT NULL DEFAULT 0,
  cache_hit        BOOLEAN NOT NULL DEFAULT false,
  created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  completed_at     TIMESTAMPTZ
);
CREATE INDEX idx_v1_discover_search_tenant ON v1_discover_search (tenant_id, created_at DESC);

CREATE TABLE v1_discover_row (
  id               BIGSERIAL PRIMARY KEY,
  search_id        BIGINT NOT NULL REFERENCES v1_discover_search(id) ON DELETE CASCADE,
  name             TEXT NOT NULL,
  url              TEXT NOT NULL,
  domain           TEXT,
  title            TEXT,
  description      TEXT,
  match            JSONB,
  is_new           BOOLEAN NOT NULL DEFAULT false
);
CREATE INDEX idx_v1_discover_row_search ON v1_discover_row (search_id);

CREATE TABLE v1_discover_view (
  id               BIGSERIAL PRIMARY KEY,
  tenant_id        UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
  name             TEXT NOT NULL,
  entity_type      TEXT NOT NULL,
  objective        TEXT NOT NULL,
  match_conditions JSONB NOT NULL,
  mode             TEXT NOT NULL,
  refresh_interval INTERVAL NOT NULL DEFAULT '7 days',
  next_run_at      TIMESTAMPTZ,
  last_run_at      TIMESTAMPTZ,
  last_search_id   BIGINT,
  created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE v1_discover_cache (
  query_hash       BYTEA PRIMARY KEY,      -- hash of entity_type + objective + conditions
  rows             JSONB NOT NULL,
  created_at       TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

Retention:

- Search rows are deleted after 90 days, except the rows of the last run of each saved search.
- Cache rows are deleted after 24 hours.

## Product Surface (UI)

This follows the RFC 0015 UI rules: results first, and complex controls in sheets.

**Discover page:**

1. A query bar at the top, with a switch for companies or people.
2. A row of condition chips sits above the results. An "Edit conditions" sheet lets the user edit, add, and remove conditions.
3. The results table comes first. Company columns: name, domain, description, `emails_count`, and `in_your_lists`. People columns: name, title, company.
4. A "Filters" sheet holds industry, location, headcount, keywords, title, and seniority.
5. A mode switch: "Fast" or "Checked". "Checked" shows `CreditCostHint` with the maximum credits.
6. Row actions: open Domain Search, find an email (people), and add to a list.
7. Bulk select opens an "Export" sheet.
8. A saved searches panel shows a "new" badge for each saved search with new rows.

**From a list:** a "Find similar companies" action starts a lookalike search.

## Security And Abuse

- Rate limits for each tenant: 5 requests a second and 50 a minute, as Hunter does.
- The AI search cap limits spend. Checked runs also need data credits.
- The sensitive-condition guard blocks searches for people by special category traits.
- The query length limit (500 characters) and the condition limit (10 conditions) bound the cost of each call.
- Search rows are tenant-scoped. The global cache stores no tenant information.
- RFC 0015 validation applies to all Parallel output.

## Observability And SLOs

Metrics:

- the share of sessions that end in export or Domain Search
- the share of lookalike searches
- the share of `parse` drafts that users accept without edits
- the cache hit rate
- the guard block rate
- Parallel cost for each session

| SLO | Target |
|---|---|
| p95, `parse` | 3 seconds or less |
| p95, `fast` (cache miss) | 5 seconds or less |
| p95, `fast` (cache hit) | 500 milliseconds or less |
| p95, `checked` run with 50 matches | 10 minutes or less |

## Cost Model

Parallel list prices (October 2026):

| Call | Price |
|---|---|
| FindAll ingest | Not listed; to confirm with Parallel (RFC 0015 Open Question 2) |
| Entity search | $0.005 for each request (100 results) |
| FindAll `base` | $0.25 for each run + $0.03 for each match |

Our billing:

| Event | Cost to tenant |
|---|---|
| `parse` | 1 AI search |
| `fast` search, cache miss | 1 AI search |
| `fast` search, cache hit | Free |
| `checked` run | 1 AI search + 25 credits for the run + 3 credits for each match |
| Saved-search refresh | The same as a manual search |
| Export | Free |

A `checked` run with 50 matches costs us $1.75 and costs the tenant 175 credits. Each credit then costs us $0.010, which is the RFC 0015 planning cost for an 85% gross margin.

## Failure Modes And Edge Cases

- **Ingest returns conditions that do not fit the query:** the user edits them. We never start a `checked` run without showing the conditions first.
- **Entity search returns rows that do not match:** `fast` mode is labeled "Fast, not checked".
- **A FindAll run takes too long:** we poll every 15 seconds for up to 30 minutes, then cancel the run and mark the search `failed`. We charge only for the matches we received.
- **The tenant runs out of credits during a checked run:** we cancel the run and return the matches we already have.
- **An entity URL has no clear domain** (a directory page or a social profile): the blocklist drops it, or `domain` is null and the row cannot open Domain Search.
- **A lookalike list has fewer than 5 delivered domains:** return `422` `not_enough_seeds`.
- **A saved search refresh fails:** keep the last good rows, and try again at the next interval.

## Alternatives Considered

- **Our own company database.** It needs a crawler, entity resolution, and constant updates. It is a separate business.
- **FindAll for every search.** Each run costs at least $0.25. Entity search costs $0.005 and answers in seconds. FindAll runs only when the user asks for checked results.
- **Parallel Monitor for saved searches.** Monitor watches for web events. A scheduled run again with a diff is simpler and uses the RFC 0015 runner.
- **An LLM classifier for the sensitive-condition guard.** It is harder to test and can change without notice. A term list with a version is easy to review. We can add a classifier later as a second layer.

## Rollout / Migration / Compatibility

1. Ship after RFC 0015, RFC 0011, and RFC 0012.
2. Ship `parse`, the guard, and `fast` mode to the internal tenant. Review samples.
3. Ship lookalike search and export, because they are why this feature fits our buyer.
4. Ship `checked` mode and saved searches to beta tenants after the Legal Review sign-off.

This RFC only adds endpoints and tables.

## Milestones

| # | Deliverable | Size |
|---|---|---|
| S1 | Parse, filters to conditions, sensitive-condition guard | M |
| S2 | Fast mode, row post-processing, cache | M |
| S3 | Lookalike search and export | M |
| S4 | Checked mode with credits | M |
| S5 | Saved searches, refresh, events | M |
| S6 | Discover page and list entry point | L |

## Metrics And Success Criteria

- The share of Discover sessions that end in export or Domain Search.
- The share of lookalike searches among all searches (buyer fit).
- The share of exported companies that later get delivered mail.
- Parallel cost for each session, against the AI searches and credits charged.

## Test Strategy

- Unit tests that turn each filter into its condition text.
- A guard table with at least 300 phrases in 7 languages, both blocked and allowed (for example, "health care software company" is allowed for companies).
- Unit tests for row post-processing: domain extraction, the blocklist, and dedupe.
- Unit tests for the cache key and the saved-search diff.
- Unit tests for checked-run credits (25 for the run + 3 for each match).
- A tenant isolation test for searches, views, `similar_to_list_id`, and `in_your_lists`.

## Definition Of Done

This RFC is done only when all items below are true. "Passes" means it passes in CI on the pull request.

### Build

1. RFC 0015, RFC 0011 (F1–F6), and RFC 0012 (D1–D4) are done.
2. The migration runs up and down. `e2e_migration_upgrade` passes.
3. `backend/openapi.json`, the SDKs, and the web clients show the new endpoints. CI finds no diff after generation.
4. The `discover` scope exists. The AI search cap is set for each plan.
5. The Parallel stub supports ingest, entity search, and FindAll runs with progress, completion, and timeout.

### Unit And API Tests

6. All unit tests in Test Strategy pass in the `test` CI job.
7. The API harness has a canonical case for each new endpoint. Each case covers success, `403` without the scope, and `404` for another tenant's search or view.

### End-To-End Tests (Backend)

8. `backend/tests/e2e_discover.rs` runs HTTP, Postgres and RabbitMQ (with `testcontainers`), the data job runner, and the Parallel stub. It passes these scenarios:

| # | Scenario | Expected result |
|---|---|---|
| a | `parse` a query | Conditions returned; 1 AI search counted |
| b | `parse` a people query about religion or health | `422` `sensitive_condition`; 0 stub calls after ingest |
| c | `fast` company search | `200`; correct domain, `emails_count`, and `in_your_lists`; directory domains dropped; one row for each domain |
| d | Tenant B runs the same search | `in_your_lists` uses tenant B's lists only; cache hit; 0 AI searches for B |
| e | `checked` run that completes | `202`, then `completed`; rows with `match` and citations; credits = 25 + 3 × matches |
| f | `checked` run that times out | `failed`; credits = 25 + 3 × matches received |
| g | Credits run out during a `checked` run | Run cancelled; partial rows returned |
| h | AI search cap reached | `429`; 0 stub calls |
| i | Lookalike with seeded delivered outcomes | The objective uses the top delivered domains; existing domains hidden |
| j | Lookalike list with 4 delivered domains | `422` `not_enough_seeds` |
| k | `similar_to_list_id` of another tenant | `404` |
| l | People search and export | List created with name and domain rows; list repair can run on it |
| m | Saved search and a scheduled refresh | New rows have `is_new = true`; event `discover.view.new_matches` sent |
| n | Rows older than 90 days | Deleted, except the last run of each saved search |
| o | `parallel.enabled = false` | `501` |

### End-To-End Tests (Web)

9. Playwright specs cover these journeys:
   - Type a query, edit the conditions in the sheet, run a `fast` search, and open Domain Search from a row.
   - From a list, start a lookalike search and export two companies to a list.
   - Run a people search, export it, and start list repair.
   - Save a search, and see the "new" badge after a refresh.
   - See the `sensitive_condition` message.
10. The specs have the `@smoke` tag, and `test:a11y` passes. Each new component has a story and a passing component test.
11. `ci/smoke_full_stack_image.bash` calls `parse` on the built image and checks the `501` response when Parallel is disabled.

### End-To-End Tests (Live, Staging)

12. A live test runs with the real Parallel API when `PARALLEL_LIVE_E2E=1`.
    - It runs 50 fixed queries (30 for companies, 20 for people) and 10 lookalike searches.
    - Reviewers check 20 rows from each search.
    - The pass targets (set 2026-10-16) are:

    | Metric | Target |
    |---|---|
    | `fast` rows that match the query | 85% or more |
    | `checked` rows that match the query | 98% or more |
    | `parse` conditions that the reviewer accepts with no edit | 90% or more |
    | Lookalike rows that a reviewer rates "similar" to the seed domains | 85% or more |
    | Sensitive people queries blocked (from a set of 100) | 100% |
    | Allowed company queries wrongly blocked (from a set of 100) | 2% or fewer |
    | p95, `parse` | 3 seconds or less |
    | p95, `fast` (cache miss) | 5 seconds or less |
    | Parallel cost for each session | 10% or less of the list price of the AI searches and credits that it uses |

    If one target is not met, the feature does not leave beta.
13. **Hunter benchmark.** We run the same 30 company queries in Hunter Discover, after RFC 0015 Legal Review item 13 has a sign-off. Reviewers check 20 rows from each tool, and they do not know which tool gave each row. We must match or beat Hunter on the share of rows that match the query.
14. The FindAll ingest price is confirmed with Parallel and written in the Cost Model.
15. **Load.** The RFC 0015 load test holds the SLOs.
16. The reports (our results, Hunter's results, load, and cost) are attached to the release pull request.

### Operations And Release

17. The dashboards show all metrics in this RFC, including the share of lookalike searches.
18. The RFC 0015 Legal Review has a sign-off. People search and the guard are in its scope.
19. The public API docs and the changelog describe the endpoints, the AI search cap, the guard, and the credits.

## Open Questions

1. What does FindAll ingest cost? Owner: backend lead.
2. Who maintains the directory blocklist and the sensitive term list? Owner: product.

## Unresolved Risks

1. **Buyer drift.** Discover is a prospecting feature. If it becomes the main entry point of the product, our message to senders gets weaker. Lookalike search and `in_your_lists` stay the default view.
2. **Beta API.** FindAll and entity search are in beta. The RFC 0015 weekly contract test finds changes early.
3. **Result quality.** Fast mode can return rows that do not match. The UI labels it, and checked mode is one click away.
4. **Guard gaps.** A term list cannot catch every way to describe a sensitive trait. Reviewers sample blocked and allowed queries each month.
