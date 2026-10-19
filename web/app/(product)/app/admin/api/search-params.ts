import { createSearchParamsCache } from "nuqs/server";
import { z } from "zod";

/**
 * @oppulence-gen kind=page
 * Shareable view-state for the Platform API route. The URL is the source of truth;
 * do not reconstruct this with router.replace. Owned by `api.lit.ts`.
 * Add --fields or --operation query params to generate parsers.
 */
export const ApiSearchParamsSchema = z.object({});

export type ApiSearchParams = z.infer<typeof ApiSearchParamsSchema>;

export const apiParsers = {} as const;

export const apiUrlKeys = {
	history: "replace",
	clearOnDefault: true,
} as const;

export const apiSearchParamsCache = createSearchParamsCache(apiParsers);
