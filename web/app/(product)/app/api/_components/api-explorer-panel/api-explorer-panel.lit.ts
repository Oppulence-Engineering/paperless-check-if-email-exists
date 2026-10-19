import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Api explorer panel — API explorer route-private panel. Presentation only.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const ApiExplorerPanelLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("api-explorer-panel"),
	domain: z.literal(""),
	owner: z.literal("route"),
	client: z.literal(false),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const ApiExplorerPanelLit = ApiExplorerPanelLitSchema.parse({
	kind: "component",
	name: "api-explorer-panel",
	domain: "",
	owner: "route",
	client: false,
	summary: "API explorer route-private panel. Presentation only.",
	schemas: ["app/(product)/app/api/_components/api-explorer-panel/api-explorer-panel.schema.ts"],
	files: [
		"app/(product)/app/api/_components/api-explorer-panel/api-explorer-panel.tsx",
		"app/(product)/app/api/_components/api-explorer-panel/api-explorer-panel.test.tsx",
		"app/(product)/app/api/_components/api-explorer-panel/api-explorer-panel.schema.ts",
		"app/(product)/app/api/_components/api-explorer-panel/api-explorer-panel.schema.test.ts",
		"app/(product)/app/api/_components/api-explorer-panel/api-explorer-panel.lit.ts",
		"app/(product)/app/api/_components/api-explorer-panel/api-explorer-panel.stories.tsx",
	],
});
