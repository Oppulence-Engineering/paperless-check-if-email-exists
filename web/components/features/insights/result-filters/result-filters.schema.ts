import { z } from "zod";

import { ExplorerFiltersSchema } from "@/components/features/insights/result-explorer/result-explorer.schema";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Result filters.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `result-filters.lit.ts`.
 */
export const ResultFiltersPropsSchema = z.object({
	filters: ExplorerFiltersSchema,
});

export type ResultFiltersPropsFields = z.infer<typeof ResultFiltersPropsSchema>;
