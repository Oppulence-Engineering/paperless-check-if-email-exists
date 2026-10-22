import { MutationCache, QueryClient } from "@tanstack/react-query";

/** Matches QueryProvider so RSC dehydrate and the browser cache share one policy. */
const APP_QUERY_STALE_TIME = 30_000;

export function createAppQueryClient(): QueryClient {
	const client: QueryClient = new QueryClient({
		mutationCache: new MutationCache({
			onSuccess: () =>
				client.invalidateQueries({
					predicate: (query) =>
						query.queryKey[0] === "overview" ||
						(query.queryKey[0] === "list-detail" &&
							["quality", "plan"].includes(String(query.queryKey[2]))) ||
						(query.queryKey[0] === "jobs" && query.queryKey[2] === "approval") ||
						["source-quality", "query-results", "tenant-usage"].includes(String(query.queryKey[0])),
				}),
		}),
		defaultOptions: {
			mutations: { retry: false },
			queries: {
				refetchOnWindowFocus: false,
				retry: (count, error) => {
					if (count >= 1 || ["AbortError", "ZodError"].includes(error.name)) return false;
					const response =
						"response" in error && error.response && typeof error.response === "object"
							? error.response
							: undefined;
					const status =
						response && "status" in response
							? response.status
							: "status" in error
								? error.status
								: undefined;
					return (
						status == null ||
						(typeof status === "number" && (status >= 500 || status === 408 || status === 429))
					);
				},
				gcTime: 5 * 60_000,
				refetchIntervalInBackground: false,
				staleTime: APP_QUERY_STALE_TIME,
			},
		},
	});
	for (const resource of [
		"tenant-settings",
		"tenant-profile",
		"tenant-domains",
		"tenant-domain",
		"reverification-status",
		"provider-endpoints",
		"tenant-api-keys",
		"tenant-webhook",
		"score-policies",
		"outcome-policies",
		"list-detail",
		"jobs",
	]) {
		client.setQueryDefaults([resource], { staleTime: 60_000 });
	}
	return client;
}

/** One client per RSC request so dehydrate never leaks across users. */
export function getQueryClient(): QueryClient {
	return createAppQueryClient();
}

/**
 * Awaited prefetch so dehydrate keeps the result. A failed seed must not
 * crash the authenticated shell — the client hook retries.
 */
export async function seedQuery(
	queryClient: QueryClient,
	options: Parameters<QueryClient["prefetchQuery"]>[0],
): Promise<void> {
	try {
		await queryClient.prefetchQuery(options);
	} catch {
		// Intentionally empty: first paint still hydrates whatever succeeded.
	}
}
