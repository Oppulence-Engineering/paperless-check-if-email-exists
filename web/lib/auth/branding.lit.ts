import { z } from "zod";

/**
 * @oppulence-gen kind=lib
 * Branding — Validated deployment branding with server-loaded organization overrides.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const BrandingLitSchema = z.object({
	kind: z.literal("lib"),
	name: z.literal("branding"),
	domain: z.literal("auth"),
	owner: z.literal("lib"),
	client: z.literal(false),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const BrandingLit = BrandingLitSchema.parse({
	kind: "lib",
	name: "branding",
	domain: "auth",
	owner: "lib",
	client: false,
	summary: "Validated deployment branding with server-loaded organization overrides.",
	schemas: ["lib/auth/branding.schema.ts"],
	files: [
		"lib/auth/branding.ts",
		"lib/auth/branding.schema.ts",
		"lib/auth/branding.schema.test.ts",
		"lib/auth/branding.test.ts",
		"lib/auth/branding.lit.ts",
	],
});
