import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Outcome entry.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `outcome-entry.lit.ts`.
 */
export const OutcomeEntryPropsSchema = z.object({});

export type OutcomeEntryPropsFields = z.infer<typeof OutcomeEntryPropsSchema>;
