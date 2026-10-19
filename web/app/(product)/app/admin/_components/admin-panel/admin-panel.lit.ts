import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Admin panel — Platform admin route-private panel. Presentation only.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const AdminPanelLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("admin-panel"),
	domain: z.literal(""),
	owner: z.literal("route"),
	client: z.literal(false),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const AdminPanelLit = AdminPanelLitSchema.parse({
	kind: "component",
	name: "admin-panel",
	domain: "",
	owner: "route",
	client: false,
	summary: "Platform admin route-private panel. Presentation only.",
	schemas: ["app/(product)/app/admin/_components/admin-panel/admin-panel.schema.ts"],
	files: [
		"app/(product)/app/admin/_components/admin-panel/admin-panel.tsx",
		"app/(product)/app/admin/_components/admin-panel/admin-panel.test.tsx",
		"app/(product)/app/admin/_components/admin-panel/admin-panel.schema.ts",
		"app/(product)/app/admin/_components/admin-panel/admin-panel.schema.test.ts",
		"app/(product)/app/admin/_components/admin-panel/admin-panel.lit.ts",
		"app/(product)/app/admin/_components/admin-panel/admin-panel.stories.tsx",
	],
});
