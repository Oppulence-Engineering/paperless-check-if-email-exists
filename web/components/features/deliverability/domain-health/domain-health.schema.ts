import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Domain health.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `domain-health.lit.ts`.
 */
export const DomainHealthPropsSchema = z.object({
	domain: z.string().trim().min(1),
});

export type DomainHealthPropsFields = z.infer<typeof DomainHealthPropsSchema>;
