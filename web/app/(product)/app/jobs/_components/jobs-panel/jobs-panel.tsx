"use client";

import "client-only";

import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";

import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { Configuration, JobsApi, V1Api } from "@oppulence/reacher-sdk";
import Link from "next/link";
import { useState, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";
import { z } from "zod";

import { Button } from "@oppulence/ui/components/button";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { cn } from "@oppulence/ui/lib/utils";
import { Group, Page, Pill, Row, Section } from "@/components/platform/page-kit";

import { type JobsPanelPropsFields } from "./jobs-panel.schema";

/** @oppulence-gen kind=component Owned by `jobs-panel.lit.ts`. */
export type JobsPanelProps = JobsPanelPropsFields & ComponentPropsWithoutRef<"section">;

const jobsApi = new JobsApi(new Configuration({ basePath: "/api/backend" }));
const v1Api = new V1Api(new Configuration({ basePath: "/api/backend" }));
const jobStatus = z.object({
	status: z.string(),
	total_records: z.number(),
	task_summary: z.record(z.string(), z.number()),
});
const bulkProgress = z.object({
	total_processed: z.number(),
	summary: z.record(z.string(), z.number()),
});
const jobEvents = z.object({
	total: z.number(),
	events: z.array(
		z.object({
			id: z.number(),
			event_type: z.string(),
			created_at: z.string(),
		}),
	),
});
const failureCenter = z.object({
	failures_total: z.number(),
	retry_summary: z.object({
		retryable_rows: z.number(),
		permanent_failures: z.number(),
	}),
	failure_breakdown: z.array(z.object({ error: z.string(), count: z.number() })),
});
const jobLatency = z.object({
	total_completed: z.number(),
	p50_duration_ms: z.number(),
	p95_duration_ms: z.number(),
	p99_duration_ms: z.number(),
});
type Tab = "progress" | "results" | "events" | "failures" | "approval" | "latency";
const tabs: Tab[] = ["progress", "results", "events", "failures", "approval", "latency"];

export function parseEmailBatch(value: string): string[] {
	return value
		.split(/[\s,;]+/)
		.map((email) => email.trim())
		.filter(Boolean);
}

function saveBlob(data: unknown, filename: string) {
	const blob = data instanceof Blob ? data : new Blob([data as BlobPart]);
	const url = URL.createObjectURL(blob);
	const link = document.createElement("a");
	link.href = url;
	link.download = filename;
	link.click();
	setTimeout(() => {
		URL.revokeObjectURL(url);
	}, 1_000);
}

export function JobsPanel({ initialJobId, className, ...props }: JobsPanelProps) {
	const queryClient = useQueryClient();
	const [input, setInput] = useState("");
	const [source, setSource] = useState("");
	const [lookup, setLookup] = useState(initialJobId ? String(initialJobId) : "");
	const [jobId, setJobId] = useState(initialJobId);
	const [tab, setTab] = useState<Tab>("progress");
	const [cursor, setCursor] = useState<number>();
	const [eventOffset, setEventOffset] = useState(0);
	const [notice, setNotice] = useState("");
	const [validation, setValidation] = useState("");
	const key = ["jobs", jobId];
	function selectedJobId(): number {
		if (!jobId) throw new Error("Select a job first.");
		return jobId;
	}

	const create = useMutation({
		mutationFn: async () => {
			const emails = parseEmailBatch(input);
			if (!emails.length) throw new Error("Enter at least one email address.");
			return (
				await v1Api.v1CreateBulkJob({
					bulkCreateRequest: {
						input: emails,
						source_key: source.trim() || undefined,
					},
				})
			).data;
		},
		onSuccess: (data) => {
			setJobId(data.job_id);
			setLookup(String(data.job_id));
			setInput("");
			setNotice(`Job ${String(data.job_id)} created.`);
			setTab("progress");
		},
	});
	const status = useQuery({
		queryKey: workflowQueryKey(...key, "status"),
		enabled: !!jobId,
		queryFn: async () =>
			jobStatus.parse((await jobsApi.v1GetJobStatus({ jobId: selectedJobId() })).data),
		refetchInterval: (query) =>
			["queued", "running", "retrying"].includes(query.state.data?.status ?? "") ? 5_000 : false,
	});
	const progress = useQuery({
		queryKey: workflowQueryKey(...key, "bulk-progress"),
		enabled: !!jobId && tab === "progress",
		queryFn: async () =>
			bulkProgress.parse((await jobsApi.v1GetBulkJobProgress({ jobId: selectedJobId() })).data),
	});
	const results = useQuery({
		queryKey: workflowQueryKey(...key, "results", cursor),
		enabled: !!jobId && tab === "results",
		queryFn: async () =>
			(
				await jobsApi.v1GetJobResults({
					jobId: selectedJobId(),
					cursor,
					limit: 50,
				})
			).data,
	});
	const bulkResults = useQuery({
		queryKey: workflowQueryKey(...key, "bulk-results"),
		enabled: !!jobId && tab === "results",
		queryFn: async () =>
			(
				await jobsApi.v1GetBulkJobResults({
					jobId: selectedJobId(),
					format: "json",
					limit: 50,
				})
			).data,
	});
	const events = useQuery({
		queryKey: workflowQueryKey(...key, "events", eventOffset),
		enabled: !!jobId && tab === "events",
		queryFn: async () =>
			jobEvents.parse(
				(
					await jobsApi.v1GetJobEvents({
						jobId: selectedJobId(),
						limit: 50,
						offset: eventOffset,
					})
				).data,
			),
	});
	const failures = useQuery({
		queryKey: workflowQueryKey(...key, "failures"),
		enabled: !!jobId && tab === "failures",
		queryFn: async () =>
			failureCenter.parse((await jobsApi.v1GetJobFailureCenter({ jobId: selectedJobId() })).data),
	});
	const approval = useQuery({
		queryKey: workflowQueryKey(...key, "approval"),
		enabled: !!jobId && tab === "approval",
		queryFn: async () => (await jobsApi.v1JobApprovalChecklist({ jobId: selectedJobId() })).data,
	});
	const latency = useQuery({
		queryKey: workflowQueryKey(...key, "latency"),
		enabled: !!jobId && tab === "latency",
		queryFn: async () =>
			jobLatency.parse((await jobsApi.v1JobLatency({ jobId: selectedJobId() })).data),
	});
	const action = useMutation({
		mutationFn: async (kind: "cancel" | "retry") => {
			if (!jobId) return;
			if (kind === "cancel") await jobsApi.v1CancelJob({ jobId });
			else await jobsApi.v1RetryJob({ jobId });
		},
		onSuccess: async (_, kind) => {
			setNotice(kind === "cancel" ? "Cancellation requested." : "Failed rows queued for retry.");
			await queryClient.invalidateQueries({ queryKey: key });
		},
	});
	const download = useMutation({
		mutationFn: async (kind: "results" | "failures") => {
			if (!jobId) return;
			const response =
				kind === "results"
					? await jobsApi.v1DownloadJobResults({ jobId, format: "csv" }, { responseType: "blob" })
					: await jobsApi.v1GetJobFailureReport(
							{ jobId },
							{ params: { format: "csv" }, responseType: "blob" },
						);
			saveBlob(response.data, `job-${String(jobId)}-${kind}.csv`);
		},
	});

	function openJob(event: SyntheticEvent<HTMLFormElement>) {
		event.preventDefault();
		const id = Number(lookup);
		if (!Number.isSafeInteger(id) || id < 1) {
			setValidation("Enter a valid job ID.");
			return;
		}
		setValidation("");
		setJobId(id);
		setTab("progress");
		setCursor(undefined);
		setEventOffset(0);
	}
	function changeJob(kind: "cancel" | "retry") {
		if (
			window.confirm(
				`${kind === "cancel" ? "Cancel" : "Retry failed rows in"} job ${String(jobId)}?`,
			)
		)
			action.mutate(kind);
	}
	const requestError = [
		create,
		status,
		progress,
		results,
		bulkResults,
		events,
		failures,
		approval,
		latency,
		action,
		download,
	].find((request) => request.isError)?.error;

	return (
		<Page
			description="Create a batch, then review progress, results, failures, and delivery readiness."
			title="Bulk jobs"
		>
			<section data-slot="jobs-panel" className={cn("flex flex-col gap-10", className)} {...props}>
				<form
					onSubmit={(event) => {
						event.preventDefault();
						create.mutate();
					}}
				>
					<Group>
						<Row>
							<h3 className="text-sm font-medium text-white">Create a job</h3>
							<div className="flex flex-col gap-1.5">
								<Label className="text-xs font-medium text-neutral-400" htmlFor="bulk-emails">
									Email addresses
								</Label>
								<textarea
									id="bulk-emails"
									aria-label="Email addresses"
									className="min-h-32 w-full rounded-lg border border-input bg-transparent px-3 py-2 text-sm dark:bg-input/30"
									placeholder="One address per line"
									required
									value={input}
									onChange={(event) => {
										setInput(event.target.value);
									}}
								/>
							</div>
							<div className="grid gap-3 sm:grid-cols-[1fr_auto] sm:items-end">
								<div className="flex flex-col gap-1.5">
									<Label className="text-xs font-medium text-neutral-400" htmlFor="bulk-source">
										Source label (optional)
									</Label>
									<Input
										className="h-10 rounded-lg"
										id="bulk-source"
										value={source}
										onChange={(event) => {
											setSource(event.target.value);
										}}
									/>
								</div>
								<Button className="h-10 rounded-lg" type="submit" disabled={create.isPending}>
									Create job
								</Button>
							</div>
						</Row>
					</Group>
				</form>
				<form onSubmit={openJob}>
					<Group>
						<Row
							description={
								<>
									List uploads also create jobs.{" "}
									<Link href="/app/lists" className="underline">
										View lists
									</Link>
								</>
							}
							label="Find a job"
							layout="inline"
						>
							<Input
								aria-label="Job ID"
								className="h-10 rounded-lg md:w-40"
								inputMode="numeric"
								value={lookup}
								onChange={(event) => {
									setLookup(event.target.value);
								}}
							/>
							<Button className="h-10 rounded-lg" variant="outline" type="submit">
								Open
							</Button>
						</Row>
					</Group>
				</form>
				{validation || requestError ? (
					<p
						role="alert"
						className="rounded-lg bg-red-500/10 px-4 py-3 text-sm text-red-300 ring-1 ring-red-500/20"
					>
						{validation || requestError?.message}
					</p>
				) : null}
				{notice ? (
					<p
						role="status"
						className="rounded-lg bg-emerald-500/10 px-4 py-3 text-sm text-emerald-300 ring-1 ring-emerald-500/20"
					>
						{notice}{" "}
						<Link href={`/app/jobs?job=${String(jobId)}`} className="underline">
							Share this job
						</Link>
					</p>
				) : null}
				{jobId ? (
					<Section
						actions={
							<Button
								className="rounded-lg"
								size="sm"
								variant="outline"
								onClick={() => {
									void queryClient.invalidateQueries({
										queryKey: key,
									});
								}}
							>
								Refresh
							</Button>
						}
						title={`Job ${String(jobId)}`}
					>
						<Group>
							{status.isPending ? (
								<Row>
									<p className="text-sm text-[#858585]" role="status">
										Loading job…
									</p>
								</Row>
							) : null}
							{status.data ? (
								<>
									<div className="flex flex-wrap items-center gap-x-4 gap-y-3 px-5 py-4">
										<Pill>{status.data.status}</Pill>
										<span className="min-w-0 flex-1 text-sm text-white">
											{status.data.total_records} records
										</span>
										<div className="flex items-center gap-2">
											<Button
												size="sm"
												variant="secondary"
												disabled={
													action.isPending ||
													["completed", "cancelled"].includes(status.data.status)
												}
												onClick={() => {
													changeJob("cancel");
												}}
											>
												Cancel
											</Button>
											<Button
												size="sm"
												variant="secondary"
												disabled={
													action.isPending ||
													!(
														status.data.task_summary.failed ||
														status.data.task_summary.dead_lettered
													)
												}
												onClick={() => {
													changeJob("retry");
												}}
											>
												Retry failed
											</Button>
										</div>
									</div>
									<Row>
										<div className="grid gap-3 sm:grid-cols-4">
											{Object.entries(status.data.task_summary).map(([label, count]) => (
												<div
													className="rounded-xl bg-white/[0.02] p-3 ring-1 ring-white/[0.06]"
													key={label}
												>
													<p className="text-xs text-[#858585]">{label.replaceAll("_", " ")}</p>
													<p className="mt-1 text-xl font-medium text-white">{count}</p>
												</div>
											))}
										</div>
									</Row>
								</>
							) : null}
						</Group>
						<div
							aria-label="Job views"
							className="inline-flex w-fit flex-wrap rounded-lg bg-white/[0.03] p-0.5 ring-1 ring-white/[0.08]"
							role="group"
						>
							{tabs.map((item) => (
								<button
									aria-pressed={tab === item}
									className={cn(
										"rounded-md px-3 py-1 text-sm font-medium transition-colors",
										tab === item
											? "bg-white/[0.08] text-white"
											: "text-[#858585] hover:text-neutral-200",
									)}
									key={item}
									onClick={() => {
										setTab(item);
									}}
									type="button"
								>
									{item[0].toUpperCase() + item.slice(1)}
								</button>
							))}
						</div>
						<Group>
							{tab === "progress" && progress.data ? (
								<Row
									description={Object.entries(progress.data.summary)
										.map(([label, count]) => `${label.replaceAll("_", " ")}: ${String(count)}`)
										.join(" · ")}
									label={`${String(progress.data.total_processed)} processed`}
								/>
							) : null}
							{tab === "results" ? (
								<Row>
									<div className="flex flex-wrap items-center justify-between gap-3">
										{bulkResults.data ? (
											<p className="text-sm text-[#858585]">
												{bulkResults.data.results.length} verified outputs in this batch.
											</p>
										) : (
											<span />
										)}
										<Button
											size="sm"
											variant="secondary"
											disabled={download.isPending}
											onClick={() => {
												download.mutate("results");
											}}
										>
											Download results CSV
										</Button>
									</div>
									{results.data?.results.length ? (
										<div className="overflow-x-auto">
											<table className="w-full text-left text-sm">
												<thead>
													<tr className="border-b border-border">
														<th className="py-2 pr-4 font-medium text-foreground">Email</th>
														<th className="py-2 pr-4 font-medium text-foreground">State</th>
														<th className="py-2 pr-4 font-medium text-foreground">Reachability</th>
														<th className="py-2 font-medium text-foreground">Error</th>
													</tr>
												</thead>
												<tbody>
													{results.data.results.map((row) => (
														<tr className="border-b border-border/60" key={row.id}>
															<td className="py-2 pr-4 text-foreground">
																{row.result?.input ?? `Row ${String(row.id)}`}
															</td>
															<td className="py-2 pr-4 text-muted-foreground">{row.task_state}</td>
															<td className="py-2 pr-4 text-muted-foreground">
																{row.result?.is_reachable ?? "—"}
															</td>
															<td className="py-2 text-muted-foreground">{row.error ?? "—"}</td>
														</tr>
													))}
												</tbody>
											</table>
										</div>
									) : (
										<p className="text-sm text-[#858585]">No results yet.</p>
									)}
									<div className="flex justify-end">
										<Button
											className="rounded-lg"
											size="sm"
											variant="outline"
											disabled={!results.data?.has_more || !results.data.next_cursor}
											onClick={() => {
												setCursor(results.data?.next_cursor ?? undefined);
											}}
										>
											Next results
										</Button>
									</div>
								</Row>
							) : null}
							{tab === "events" ? (
								<>
									{events.data?.events.length ? (
										events.data.events.map((event) => (
											<div className="px-5 py-4 text-sm text-white" key={event.id}>
												{event.event_type}{" "}
												<span className="text-xs text-[#858585]">
													· {new Date(event.created_at).toLocaleString()}
												</span>
											</div>
										))
									) : (
										<Row>
											<p className="text-sm text-[#858585]">No events yet.</p>
										</Row>
									)}
									<div className="flex justify-end px-5 py-3">
										<Button
											className="rounded-lg"
											size="sm"
											variant="outline"
											disabled={!events.data || eventOffset + 50 >= events.data.total}
											onClick={() => {
												setEventOffset(eventOffset + 50);
											}}
										>
											More events
										</Button>
									</div>
								</>
							) : null}
							{tab === "failures" ? (
								<>
									<Row
										label={`${String(failures.data?.failures_total ?? 0)} failed rows · ${String(failures.data?.retry_summary.retryable_rows ?? 0)} retryable · ${String(failures.data?.retry_summary.permanent_failures ?? 0)} permanent`}
										layout="inline"
									>
										<Button
											size="sm"
											variant="secondary"
											disabled={download.isPending}
											onClick={() => {
												download.mutate("failures");
											}}
										>
											Download failure report
										</Button>
									</Row>
									{failures.data?.failure_breakdown.map((item) => (
										<div
											className="flex items-center justify-between gap-4 px-5 py-4 text-sm"
											key={item.error}
										>
											<span className="min-w-0 truncate text-white">{item.error}</span>
											<span className="text-[#858585]">{String(item.count)}</span>
										</div>
									))}
								</>
							) : null}
							{tab === "approval" && approval.data ? (
								<Row
									description={`${String(approval.data.safe_to_send_count)} safe addresses (${String(approval.data.safe_to_send_pct)}%)`}
									label={approval.data.ready_to_send ? "Ready to send" : "Review before sending"}
								>
									<p className="text-sm text-neutral-300">{approval.data.recommendation}</p>
								</Row>
							) : null}
							{tab === "latency" && latency.data ? (
								<Row>
									<div className="grid gap-3 sm:grid-cols-4">
										{[
											["Completed", latency.data.total_completed],
											["Median", latency.data.p50_duration_ms],
											["P95", latency.data.p95_duration_ms],
											["P99", latency.data.p99_duration_ms],
										].map(([label, value]) => (
											<div
												className="rounded-xl bg-white/[0.02] p-3 ring-1 ring-white/[0.06]"
												key={label}
											>
												<p className="text-xs text-[#858585]">{label}</p>
												<p className="mt-1 text-xl font-medium text-white">
													{String(value)}
													{label === "Completed" ? "" : " ms"}
												</p>
											</div>
										))}
									</div>
								</Row>
							) : null}
						</Group>
					</Section>
				) : null}
			</section>
		</Page>
	);
}
