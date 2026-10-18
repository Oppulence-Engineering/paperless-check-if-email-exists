import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Job review — Shows the send readiness checklist for one bulk job.
 *
 * Living Interface Template. This file is the contract, not the
 * implementation. Implementation lives in the siblings listed in `files`.
 */
export const JobReviewLitSchema = z.object({
	kind: z.literal("component"),
	name: z.literal("job-review"),
	domain: z.literal("lists"),
	owner: z.literal("feature"),
	client: z.literal(true),
	summary: z.string().min(1),
	schemas: z.array(z.string()),
	files: z.array(z.string()),
});

export const JobReviewLit = JobReviewLitSchema.parse({
	kind: "component",
	name: "job-review",
	domain: "lists",
	owner: "feature",
	client: true,
	summary: "Shows the send readiness checklist for one bulk job.",
	schemas: ["components/features/lists/job-review/job-review.schema.ts"],
	files: [
		"components/features/lists/job-review/job-review.tsx",
		"components/features/lists/job-review/job-review.test.tsx",
		"components/features/lists/job-review/job-review.schema.ts",
		"components/features/lists/job-review/job-review.schema.test.ts",
		"components/features/lists/job-review/job-review.lit.ts",
		"components/features/lists/job-review/job-review.stories.tsx",
	],
});
