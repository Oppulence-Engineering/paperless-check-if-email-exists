import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Address timeline — Shows earlier results, delivery events, and suppression status for one address.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const AddressTimelineLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("address-timeline"),
	domain: z.literal("verification"),
	owner: z.literal("feature"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const AddressTimelineLit = AddressTimelineLitSchema.parse({
	kind: "component",
	name: "address-timeline",
	domain: "verification",
	owner: "feature",
	client: true,
	summary: "Shows earlier results, delivery events, and suppression status for one address.",
	schemas: ["components/features/verification/address-timeline/address-timeline.schema.ts"],
	files: [
		"components/features/verification/address-timeline/address-timeline.tsx",
		"components/features/verification/address-timeline/address-timeline.test.tsx",
		"components/features/verification/address-timeline/address-timeline.schema.ts",
		"components/features/verification/address-timeline/address-timeline.schema.test.ts",
		"components/features/verification/address-timeline/address-timeline.lit.ts",
		"components/features/verification/address-timeline/address-timeline.stories.tsx",
	],
});
