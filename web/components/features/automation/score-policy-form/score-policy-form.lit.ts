import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Score policy form — Creates a score policy, or changes one.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const ScorePolicyFormLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("score-policy-form"),
	domain: z.literal("automation"),
	owner: z.literal("feature"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const ScorePolicyFormLit = ScorePolicyFormLitSchema.parse({
	kind: "component",
	name: "score-policy-form",
	domain: "automation",
	owner: "feature",
	client: true,
	summary: "Creates a score policy, or changes one.",
	schemas: ["components/features/automation/score-policy-form/score-policy-form.schema.ts"],
	files: [
		"components/features/automation/score-policy-form/score-policy-form.tsx",
		"components/features/automation/score-policy-form/score-policy-form.test.tsx",
		"components/features/automation/score-policy-form/score-policy-form.schema.ts",
		"components/features/automation/score-policy-form/score-policy-form.schema.test.ts",
		"components/features/automation/score-policy-form/score-policy-form.lit.ts",
		"components/features/automation/score-policy-form/score-policy-form.stories.tsx",
	],
});
