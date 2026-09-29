import { createSearchParamsCache } from "nuqs/server";
import { z } from "zod";

/**
 * @oppulence-gen kind=page
 * Shareable view-state for the Outcomes route. The URL is the source of truth;
 * do not reconstruct this with router.replace. Owned by `outcomes.lit.ts`.
 * Add --fields or --operation query params to generate parsers.
 */
export const OutcomesSearchParamsSchema = z.object({});

export type OutcomesSearchParams = z.infer<typeof OutcomesSearchParamsSchema>;

export const outcomesParsers = {} as const;

export const outcomesUrlKeys = {
	history: "replace",
	clearOnDefault: true,
} as const;

export const outcomesSearchParamsCache = createSearchParamsCache(outcomesParsers);
