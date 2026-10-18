import { describe, expect, it } from "vitest";

import { PipelineRunsPropsSchema } from "./pipeline-runs.schema";

describe("PipelineRunsPropsSchema", () => {
	it("needs a pipeline ID and accepts a selected run", () => {
		expect(PipelineRunsPropsSchema.safeParse({ pipelineId: 1 }).success).toBe(true);
		expect(PipelineRunsPropsSchema.safeParse({ pipelineId: 1, selectedRunId: 4 }).success).toBe(
			true,
		);
	});

	it("rejects props without a pipeline ID", () => {
		expect(PipelineRunsPropsSchema.safeParse({}).success).toBe(false);
	});
});
