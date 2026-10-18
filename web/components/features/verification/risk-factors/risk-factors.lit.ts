import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Risk factors — Shared verification view: risk-factors.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const RiskFactorsLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("risk-factors"),
	domain: z.literal("verification"),
	owner: z.literal("feature"),
	client: z.literal(false),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const RiskFactorsLit = RiskFactorsLitSchema.parse({
	kind: "component",
	name: "risk-factors",
	domain: "verification",
	owner: "feature",
	client: false,
	summary: "Shared verification view: risk-factors.",
	schemas: ["components/features/verification/risk-factors/risk-factors.schema.ts"],
	files: [
		"components/features/verification/risk-factors/risk-factors.tsx",
		"components/features/verification/risk-factors/risk-factors.test.tsx",
		"components/features/verification/risk-factors/risk-factors.schema.ts",
		"components/features/verification/risk-factors/risk-factors.schema.test.ts",
		"components/features/verification/risk-factors/risk-factors.lit.ts",
		"components/features/verification/risk-factors/risk-factors.stories.tsx",
	],
});
