# RFC 0032: Preview-First HubSpot Writeback

| Field | Value |
|---|---|
| Status | Proposed; demand-gated stage 3 |
| Updated | 2026-10-23 |
| Owners | Rust integration lead and web lead |
| Depends on | RFCs 0027, 0029, 0030; requested workflow from repeat-audit customers |
| Shared prerequisite | RFC 0023 tenant secret cipher only, or an equivalent reviewed credential service |

## Problem and scope

CSV proves audit value first. When repeat buyers ask to avoid manual merging,
support a limited HubSpot correction workflow with the same evidence and policy
gates as CSV. This RFC proposes new functionality; no HubSpot research writeback
was found in the [audit](../audits/2026-10-23-verify-north-star.md).

Start with updates to Verify-owned properties on explicitly matched existing
contacts; read companies to corroborate identity. Native contact fields and company
updates remain preview/CSV suggestions until a stronger field-specific concurrency
contract is approved. Creating contacts, merges, deletion, sequences, campaign
enrollment, and general bidirectional synchronization are outside the first release.

## Connection and change contract

Store each HubSpot connection in the Rust product backend, scoped to tenant and
optionally a client/workspace mapping. Use minimum required OAuth scopes, encrypted
server-side credentials, state validation, rotation, revocation, and explicit
installation authority. Better Auth sign-in tokens are not CRM credentials.
Reuse the cipher slice of RFC 0023; its full webhook outbox is not a prerequisite.

Propose connection routes plus `/v1/audits/{id}/crm-previews` and an explicit apply
operation. A preview includes the portal/connection, source row and CRM object IDs,
current CRM values, proposed fields, before/after changes, evidence references,
conflicts, and eligibility. Resolve mappings by retained CRM IDs and corroborating
identity evidence; do not choose an ambiguous object by email/name alone.

Default to filling approved blank Verify-owned fields. Overwriting a nonblank
Verify-owned value requires an explicit field-level choice. Never overwrite CRM opt-out, subscription, or
consent fields or infer permission to contact from a valid mailbox. Customer do-not-
contact signals block the update or require review according to the agreed contract.

## Apply and recovery

Persist a versioned plan and actor approval. At apply time, reload current CRM
values, connection authority, evidence, and sending/suppression policy. Refuse
stale plans/conflicting fields and show a refreshed diff. HubSpot concurrency
semantics must be verified: if conditional writes are unavailable, expose the
remaining race and limit the first release to a reviewed field allowlist with
immediate read-before-write and read-after-write checks. Do not promise atomicity.

Use a durable operation ledger per tenant/connection/object/plan version, stable
retry keys, rate-limit backoff, and per-object outcomes. An ambiguous timeout must
read/reconcile the remote state before replay. Partial success is visible and
retrying failed rows cannot rewrite successful ones. Stop new work on revocation.
Any reversal is a newly previewed change against current CRM state, not a blind
rollback that erases subsequent customer edits.

## Rollout and acceptance

1. Verify current API terms, scopes, rate limits, sandbox behavior, and field update
   semantics; record them before implementation. Keep credentials out of logs/UI.
2. Ship read-only matching/diff preview for two requesting design partners.
3. Enable a small, approved field allowlist with explicit apply and per-row history;
   expand only with measured reduction in customer merge time.

Test cross-tenant/portal confusion, OAuth state/revocation, ambiguous matches,
missing original CRM IDs, concurrent CRM edits, late suppressions/opt-outs,
permission changes, rate limits, partial failures, and timeout reconciliation.
Validate previews with sandbox contracts before any bounded pilot. Regenerate
API/SDK contracts and preserve CSV as the recovery path. RFC 0028 records customer
time saved and added integration/support cost; a connector alone is not evidence
of retention or margin.

## Technical specification

Apply [Verify technical contracts v1](verify-technical-contracts.md). Implement the
adapter in `backend/src/audits/crm.rs`; the web uses the same-origin BFF. The API
paths below specify the proposed integration against HubSpot CRM v3/OAuth; verify
current provider documentation, scopes and sandbox behavior before enabling it.
No live HubSpot account or API contract was validated by this documentation change.

### Connection, OAuth, and credential storage

