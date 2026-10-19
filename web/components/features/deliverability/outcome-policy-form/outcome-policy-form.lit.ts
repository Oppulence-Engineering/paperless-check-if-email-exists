import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Outcome policy form — Creates or edits an outcome policy and its rule for each campaign event.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const OutcomePolicyFormLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("outcome-policy-form"),
	domain: z.literal("deliverability"),
	owner: z.literal("feature"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const OutcomePolicyFormLit = OutcomePolicyFormLitSchema.parse({
	kind: "component",
	name: "outcome-policy-form",
	domain: "deliverability",
	owner: "feature",
	client: true,
	summary: "Creates or edits an outcome policy and its rule for each campaign event.",
	schemas: ["components/features/deliverability/outcome-policy-form/outcome-policy-form.schema.ts"],
	files: [
		"components/features/deliverability/outcome-policy-form/outcome-policy-form.tsx",
		"components/features/deliverability/outcome-policy-form/outcome-policy-form.test.tsx",
		"components/features/deliverability/outcome-policy-form/outcome-policy-form.schema.ts",
		"components/features/deliverability/outcome-policy-form/outcome-policy-form.schema.test.ts",
		"components/features/deliverability/outcome-policy-form/outcome-policy-form.lit.ts",
		"components/features/deliverability/outcome-policy-form/outcome-policy-form.stories.tsx",
	],
});
