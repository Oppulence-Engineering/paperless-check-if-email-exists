import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Suppression form — Adds addresses or imports a CSV file into the suppression list.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const SuppressionFormLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("suppression-form"),
	domain: z.literal("deliverability"),
	owner: z.literal("feature"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const SuppressionFormLit = SuppressionFormLitSchema.parse({
	kind: "component",
	name: "suppression-form",
	domain: "deliverability",
	owner: "feature",
	client: true,
	summary: "Adds addresses or imports a CSV file into the suppression list.",
	schemas: ["components/features/deliverability/suppression-form/suppression-form.schema.ts"],
	files: [
		"components/features/deliverability/suppression-form/suppression-form.tsx",
		"components/features/deliverability/suppression-form/suppression-form.test.tsx",
		"components/features/deliverability/suppression-form/suppression-form.schema.ts",
		"components/features/deliverability/suppression-form/suppression-form.schema.test.ts",
		"components/features/deliverability/suppression-form/suppression-form.lit.ts",
		"components/features/deliverability/suppression-form/suppression-form.stories.tsx",
	],
});
