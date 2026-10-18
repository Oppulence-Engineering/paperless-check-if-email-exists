import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Provider details — Shared verification view: provider-details.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const ProviderDetailsLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("provider-details"),
	domain: z.literal("verification"),
	owner: z.literal("feature"),
	client: z.literal(false),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const ProviderDetailsLit = ProviderDetailsLitSchema.parse({
	kind: "component",
	name: "provider-details",
	domain: "verification",
	owner: "feature",
	client: false,
	summary: "Shared verification view: provider-details.",
	schemas: ["components/features/verification/provider-details/provider-details.schema.ts"],
	files: [
		"components/features/verification/provider-details/provider-details.tsx",
		"components/features/verification/provider-details/provider-details.test.tsx",
		"components/features/verification/provider-details/provider-details.schema.ts",
		"components/features/verification/provider-details/provider-details.schema.test.ts",
		"components/features/verification/provider-details/provider-details.lit.ts",
		"components/features/verification/provider-details/provider-details.stories.tsx",
	],
});
