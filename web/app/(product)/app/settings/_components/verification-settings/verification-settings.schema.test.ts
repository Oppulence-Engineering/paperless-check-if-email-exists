import { describe, expect, it } from "vitest";

import { VerificationSettingsPropsSchema } from "./verification-settings.schema";

describe("VerificationSettingsPropsSchema", () => {
	it("parses the generated domain props", () => {
		const parsed = VerificationSettingsPropsSchema.safeParse({
			organizationId: "org-1",
			organizationRole: "owner",
		});
		expect(parsed.success).toBe(true);
	});

	it("rejects an empty payload when fields are required", () => {
		expect(VerificationSettingsPropsSchema.safeParse({}).success).toBe(false);
	});
});
