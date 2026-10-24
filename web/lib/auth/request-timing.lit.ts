import { z } from "zod";

/**
 * @oppulence-gen kind=lib
 * Request timing — Request-scoped timings without identity or URL data
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const RequestTimingLitSchema = z.object({
	kind: z.literal("lib"),
	name: z.literal("request-timing"),
	domain: z.literal("auth"),
	owner: z.literal("lib"),
	client: z.literal(false),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const RequestTimingLit = RequestTimingLitSchema.parse({
	kind: "lib",
	name: "request-timing",
	domain: "auth",
	owner: "lib",
	client: false,
	summary: "Request-scoped timings without identity or URL data",
	schemas: ["lib/auth/request-timing.schema.ts"],
	files: [
		"lib/auth/request-timing.ts",
		"lib/auth/request-timing.schema.ts",
		"lib/auth/request-timing.schema.test.ts",
		"lib/auth/request-timing.test.ts",
		"lib/auth/request-timing.lit.ts",
	],
});
