# RFC 0009: Email Finder and Selective List Repair

## Verify disposition — 2026-10-23

**Partial; selective repair in stage 2.** Existing pattern finder and the connected published-source research pilot are foundations, not proof of Hunter parity or calibrated identity confidence. Build only the narrow repair improvements justified by RFC 0002, under RFC 0001 source rights and RFC 0005 spend/execution controls. Preserve the current finder API and conservative catch-all handling.

[Current build order](README.md) · [Codebase audit](../audits/2026-10-23-verify-north-star.md). This disposition supersedes conflicting historical scope, priority, or baseline statements below; it does not certify every original acceptance criterion.

| | |
|---|---|
| **Status** | Partial; selective repair in stage 2; audited 2026-10-23 |
| **Created** | 2026-10-16 |
| **Owners** | To assign: backend lead, web lead |
| **Depends on** | RFC 0001 (rights), RFC 0005 (repair budgets/execution) |
| **Used by** | RFC 0018 (domain sourcing), RFC 0022 ("Find email" on people rows) |
| **Release gates** | Current scoped RFC acceptance; RFC 0001 rights; RFC 0002 measured value. Earlier gates below are historical. |

## Implementation decision — 2026-10-20

The [company-to-contact integration](../paperless-research-integration.md) is the
current implementation plan. Go owns company/person research and source evidence;
Rust owns finder orchestration, verification, tenant quotas, suppression, and
approved export. In Paperless's managed staging and production deployments,
candidate checks use MillionVerifier through the existing Rust API, with stable
retry keys and bounded attempts. SMTP/proxy fallback is disabled in those
deployments; the generic self-hosted Reacher defaults remain separate.
Catch-all results remain review items and never establish mailbox ownership.

The SMTP pipeline and new Rust sourcing services described below are historical
proposals, not instructions for this release. Calibrated confidence and superiority
claims require benchmark evidence; current results expose separate person,
employer, role, and mailbox claims instead. Preserve the existing finder contract.

## Earlier proposal

Make the email finder as good as the Hunter.io Email Finder, and better on catch-all domains. The new finder uses evidence in this order:

1. Addresses that are published on the public web, with source URLs.
2. The email pattern of the domain, learned from web evidence, our own verification results, and the tenant's own delivery outcomes.
3. At most three SMTP checks.

Each result has a **calibrated** confidence score: a score of 90 means that about 90% of addresses with that score are correct. Each result also shows the method, the sources, and the next-best alternatives.

This RFC also adds **list repair**: run the finder over the rows of a list that have a name and a company but a missing or invalid address. The results go into the existing remediation plan as `fix_then_send` rows.

## Motivation / Problem

Facts from the current code:

- `finder/patterns.rs::normalize_name` keeps only ASCII letters and digits after Unicode folding. A name in a non-Latin script (for example, "王小明") becomes an empty string, and the request fails with `400`.
- The same function removes hyphens and spaces. "Jean-Luc" becomes only "jeanluc", and "van der Berg" becomes only "vanderberg".
- `normalize_domain` only trims and lowercases. An input such as `https://www.acme.com/` does not work.
- The default `parallel` strategy sends one SMTP probe for each of the 13 patterns. This load can cause greylisting and can damage the reputation of our probe IPs. `ThrottleManager` limits the whole worker, not each domain.
- On a catch-all domain, every candidate passes SMTP. The ranking then uses only pattern popularity.
- The response has no source, no method, and a confidence score that is not calibrated.

Hunter's finder uses a web index and the domain pattern. Its help center calls its score "only an indication, not a guarantee" and suggests that users filter out scores below 85.

## Buyer Fit

Our buyer already has a list and sends email to it. The finder serves this buyer mainly through **list repair**:

1. **Fill missing addresses.** A CRM export or an event list has a name and a company, but no address.
2. **Fix bad addresses.** A row is invalid, but the name and the domain are known.
3. **Follow domain changes.** A company moves from `acme.com` to `acme.io`.

Single lookups stay available on the finder page and in the API.

## How We Beat Hunter

