# RFC 0031: Budgeted Selective Repair and Batch Execution

| Field | Value |
|---|---|
| Status | Proposed; stage 2 |
| Updated | 2026-10-23 |
| Owners | Go execution lead, Rust billing/policy lead, web lead |
| Depends on | RFCs 0027, 0029, 0030; RFC 0028 cost definitions |
| Reuses | Existing Go research sessions/executors; Rust verification operations, quotas, and retry keys |
| Narrows | RFCs 0011/0014 repair; replaces RFC 0015's second runner/credit system |

## Problem and outcome

The connected pilot bounds counts but cannot quote a maximum dollar charge,
selectively rerun a subset with explicit approval, report complete research cost,
or support representative agency batches. Its ten-row limit, three-candidate
limit, thirty-verification limit, fixed page reads, and bounded gateway response
are deliberate safety constraints. A larger constant is not a scaling design.

Repair only the missing or questionable claims that matter to the customer's
target roles. Stop when the approved objective is satisfied, the budget is
exhausted, the customer cancels, or no permitted next step remains.

## Plan, authorize, execute

Propose `/v1/audits/{id}/repair-plans` for a selected set of input/claim IDs. A plan
contains evidence versions, intended sources/actions, expected limitations,
candidate/request caps, maximum provider cost, **maximum customer charge**, USD
currency, price/markup version, expiry, and stable plan ID. Internal cost and
customer charge are separate values. Use integer monetary units, never floats.

Planning and reviewing a plan make no paid calls. An authorized audit owner approves
the exact plan version through an idempotent approval endpoint; changed rows,
prices, sources, or increased ceilings require a new approval. The audit UI shows
the customer charge and scope, not internal service credentials or accounting IDs.

Rust owns customer authorization and the charge contract. Go owns the authoritative
research provider-attempt/cost ledger and scheduling. Rust verification operations
retain their existing quota/attempt authority; Go references them rather than
buying verification twice. Neither service assumes a distributed transaction.

Use stable operation IDs across the bridge and a replay-safe admission handshake.
Go durably records the authorization reference and reserves worst-case cost before
dispatch; transactionally enforce run and tenant caps under concurrent workers.
Never dispatch without confirmed reservation. Authorization lookup failure, unknown
price, or unavailable budget storage fails closed.

Ledger states distinguish reserved, running, settled, failed, refunded, and
unknown-provider-outcome. A timeout is not proof of no charge. Retain reservation
for unresolved outcomes and reconcile by provider operation ID before purchasing
again. Record unsuccessful calls and partial results. Commit actual charge and
release unused reservation only after settlement; customer spend never exceeds
the approved ceiling. Quota reservation is not measured provider expenditure.

## Selective research policy

Reuse permitted, fresh tenant evidence first; use bounded first-party sources when
sufficient; escalate to a licensed paid source only for unresolved claims within
the approved plan. Provide per-row stop reasons and remaining gaps. Name-only
company resolution and ambiguous domain matches need explicit evidence review.
Inferred addresses and catch-all results stay uncertain. No SMTP fallback on the
managed MillionVerifier path and no weakened identity gate to increase yield.

Full Domain Search or Discover from RFCs 0012/0013 is not required. Extract only the
source adapters/role enrichment that demonstrably improve the audit benchmark.

## Batch capacity and failure behavior

Reuse Go's durable row execution. Add cursor-based row/candidate pagination,
checkpointed batches, bounded concurrency, fair tenant scheduling, backpressure,
explicit cancellation, run progress, and streaming/chunked export delivery. Pending
cell work determines run completion; import completion alone does not.

Cancellation prevents new calls and reconciles in-flight calls; it cannot promise
to undo already purchased work. Retries resume unfinished operations with the same
keys. Keep failed/no-match rows in the original cohort and preserve their costs.

## Incremental rollout

1. Add plans and cost-ledger reconciliation at current 10/3/30 limits. With only
   permitted first-party work, still account for verification and human effort.
2. Enable one licensed repair source for selected pilot rows after budget-race and
   unknown-outcome tests pass. Use a per-source and per-tenant kill switch.
3. Validate 100-account batches, then a representative 400-account segment, before
   the approximately 1,200-account three-segment evaluation. These are capacity
   test targets, not promises or permission to bypass configured limits.
4. Promote tenant limits only after measured worker/DB/queue capacity, response
   sizes, completion times, and budget reconciliation meet a recorded release gate.
   Retain old defaults and a rollback path; do not increase every tenant at once.

## Acceptance

- Concurrent workers cannot overspend either charge or provider-cost ceiling.
- Duplicate approvals, bridge failures after admission, worker restarts, provider
  timeouts, and polling never create duplicate purchases or invisible charges.
- Cancellation, exhausted budget, no licensed source, and unresolved outcomes
  produce reviewable partial results with original row IDs and explicit costs.
- Test pagination without lost/duplicated rows, bounded memory, fair tenant
  progress, and export reconciliation at each capacity tier.
- Compare selective repair with the same baseline cohort using RFC 0028 full-cost,
  coverage, false-match, and review-time metrics. No automatic price/entitlement
  expansion follows merely from passing the load test.
