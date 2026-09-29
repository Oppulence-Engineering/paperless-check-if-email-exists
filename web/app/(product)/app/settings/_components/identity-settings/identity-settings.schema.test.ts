import { describe, expect, it } from "vitest";

import {
	IdentitySCIMMappingsSchema,
	IdentitySettingsPropsSchema,
} from "./identity-settings.schema";

describe("IdentitySettingsPropsSchema", () => {
	it("parses the generated domain props", () => {
		const parsed = IdentitySettingsPropsSchema.safeParse({
			organizationId: "org_1",
			organizationRole: "owner",
			scope: "enterprise",
			userId: "user_1",
		});
		expect(parsed.success).toBe(true);
	});

	it("accepts the dedicated workspace scope", () => {
		expect(
			IdentitySettingsPropsSchema.safeParse({
				organizationId: "org_1",
				organizationRole: "admin",
				scope: "workspace",
				userId: "user_1",
			}).success,
		).toBe(true);
	});

	it("rejects an empty payload when fields are required", () => {
		expect(IdentitySettingsPropsSchema.safeParse({}).success).toBe(false);
	});
});

describe("IdentitySCIMMappingsSchema", () => {
	it("accepts only built-in organization roles", () => {
		expect(
			IdentitySCIMMappingsSchema.safeParse({
				mappings: [{ providerId: "entra", group: "finance", role: "owner" }],
			}).success,
		).toBe(false);
	});
});
