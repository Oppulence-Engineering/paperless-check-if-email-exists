# Source Quality

Source quality reporting shows where risky, invalid, or negatively performing contacts came from. It helps RevOps and growth teams compare vendors, imports, forms, campaigns, and enrichment tools.

## Attach Source Keys

List uploads accept multipart form fields:

- `source_key`
- `source`

Bulk jobs accept JSON fields:

- `source_key`
- `source`

Source keys are normalized to lowercase. Use stable values such as:

- `apollo-import`
- `hubspot-list`
- `salesforce-campaign`
- `signup-form`
- `csv-vendor`
- `enrichment-tool`
- `manual-upload`

## Read Source Quality

```http
GET /v1/sources/quality
```

Optional filters:

- `source_key`
- `min_records`
- `limit`

Example:

```http
GET /v1/sources/quality?min_records=100&limit=25
```

Each source row includes:

- total and processed record counts
- valid, risky, unknown, and invalid counts
- safe-to-send count
- recommendation action distribution
- delivered, opened, clicked, bounced, complained, and unsubscribed outcome counts
- risky percentage
- invalid percentage
- unsafe recommendation percentage
- negative outcome percentage
- `quality_grade`
- customer-readable summary

Example summary:

```text
This source produces 18% risky contacts, 6% invalid contacts, and 3% negative outcomes.
```

## How Grades Work

Grades are deterministic and currently based on:

- invalid percentage
- risky percentage
- unsafe recommendation percentage
- negative outcome percentage

Higher invalid, risky, suppress/drop/review, bounce, complaint, and unsubscribe rates lower the grade.
