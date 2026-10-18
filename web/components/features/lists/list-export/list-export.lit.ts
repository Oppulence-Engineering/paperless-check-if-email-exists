import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * List export — Downloads a cleaned list, builds the remediation plan, and exports its rows.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const ListExportLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("list-export"),
	domain: z.literal("lists"),
	owner: z.literal("feature"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const ListExportLit = ListExportLitSchema.parse({
	kind: "component",
	name: "list-export",
	domain: "lists",
	owner: "feature",
	client: true,
	summary: "Downloads a cleaned list, builds the remediation plan, and exports its rows.",
	schemas: ["components/features/lists/list-export/list-export.schema.ts"],
	files: [
		"components/features/lists/list-export/list-export.tsx",
		"components/features/lists/list-export/list-export.test.tsx",
		"components/features/lists/list-export/list-export.schema.ts",
		"components/features/lists/list-export/list-export.schema.test.ts",
		"components/features/lists/list-export/list-export.lit.ts",
		"components/features/lists/list-export/list-export.stories.tsx",
	],
});
