import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Source quality.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `source-quality.lit.ts`.
 */
export const SourceQualityPropsSchema = z.object({});

export type SourceQualityPropsFields = z.infer<typeof SourceQualityPropsSchema>;
