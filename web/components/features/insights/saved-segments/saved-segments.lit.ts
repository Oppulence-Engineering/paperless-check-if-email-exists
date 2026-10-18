import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Saved segments — Saves, opens, renames, and deletes saved segments of explorer filters.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const SavedSegmentsLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("saved-segments"),
	domain: z.literal("insights"),
	owner: z.literal("feature"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const SavedSegmentsLit = SavedSegmentsLitSchema.parse({
	kind: "component",
	name: "saved-segments",
	domain: "insights",
	owner: "feature",
	client: true,
	summary: "Saves, opens, renames, and deletes saved segments of explorer filters.",
	schemas: ["components/features/insights/saved-segments/saved-segments.schema.ts"],
	files: [
		"components/features/insights/saved-segments/saved-segments.tsx",
		"components/features/insights/saved-segments/saved-segments.test.tsx",
		"components/features/insights/saved-segments/saved-segments.schema.ts",
		"components/features/insights/saved-segments/saved-segments.schema.test.ts",
		"components/features/insights/saved-segments/saved-segments.lit.ts",
		"components/features/insights/saved-segments/saved-segments.stories.tsx",
	],
});
