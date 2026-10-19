/**
 * @oppulence-gen kind=hook
 * Query-key factory and staleTime for V1 email history.
 * Keep this module free of `"use client"` so server prefetch can import it.
 * Owned by `use-v1-email-history.lit.ts`.
 */
export const v1EmailHistoryKeys = {
	all: ["v1-email-history"] as const,
	list: (query?: Record<string, unknown>) =>
		[...v1EmailHistoryKeys.all, "list", query ?? {}] as const,
	detail: (id?: string, query?: Record<string, unknown>) =>
		[...v1EmailHistoryKeys.all, "detail", id ?? "", query ?? {}] as const,
};

export const V1_EMAIL_HISTORY_STALE_TIME = 15_000;
