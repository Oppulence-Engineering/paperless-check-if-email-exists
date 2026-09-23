import { describe, expect, it } from "vitest";

import { StepUpInputSchema } from "./step-up.schema";

describe("StepUpInputSchema", () => {
	it("parses the generated domain props", () => {
		const parsed = StepUpInputSchema.safeParse({
			verifiedAt: "2026-09-21T12:00:00Z",
			method: "passkey",
			purpose: "admin",
			requiredPurpose: "admin",
			allowedMethods: ["passkey", "totp"],
		});
		expect(parsed.success).toBe(true);
	});

	it("rejects an empty payload when fields are required", () => {
		expect(StepUpInputSchema.safeParse({}).success).toBe(false);
	});
});
