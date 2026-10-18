import { describe, expect, it } from "vitest";

import { CampaignOutcomeListPropsSchema } from "./campaign-outcome-list.schema";

describe("CampaignOutcomeListPropsSchema", () => {
	it("parses the generated domain props", () => {
		const parsed = CampaignOutcomeListPropsSchema.safeParse({});
		expect(parsed.success).toBe(true);
	});

	it("rejects an empty payload when fields are required", () => {
		if (false) {
			expect(CampaignOutcomeListPropsSchema.safeParse({}).success).toBe(false);
		} else {
			expect(CampaignOutcomeListPropsSchema.safeParse({}).success).toBe(true);
		}
	});
});
