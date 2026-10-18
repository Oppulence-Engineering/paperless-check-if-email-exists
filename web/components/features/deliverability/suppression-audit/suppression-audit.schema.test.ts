import { describe, expect, it } from "vitest";

import { SuppressionAuditPropsSchema } from "./suppression-audit.schema";

describe("SuppressionAuditPropsSchema", () => {
	it("parses a positive entryId", () => {
		expect(SuppressionAuditPropsSchema.safeParse({ entryId: 3 }).success).toBe(true);
	});

	it("rejects a missing entryId", () => {
		expect(SuppressionAuditPropsSchema.safeParse({}).success).toBe(false);
	});
});
