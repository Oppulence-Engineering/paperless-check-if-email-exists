import { z } from "zod";

/**
 * @oppulence-gen kind=page
 * Check an email — Authenticated single-address verification route.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const CheckLitSchema = z.object({
	kind: z.literal("page"),
	name: z.literal("check"),
	domain: z.literal("check"),
	owner: z.literal("page"),
	client: z.literal(false),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const CheckLit = CheckLitSchema.parse({
	kind: "page",
	name: "check",
	domain: "check",
	owner: "page",
	client: false,
	summary: "Authenticated single-address verification route.",
	schemas: ["app/(product)/app/check/search-params.ts"],
	files: [
		"app/(product)/app/check/check.lit.ts",
		"app/(product)/app/check/page.tsx",
		"app/(product)/app/check/loading.tsx",
		"app/(product)/app/check/error.tsx",
		"app/(product)/app/check/search-params.ts",
	],
});
