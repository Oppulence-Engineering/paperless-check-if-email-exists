import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Review delivery feedback and manage provider endpoints.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const OutcomesPanelLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("outcomes-panel"),
	domain: z.literal(""),
	owner: z.literal("route"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const OutcomesPanelLit = OutcomesPanelLitSchema.parse({
	kind: "component",
	name: "outcomes-panel",
	domain: "",
	owner: "route",
	client: true,
	summary: "Review delivery feedback and manage provider endpoints.",
	schemas: ["app/(product)/app/outcomes/_components/outcomes-panel/outcomes-panel.schema.ts"],
	files: [
		"app/(product)/app/outcomes/_components/outcomes-panel/outcomes-panel.tsx",
		"app/(product)/app/outcomes/_components/outcomes-panel/outcomes-panel.test.tsx",
		"app/(product)/app/outcomes/_components/outcomes-panel/outcomes-panel.schema.ts",
		"app/(product)/app/outcomes/_components/outcomes-panel/outcomes-panel.schema.test.ts",
		"app/(product)/app/outcomes/_components/outcomes-panel/outcomes-panel.lit.ts",
		"app/(product)/app/outcomes/_components/outcomes-panel/outcomes-panel.stories.tsx",
	],
});
