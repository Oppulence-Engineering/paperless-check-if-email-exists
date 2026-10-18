import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Suppression audit — Shows the audit timeline of one suppression entry.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const SuppressionAuditLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("suppression-audit"),
	domain: z.literal("deliverability"),
	owner: z.literal("feature"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const SuppressionAuditLit = SuppressionAuditLitSchema.parse({
	kind: "component",
	name: "suppression-audit",
	domain: "deliverability",
	owner: "feature",
	client: true,
	summary: "Shows the audit timeline of one suppression entry.",
	schemas: ["components/features/deliverability/suppression-audit/suppression-audit.schema.ts"],
	files: [
		"components/features/deliverability/suppression-audit/suppression-audit.tsx",
		"components/features/deliverability/suppression-audit/suppression-audit.test.tsx",
		"components/features/deliverability/suppression-audit/suppression-audit.schema.ts",
		"components/features/deliverability/suppression-audit/suppression-audit.schema.test.ts",
		"components/features/deliverability/suppression-audit/suppression-audit.lit.ts",
		"components/features/deliverability/suppression-audit/suppression-audit.stories.tsx",
	],
});
