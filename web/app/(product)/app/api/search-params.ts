import { createSearchParamsCache } from "nuqs/server";
import { z } from "zod";

/**
 * @oppulence-gen kind=page
 * Shareable view-state for the API explorer route. The URL is the source of truth;
 * do not reconstruct this with router.replace. Owned by `api-explorer.lit.ts`.
 * Add --fields or --operation query params to generate parsers.
 */
export const ApiExplorerSearchParamsSchema = z.object({});

export type ApiExplorerSearchParams = z.infer<typeof ApiExplorerSearchParamsSchema>;

export const apiExplorerParsers = {} as const;

export const apiExplorerUrlKeys = {
	history: "replace",
	clearOnDefault: true,
} as const;

export const apiExplorerSearchParamsCache = createSearchParamsCache(apiExplorerParsers);
