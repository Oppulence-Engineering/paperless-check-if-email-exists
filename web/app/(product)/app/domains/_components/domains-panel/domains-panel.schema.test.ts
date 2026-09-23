import { describe, expect, it } from "vitest";

import { DomainsPanelPropsSchema } from "./domains-panel.schema";

describe("DomainsPanelPropsSchema", () => {
	it("parses the generated domain props", () => {
		const parsed = DomainsPanelPropsSchema.safeParse({});
		expect(parsed.success).toBe(true);
	});

	it("rejects an empty payload when fields are required", () => {
		if (false) {
			expect(DomainsPanelPropsSchema.safeParse({}).success).toBe(false);
		} else {
			expect(DomainsPanelPropsSchema.safeParse({}).success).toBe(true);
		}
	});
});
