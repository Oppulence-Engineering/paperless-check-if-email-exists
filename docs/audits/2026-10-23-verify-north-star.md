# Verify codebase audit against the October 2026 thesis

Date: 2026-10-23. Source revision: `96413a6876c12515c146f75f940dd17ea3470600`.
Scope: this repository's product UI, Rust research gateway and exports, verification,
auth, list/suppression/outcome primitives, migrations, contracts, benchmark tooling,
and all 26 existing RFC dispositions. This is a product/architecture audit, not a
complete security assessment.

## Conclusion

The repository contains a working foundation for a bounded company-to-contact
pilot and substantial email-quality infrastructure. It does not yet contain the
complete agency audit described in the [Verify thesis](../strategy/verify-business-thesis-2026-10.md).
The next work should make a customer-owned list audit traceable, reviewable, and
measurable before expanding discovery or sender analytics.

The separate Go repository and live deployments were not available for inspection
in this audit. Statements about Go internals and the October 20 production release
are attributed to the [integration record](../paperless-research-integration.md),
the supplied thesis, and its linked release PR. They are not new production checks.
The integration record contains both release evidence and older unfinished
checklists; neither is treated as proof of current operational health.

## Implemented foundation and limits

| Area | Evidence inspected | What it establishes / limit |
|---|---|---|
| Tenant authority | [Research bridge](../../backend/src/research.rs), [auth middleware](../../backend/src/http/mod.rs), [BFF](../../web/app/api/backend/[...path]/route.ts) | Tenant-to-Go mapping and credentials stay server-side; public authority is overwritten. This does not audit Go storage or its internal graph. |
| Research import and execution | [Research API](../../backend/src/http/v1/research.rs), `CreateResearchRequest`, `create_in_go` | CSV/JSON, target role, retry key, ten rows, 1–3 candidates/company, 1–30 verification requests. Adding the contact column uses `runImmediately: true`; the connected flow has no separate priced-plan approval step. |
| Review UI | [Research panel](../../web/app/(product)/app/research/_components/research-panel/research-panel.tsx) | Progress, sources/dates, separate person/employer/role/mailbox evidence, row selection, and downloads. Selection is React state; there is no durable approve/hold/reject/request-research mutation. |
| Eligibility and export | [Research API](../../backend/src/http/v1/research.rs), `ready_candidate`, `export`, `current_policy_allows` | Conservative published-person/title/address gate, MillionVerifier `ok` + `send`, no catch-all; fresh verification history, suppression, negative feedback, and policy checks at export. Only `approved`/`review` modes; first eligible candidate per selected company row in approved mode. |
| Managed verification | [Verification adapter](../../backend/src/verification.rs), [operation admission](../../backend/src/http/shared/verification_operation.rs), [provider-attempt migration](../../backend/migrations/20261020000006_verification_provider_attempts.up.sql) | Provider evidence and durable verification operations exist. SMTP-based upstream engine usage is a separate configuration, not the managed research contract. |
| List repair | [Upload](../../backend/src/http/v1/lists/post.rs), [remediation](../../backend/src/http/v1/lists/remediation.rs) | Email-list originals, deduplication, persistent plans, and fixed/safe/review/drop partitions already exist. These do not supply contact identity review or automatically join research sessions to source rows. |
| Decisions and suppressions | [Decision types](../../backend/src/decision/types.rs), [suppression](../../backend/src/suppression.rs), [events API](../../backend/src/http/v1/suppressions/events.rs) | Recommendations, policy modes, expiry/provenance/events, and tenant-wide suppression exist. The original RFC 0026 strict/balanced/aggressive names are not the current API enum. |
| Outcomes and visibility | [Provider adapters](../../backend/src/http/v1/provider_outcomes.rs), [source quality](../../backend/src/http/v1/source_quality.rs), [job events](../../backend/src/http/v1/jobs/get_events.rs), [idempotency](../../backend/src/http/idempotency.rs) | Inbound adapters, analytics, job events/retries, and replay primitives exist; broad original RFC acceptance criteria still need individual reconciliation. |
| Maintenance | [Reverification settings](../../backend/src/http/v1/reverification/update.rs), [list diff](../../backend/src/http/v1/lists/diff.rs), [pipelines](../../backend/src/pipelines.rs) | Workspace mailbox freshness scheduling and list differences exist. This is not current-employment monitoring or recurring contact repair. |
| Billing | [Plan definitions](../../web/lib/billing/plans.ts), [auth integration](../../web/lib/auth/auth.ts) | Existing verification plans are Free, Starter ($49), Professional ($149), Enterprise. The proposed $249 audit and $149–$299 recurring offer are not configured audit products. |
| Evaluation | [Benchmark script](../../scripts/benchmark_contacts.py), [protocol](../benchmarks/contact-research/README.md) | Equal account cohorts, independent match flags, provider cost, and cleanup minutes are supported. Data is synthetic; labor/infrastructure are excluded and deduplication is an input responsibility. No measured commercial advantage follows. |

