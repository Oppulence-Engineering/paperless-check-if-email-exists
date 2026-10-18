import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Pipeline list.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `pipeline-list.lit.ts`.
 */
export const PipelineListPropsSchema = z.object({
	selectedId: z.number().int().positive().optional(),
});

export type PipelineListPropsFields = z.infer<typeof PipelineListPropsSchema>;
