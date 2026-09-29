import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Pipelines panel.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `pipelines-panel.lit.ts`.
 */
export const PipelinesPanelPropsSchema = z.object({});

export type PipelinesPanelPropsFields = z.infer<typeof PipelinesPanelPropsSchema>;
