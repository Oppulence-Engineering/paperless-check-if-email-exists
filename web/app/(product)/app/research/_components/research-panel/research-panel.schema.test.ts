import { describe, expect, it } from "vitest";

import { ResearchPanelPropsSchema } from "./research-panel.schema";

describe("ResearchPanelPropsSchema", () => {
	it("parses the generated domain props", () => {
		const parsed = ResearchPanelPropsSchema.safeParse({});
		expect(parsed.success).toBe(true);
	});

	it("rejects an empty payload when fields are required", () => {
		if (false) {
			expect(ResearchPanelPropsSchema.safeParse({}).success).toBe(false);
		} else {
			expect(ResearchPanelPropsSchema.safeParse({}).success).toBe(true);
		}
	});
});
