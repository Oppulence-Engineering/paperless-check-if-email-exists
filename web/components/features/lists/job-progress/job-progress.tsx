"use client";

import "client-only";

import type { ComponentPropsWithoutRef } from "react";
import { useQuery } from "@tanstack/react-query";
import { Configuration, JobsApi } from "@oppulence/reacher-sdk";

import { Progress } from "@oppulence/ui/components/progress";
import { cn } from "@oppulence/ui/lib/utils";

import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";
import { BulkProgressSchema, JobLatencySchema } from "@/lib/api/schemas/jobs";

import { type JobProgressPropsFields } from "./job-progress.schema";

/**
 * @oppulence-gen kind=component
 * JobProgress is a client data component.
 * Shows the progress bar and time left for one running bulk job.
 *
 * It reads progress and latency through the generated SDK and TanStack Query.
 * It polls every 5 seconds until all rows are processed, then it shows nothing.
 * Owned by `job-progress.lit.ts`.
 */
export type JobProgressProps = JobProgressPropsFields & ComponentPropsWithoutRef<"section">;

const jobsApi = new JobsApi(new Configuration({ basePath: "/api/backend" }));

/**
 * The seconds until the job is done at the rate since it started. This is the
 * same estimate as the failure center (backend/src/http/v1/jobs/failure_center.rs).
 */
export function etaSeconds(total: number, processed: number, startedAt: string, now: number) {
	const remaining = total - processed;
	if (remaining <= 0 || processed <= 0) return null;
	const elapsed = Math.max(1, (now - Date.parse(startedAt)) / 1000);
	return Math.ceil(remaining / (processed / elapsed));
}

/** A short "time left" text, for example "About 3 min left". */
export function timeLeft(seconds: number | null | undefined): string {
	if (seconds == null) return "Estimating time left…";
	if (seconds < 60) return "Less than a minute left";
	if (seconds < 3600) return `About ${String(Math.round(seconds / 60))} min left`;
	return `About ${String(Math.round(seconds / 3600))} h left`;
}

export function JobProgress({ className, jobId, ...props }: JobProgressProps) {
	const progress = useQuery({
		queryKey: workflowQueryKey("jobs", jobId, "bulk-progress"),
		queryFn: async () =>
			BulkProgressSchema.parse((await jobsApi.v1GetBulkJobProgress({ jobId })).data),
		refetchInterval: (query) => {
			const data = query.state.data;
			return !data || data.total_processed < data.total_records ? 5_000 : false;
		},
	});
	const running = !!progress.data && progress.data.total_processed < progress.data.total_records;
	const latency = useQuery({
		queryKey: workflowQueryKey("jobs", jobId, "latency"),
		enabled: running,
		queryFn: async () => JobLatencySchema.parse((await jobsApi.v1JobLatency({ jobId })).data),
		refetchInterval: running ? 5_000 : false,
	});
	if (!progress.data || !running) return null;
	const { total_processed: processed, total_records: total, created_at } = progress.data;

	return (
		<section data-slot="job-progress" className={cn("flex flex-col gap-2", className)} {...props}>
			<Progress
				aria-label="Verification progress"
				className="bg-white/[0.06]"
				value={(processed / Math.max(1, total)) * 100}
			/>
			<p className="text-xs text-[#858585]" role="status">
				{processed} of {total} rows checked ·{" "}
				{timeLeft(etaSeconds(total, processed, created_at, progress.dataUpdatedAt))}
				{latency.data?.total_completed
					? ` · Median check time ${String(latency.data.p50_duration_ms)} ms`
					: ""}
			</p>
		</section>
	);
}
