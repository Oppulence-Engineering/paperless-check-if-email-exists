import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Pipeline detail.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `pipeline-detail.lit.ts`.
 */
export const PipelineDetailPropsSchema = z.object({
	pipelineId: z.number().int().positive(),
});

export type PipelineDetailPropsFields = z.infer<typeof PipelineDetailPropsSchema>;
