/**
 * @oppulence-gen kind=hook
 * Query-key factory and staleTime for V1 list lists.
 * Keep this module free of `"use client"` so server prefetch can import it.
 * Owned by `use-v1-list-lists.lit.ts`.
 */
export const v1ListListsKeys = {
	all: ["v1-list-lists"] as const,
	list: (query?: Record<string, unknown>) => [...v1ListListsKeys.all, "list", query ?? {}] as const,
	detail: (id?: string, query?: Record<string, unknown>) =>
		[...v1ListListsKeys.all, "detail", id ?? "", query ?? {}] as const,
};

export const V1_LIST_LISTS_STALE_TIME = 15_000;
