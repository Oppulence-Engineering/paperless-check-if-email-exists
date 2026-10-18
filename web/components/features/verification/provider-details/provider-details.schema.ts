import { z } from "zod";

import { V1CheckEmail200Response } from "@/lib/api/generated/zod/v1/v1";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Provider details.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `provider-details.lit.ts`.
 */
export const ProviderDetailsPropsSchema = z.object({
	result: V1CheckEmail200Response,
});

export type ProviderDetailsPropsFields = z.infer<typeof ProviderDetailsPropsSchema>;
