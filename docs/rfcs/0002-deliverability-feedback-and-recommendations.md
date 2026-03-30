# RFC 0002: Deliverability Feedback and Recommendations

## Status

Draft

## Summary

Add a deterministic `recommendation` object to result-bearing APIs so callers get an actionable decision layer on top of the existing score, `safe_to_send`, reason codes, and bounce-risk assessment. This remains heuristic and rule-driven; it does not introduce a new machine-learning system.

## Motivation / Problem

- Reacher already returns rich scoring data, but callers still have to translate it into operational next steps.
- `safe_to_send` is intentionally strict and boolean; it does not explain intermediate actions like “review manually” or “send with caution”.
- Approval checklist logic exists at the job level, but there is no consistent per-email recommendation surface.

## Goals

- Produce a deterministic recommendation for every verification result.
- Reuse existing scoring, bounce-risk, freshness, and suppression signals.
- Keep all changes additive and backward-compatible.
- Make recommendations available in single-check, bulk, history, query, and download surfaces.

## Non-Goals

- Training or shipping a new ML classifier.
- Replacing the existing `score`, `safe_to_send`, or `bounce_risk` fields.
- Adding a UI workflow or human-approval system.

## Current State In Reacher

- The current response model already includes `score`, `category`, `sub_reason`, `safe_to_send`, `reason_codes`, and `signals`.
- Bounce-risk assessment already exists as a separate heuristic layer.
- Job approval and list quality endpoints return aggregate recommendations, not row-level decisions.

## Proposed Design

Introduce `RecommendationEngineV1`, a rule engine evaluated after scoring and bounce-risk assessment.

### Evaluation Model

The recommendation engine executes after the current score has been computed and, when available, after bounce-risk assessment has been attached. It never mutates the existing score payload.

The engine returns:

- one top-level recommendation action
- one summary string
- one confidence level
- one priority level
- an ordered list of reason objects
- the engine version used for evaluation

Recommendation actions are:

- `send`
- `send_with_caution`
- `review`
- `suppress`
- `drop`

Priority values are:

- `low`
- `medium`
- `high`
- `blocking`

Confidence values are:

- `high`
- `medium`
- `low`

### Reason Catalog

The first version of the engine uses a fixed reason catalog. A single recommendation may carry multiple reasons, but ordering is stable and derived from severity.

Initial reason codes are:

- `hard_invalid_syntax`
- `hard_invalid_recipient`
- `hard_provider_rejection`
- `hard_disabled_mailbox`
- `active_suppression`
- `safe_to_send_clean`
- `bounce_risk_medium`
- `bounce_risk_high`
- `stale_result`
- `catch_all`
- `role_account`
- `disposable`
- `spam_trap`
- `full_inbox`
- `unknown_smtp`
- `recently_verified`

Evaluation inputs, in order of authority, are:

1. Hard invalid conditions such as invalid syntax, invalid recipient, disabled mailbox, or provider rejection.
2. Active suppression state.
3. `safe_to_send`.
4. Bounce-risk category and recommended action.
5. Freshness age and result staleness.
6. Risk flags such as disposable, catch-all, role account, spam trap, and full inbox.

The engine is short-circuiting only for hard-invalid states and active suppressions. All other conditions accumulate explanations before the final action is chosen.

The engine uses the following default mapping:

- Hard invalid or active suppression -> `drop` with `blocking` priority.
- `safe_to_send = true` and bounce-risk `safe`/`low` -> `send`.
- `safe_to_send = true` and bounce-risk `medium` -> `send_with_caution`.
- `safe_to_send = false` but not hard invalid -> `review`.
- Spam-trap or disposable with invalid/risky score -> `suppress`.

Every recommendation includes ordered explanation objects with:

- `code`
- `message`
- `evidence`

The `evidence` object is intentionally compact and limited to scalar facts already present in the response payload, such as:

- `score`
- `score_category`
- `safe_to_send`
- `bounce_risk_category`
- `result_age_days`
- `is_disposable`
- `is_role_account`
- `is_catch_all`

### Default Decision Matrix

The decision matrix in v1 is:

| Primary condition | Recommendation |
|---|---|
| Hard invalid or active suppression | `drop` |
| `safe_to_send = true` and bounce-risk `safe`/`low` | `send` |
| `safe_to_send = true` and bounce-risk `medium` | `send_with_caution` |
| `safe_to_send = false` and category `risky` or `unknown` | `review` |
| Spam-trap or disposable with invalid or risky score | `suppress` |
| Fresh result with only role/catch-all concerns | `review` |

When multiple rules match, the engine chooses the action with the highest severity in this order:

