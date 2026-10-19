"use client";

import "client-only";

import { useQuery } from "@tanstack/react-query";

import {
	fetchV1EmailHistory,
	type V1EmailHistoryQuery,
} from "@/hooks/queries/utils/fetch-v1-email-history";
import {
	v1EmailHistoryKeys,
	V1_EMAIL_HISTORY_STALE_TIME,
} from "@/hooks/queries/utils/v1-email-history-keys";

/**
 * @oppulence-gen kind=hook
 * Client query hook for V1 email history. Keys, staleTime, and the fetcher stay in
 * non-client modules. Owned by `use-v1-email-history.lit.ts`.
 */
export function useV1EmailHistory(email: string, query?: V1EmailHistoryQuery) {
	return useQuery({
		queryKey: v1EmailHistoryKeys.detail(email, query),
		queryFn: ({ signal }) => fetchV1EmailHistory(email, query, signal),
		staleTime: V1_EMAIL_HISTORY_STALE_TIME,
		enabled: Boolean(email),
	});
}
