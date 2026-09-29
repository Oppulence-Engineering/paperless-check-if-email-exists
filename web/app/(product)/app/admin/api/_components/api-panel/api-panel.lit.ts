import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Api panel — Platform API route-private panel. Presentation only.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const ApiPanelLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("api-panel"),
	domain: z.literal(""),
	owner: z.literal("route"),
	client: z.literal(false),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const ApiPanelLit = ApiPanelLitSchema.parse({
	kind: "component",
	name: "api-panel",
	domain: "",
	owner: "route",
	client: false,
	summary: "Platform API route-private panel. Presentation only.",
	schemas: ["app/(product)/app/admin/api/_components/api-panel/api-panel.schema.ts"],
	files: [
		"app/(product)/app/admin/api/_components/api-panel/api-panel.tsx",
		"app/(product)/app/admin/api/_components/api-panel/api-panel.test.tsx",
		"app/(product)/app/admin/api/_components/api-panel/api-panel.schema.ts",
		"app/(product)/app/admin/api/_components/api-panel/api-panel.schema.test.ts",
		"app/(product)/app/admin/api/_components/api-panel/api-panel.lit.ts",
		"app/(product)/app/admin/api/_components/api-panel/api-panel.stories.tsx",
	],
});
