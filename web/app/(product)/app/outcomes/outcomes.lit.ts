import { z } from "zod";

/**
 * @oppulence-gen kind=page
 * Outcomes — Review delivery outcomes and configure provider callbacks
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const OutcomesLitSchema = z.object({
	kind: z.literal("page"),
	name: z.literal("outcomes"),
	domain: z.literal("outcomes"),
	owner: z.literal("page"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const OutcomesLit = OutcomesLitSchema.parse({
	kind: "page",
	name: "outcomes",
	domain: "outcomes",
	owner: "page",
	client: true,
	summary: "Review delivery outcomes and configure provider callbacks",
	schemas: ["app/(product)/app/outcomes/search-params.ts"],
	files: [
		"app/(product)/app/outcomes/outcomes.lit.ts",
		"app/(product)/app/outcomes/page.tsx",
		"app/(product)/app/outcomes/loading.tsx",
		"app/(product)/app/outcomes/error.tsx",
		"app/(product)/app/outcomes/search-params.ts",
	],
});
