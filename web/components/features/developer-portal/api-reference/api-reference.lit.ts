import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Api reference — Shows an OpenAPI document with Scalar: sidebar, search, models, code samples, and a request client that calls this host only.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const ApiReferenceLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("api-reference"),
	domain: z.literal("developer-portal"),
	owner: z.literal("feature"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const ApiReferenceLit = ApiReferenceLitSchema.parse({
	kind: "component",
	name: "api-reference",
	domain: "developer-portal",
	owner: "feature",
	client: true,
	summary:
		"Shows an OpenAPI document with Scalar: sidebar, search, models, code samples, and a request client that calls this host only.",
	schemas: ["components/features/developer-portal/api-reference/api-reference.schema.ts"],
	files: [
		"components/features/developer-portal/api-reference/api-reference.tsx",
		"components/features/developer-portal/api-reference/api-reference.test.tsx",
		"components/features/developer-portal/api-reference/api-reference.schema.ts",
		"components/features/developer-portal/api-reference/api-reference.schema.test.ts",
		"components/features/developer-portal/api-reference/api-reference.lit.ts",
		"components/features/developer-portal/api-reference/api-reference.stories.tsx",
	],
});
