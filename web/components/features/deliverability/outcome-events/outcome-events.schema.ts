import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Outcome events.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `outcome-events.lit.ts`.
 */
export const OutcomeEventsPropsSchema = z.object({});

export type OutcomeEventsPropsFields = z.infer<typeof OutcomeEventsPropsSchema>;
