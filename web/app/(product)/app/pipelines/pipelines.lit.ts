import { z } from "zod";

/**
 * @oppulence-gen kind=page
 * Pipelines — Create, run, pause, resume, and inspect verification pipelines
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const PipelinesLitSchema = z.object({
	kind: z.literal("page"),
	name: z.literal("pipelines"),
	domain: z.literal("pipelines"),
	owner: z.literal("page"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const PipelinesLit = PipelinesLitSchema.parse({
	kind: "page",
	name: "pipelines",
	domain: "pipelines",
	owner: "page",
	client: true,
	summary: "Create, run, pause, resume, and inspect verification pipelines",
	schemas: ["app/(product)/app/pipelines/search-params.ts"],
	files: [
		"app/(product)/app/pipelines/pipelines.lit.ts",
		"app/(product)/app/pipelines/page.tsx",
		"app/(product)/app/pipelines/loading.tsx",
		"app/(product)/app/pipelines/error.tsx",
		"app/(product)/app/pipelines/search-params.ts",
	],
});