| Area | Hunter.io | This RFC |
|---|---|---|
| Meaning of the score | "An indication, not a guarantee" | Calibrated: each 10-point score bucket is within ±3 points of the measured precision |
| Catch-all domains | Score from web sources and the pattern | The same, plus the tenant's own delivery outcomes for that domain |
| Explanation | Source list | Sources, method, factors, and alternatives |
| List workflow | Bulk finder from a CSV | List repair inside the remediation plan, with before and after values and export |
| SMTP load | Not disclosed | At most 3 checks for each lookup, and 1.5 or fewer on average |

## Goals

- Return a found address with its source URLs when it is published on the web.
- Learn the pattern of each domain, and use it before any SMTP check.
- Use the tenant's own delivery outcomes to resolve catch-all domains for that tenant.
- Give a calibrated 0–100 confidence score, a level, a method, factors, sources, and up to 3 alternatives.
- Support names in all scripts, hyphenated names, and surnames with particles.
- Accept a domain in any common form, or a company name.
- Repair a full list, with a cost estimate and an approval step.
- Keep the current request and response fields compatible.

## Non-Goals

- Lookup from a LinkedIn URL or handle.
- Phone numbers and social profiles.
- A browser extension.
- Nickname matching (for example, "Bob" for "Robert"). We can add it when the evaluation set shows a need.

<a id="current-state-in-reacher"></a>

## Original baseline (historical)

- `POST /v1/find_email` takes `first_name`, `last_name`, `domain`, and `strategy`. It creates a `v1_finder_job`, a `v1_bulk_job`, and one task for each candidate in the `check_email` queue. In `waterfall` mode the task priority comes from `pattern_priority`.
- `GET /v1/find_email/{job_id}` calls `sync_finder_results` and returns the candidates, a best match, and a `ConfidenceExplanation`.
- `explain_confidence` adds a pattern quality score, a `-20` for no MX, a `-15` for catch-all, and a `±10` from the email score.
- `precheck_domain` probes a random address to detect catch-all. It skips the probe for known consumer providers.
- Remediation rows (`v1_remediation_rows`) already have `classification` (for example `fix_then_send`, `review`), `rule_id`, `original_email`, `effective_email`, `before`, `after`, and `reasons`.
- `v1_contact_outcomes` stores each tenant's delivery events, with `canonical_email`, `event_type`, and `occurred_at`.

## Proposed Design

### 1. Input Resolution

**Names.**

- The caller sends `first_name` and `last_name`, or `full_name`.
- `full_name` parsing:
  - "Doe, Jane" → first "Jane", last "Doe".
  - We remove titles (Dr, Mr, Mrs, Ms, Prof) and suffixes (Jr, Sr, II, III, PhD).
  - Particles (van, von, de, da, del, della, der, di, du, la, le, st, ten, ter, bin, al) join the surname.
  - When the parse is not clear, we return `422` with `name_ambiguous` and ask for `first_name` and `last_name`.
- Each name part gives a small set of variants, with at most 4 for each part:

| Input | Variants |
|---|---|
| "José" | `jose` |
| "王小明" (with a transliteration) | `wangxiaoming`, `wang`, `xiaoming` |
| "Jean-Luc" | `jeanluc`, `jean-luc`, `jean` |
| "van der Berg" | `vanderberg`, `berg` |
| "García López" | `garcialopez`, `garcia` |

- Transliteration uses a new crate (for example `deunicode`). The original names stay in the job for display.

**Domain.** We use the RFC 0032 normalization. A free-provider domain returns `422` with `free_provider_domain`.

**Company to domain.**

1. Look for an exact, case-insensitive match on `v1_web_domain.organization`.
2. Otherwise run a Task `lite` with the output `{domain, organization}`.
3. Keep the domain only when it has MX records.
4. More than one good match returns `409` with the candidates.

### 2. Evidence Ladder

