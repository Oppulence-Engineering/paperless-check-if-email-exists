import { describe, expect, it } from "vitest";

import { PipelineDetailPropsSchema } from "./pipeline-detail.schema";

describe("PipelineDetailPropsSchema", () => {
	it("needs the ID of one pipeline", () => {
		expect(PipelineDetailPropsSchema.safeParse({ pipelineId: 7 }).success).toBe(true);
	});

	it("rejects props without a pipeline ID", () => {
		expect(PipelineDetailPropsSchema.safeParse({}).success).toBe(false);
		expect(PipelineDetailPropsSchema.safeParse({ pipelineId: -1 }).success).toBe(false);
	});
});
