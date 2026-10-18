import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Result explorer — Finds verification results with every filter and opens one result in a sheet.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const ResultExplorerLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("result-explorer"),
	domain: z.literal("insights"),
	owner: z.literal("feature"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const ResultExplorerLit = ResultExplorerLitSchema.parse({
	kind: "component",
	name: "result-explorer",
	domain: "insights",
	owner: "feature",
	client: true,
	summary: "Finds verification results with every filter and opens one result in a sheet.",
	schemas: ["components/features/insights/result-explorer/result-explorer.schema.ts"],
	files: [
		"components/features/insights/result-explorer/result-explorer.tsx",
		"components/features/insights/result-explorer/result-explorer.test.tsx",
		"components/features/insights/result-explorer/result-explorer.schema.ts",
		"components/features/insights/result-explorer/result-explorer.schema.test.ts",
		"components/features/insights/result-explorer/result-explorer.lit.ts",
		"components/features/insights/result-explorer/result-explorer.stories.tsx",
	],
});
