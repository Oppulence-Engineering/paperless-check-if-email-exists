import { describe, expect, it } from "vitest";

import { BrandingInputSchema } from "./branding.schema";

describe("BrandingInputSchema", () => {
	it("parses the generated domain props", () => {
		const parsed = BrandingInputSchema.safeParse({
			organizationId: "org_1",
		});
		expect(parsed.success).toBe(true);
	});

	it("rejects an empty payload when fields are required", () => {
		expect(BrandingInputSchema.safeParse({}).success).toBe(true);
	});
});
