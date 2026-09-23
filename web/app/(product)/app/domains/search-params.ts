import { createSearchParamsCache } from "nuqs/server";
import { z } from "zod";

/**
 * @oppulence-gen kind=page
 * Shareable view-state for the Domains route. The URL is the source of truth;
 * do not reconstruct this with router.replace. Owned by `domains.lit.ts`.
 * Add --fields or --operation query params to generate parsers.
 */
export const DomainsSearchParamsSchema = z.object({});

export type DomainsSearchParams = z.infer<typeof DomainsSearchParamsSchema>;

export const domainsParsers = {} as const;

export const domainsUrlKeys = {
	history: "replace",
	clearOnDefault: true,
} as const;

export const domainsSearchParamsCache = createSearchParamsCache(domainsParsers);
