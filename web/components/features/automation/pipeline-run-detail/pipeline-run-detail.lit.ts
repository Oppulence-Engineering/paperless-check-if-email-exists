import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Pipeline run detail — Shows the stats, the delivery, the errors, the links, and the times of one pipeline run.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const PipelineRunDetailLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("pipeline-run-detail"),
	domain: z.literal("automation"),
	owner: z.literal("feature"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const PipelineRunDetailLit = PipelineRunDetailLitSchema.parse({
	kind: "component",
	name: "pipeline-run-detail",
	domain: "automation",
	owner: "feature",
	client: true,
	summary:
		"Shows the stats, the delivery, the errors, the links, and the times of one pipeline run.",
	schemas: ["components/features/automation/pipeline-run-detail/pipeline-run-detail.schema.ts"],
	files: [
		"components/features/automation/pipeline-run-detail/pipeline-run-detail.tsx",
		"components/features/automation/pipeline-run-detail/pipeline-run-detail.test.tsx",
		"components/features/automation/pipeline-run-detail/pipeline-run-detail.schema.ts",
		"components/features/automation/pipeline-run-detail/pipeline-run-detail.schema.test.ts",
		"components/features/automation/pipeline-run-detail/pipeline-run-detail.lit.ts",
		"components/features/automation/pipeline-run-detail/pipeline-run-detail.stories.tsx",
	],
});