| Table | Required fields and constraints |
|---|---|
| `audit_crm_connections` | Tenant/connection UUID PK, provider=`hubspot`, portal ID as decimal string, optional client FK, state, granted scope list, token expiry, credential version, version, actor/timestamps. Unique active `(tenant_id,provider,portal_id,client_id)` with explicit NULL handling. |
| `audit_crm_credentials` | Tenant/connection FK, credential version, key ID, nonce, encrypted token bundle, created time. No public serializer. |
| `audit_crm_oauth_states` | SHA-256 state hash PK, tenant, initiating user/session binding, intended client, exact redirect URI, created/expiry/consumed times. Ten-minute expiry, single consumption. |
| `audit_crm_object_mappings` | Tenant/connection/audit/input row/object type/object ID, identity evidence version/hash, confirmation actor/time. Composite FKs; reject multiple candidate objects rather than choosing one. |
| `audit_crm_previews` | Tenant/preview UUID, audit/connection FKs, generation, policy epoch, connection version, immutable changes/hash, expiry, version/state. |
| `audit_crm_apply_items` | Tenant/item UUID, operation/preview/connection/input row, object type/ID, before/after hashes and values, field allowlist version, state, request hash, attempts, outcome/evidence references. Unique `(tenant_id,preview_id,object_type,object_id)`. |

Connection setup requires a currently authorized owner/admin session with manage
scope. API keys may use an existing explicitly authorized connection but cannot
complete an interactive user's OAuth consent. Generate 32 random bytes for `state`,
store only its hash, bind tenant/user/client/redirect and validate expiry/session
before atomic consumption. Callback code exchange uses the exact configured HTTPS
redirect URI and server client secret. Use PKCE if supported by the selected HubSpot
app flow; confirm this in the adapter contract rather than sending an unsupported
parameter. Reject callbacks whose portal does not match a preselected connection.
Never accept tenant/portal authority from unsigned callback query parameters.

Use the RFC 0023 cipher slice: AES-256-GCM with a fresh nonce and key ID; bind
`tenant_id || connection_id || credential_kind || credential_version` as associated
data with an unambiguous length-prefixed encoding. Keys come from the configured
secret service/environment, not the database. Rotation decrypts with the recorded
key and reseals under the active key in a CAS update. Redact access/refresh tokens,
authorization codes and response bodies from logs and traces.

Serialize refreshes with a connection credential-version lease; do external token
I/O outside the database transaction. Only the lease owner may CAS-persist the new
bundle. Other workers wait/read the new version rather than independently refreshing
an old rotating token. An uncertain refresh that may have rotated a token enters
`reauthorization_required` unless the provider documents a safe recovery path.
On revoke/disconnect, stop new apply admission, invalidate previews and purge tokens;
retain only permitted operation receipts. Reauthorization never resumes old approved
writes automatically.

### Routes and field allowlist

| Route | Contract |
|---|---|
| `POST /v1/audit-crm-connections/hubspot/authorize` | Manage; client ID and expected client version; return server-generated authorization URL. |
| `GET /v1/audit-crm-connections/hubspot/callback` | OAuth code/state validation plus initiating-session binding; no audit API-key substitute. |
| `GET /v1/audit-crm-connections` | Manage; portal/client/status/scopes, never tokens. |
| `DELETE /v1/audit-crm-connections/{id}` | Manage; versioned revoke, idempotent. |
| `POST /v1/audits/{id}/crm-previews` | Write; connection ID, input/object mappings, approved candidate versions, requested fields, expected audit version. |
| `GET /v1/audits/{id}/crm-previews/{preview_id}` | Read; per-field before/after, eligibility/conflicts, expiry/hash. |
| `POST /v1/audits/{id}/crm-previews/{preview_id}/apply` | Manage+export; preview version/hash and explicit selected changes; returns 202. |

OAuth callbacks are an explicit exception to the shared write-header convention:
they use single-use state/code instead of an `Idempotency-Key`. Authorization URL
creation and all authenticated configuration/apply writes retain that convention.

First-release writes target existing **contacts only** and only integration-owned
properties: `verify_audit_id`, `verify_candidate_id`, `verify_role`,
`verify_company_domain`, `verify_checked_at`, and `verify_operation_id`. Companies
can be read for identity corroboration; company writes are a later reviewed slice.
Provision these properties in an explicit setup operation and verify their types
before enabling apply. Request contact read/write and only the schema permissions
needed for that setup. No scopes for marketing email, subscriptions, sequences,
object deletion or broad CRM administration.

Native `email`, `firstname`, `lastname`, `jobtitle` and company association changes
remain suggestions in the preview/CSV until a later field-specific concurrency
contract is approved. Consent/opt-out/subscription properties are never writable.
The customer sees the exact property labels and changed values before apply. Even
Verify-owned fields can be edited in HubSpot; do not imply they are provider-locked.

