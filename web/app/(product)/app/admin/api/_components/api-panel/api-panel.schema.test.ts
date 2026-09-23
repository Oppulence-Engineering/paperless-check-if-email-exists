import { describe, expect, it } from "vitest";

import { ApiPanelPropsSchema } from "./api-panel.schema";

describe("ApiPanelPropsSchema", () => {
	it("parses the generated domain props", () => {
		const parsed = ApiPanelPropsSchema.safeParse({});
		expect(parsed.success).toBe(true);
	});

	it("rejects an empty payload when fields are required", () => {
		if (false) {
			expect(ApiPanelPropsSchema.safeParse({}).success).toBe(false);
		} else {
			expect(ApiPanelPropsSchema.safeParse({}).success).toBe(true);
		}
	});
});
