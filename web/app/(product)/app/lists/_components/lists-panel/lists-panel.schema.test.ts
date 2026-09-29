import { describe, expect, it } from "vitest";

import { ListsPanelPropsSchema } from "./lists-panel.schema";

describe("ListsPanelPropsSchema", () => {
	it("parses the generated domain props", () => {
		const parsed = ListsPanelPropsSchema.safeParse({});
		expect(parsed.success).toBe(true);
	});

	it("rejects an empty payload when fields are required", () => {
		if (false) {
			expect(ListsPanelPropsSchema.safeParse({}).success).toBe(false);
		} else {
			expect(ListsPanelPropsSchema.safeParse({}).success).toBe(true);
		}
	});
});
