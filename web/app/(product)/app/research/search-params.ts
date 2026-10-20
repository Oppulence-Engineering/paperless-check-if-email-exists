import { createSearchParamsCache, parseAsString } from "nuqs/server";
import { z } from "zod";

/**
 * @oppulence-gen kind=page
 * Shareable view-state for the Company research route. The URL is the source of truth;
 * do not reconstruct this with router.replace. Owned by `research.lit.ts`.
 * Add --fields or --operation query params to generate parsers.
 */
export const ResearchSearchParamsSchema = z.object({
	session: z.string().default(""),
});

export type ResearchSearchParams = z.infer<typeof ResearchSearchParamsSchema>;

export const researchParsers = {
	session: parseAsString.withDefault(""),
} as const;

export const researchUrlKeys = {
	history: "replace",
	clearOnDefault: true,
} as const;

export const researchSearchParamsCache = createSearchParamsCache(researchParsers);
