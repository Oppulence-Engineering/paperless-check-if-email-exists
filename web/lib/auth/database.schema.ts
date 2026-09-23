import { z } from "zod";

/**
 * @oppulence-gen kind=lib
 * Runtime contract for Database.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `database.lit.ts`.
 */
export const DatabaseInputSchema = z.object({
	timeoutMs: z.number().int().positive().max(10_000).default(2_000),
});

export type DatabaseInput = z.infer<typeof DatabaseInputSchema>;
