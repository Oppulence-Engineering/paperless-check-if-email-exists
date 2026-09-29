# Suppression Intelligence

Suppression entries now carry operational context, not just an email address. This makes suppression useful for compliance, sender reputation, list repair, and audit workflows.

## Add Or Refresh Suppressions

```http
POST /v1/suppressions
```

`POST /v1/suppressions/import` accepts the same request shape for import-style workflows.

Example:

```json
{
  "emails": ["alex@example.com"],
  "reason": "complaint",
  "reason_detail": "Complaint imported from ESP feedback",
  "source": "hubspot",
  "source_type": "provider_event",
  "source_ref": "campaign-123",
  "created_by": "revops@example.com",
  "expires_at": "2026-12-31T23:59:59Z",
  "confidence": "high",
  "auto_suppressed_by_policy": true,
  "metadata": {
    "batch_id": "esp-2026-06-30"
  }
}
```

Legacy fields still work:

- `emails`
- `reason`
- `source`
- `notes`

Supported reasons are:

- `manual`
- `bounce`
- `invalid`
- `spam_trap`
- `unsubscribe`
- `complaint`
- `auto_invalid`

## Blocking Rules

Only active, unexpired entries block an email. Suppression checks update `last_seen_at`.

```http
GET /v1/suppressions/check?email=alex@example.com
```

The response includes suppression provenance such as reason, source, creator, expiry, and metadata when a blocking entry exists.

## List And Filter

```http
GET /v1/suppressions
```

Useful filters:

- `status=active|inactive|merged|all`
- `reason=bounce`
- `source_type=provider_event`
- `source_ref=campaign-123`
- `include_expired=true`
- `limit`
- `offset`

## Revoke

```http
DELETE /v1/suppressions/{id}
```

Delete is a soft revoke. It changes the entry status rather than physically removing the audit trail.

## Events

```http
GET /v1/suppressions/{id}/events
```

Events include status changes, reason/source changes, actor details, and metadata.

## Export

```http
GET /v1/suppressions/export?format=csv
```

Exports stream as CSV and support the same filters as list endpoints.
