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

## Technical specification

Apply [Verify technical contracts v1](verify-technical-contracts.md). All monetary
fields below are USD micro-units serialized as decimal strings. The plan is an
immutable quote, not an estimate that a worker may increase after approval.

### Plan and reservation schema

| Rust table | Required fields and constraints |
|---|---|
| `audit_repair_plans` | Tenant/plan UUID, audit FK, version, generation, criteria/source/price versions, selected input/claim snapshot hash, actions JSON, candidate/request/verification caps, maximum customer charge, internal maximum provider cost, expiry, state, approval actor/time. Positive limits; unique `(tenant_id,id,version)`. |
| `audit_budget_accounts` | Tenant/currency/UTC month PK, customer charge limit, reserved and settled amounts, version. Nonnegative values; `reserved + settled <= limit`. Shared by manual repair and recurring work. |
| `audit_budget_reservations` | Tenant/reservation UUID PK, budget account FK, plan/version, operation UUID, ceiling, reserved balance, settled total, state, last Go event sequence. Unique operation FK and unique approved plan/version. |
| `audit_charge_events` | Tenant/event UUID PK, reservation FK, source service/operation/sequence, kind (`settlement`,`refund`,`release`), amount, provider reference, occurred/received times. Unique source event identity; positive entries. |
| `audit_repair_steps` | Tenant/step UUID, operation/input/claim IDs, action/source policy/version, state, estimated ceiling, Go step ID or Rust verification operation ID, stop reason. Composite audit/input FKs; unique `(tenant_id,operation_id,input_row_id,claim_kind,step_ordinal)`. |

Create budget accounts only from an authorized limit configuration. Increasing a
limit does not change existing quote ceilings or authorize a new run. No SQL path
may lower a limit below committed plus reserved spend. Persist the accounting UTC
month at admission; crossing midnight/month-end never moves a reservation to a new
period. Unknown costs remain reserved in the original account. RFC 0033 uses these
same rows and locks, not an independent maintenance balance.

Approval transaction locks audit, plan and budget account in a consistent order,
validates all bound versions and expiry, then performs the equivalent of:

```sql
UPDATE audit_budget_accounts
SET reserved_micros = reserved_micros + $1,
    version = version + 1
WHERE tenant_id = $2 AND currency = 'USD' AND period_start = $3
  AND $1 >= 0
  AND $1 <= limit_micros - settled_micros - reserved_micros
RETURNING version;
```

No returned row means no admission (`409 budget_exhausted`). In the same transaction
insert the reservation, audit operation, dispatch intent and cached response. A
unique plan/version reservation prevents two different request keys purchasing the
same approved plan. A repeated approval returns its existing operation. Never hold
the database lock while making an HTTP call. Checked integer arithmetic must reject
overflow before SQL/price evaluation.

### Public plan/approval contract

```http
POST /v1/audits/{audit_id}/repair-plans
Idempotency-Key: <UUID>
```

```json
{
  "expected_version": "8",
  "input_row_ids": ["40000000-0000-4000-8000-000000000001"],
  "claim_kinds": ["employment", "role_relevance"],
  "limits": {
    "currency": "USD",
    "max_customer_charge_micros": "2000000",
    "max_candidates_per_row": 3,
    "max_verifications": 3
  }
}
```

A 201 response includes plan ID/version/hash, the exact row/claim scope, permissible
actions, maximum customer charge, price version, expiry (initially 15 minutes),
and limitations. Return internal cost fields only to internal finance/operations
authority; customers see their charge ceiling and service scope. Empty selection,
duplicate row IDs, wrong-audit IDs, unsupported claims or missing approved source
rights reject the plan. Planning reads cached evidence and tariffs and makes zero
paid calls. An unknown or expired provider tariff prevents that paid action being
quoted; it does not imply a zero cost.

`POST /v1/audits/{id}/repair-plans/{plan_id}/approve` requires spend scope and
`{"expected_version":"1","plan_hash":"<sha256>","max_customer_charge_micros":"2000000"}`.
It returns the durable 202 operation. The approved amount must equal the quoted
ceiling; changing it requires a new plan. Free/trial quotes still require an explicit
run approval and bounded provider cost; a zero customer charge is not unlimited
provider authorization. `GET .../repair-plans/{plan_id}` is read-only.

`POST /v1/audits/{id}/operations/{operation_id}/cancel` requires spend scope,
expected operation version and an idempotency key. Return 202 while in-flight
attempts settle. Progress reads expose input/claim totals, terminal/pending counts,
customer reserved/settled amounts, unresolved outcome count and per-row stop codes.
Do not report completed while a paid attempt is still unknown.

### Two-service admission without a distributed transaction

