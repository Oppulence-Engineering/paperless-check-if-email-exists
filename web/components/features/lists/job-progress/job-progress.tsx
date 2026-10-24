"use client";

import "client-only";

import type { ComponentPropsWithoutRef } from "react";
import { useJobStatus } from "@/hooks/queries/use-job-status";

import { Progress } from "@oppulence/ui/components/progress";
import { cn } from "@oppulence/ui/lib/utils";

import { ACTIVE_JOB_STATES } from "@/lib/api/schemas/jobs";

import { type JobProgressPropsFields } from "./job-progress.schema";

/**
 * @oppulence-gen kind=component
 * JobProgress is a client data component.
 * Shows the progress bar and time left for one running bulk job.
 *
 * It reads job status through the generated SDK and TanStack Query.
 * It polls every 5 seconds until the job is terminal, then it shows nothing.
 * Owned by `job-progress.lit.ts`.
 */
export type JobProgressProps = JobProgressPropsFields & ComponentPropsWithoutRef<"section">;

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
	const progress = useJobStatus(jobId, true);
	const running = ACTIVE_JOB_STATES.includes(progress.data?.status ?? "");
	if (!progress.data || !running) return null;
	const { total_records: total, created_at, task_summary } = progress.data;
	const processed = ["completed", "failed", "dead_lettered", "cancelled"].reduce(
		(sum, state) => sum + (task_summary[state] ?? 0),
		0,
	);

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
			</p>
		</section>
	);
}
