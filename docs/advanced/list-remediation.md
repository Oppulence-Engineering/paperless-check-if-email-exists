# List Remediation

List remediation turns a completed list verification into durable cleaned outputs. It is designed for customers who want files they can send, review, suppress, or import downstream without building their own row classifier.

## Create A Plan

```http
POST /v1/lists/{list_id}/remediation-plan
```

Request options:

```json
{
  "allow_partial": false,
  "apply_domain_typos": true,
  "normalize_emails": true,
  "deduplicate": true,
  "drop_suppressed": true
}
```

By default, Reacher only creates a plan after the list has completed. Set `allow_partial=true` when an in-progress list should be remediated from the rows that have already finished.

The response includes immutable plan metadata, summary counts, and up to 100 preview rows.

## Classifications

| Classification | Meaning |
|---|---|
| `fixed` | Reacher changed the row and the corrected row is usable. |
| `safe` | The row is safe without changes. |
| `review` | The row should be inspected before use. |
| `drop` | The row should be excluded from cleaned output. |

Automatic fixes include trimming whitespace, lowercasing the domain portion, removing zero-width/control characters, applying clear domain typo corrections, and using safe canonical forms such as Gmail dot or plus normalization.

## Read The Latest Plan

```http
GET /v1/lists/{list_id}/remediation-plan
```

This returns the newest matching plan for the list.

## Create An Export

```http
POST /v1/lists/{list_id}/remediation-exports
```

Example:

```json
{
  "plan_id": 123,
  "partitions": ["safe_to_send"],
  "format": "csv"
}
```

Supported partitions:

- `safe_to_send`: fixed plus safe rows
- `fixed`
- `safe`
- `review`
- `drop`
- `changed`
- `all`

If no partition is supplied, Reacher exports `safe_to_send`.

## Download An Export

```http
GET /v1/lists/{list_id}/remediation-exports/{export_id}/download
```

Exports stream as CSV in 500-row batches. The original columns are preserved and Reacher appends:

- `remediation_classification`
- `remediation_rule_id`
- `remediation_confidence`
- `original_email`
- `effective_email`
- `remediation_changed`
- `remediation_reasons`
