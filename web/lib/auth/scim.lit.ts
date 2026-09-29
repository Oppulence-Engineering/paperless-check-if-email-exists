import { z } from "zod";

/**
 * @oppulence-gen kind=lib
 * Scim — Better Auth managed SCIM connections and organization projection.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const ScimLitSchema = z.object({
	kind: z.literal("lib"),
	name: z.literal("scim"),
	domain: z.literal("auth"),
	owner: z.literal("lib"),
	client: z.literal(false),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const ScimLit = ScimLitSchema.parse({
	kind: "lib",
	name: "scim",
	domain: "auth",
	owner: "lib",
	client: false,
	summary: "Better Auth managed SCIM connections and organization projection.",
	schemas: ["lib/auth/scim.schema.ts"],
	files: [
		"lib/auth/scim.ts",
		"lib/auth/scim.schema.ts",
		"lib/auth/scim.schema.test.ts",
		"lib/auth/scim.test.ts",
		"lib/auth/scim.lit.ts",
	],
});
