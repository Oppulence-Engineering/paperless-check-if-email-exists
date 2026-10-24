# RFC 0003: Agency Audits and Import Lineage

| Field | Value |
|---|---|
| Status | Proposed; stage 1a |
| Updated | 2026-10-24 |
| Owners | Rust, Go, and web leads; assign before implementation |
| Depends on | RFC 0001 pilot gate; RFC 0002 measurement identifiers |
| Enables | RFCs 0004–0007; client context for RFC 0011 |

## Problem and outcome

The [research API](../../backend/src/http/v1/research.rs) takes a free-form name
and raw CSV/JSON, then starts a contact column. It exposes row keys but omits them
from exported CSV. Agencies need to associate work with clients and campaigns,
preserve customer IDs, and reconcile every corrected row back to the original.
The [email-list importer](../../backend/src/http/v1/lists/post.rs) has useful
original-row and deduplication primitives but is not a company/contact audit model.

## Ownership and model

Rust owns tenant-scoped `audit_clients`, `audit_campaigns`, `contact_audits`,
`audit_imports`, and `audit_input_rows`. Go continues to own research sessions,
execution, and research observations. Join with immutable IDs; do not recreate
Go's research engine or put product data in Better Auth's identity database.

Each audit stores client/campaign, target-role criteria and version, source label,
created-by/time, agreed scope, and evaluation cohort reference. Each input row has
an internal UUID, original customer ID (which need not be unique), source file
hash/version and row number, original fields, mapped fields, normalization warnings,
duplicate group, and explicit Go session/row references. Store large Go IDs as
decimal strings through web/SDK boundaries.

Deduplicate work without deleting original rows. Repeated customer IDs or equal
names are not proof of identity; preserve all occurrences and their mappings.
Corrected values are revisions with reasons, never silent replacement of originals.

Client/campaign labels organize work **within a tenant**. They do not introduce
client-level ACL isolation. Use separate existing workspaces for clients requiring
isolation; all authorized agency members otherwise share tenant visibility.
Suppressions remain tenant-wide until RFC 0011 is deliberately adopted.

## Proposed interfaces and flow

Add client/campaign CRUD and `/v1/audits` resources. Split import into
`POST /v1/audits/{id}/imports/preview` and an idempotent commit using the preview
hash and mapping version. Preview maps company, domain, person, role, email, and
original-ID columns; returns row counts, missing fields, malformed values, duplicate
groups, and company-domain conflicts. Preview/commit do not purchase research.

After commit, explicit attachment links accepted rows to existing pilot research
sessions; automatic paid runs use RFC 0005's approval/reservation slice. Keep the ten-row pilot cap until RFC 0005 passes its
capacity gate. Missing-domain and ambiguous rows remain visible for review; do not
claim name-only company resolution merely because import succeeds.

Introduce the six audit scopes in the shared technical contract across Rust,
API-key validation, and shared route authorization. Map member/admin roles
explicitly; existing `find` keys do not silently acquire new audit authority.
Reads, exports, related IDs, and joins all validate tenant ownership. Existing
research API and keys retain their current behavior.

The web uses the existing BFF, generated SDKs, TanStack Query, and runtime schemas.
Add audit list/detail and field mapping to the research experience, preserving the
original independent research page for compatibility.

## Incremental delivery and compatibility

1. Add tenant-owned audit containers and import previews with original-row storage.
   Deliver a downloadable annotated input file even before new research features.
2. Link bounded existing research runs and show progress against source rows.
3. Expose lineage to RFC 0004 exports and RFC 0002 measurement. Legacy sessions
   can be attached explicitly; missing original IDs are marked unknown, not invented.

Use additive migrations and composite tenant/resource constraints. Preview expiry
or changed mappings require a new preview; request-key/body conflicts return 409.
Keep uploads and exports under RFC 0001 retention. File parsing has byte/row limits
and spreadsheet-safe exports; reject overflow visibly rather than truncating it.

## Acceptance

