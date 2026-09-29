import { z } from "zod";

/**
 * @oppulence-gen kind=lib
 * Server config — One validated read of the server environment, checked at boot.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const ServerConfigLitSchema = z.object({
	kind: z.literal("lib"),
	name: z.literal("server-config"),
	domain: z.literal("config"),
	owner: z.literal("lib"),
	client: z.literal(false),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const ServerConfigLit = ServerConfigLitSchema.parse({
	kind: "lib",
	name: "server-config",
	domain: "config",
	owner: "lib",
	client: false,
	summary: "One validated read of the server environment, checked at boot.",
	schemas: ["lib/config/server-config.schema.ts"],
	files: [
		"lib/config/server-config.ts",
		"lib/config/server-config.schema.ts",
		"lib/config/server-config.schema.test.ts",
		"lib/config/server-config.test.ts",
		"lib/config/server-config.lit.ts",
	],
});
