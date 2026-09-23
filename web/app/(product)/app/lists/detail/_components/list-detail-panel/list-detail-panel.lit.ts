import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * List quality, remediation, exports, and team comments.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const ListDetailPanelLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("list-detail-panel"),
	domain: z.literal(""),
	owner: z.literal("route"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const ListDetailPanelLit = ListDetailPanelLitSchema.parse({
	kind: "component",
	name: "list-detail-panel",
	domain: "",
	owner: "route",
	client: true,
	summary: "List quality, remediation, exports, and team comments.",
	schemas: [
		"app/(product)/app/lists/detail/_components/list-detail-panel/list-detail-panel.schema.ts",
	],
	files: [
		"app/(product)/app/lists/detail/_components/list-detail-panel/list-detail-panel.tsx",
		"app/(product)/app/lists/detail/_components/list-detail-panel/list-detail-panel.test.tsx",
		"app/(product)/app/lists/detail/_components/list-detail-panel/list-detail-panel.schema.ts",
		"app/(product)/app/lists/detail/_components/list-detail-panel/list-detail-panel.schema.test.ts",
		"app/(product)/app/lists/detail/_components/list-detail-panel/list-detail-panel.lit.ts",
		"app/(product)/app/lists/detail/_components/list-detail-panel/list-detail-panel.stories.tsx",
	],
});
