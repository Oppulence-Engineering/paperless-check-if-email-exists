import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Billing plan — Workspace plan with upgrade and billing portal actions.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const BillingPlanLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("billing-plan"),
	domain: z.literal(""),
	owner: z.literal("route"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const BillingPlanLit = BillingPlanLitSchema.parse({
	kind: "component",
	name: "billing-plan",
	domain: "",
	owner: "route",
	client: true,
	summary: "Workspace plan with upgrade and billing portal actions.",
	schemas: ["app/(product)/app/settings/_components/billing-plan/billing-plan.schema.ts"],
	files: [
		"app/(product)/app/settings/_components/billing-plan/billing-plan.tsx",
		"app/(product)/app/settings/_components/billing-plan/billing-plan.test.tsx",
		"app/(product)/app/settings/_components/billing-plan/billing-plan.schema.ts",
		"app/(product)/app/settings/_components/billing-plan/billing-plan.schema.test.ts",
		"app/(product)/app/settings/_components/billing-plan/billing-plan.lit.ts",
		"app/(product)/app/settings/_components/billing-plan/billing-plan.stories.tsx",
	],
});
