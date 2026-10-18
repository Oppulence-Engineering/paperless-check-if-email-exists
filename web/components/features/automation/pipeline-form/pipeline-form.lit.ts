import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Pipeline form — Creates a paused pipeline, or edits the settings of one pipeline.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const PipelineFormLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("pipeline-form"),
	domain: z.literal("automation"),
	owner: z.literal("feature"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const PipelineFormLit = PipelineFormLitSchema.parse({
	kind: "component",
	name: "pipeline-form",
	domain: "automation",
	owner: "feature",
	client: true,
	summary: "Creates a paused pipeline, or edits the settings of one pipeline.",
	schemas: ["components/features/automation/pipeline-form/pipeline-form.schema.ts"],
	files: [
		"components/features/automation/pipeline-form/pipeline-form.tsx",
		"components/features/automation/pipeline-form/pipeline-form.test.tsx",
		"components/features/automation/pipeline-form/pipeline-form.schema.ts",
		"components/features/automation/pipeline-form/pipeline-form.schema.test.ts",
		"components/features/automation/pipeline-form/pipeline-form.lit.ts",
		"components/features/automation/pipeline-form/pipeline-form.stories.tsx",
	],
});
