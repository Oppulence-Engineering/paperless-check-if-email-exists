import { z } from "zod";

/**
 * @oppulence-gen kind=lib
 * Logger — Structured single-line logs carrying the request id.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const LoggerLitSchema = z.object({
	kind: z.literal("lib"),
	name: z.literal("logger"),
	domain: z.literal("observability"),
	owner: z.literal("lib"),
	client: z.literal(false),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const LoggerLit = LoggerLitSchema.parse({
	kind: "lib",
	name: "logger",
	domain: "observability",
	owner: "lib",
	client: false,
	summary: "Structured single-line logs carrying the request id.",
	schemas: ["lib/observability/logger.schema.ts"],
	files: [
		"lib/observability/logger.ts",
		"lib/observability/logger.schema.ts",
		"lib/observability/logger.schema.test.ts",
		"lib/observability/logger.test.ts",
		"lib/observability/logger.lit.ts",
	],
});