```text
resolved input (name variants + domain)
  │
  ├─ 0. opt-out check (address after step 2 or 4; domain now) ──────────► 451
  ├─ 1. domain sourced in the last 30 days? ── no ──► source the domain (section 3)
  ├─ 2. store has an address for this person? ── yes ──► re-verify if older than 30 days
  │                                                     └──► method = "found"
  ├─ 3. person task (Task "lite"), only when the pattern posterior is below 0.6
  │        └─ address with a citation ──────────────────────────► method = "found"
  ├─ 4. rank candidates by pattern posterior (section 4) and catch-all evidence (section 5)
  │        ├─ domain is not catch-all: SMTP-check candidates in rank order, at most 3
  │        │      ├─ deliverable ──────────────────────────────► method = "pattern" or "probe"
  │        │      └─ none deliverable ─────────────────────────► no address, 0 credits
  │        └─ domain is catch-all: no SMTP check of candidates
  │               └─ top candidate ──► accept_all = true ──────► method = "pattern" or "guess"
  └─ 5. calibrate the score (section 6); below min_confidence ──► alternatives only, 0 credits
```

`method` values:

| Method | Meaning |
|---|---|
| `found` | The address is published on a cited page. |
| `pattern` | The domain pattern posterior is 0.6 or more, and the evidence weight is 3 or more. |
| `probe` | SMTP confirmed a candidate, but the pattern evidence is weak. |
| `guess` | A catch-all domain with weak pattern evidence. It never sets `safe_to_send`. |

### 3. Domain Sourcing

A sourcing job has the dedupe key `source_domain:{domain}`. RFC 0032 single-flight makes sure that each domain is sourced once, even when many tenants ask at the same time.

**Stage A: known pages (cheap).** Extract these paths from the domain: `/`, `/contact`, `/about`, `/team`, `/impressum`, `/imprint`. The cost is $0.001 for each URL. We find addresses on `@{domain}` with a strict pattern. Generic addresses (role accounts, from the existing role list) are stored as `generic`.

**Stage B: roster task.** Task `base` with this output schema:

```json
{
  "type": "object",
  "properties": {
    "emails": {
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "email": { "type": "string" },
          "first_name": { "type": "string" },
          "last_name": { "type": "string" },
          "position": { "type": "string" }
        },
        "required": ["email"]
      }
    }
  },
  "required": ["emails"]
}
```

Both stages use the RFC 0032 validation rules. In particular, an address is kept only when a citation excerpt contains it.

**Freshness.**

| Result | Next sourcing |
|---|---|
| `ok` | After 30 days |
| `empty` | After 30 days |
| `failed` | After 24 hours |

### 4. Pattern Model

The pattern set is the current set in `finder/patterns.rs`. We add a new pattern only when it appears on at least 1% of the domains in the evaluation set.

For each domain we compute a posterior share for each pattern with a Dirichlet-multinomial model:

```text
P(p) = (α_p + W_p) / (Σα + ΣW)

α_p  = 2 × the global share of pattern p (computed each night over all domains with a dominant pattern)
W_p  = Σ (source weight × recency factor) over the evidence for p
recency factor = 0.5 ^ (age in months / 24)
```

| Evidence source | Weight | Where it is stored |
|---|---|---|
| Web address with a matching name | 1.0 | Global (`v1_domain_pattern_evidence`) |
| Deliverable SMTP check on a domain that is not catch-all | 1.0 | Global (domain and pattern only, not the address) |
| A `delivered` outcome of this tenant | 0.7 | Not stored; computed at query time for this tenant only |

A domain has a **dominant pattern** when `P(p) ≥ 0.6` and `ΣW ≥ 3`. Recency weighting handles domains that change pattern over time. The result shows the dominant pattern and its share.

### 5. Catch-All Resolution

SMTP gives no signal on a catch-all domain. We rank candidates by the posterior from section 4, with two extra tenant-private signals:

1. **Delivered shapes.** We read this tenant's `delivered` outcomes for the domain from the last 24 months. We classify the shape of each local part with a given-name dictionary (`finder/names.rs`). For example, `jane.doe` has a known first name before the dot, so it supports `first.last`. Each match adds evidence with weight 0.7.
2. **Known bounces.** A candidate that hard-bounced for this tenant is never returned.

