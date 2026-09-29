"use client";

import "client-only";

import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";

import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { Configuration, PipelinesApi } from "@oppulence/reacher-sdk";
import Link from "next/link";
import { useState, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";

import { Button } from "@oppulence/ui/components/button";
import { Card, CardContent, CardHeader, CardTitle } from "@oppulence/ui/components/card";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { cn } from "@oppulence/ui/lib/utils";
import { type PipelinesPanelPropsFields } from "./pipelines-panel.schema";

/** @oppulence-gen kind=component Owned by `pipelines-panel.lit.ts`. */
export type PipelinesPanelProps = PipelinesPanelPropsFields & ComponentPropsWithoutRef<"section">;

const api = new PipelinesApi(new Configuration({ basePath: "/api/backend" }));
export function PipelinesPanel({ className, ...props }: PipelinesPanelProps) {
	const client = useQueryClient();
	const [name, setName] = useState("");
	const [sourceType, setSourceType] = useState<"list_snapshot" | "push">("list_snapshot");
	const [listId, setListId] = useState("");
	const [tokenId, setTokenId] = useState("");
	const [cron, setCron] = useState("0 9 * * 1");
	const [timezone, setTimezone] = useState("UTC");
	const [selectedId, setSelectedId] = useState<number>();
	const [selectedRun, setSelectedRun] = useState<number>();
	const [rename, setRename] = useState("");
	const [pushRows, setPushRows] = useState("");
	const [offset, setOffset] = useState(0);
	const [notice, setNotice] = useState("");
	const list = useQuery({
		queryKey: workflowQueryKey("pipelines"),
		queryFn: async () => (await api.v1ListPipelines({ limit: 100 })).data,
	});
	const detail = useQuery({
		queryKey: workflowQueryKey("pipeline", selectedId),
		enabled: !!selectedId,
		queryFn: async () => {
			if (!selectedId) throw new Error("Select a pipeline.");
			return (await api.v1GetPipeline({ pipelineId: selectedId })).data;
		},
	});
	const runs = useQuery({
		queryKey: workflowQueryKey("pipeline-runs", selectedId, offset),
		enabled: !!selectedId,
		queryFn: async () => {
			if (!selectedId) throw new Error("Select a pipeline.");
			return (await api.v1ListPipelineRuns({ pipelineId: selectedId, limit: 20, offset })).data;
		},
	});
	const run = useQuery({
		queryKey: workflowQueryKey("pipeline-run", selectedId, selectedRun),
		enabled: !!selectedId && !!selectedRun,
		queryFn: async () => {
			if (!selectedId || !selectedRun) throw new Error("Select a run.");
			return (await api.v1GetPipelineRun({ pipelineId: selectedId, runId: selectedRun })).data;
		},
	});
	const create = useMutation({
		mutationFn: async () => {
			const source =
				sourceType === "list_snapshot"
					? { type: "list_snapshot" as const, list_id: Number(listId) }
					: { type: "push" as const, token_id: tokenId.trim(), accepted_format: "json" };
			if (
				!name.trim() ||
				(sourceType === "list_snapshot" &&
					(!Number.isSafeInteger(Number(listId)) || Number(listId) < 1)) ||
				(sourceType === "push" && !tokenId.trim())
			)
				throw new Error("Enter a name and valid source.");
			return (
				await api.v1CreatePipeline({
					createPipelineInput: {
						name: name.trim(),
						source,
						schedule: { cron: cron.trim(), timezone: timezone.trim() },
						delivery: { dashboard: true },
						status: "paused",
					},
				})
			).data;
		},
		onSuccess: async (data) => {
			setSelectedId(data.id);
			setName("");
			setNotice("Pipeline created in paused state. Review before resuming.");
			await client.invalidateQueries({ queryKey: workflowQueryKey("pipelines") });
		},
	});
	const action = useMutation({
		mutationFn: async (kind: "pause" | "resume" | "trigger" | "delete" | "rename" | "push") => {
			if (!selectedId) throw new Error("Select a pipeline.");
			if (kind === "pause") await api.v1PausePipeline({ pipelineId: selectedId });
			if (kind === "resume") await api.v1ResumePipeline({ pipelineId: selectedId });
			if (kind === "trigger")
				await api.v1TriggerPipeline({
					pipelineId: selectedId,
					triggerPipelineInput: { reason: "Manual run from workspace" },
				});
			if (kind === "delete") await api.v1DeletePipeline({ pipelineId: selectedId });
			if (kind === "rename")
				await api.v1UpdatePipeline({
					pipelineId: selectedId,
					updatePipelineInput: { name: rename.trim() },
				});
			if (kind === "push") {
				const emails = pushRows
					.split(/[\s,;]+/)
					.map((value) => value.trim())
					.filter(Boolean);
				if (!emails.length) throw new Error("Enter at least one address to push.");
				await api.v1PushPipeline({
					pipelineId: selectedId,
					idempotencyKey: crypto.randomUUID(),
					pushPipelineInput: { rows: emails.map((email) => ({ email })), email_column: "email" },
				});
			}
			return kind;
		},
		onSuccess: async (kind) => {
			setNotice(kind === "delete" ? "Pipeline deleted." : `Pipeline ${kind} request completed.`);
			if (kind === "delete") setSelectedId(undefined);
			if (kind === "push") setPushRows("");
			await client.invalidateQueries({ queryKey: workflowQueryKey("pipelines") });
			await client.invalidateQueries({ queryKey: workflowQueryKey("pipeline", selectedId) });
			await client.invalidateQueries({ queryKey: workflowQueryKey("pipeline-runs", selectedId) });
		},
	});
	const error = [list, detail, runs, run, create, action].find((request) => request.isError)?.error;
	return (
		<section data-slot="pipelines-panel" className={cn("space-y-6", className)} {...props}>
			<header>
				<h1 className="text-3xl font-semibold">Pipelines</h1>
				<p className="mt-2 text-muted-foreground">
					Schedule verification, run it on demand, and inspect each delivery.
				</p>
			</header>
			{error ? (
				<p role="alert" className="text-destructive">
					{error.message}
				</p>
			) : null}
			{notice ? <p role="status">{notice}</p> : null}
			<Card>
				<CardHeader>
					<CardTitle>Your pipelines</CardTitle>
				</CardHeader>
				<CardContent>
					{list.data?.pipelines.length ? (
						<ul className="divide-y">
							{list.data.pipelines.map((pipeline) => (
								<li className="flex items-center justify-between gap-3 py-3" key={pipeline.id}>
									<div>
										<p className="font-medium">{pipeline.name}</p>
										<p className="text-sm text-muted-foreground">
											{pipeline.status} · Next{" "}
											{pipeline.next_run_at
												? new Date(pipeline.next_run_at).toLocaleString()
												: "not scheduled"}
										</p>
									</div>
									<Button
										variant="outline"
										size="sm"
										onClick={() => {
											setSelectedId(pipeline.id);
											setSelectedRun(undefined);
											setOffset(0);
											setRename(pipeline.name);
										}}
									>
										Open
									</Button>
								</li>
							))}
						</ul>
					) : (
						<p>No pipelines yet.</p>
					)}
				</CardContent>
			</Card>
			{detail.data ? (
				<Card>
					<CardHeader>
						<CardTitle>{detail.data.name}</CardTitle>
					</CardHeader>
					<CardContent className="space-y-5">
						<p>
							{detail.data.status} · {detail.data.source.type.replaceAll("_", " ")} ·{" "}
							{detail.data.schedule.cron} ({detail.data.schedule.timezone})
						</p>
						<div className="flex flex-wrap gap-2">
							<Button
								variant="outline"
								disabled={action.isPending || detail.data.status === "paused"}
								onClick={() => {
									action.mutate("pause");
								}}
							>
								Pause
							</Button>
							<Button
								variant="outline"
								disabled={action.isPending || detail.data.status === "active"}
								onClick={() => {
									action.mutate("resume");
								}}
							>
								Resume
							</Button>
							<Button
								disabled={action.isPending}
								onClick={() => {
									if (window.confirm("Run this pipeline now?")) action.mutate("trigger");
								}}
							>
								Run now
							</Button>
							<Button
								variant="destructive"
								disabled={action.isPending}
								onClick={() => {
									if (window.confirm("Delete this pipeline and stop future runs?"))
										action.mutate("delete");
								}}
							>
								Delete
							</Button>
						</div>
						<form
							className="flex gap-2"
							onSubmit={(event: SyntheticEvent<HTMLFormElement>) => {
								event.preventDefault();
								if (rename.trim()) action.mutate("rename");
							}}
						>
							<Label className="sr-only" htmlFor="pipeline-rename">
								Pipeline name
							</Label>
							<Input
								id="pipeline-rename"
								value={rename}
								onChange={(event) => {
									setRename(event.target.value);
								}}
							/>
							<Button type="submit" variant="outline" disabled={action.isPending}>
								Save name
							</Button>
						</form>
						{detail.data.source.type === "push" ? (
							<div className="space-y-2">
								<Label htmlFor="pipeline-push">Push email addresses</Label>
								<textarea
									id="pipeline-push"
									aria-label="Push email addresses"
									className="min-h-24 w-full rounded border bg-background p-3"
									value={pushRows}
									onChange={(event) => {
										setPushRows(event.target.value);
									}}
								/>
								<Button
									disabled={action.isPending || !pushRows.trim()}
									onClick={() => {
										action.mutate("push");
									}}
								>
									Push rows
								</Button>
							</div>
						) : null}
						<div>
							<h3 className="font-semibold">Run history</h3>
							{runs.data?.runs.length ? (
								<ul className="divide-y">
									{runs.data.runs.map((item) => (
										<li className="flex items-center justify-between gap-3 py-3" key={item.id}>
											<div>
												<p>
													Run {String(item.id)} · {item.status}
												</p>
												<p className="text-sm text-muted-foreground">
													{new Date(item.created_at).toLocaleString()} · Delivery{" "}
													{item.delivery_status}
												</p>
											</div>
											<Button
												size="sm"
												variant="outline"
												onClick={() => {
													setSelectedRun(item.id);
												}}
											>
												Inspect
											</Button>
										</li>
									))}
								</ul>
							) : (
								<p>No runs yet.</p>
							)}
						</div>
						<div className="flex gap-2">
							<Button
								size="sm"
								variant="outline"
								disabled={offset === 0}
								onClick={() => {
									setOffset(Math.max(0, offset - 20));
								}}
							>
								Previous
							</Button>
							<Button
								size="sm"
								variant="outline"
								disabled={!runs.data || offset + 20 >= runs.data.total}
								onClick={() => {
									setOffset(offset + 20);
								}}
							>
								Next
							</Button>
						</div>
						{run.data ? (
							<div className="rounded border p-4">
								<p className="font-semibold">
									Run {String(run.data.id)} · {run.data.status}
								</p>
								<p>
									Delivery: {run.data.delivery_status} · Billed: {run.data.billed_emails}
								</p>
								{run.data.error_message ? (
									<p role="alert" className="text-destructive">
										{run.data.error_message}
									</p>
								) : null}
								{run.data.job_id ? (
									<Link className="underline" href={`/app/jobs?job=${String(run.data.job_id)}`}>
										View job results
									</Link>
								) : null}
							</div>
						) : null}
					</CardContent>
				</Card>
			) : null}
			<Card>
				<CardHeader>
					<CardTitle>Create a pipeline</CardTitle>
				</CardHeader>
				<CardContent>
					<form
						className="grid gap-3 sm:grid-cols-2"
						onSubmit={(event: SyntheticEvent<HTMLFormElement>) => {
							event.preventDefault();
							create.mutate();
						}}
					>
						<div>
							<Label htmlFor="pipeline-name">Name</Label>
							<Input
								id="pipeline-name"
								required
								value={name}
								onChange={(event) => {
									setName(event.target.value);
								}}
							/>
						</div>
						<div>
							<Label htmlFor="pipeline-source">Source</Label>
							<select
								id="pipeline-source"
								className="h-9 w-full rounded border bg-background px-3"
								value={sourceType}
								onChange={(event) => {
									setSourceType(event.target.value === "push" ? "push" : "list_snapshot");
								}}
							>
								<option value="list_snapshot">Saved list</option>
								<option value="push">Push API</option>
							</select>
						</div>
						{sourceType === "list_snapshot" ? (
							<div>
								<Label htmlFor="pipeline-list">List ID</Label>
								<Input
									id="pipeline-list"
									inputMode="numeric"
									value={listId}
									onChange={(event) => {
										setListId(event.target.value);
									}}
								/>
							</div>
						) : (
							<div>
								<Label htmlFor="pipeline-token">Push token ID</Label>
								<Input
									id="pipeline-token"
									value={tokenId}
									onChange={(event) => {
										setTokenId(event.target.value);
									}}
								/>
							</div>
						)}
						<div>
							<Label htmlFor="pipeline-cron">Cron schedule</Label>
							<Input
								id="pipeline-cron"
								value={cron}
								onChange={(event) => {
									setCron(event.target.value);
								}}
							/>
						</div>
						<div>
							<Label htmlFor="pipeline-timezone">Timezone</Label>
							<Input
								id="pipeline-timezone"
								value={timezone}
								onChange={(event) => {
									setTimezone(event.target.value);
								}}
							/>
						</div>
						<p className="text-sm text-muted-foreground sm:col-span-2">
							New pipelines start paused. Review the source and schedule before resuming.
						</p>
						<Button type="submit" disabled={create.isPending}>
							Create paused pipeline
						</Button>
					</form>
				</CardContent>
			</Card>
		</section>
	);
}
