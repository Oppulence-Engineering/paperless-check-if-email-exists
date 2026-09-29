import { createSearchParamsCache } from "nuqs/server";
import { z } from "zod";

/**
 * @oppulence-gen kind=page
 * Shareable view-state for the Find an email route. The URL is the source of truth;
 * do not reconstruct this with router.replace. Owned by `finder.lit.ts`.
 * Add --fields or --operation query params to generate parsers.
 */
export const FinderSearchParamsSchema = z.object({});

export type FinderSearchParams = z.infer<typeof FinderSearchParamsSchema>;

export const finderParsers = {} as const;

export const finderUrlKeys = {
	history: "replace",
	clearOnDefault: true,
} as const;

export const finderSearchParamsCache = createSearchParamsCache(finderParsers);
