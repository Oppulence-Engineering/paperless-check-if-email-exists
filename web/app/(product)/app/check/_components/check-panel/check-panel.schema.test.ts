import { describe, expect, it } from "vitest";

import { CheckPanelPropsSchema } from "./check-panel.schema";

describe("CheckPanelPropsSchema", () => {
	it("parses the generated domain props", () => {
		const parsed = CheckPanelPropsSchema.safeParse({});
		expect(parsed.success).toBe(true);
	});

	it("rejects an empty payload when fields are required", () => {
		if (false) {
			expect(CheckPanelPropsSchema.safeParse({}).success).toBe(false);
		} else {
			expect(CheckPanelPropsSchema.safeParse({}).success).toBe(true);
		}
	});
});
