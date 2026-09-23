import { describe, expect, it } from "vitest";

import { IdentityAuditInputSchema } from "./identity-audit.schema";

describe("IdentityAuditInputSchema", () => {
	it("parses the generated domain props", () => {
		const parsed = IdentityAuditInputSchema.safeParse({
			actorId: null,
			organizationId: "org-1",
			action: "sign-in",
			targetId: "user-1",
			result: "success",
			requestId: null,
		});
		expect(parsed.success).toBe(true);
	});

	it("rejects an empty payload", () => {
		expect(IdentityAuditInputSchema.safeParse({}).success).toBe(false);
	});
});
