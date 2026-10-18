import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Job progress.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `job-progress.lit.ts`.
 */
export const JobProgressPropsSchema = z.object({
	jobId: z.number().int().positive(),
});

export type JobProgressPropsFields = z.infer<typeof JobProgressPropsSchema>;
