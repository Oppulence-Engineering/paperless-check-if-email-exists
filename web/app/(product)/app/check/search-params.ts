import { createSearchParamsCache, parseAsString } from "nuqs/server";
import { z } from "zod";

/**
 * @oppulence-gen kind=page
 * Shareable view-state for the Check an email route. The URL is the source of truth;
 * do not reconstruct this with router.replace. Owned by `check.lit.ts`.
 */
export const CheckSearchParamsSchema = z.object({
	email: z.string().optional(),
});

export type CheckSearchParams = z.infer<typeof CheckSearchParamsSchema>;

export const checkParsers = {
	email: parseAsString,
} as const;

export const checkUrlKeys = {
	history: "replace",
	clearOnDefault: true,
} as const;

export const checkSearchParamsCache = createSearchParamsCache(checkParsers);
