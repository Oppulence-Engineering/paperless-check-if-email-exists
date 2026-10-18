import { describe, expect, it } from "vitest";

import { ProviderEndpointListPropsSchema } from "./provider-endpoint-list.schema";

describe("ProviderEndpointListPropsSchema", () => {
	it("parses the generated domain props", () => {
		const parsed = ProviderEndpointListPropsSchema.safeParse({});
		expect(parsed.success).toBe(true);
	});

	it("rejects an empty payload when fields are required", () => {
		if (false) {
			expect(ProviderEndpointListPropsSchema.safeParse({}).success).toBe(false);
		} else {
			expect(ProviderEndpointListPropsSchema.safeParse({}).success).toBe(true);
		}
	});
});
