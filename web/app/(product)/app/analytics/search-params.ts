import { createSearchParamsCache } from "nuqs/server";
import { z } from "zod";

/**
 * @oppulence-gen kind=page
 * Shareable view-state for the Analytics route. The URL is the source of truth;
 * do not reconstruct this with router.replace. Owned by `analytics.lit.ts`.
 * Add --fields or --operation query params to generate parsers.
 */
export const AnalyticsSearchParamsSchema = z.object({});

export type AnalyticsSearchParams = z.infer<typeof AnalyticsSearchParamsSchema>;

export const analyticsParsers = {} as const;

export const analyticsUrlKeys = {
	history: "replace",
	clearOnDefault: true,
} as const;

export const analyticsSearchParamsCache = createSearchParamsCache(analyticsParsers);
