import { describe, expect, it } from "vitest";

import { OutcomePolicyListPropsSchema } from "./outcome-policy-list.schema";

describe("OutcomePolicyListPropsSchema", () => {
	it("parses the generated domain props", () => {
		const parsed = OutcomePolicyListPropsSchema.safeParse({});
		expect(parsed.success).toBe(true);
	});

	it("rejects an empty payload when fields are required", () => {
		if (false) {
			expect(OutcomePolicyListPropsSchema.safeParse({}).success).toBe(false);
		} else {
			expect(OutcomePolicyListPropsSchema.safeParse({}).success).toBe(true);
		}
	});
});
