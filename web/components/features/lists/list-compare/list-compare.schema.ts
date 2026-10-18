import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for List compare.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `list-compare.lit.ts`.
 */
export const ListComparePropsSchema = z.object({
	listId: z.number().int().positive(),
});

export type ListComparePropsFields = z.infer<typeof ListComparePropsSchema>;
