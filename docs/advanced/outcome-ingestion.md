# Outcome Ingestion

Outcome ingestion lets customers close the loop between verification and real campaign or product events. Bounces, complaints, and unsubscribes can automatically feed suppression intelligence, while all outcomes improve source-quality reporting.

## Ingest Outcomes

```http
POST /v1/outcomes
```

Example:

```json
{
  "provider": "hubspot",
  "source_key": "apollo-import",
  "outcomes": [
    {
      "email": "alex@example.com",
      "event_type": "bounced",
      "campaign_id": "campaign-123",
      "occurred_at": "2026-06-30T12:00:00Z",
      "metadata": {
        "provider_event_id": "evt_123"
      }
    }
  ]
}
```

Supported event types:

- `bounced`
- `delivered`
- `opened`
- `clicked`
- `complained`
- `unsubscribed`

The request can contain up to 10,000 outcomes.

## Response

```json
{
  "ingested": 1,
  "auto_suppressed": 1,
  "ignored": 0
}
```

Rows with blank or invalid normalized emails are ignored. Invalid event types return `400`.

## Suppression Side Effects

These events create or refresh active suppression entries:

| Event | Suppression reason |
|---|---|
| `bounced` | `bounce` |
| `complained` | `complaint` |
| `unsubscribed` | `unsubscribe` |

Suppression metadata includes:

- `outcome_provider`
- `outcome_id`
- `auto_suppressed_by_policy`
- any metadata supplied in the outcome row

Delivered, opened, and clicked outcomes are stored for analytics but do not suppress the email.

## Source Quality

Set `source_key` either on the top-level request or per outcome row. Per-row `source_key` wins. Outcomes with a source key are included in `/v1/sources/quality`.
