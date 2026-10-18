import { describe, expect, it } from "vitest";

import { ScorePolicyFormPropsSchema } from "./score-policy-form.schema";

const policy = {
	id: 1,
	name: "strict",
	is_default: true,
	rules: { suppress: { score_max: 30 }, send: { score_min: 80, safe_to_send: true } },
	created_at: "2026-10-18T05:28:23.249217Z",
	updated_at: "2026-10-18T05:28:23.249217Z",
};

describe("ScorePolicyFormPropsSchema", () => {
	it("parses the create form and the edit form", () => {
		expect(ScorePolicyFormPropsSchema.safeParse({}).success).toBe(true);
		expect(ScorePolicyFormPropsSchema.safeParse({ policy }).success).toBe(true);
	});

	it("rejects a policy without a name", () => {
		expect(
			ScorePolicyFormPropsSchema.safeParse({ policy: { ...policy, name: undefined } }).success,
		).toBe(false);
	});
});
