import { z } from "zod";

/**
 * @oppulence-gen kind=lib
 * Runtime contract for the managed SCIM catalog.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `scim.lit.ts`.
 */
export const ScimInputSchema = z.object({
	credentialHashSecret: z.string().min(32),
});

export type ScimInput = z.infer<typeof ScimInputSchema>;
