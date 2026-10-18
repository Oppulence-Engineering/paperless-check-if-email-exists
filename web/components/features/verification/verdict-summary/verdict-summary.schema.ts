import { z } from "zod";

import { V1CheckEmail200Response } from "@/lib/api/generated/zod/v1/v1";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Verdict summary.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `verdict-summary.lit.ts`.
 */
export const VerdictSummaryPropsSchema = z.object({
	result: V1CheckEmail200Response,
});

export type VerdictSummaryPropsFields = z.infer<typeof VerdictSummaryPropsSchema>;
