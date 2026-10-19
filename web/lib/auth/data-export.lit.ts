import { z } from "zod";

/**
 * @oppulence-gen kind=lib
 * Data export — Everything this application holds about one person, as one JSON document.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const DataExportLitSchema = z.object({
	kind: z.literal("lib"),
	name: z.literal("data-export"),
	domain: z.literal("auth"),
	owner: z.literal("lib"),
	client: z.literal(false),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const DataExportLit = DataExportLitSchema.parse({
	kind: "lib",
	name: "data-export",
	domain: "auth",
	owner: "lib",
	client: false,
	summary: "Everything this application holds about one person, as one JSON document.",
	schemas: ["lib/auth/data-export.schema.ts"],
	files: [
		"lib/auth/data-export.ts",
		"lib/auth/data-export.schema.ts",
		"lib/auth/data-export.schema.test.ts",
		"lib/auth/data-export.test.ts",
		"lib/auth/data-export.lit.ts",
	],
});