Use retained CRM object IDs plus person/company evidence. An exact email search is
a candidate lookup, not authorization to update the returned object. No ID or
multiple plausible matches produces `mapping_required`. An object from another
portal or client-bound connection is rejected before revealing its values.

### Preview, apply, and remote concurrency

The adapter implements `read_contact`, `read_company`, `read_properties`,
`patch_contact`, `refresh_token`, and `revoke_credentials` where supported. Expected
CRM reads/writes use `/crm/v3/objects/contacts/{id}` and property metadata endpoints;
OAuth token exchange uses `/oauth/v1/token`. Pin provider origins, reject redirects,
set connect/total timeouts (5/30 seconds initially), bound response size and reject
unexpected object IDs. Do not follow arbitrary URLs returned in CRM property values.

Preview reads only the allowlisted fields and any required do-not-contact evidence.
Store canonical sorted field/value hashes, remote `updatedAt`, evidence versions,
and a 15-minute maximum expiry. Missing a required opt-out/DNC signal produces
`policy_unknown`; it does not mean consent. Keep this customer policy distinct from
mailbox validity. A preview with any unknown eligibility cannot offer that row as
applicable.

If selected input rows map to the same remote object, preview marks them
`duplicate_object_mapping` and requires an explicit choice of one source row for
that apply plan. Do not order-dependent merge their fields or silently discard one.
Apply admits only the approved subset and persists one item per remote object.
Acquire a durable tenant/connection/object fence before dispatch; an unresolved
older item blocks newer writes to that object. Under that fence:

1. Revalidate connection, mapping, deletion, rights, current approval and suppression.
2. Re-read the approved before-values. A difference or changed preview generation
   returns `remote_conflict` without sending a patch.
3. Persist `dispatching` with exact payload hash and operation marker before HTTP.
   Patch only the chosen Verify-owned fields plus `verify_operation_id`.
4. Read back those fields. Record `applied` only when values and marker match;
   otherwise retain the observed diff as `conflict` or `outcome_unknown`.

HubSpot's conditional update guarantees must be confirmed in contract tests. Until
then, v1 assumes there is **no compare-and-swap**: the read/patch interval has a race
with external editors. Narrow fields reduce impact but do not remove the race.
Disclose this limitation in the apply confirmation and keep native customer-field
overwrite disabled. Local fencing serializes Verify workers, not HubSpot users.

```text
queued -> validating -> dispatching -> applied
                     -> conflict   -> outcome_unknown -> applied_observed
                     -> blocked                       -> conflict
                                                     -> manual_resolution
```

A lost patch response is never blindly retried. Fetch current values and operation
marker: matching marker/after-values yields `applied_observed` (observed desired
state, not proof which actor wrote it). A newer marker/different values is a
conflict. Unchanged before-values do not prove a timed-out request cannot finish
later; keep the item unknown unless a provider receipt establishes non-execution.
Manual resolution records actor/reason and requires a new preview before another
write. Do not mark an item successful solely from a 2xx without its read-back result.

A documented rejection before execution (for example a confirmed 429) may retry
with `Retry-After` and jitter. Initial limits are two concurrent calls per portal
and five requests/second, reduced to current provider entitlements if lower. A 401
allows one serialized refresh; revoked scope/refresh failure pauses the connection.
No automatic retry of ambiguous 5xx/timeout writes. Batch progress distinguishes
applied, applied-observed, conflict, blocked, unknown and pending items; successful
items never replay when another item fails.

### Implementation and acceptance gates

Deploy cipher/connection schema, then read-only previews, then property setup and
small sandbox applies. Keep CRM feature/admission flags off by default. Existing
research/export paths do not need OAuth credentials. On rollback disable apply and
continue read-back reconciliation and token revocation. Do not "undo" CRM values
without a newly approved current-state preview.

Required integration fixtures: wrong-tenant OAuth callback; consumed/expired state;
wrong portal; encrypted bundle copied between tenants; concurrent refresh; secret
rotation; deleted remote contact; multiple email matches; client mismatch; native
property injection; changed remote field after preview; suppression before patch;
429 with `Retry-After`; patch applied then timeout; patch still pending after a
read; read-back mismatch; scope revocation halfway through a batch. Sandbox tests
must validate actual property types and provider concurrency semantics. Record the
provider API version, test date, app scopes and response fixtures in the adapter's
contract test documentation before enabling a design partner.

Measure apply latency, conflicts, unknown outcomes and their age, rate-limit wait,
refresh failures, reconciliation backlog, and customer merge minutes saved. Never
log raw property values, OAuth secrets or email addresses as metric labels.
