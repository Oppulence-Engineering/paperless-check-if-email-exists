import { describe, expect, it } from "vitest";

import { SettingsDashboardRoutePropsSchema } from "./settings-dashboard-route.schema";

describe("SettingsDashboardRoutePropsSchema", () => {
	it("accepts a signed-in settings route", () => {
		expect(
			SettingsDashboardRoutePropsSchema.safeParse({
				section: "overview",
				organizationId: "org-1",
				organizationRole: "owner",
				userId: "user-1",
				userName: "Owner",
				userEmail: "owner@example.com",
			}).success,
		).toBe(true);
	});

	it("requires a valid email address", () => {
		expect(
			SettingsDashboardRoutePropsSchema.safeParse({
				section: "overview",
				organizationId: "org-1",
				organizationRole: "owner",
				userId: "user-1",
				userName: "Owner",
				userEmail: "invalid",
			}).success,
		).toBe(false);
	});
});
