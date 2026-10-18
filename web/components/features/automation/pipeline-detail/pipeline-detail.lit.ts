import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Pipeline detail — Shows the settings of one pipeline and its actions: pause, resume, run, edit, delete, and push.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const PipelineDetailLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("pipeline-detail"),
	domain: z.literal("automation"),
	owner: z.literal("feature"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const PipelineDetailLit = PipelineDetailLitSchema.parse({
	kind: "component",
	name: "pipeline-detail",
	domain: "automation",
	owner: "feature",
	client: true,
	summary:
		"Shows the settings of one pipeline and its actions: pause, resume, run, edit, delete, and push.",
	schemas: ["components/features/automation/pipeline-detail/pipeline-detail.schema.ts"],
	files: [
		"components/features/automation/pipeline-detail/pipeline-detail.tsx",
		"components/features/automation/pipeline-detail/pipeline-detail.test.tsx",
		"components/features/automation/pipeline-detail/pipeline-detail.schema.ts",
		"components/features/automation/pipeline-detail/pipeline-detail.schema.test.ts",
		"components/features/automation/pipeline-detail/pipeline-detail.lit.ts",
		"components/features/automation/pipeline-detail/pipeline-detail.stories.tsx",
	],
});
