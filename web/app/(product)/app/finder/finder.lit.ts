import { z } from "zod";

/**
 * @oppulence-gen kind=page
 * Find an email — Find and verify likely work addresses for a person and domain
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const FinderLitSchema = z.object({
	kind: z.literal("page"),
	name: z.literal("finder"),
	domain: z.literal("finder"),
	owner: z.literal("page"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const FinderLit = FinderLitSchema.parse({
	kind: "page",
	name: "finder",
	domain: "finder",
	owner: "page",
	client: true,
	summary: "Find and verify likely work addresses for a person and domain",
	schemas: ["app/(product)/app/finder/search-params.ts"],
	files: [
		"app/(product)/app/finder/finder.lit.ts",
		"app/(product)/app/finder/page.tsx",
		"app/(product)/app/finder/loading.tsx",
		"app/(product)/app/finder/error.tsx",
		"app/(product)/app/finder/search-params.ts",
	],
});
