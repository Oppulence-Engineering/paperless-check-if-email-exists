# Job Observability

The job failure center gives customers one place to inspect failed bulk jobs, estimate progress, and export failed rows for retry or support workflows.

## Failure Center

```http
GET /v1/jobs/{job_id}/failure-center
```

Optional query:

- `preview_limit`: maximum failed rows to include in the response, capped at 200

The response includes:

- job status
- total records
- processed rows
- progress percentage
- task-state counts
- retry summary
- total failures
- estimated completion seconds
- top failure breakdown
- failed-row preview
- failure-report URL
- created, updated, and completed timestamps

Task states include:

- `queued`
- `running`
- `retrying`
- `completed`
- `failed`
- `dead_lettered`
- `cancelled`

## Failure Report

```http
GET /v1/jobs/{job_id}/failure-report?format=csv
GET /v1/jobs/{job_id}/failure-report?format=json
```

`format=csv` streams CSV. `format=json` streams newline-delimited JSON.

Rows include:

- `task_id`
- `row_index`
- `input`
- `task_state`
- `error`
- `retry_count`
- `updated_at`
- `payload`
- `extra`

The report includes rows whose task state is `failed`, `dead_lettered`, or `cancelled`.
