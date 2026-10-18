import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Job problems — Shows the failures, retries, and event log for one bulk job.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const JobProblemsLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("job-problems"),
	domain: z.literal("lists"),
	owner: z.literal("feature"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const JobProblemsLit = JobProblemsLitSchema.parse({
	kind: "component",
	name: "job-problems",
	domain: "lists",
	owner: "feature",
	client: true,
	summary: "Shows the failures, retries, and event log for one bulk job.",
	schemas: ["components/features/lists/job-problems/job-problems.schema.ts"],
	files: [
		"components/features/lists/job-problems/job-problems.tsx",
		"components/features/lists/job-problems/job-problems.test.tsx",
		"components/features/lists/job-problems/job-problems.schema.ts",
		"components/features/lists/job-problems/job-problems.schema.test.ts",
		"components/features/lists/job-problems/job-problems.lit.ts",
		"components/features/lists/job-problems/job-problems.stories.tsx",
	],
});
