import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Result detail — Shows the full verdict of one result from the result explorer.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const ResultDetailLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("result-detail"),
	domain: z.literal("insights"),
	owner: z.literal("feature"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const ResultDetailLit = ResultDetailLitSchema.parse({
	kind: "component",
	name: "result-detail",
	domain: "insights",
	owner: "feature",
	client: true,
	summary: "Shows the full verdict of one result from the result explorer.",
	schemas: ["components/features/insights/result-detail/result-detail.schema.ts"],
	files: [
		"components/features/insights/result-detail/result-detail.tsx",
		"components/features/insights/result-detail/result-detail.test.tsx",
		"components/features/insights/result-detail/result-detail.schema.ts",
		"components/features/insights/result-detail/result-detail.schema.test.ts",
		"components/features/insights/result-detail/result-detail.lit.ts",
		"components/features/insights/result-detail/result-detail.stories.tsx",
	],
});