Tenant-private evidence never leaves the tenant. It is not written to global tables.

To make this query fast, we add a stored domain column and an index to `v1_contact_outcomes`:

```sql
ALTER TABLE v1_contact_outcomes
  ADD COLUMN domain TEXT GENERATED ALWAYS AS (split_part(canonical_email, '@', 2)) STORED;
CREATE INDEX idx_v1_contact_outcomes_tenant_domain
  ON v1_contact_outcomes (tenant_id, domain, event_type, occurred_at DESC);
```

A stored generated column rewrites the table. If the table is large in production, we use a plain column, a batched backfill, a trigger for new rows, and `CREATE INDEX CONCURRENTLY` instead.

### 6. Confidence: Raw Score And Calibration

**Raw score.** A deterministic function of these factors:

- the method
- the number of independent sources and their age
- the pattern posterior and the evidence weight
- the SMTP result
- catch-all status
- name ambiguity

Each factor that changes the score appears in `factors`, for example `"pattern_posterior: 0.82 (+24)"`.

**Calibration.** A monotonic, piecewise-linear table maps the raw score to a calibrated score.

- The eval harness fits the table on the evaluation set (`data_eval --fit-calibration finder`).
- The table is checked in as `finder/calibration.rs`, with the version `finder_v2.N`.
- Rule: for each 10-point bucket with at least 50 samples, the measured precision is within ±3 points of the mean score in that bucket.

**Levels.**

| Level | Calibrated score |
|---|---|
| `high` | 90–100 |
| `medium` | 70–89 |
| `low` | 50–69 |
| `very_low` | 0–49 |

A result below `min_confidence` (default 50) is not returned as `best_match`. It appears only in `alternatives`, and it costs 0 credits.

### 7. SMTP Budget And Domain Limits

- At most 3 SMTP checks for each lookup, in rank order.
- Store hits are verified again only when the last check is older than 30 days.
- A new per-domain limit: at most 30 finder SMTP checks for each domain each minute, across all tenants. Extra checks wait in the queue.
- A new per-tenant fan-out limit: at most 1,000 lookups on one domain each day.

### 8. Ambiguity

Two people with the same name at one domain give `ambiguous: true`. The best-supported address is `best_match`, and the other is in `alternatives`. The lookup costs 1 credit.

### 9. List Repair

`POST /v1/lists/{list_id}/repair` runs the finder over list rows.

1. **Select rows.** `scope` decides the rows: `missing` (no address), `invalid` (the last result was invalid), or `both`. A row needs a name and a domain or company in the mapped columns.
2. **Estimate.** The maximum cost is 1 credit for each selected row. Repeats in the period are free. `dry_run: true` returns only the estimate.
3. **Approve.** An estimate above the tenant threshold uses the RFC 0032 approval step.
4. **Run.** A `list_repair` data job runs the evidence ladder for each row, with a concurrency limit. Domain sourcing is shared through single-flight.
5. **Write results** to a remediation plan:

| Result | `classification` | `rule_id` | `effective_email` |
|---|---|---|---|
| Score at or above `min_confidence` (default 90 for lists) | `fix_then_send` | `finder_repair` | The found address |
| Address below `min_confidence` | `review` | `finder_repair_low_confidence` | The original address |
| No address | `review` | `finder_repair_not_found` | The original address |

`reasons` holds the method, the score, the number of sources, and the alternatives.

6. **Export** through the existing remediation exports.
7. **Credits:** 1 for each `fix_then_send` row. All other rows cost 0.

### 10. Shadow Mode

Before v2 becomes the default, the flag `finder_v2_shadow` runs v2 in the background for each v1 request of the internal tenant and opted-in tenants.

- The shadow run does not change the response and costs no credits.
- We store both answers in `v1_finder_shadow`. Later we join them with delivery outcomes.
- **Exit criteria:** after at least 2,000 shadow lookups:
  - v2 precision is equal to or better than v1 on domains that are not catch-all
  - v2 coverage is higher than v1
  - v2 uses fewer SMTP checks