Rust owns the full customer-charge escrow. Go owns the provider-attempt ledger
and reserves against the run's delegated charge and provider-cost ceilings. Go
cannot spend against the customer's unreserved remaining account balance.

1. Rust commits the approval/reservation/operation/outbox transaction above.
2. Worker calls proposed `POST .../internal/verifier-repair-runs` with contract
   version, operation UUID, plan hash, tenant/workspace binding, source grants,
   exact caps, tariff version and reservation/authorization reference.
3. Go checks Rust's authenticated authorization lookup, including cancellation,
   deletion and source rights. Go transactionally stores the operation ID/hash,
   delegated caps and steps before dispatching any paid action. Same ID/body
   replays; different body conflicts. Missing authorization fails closed.
4. Rust records Go admission using its current lease token. On timeout or lease
   loss, reconcile `GET .../internal/verifier-operations/{operation_id}`; do not
   mint another operation or release the escrow merely because the POST timed out.
5. Go exposes immutable, sequential ledger events through a paginated internal
   operation-events endpoint. Rust polls/checkpoints and applies each event once.
   A sequence gap triggers reconciliation, never speculative balance release.

Propose Rust `GET /internal/verify/repair-authorizations/{operation_id}` for the
trusted Go service. Return contract version, original tenant/workspace binding,
plan hash, authorization version, reservation ID, delegated provider/customer caps,
source-policy versions, cancellation/deletion state, and admission expiry. Missing,
expired, other-workspace or revoked authority rejects dispatch. The response is
server-to-server only; an opaque reservation UUID is not a bearer credential.
Go event pages contain `sequence`, `attempt_id`, `event_kind`, provider-cost and
customer-charge micros, currency, tariff version, receipt hash and occurrence time.
A `run_closed` event additionally states final sequence and ledger hash; Rust checks
these before releasing the remaining run reservation.

```mermaid
sequenceDiagram
    participant U as Authorized customer
    participant R as Rust API and ledger
    participant G as Go executor and ledger
    participant P as Provider
    U->>R: Approve exact plan and charge ceiling
    R->>R: Reserve budget + persist operation/outbox
    R-->>U: 202 with stable operation ID
    R->>G: Admit same operation and delegated ceilings
    G->>R: Validate current authorization
    G->>G: Persist run + reserve bounded attempt
    G->>P: Dispatch stable attempt
    alt Confirmed result and charge
        P-->>G: Result and receipt
        G-->>R: Ordered settlement event
        R->>R: Settle once; release confirmed unused amount
    else Ambiguous timeout
        G-->>R: Outcome unknown
        R->>R: Keep reservation; reconcile same operation
    end
```

All internal calls use the trusted bridge with server-validated workspace binding.
A lost acknowledgement is retried with the same operation. To cancel an apparently
unadmitted run, Go must persist a cancellation tombstone for that operation before
Rust can release its reservation; a delayed admission then sees the tombstone.
Disablement/deletion does not erase ledger entries or stop settlement. Quote expiry
blocks new admission, but accepted work may settle after expiry.

### Attempt ledger and charge arithmetic

Go needs durable `repair_runs`, `repair_steps`, `provider_attempts`, and
`repair_ledger_events` (logical contracts; final DDL belongs in the Go repository).
Each attempt has run/step IDs, stable attempt UUID, source/provider, provider
idempotency/operation reference if supported, tariff version, maximum provider cost,
maximum customer charge, state, timestamps, result hash and settlement reference.
Unique `(run_id,step_id,attempt_ordinal)` and provider idempotency reference prevent
repeated local admission. Mutating run balances and attempt reservation is one Go
transaction guarded by a run lock and a tenant/provider concurrency slot.

```text
planned -> reserved -> dispatched -> settled
                    |             -> outcome_unknown -> settled
                    -> cancelled
```

Dispatch requires remaining provider and delegated customer capacity for the full
worst-case attempt. For variable-priced providers, a contractually enforceable
request cap is required; otherwise do not enable the adapter. Compute charges using
versioned integer rational tariffs with ceiling rounding, e.g.
`charge = ceil(provider_micros * markup_numerator / markup_denominator) + fixed_fee`.
Reserve that worst case, including verification charges. The precise tariff is a
commercial configuration, not a hard-coded margin promised by this RFC.

An HTTP timeout, connection loss after send, or ambiguous 5xx is `outcome_unknown`.
It retains its full reservation. Retry a dispatched attempt only when the provider
supports an enforced idempotency key, or a provider receipt proves the earlier
attempt cannot execute/charge. Otherwise stop that step for reconciliation/manual
resolution; a different key is a new purchase needing available approved budget.
Pre-dispatch failures can release the unused attempt reservation. Failed/empty
results with confirmed charges settle as costs. Refunds reference settled events,
cannot exceed them, and do not create an automatic new purchase authorization.

