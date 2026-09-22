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
      "provider_event_id": "evt_123",
      "provider_message_id": "msg_456",
      "campaign_id": "campaign-123",
      "occurred_at": "2026-06-30T12:00:00Z",
      "metadata": {}
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

Rows with blank emails are ignored. Invalid event types return `400`. Supply a stable `provider_event_id` whenever the sender provides one; Reacher uses it to make provider retries idempotent.

## Native Provider Webhooks

Reacher can authenticate and normalize SendGrid, Amazon SES/SNS, Mailgun, and Postmark webhooks. Manage endpoints with:

- `GET /v1/provider-endpoints`
- `POST /v1/provider-endpoints`
- `PATCH /v1/provider-endpoints/{endpoint_id}`
- `DELETE /v1/provider-endpoints/{endpoint_id}`

Creating an endpoint returns its delivery token once. Configure the provider to send events to the returned `webhook_path`. Endpoint management requires the tenant `settings` scope.

Provider settings belong in `provider_config.settings`:

| Provider | Required settings |
|---|---|
| SendGrid | `public_key_pem`; optional `verification_timestamp_tolerance_seconds` |
| Amazon SES | `topic_arns` containing the allowed SNS topic ARNs |
| Mailgun | `signing_key`; optional `verification_timestamp_tolerance_seconds` |
| Postmark | No signing setting; use the delivery token and configure `allowed_ips` when possible |

Each request is recorded as a durable receipt before normalization. Provider event and message IDs are retained, duplicate events are ignored, and bounce, complaint, and unsubscribe events update suppressions automatically.

An endpoint may also forward normalized outcomes to another service. Configure `outcome_webhook_url`, optional `outcome_webhook_headers`, and optional `outcome_webhook_signing_secret` in its settings. Reacher blocks private or unsafe callback targets, does not follow redirects, signs the exact JSON body, and retains failed deliveries on the receipt for recovery.

Use `GET /v1/outcomes` to inspect normalized outcomes. It accepts `limit`, `offset`, `email`, `event_type`, `source_key`, and `since` filters.

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
