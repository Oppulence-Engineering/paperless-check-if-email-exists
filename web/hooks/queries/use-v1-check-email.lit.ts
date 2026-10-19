import { z } from "zod";

/**
 * @oppulence-gen kind=mutation
 * V1 check email — Mutation through the generated SDK and same-origin backend route.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const V1CheckEmailLitSchema = z.object({
	kind: z.literal("mutation"),
	name: z.literal("v1-check-email"),
	domain: z.literal(""),
	owner: z.literal("hook"),
	client: z.literal(false),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const V1CheckEmailLit = V1CheckEmailLitSchema.parse({
	kind: "mutation",
	name: "v1-check-email",
	domain: "",
	owner: "hook",
	client: false,
	summary: "Mutation through the generated SDK and same-origin backend route.",
	schemas: [],
	files: [
		"hooks/queries/use-v1-check-email.ts",
		"hooks/queries/utils/mutate-v1-check-email.ts",
		"hooks/queries/use-v1-check-email.lit.ts",
	],
});
