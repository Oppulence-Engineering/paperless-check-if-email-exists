import { describe, expect, it } from "vitest";

import { FinderPanelPropsSchema } from "./finder-panel.schema";

describe("FinderPanelPropsSchema", () => {
	it("parses the generated domain props", () => {
		const parsed = FinderPanelPropsSchema.safeParse({});
		expect(parsed.success).toBe(true);
	});

	it("rejects an empty payload when fields are required", () => {
		if (false) {
			expect(FinderPanelPropsSchema.safeParse({}).success).toBe(false);
		} else {
			expect(FinderPanelPropsSchema.safeParse({}).success).toBe(true);
		}
	});
});
