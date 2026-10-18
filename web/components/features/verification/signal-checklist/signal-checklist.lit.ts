import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Signal checklist — Shared verification view: signal-checklist.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const SignalChecklistLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("signal-checklist"),
	domain: z.literal("verification"),
	owner: z.literal("feature"),
	client: z.literal(false),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const SignalChecklistLit = SignalChecklistLitSchema.parse({
	kind: "component",
	name: "signal-checklist",
	domain: "verification",
	owner: "feature",
	client: false,
	summary: "Shared verification view: signal-checklist.",
	schemas: ["components/features/verification/signal-checklist/signal-checklist.schema.ts"],
	files: [
		"components/features/verification/signal-checklist/signal-checklist.tsx",
		"components/features/verification/signal-checklist/signal-checklist.test.tsx",
		"components/features/verification/signal-checklist/signal-checklist.schema.ts",
		"components/features/verification/signal-checklist/signal-checklist.schema.test.ts",
		"components/features/verification/signal-checklist/signal-checklist.lit.ts",
		"components/features/verification/signal-checklist/signal-checklist.stories.tsx",
	],
});
