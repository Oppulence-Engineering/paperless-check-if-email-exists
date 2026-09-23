import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Lists panel — Email lists route-private panel. Presentation only.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const ListsPanelLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("lists-panel"),
	domain: z.literal(""),
	owner: z.literal("route"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const ListsPanelLit = ListsPanelLitSchema.parse({
	kind: "component",
	name: "lists-panel",
	domain: "",
	owner: "route",
	client: true,
	summary: "Email lists route-private panel. Presentation only.",
	schemas: ["app/(product)/app/lists/_components/lists-panel/lists-panel.schema.ts"],
	files: [
		"app/(product)/app/lists/_components/lists-panel/lists-panel.tsx",
		"app/(product)/app/lists/_components/lists-panel/lists-panel.test.tsx",
		"app/(product)/app/lists/_components/lists-panel/lists-panel.schema.ts",
		"app/(product)/app/lists/_components/lists-panel/lists-panel.schema.test.ts",
		"app/(product)/app/lists/_components/lists-panel/lists-panel.lit.ts",
		"app/(product)/app/lists/_components/lists-panel/lists-panel.stories.tsx",
	],
});
