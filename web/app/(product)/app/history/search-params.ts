import { createSearchParamsCache } from "nuqs/server";
import { z } from "zod";

/**
 * @oppulence-gen kind=page
 * Shareable view-state for the Verification history route. The URL is the source of truth;
 * do not reconstruct this with router.replace. Owned by `history.lit.ts`.
 * Add --fields or --operation query params to generate parsers.
 */
export const HistorySearchParamsSchema = z.object({});

export type HistorySearchParams = z.infer<typeof HistorySearchParamsSchema>;

export const historyParsers = {} as const;

export const historyUrlKeys = {
	history: "replace",
	clearOnDefault: true,
} as const;

export const historySearchParamsCache = createSearchParamsCache(historyParsers);
