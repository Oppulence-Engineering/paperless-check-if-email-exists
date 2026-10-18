import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Provider endpoint form — Creates a paused provider endpoint, or edits the label and the IP allow list of one endpoint.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const ProviderEndpointFormLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("provider-endpoint-form"),
	domain: z.literal("deliverability"),
	owner: z.literal("feature"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const ProviderEndpointFormLit = ProviderEndpointFormLitSchema.parse({
	kind: "component",
	name: "provider-endpoint-form",
	domain: "deliverability",
	owner: "feature",
	client: true,
	summary:
		"Creates a paused provider endpoint, or edits the label and the IP allow list of one endpoint.",
	schemas: [
		"components/features/deliverability/provider-endpoint-form/provider-endpoint-form.schema.ts",
	],
	files: [
		"components/features/deliverability/provider-endpoint-form/provider-endpoint-form.tsx",
		"components/features/deliverability/provider-endpoint-form/provider-endpoint-form.test.tsx",
		"components/features/deliverability/provider-endpoint-form/provider-endpoint-form.schema.ts",
		"components/features/deliverability/provider-endpoint-form/provider-endpoint-form.schema.test.ts",
		"components/features/deliverability/provider-endpoint-form/provider-endpoint-form.lit.ts",
		"components/features/deliverability/provider-endpoint-form/provider-endpoint-form.stories.tsx",
	],
});
