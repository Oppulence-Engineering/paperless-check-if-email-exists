import { z } from "zod";

/**
 * @oppulence-gen kind=page
 * Jobs — Create bulk jobs and inspect progress, results, approvals, and failures
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const JobsLitSchema = z.object({
	kind: z.literal("page"),
	name: z.literal("jobs"),
	domain: z.literal("jobs"),
	owner: z.literal("page"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const JobsLit = JobsLitSchema.parse({
	kind: "page",
	name: "jobs",
	domain: "jobs",
	owner: "page",
	client: true,
	summary: "Create bulk jobs and inspect progress, results, approvals, and failures",
	schemas: ["app/(product)/app/jobs/search-params.ts"],
	files: [
		"app/(product)/app/jobs/jobs.lit.ts",
		"app/(product)/app/jobs/page.tsx",
		"app/(product)/app/jobs/loading.tsx",
		"app/(product)/app/jobs/error.tsx",
		"app/(product)/app/jobs/search-params.ts",
	],
});
