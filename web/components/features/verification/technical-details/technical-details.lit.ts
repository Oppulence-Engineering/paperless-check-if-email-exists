import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Technical details — Shared verification view: technical-details.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const TechnicalDetailsLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("technical-details"),
	domain: z.literal("verification"),
	owner: z.literal("feature"),
	client: z.literal(false),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const TechnicalDetailsLit = TechnicalDetailsLitSchema.parse({
	kind: "component",
	name: "technical-details",
	domain: "verification",
	owner: "feature",
	client: false,
	summary: "Shared verification view: technical-details.",
	schemas: ["components/features/verification/technical-details/technical-details.schema.ts"],
	files: [
		"components/features/verification/technical-details/technical-details.tsx",
		"components/features/verification/technical-details/technical-details.test.tsx",
		"components/features/verification/technical-details/technical-details.schema.ts",
		"components/features/verification/technical-details/technical-details.schema.test.ts",
		"components/features/verification/technical-details/technical-details.lit.ts",
		"components/features/verification/technical-details/technical-details.stories.tsx",
	],
});
