import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Outcome policy list.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `outcome-policy-list.lit.ts`.
 */
export const OutcomePolicyListPropsSchema = z.object({});

export type OutcomePolicyListPropsFields = z.infer<typeof OutcomePolicyListPropsSchema>;
