import { describe, expect, it } from "vitest";

import { TenantSummarySchema } from "./platform-admin.schema";

describe("TenantSummarySchema", () => {
	it("parses the generated domain props", () => {
		expect(
			TenantSummarySchema.safeParse({
				id: "org_1",
				name: "Acme",
				slug: "acme",
				createdAt: new Date(),
				archivedAt: null,
				memberCount: 3,
			}).success,
		).toBe(true);
	});

	it("rejects an empty payload when fields are required", () => {
		expect(TenantSummarySchema.safeParse({}).success).toBe(false);
		expect(
			TenantSummarySchema.safeParse({
				id: "org_1",
				name: "Acme",
				slug: "acme",
				createdAt: "yesterday",
				archivedAt: null,
				memberCount: 3,
			}).success,
		).toBe(false);
	});
});
