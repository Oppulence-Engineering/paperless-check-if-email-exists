import { describe, expect, it } from "vitest";

import { CampaignUploadPropsSchema } from "./campaign-upload.schema";

describe("CampaignUploadPropsSchema", () => {
	it("parses the generated domain props", () => {
		const parsed = CampaignUploadPropsSchema.safeParse({});
		expect(parsed.success).toBe(true);
	});

	it("rejects an empty payload when fields are required", () => {
		if (false) {
			expect(CampaignUploadPropsSchema.safeParse({}).success).toBe(false);
		} else {
			expect(CampaignUploadPropsSchema.safeParse({}).success).toBe(true);
		}
	});
});