- Round-trip a realistic list with duplicate IDs, duplicates, missing emails,
  malformed domains, Unicode names, formula cells, and company conflicts. Every
  original row remains addressable and no research begins during preview.
- Same import/commit retry returns the same resource; changed payload conflicts.
- Concurrent runs cannot lose lineage or create duplicate paid work for the same
  linked operation. Failed linkage is recoverable by stable operation ID.
- Cross-tenant client, campaign, import, session, row, and export IDs are rejected
  before data access or provider work; scope/role refusals are tested through BFF
  and API-key entry points.
- Regenerate OpenAPI and SDKs together, test additive compatibility, and validate
  a browser import-to-linked-run journey using a stub provider before pilot rollout.

## Technical specification

Use [Verify technical contracts v1](verify-technical-contracts.md) for IDs,
scopes, errors, idempotency, pagination, operations, and rollout. All routes below
are proposed additions. Extend `backend/src/research.rs`; do not expose the Go
workspace key or let a request choose its Go tenant/workspace.

### Relational model and constraints

The core migration must enforce these relationships, not just check them in UI:

```sql
CREATE TABLE audit_clients (
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    id UUID NOT NULL,
    name TEXT NOT NULL CHECK (length(name) BETWEEN 1 AND 200),
    version BIGINT NOT NULL DEFAULT 1 CHECK (version > 0),
    archived_at TIMESTAMPTZ,
    PRIMARY KEY (tenant_id, id)
);
CREATE TABLE audit_campaigns (
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    id UUID NOT NULL,
    client_id UUID NOT NULL,
    name TEXT NOT NULL CHECK (length(name) BETWEEN 1 AND 200),
    version BIGINT NOT NULL DEFAULT 1 CHECK (version > 0),
    PRIMARY KEY (tenant_id, id),
    UNIQUE (tenant_id, client_id, id),
    FOREIGN KEY (tenant_id, client_id) REFERENCES audit_clients(tenant_id, id)
);
CREATE TABLE contact_audits (
    tenant_id UUID NOT NULL REFERENCES tenants(id),
    id UUID NOT NULL,
    client_id UUID NOT NULL,
    campaign_id UUID,
    name TEXT NOT NULL,
    target_criteria JSONB NOT NULL,
    criteria_version BIGINT NOT NULL DEFAULT 1,
    generation BIGINT NOT NULL DEFAULT 1,
    version BIGINT NOT NULL DEFAULT 1,
    state TEXT NOT NULL CHECK (state IN ('draft','active','closed','deleting')),
    PRIMARY KEY (tenant_id, id),
    FOREIGN KEY (tenant_id, client_id) REFERENCES audit_clients(tenant_id, id),
    FOREIGN KEY (tenant_id, client_id, campaign_id)
        REFERENCES audit_campaigns(tenant_id, client_id, id)
);
```

Add positive checks to all revision counters, actor/creation/update timestamps,
`(tenant_id,created_at,id)` list indexes, and an index on each referencing FK.
Archived clients remain readable but cannot receive new audits. Audit creation
locks the client row and checks `archived_at IS NULL` in the insertion transaction;
archive takes the same lock, so concurrent creation cannot bypass the check.
Deletion uses
RFC 0001's tombstone workflow; do not cascade-delete active external operations.
Campaign/client labels are mutable; moving an audit between clients is not supported
in v1 because retained CRM/source authorizations can be client-specific.

| Table | Required fields and constraints |
|---|---|
| `audit_imports` | Tenant/audit/import IDs, request key/hash, raw SHA-256, mapping JSON/version, normalized SHA-256, parser version, state, counts, expiry. Composite audit FK; unique `(tenant_id,request_key)`; unique `(tenant_id,audit_id,id)`. |
| `audit_import_payloads` | Tenant/import FK, raw bytes, content type, byte count, deletion deadline. Database-only access, excluded from list APIs/logging. Pilot cap 1 MiB per file; enforce database/storage encryption in deployment. |
| `audit_input_rows` | Tenant/audit/import/row UUIDs, 1-based source record number, nullable original customer ID, original JSON, normalized JSON, warnings JSON, eligibility state, duplicate-group ID, version. Composite import FK includes audit ID; unique `(tenant_id,import_id,source_row_number)` and `(tenant_id,audit_id,id)`. |
| `audit_row_revisions` | Tenant/input row/revision, proposed normalized values, reason, actor/time; append-only. Composite FK to input row. Import originals are immutable. |
| `audit_research_links` | Tenant/audit/input row, Rust operation UUID, Go session/row IDs, state, last reconciled time. Composite FKs to audit/input row and operation; unique `(tenant_id,operation_id,input_row_id)` and `(tenant_id,go_session_id,go_row_id)`. |

