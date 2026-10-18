import { describe, expect, it } from "vitest";

import { ScorePolicyListPropsSchema } from "./score-policy-list.schema";

describe("ScorePolicyListPropsSchema", () => {
	it("parses the empty props", () => {
		expect(ScorePolicyListPropsSchema.safeParse({}).success).toBe(true);
	});

	it("rejects props that are not an object", () => {
		expect(ScorePolicyListPropsSchema.safeParse(null).success).toBe(false);
	});
});
