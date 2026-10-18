import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Domain health — Checks the reputation of one domain and shows the result.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const DomainHealthLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("domain-health"),
	domain: z.literal("deliverability"),
	owner: z.literal("feature"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const DomainHealthLit = DomainHealthLitSchema.parse({
	kind: "component",
	name: "domain-health",
	domain: "deliverability",
	owner: "feature",
	client: true,
	summary: "Checks the reputation of one domain and shows the result.",
	schemas: ["components/features/deliverability/domain-health/domain-health.schema.ts"],
	files: [
		"components/features/deliverability/domain-health/domain-health.tsx",
		"components/features/deliverability/domain-health/domain-health.test.tsx",
		"components/features/deliverability/domain-health/domain-health.schema.ts",
		"components/features/deliverability/domain-health/domain-health.schema.test.ts",
		"components/features/deliverability/domain-health/domain-health.lit.ts",
		"components/features/deliverability/domain-health/domain-health.stories.tsx",
	],
});
