import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Check panel — Check an email route-private panel. Presentation only.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const CheckPanelLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("check-panel"),
	domain: z.literal(""),
	owner: z.literal("route"),
	client: z.literal(false),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const CheckPanelLit = CheckPanelLitSchema.parse({
	kind: "component",
	name: "check-panel",
	domain: "",
	owner: "route",
	client: false,
	summary: "Check an email route-private panel. Presentation only.",
	schemas: ["app/(product)/app/check/_components/check-panel/check-panel.schema.ts"],
	files: [
		"app/(product)/app/check/_components/check-panel/check-panel.tsx",
		"app/(product)/app/check/_components/check-panel/check-panel.test.tsx",
		"app/(product)/app/check/_components/check-panel/check-panel.schema.ts",
		"app/(product)/app/check/_components/check-panel/check-panel.schema.test.ts",
		"app/(product)/app/check/_components/check-panel/check-panel.lit.ts",
		"app/(product)/app/check/_components/check-panel/check-panel.stories.tsx",
	],
});
