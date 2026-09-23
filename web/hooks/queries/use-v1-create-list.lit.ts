import { z } from "zod";

/**
 * @oppulence-gen kind=mutation
 * V1 create list — Upload an email list
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const V1CreateListLitSchema = z.object({
	kind: z.literal("mutation"),
	name: z.literal("v1-create-list"),
	domain: z.literal(""),
	owner: z.literal("hook"),
	client: z.literal(false),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const V1CreateListLit = V1CreateListLitSchema.parse({
	kind: "mutation",
	name: "v1-create-list",
	domain: "",
	owner: "hook",
	client: false,
	summary: "Upload an email list",
	schemas: [],
	files: [
		"hooks/queries/use-v1-create-list.ts",
		"hooks/queries/utils/mutate-v1-create-list.ts",
		"hooks/queries/utils/v1-create-list-keys.ts",
		"hooks/queries/use-v1-create-list.lit.ts",
	],
});
