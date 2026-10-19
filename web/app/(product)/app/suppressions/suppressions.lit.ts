import { z } from "zod";

/**
 * @oppulence-gen kind=page
 * Suppressions — Search, check, add, import, export, and audit suppressed addresses
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const SuppressionsLitSchema = z.object({
	kind: z.literal("page"),
	name: z.literal("suppressions"),
	domain: z.literal("suppressions"),
	owner: z.literal("page"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const SuppressionsLit = SuppressionsLitSchema.parse({
	kind: "page",
	name: "suppressions",
	domain: "suppressions",
	owner: "page",
	client: true,
	summary: "Search, check, add, import, export, and audit suppressed addresses",
	schemas: ["app/(product)/app/suppressions/search-params.ts"],
	files: [
		"app/(product)/app/suppressions/suppressions.lit.ts",
		"app/(product)/app/suppressions/page.tsx",
		"app/(product)/app/suppressions/loading.tsx",
		"app/(product)/app/suppressions/error.tsx",
		"app/(product)/app/suppressions/search-params.ts",
	],
});
