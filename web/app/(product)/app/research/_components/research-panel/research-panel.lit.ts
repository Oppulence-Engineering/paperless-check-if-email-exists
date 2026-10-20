import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Research panel — Company import, bounded research, evidence review and approved export.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const ResearchPanelLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("research-panel"),
	domain: z.literal(""),
	owner: z.literal("route"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const ResearchPanelLit = ResearchPanelLitSchema.parse({
	kind: "component",
	name: "research-panel",
	domain: "",
	owner: "route",
	client: true,
	summary: "Company import, bounded research, evidence review and approved export.",
	schemas: ["app/(product)/app/research/_components/research-panel/research-panel.schema.ts"],
	files: [
		"app/(product)/app/research/_components/research-panel/research-panel.tsx",
		"app/(product)/app/research/_components/research-panel/research-panel.test.tsx",
		"app/(product)/app/research/_components/research-panel/research-panel.schema.ts",
		"app/(product)/app/research/_components/research-panel/research-panel.schema.test.ts",
		"app/(product)/app/research/_components/research-panel/research-panel.lit.ts",
		"app/(product)/app/research/_components/research-panel/research-panel.stories.tsx",
	],
});
