import { describe, expect, it } from "vitest";

import { OutcomePolicyFormPropsSchema } from "./outcome-policy-form.schema";

const policy = {
	id: 1,
	name: "default",
	is_default: true,
	rules: {
		soft_bounce: { action: "suppress_after", threshold_count: 3, threshold_window_days: 30 },
		outcome_ttl_days: 90,
	},
	created_at: "2026-10-18T04:53:29.264641Z",
	updated_at: "2026-10-18T04:53:29.264641Z",
};

describe("OutcomePolicyFormPropsSchema", () => {
	it("parses the create form and the edit form", () => {
		expect(OutcomePolicyFormPropsSchema.safeParse({}).success).toBe(true);
		expect(OutcomePolicyFormPropsSchema.safeParse({ policy }).success).toBe(true);
	});

	it("rejects a rule without an action", () => {
		expect(
			OutcomePolicyFormPropsSchema.safeParse({
				policy: { ...policy, rules: { hard_bounce: { score_override: "invalid" } } },
			}).success,
		).toBe(false);
	});
});
