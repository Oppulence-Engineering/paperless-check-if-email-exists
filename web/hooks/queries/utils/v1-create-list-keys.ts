/**
 * @oppulence-gen kind=mutation
 * Query-key factory and staleTime for V1 create list.
 * Keep this module free of `"use client"` so server prefetch can import it.
 * Owned by `use-v1-create-list.lit.ts`.
 */
export const v1CreateListKeys = {
	all: ["v1-create-list"] as const,
	list: (query?: Record<string, unknown>) =>
		[...v1CreateListKeys.all, "list", query ?? {}] as const,
	detail: (id?: string, query?: Record<string, unknown>) =>
		[...v1CreateListKeys.all, "detail", id ?? "", query ?? {}] as const,
};

export const V1_CREATE_LIST_STALE_TIME = 15_000;
