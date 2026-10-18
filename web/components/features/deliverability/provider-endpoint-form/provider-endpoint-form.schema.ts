import { z } from "zod";

import { V1ListProviderEndpoints200Response } from "@/lib/api/generated/zod/outcomes/outcomes";

/**
 * @oppulence-gen kind=component
 * Runtime contract for Provider endpoint form.
 *
 * This module is validation only: no React, no fetch, no stores. Types are
 * inferred from the schema so the shape cannot drift from the check.
 * Owned by the sibling `provider-endpoint-form.lit.ts`.
 */
const ProviderEndpointSchema = V1ListProviderEndpoints200Response.shape.provider_endpoints.element;

export type ProviderEndpoint = z.infer<typeof ProviderEndpointSchema>;

/** Without `endpoint` the form creates an endpoint. With it, the form edits that endpoint. */
export const ProviderEndpointFormPropsSchema = z.object({
	endpoint: ProviderEndpointSchema.optional(),
});

export type ProviderEndpointFormPropsFields = z.infer<typeof ProviderEndpointFormPropsSchema>;
