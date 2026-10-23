# RFC 0029: Agency Audits and Import Lineage

| Field | Value |
|---|---|
| Status | Proposed; stage 1a |
| Updated | 2026-10-23 |
| Owners | Rust, Go, and web leads; assign before implementation |
| Depends on | RFC 0027 pilot gate; RFC 0028 measurement identifiers |
| Enables | RFCs 0030–0033; client context for RFC 0019 |

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
Suppressions remain tenant-wide until RFC 0019 is deliberately adopted.

## Proposed interfaces and flow

Add client/campaign CRUD and `/v1/audits` resources. Split import into
`POST /v1/audits/{id}/imports/preview` and an idempotent commit using the preview
hash and mapping version. Preview maps company, domain, person, role, email, and
original-ID columns; returns row counts, missing fields, malformed values, duplicate
groups, and company-domain conflicts. Preview/commit do not purchase research.

After commit, an explicit bounded run action links accepted rows to existing
`/v1/research` sessions. Keep the ten-row pilot cap until RFC 0031 passes its
capacity gate. Missing-domain and ambiguous rows remain visible for review; do not
claim name-only company resolution merely because import succeeds.

Introduce `audits.read`, `audits.write`, `audits.review`, and `audits.export` scopes
in Rust, API-key validation, and shared route authorization. Map member/admin roles
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
3. Expose lineage to RFC 0030 exports and RFC 0028 measurement. Legacy sessions
   can be attached explicitly; missing original IDs are marked unknown, not invented.

Use additive migrations and composite tenant/resource constraints. Preview expiry
or changed mappings require a new preview; request-key/body conflicts return 409.
Keep uploads and exports under RFC 0027 retention. File parsing has byte/row limits
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
