import { z } from "zod";

/**
 * @oppulence-gen kind=lib
 * Platform admin — Cross-tenant visibility for support, gated by an environment allowlist and audited.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const PlatformAdminLitSchema = z.object({
	kind: z.literal("lib"),
	name: z.literal("platform-admin"),
	domain: z.literal("admin"),
	owner: z.literal("lib"),
	client: z.literal(false),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const PlatformAdminLit = PlatformAdminLitSchema.parse({
	kind: "lib",
	name: "platform-admin",
	domain: "admin",
	owner: "lib",
	client: false,
	summary: "Cross-tenant visibility for support, gated by an environment allowlist and audited.",
	schemas: ["lib/admin/platform-admin.schema.ts"],
	files: [
		"lib/admin/platform-admin.ts",
		"lib/admin/platform-admin.schema.ts",
		"lib/admin/platform-admin.schema.test.ts",
		"lib/admin/platform-admin.test.ts",
		"lib/admin/platform-admin.lit.ts",
	],
});
