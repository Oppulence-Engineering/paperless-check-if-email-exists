import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Manage suppression decisions and their audit history.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const SuppressionsPanelLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("suppressions-panel"),
	domain: z.literal(""),
	owner: z.literal("route"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const SuppressionsPanelLit = SuppressionsPanelLitSchema.parse({
	kind: "component",
	name: "suppressions-panel",
	domain: "",
	owner: "route",
	client: true,
	summary: "Manage suppression decisions and their audit history.",
	schemas: [
		"app/(product)/app/suppressions/_components/suppressions-panel/suppressions-panel.schema.ts",
	],
	files: [
		"app/(product)/app/suppressions/_components/suppressions-panel/suppressions-panel.tsx",
		"app/(product)/app/suppressions/_components/suppressions-panel/suppressions-panel.test.tsx",
		"app/(product)/app/suppressions/_components/suppressions-panel/suppressions-panel.schema.ts",
		"app/(product)/app/suppressions/_components/suppressions-panel/suppressions-panel.schema.test.ts",
		"app/(product)/app/suppressions/_components/suppressions-panel/suppressions-panel.lit.ts",
		"app/(product)/app/suppressions/_components/suppressions-panel/suppressions-panel.stories.tsx",
	],
});
