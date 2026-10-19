import { createSearchParamsCache } from "nuqs/server";
import { z } from "zod";

/**
 * @oppulence-gen kind=page
 * Shareable view-state for the Suppressions route. The URL is the source of truth;
 * do not reconstruct this with router.replace. Owned by `suppressions.lit.ts`.
 * Add --fields or --operation query params to generate parsers.
 */
export const SuppressionsSearchParamsSchema = z.object({});

export type SuppressionsSearchParams = z.infer<typeof SuppressionsSearchParamsSchema>;

export const suppressionsParsers = {} as const;

export const suppressionsUrlKeys = {
	history: "replace",
	clearOnDefault: true,
} as const;

export const suppressionsSearchParamsCache = createSearchParamsCache(suppressionsParsers);
