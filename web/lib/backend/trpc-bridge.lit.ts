import { z } from "zod";

/**
 * @oppulence-gen kind=lib
 * Trpc bridge — Authenticated JSON tRPC transport that reuses the backend BFF.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const TrpcBridgeLitSchema = z.object({
	kind: z.literal("lib"),
	name: z.literal("trpc-bridge"),
	domain: z.literal("backend"),
	owner: z.literal("lib"),
	client: z.literal(false),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const TrpcBridgeLit = TrpcBridgeLitSchema.parse({
	kind: "lib",
	name: "trpc-bridge",
	domain: "backend",
	owner: "lib",
	client: false,
	summary: "Authenticated JSON tRPC transport that reuses the backend BFF.",
	schemas: ["lib/backend/trpc-bridge.schema.ts"],
	files: [
		"lib/backend/trpc-bridge.ts",
		"lib/backend/trpc-bridge.schema.ts",
		"lib/backend/trpc-bridge.schema.test.ts",
		"lib/backend/trpc-bridge.test.ts",
		"lib/backend/trpc-bridge.lit.ts",
	],
});
