import { z } from "zod";

import { ExplorerFiltersSchema } from "@/components/features/insights/result-explorer/result-explorer.schema";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Saved segments.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `saved-segments.lit.ts`.
 */
export const SavedSegmentsPropsSchema = z.object({
	filters: ExplorerFiltersSchema,
});

export type SavedSegmentsPropsFields = z.infer<typeof SavedSegmentsPropsSchema>;
