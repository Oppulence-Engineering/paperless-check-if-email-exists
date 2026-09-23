import { createParser, createSearchParamsCache } from "nuqs/server";
import { z } from "zod";

/**
 * @oppulence-gen kind=page
 * Shareable view-state for the Accept invitation route. The URL is the source of truth;
 * do not reconstruct this with router.replace. Owned by `accept.lit.ts`.
 * Add --fields or --operation query params to generate parsers.
 */
export const AcceptSearchParamsSchema = z.object({
	invitation: z.string().min(1),
	return_to: z.string().min(1),
});

export type AcceptSearchParams = z.infer<typeof AcceptSearchParamsSchema>;

const stringParam = createParser({
	parse: (value) => value,
	serialize: (value: string) => value,
});

export const acceptParsers = {
	invitation: stringParam,
	return_to: stringParam.withDefault("/app/settings?settings=identity"),
} as const;

export const acceptSearchParamsCache = createSearchParamsCache(acceptParsers);
