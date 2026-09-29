import { z } from "zod";

/**
 * @oppulence-gen kind=lib
 * Identity audit — Append-only identity event recording without credential material.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const IdentityAuditLitSchema = z.object({
	kind: z.literal("lib"),
	name: z.literal("identity-audit"),
	domain: z.literal("auth"),
	owner: z.literal("lib"),
	client: z.literal(false),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const IdentityAuditLit = IdentityAuditLitSchema.parse({
	kind: "lib",
	name: "identity-audit",
	domain: "auth",
	owner: "lib",
	client: false,
	summary: "Append-only identity event recording without credential material.",
	schemas: ["lib/auth/identity-audit.schema.ts"],
	files: [
		"lib/auth/identity-audit.ts",
		"lib/auth/identity-audit.schema.ts",
		"lib/auth/identity-audit.schema.test.ts",
		"lib/auth/identity-audit.test.ts",
		"lib/auth/identity-audit.lit.ts",
	],
});
