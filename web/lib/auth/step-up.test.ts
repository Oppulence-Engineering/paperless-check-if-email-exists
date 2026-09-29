import { describe, expect, it } from "vitest";

import { hasValidStepUp } from "./step-up";

describe("hasValidStepUp", () => {
	it("requires a recent grant with the expected factor and purpose", () => {
		const now = Date.parse("2026-09-21T12:00:00Z");
		expect(
			hasValidStepUp({
				verifiedAt: new Date(now - 60_000),
				method: "totp",
				purpose: "admin",
				requiredPurpose: "admin",
				allowedMethods: ["passkey", "totp"],
				now,
			}),
		).toBe(true);
		expect(
			hasValidStepUp({
				verifiedAt: new Date(now - 60_000),
				method: "email-otp",
				purpose: "account-delete",
				requiredPurpose: "admin",
				allowedMethods: ["passkey", "totp"],
				now,
			}),
		).toBe(false);
		expect(
			hasValidStepUp({
				verifiedAt: new Date(now - 15 * 60_000 - 1),
				method: "totp",
				purpose: "admin",
				requiredPurpose: "admin",
				allowedMethods: ["totp"],
				now,
			}),
		).toBe(false);
	});
});