`target_criteria` is a versioned typed object with `role_terms`, optional geography,
`named_person_required`, and `responsibility_requirement` (nullable string).
Reject unknown fields, empty role terms, and unsupported operators. It is not an
arbitrary SQL/expression language. Changing criteria increments both version and
generation and invalidates affected review/repair previews.

### Parser, normalization, and preview protocol

Initially accept UTF-8 CSV (optional BOM, quoted multiline cells) or a JSON array
of objects. Reject other encodings, duplicate/empty CSV headers, duplicate JSON
keys, nesting deeper than 4, more than 128 columns, fields over 4 KiB, or records
over 16 KiB. Check byte limits while reading, before buffering/parsing. Initial
limits are 1 MiB and ten records per import; neither parsing nor preview silently
truncates. Stage 2 may raise the tenant cap after load tests, not by default.

Keep Unicode names verbatim. For domains, trim, parse an optional HTTP(S) URL,
extract the host, IDNA-normalize and lowercase, remove its trailing dot, and reject
credentials/IP literals/invalid labels. Preserve the submitted value separately.
For emails, trim and lowercase the domain only; preserve the local part. Do not
infer a person's identity from an equal normalized email or infer a company from
an email provider domain. Network resolution/fetching is absent from preview.

```http
POST /v1/audits/{audit_id}/imports/preview
Content-Type: application/json
Idempotency-Key: <UUID>
```

```json
{
  "expected_version": "1",
  "format": "csv",
  "content": "crm_id,company,domain,person\n42,Example,example.com,Ada\n",
  "mapping": {
    "original_id": "crm_id", "company": "company",
    "domain": "domain", "person": "person"
  },
  "source_label": "customer-export-2026-10"
}
```

Return 201 with `import_id`, `version`, `preview_hash`, `expires_at`, total/valid/
review-required counts, a sample of at most 20 records, and coded warnings such as
`missing_domain`, `invalid_email`, `duplicate_original_id`, and
`company_domain_conflict`. Hash binds raw file hash, mapping, parser version,
criteria version, and normalized output. Warning text is explanatory, not a client
state enum. Potentially untrusted cells render as text in the browser.

`POST /v1/audits/{audit_id}/imports/{import_id}/commit` takes
`{"expected_version":"1","preview_hash":"<sha256>"}`. Under the tenant/audit/import
locks, verify expiry (24 hours), deletion fence, criteria version and preview hash;
insert all original rows, increment audit generation, mark committed, and save the
idempotent response in one transaction. Return 201 with row counts and rows URL.
Concurrent commits for the same `import_id`, even with different keys, find the
same committed import and cannot insert its rows twice. Different preview requests
can create distinct imports with identical content; that is an explicit re-import,
not retry deduplication. Duplicate-work keys do not establish import identity. A changed mapping requires a new preview. Retry after
expiry returns the saved successful commit if already committed, otherwise 410.

Invalid/ambiguous records commit as `needs_review` with their reasons. Structural
parse failure rejects the entire preview with record/column coordinates and no
partial commit. Duplicate work keys use tenant-scoped HMACs over normalized target
inputs plus criteria version; every occurrence retains its own UUID. Store the
algorithm/key version. These keys are reuse hints, never identity proof or public
identifiers. A reuse decision must still validate rights and evidence freshness.

### Bridge admission and row lineage

