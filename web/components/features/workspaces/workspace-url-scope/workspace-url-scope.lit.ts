import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Workspace url scope — Opens the workspace a shared link names, when the reader belongs to it.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const WorkspaceUrlScopeLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("workspace-url-scope"),
	domain: z.literal("workspaces"),
	owner: z.literal("feature"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const WorkspaceUrlScopeLit = WorkspaceUrlScopeLitSchema.parse({
	kind: "component",
	name: "workspace-url-scope",
	domain: "workspaces",
	owner: "feature",
	client: true,
	summary: "Opens the workspace a shared link names, when the reader belongs to it.",
	schemas: ["components/features/workspaces/workspace-url-scope/workspace-url-scope.schema.ts"],
	files: [
		"components/features/workspaces/workspace-url-scope/workspace-url-scope.tsx",
		"components/features/workspaces/workspace-url-scope/workspace-url-scope.test.tsx",
		"components/features/workspaces/workspace-url-scope/workspace-url-scope.schema.ts",
		"components/features/workspaces/workspace-url-scope/workspace-url-scope.schema.test.ts",
		"components/features/workspaces/workspace-url-scope/workspace-url-scope.lit.ts",
		"components/features/workspaces/workspace-url-scope/workspace-url-scope.stories.tsx",
	],
});
