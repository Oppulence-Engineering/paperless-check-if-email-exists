import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Safe setup and status journeys for server integrations.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const IntegrationsPanelLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("integrations-panel"),
	domain: z.literal(""),
	owner: z.literal("route"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const IntegrationsPanelLit = IntegrationsPanelLitSchema.parse({
	kind: "component",
	name: "integrations-panel",
	domain: "",
	owner: "route",
	client: true,
	summary: "Safe setup and status journeys for server integrations.",
	schemas: [
		"app/(product)/app/integrations/_components/integrations-panel/integrations-panel.schema.ts",
	],
	files: [
		"app/(product)/app/integrations/_components/integrations-panel/integrations-panel.tsx",
		"app/(product)/app/integrations/_components/integrations-panel/integrations-panel.test.tsx",
		"app/(product)/app/integrations/_components/integrations-panel/integrations-panel.schema.ts",
		"app/(product)/app/integrations/_components/integrations-panel/integrations-panel.schema.test.ts",
		"app/(product)/app/integrations/_components/integrations-panel/integrations-panel.lit.ts",
		"app/(product)/app/integrations/_components/integrations-panel/integrations-panel.stories.tsx",
	],
});
