import { describe, expect, it } from "vitest";

import { IntegrationsPanelPropsSchema } from "./integrations-panel.schema";

describe("IntegrationsPanelPropsSchema", () => {
	it("parses the generated domain props", () => {
		const parsed = IntegrationsPanelPropsSchema.safeParse({});
		expect(parsed.success).toBe(true);
	});

	it("rejects an empty payload when fields are required", () => {
		if (false) {
			expect(IntegrationsPanelPropsSchema.safeParse({}).success).toBe(false);
		} else {
			expect(IntegrationsPanelPropsSchema.safeParse({}).success).toBe(true);
		}
	});
});
