import { z } from "zod";

import type { V1GetPipelineRun200Response } from "@/lib/api/generated/zod/pipelines/pipelines";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Pipeline run detail.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `pipeline-run-detail.lit.ts`.
 */

export type PipelineRun = z.infer<typeof V1GetPipelineRun200Response>;

/** The run states that do not change again (`PipelineRunStatus::is_terminal`). */
export const FINAL_RUN_STATES = ["completed", "failed", "cancelled", "skipped"];

export const PipelineRunDetailPropsSchema = z.object({
	pipelineId: z.number().int().positive(),
	runId: z.number().int().positive(),
});

export type PipelineRunDetailPropsFields = z.infer<typeof PipelineRunDetailPropsSchema>;
