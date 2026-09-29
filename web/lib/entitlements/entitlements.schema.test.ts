import { describe, expect, it } from "vitest";

import { EntitlementsSchema, FeatureSchema, PlanSchema } from "./entitlements.schema";

describe("EntitlementsSchema", () => {
	it("parses the generated domain props", () => {
		expect(
			EntitlementsSchema.safeParse({
				plan: "pro",
				active: true,
				seats: { used: 2, limit: 50 },
			}).success,
		).toBe(true);
		expect(
			EntitlementsSchema.safeParse({
				plan: "enterprise",
				active: true,
				seats: { used: 900, limit: null },
			}).success,
		).toBe(true);
		expect(PlanSchema.safeParse("starter").success).toBe(true);
		expect(FeatureSchema.safeParse("execute_actions").success).toBe(true);
	});

	it("rejects an empty payload when fields are required", () => {
		expect(EntitlementsSchema.safeParse({}).success).toBe(false);
		expect(PlanSchema.safeParse("platinum").success).toBe(false);
		expect(FeatureSchema.safeParse("actions_page").success).toBe(false);
		expect(
			EntitlementsSchema.safeParse({ plan: "free", active: true, seats: { used: -1, limit: 3 } })
				.success,
		).toBe(false);
	});
});