## Gaps in delivery order

| Priority | Finding and consequence | Next specification |
|---|---|---|
| P0 before new source use | The local bridge proves an authority pattern, not provider licensing, cross-repository deletion, or separation from the finance graph. Record and enforce those boundaries before expanding data use. | [0001](../rfcs/0001-data-rights-and-product-boundaries.md) |
| P0 before commercial claims | No full-cost, independently adjudicated outcome ledger or paid/repeat-purchase cohort exists in the inspected workflow. The benchmark's `cost_per_usable_contact_usd` is provider-only. | [0002](../rfcs/0002-paid-audit-validation-and-unit-economics.md) |
| P1 first complete audit | Research has a run name but no typed client/campaign/audit resource or public field-mapping preview. Research CSV headers omit `row_key`, customer original IDs, original values, and change history. Customers cannot reliably merge corrections back to their source list. | [0003](../rfcs/0003-agency-audits-and-import-lineage.md), [0004](../rfcs/0004-evidence-review-and-audit-delivery.md) |
| P1 first complete audit | Evidence is compressed into status strings and one source/date per candidate. Local `ready_candidate` checks those strings but does not independently confirm employment or require a fresh claim observation date. There is no claim-level contradiction history, durable reviewer decision, selected candidate record, or complete Ready/Needs Review/Excluded delivery. | [0004](../rfcs/0004-evidence-review-and-audit-delivery.md) |
| P1 before paid repair/scale | The public contract has verification-count caps, not a dollar quote, customer-approved charge ceiling, measured research cost, selective row repair, or a paginated large-run lifecycle. Raising a constant is insufficient. | [0005](../rfcs/0005-budgeted-selective-repair-and-batch-execution.md) |
| P2 with client-scope demand | Suppressions are tenant-wide. Client labels must not imply client ACL isolation or scoped do-not-contact behavior. Use separate workspaces when isolation is required until explicit scopes ship. | [0011](../rfcs/0011-suppression-scopes.md) after 0003/0004 |
| P2 after requested repeat workflow | No inspected research route performs HubSpot preview/writeback; workspace mailbox rechecks do not detect role changes. | [0006](../rfcs/0006-preview-first-hubspot-writeback.md), [0007](../rfcs/0007-recurring-contact-quality.md) |

## RFC reconciliation

- RFCs 0010, 0014–0016, 0026–0031 are not ten untouched greenfield projects. Implemented primitives
  overlap most of them. Preserve API compatibility and finish observed gaps rather
  than rebuild from their historical “Current State” sections.
- RFCs 0008–0009, 0018, 0022, 0032 already contain October 20 architecture corrections. Go owns
  research; a second Rust crawler/global store and compulsory Parallel sourcing
  are superseded. Separate narrow repair/enrichment from broad discovery.
- RFC 0033's old build order is stale. In particular, `/v1/sources/quality`
  already maps to the `bulk` scope in `required_route_scope`, enforced through
  `resolve_tenant`. The absence of a handler-local `check_scope` is not an open
  authorization gap. Do not re-list that task as a launch blocker.
- The MillionVerifier adapter still detects provider from the domain before MX
  lookup and does not repeat provider detection afterward. RFC 0017's attribution
  slice remains relevant, but neither provider reports nor MCP are prerequisites
  for proving the audit business.
- RFCs 0011–0013, 0017, 0019–0021, 0023–0025 remain optional delivery/integration extensions with narrower
  demand gates. RFCs 0001–0007 cover the missing contact-audit path.

The [RFC index](../rfcs/README.md) records every existing and new RFC, its
disposition, dependencies, and the stage where it contributes.

## Verification performed

Before documentation edits:

- `python3 -m unittest discover -s scripts -p test_benchmark_contacts.py -v`:
  **3 passed** (missing costs, comparable cohorts, mailbox vs usable-contact distinction).
- `python3 scripts/benchmark_contacts.py docs/benchmarks/contact-research/synthetic.csv`:
  completed; output explicitly identifies synthetic cohorts and excludes
  unrecorded labor/infrastructure.
- Focused Vitest research-panel and schema suites: **2 files, 6 tests passed**.
- Read [Rust research integration tests](../../backend/tests/e2e_research.rs),
  including retry reuse, tenant refusal, formula-safe CSV, and current-policy
  exclusions. They use a stub Go upstream; they do not establish live Go behavior.

Rust tests were not rerun: `cargo` is unavailable on this environment's PATH.
No live providers, billing, CRM, production endpoints, or customer data were used.
This change updates documentation only; it does not implement the identified gaps.

Documentation validation after edits: all 33 RFCs are listed in the delivery index
and documentation navigation; relative file links and local heading anchors were
checked, and `git diff --check` passed. That original audit edit preserved RFC IDs
and baseline anchors; no application source or billing configuration changed then.
On 2026-10-24 the RFCs were renumbered by priority, and references in this audit were
updated using the [former-to-current mapping](../rfcs/renumbering.md). The source
findings and validation results above still describe the original audit date.