## Public API / Interface Changes

`POST /v1/find_email`, with new optional fields:

```json
{
  "first_name": "Jane",
  "last_name": "Doe",
  "full_name": null,
  "domain": "https://www.acme.com/",
  "company": null,
  "strategy": "waterfall",
  "min_confidence": 50,
  "wait_seconds": 10
}
```

- Either `domain` or `company` is required. Either `first_name` and `last_name`, or `full_name`, is required.
- `wait_seconds` (0–20, default 0) returns the result inline when it is ready in time. Otherwise the response is `202` with `job_id`.
- `strategy` now controls only the SMTP order in step 4.

`GET /v1/find_email/{job_id}` keeps all current fields and adds:

```json
{
  "status": "completed",
  "best_match": {
    "email": "jane.doe@acme.com",
    "score": 92,
    "confidence": "high",
    "pattern": "first.last",
    "method": "found",
    "accept_all": false,
    "ambiguous": false,
    "confidence_score": 96,
    "confidence_level": "high",
    "confidence_version": "finder_v2.1",
    "factors": ["sources: 2 (+10)", "smtp: deliverable (+15)"],
    "sources": [
      { "url": "https://acme.com/team", "title": "Our team", "last_seen_at": "2026-10-10T08:12:00Z" }
    ]
  },
  "alternatives": [
    { "email": "jdoe@acme.com", "pattern": "flast", "confidence_score": 12, "method": "pattern" }
  ],
  "domain": { "domain": "acme.com", "accept_all": false, "pattern": "first.last", "pattern_share": 0.82 },
  "degraded": false,
  "credits_charged": 1
}
```

List repair:

- `POST /v1/lists/{list_id}/repair` with `{ "columns": { "first_name": "First", "last_name": "Last", "company": "Company", "domain": "Website" }, "scope": "both", "min_confidence": 90, "dry_run": false }`
- The response is `202` with `data_job_id` and the estimate, or `200` with the estimate when `dry_run` is true.

Pattern lookup:

- `GET /v1/domains/{domain}/pattern` returns the dominant pattern, its share, the evidence weight, and `accept_all`. It is free and never starts a Parallel call.

Errors (in addition to RFC 0032):

| Status | `error` | When |
|---|---|---|
| `409` | `company_ambiguous` | The company matches more than one domain. |
| `422` | `free_provider_domain` | The domain is a consumer provider. |
| `422` | `name_ambiguous` | `full_name` cannot be parsed. |
| `429` | `domain_fanout_limit` | The tenant reached 1,000 lookups on one domain today. |

Events:

- `finder.completed`, for async lookups only
- `list_repair.completed`

Scopes:

- `find` for lookups and patterns.
- `find` and `lists` for list repair.

## Data Model / Storage Changes

