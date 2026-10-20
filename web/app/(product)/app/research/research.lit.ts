import { z } from "zod";

/**
 * @oppulence-gen kind=page
 * Company research — Find relevant contacts from company lists and review evidence before export
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const ResearchLitSchema = z.object({
	kind: z.literal("page"),
	name: z.literal("research"),
	domain: z.literal("research"),
	owner: z.literal("page"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const ResearchLit = ResearchLitSchema.parse({
	kind: "page",
	name: "research",
	domain: "research",
	owner: "page",
	client: true,
	summary: "Find relevant contacts from company lists and review evidence before export",
	schemas: ["app/(product)/app/research/search-params.ts"],
	files: [
		"app/(product)/app/research/research.lit.ts",
		"app/(product)/app/research/page.tsx",
		"app/(product)/app/research/loading.tsx",
		"app/(product)/app/research/error.tsx",
		"app/(product)/app/research/search-params.ts",
	],
});
