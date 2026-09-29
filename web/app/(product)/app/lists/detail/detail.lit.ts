import { z } from "zod";

/**
 * @oppulence-gen kind=page
 * List detail — Review quality, remediation, exports, and team comments for one list
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const ListDetailLitSchema = z.object({
	kind: z.literal("page"),
	name: z.literal("list-detail"),
	domain: z.literal("detail"),
	owner: z.literal("page"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const ListDetailLit = ListDetailLitSchema.parse({
	kind: "page",
	name: "list-detail",
	domain: "detail",
	owner: "page",
	client: true,
	summary: "Review quality, remediation, exports, and team comments for one list",
	schemas: ["app/(product)/app/lists/detail/search-params.ts"],
	files: [
		"app/(product)/app/lists/detail/detail.lit.ts",
		"app/(product)/app/lists/detail/page.tsx",
		"app/(product)/app/lists/detail/loading.tsx",
		"app/(product)/app/lists/detail/error.tsx",
		"app/(product)/app/lists/detail/search-params.ts",
	],
});
