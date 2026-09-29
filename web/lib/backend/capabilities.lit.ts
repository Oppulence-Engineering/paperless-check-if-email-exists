import { z } from "zod";

/**
 * @oppulence-gen kind=lib
 * Capabilities: Validated backend capability manifest with a safe browser-facing projection.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const CapabilitiesLitSchema = z.object({
	kind: z.literal("lib"),
	name: z.literal("capabilities"),
	domain: z.literal("backend"),
	owner: z.literal("lib"),
	client: z.literal(false),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const CapabilitiesLit = CapabilitiesLitSchema.parse({
	kind: "lib",
	name: "capabilities",
	domain: "backend",
	owner: "lib",
	client: false,
	summary: "Validated backend capability manifest with a safe browser-facing projection.",
	schemas: ["lib/backend/capabilities.schema.ts"],
	files: [
		"lib/backend/capabilities.ts",
		"lib/backend/capabilities.schema.ts",
		"lib/backend/capabilities.schema.test.ts",
		"lib/backend/capabilities.test.ts",
		"lib/backend/capabilities.lit.ts",
	],
});
