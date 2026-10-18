import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Provider endpoint list — Lists the provider endpoints and changes their status, token, label, and IP allow list.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const ProviderEndpointListLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("provider-endpoint-list"),
	domain: z.literal("deliverability"),
	owner: z.literal("feature"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const ProviderEndpointListLit = ProviderEndpointListLitSchema.parse({
	kind: "component",
	name: "provider-endpoint-list",
	domain: "deliverability",
	owner: "feature",
	client: true,
	summary:
		"Lists the provider endpoints and changes their status, token, label, and IP allow list.",
	schemas: [
		"components/features/deliverability/provider-endpoint-list/provider-endpoint-list.schema.ts",
	],
	files: [
		"components/features/deliverability/provider-endpoint-list/provider-endpoint-list.tsx",
		"components/features/deliverability/provider-endpoint-list/provider-endpoint-list.test.tsx",
		"components/features/deliverability/provider-endpoint-list/provider-endpoint-list.schema.ts",
		"components/features/deliverability/provider-endpoint-list/provider-endpoint-list.schema.test.ts",
		"components/features/deliverability/provider-endpoint-list/provider-endpoint-list.lit.ts",
		"components/features/deliverability/provider-endpoint-list/provider-endpoint-list.stories.tsx",
	],
});
