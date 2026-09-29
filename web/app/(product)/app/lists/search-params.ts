import { createSearchParamsCache } from "nuqs/server";
import { z } from "zod";

/**
 * @oppulence-gen kind=page
 * Shareable view-state for the Email lists route. The URL is the source of truth;
 * do not reconstruct this with router.replace. Owned by `lists.lit.ts`.
 * Add --fields or --operation query params to generate parsers.
 */
export const ListsSearchParamsSchema = z.object({});

export type ListsSearchParams = z.infer<typeof ListsSearchParamsSchema>;

export const listsParsers = {} as const;

export const listsUrlKeys = {
	history: "replace",
	clearOnDefault: true,
} as const;

export const listsSearchParamsCache = createSearchParamsCache(listsParsers);
