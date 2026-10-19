import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Job problems.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `job-problems.lit.ts`.
 */
export const JobProblemsPropsSchema = z.object({
	jobId: z.number().int().positive(),
});

export type JobProblemsPropsFields = z.infer<typeof JobProblemsPropsSchema>;
