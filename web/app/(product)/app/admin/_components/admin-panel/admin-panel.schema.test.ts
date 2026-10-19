import { describe, expect, it } from "vitest";

import { AdminPanelPropsSchema } from "./admin-panel.schema";

describe("AdminPanelPropsSchema", () => {
	it("parses the generated domain props", () => {
		expect(
			AdminPanelPropsSchema.safeParse({
				tenants: [
					{
						id: "org_1",
						name: "Acme",
						slug: "acme",
						createdAt: new Date(),
						archivedAt: null,
						memberCount: 1,
					},
				],
			}).success,
		).toBe(true);
		expect(AdminPanelPropsSchema.safeParse({ tenants: [] }).success).toBe(true);
	});

	it("rejects an empty payload when fields are required", () => {
		expect(AdminPanelPropsSchema.safeParse({}).success).toBe(false);
		expect(AdminPanelPropsSchema.safeParse({ tenants: [{ id: "org_1" }] }).success).toBe(false);
	});
});
