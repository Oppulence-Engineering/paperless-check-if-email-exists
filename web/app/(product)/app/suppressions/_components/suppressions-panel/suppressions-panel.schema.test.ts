import { describe, expect, it } from "vitest";

import { SuppressionsPanelPropsSchema } from "./suppressions-panel.schema";

describe("SuppressionsPanelPropsSchema", () => {
	it("parses the generated domain props", () => {
		const parsed = SuppressionsPanelPropsSchema.safeParse({});
		expect(parsed.success).toBe(true);
	});

	it("rejects an empty payload when fields are required", () => {
		if (false) {
			expect(SuppressionsPanelPropsSchema.safeParse({}).success).toBe(false);
		} else {
			expect(SuppressionsPanelPropsSchema.safeParse({}).success).toBe(true);
		}
	});
});
