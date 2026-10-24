import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Verdict summary — Shows the verdict for one checked address.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const VerdictSummaryLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("verdict-summary"),
	domain: z.literal("verification"),
	owner: z.literal("feature"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const VerdictSummaryLit = VerdictSummaryLitSchema.parse({
	kind: "component",
	name: "verdict-summary",
	domain: "verification",
	owner: "feature",
	client: true,
	summary: "Shows the verdict for one checked address.",
	schemas: ["components/features/verification/verdict-summary/verdict-summary.schema.ts"],
	files: [
		"components/features/verification/verdict-summary/verdict-summary.tsx",
		"components/features/verification/verdict-summary/verdict-summary.test.tsx",
		"components/features/verification/verdict-summary/verdict-summary.schema.ts",
		"components/features/verification/verdict-summary/verdict-summary.schema.test.ts",
		"components/features/verification/verdict-summary/verdict-summary.lit.ts",
		"components/features/verification/verdict-summary/verdict-summary.stories.tsx",
	],
});
