import { z } from "zod";

import type { V1GetJobResults200Response } from "@/lib/api/generated/zod/jobs/jobs";

/*
 * Runtime contracts for job responses that backend/openapi.json types as `{}`.
 * The shapes follow the Rust handlers in backend/src/http/v1/jobs and
 * backend/src/http/v1/bulk. They accept extra fields, so a new backend field
 * does not break the page.
 */

/** The job states that can still change (`job_state` in the migrations). */
export const ACTIVE_JOB_STATES = ["pending", "running", "cancelling"];

/** GET /v1/jobs/{job_id} (jobs/get_status.rs). */
export const JobStatusSchema = z.looseObject({
	job_id: z.number(),
	status: z.string(),
	total_records: z.number(),
	created_at: z.string(),
	completed_at: z.string().nullish(),
	cancelled_at: z.string().nullish(),
	task_summary: z.record(z.string(), z.number()),
});

/** GET /v1/bulk/{job_id} (bulk/get_progress.rs). */
export const BulkProgressSchema = z.looseObject({
	created_at: z.string(),
	finished_at: z.string().nullish(),
	total_records: z.number(),
	total_processed: z.number(),
});

/** GET /v1/jobs/{job_id}/latency (jobs/latency.rs). The durations are in milliseconds. */
export const JobLatencySchema = z.looseObject({
	total_completed: z.number(),
	p50_duration_ms: z.number(),
	p95_duration_ms: z.number(),
});

/** GET /v1/jobs/{job_id}/events (jobs/get_events.rs). */
export const JobEventsSchema = z.looseObject({
	total: z.number(),
	events: z.array(
		z.looseObject({
			id: z.number(),
			task_id: z.number().nullish(),
			event_type: z.string(),
			event_data: z.record(z.string(), z.unknown()).nullish(),
			actor: z.string().nullish(),
			created_at: z.string(),
		}),
	),
});

/** GET /v1/jobs/{job_id}/failure-center (jobs/failure_center.rs). */
export const FailureCenterSchema = z.looseObject({
	status: z.string(),
	total_records: z.number(),
	processed_rows: z.number(),
	progress_pct: z.number(),
	task_states: z.record(z.string(), z.number()),
	failures_total: z.number(),
	retry_summary: z.looseObject({
		retryable_rows: z.number(),
		permanent_failures: z.number(),
		rows_with_retries: z.number(),
		max_retry_count: z.number(),
	}),
	estimated_completion_seconds: z.number().nullish(),
	failure_breakdown: z.array(z.looseObject({ error: z.string(), count: z.number() })),
	failed_rows: z.array(
		z.looseObject({
			id: z.number(),
			row_index: z.number().nullish(),
			input: z.string().nullish(),
			task_state: z.string(),
			error: z.string().nullish(),
			retry_count: z.number(),
		}),
	),
});

/** One row of GET /v1/jobs/{job_id}/results. */
export type JobResultRow = z.infer<typeof V1GetJobResults200Response>["results"][number];
