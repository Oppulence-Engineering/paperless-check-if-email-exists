import { z } from "zod";

/**
 * @oppulence-gen kind=page
 * Platform API — Restricted Rust platform control plane.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const ApiLitSchema = z.object({
	kind: z.literal("page"),
	name: z.literal("api"),
	domain: z.literal("api"),
	owner: z.literal("page"),
	client: z.literal(false),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const ApiLit = ApiLitSchema.parse({
	kind: "page",
	name: "api",
	domain: "api",
	owner: "page",
	client: false,
	summary: "Restricted Rust platform control plane.",
	schemas: ["app/(product)/app/admin/api/search-params.ts"],
	files: [
		"app/(product)/app/admin/api/api.lit.ts",
		"app/(product)/app/admin/api/page.tsx",
		"app/(product)/app/admin/api/loading.tsx",
		"app/(product)/app/admin/api/error.tsx",
		"app/(product)/app/admin/api/search-params.ts",
	],
});
