import { z } from "zod";

/**
 * @oppulence-gen kind=page
 * Email lists — Manage uploaded lists
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const ListsLitSchema = z.object({
	kind: z.literal("page"),
	name: z.literal("lists"),
	domain: z.literal("lists"),
	owner: z.literal("page"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const ListsLit = ListsLitSchema.parse({
	kind: "page",
	name: "lists",
	domain: "lists",
	owner: "page",
	client: true,
	summary: "Manage uploaded lists",
	schemas: ["app/(product)/app/lists/search-params.ts"],
	files: [
		"app/(product)/app/lists/lists.lit.ts",
		"app/(product)/app/lists/page.tsx",
		"app/(product)/app/lists/loading.tsx",
		"app/(product)/app/lists/error.tsx",
		"app/(product)/app/lists/search-params.ts",
	],
});
