"use client";
import "client-only";
import { useEffect, useRef } from "react";
import { useQuery, useQueryClient } from "@tanstack/react-query";
import { fetchJobStatus } from "@/hooks/queries/utils/fetch-job-status";
import { jobStatusKeys, JOB_STATUS_STALE_TIME } from "@/hooks/queries/utils/job-status-keys";
import { ACTIVE_JOB_STATES } from "@/lib/api/schemas/jobs";

/** @oppulence-gen kind=hook Owned by `use-job-status.lit.ts`. */
export function useJobStatus(jobId: number | undefined, poll = false) {
	const client = useQueryClient();
	const previous = useRef<string | undefined>(undefined);
	const status = useQuery({
		queryKey: jobStatusKeys.detail(jobId == null ? undefined : String(jobId)),
		enabled: !!jobId,
		queryFn: ({ signal }) => fetchJobStatus(String(jobId), signal),
		staleTime: (query) =>
			ACTIVE_JOB_STATES.includes(query.state.data?.status ?? "") ? 5_000 : JOB_STATUS_STALE_TIME,
		refetchOnWindowFocus: (query) =>
			poll && ACTIVE_JOB_STATES.includes(query.state.data?.status ?? ""),
		refetchInterval: (query) =>
			poll &&
			query.state.status !== "error" &&
			ACTIVE_JOB_STATES.includes(query.state.data?.status ?? "")
				? 5_000
				: false,
	});
	useEffect(() => {
		const state = status.data?.status;
		if (!poll || !state) return;
		if (previous.current !== `${jobId}:${state}` && !ACTIVE_JOB_STATES.includes(state)) {
			void client.invalidateQueries({
				predicate: (query) =>
					["overview", "list-detail", "v1-list-lists", "recent-jobs"].includes(
						String(query.queryKey[0]),
					) ||
					(query.queryKey[0] === "jobs" &&
						query.queryKey[1] === jobId &&
						query.queryKey[2] !== "status"),
			});
		}
		previous.current = `${jobId}:${state}`;
	}, [client, jobId, poll, status.data?.status]);
	return status;
}
