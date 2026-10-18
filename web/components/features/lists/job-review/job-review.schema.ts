import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Job review.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `job-review.lit.ts`.
 */
export const JobReviewPropsSchema = z.object({
	jobId: z.number().int().positive(),
});

export type JobReviewPropsFields = z.infer<typeof JobReviewPropsSchema>;
