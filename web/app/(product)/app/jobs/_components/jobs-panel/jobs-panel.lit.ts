import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Create and inspect bulk verification jobs in the active workspace.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const JobsPanelLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("jobs-panel"),
	domain: z.literal(""),
	owner: z.literal("route"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const JobsPanelLit = JobsPanelLitSchema.parse({
	kind: "component",
	name: "jobs-panel",
	domain: "",
	owner: "route",
	client: true,
	summary: "Create and inspect bulk verification jobs in the active workspace.",
	schemas: ["app/(product)/app/jobs/_components/jobs-panel/jobs-panel.schema.ts"],
	files: [
		"app/(product)/app/jobs/_components/jobs-panel/jobs-panel.tsx",
		"app/(product)/app/jobs/_components/jobs-panel/jobs-panel.test.tsx",
		"app/(product)/app/jobs/_components/jobs-panel/jobs-panel.schema.ts",
		"app/(product)/app/jobs/_components/jobs-panel/jobs-panel.schema.test.ts",
		"app/(product)/app/jobs/_components/jobs-panel/jobs-panel.lit.ts",
		"app/(product)/app/jobs/_components/jobs-panel/jobs-panel.stories.tsx",
	],
});
