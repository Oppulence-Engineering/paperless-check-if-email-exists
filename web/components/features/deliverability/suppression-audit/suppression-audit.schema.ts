import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Suppression audit.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `suppression-audit.lit.ts`.
 */
export const SuppressionAuditPropsSchema = z.object({
	/** The suppression entry ID from GET /v1/suppressions. */
	entryId: z.number().int().positive(),
});

export type SuppressionAuditPropsFields = z.infer<typeof SuppressionAuditPropsSchema>;
