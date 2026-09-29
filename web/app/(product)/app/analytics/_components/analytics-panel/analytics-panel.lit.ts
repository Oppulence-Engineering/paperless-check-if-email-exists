import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Inspect verification results, activity, source quality, and domain risk.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const AnalyticsPanelLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("analytics-panel"),
	domain: z.literal(""),
	owner: z.literal("route"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const AnalyticsPanelLit = AnalyticsPanelLitSchema.parse({
	kind: "component",
	name: "analytics-panel",
	domain: "",
	owner: "route",
	client: true,
	summary: "Inspect verification results, activity, source quality, and domain risk.",
	schemas: ["app/(product)/app/analytics/_components/analytics-panel/analytics-panel.schema.ts"],
	files: [
		"app/(product)/app/analytics/_components/analytics-panel/analytics-panel.tsx",
		"app/(product)/app/analytics/_components/analytics-panel/analytics-panel.test.tsx",
		"app/(product)/app/analytics/_components/analytics-panel/analytics-panel.schema.ts",
		"app/(product)/app/analytics/_components/analytics-panel/analytics-panel.schema.test.ts",
		"app/(product)/app/analytics/_components/analytics-panel/analytics-panel.lit.ts",
		"app/(product)/app/analytics/_components/analytics-panel/analytics-panel.stories.tsx",
	],
});
