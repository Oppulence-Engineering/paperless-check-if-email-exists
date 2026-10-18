import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Campaign outcome list — Shows the campaign results with filters and pages of 20 rows.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const CampaignOutcomeListLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("campaign-outcome-list"),
	domain: z.literal("deliverability"),
	owner: z.literal("feature"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const CampaignOutcomeListLit = CampaignOutcomeListLitSchema.parse({
	kind: "component",
	name: "campaign-outcome-list",
	domain: "deliverability",
	owner: "feature",
	client: true,
	summary: "Shows the campaign results with filters and pages of 20 rows.",
	schemas: [
		"components/features/deliverability/campaign-outcome-list/campaign-outcome-list.schema.ts",
	],
	files: [
		"components/features/deliverability/campaign-outcome-list/campaign-outcome-list.tsx",
		"components/features/deliverability/campaign-outcome-list/campaign-outcome-list.test.tsx",
		"components/features/deliverability/campaign-outcome-list/campaign-outcome-list.schema.ts",
		"components/features/deliverability/campaign-outcome-list/campaign-outcome-list.schema.test.ts",
		"components/features/deliverability/campaign-outcome-list/campaign-outcome-list.lit.ts",
		"components/features/deliverability/campaign-outcome-list/campaign-outcome-list.stories.tsx",
	],
});
