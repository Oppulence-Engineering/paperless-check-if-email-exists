import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Outcome entry — Records one delivery event by hand.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const OutcomeEntryLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("outcome-entry"),
	domain: z.literal("deliverability"),
	owner: z.literal("feature"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const OutcomeEntryLit = OutcomeEntryLitSchema.parse({
	kind: "component",
	name: "outcome-entry",
	domain: "deliverability",
	owner: "feature",
	client: true,
	summary: "Records one delivery event by hand.",
	schemas: ["components/features/deliverability/outcome-entry/outcome-entry.schema.ts"],
	files: [
		"components/features/deliverability/outcome-entry/outcome-entry.tsx",
		"components/features/deliverability/outcome-entry/outcome-entry.test.tsx",
		"components/features/deliverability/outcome-entry/outcome-entry.schema.ts",
		"components/features/deliverability/outcome-entry/outcome-entry.schema.test.ts",
		"components/features/deliverability/outcome-entry/outcome-entry.lit.ts",
		"components/features/deliverability/outcome-entry/outcome-entry.stories.tsx",
	],
});
