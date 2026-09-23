import { z } from "zod";

/**
 * @oppulence-gen kind=lib
 * Runtime contract for Platform admin.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `platform-admin.lit.ts`.
 */
export const TenantSummarySchema = z.object({
	id: z.string().min(1),
	name: z.string(),
	slug: z.string(),
	createdAt: z.date(),
	archivedAt: z.date().nullable(),
	memberCount: z.number().int().nonnegative(),
});

export type TenantSummary = z.infer<typeof TenantSummarySchema>;
