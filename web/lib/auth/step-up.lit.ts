import { z } from "zod";

/**
 * @oppulence-gen kind=lib
 * Step up — Validates recent, purpose-bound identity verification grants.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const StepUpLitSchema = z.object({
	kind: z.literal("lib"),
	name: z.literal("step-up"),
	domain: z.literal("auth"),
	owner: z.literal("lib"),
	client: z.literal(false),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const StepUpLit = StepUpLitSchema.parse({
	kind: "lib",
	name: "step-up",
	domain: "auth",
	owner: "lib",
	client: false,
	summary: "Validates recent, purpose-bound identity verification grants.",
	schemas: ["lib/auth/step-up.schema.ts"],
	files: [
		"lib/auth/step-up.ts",
		"lib/auth/step-up.schema.ts",
		"lib/auth/step-up.schema.test.ts",
		"lib/auth/step-up.test.ts",
		"lib/auth/step-up.lit.ts",
	],
});
