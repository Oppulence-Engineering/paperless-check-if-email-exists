import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Result filters — Edits the score, job, date, recommendation, and policy filters of the result explorer.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const ResultFiltersLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("result-filters"),
	domain: z.literal("insights"),
	owner: z.literal("feature"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const ResultFiltersLit = ResultFiltersLitSchema.parse({
	kind: "component",
	name: "result-filters",
	domain: "insights",
	owner: "feature",
	client: true,
	summary: "Edits the score, job, date, recommendation, and policy filters of the result explorer.",
	schemas: ["components/features/insights/result-filters/result-filters.schema.ts"],
	files: [
		"components/features/insights/result-filters/result-filters.tsx",
		"components/features/insights/result-filters/result-filters.test.tsx",
		"components/features/insights/result-filters/result-filters.schema.ts",
		"components/features/insights/result-filters/result-filters.schema.test.ts",
		"components/features/insights/result-filters/result-filters.lit.ts",
		"components/features/insights/result-filters/result-filters.stories.tsx",
	],
});
