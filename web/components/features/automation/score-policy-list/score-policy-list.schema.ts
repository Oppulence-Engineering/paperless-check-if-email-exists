import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Score policy list.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `score-policy-list.lit.ts`.
 */
export const ScorePolicyListPropsSchema = z.object({});

export type ScorePolicyListPropsFields = z.infer<typeof ScorePolicyListPropsSchema>;
