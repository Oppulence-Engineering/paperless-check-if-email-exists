import { z } from "zod";

/**
 * @oppulence-gen kind=page
 * Platform admin — Cross-tenant admin route for support.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const AdminLitSchema = z.object({
	kind: z.literal("page"),
	name: z.literal("admin"),
	domain: z.literal("admin"),
	owner: z.literal("page"),
	client: z.literal(false),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const AdminLit = AdminLitSchema.parse({
	kind: "page",
	name: "admin",
	domain: "admin",
	owner: "page",
	client: false,
	summary: "Cross-tenant admin route for support.",
	schemas: ["app/(product)/app/admin/search-params.ts"],
	files: [
		"app/(product)/app/admin/admin.lit.ts",
		"app/(product)/app/admin/page.tsx",
		"app/(product)/app/admin/loading.tsx",
		"app/(product)/app/admin/error.tsx",
		"app/(product)/app/admin/search-params.ts",
	],
});
