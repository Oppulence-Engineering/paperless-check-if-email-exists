"use client";

import "client-only";

import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";

import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { Configuration, JobsApi, V1Api } from "@oppulence/reacher-sdk";
import Link from "next/link";
import { useState, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";
import { z } from "zod";

import { Button } from "@oppulence/ui/components/button";
import { Card, CardContent, CardHeader, CardTitle } from "@oppulence/ui/components/card";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { cn } from "@oppulence/ui/lib/utils";
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
		<section data-slot="jobs-panel" className={cn("space-y-6", className)} {...props}>
			<header>
				<h1 className="text-3xl font-semibold">Bulk jobs</h1>
				<p className="mt-2 text-muted-foreground">
					Create a batch, then review progress, results, failures, and delivery readiness.
				</p>
			</header>
			<div className="grid gap-6 lg:grid-cols-2">
				<Card>
					<CardHeader>
						<CardTitle>Create a job</CardTitle>
					</CardHeader>
					<CardContent>
						<form
							className="space-y-4"
							onSubmit={(event) => {
								event.preventDefault();
								create.mutate();
							}}
						>
							<div className="space-y-2">
								<Label htmlFor="bulk-emails">Email addresses</Label>
								<textarea
									id="bulk-emails"
									aria-label="Email addresses"
									className="min-h-32 w-full rounded border border-border bg-background p-3"
									placeholder="One address per line"
									required
									value={input}
									onChange={(event) => {
										setInput(event.target.value);
									}}
								/>
							</div>
							<div className="space-y-2">
								<Label htmlFor="bulk-source">Source label (optional)</Label>
								<Input
									id="bulk-source"
									value={source}
									onChange={(event) => {
										setSource(event.target.value);
									}}
								/>
							</div>
							<Button type="submit" disabled={create.isPending}>
								Create job
							</Button>
						</form>
					</CardContent>
				</Card>
				<Card>
					<CardHeader>
						<CardTitle>Find a job</CardTitle>
					</CardHeader>
					<CardContent>
						<form className="flex gap-2" onSubmit={openJob}>
							<Input
								aria-label="Job ID"
								inputMode="numeric"
								value={lookup}
								onChange={(event) => {
									setLookup(event.target.value);
								}}
							/>
							<Button variant="outline" type="submit">
								Open
							</Button>
						</form>
						<p className="mt-3 text-sm text-muted-foreground">
							List uploads also create jobs.{" "}
							<Link href="/app/lists" className="underline">
								View lists
							</Link>
						</p>
					</CardContent>
				</Card>
			</div>
			{validation || requestError ? (
				<p role="alert" className="text-sm text-destructive">
					{validation || requestError?.message}
				</p>
			) : null}
			{notice ? (
				<p role="status" className="text-sm text-emerald-700">
					{notice}{" "}
					<Link href={`/app/jobs?job=${String(jobId)}`} className="underline">
						Share this job
					</Link>
				</p>
			) : null}
			{jobId ? (
				<Card>
					<CardHeader>
						<CardTitle>Job {String(jobId)}</CardTitle>
					</CardHeader>
					<CardContent className="space-y-5">
						{status.isPending ? <p role="status">Loading job…</p> : null}
						{status.data ? (
							<div className="space-y-3">
								<div className="flex flex-wrap items-center gap-3">
									<span className="rounded-full bg-muted px-3 py-1 text-sm">
										{status.data.status}
									</span>
									<span>{status.data.total_records} records</span>
									<Button
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
								</div>
								<div className="grid gap-2 sm:grid-cols-4">
									{Object.entries(status.data.task_summary).map(([label, count]) => (
										<div className="rounded border p-3" key={label}>
											<p className="text-xs text-muted-foreground">{label.replaceAll("_", " ")}</p>
											<p className="text-xl font-semibold">{count}</p>
										</div>
									))}
								</div>
								<div className="flex gap-2">
									<Button
										size="sm"
										variant="outline"
										disabled={
											action.isPending || ["completed", "cancelled"].includes(status.data.status)
										}
										onClick={() => {
											changeJob("cancel");
										}}
									>
										Cancel
									</Button>
									<Button
										size="sm"
										variant="outline"
										disabled={
											action.isPending ||
											!(status.data.task_summary.failed || status.data.task_summary.dead_lettered)
										}
										onClick={() => {
											changeJob("retry");
										}}
									>
										Retry failed
									</Button>
								</div>
							</div>
						) : null}
						<nav aria-label="Job views" className="flex flex-wrap gap-2">
							{tabs.map((item) => (
								<Button
									type="button"
									size="sm"
									key={item}
									variant={tab === item ? "default" : "outline"}
									onClick={() => {
										setTab(item);
									}}
								>
									{item[0].toUpperCase() + item.slice(1)}
								</Button>
							))}
						</nav>
						{tab === "progress" && progress.data ? (
							<div>
								<p>{progress.data.total_processed} processed</p>
								<p className="text-sm text-muted-foreground">
									{Object.entries(progress.data.summary)
										.map(([label, count]) => `${label.replaceAll("_", " ")}: ${String(count)}`)
										.join(" · ")}
								</p>
							</div>
						) : null}
						{tab === "results" ? (
							<div className="space-y-3">
								<Button
									size="sm"
									variant="outline"
									disabled={download.isPending}
									onClick={() => {
										download.mutate("results");
									}}
								>
									Download results CSV
								</Button>
								{bulkResults.data ? (
									<p className="text-sm text-muted-foreground">
										{bulkResults.data.results.length} verified outputs in this batch.
									</p>
								) : null}
								{results.data?.results.length ? (
									<div className="overflow-x-auto">
										<table className="w-full text-sm">
											<thead>
												<tr>
													<th className="p-2 text-left">Email</th>
													<th className="p-2 text-left">State</th>
													<th className="p-2 text-left">Reachability</th>
													<th className="p-2 text-left">Error</th>
												</tr>
											</thead>
											<tbody>
												{results.data.results.map((row) => (
													<tr className="border-t" key={row.id}>
														<td className="p-2">{row.result?.input ?? `Row ${String(row.id)}`}</td>
														<td className="p-2">{row.task_state}</td>
														<td className="p-2">{row.result?.is_reachable ?? "—"}</td>
														<td className="p-2">{row.error ?? "—"}</td>
													</tr>
												))}
											</tbody>
										</table>
									</div>
								) : (
									<p>No results yet.</p>
								)}
								<Button
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
						) : null}
						{tab === "events" ? (
							<div className="space-y-3">
								{events.data?.events.length ? (
									<ol className="space-y-2">
										{events.data.events.map((event) => (
											<li className="rounded border p-3 text-sm" key={event.id}>
												{event.event_type} · {new Date(event.created_at).toLocaleString()}
											</li>
										))}
									</ol>
								) : (
									<p>No events yet.</p>
								)}
								<Button
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
						) : null}
						{tab === "failures" ? (
							<div className="space-y-3">
								<p>
									{failures.data?.failures_total ?? 0} failed rows ·{" "}
									{failures.data?.retry_summary.retryable_rows ?? 0} retryable ·{" "}
									{failures.data?.retry_summary.permanent_failures ?? 0} permanent
								</p>
								<Button
									size="sm"
									variant="outline"
									disabled={download.isPending}
									onClick={() => {
										download.mutate("failures");
									}}
								>
									Download failure report
								</Button>
								{failures.data?.failure_breakdown.map((item) => (
									<p className="rounded border p-2 text-sm" key={item.error}>
										{item.error}: {String(item.count)}
									</p>
								))}
							</div>
						) : null}
						{tab === "approval" && approval.data ? (
							<div>
								<p className="font-medium">
									{approval.data.ready_to_send ? "Ready to send" : "Review before sending"}
								</p>
								<p>{approval.data.recommendation}</p>
								<p className="text-sm">
									{approval.data.safe_to_send_count} safe addresses (
									{approval.data.safe_to_send_pct}
									%)
								</p>
							</div>
						) : null}
						{tab === "latency" && latency.data ? (
							<div className="grid gap-2 sm:grid-cols-4">
								{[
									["Completed", latency.data.total_completed],
									["Median", latency.data.p50_duration_ms],
									["P95", latency.data.p95_duration_ms],
									["P99", latency.data.p99_duration_ms],
								].map(([label, value]) => (
									<div className="rounded border p-3" key={label}>
										<p className="text-xs">{label}</p>
										<p>
											{String(value)}
											{label === "Completed" ? "" : " ms"}
										</p>
									</div>
								))}
							</div>
						) : null}
					</CardContent>
				</Card>
			) : null}
		</section>
	);
}
