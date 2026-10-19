import { z } from "zod";

/**
 * @oppulence-gen kind=page
 * Domains — Review and manage verified sending domains for this workspace
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const DomainsLitSchema = z.object({
	kind: z.literal("page"),
	name: z.literal("domains"),
	domain: z.literal("domains"),
	owner: z.literal("page"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const DomainsLit = DomainsLitSchema.parse({
	kind: "page",
	name: "domains",
	domain: "domains",
	owner: "page",
	client: true,
	summary: "Review and manage verified sending domains for this workspace",
	schemas: ["app/(product)/app/domains/search-params.ts"],
	files: [
		"app/(product)/app/domains/domains.lit.ts",
		"app/(product)/app/domains/page.tsx",
		"app/(product)/app/domains/loading.tsx",
		"app/(product)/app/domains/error.tsx",
		"app/(product)/app/domains/search-params.ts",
	],
});
