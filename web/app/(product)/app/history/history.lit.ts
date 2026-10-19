import { z } from "zod";

/**
 * @oppulence-gen kind=page
 * Verification history — Look up earlier checks
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const HistoryLitSchema = z.object({
	kind: z.literal("page"),
	name: z.literal("history"),
	domain: z.literal("history"),
	owner: z.literal("page"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const HistoryLit = HistoryLitSchema.parse({
	kind: "page",
	name: "history",
	domain: "history",
	owner: "page",
	client: true,
	summary: "Look up earlier checks",
	schemas: ["app/(product)/app/history/search-params.ts"],
	files: [
		"app/(product)/app/history/history.lit.ts",
		"app/(product)/app/history/page.tsx",
		"app/(product)/app/history/loading.tsx",
		"app/(product)/app/history/error.tsx",
		"app/(product)/app/history/search-params.ts",
	],
});
