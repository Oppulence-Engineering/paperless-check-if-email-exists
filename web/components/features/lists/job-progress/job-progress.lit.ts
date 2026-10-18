import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Job progress — Shows the progress bar and time left for one running bulk job.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const JobProgressLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("job-progress"),
	domain: z.literal("lists"),
	owner: z.literal("feature"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const JobProgressLit = JobProgressLitSchema.parse({
	kind: "component",
	name: "job-progress",
	domain: "lists",
	owner: "feature",
	client: true,
	summary: "Shows the progress bar and time left for one running bulk job.",
	schemas: ["components/features/lists/job-progress/job-progress.schema.ts"],
	files: [
		"components/features/lists/job-progress/job-progress.tsx",
		"components/features/lists/job-progress/job-progress.test.tsx",
		"components/features/lists/job-progress/job-progress.schema.ts",
		"components/features/lists/job-progress/job-progress.schema.test.ts",
		"components/features/lists/job-progress/job-progress.lit.ts",
		"components/features/lists/job-progress/job-progress.stories.tsx",
	],
});
