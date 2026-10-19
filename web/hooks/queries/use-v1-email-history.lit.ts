import { z } from "zod";

/**
 * @oppulence-gen kind=hook
 * V1 email history — Read verification history for an address
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const V1EmailHistoryLitSchema = z.object({
	kind: z.literal("hook"),
	name: z.literal("v1-email-history"),
	domain: z.literal(""),
	owner: z.literal("hook"),
	client: z.literal(false),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const V1EmailHistoryLit = V1EmailHistoryLitSchema.parse({
	kind: "hook",
	name: "v1-email-history",
	domain: "",
	owner: "hook",
	client: false,
	summary: "Read verification history for an address",
	schemas: [],
	files: [
		"hooks/queries/use-v1-email-history.ts",
		"hooks/queries/utils/fetch-v1-email-history.ts",
		"hooks/queries/utils/v1-email-history-keys.ts",
		"hooks/queries/use-v1-email-history.lit.ts",
	],
});
