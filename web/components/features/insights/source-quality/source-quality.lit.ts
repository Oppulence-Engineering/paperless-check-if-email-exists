import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Source quality — Shows every quality counter for each list source.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const SourceQualityLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("source-quality"),
	domain: z.literal("insights"),
	owner: z.literal("feature"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const SourceQualityLit = SourceQualityLitSchema.parse({
	kind: "component",
	name: "source-quality",
	domain: "insights",
	owner: "feature",
	client: true,
	summary: "Shows every quality counter for each list source.",
	schemas: ["components/features/insights/source-quality/source-quality.schema.ts"],
	files: [
		"components/features/insights/source-quality/source-quality.tsx",
		"components/features/insights/source-quality/source-quality.test.tsx",
		"components/features/insights/source-quality/source-quality.schema.ts",
		"components/features/insights/source-quality/source-quality.schema.test.ts",
		"components/features/insights/source-quality/source-quality.lit.ts",
		"components/features/insights/source-quality/source-quality.stories.tsx",
	],
});
