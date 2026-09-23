import { createSearchParamsCache } from "nuqs/server";
import { z } from "zod";

/**
 * @oppulence-gen kind=page
 * Shareable view-state for the Platform admin route. The URL is the source of truth;
 * do not reconstruct this with router.replace. Owned by `admin.lit.ts`.
 * Add --fields or --operation query params to generate parsers.
 */
export const AdminSearchParamsSchema = z.object({});

export type AdminSearchParams = z.infer<typeof AdminSearchParamsSchema>;

export const adminParsers = {} as const;

export const adminUrlKeys = {
	history: "replace",
	clearOnDefault: true,
} as const;

export const adminSearchParamsCache = createSearchParamsCache(adminParsers);
