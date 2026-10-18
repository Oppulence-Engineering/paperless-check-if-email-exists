import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Pipeline runs.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `pipeline-runs.lit.ts`.
 */
export const PipelineRunsPropsSchema = z.object({
	pipelineId: z.number().int().positive(),
	selectedRunId: z.number().int().positive().optional(),
});

export type PipelineRunsPropsFields = z.infer<typeof PipelineRunsPropsSchema>;
