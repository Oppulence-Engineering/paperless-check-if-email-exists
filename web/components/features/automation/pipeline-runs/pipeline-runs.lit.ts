import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Pipeline runs — Lists the runs of one pipeline and polls while a run is active.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const PipelineRunsLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("pipeline-runs"),
	domain: z.literal("automation"),
	owner: z.literal("feature"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const PipelineRunsLit = PipelineRunsLitSchema.parse({
	kind: "component",
	name: "pipeline-runs",
	domain: "automation",
	owner: "feature",
	client: true,
	summary: "Lists the runs of one pipeline and polls while a run is active.",
	schemas: ["components/features/automation/pipeline-runs/pipeline-runs.schema.ts"],
	files: [
		"components/features/automation/pipeline-runs/pipeline-runs.tsx",
		"components/features/automation/pipeline-runs/pipeline-runs.test.tsx",
		"components/features/automation/pipeline-runs/pipeline-runs.schema.ts",
		"components/features/automation/pipeline-runs/pipeline-runs.schema.test.ts",
		"components/features/automation/pipeline-runs/pipeline-runs.lit.ts",
		"components/features/automation/pipeline-runs/pipeline-runs.stories.tsx",
	],
});
