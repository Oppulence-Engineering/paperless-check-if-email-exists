import { describe, expect, it } from "vitest";

import { WorkspaceUrlScopePropsSchema } from "./workspace-url-scope.schema";

describe("WorkspaceUrlScopePropsSchema", () => {
	it("parses the generated domain props", () => {
		expect(
			WorkspaceUrlScopePropsSchema.safeParse({
				workspaces: [{ id: "org_1", name: "Acme", slug: "acme", role: "owner", logoUrl: null }],
				activeWorkspaceId: "org_1",
			}).success,
		).toBe(true);
	});

	it("rejects an empty payload when fields are required", () => {
		expect(WorkspaceUrlScopePropsSchema.safeParse({}).success).toBe(false);
		expect(
			WorkspaceUrlScopePropsSchema.safeParse({ workspaces: [], activeWorkspaceId: "" }).success,
		).toBe(false);
	});
});
