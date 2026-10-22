import { z } from "zod";

/**
 * @oppulence-gen kind=hook
 * Job status — Shared job status and one visible polling owner
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const JobStatusLitSchema = z.object({
	kind: z.literal("hook"),
	name: z.literal("job-status"),
	domain: z.literal(""),
	owner: z.literal("hook"),
	client: z.literal(false),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const JobStatusLit = JobStatusLitSchema.parse({
	kind: "hook",
	name: "job-status",
	domain: "",
	owner: "hook",
	client: false,
	summary: "Shared job status and one visible polling owner",
	schemas: [],
	files: [
		"hooks/queries/use-job-status.ts",
		"hooks/queries/utils/fetch-job-status.ts",
		"hooks/queries/utils/fetch-job-status.test.ts",
		"hooks/queries/utils/prefetch-job-status.ts",
		"hooks/queries/utils/job-status-keys.ts",
		"hooks/queries/utils/job-status-keys.test.ts",
		"hooks/queries/use-job-status.lit.ts",
	],
});
