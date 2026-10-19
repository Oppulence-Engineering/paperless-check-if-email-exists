import { z } from "zod";

/**
 * @oppulence-gen kind=lib
 * Entitlements — What a plan buys: features, seats, and the reason a capability was refused.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const EntitlementsLitSchema = z.object({
	kind: z.literal("lib"),
	name: z.literal("entitlements"),
	domain: z.literal("entitlements"),
	owner: z.literal("lib"),
	client: z.literal(false),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const EntitlementsLit = EntitlementsLitSchema.parse({
	kind: "lib",
	name: "entitlements",
	domain: "entitlements",
	owner: "lib",
	client: false,
	summary: "What a plan buys: features, seats, and the reason a capability was refused.",
	schemas: ["lib/entitlements/entitlements.schema.ts"],
	files: [
		"lib/entitlements/entitlements.ts",
		"lib/entitlements/entitlements.schema.ts",
		"lib/entitlements/entitlements.schema.test.ts",
		"lib/entitlements/entitlements.test.ts",
		"lib/entitlements/entitlements.lit.ts",
	],
});
