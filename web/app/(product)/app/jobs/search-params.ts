import { createSearchParamsCache } from "nuqs/server";
import { z } from "zod";

/**
 * @oppulence-gen kind=page
 * Shareable view-state for the Jobs route. The URL is the source of truth;
 * do not reconstruct this with router.replace. Owned by `jobs.lit.ts`.
 * Add --fields or --operation query params to generate parsers.
 */
export const JobsSearchParamsSchema = z.object({
	job: z.coerce.number().int().positive().optional(),
});

export type JobsSearchParams = z.infer<typeof JobsSearchParamsSchema>;

export const jobsParsers = {} as const;

export const jobsUrlKeys = {
	history: "replace",
	clearOnDefault: true,
} as const;

export const jobsSearchParamsCache = createSearchParamsCache(jobsParsers);
