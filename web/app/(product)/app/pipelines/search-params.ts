import { createSearchParamsCache } from "nuqs/server";
import { z } from "zod";

/**
 * @oppulence-gen kind=page
 * Shareable view-state for the Pipelines route. The URL is the source of truth;
 * do not reconstruct this with router.replace. Owned by `pipelines.lit.ts`.
 * Add --fields or --operation query params to generate parsers.
 */
export const PipelinesSearchParamsSchema = z.object({});

export type PipelinesSearchParams = z.infer<typeof PipelinesSearchParamsSchema>;

export const pipelinesParsers = {} as const;

export const pipelinesUrlKeys = {
	history: "replace",
	clearOnDefault: true,
} as const;

export const pipelinesSearchParamsCache = createSearchParamsCache(pipelinesParsers);
