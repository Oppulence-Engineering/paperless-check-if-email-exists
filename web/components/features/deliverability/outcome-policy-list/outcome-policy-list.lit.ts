import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Outcome policy list — Lists the outcome policies and shows each rule in plain words.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const OutcomePolicyListLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("outcome-policy-list"),
	domain: z.literal("deliverability"),
	owner: z.literal("feature"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const OutcomePolicyListLit = OutcomePolicyListLitSchema.parse({
	kind: "component",
	name: "outcome-policy-list",
	domain: "deliverability",
	owner: "feature",
	client: true,
	summary: "Lists the outcome policies and shows each rule in plain words.",
	schemas: ["components/features/deliverability/outcome-policy-list/outcome-policy-list.schema.ts"],
	files: [
		"components/features/deliverability/outcome-policy-list/outcome-policy-list.tsx",
		"components/features/deliverability/outcome-policy-list/outcome-policy-list.test.tsx",
		"components/features/deliverability/outcome-policy-list/outcome-policy-list.schema.ts",
		"components/features/deliverability/outcome-policy-list/outcome-policy-list.schema.test.ts",
		"components/features/deliverability/outcome-policy-list/outcome-policy-list.lit.ts",
		"components/features/deliverability/outcome-policy-list/outcome-policy-list.stories.tsx",
	],
});
