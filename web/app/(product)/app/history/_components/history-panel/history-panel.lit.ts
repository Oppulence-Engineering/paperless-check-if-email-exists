import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * History panel — Verification history route-private panel. Presentation only.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const HistoryPanelLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("history-panel"),
	domain: z.literal(""),
	owner: z.literal("route"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const HistoryPanelLit = HistoryPanelLitSchema.parse({
	kind: "component",
	name: "history-panel",
	domain: "",
	owner: "route",
	client: true,
	summary: "Verification history route-private panel. Presentation only.",
	schemas: ["app/(product)/app/history/_components/history-panel/history-panel.schema.ts"],
	files: [
		"app/(product)/app/history/_components/history-panel/history-panel.tsx",
		"app/(product)/app/history/_components/history-panel/history-panel.test.tsx",
		"app/(product)/app/history/_components/history-panel/history-panel.schema.ts",
		"app/(product)/app/history/_components/history-panel/history-panel.schema.test.ts",
		"app/(product)/app/history/_components/history-panel/history-panel.lit.ts",
		"app/(product)/app/history/_components/history-panel/history-panel.stories.tsx",
	],
});
