"use client";

import "client-only";
import { errorMessage } from "@/lib/api/error-message";

import { useState, type ComponentPropsWithoutRef } from "react";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { Configuration, JobsApi } from "@oppulence/reacher-sdk";

import { Button } from "@oppulence/ui/components/button";
import { Progress } from "@oppulence/ui/components/progress";
import { cn } from "@oppulence/ui/lib/utils";

import { Group, Pill, Row } from "@/components/platform/page-kit";
import { Stat } from "@/components/features/lists/job-review/job-review";
import { timeLeft } from "@/components/features/lists/job-progress/job-progress";
import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";
import { saveFile } from "@/lib/downloads";
import {
	ACTIVE_JOB_STATES,
	FailureCenterSchema,
	JobEventsSchema,
	JobLatencySchema,
} from "@/lib/api/schemas/jobs";
import { humanize, jobStatus, taskState } from "@/lib/verification/labels";

import { type JobProblemsPropsFields } from "./job-problems.schema";

/**
 * @oppulence-gen kind=component
 * JobProblems is a client data component.
 * Shows the failures, retries, and event log for one bulk job.
 *
 * It reads the failure center, check times, and job events through the generated SDK
 * and TanStack Query. It polls every 5 seconds while the job can still change.
 * Owned by `job-problems.lit.ts`.
 */
export type JobProblemsProps = JobProblemsPropsFields & ComponentPropsWithoutRef<"section">;

const jobsApi = new JobsApi(new Configuration({ basePath: "/api/backend" }));
const EVENT_PAGE = 50;

/** Event data as "key: value" pairs, for example "retry_count: 1". */
function eventData(data: Record<string, unknown> | null | undefined): string {
	return Object.entries(data ?? {})
		.filter(([key]) => ["retry_count", "reason", "message", "count"].includes(key))
		.map(
			([key, value]) =>
				`${key.replaceAll("_", " ")}: ${typeof value === "object" ? JSON.stringify(value) : String(value)}`,
		)
		.join(" · ");
}

