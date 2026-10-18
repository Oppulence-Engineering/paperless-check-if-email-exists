import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Overview panel — Signed-in home: alerts, usage, lists, deliverability, automation, and setup steps.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const OverviewPanelLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("overview-panel"),
	domain: z.literal(""),
	owner: z.literal("route"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const OverviewPanelLit = OverviewPanelLitSchema.parse({
	kind: "component",
	name: "overview-panel",
	domain: "",
	owner: "route",
	client: true,
	summary: "Signed-in home: alerts, usage, lists, deliverability, automation, and setup steps.",
	schemas: ["app/(product)/app/_components/overview-panel/overview-panel.schema.ts"],
	files: [
		"app/(product)/app/_components/overview-panel/overview-panel.tsx",
		"app/(product)/app/_components/overview-panel/overview-panel.test.tsx",
		"app/(product)/app/_components/overview-panel/overview-panel.schema.ts",
		"app/(product)/app/_components/overview-panel/overview-panel.schema.test.ts",
		"app/(product)/app/_components/overview-panel/overview-panel.lit.ts",
		"app/(product)/app/_components/overview-panel/overview-panel.stories.tsx",
	],
});
