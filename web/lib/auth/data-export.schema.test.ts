import { describe, expect, it } from "vitest";

import { DataExportSchema } from "./data-export.schema";

const payload = {
	exportedAt: new Date().toISOString(),
	scope: "Identity, membership, sessions and identity events.",
	identity: {
		id: "user_1",
		name: "Ada",
		email: "ada@example.com",
		emailVerified: true,
		createdAt: new Date().toISOString(),
	},
	memberships: [
		{
			organizationId: "org_1",
			organizationName: "Acme",
			role: "owner",
			joinedAt: new Date().toISOString(),
		},
	],
	sessions: [
		{
			id: "sess_1",
			createdAt: new Date().toISOString(),
			expiresAt: new Date().toISOString(),
			ipAddress: null,
			userAgent: null,
		},
	],
	identityEvents: [
		{
			action: "account.export",
			result: "success",
			organizationId: "org_1",
			createdAt: new Date().toISOString(),
		},
	],
};

describe("DataExportSchema", () => {
	it("parses the generated domain props", () => {
		expect(DataExportSchema.safeParse(payload).success).toBe(true);
		expect(
			DataExportSchema.safeParse({ ...payload, memberships: [], sessions: [], identityEvents: [] })
				.success,
		).toBe(true);
	});

	it("rejects an empty payload when fields are required", () => {
		expect(DataExportSchema.safeParse({}).success).toBe(false);
		// An export without a scope statement overclaims what it contains.
		expect(DataExportSchema.safeParse({ ...payload, scope: "" }).success).toBe(false);
		expect(DataExportSchema.safeParse({ ...payload, exportedAt: "today" }).success).toBe(false);
	});
});
