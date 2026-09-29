# Push Pipelines

Push pipelines let a CRM, lead list, workflow engine, or application send an audience directly into Reacher without first creating a Reacher list.

Create a pipeline whose source has `type: "push"`, then submit up to 10,000 rows:

```http
POST /v1/pipelines/{pipeline_id}/push
Idempotency-Key: paper-list-44-2026-07-17
```

```json
{
  "source_key": "paper:list:44",
  "email_column": "email",
  "rows": [
    {
      "email": "alex@example.com",
      "lead_id": "99"
    }
  ]
}
```

The response contains the durable batch and pipeline run identifiers. Reusing an idempotency key for the same pipeline returns the original run instead of starting duplicate work. Push pipelines are event-driven and are not executed by the schedule runner.

Additional row fields are retained through the pipeline so callers can correlate results with their own lead, list, campaign, or workflow identifiers.
