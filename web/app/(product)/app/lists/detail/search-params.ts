import { createSearchParamsCache } from "nuqs/server";
import { z } from "zod";

/**
 * @oppulence-gen kind=page
 * Shareable view-state for the List detail route. The URL is the source of truth;
 * do not reconstruct this with router.replace. Owned by `list-detail.lit.ts`.
 * Add --fields or --operation query params to generate parsers.
 */
export const ListDetailSearchParamsSchema = z.object({});

export type ListDetailSearchParams = z.infer<typeof ListDetailSearchParamsSchema>;

export const listDetailParsers = {} as const;

export const listDetailUrlKeys = {
	history: "replace",
	clearOnDefault: true,
} as const;

export const listDetailSearchParamsCache = createSearchParamsCache(listDetailParsers);
