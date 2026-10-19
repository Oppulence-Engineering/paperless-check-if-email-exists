import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Jobs panel.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `jobs-panel.lit.ts`.
 */
export const JobsPanelPropsSchema = z.object({
	initialJobId: z.number().int().positive().optional(),
});

export type JobsPanelPropsFields = z.infer<typeof JobsPanelPropsSchema>;
