import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Workspace domain status and management.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const DomainsPanelLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("domains-panel"),
	domain: z.literal(""),
	owner: z.literal("route"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const DomainsPanelLit = DomainsPanelLitSchema.parse({
	kind: "component",
	name: "domains-panel",
	domain: "",
	owner: "route",
	client: true,
	summary: "Workspace domain status and management.",
	schemas: ["app/(product)/app/domains/_components/domains-panel/domains-panel.schema.ts"],
	files: [
		"app/(product)/app/domains/_components/domains-panel/domains-panel.tsx",
		"app/(product)/app/domains/_components/domains-panel/domains-panel.test.tsx",
		"app/(product)/app/domains/_components/domains-panel/domains-panel.schema.ts",
		"app/(product)/app/domains/_components/domains-panel/domains-panel.schema.test.ts",
		"app/(product)/app/domains/_components/domains-panel/domains-panel.lit.ts",
		"app/(product)/app/domains/_components/domains-panel/domains-panel.stories.tsx",
	],
});
