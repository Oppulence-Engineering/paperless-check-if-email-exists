import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Finder panel.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `finder-panel.lit.ts`.
 */
export const FinderPanelPropsSchema = z.object({
	initialJobId: z.number().int().positive().optional(),
});

export type FinderPanelPropsFields = z.infer<typeof FinderPanelPropsSchema>;
