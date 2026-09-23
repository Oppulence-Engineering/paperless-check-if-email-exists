import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Workspace switcher — Changes the active Better Auth organization in the product shell.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const WorkspaceSwitcherLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("workspace-switcher"),
	domain: z.literal("workspaces"),
	owner: z.literal("feature"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const WorkspaceSwitcherLit = WorkspaceSwitcherLitSchema.parse({
	kind: "component",
	name: "workspace-switcher",
	domain: "workspaces",
	owner: "feature",
	client: true,
	summary: "Persistent tenant context and secure active-workspace switching for the product shell.",
	schemas: ["components/features/workspaces/workspace-switcher/workspace-switcher.schema.ts"],
	files: [
		"components/features/workspaces/workspace-switcher/workspace-switcher.tsx",
		"components/features/workspaces/workspace-switcher/workspace-switcher.test.tsx",
		"components/features/workspaces/workspace-switcher/workspace-switcher.schema.ts",
		"components/features/workspaces/workspace-switcher/workspace-switcher.schema.test.ts",
		"components/features/workspaces/workspace-switcher/workspace-switcher.lit.ts",
		"components/features/workspaces/workspace-switcher/workspace-switcher.stories.tsx",
	],
});