Rust verification continues through existing durable verification operations and
`verification_provider_attempts`; link the stable operation ID and reuse its result.
Reserve its worst-case cost/charge in the repair quote before submission. Verification
quotas and audit currency reservations are distinct checks. The verification attempt
is recorded once in full-cost reporting, not once in each service. Reconciliation
joins by verification operation/dispatch ID and records actual provider expenditure
or explicit unknown cost; quota decrements are not expenditure receipts.

On settlement, atomically reduce reserved amount and increase settled amount by the
confirmed charge. Release only confirmed unused amounts. Final run close requires
all steps terminal, all provider outcomes settled, and matching ledger sequence/hash
between services. Cost exceeding a quoted provider bound is an adapter incident:
stop further dispatch, record actual cost, and cap the customer charge at approval.
The platform bears an unexpected overage; it must never hide the cost or debit beyond
authorization. Promotion gates require these exceptions to be zero in test cohorts.

### Selection algorithm and cancellation fences

For each unresolved claim, choose the first allowed action in a versioned policy:
reuse fresh permitted evidence → bounded first-party lookup → one licensed adapter
→ verification for a sufficiently supported candidate. Re-evaluate after each result.
Skip work if new evidence already satisfies the objective. Limits include attempts
per source/claim and candidates per row, not just successful returns. Stable step
ordinals make retries resume the same plan. A missing/ambiguous company domain
requires review before fetching an inferred company's people.

Cancellation increments an authorization version and stops new dispatch. Go checks
the current fence immediately before attempt reservation/dispatch; revoke cached
authorization on a cancellation signal and bound any lease to 60 seconds. For a
strict cancellation acknowledgement, Go closes dispatch admission under the run
lock and reports in-flight attempts; Rust does not label it cancelled before this
acknowledgement. Calls admitted before that fence can complete and cost money.
Never promise cross-service instantaneous cancellation. Keep accounting for them.

### Capacity, backpressure, and release gates

Paginate immutable input/candidate snapshots with the shared cursor contract. Workers
claim bounded batches (initially 25 steps), checkpoint each terminal step, and honor
per-tenant/provider limits. Initial provider concurrency is two calls per tenant;
overall audit-operation limits in the shared contract are ceilings, not allocations.
Use weighted round-robin eligible tenants; avoid one large audit monopolizing the
queue. Respect `Retry-After` for a confirmed rate-limit rejection. Apply exponential
backoff with jitter (1 second to 5 minutes) only where a retry is known safe.

Transport page bodies remain below the bridge's 8 MiB cap; target at most 1 MiB per
page. Stage 2 large exports use a private streaming/chunk adapter with a manifest,
checksummed chunks, bounded buffers and abort cleanup; reject enabled large-run
configuration if that adapter is missing. The BFF must stream without buffering the
whole archive. Do not put all results in a single operation response.

For each 10 → 100 → 400 account promotion, record worker CPU/RAM, database connection
pool, provider rate limits and queue settings. Run two concurrent tenants including
a small ten-row audit. Proposed gates: zero lost/duplicate input or ledger events;
zero overspend/duplicate purchases; reconciliation totals exactly equal; p95
non-provider API latency below 500 ms; healthy worker queue admission below 60 s;
peak memory below 70% of its configured limit; no unbounded DB-connection growth.
Measure separately provider processing time, admission delay and completion time.
These are release targets, not observed performance. A tiny DB pool must not be
"fixed" solely by increasing workers; tune bounded concurrency from the measurements.

| Fault/test | Required observation |
|---|---|
| Ten simultaneous $1 approvals against $3 remaining | Exactly three reserve; seven reject; no overspend. |
| Crash before/after each Rust/Go admission commit | One run, one reservation, recoverable mapping. |
| Timeout after provider accepts purchase | Unknown outcome keeps escrow; no new purchase key. |
| Two workers/expired lease process one step | One attempt admission; later worker reconciles. |
| Cancellation arrives before a delayed admission POST | Go tombstone rejects admission; safe release. |
| Duplicate/out-of-order settlement or excess refund | Deduplicate or reject; balances remain valid. |
| Suppression/revocation arrives during repair | No new permitted-use violation; output withheld. |
| Month rollover and concurrent recurring run | Shared period locks enforce both configured ceilings. |

Use PostgreSQL concurrency/failpoint tests, Go provider stubs that charge before
returning a timeout, and cross-service restart tests. Metrics include reserved and
unreconciled amounts, age of unknown outcomes, event sequence lag, retries by cause,
budget denials, queue age, cancellation acknowledgement delay and per-provider
attempt latency. Operator reconciliation requires a receipt and an append-only
reason/actor event; never edit balances directly to clear a stuck run.
