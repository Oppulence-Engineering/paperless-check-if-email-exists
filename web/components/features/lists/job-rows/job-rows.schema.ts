import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Job rows.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `job-rows.lit.ts`.
 */
export const JobRowsPropsSchema = z.object({
	jobId: z.number().int().positive(),
});

export type JobRowsPropsFields = z.infer<typeof JobRowsPropsSchema>;
