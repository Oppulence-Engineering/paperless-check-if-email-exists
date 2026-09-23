import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Create and operate scheduled or push verification pipelines.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const PipelinesPanelLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("pipelines-panel"),
	domain: z.literal(""),
	owner: z.literal("route"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const PipelinesPanelLit = PipelinesPanelLitSchema.parse({
	kind: "component",
	name: "pipelines-panel",
	domain: "",
	owner: "route",
	client: true,
	summary: "Create and operate scheduled or push verification pipelines.",
	schemas: ["app/(product)/app/pipelines/_components/pipelines-panel/pipelines-panel.schema.ts"],
	files: [
		"app/(product)/app/pipelines/_components/pipelines-panel/pipelines-panel.tsx",
		"app/(product)/app/pipelines/_components/pipelines-panel/pipelines-panel.test.tsx",
		"app/(product)/app/pipelines/_components/pipelines-panel/pipelines-panel.schema.ts",
		"app/(product)/app/pipelines/_components/pipelines-panel/pipelines-panel.schema.test.ts",
		"app/(product)/app/pipelines/_components/pipelines-panel/pipelines-panel.lit.ts",
		"app/(product)/app/pipelines/_components/pipelines-panel/pipelines-panel.stories.tsx",
	],
});
