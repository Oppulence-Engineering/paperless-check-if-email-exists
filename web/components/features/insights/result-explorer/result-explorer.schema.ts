import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Result explorer.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `result-explorer.lit.ts`.
 */

/**
 * The filters of GET /v1/query, with the SDK parameter names. A missing key
 * means no filter. `since` and `until` are local dates (YYYY-MM-DD).
 */
export const ExplorerFiltersSchema = z.object({
	category: z.string().min(1).optional(),
	safeToSend: z.boolean().optional(),
	minScore: z.number().int().min(0).max(100).optional(),
	maxScore: z.number().int().min(0).max(100).optional(),
	jobId: z.number().int().positive().optional(),
	since: z.iso.date().optional(),
	until: z.iso.date().optional(),
	recommendationAction: z.string().min(1).optional(),
	policyMode: z.string().min(1).optional(),
	policyDecision: z.string().min(1).optional(),
});

export type ExplorerFilters = z.infer<typeof ExplorerFiltersSchema>;

export const ResultExplorerPropsSchema = z.object({
	filters: ExplorerFiltersSchema,
});

export type ResultExplorerPropsFields = z.infer<typeof ResultExplorerPropsSchema>;
