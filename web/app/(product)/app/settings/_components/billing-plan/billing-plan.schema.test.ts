import { describe, expect, it } from "vitest";

import { BillingPlanPropsSchema } from "./billing-plan.schema";

describe("BillingPlanPropsSchema", () => {
	it("parses a plan with its billing flags", () => {
		const parsed = BillingPlanPropsSchema.safeParse({
			plan: "professional",
			billingEnabled: true,
			canManage: false,
		});
		expect(parsed.success).toBe(true);
	});

	it("rejects a plan the backend does not know", () => {
		expect(
			BillingPlanPropsSchema.safeParse({ plan: "pro", billingEnabled: true, canManage: true })
				.success,
		).toBe(false);
	});
});
