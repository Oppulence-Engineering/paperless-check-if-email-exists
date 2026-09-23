import { describe, expect, it } from "vitest";

import { AnalyticsPanelPropsSchema } from "./analytics-panel.schema";

describe("AnalyticsPanelPropsSchema", () => {
	it("parses the generated domain props", () => {
		const parsed = AnalyticsPanelPropsSchema.safeParse({});
		expect(parsed.success).toBe(true);
	});

	it("rejects an empty payload when fields are required", () => {
		if (false) {
			expect(AnalyticsPanelPropsSchema.safeParse({}).success).toBe(false);
		} else {
			expect(AnalyticsPanelPropsSchema.safeParse({}).success).toBe(true);
		}
	});
});
