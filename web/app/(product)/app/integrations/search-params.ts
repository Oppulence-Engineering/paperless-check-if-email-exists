import { createSearchParamsCache } from "nuqs/server";
import { z } from "zod";

/**
 * @oppulence-gen kind=page
 * Shareable view-state for the Integration setup route. The URL is the source of truth;
 * do not reconstruct this with router.replace. Owned by `integrations.lit.ts`.
 * Add --fields or --operation query params to generate parsers.
 */
export const IntegrationsSearchParamsSchema = z.object({});

export type IntegrationsSearchParams = z.infer<typeof IntegrationsSearchParamsSchema>;

export const integrationsParsers = {} as const;

export const integrationsUrlKeys = {
	history: "replace",
	clearOnDefault: true,
} as const;

export const integrationsSearchParamsCache = createSearchParamsCache(integrationsParsers);
