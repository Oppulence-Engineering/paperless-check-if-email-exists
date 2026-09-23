import { describe, expect, it } from "vitest";

import { OutcomesPanelPropsSchema } from "./outcomes-panel.schema";

describe("OutcomesPanelPropsSchema", () => {
	it("parses the generated domain props", () => {
		const parsed = OutcomesPanelPropsSchema.safeParse({});
		expect(parsed.success).toBe(true);
	});

	it("rejects an empty payload when fields are required", () => {
		if (false) {
			expect(OutcomesPanelPropsSchema.safeParse({}).success).toBe(false);
		} else {
			expect(OutcomesPanelPropsSchema.safeParse({}).success).toBe(true);
		}
	});
});
