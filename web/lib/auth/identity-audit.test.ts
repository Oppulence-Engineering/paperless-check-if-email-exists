import { describe, expect, it } from "vitest";

import { IdentityAuditInputSchema } from "./identity-audit.schema";

describe("identityAudit", () => {
	it("does not accept token-shaped extra data", () => {
		const parsed = IdentityAuditInputSchema.parse({
			actorId: "user-1",
			organizationId: "org-1",
			action: "session.revoked",
			targetId: "session-1",
			result: "success",
			requestId: "request-1",
			token: "must-not-be-recorded",
		});
		expect(parsed).not.toHaveProperty("token");
	});
});
