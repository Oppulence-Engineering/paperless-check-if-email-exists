import { describe, expect, it } from "vitest";

import { ScimInputSchema } from "./scim.schema";

describe("ScimInputSchema", () => {
	it("parses the generated domain props", () => {
		const parsed = ScimInputSchema.safeParse({
			credentialHashSecret: "test-scim-credential-hash-secret-0001",
		});
		expect(parsed.success).toBe(true);
	});

	it("rejects an empty payload when fields are required", () => {
		expect(ScimInputSchema.safeParse({}).success).toBe(false);
	});
});
