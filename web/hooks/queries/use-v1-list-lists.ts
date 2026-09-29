"use client";

import "client-only";

import { useQuery } from "@tanstack/react-query";

import { fetchV1ListLists, type V1ListListsQuery } from "@/hooks/queries/utils/fetch-v1-list-lists";
import {
	v1ListListsKeys,
	V1_LIST_LISTS_STALE_TIME,
} from "@/hooks/queries/utils/v1-list-lists-keys";

/**
 * @oppulence-gen kind=hook
 * Client query hook for V1 list lists. Keys, staleTime, and the fetcher stay in
 * non-client modules. Owned by `use-v1-list-lists.lit.ts`.
 */
export function useV1ListLists(query?: V1ListListsQuery) {
	return useQuery({
		queryKey: v1ListListsKeys.list(query),
		queryFn: ({ signal }) => fetchV1ListLists(query, signal),
		staleTime: V1_LIST_LISTS_STALE_TIME,
	});
}
