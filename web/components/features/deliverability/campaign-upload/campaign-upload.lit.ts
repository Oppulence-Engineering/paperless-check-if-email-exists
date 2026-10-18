import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Campaign upload — Uploads a CSV file of campaign results and shows the counts and the row errors.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const CampaignUploadLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("campaign-upload"),
	domain: z.literal("deliverability"),
	owner: z.literal("feature"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const CampaignUploadLit = CampaignUploadLitSchema.parse({
	kind: "component",
	name: "campaign-upload",
	domain: "deliverability",
	owner: "feature",
	client: true,
	summary: "Uploads a CSV file of campaign results and shows the counts and the row errors.",
	schemas: ["components/features/deliverability/campaign-upload/campaign-upload.schema.ts"],
	files: [
		"components/features/deliverability/campaign-upload/campaign-upload.tsx",
		"components/features/deliverability/campaign-upload/campaign-upload.test.tsx",
		"components/features/deliverability/campaign-upload/campaign-upload.schema.ts",
		"components/features/deliverability/campaign-upload/campaign-upload.schema.test.ts",
		"components/features/deliverability/campaign-upload/campaign-upload.lit.ts",
		"components/features/deliverability/campaign-upload/campaign-upload.stories.tsx",
	],
});
