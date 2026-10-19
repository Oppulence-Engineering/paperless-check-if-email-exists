import { describe, expect, it } from "vitest";

import { WorkspaceAdminActionSchema, WorkspaceLogoSchema } from "./schemas";

describe("workspace administration schemas", () => {
	it("accepts HTTPS, same-origin, and bounded raster logo sources", () => {
		expect(WorkspaceLogoSchema.parse("https://cdn.example.com/logo.webp")).toBe(
			"https://cdn.example.com/logo.webp",
		);
		expect(WorkspaceLogoSchema.parse("/logos/workspace.png")).toBe("/logos/workspace.png");
		expect(WorkspaceLogoSchema.parse("data:image/png;base64,aGVsbG8=")).toContain(
			"data:image/png;base64,",
		);
	});

	it("rejects executable or oversized uploaded logo sources", () => {
		expect(() => WorkspaceLogoSchema.parse("data:image/svg+xml;base64,PHN2Zz4=")).toThrow();
		expect(() =>
			WorkspaceLogoSchema.parse(`data:image/png;base64,${"A".repeat(350_000)}`),
		).toThrow();
	});

	it("keeps ownership separate from ordinary member role changes", () => {
		expect(
			WorkspaceAdminActionSchema.parse({
				action: "change_member_role",
				memberId: "member-2",
				role: "admin",
			}),
		).toEqual({ action: "change_member_role", memberId: "member-2", role: "admin" });
		expect(() =>
			WorkspaceAdminActionSchema.parse({
				action: "change_member_role",
				memberId: "member-2",
				role: "owner",
			}),
		).toThrow();
	});
});
