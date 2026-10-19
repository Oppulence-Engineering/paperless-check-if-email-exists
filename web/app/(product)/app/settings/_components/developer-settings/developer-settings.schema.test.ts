import { describe, expect, it } from "vitest";

import { DeveloperSettingsPropsSchema } from "./developer-settings.schema";

describe("DeveloperSettingsPropsSchema", () => {
	it("requires a workspace and role", () => {
		expect(
			DeveloperSettingsPropsSchema.safeParse({ organizationId: "org-1", organizationRole: "owner" })
				.success,
		).toBe(true);
		expect(DeveloperSettingsPropsSchema.safeParse({}).success).toBe(false);
	});
});
