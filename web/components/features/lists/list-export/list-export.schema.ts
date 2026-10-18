import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for List export.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `list-export.lit.ts`.
 */
export const ListExportPropsSchema = z.object({
	listId: z.number().int().positive(),
});

export type ListExportPropsFields = z.infer<typeof ListExportPropsSchema>;
