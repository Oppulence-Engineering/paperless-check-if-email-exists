import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for List detail panel.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `list-detail-panel.lit.ts`.
 */
export const ListDetailPanelPropsSchema = z.object({
	listId: z.number().int().positive().optional(),
});

export type ListDetailPanelPropsFields = z.infer<typeof ListDetailPanelPropsSchema>;
