import { describe, expect, it } from "vitest";

import { V1GetPipelineRun200Response } from "@/lib/api/generated/zod/pipelines/pipelines";

import { PipelineRunDetailPropsSchema } from "./pipeline-run-detail.schema";

const run = {
	id: 2,
	pipeline_id: 2,
	tenant_id: "9a8b3676-6178-5756-9e7d-1172c6bd5408",
	trigger_type: "push",
	status: "queued",
	source_snapshot: { source: { type: "push" }, push_batch_id: 4, source_key: null },
	stats: { accepted_rows: 2, total_valid: 1 },
	billed_emails: 0,
	delivery_status: "not_requested",
	delivery_attempts: 0,
	created_at: "2026-10-18T05:28:45.252340Z",
	updated_at: "2026-10-18T05:28:45.252340Z",
};

describe("PipelineRunDetailPropsSchema", () => {
	it("needs a pipeline ID and a run ID", () => {
		expect(PipelineRunDetailPropsSchema.safeParse({ pipelineId: 1, runId: 2 }).success).toBe(true);
		expect(PipelineRunDetailPropsSchema.safeParse({ pipelineId: 1 }).success).toBe(false);
	});
});

describe("the generated pipeline run schema", () => {
	it("keeps the run stats and the push snapshot", () => {
		const parsed = V1GetPipelineRun200Response.parse(run);
		expect(parsed.stats.accepted_rows).toBe(2);
		expect(parsed.stats.total_valid).toBe(1);
		expect(parsed.source_snapshot).toEqual(run.source_snapshot);
	});

	it("rejects a run without its status", () => {
		expect(V1GetPipelineRun200Response.safeParse({ ...run, status: undefined }).success).toBe(
			false,
		);
	});
});
