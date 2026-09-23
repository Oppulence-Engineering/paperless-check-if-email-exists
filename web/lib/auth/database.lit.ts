import { z } from "zod";

/**
 * @oppulence-gen kind=lib
 * Database — PostgreSQL connection shared by Better Auth and readiness checks.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const DatabaseLitSchema = z.object({
	kind: z.literal("lib"),
	name: z.literal("database"),
	domain: z.literal("auth"),
	owner: z.literal("lib"),
	client: z.literal(false),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const DatabaseLit = DatabaseLitSchema.parse({
	kind: "lib",
	name: "database",
	domain: "auth",
	owner: "lib",
	client: false,
	summary: "PostgreSQL connection shared by Better Auth and readiness checks.",
	schemas: ["lib/auth/database.schema.ts"],
	files: [
		"lib/auth/database.ts",
		"lib/auth/database.schema.ts",
		"lib/auth/database.schema.test.ts",
		"lib/auth/database.test.ts",
		"lib/auth/database.lit.ts",
	],
});
