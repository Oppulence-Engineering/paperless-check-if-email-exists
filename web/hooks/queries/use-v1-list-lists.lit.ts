import { z } from "zod";

/**
 * @oppulence-gen kind=hook
 * V1 list lists — List uploaded email files
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const V1ListListsLitSchema = z.object({
	kind: z.literal("hook"),
	name: z.literal("v1-list-lists"),
	domain: z.literal(""),
	owner: z.literal("hook"),
	client: z.literal(false),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const V1ListListsLit = V1ListListsLitSchema.parse({
	kind: "hook",
	name: "v1-list-lists",
	domain: "",
	owner: "hook",
	client: false,
	summary: "List uploaded email files",
	schemas: [],
	files: [
		"hooks/queries/use-v1-list-lists.ts",
		"hooks/queries/utils/fetch-v1-list-lists.ts",
		"hooks/queries/utils/v1-list-lists-keys.ts",
		"hooks/queries/use-v1-list-lists.lit.ts",
	],
});
