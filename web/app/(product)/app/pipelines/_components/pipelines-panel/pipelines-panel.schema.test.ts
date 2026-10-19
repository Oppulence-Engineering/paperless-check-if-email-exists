import { describe, expect, it } from "vitest";

import { PipelinesPanelPropsSchema } from "./pipelines-panel.schema";

describe("PipelinesPanelPropsSchema", () => {
	it("parses the generated domain props", () => {
		const parsed = PipelinesPanelPropsSchema.safeParse({});
		expect(parsed.success).toBe(true);
	});

	it("rejects an empty payload when fields are required", () => {
		if (false) {
			expect(PipelinesPanelPropsSchema.safeParse({}).success).toBe(false);
		} else {
			expect(PipelinesPanelPropsSchema.safeParse({}).success).toBe(true);
		}
	});
});
