import { describe, expect, it } from "vitest";

import { PipelineListPropsSchema } from "./pipeline-list.schema";

describe("PipelineListPropsSchema", () => {
	it("parses the list with and without a selected pipeline", () => {
		expect(PipelineListPropsSchema.safeParse({}).success).toBe(true);
		expect(PipelineListPropsSchema.safeParse({ selectedId: 3 }).success).toBe(true);
	});

	it("rejects a selected ID that is not a positive whole number", () => {
		expect(PipelineListPropsSchema.safeParse({ selectedId: 0 }).success).toBe(false);
		expect(PipelineListPropsSchema.safeParse({ selectedId: 1.5 }).success).toBe(false);
	});
});
