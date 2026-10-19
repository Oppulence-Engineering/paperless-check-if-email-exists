import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Outcome events — Shows the provider events with filters and pages of 20 rows.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const OutcomeEventsLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("outcome-events"),
	domain: z.literal("deliverability"),
	owner: z.literal("feature"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const OutcomeEventsLit = OutcomeEventsLitSchema.parse({
	kind: "component",
	name: "outcome-events",
	domain: "deliverability",
	owner: "feature",
	client: true,
	summary: "Shows the provider events with filters and pages of 20 rows.",
	schemas: ["components/features/deliverability/outcome-events/outcome-events.schema.ts"],
	files: [
		"components/features/deliverability/outcome-events/outcome-events.tsx",
		"components/features/deliverability/outcome-events/outcome-events.test.tsx",
		"components/features/deliverability/outcome-events/outcome-events.schema.ts",
		"components/features/deliverability/outcome-events/outcome-events.schema.test.ts",
		"components/features/deliverability/outcome-events/outcome-events.lit.ts",
		"components/features/deliverability/outcome-events/outcome-events.stories.tsx",
	],
});
