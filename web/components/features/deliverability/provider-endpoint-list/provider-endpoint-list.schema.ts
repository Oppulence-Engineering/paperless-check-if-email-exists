import { z } from "zod";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Provider endpoint list.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `provider-endpoint-list.lit.ts`.
 */
export const ProviderEndpointListPropsSchema = z.object({});

export type ProviderEndpointListPropsFields = z.infer<typeof ProviderEndpointListPropsSchema>;
