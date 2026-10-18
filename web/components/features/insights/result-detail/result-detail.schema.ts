import { z } from "zod";

import { V1QueryResults200Response } from "@/lib/api/generated/zod/query/query";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Result detail.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `result-detail.lit.ts`.
 */
export const ResultDetailPropsSchema = z.object({
	row: V1QueryResults200Response.shape.results.element,
});

export type ResultDetailPropsFields = z.infer<typeof ResultDetailPropsSchema>;
