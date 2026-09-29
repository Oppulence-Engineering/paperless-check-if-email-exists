# Decision Policies

Decision policies turn raw verification output into an action customers can use directly in sending, signup, enrichment, and list-cleaning workflows.

Every newly prepared verification result can include:

- `recommendation`: the row-level action Reacher recommends.
- `policy_evaluation`: the policy decision and reasons that produced the recommendation.

Historical rows created before this capability may have these fields set to `null`.

## Recommendation Actions

| Action | Meaning |
|---|---|
| `send` | The address is acceptable for the selected policy. |
| `send_with_caution` | The address can be used, but it has elevated risk such as medium bounce risk. |
| `review` | The address is ambiguous and should be manually reviewed before automated use. |
| `suppress` | The address should be suppressed before use, usually because of spam-trap, disposable, complaint, unsubscribe, or previous bounce signals. |
| `drop` | The address should be excluded from this workflow, usually because it is invalid or already suppressed. |
| `fix_then_send` | Reacher found a safe correction, such as a likely domain typo, before use. |

Each recommendation includes:

- `action`
- `policy_mode`
- `policy_profile_key`
- `confidence`
- `priority`
- `summary`
- `reasons`
- `suggested_email`
- `engine_version`
- `evaluated_at`

Example:

```json
{
  "recommendation": {
    "action": "fix_then_send",
    "policy_mode": "deliverability",
    "confidence": "medium",
    "priority": "high",
    "summary": "Email has a safe correction available before use.",
    "suggested_email": "alex@gmail.com",
    "engine_version": "decision_v1",
    "evaluated_at": "2026-06-30T12:00:00Z",
    "reasons": [
      {
        "code": "possible_domain_typo",
        "severity": "warning",
        "message": "Email has a possible domain typo with a suggested correction.",
        "evidence": {
          "suggested_email": "alex@gmail.com",
          "score": 74
        }
      }
    ]
  }
}
```

## Policy Modes

Use `policy_mode` on check-email requests when the same verification result should be interpreted for a specific workflow.

| Mode | Intended use |
|---|---|
| `growth` | More permissive. Sends valid and most risky rows unless there are dangerous signals. |
| `deliverability` | Default. Protects sender reputation and reviews stale, risky, or high-bounce-risk rows. |
| `signup_protection` | Blocks risky signups and suppresses disposable or high-risk domains. |
| `enterprise_strict` | Sends only clean, fresh, individual recipients with low bounce risk. |
| `custom` | Evaluates a tenant-defined rule profile selected by `policy_profile_key`. |

`/v1/me/settings` exposes `default_policy_mode`, which defaults to `deliverability`.

## Custom Policy Rules

Custom policies are constrained JSON rules stored in `tenant_policy_profiles.rules`.

Allowed fields:

- `score`
- `category`
- `safe_to_send`
- `is_disposable`
- `is_role_account`
- `is_catch_all`
- `bounce_risk_category`
- `active_suppression`

Allowed operators:

- `eq`
- `neq`
- `lt`
- `lte`
- `gt`
- `gte`

Example rule shape:

```json
{
  "default_decision": "review",
  "rules": [
    {
      "field": "score",
      "operator": "gte",
      "value": 90,
      "decision": "send",
      "reason_code": "high_score"
    },
    {
      "field": "is_disposable",
      "operator": "eq",
      "value": true,
      "decision": "suppress",
      "reason_code": "disposable_provider"
    }
  ]
}
```

Use `policy_mode=custom` together with `policy_profile_key`.

## Explainable Reasons

Reasons are stable, customer-facing codes. They can include scalar evidence such as score, category, booleans, freshness age, bounce-risk category, previous outcome type, or suggested email.

Important reason codes include:

- `catch_all_corporate_domain`
- `possible_domain_typo`
- `stale_verification`
- `role_account`
- `disposable_provider`
- `weak_mail_infrastructure`
- `previously_bounced_for_tenant`
- `active_suppression`

## Where Decisions Appear

Decision metadata is returned or persisted on:

- `/v0/check_email`
- `/v1/check_email`
- bulk job rows
- list rows and list downloads
- job results and job downloads
- email history
- query results
- CSV and NDJSON exports
