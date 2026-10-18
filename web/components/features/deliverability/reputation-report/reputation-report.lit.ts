import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Reputation report — Shows the result of a domain reputation check.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const ReputationReportLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("reputation-report"),
	domain: z.literal("deliverability"),
	owner: z.literal("feature"),
	client: z.literal(false),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const ReputationReportLit = ReputationReportLitSchema.parse({
	kind: "component",
	name: "reputation-report",
	domain: "deliverability",
	owner: "feature",
	client: false,
	summary: "Shows the result of a domain reputation check.",
	schemas: ["components/features/deliverability/reputation-report/reputation-report.schema.ts"],
	files: [
		"components/features/deliverability/reputation-report/reputation-report.tsx",
		"components/features/deliverability/reputation-report/reputation-report.test.tsx",
		"components/features/deliverability/reputation-report/reputation-report.schema.ts",
		"components/features/deliverability/reputation-report/reputation-report.schema.test.ts",
		"components/features/deliverability/reputation-report/reputation-report.lit.ts",
		"components/features/deliverability/reputation-report/reputation-report.stories.tsx",
	],
});
