import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * List compare — Compares one list with an earlier list and shows what changed.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const ListCompareLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("list-compare"),
	domain: z.literal("lists"),
	owner: z.literal("feature"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const ListCompareLit = ListCompareLitSchema.parse({
	kind: "component",
	name: "list-compare",
	domain: "lists",
	owner: "feature",
	client: true,
	summary: "Compares one list with an earlier list and shows what changed.",
	schemas: ["components/features/lists/list-compare/list-compare.schema.ts"],
	files: [
		"components/features/lists/list-compare/list-compare.tsx",
		"components/features/lists/list-compare/list-compare.test.tsx",
		"components/features/lists/list-compare/list-compare.schema.ts",
		"components/features/lists/list-compare/list-compare.schema.test.ts",
		"components/features/lists/list-compare/list-compare.lit.ts",
		"components/features/lists/list-compare/list-compare.stories.tsx",
	],
});
