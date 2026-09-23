import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Find and verify work email candidates in the active workspace.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const FinderPanelLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("finder-panel"),
	domain: z.literal(""),
	owner: z.literal("route"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const FinderPanelLit = FinderPanelLitSchema.parse({
	kind: "component",
	name: "finder-panel",
	domain: "",
	owner: "route",
	client: true,
	summary: "Find and verify work email candidates in the active workspace.",
	schemas: ["app/(product)/app/finder/_components/finder-panel/finder-panel.schema.ts"],
	files: [
		"app/(product)/app/finder/_components/finder-panel/finder-panel.tsx",
		"app/(product)/app/finder/_components/finder-panel/finder-panel.test.tsx",
		"app/(product)/app/finder/_components/finder-panel/finder-panel.schema.ts",
		"app/(product)/app/finder/_components/finder-panel/finder-panel.schema.test.ts",
		"app/(product)/app/finder/_components/finder-panel/finder-panel.lit.ts",
		"app/(product)/app/finder/_components/finder-panel/finder-panel.stories.tsx",
	],
});