`drop` > `suppress` > `review` > `send_with_caution` > `send`

## Public API / Interface Changes

Add `recommendation` to result-bearing responses from:

- `POST /v1/check_email`
- `POST /v0/check_email`
- `GET /v1/bulk/{job_id}/results`
- `GET /v1/emails/{email}/history`
- `GET /v1/query`
- job and list download payloads

Recommendation response example:

```json
{
  "recommendation": {
    "action": "review",
    "priority": "high",
    "confidence": "medium",
    "summary": "Result is not safe to send because the mailbox is catch-all and the bounce-risk assessment is elevated.",
    "reasons": [
      {
        "code": "catch_all",
        "message": "The recipient domain is configured as catch-all.",
        "evidence": {
          "is_catch_all": true
        }
      },
      {
        "code": "bounce_risk_medium",
        "message": "Bounce-risk assessment recommends manual review.",
        "evidence": {
          "bounce_risk_category": "medium"
        }
      }
    ],
    "engine_version": "recommendation_v1",
    "evaluated_at": "2026-03-30T00:00:00Z"
  }
}
```

`recommendation` shape:

- `action`
- `priority`
- `confidence`
- `summary`
- `reasons`
- `evaluated_at`

Add download columns:

- `recommended_action`
- `recommendation_priority`
- `recommendation_confidence`
- `recommendation_reasons`

API compatibility rule:

- If `recommendation` is not yet backfilled for a historical row, the response must include either the computed `recommendation` or `null`. The field must never be omitted once this RFC ships.

## Data Model / Storage Changes

- Add `recommendation JSONB` to `v1_task_result`.
- Persist the computed recommendation with the result so downloads, history, and query surfaces remain stable over time.
- Do not create a separate recommendation table in the first version.

Stored JSON shape:

- `action`
- `priority`
- `confidence`
- `summary`
- `reasons`
- `engine_version`
- `evaluated_at`

Indexing decisions:

- no standalone index on the JSON payload in v1
- recommendation fields are read as part of existing result retrieval paths
- if filtering by recommendation becomes common, a later RFC can add generated columns

## Auth, Permissions, And Tenant Isolation

- Recommendation visibility follows the existing permissions of the underlying result-bearing endpoint.
- No new scope is required.
- Recommendations may only reference suppression state inside the same tenant.

## Failure Modes And Edge Cases

- If bounce-risk assessment is unavailable, recommendation falls back to score-based rules and sets `confidence = low`.
- If freshness metadata is unavailable, the engine does not apply stale-result penalties.
- If a row is stored before recommendation evaluation fails, Reacher will return the underlying result with `recommendation = null` and log the evaluation failure; this should be treated as a bug, not a hard request failure.

Additional edge rules:

- If suppression lookup fails, recommendation evaluation degrades to the non-suppression inputs and sets `confidence = low`.
- Historical rows older than the freshness horizon may receive `review` even if their original raw score is strong.
- A recommendation may disagree with `safe_to_send`; that disagreement is expected and should be explainable via the reason list.

## Alternatives Considered

- Reusing `safe_to_send` as the only recommendation. Rejected because it collapses too many operational states into a boolean.
- A separate recommendation endpoint. Rejected because callers need the decision in the same payload as the result.
- A learned classifier. Rejected because existing signals are already strong enough for a first deterministic version.

## Rollout / Migration / Compatibility

- Add the field additively and keep it nullable during migration.
- Backfill recommendations for historical rows lazily when they are read or downloaded.
- Preserve all existing scoring semantics, including `safe_to_send`.

Implementation phases:

1. Engine and persistence
2. Single-check response surfaces
3. Bulk/history/query/download surfaces
4. Lazy backfill for historical rows

## Metrics And Success Criteria

- Percentage of result payloads with non-null recommendations.
- Distribution of recommendation actions by category.
- Agreement and disagreement rate between `safe_to_send` and `recommendation.action`.
- Reduction in manual downstream rule-building in customer integrations.

## Test Strategy

- Golden-path recommendation fixtures for valid, risky, unknown, invalid, and suppressed addresses.
- Bounce-risk interaction tests.
- Freshness degradation tests.
- CSV/download rendering tests.
- History/query response tests to verify persisted recommendations survive replay.

The minimum suite should also include:

- explicit disagreement tests between `safe_to_send` and recommendation action
- null-bounce-risk fallback tests
- suppression-lookup fallback tests
- engine-version persistence tests

## Unresolved Risks

- Recommendation language can be read as a guarantee if the summaries are too strong.
- A persisted recommendation can age poorly if business rules change faster than rows are recomputed.