```sql
ALTER TABLE v1_finder_job
  ADD COLUMN data_job_id BIGINT REFERENCES v1_data_job(id) ON DELETE SET NULL,
  ADD COLUMN input_full_name TEXT,
  ADD COLUMN input_company TEXT,
  ADD COLUMN method TEXT,
  ADD COLUMN confidence_score SMALLINT,
  ADD COLUMN confidence_version TEXT,
  ADD COLUMN accept_all BOOLEAN,
  ADD COLUMN ambiguous BOOLEAN NOT NULL DEFAULT false,
  ADD COLUMN degraded BOOLEAN NOT NULL DEFAULT false,
  ADD COLUMN smtp_checks INTEGER NOT NULL DEFAULT 0,
  ADD COLUMN credits_charged INTEGER NOT NULL DEFAULT 0;

ALTER TABLE v1_finder_result
  ADD COLUMN method TEXT,
  ADD COLUMN confidence_score SMALLINT,
  ADD COLUMN sources_count INTEGER NOT NULL DEFAULT 0;

CREATE TABLE v1_finder_shadow (
  id                BIGSERIAL PRIMARY KEY,
  tenant_id         UUID NOT NULL REFERENCES tenants(id) ON DELETE CASCADE,
  finder_job_id     INTEGER NOT NULL REFERENCES v1_finder_job(id) ON DELETE CASCADE,
  v1_email          TEXT,
  v1_score          SMALLINT,
  v2_email          TEXT,
  v2_method         TEXT,
  v2_confidence     SMALLINT,
  v2_smtp_checks    INTEGER,
  created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

The `v1_contact_outcomes` domain column and index are in section 5. The store tables are in RFC 0032.

## Product Surface (UI)

This follows the RFC 0032 UI rules: results first, and complex controls in sheets.

**Finder page** (`web/app/(product)/app/finder`):

1. A compact input bar: name (or full name) and domain (or company).
2. The result card comes first. It shows:
   - the address, with a copy button
   - `MethodBadge` and `ConfidenceBadge`
   - an `accept_all` note
   - `CitationList`
   - the domain pattern
3. The alternatives sit in a collapsed list under the card.
4. An "Advanced" sheet holds `strategy`, `min_confidence`, and `wait_seconds`.
5. `CreditCostHint` shows "1 credit if found".

**List repair** (list detail page):

1. A "Repair addresses" button opens a sheet. The sheet has the column mapping (auto-detected), the scope, `min_confidence`, and the estimate from `dry_run`.
2. Approve in the same sheet. Then `DataJobStatus` shows the progress.
3. The results appear in the remediation tab, with before and after values and the evidence for each row.

## Security And Abuse

- Rate limits for each tenant: 15 requests a second and 500 a minute by default. Plans can change them.
- The per-domain fan-out limit (section 7) stops mass guessing on one company.
- Free-provider domains are refused, so the finder cannot probe consumer mailboxes.
- RFC 0032 validation applies to all Parallel output.
- Tenant-private evidence is read with the caller's `tenant_id` only. A test checks that tenant B's outcomes never change tenant A's result.

## Observability And SLOs

Metrics, from the job and provider call tables:

- coverage and precision, by method
- SMTP checks for each lookup
- the store hit rate
- Parallel cost for each returned address
- the share of degraded results

**Calibration drift:** each week we compare predicted scores with delivery outcomes. A difference of more than 5 points in any bucket creates an alert.

| SLO | Target |
|---|---|
| p95, address in the store and verified in the last 30 days | 1 second or less |
| p95, new domain | 15 seconds or less |
| List repair throughput, with 80% of domains already sourced | 600 rows a minute or more |

## Cost Model

| Case | Parallel cost |
|---|---|
| Store hit | $0 |
| Known domain, person task needed | $0.005 |
| New domain (Stage A, 6 URLs, + Stage B) | $0.016 |
| New domain + person task | $0.021 |

Sourcing is paid once in 30 days for each domain and is shared by all tenants. With a store hit rate of 50%, the average cost is about $0.008 for each lookup.

## Failure Modes And Edge Cases

- **Parallel is down or the circuit is open:** skip steps 1 and 3. Use the store, the pattern model, and SMTP. Set `degraded: true`.
- **Parallel returns an address on another domain, or one that is not in any excerpt:** drop it.
- **The page lists a generic address only:** store it as `generic`. Never return it as a person's address.
- **The name has no Latin transliteration:** return `422` with `name_unsupported`.
- **The pattern changed recently:** recency weighting lowers old evidence. Both patterns appear in the alternatives.
- **The domain redirects to a parent domain:** use the domain with MX records. Show the input domain and the mail domain.
- **The SMTP server greylists:** the check retries through the normal task retry. It does not count against the 3-check budget twice.
- **The tenant's outcomes contain a bounced address equal to the top candidate:** skip it and take the next candidate.
- **An opt-out arrives during a list repair:** the row becomes `review` with `opted_out`, and it costs 0.

## Alternatives Considered

- **Our own web crawler.** It is a large, separate business. Parallel gives search, extraction, and citations as a service.
- **A bought contact database.** It adds licence cost and a second privacy chain, and it has no sources for each address.
- **An LLM to guess the pattern.** It gives no evidence and cannot be calibrated.
- **A raw score without calibration.** A raw score is easy to compute, but customers cannot set a safe threshold with it. A calibrated score is a clear advantage over Hunter.

## Rollout / Migration / Compatibility

1. Ship RFC 0032 M1–M4.
2. Ship finder v2 behind `finder_v2_shadow` for the internal tenant. Run shadow mode until the exit criteria are met.
3. Run the live evaluation and the Hunter benchmark. Fit the calibration table.
4. Enable `finder_v2` for beta tenants after the Legal Review sign-off.
5. Enable `list_repair` for beta tenants.
6. Make v2 the default for all tenants. Keep v1 behavior when `parallel.enabled = false`.

The response only adds fields. Old clients keep working.

## Milestones

| # | Deliverable | Size |
|---|---|---|
| F1 | Name parsing, variants, transliteration, domain and company resolution | M |
| F2 | Domain sourcing (Stages A and B) and the store write path | M |
| F3 | Pattern model, tenant-private evidence, outcome index | M |
| F4 | Raw score, factors, calibration table, eval fit | M |
| F5 | Evidence ladder, SMTP budget, domain limits, shadow mode | M |
| F6 | List repair job and remediation integration | M |
| F7 | Finder page and list repair sheet | M |

## Metrics And Success Criteria

- Coverage and precision, for each method and for catch-all domains.
- The bounce rate of repaired addresses in later ESP outcomes.
- The share of list repair rows that tenants export.
- Average SMTP checks for each lookup.
- Parallel cost for each returned address.

## Test Strategy

- A name-parsing table with at least 300 names: Latin, accented, hyphenated, particles, comma form, titles and suffixes, CJK, Cyrillic, Arabic, and Devanagari.
- Unit tests for variant generation and the 4-variant cap.
- Unit tests for the posterior, recency weighting, and the dominant-pattern rule.
- Unit tests that check that the calibration table is monotonic.
- Unit tests for each ladder branch, with the Parallel stub and sandbox SMTP.
- A tenant isolation test for tenant-private evidence.
- A regression test: with Parallel disabled, the finder gives the same answer as v1 on non-catch-all sandbox domains.

## Definition Of Done

This RFC is done only when all items below are true. "Passes" means it passes in CI on the pull request.

### Build

1. RFC 0032 M1–M4 are done.
2. The migration runs up and down. `e2e_migration_upgrade` passes.
3. `backend/openapi.json`, the SDKs, and the web clients show all new fields and endpoints. CI finds no diff after generation.

### Unit And API Tests

4. All unit tests in Test Strategy pass in the `test` CI job.
5. The API harness has canonical cases for the changed finder endpoints, the pattern endpoint, and list repair. Each case covers success, `403` without the scope, and `404` for another tenant's job.

### End-To-End Tests (Backend)

6. `backend/tests/e2e_finder_v2.rs` runs HTTP, Postgres and RabbitMQ (with `testcontainers`), the worker, the data job runner, the Parallel stub, and sandbox SMTP. It passes these scenarios:

| # | Scenario | Expected result |
|---|---|---|
| a | New domain, address on a cited page | `method = found`, sources, 1 credit, ledger rows, provider call rows |
| b | Same lookup again in the same period | 0 credits, 0 new stub calls |
| c | Dominant pattern on a domain that is not catch-all | `method = pattern`, exactly 1 SMTP check |
| d | Dominant pattern on a catch-all domain | `accept_all = true`, `safe_to_send = false` |
| e | Catch-all domain, weak web evidence, tenant has delivered outcomes with `first.last` shapes | Top candidate is `first.last`; tenant B with no outcomes gets a lower score |
| f | Catch-all domain, the top candidate hard-bounced for this tenant | The next candidate is returned |
| g | No evidence on a catch-all domain | `method = guess`, below 50, alternatives only, 0 credits |
| h | Stub timeout or `500` | `degraded = true`, 0 Parallel credits, SMTP path works |
| i | Off-domain address, or an address in no excerpt | Dropped, not stored |
| j | Opted-out address or domain | `451`, nothing stored |
| k | Credit limit reached | `402`, 0 stub calls |
| l | `parallel.enabled = false` | Same answer as v1 |
| m | `company` with one match and with two matches | Domain resolved; `409` with candidates |
| n | `domain` given as a URL with `www.` | Normalized and accepted |
| o | Names: "王小明", "Jean-Luc Picard", "Ana van der Berg", "Doe, Jane" | Correct variants and a result |
| p | `wait_seconds = 10` on a fast lookup | Inline `200` |
| q | 1,001st lookup on one domain in a day | `429` `domain_fanout_limit` |
| r | List repair: `dry_run`, approval, run, remediation rows, export | Correct estimate, rows with the correct classification and `effective_email`, credits only for `fix_then_send` rows |
| s | Shadow mode on | Response is unchanged; one `v1_finder_shadow` row |

### End-To-End Tests (Web)

7. Playwright specs cover:
   - a single lookup, with the result card, the badges, the sources, and the alternatives
   - the advanced sheet
   - list repair from the list page to the remediation tab
   - the `402`, `451`, and degraded states
8. The specs have the `@smoke` tag, and `test:a11y` passes. Each new component has a story and a passing component test.
9. `ci/smoke_full_stack_image.bash` calls the finder on the built image and checks the response.

### End-To-End Tests (Live, Staging)

10. A live test runs on staging with the real Parallel API when `PARALLEL_LIVE_E2E=1`.
    - It uses an evaluation set of 1,000 people whose addresses had a recent `delivered` outcome in our internal tenant. At least 300 of them are on catch-all domains, and at least 100 have names in a non-Latin script.
    - Precision is the share of returned addresses that match the known delivered address.
    - The pass targets (set 2026-10-16) are:

    | Metric | Target |
    |---|---|
    | Precision of `found` addresses | 99% or more |
    | Precision of `pattern` addresses on domains that are not catch-all | 97% or more |
    | Precision of `found` and `pattern` addresses on catch-all domains | 90% or more |
    | Coverage: lookups that return an address | 80% or more |
    | Calibration: each 10-point bucket with 50 or more samples | Within ±3 points of the measured precision |
    | Average SMTP checks for each lookup | 1.5 or fewer (today: up to 13) |
    | p95, address in the store and verified in the last 30 days | 1 second or less |
    | p95, new domain | 15 seconds or less |
    | Median Parallel cost for each returned address | $0.01 or less |

    If one target is not met, the feature does not leave beta.
11. **Hunter benchmark.** We run the same evaluation set through the Hunter Email Finder API, after RFC 0032 Legal Review item 13 has a sign-off. We must match or beat Hunter on precision, on coverage, and on precision on catch-all domains.
12. **Shadow exit.** The shadow mode exit criteria in section 10 are met.
13. **Load.** The RFC 0032 load test holds the SLOs.
14. The reports (our results, Hunter's results, calibration, shadow, load, and cost) are attached to the release pull request.

### Operations And Release

15. The dashboards show all metrics in this RFC. The calibration drift alert fires in a staging drill.
16. Rollback works on staging: turn off `finder_v2`, and the finder returns to v1 behavior with no data loss.
17. The RFC 0032 Legal Review has a sign-off.
18. The public API docs and the changelog describe the new fields, list repair, the credits, and the new errors.

## Open Questions

1. Which given-name dictionary can we use, and under which licence? Owner: backend lead.
2. Can verification-based pattern evidence (domain and pattern only) be shared across tenants by default? Owner: legal owner.
3. Is 90 the right default `min_confidence` for list repair? Owner: product.

## Unresolved Risks

1. **Legal.** See RFC 0032 Legal Review.
2. **Parallel accuracy.** The Task API can return an address that is not on the cited page. The excerpt rule removes these, but it can also remove some correct addresses.
3. **Cost.** The margin depends on the store hit rate.
4. **Calibration data.** Calibration needs enough delivered outcomes on catch-all domains. If the internal tenant has too few, we must use a second evaluation source.
