import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Score policy list — Lists the score policies and shows each rule in plain words.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const ScorePolicyListLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("score-policy-list"),
	domain: z.literal("automation"),
	owner: z.literal("feature"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const ScorePolicyListLit = ScorePolicyListLitSchema.parse({
	kind: "component",
	name: "score-policy-list",
	domain: "automation",
	owner: "feature",
	client: true,
	summary: "Lists the score policies and shows each rule in plain words.",
	schemas: ["components/features/automation/score-policy-list/score-policy-list.schema.ts"],
	files: [
		"components/features/automation/score-policy-list/score-policy-list.tsx",
		"components/features/automation/score-policy-list/score-policy-list.test.tsx",
		"components/features/automation/score-policy-list/score-policy-list.schema.ts",
		"components/features/automation/score-policy-list/score-policy-list.schema.test.ts",
		"components/features/automation/score-policy-list/score-policy-list.lit.ts",
		"components/features/automation/score-policy-list/score-policy-list.stories.tsx",
	],
});
