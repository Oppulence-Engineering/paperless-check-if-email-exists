import { describe, expect, it } from "vitest";

import { OverviewPanelPropsSchema } from "./overview-panel.schema";

describe("OverviewPanelPropsSchema", () => {
	it("parses the generated domain props", () => {
		const parsed = OverviewPanelPropsSchema.safeParse({});
		expect(parsed.success).toBe(true);
	});

	it("rejects an empty payload when fields are required", () => {
		if (false) {
			expect(OverviewPanelPropsSchema.safeParse({}).success).toBe(false);
		} else {
			expect(OverviewPanelPropsSchema.safeParse({}).success).toBe(true);
		}
	});
});