export function JobProblems({ className, jobId, ...props }: JobProblemsProps) {
	const queryClient = useQueryClient();
	const [eventOffset, setEventOffset] = useState(0);
	const [notice, setNotice] = useState("");
	const failures = useQuery({
		queryKey: workflowQueryKey("jobs", jobId, "failures"),
		queryFn: async ({ signal }) =>
			FailureCenterSchema.parse((await jobsApi.v1GetJobFailureCenter({ jobId })).data),
	});
	const events = useQuery({
		queryKey: workflowQueryKey("jobs", jobId, "events", eventOffset),
		queryFn: async ({ signal }) =>
			JobEventsSchema.parse(
				(await jobsApi.v1GetJobEvents({ jobId, limit: EVENT_PAGE, offset: eventOffset })).data,
			),
	});
	const latency = useQuery({
		queryKey: workflowQueryKey("jobs", jobId, "latency"),
		queryFn: async ({ signal }) =>
			JobLatencySchema.parse((await jobsApi.v1JobLatency({ jobId }, { signal })).data),
	});
	const retry = useMutation({
		mutationFn: async () => (await jobsApi.v1RetryJob({ jobId })).data,
		onSuccess: async (data) => {
			setNotice(`${String(data.tasks_retried)} failed rows are queued again.`);
			await queryClient.invalidateQueries({ queryKey: workflowQueryKey("jobs", jobId) });
		},
	});
	const report = useMutation({
		mutationFn: async () => {
			const response = await jobsApi.v1GetJobFailureReport(
				{ jobId },
				{ params: { format: "csv" }, responseType: "blob" },
			);
			saveFile(response.data, `job-${String(jobId)}-failures.csv`);
		},
	});
	const data = failures.data;
	const error = [failures, events, latency, retry, report].find(
		(request) => request.isError,
	)?.error;

	return (
		<section data-slot="job-problems" className={cn("flex flex-col gap-8", className)} {...props}>
			{error ? (
				<p
					className="rounded-lg bg-red-500/10 px-4 py-3 text-sm text-red-300 ring-1 ring-red-500/20"
					role="alert"
				>
					{errorMessage(error)}
				</p>
			) : null}
			{notice ? (
				<p
					className="rounded-lg bg-emerald-500/10 px-4 py-3 text-sm text-emerald-300 ring-1 ring-emerald-500/20"
					role="status"
				>
					{notice}
				</p>
			) : null}
			{data ? (
				<div className="flex flex-col gap-4">
					<h2 className="text-lg font-medium text-white">Failures</h2>
					<Group>
						<Row
							description={`${String(data.processed_rows)} of ${String(data.total_records)} rows done${
								ACTIVE_JOB_STATES.includes(data.status)
									? ` · ${timeLeft(data.estimated_completion_seconds)}`
									: ""
							}`}
							label={
								<span className="flex items-center gap-2">
									{data.failures_total} failed rows
									<Pill tone={jobStatus.tone(data.status)}>{jobStatus.label(data.status)}</Pill>
								</span>
							}
						>
							<div className="flex flex-wrap gap-2 md:justify-end">
								<Button
									className="rounded-lg"
									disabled={report.isPending}
									onClick={() => {
										report.mutate();
									}}
									size="sm"
									type="button"
									variant="outline"
								>
									Download failure report
								</Button>
								<Button
									className="rounded-lg"
									disabled={retry.isPending || data.failures_total === 0}
									onClick={() => {
										if (window.confirm(`Retry the failed rows in job ${String(jobId)}?`))
											retry.mutate();
									}}
									size="sm"
									type="button"
								>
									Retry failed
								</Button>
							</div>
						</Row>
						<Row>
							<Progress
								aria-label="Rows done"
								className="bg-white/[0.06]"
								value={data.progress_pct}
							/>
							<div className="grid grid-cols-2 gap-3 sm:grid-cols-4">
								<Stat label="Can retry" value={data.retry_summary.retryable_rows} />
								<Stat label="Failed for good" value={data.retry_summary.permanent_failures} />
								<Stat label="Rows retried" value={data.retry_summary.rows_with_retries} />
								<Stat label="Most retries on a row" value={data.retry_summary.max_retry_count} />
							</div>
							<div className="flex flex-wrap gap-2">
								{Object.entries(data.task_states).map(([state, count]) => (
									<Pill key={state} tone={count ? taskState.tone(state) : "gray"}>
										{taskState.label(state)}: {count}
									</Pill>
								))}
							</div>
						</Row>
						{latency.data?.total_completed ? (
							<Row
								description={`Over ${String(latency.data.total_completed)} checked rows`}
								label={`Median check time ${String(latency.data.p50_duration_ms)} ms · 95% within ${String(latency.data.p95_duration_ms)} ms`}
							/>
						) : null}
						{data.failure_breakdown.map((item) => (
							<div
								className="flex items-center justify-between gap-4 px-5 py-4 text-sm"
								key={item.error}
							>
								<span className="min-w-0 break-words text-white">{item.error}</span>
								<span className="text-[#858585]">{item.count}</span>
							</div>
						))}
						{data.failed_rows.map((row) => (
							<div
								className="flex flex-wrap items-center gap-x-4 gap-y-1 px-5 py-4 text-sm"
								key={row.id}
							>
								<span className="min-w-0 flex-1 break-all text-white">
									{row.input || `Row ${String((row.row_index ?? 0) + 1)}`}
								</span>
								<Pill tone={taskState.tone(row.task_state)}>{taskState.label(row.task_state)}</Pill>
								<span className="w-full text-xs text-[#858585]">
									{row.error ?? "No error text"} · {row.retry_count} retries
								</span>
							</div>
						))}
					</Group>
				</div>
			) : null}
			<div className="flex flex-col gap-4">
				<h2 className="text-lg font-medium text-white">Event log</h2>
				<Group>
					{events.data?.events.length ? (
						events.data.events.map((event) => (
							<div className="flex flex-col gap-1 px-5 py-4" key={event.id}>
								<p className="text-sm text-white">
									{humanize(event.event_type.replaceAll(".", " "))}
									{event.task_id ? (
										<span className="text-[#858585]"> · Task {event.task_id}</span>
									) : null}
								</p>
								<p className="text-xs break-words text-[#858585]">
									{new Date(event.created_at).toLocaleString()} · {"System"}
									{event.event_data ? ` · ${eventData(event.event_data)}` : ""}
								</p>
							</div>
						))
					) : (
						<p className="p-5 text-sm text-[#858585]">
							{events.isPending ? "Loading events…" : "No events yet."}
						</p>
					)}
				</Group>
				{events.data && events.data.total > EVENT_PAGE ? (
					<div className="flex justify-end gap-2">
						<Button
							className="rounded-lg"
							disabled={eventOffset === 0}
							onClick={() => {
								setEventOffset(Math.max(0, eventOffset - EVENT_PAGE));
							}}
							size="sm"
							type="button"
							variant="outline"
						>
							Newer events
						</Button>
						<Button
							className="rounded-lg"
							disabled={eventOffset + EVENT_PAGE >= events.data.total}
							onClick={() => {
								setEventOffset(eventOffset + EVENT_PAGE);
							}}
							size="sm"
							type="button"
							variant="outline"
						>
							Older events
						</Button>
					</div>
				) : null}
			</div>
		</section>
	);
}
