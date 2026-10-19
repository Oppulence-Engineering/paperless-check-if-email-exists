import { describe, expect, it } from "vitest";

import { WorkspaceSwitcherPropsSchema } from "./workspace-switcher.schema";

describe("WorkspaceSwitcherPropsSchema", () => {
	it("parses validated tenant summaries", () => {
		const parsed = WorkspaceSwitcherPropsSchema.safeParse({
			activeWorkspaceId: "workspace-1",
			workspaces: [
				{
					id: "workspace-1",
					name: "Northstar Freight",
					slug: "northstar-freight",
					role: "owner",
					logoUrl: null,
				},
			],
		});
		expect(parsed.success).toBe(true);
	});

	it("accepts an empty tenant list so account controls remain available", () => {
		expect(
			WorkspaceSwitcherPropsSchema.safeParse({
				activeWorkspaceId: "workspace-1",
				workspaces: [],
			}).success,
		).toBe(true);
	});
});