Add a negotiated Go `audit_lineage_v1` contract alongside the existing import API:

1. Rust writes an `audit_operations` row and dispatch intent with accepted input
   UUIDs. The worker resolves the existing trusted tenant-to-workspace binding.
2. Go accepts `POST .../internal/verifier-audit-imports` with operation ID, audit
   ID, original Rust tenant UUID, server-resolved Go workspace, contract version 1,
   expected version `"0"`, criteria version and rows containing `external_row_id`
   (the input UUID). Go validates the trusted tenant/workspace mapping and equality
   with the persisted operation authority before admitting rows.
   Import creates **no executing column**. Go atomically persists the operation,
   its payload hash, imported rows and explicit UUID-to-Go-row mapping.
3. `GET .../internal/verifier-operations/{operation_id}` returns that mapping and
   admission state. Replaying identical admission returns the same session/rows;
   changed body returns 409. Rust persists mappings before any run is authorized.
4. Bounded execution starts through a separate authorized operation. Never join
   by CSV position, company name, or response ordering. Every candidate/result
   carries the source input UUID and Go row ID; mismatches fail reconciliation.
   Each original input gets a distinct Go row even when execution reuses evidence;
   reuse shares an evidence/attempt reference below this mapping, not a row identity.

The current `create_in_go` imports and starts `runImmediately:true` in one user
flow. Do not call it from audit preview/commit. The Go side of this new contract
requires an implementation and integration tests in its own repository. Missing
capability keeps execution disabled while local imports remain useful.

Stage 1 can link an explicitly selected existing pilot session after checking its
workspace and manually confirming each input mapping. Imported evidence with
unknown lineage is marked legacy/unknown. Automatic paid execution of newly linked
audits waits for RFC 0005's approval/reservation slice. Rights-cleared concierge
work can proceed under separately agreed paid-audit scope; any paid dispatch still
requires audit-owner approval, a durable worst-case reservation, stable operation
IDs and reconciliation under RFC 0005. A controlled manual ledger can implement
that pilot slice; it is not an exemption from spend authorization. This avoids an
unpriced run action being introduced as a temporary API bypass.

### Endpoint and implementation checklist

| Route | Semantics |
|---|---|
| `GET/POST /v1/audit-clients` | Cursor list / create with read+write. |
| `PATCH /v1/audit-clients/{id}` | Versioned rename/archive; existing audits preserved. |
| `GET/POST /v1/audit-clients/{id}/campaigns` | Tenant/client-bound list/create. |
| `PATCH /v1/audit-clients/{id}/campaigns/{campaign_id}` | Versioned rename; reject wrong client. |
| `GET/POST /v1/audits` | Filtered list / typed criteria and client/campaign creation. |
| `GET/PATCH /v1/audits/{id}` | Detail / versioned name, criteria or lifecycle update. |
| `GET /v1/audits/{id}/imports/{import_id}` | Preview/commit state, counts and warnings, no raw blob. |
| `GET /v1/audits/{id}/rows` | Stable input UUIDs, original/normalized values, latest revisions. |

Implement parser/repository/bridge boundaries in `backend/src/audits/imports.rs`
and routes under `backend/src/http/v1/audits/`. Add an explicit session-attachment
endpoint only with the same operation/tenant/mapping checks; never accept arbitrary
Go session ownership from the browser. Extend web runtime schemas and invalidate
query keys by tenant/audit/generation after mutations. Preview refresh never commits
or purchases work. Existing sessions are not bulk-backfilled into paid audits.

Required fixtures include quoted newlines, BOM, IDNA hosts, `=HYPERLINK(...)`,
duplicate IDs, duplicate headers, 64-bit Go IDs above JavaScript's safe-integer
range, and file/record limits at boundary ±1. Inject failures before/after Go
admission and Rust mapping commit; the same operation must recover identical row
UUIDs with zero research calls. Database tests must attempt cross-tenant/cross-audit
FK inserts directly. Metrics: parsed/rejected/committed records, preview expiry,
lineage mismatch count, and oldest unreconciled bridge operation.
