import type { QueryClient } from "@tanstack/react-query";

import { loadJobStatus } from "@/hooks/queries/utils/fetch-job-status";
import { jobStatusKeys, JOB_STATUS_STALE_TIME } from "@/hooks/queries/utils/job-status-keys";
import { requestUpstreamJson } from "@/lib/api/request-json.server";
import { seedQuery } from "@/lib/query/get-query-client";

/**
 * @oppulence-gen kind=hook
 * Seed the same keys and staleTime useJobStatus reads.
 * Talks to the backend with a server-minted JWT — never loops through `/api/backend`.
 * Owned by `use-job-status.lit.ts`.
 */
export async function prefetchJobStatus(queryClient: QueryClient, job: string): Promise<void> {
	await seedQuery(queryClient, {
		queryKey: jobStatusKeys.detail(job),
		queryFn: ({ signal }) => loadJobStatus(requestUpstreamJson, job, signal),
		staleTime: JOB_STATUS_STALE_TIME,
	});
}
