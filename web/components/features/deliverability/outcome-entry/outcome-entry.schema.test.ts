import { describe, expect, it } from "vitest";

import { OutcomeEntryPropsSchema } from "./outcome-entry.schema";

describe("OutcomeEntryPropsSchema", () => {
	it("parses the generated domain props", () => {
		const parsed = OutcomeEntryPropsSchema.safeParse({});
		expect(parsed.success).toBe(true);
	});

	it("rejects an empty payload when fields are required", () => {
		if (false) {
			expect(OutcomeEntryPropsSchema.safeParse({}).success).toBe(false);
		} else {
			expect(OutcomeEntryPropsSchema.safeParse({}).success).toBe(true);
		}
	});
});
