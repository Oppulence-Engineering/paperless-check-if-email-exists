import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Pipeline list — Lists the pipelines with the status, the schedule, the next run, and the last run.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const PipelineListLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("pipeline-list"),
	domain: z.literal("automation"),
	owner: z.literal("feature"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const PipelineListLit = PipelineListLitSchema.parse({
	kind: "component",
	name: "pipeline-list",
	domain: "automation",
	owner: "feature",
	client: true,
	summary: "Lists the pipelines with the status, the schedule, the next run, and the last run.",
	schemas: ["components/features/automation/pipeline-list/pipeline-list.schema.ts"],
	files: [
		"components/features/automation/pipeline-list/pipeline-list.tsx",
		"components/features/automation/pipeline-list/pipeline-list.test.tsx",
		"components/features/automation/pipeline-list/pipeline-list.schema.ts",
		"components/features/automation/pipeline-list/pipeline-list.schema.test.ts",
		"components/features/automation/pipeline-list/pipeline-list.lit.ts",
		"components/features/automation/pipeline-list/pipeline-list.stories.tsx",
	],
});
