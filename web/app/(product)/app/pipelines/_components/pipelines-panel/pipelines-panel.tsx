"use client";

import "client-only";

import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";

import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { Configuration, PipelinesApi } from "@oppulence/reacher-sdk";
import Link from "next/link";
import { useState, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";

import { Button } from "@oppulence/ui/components/button";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { cn } from "@oppulence/ui/lib/utils";
import { Group, Page, Pill, Row, Section } from "@/components/platform/page-kit";

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
	const field = "h-10 rounded-lg md:w-72";
	return (
		<Page
			description="Schedule verification, run it on demand, and inspect each delivery."
			title="Pipelines"
		>
			<section
				data-slot="pipelines-panel"
				className={cn("flex flex-col gap-10", className)}
				{...props}
			>
				{error ? (
					<p
						role="alert"
						className="rounded-lg bg-red-500/10 px-4 py-3 text-sm text-red-300 ring-1 ring-red-500/20"
					>
						{error.message}
					</p>
				) : null}
				{notice ? (
					<p
						role="status"
						className="rounded-lg bg-emerald-500/10 px-4 py-3 text-sm text-emerald-300 ring-1 ring-emerald-500/20"
					>
						{notice}
					</p>
				) : null}
				<Section title="Your pipelines">
					<Group>
						{list.data?.pipelines.length ? (
							list.data.pipelines.map((pipeline) => (
								<div
									className="flex flex-wrap items-center gap-x-4 gap-y-3 px-5 py-4"
									key={pipeline.id}
								>
									<div className="min-w-0 flex-1">
										<p className="truncate text-sm text-white">{pipeline.name}</p>
										<p className="text-xs text-[#858585]">
											Next{" "}
											{pipeline.next_run_at
												? new Date(pipeline.next_run_at).toLocaleString()
												: "not scheduled"}
										</p>
									</div>
									<Pill tone={pipeline.status === "active" ? "green" : "gray"}>
										{pipeline.status}
									</Pill>
									<Button
										variant="secondary"
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
								</div>
							))
						) : (
							<Row>
								<p className="text-sm text-[#858585]">No pipelines yet.</p>
							</Row>
						)}
					</Group>
				</Section>
				{detail.data ? (
					<Section
						description={`${detail.data.status} · ${detail.data.source.type.replaceAll("_", " ")} · ${detail.data.schedule.cron} (${detail.data.schedule.timezone})`}
						title={detail.data.name}
					>
						<Group>
							<Row>
								<div className="flex flex-wrap gap-2">
									<Button
										className="rounded-lg"
										variant="outline"
										disabled={action.isPending || detail.data.status === "paused"}
										onClick={() => {
											action.mutate("pause");
										}}
									>
										Pause
									</Button>
									<Button
										className="rounded-lg"
										variant="outline"
										disabled={action.isPending || detail.data.status === "active"}
										onClick={() => {
											action.mutate("resume");
										}}
									>
										Resume
									</Button>
									<Button
										className="rounded-lg"
										disabled={action.isPending}
										onClick={() => {
											if (window.confirm("Run this pipeline now?")) action.mutate("trigger");
										}}
									>
										Run now
									</Button>
									<Button
										className="rounded-lg"
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
							</Row>
							<Row label={<Label htmlFor="pipeline-rename">Pipeline name</Label>}>
								<form
									className="flex w-full gap-2 md:justify-end"
									onSubmit={(event: SyntheticEvent<HTMLFormElement>) => {
										event.preventDefault();
										if (rename.trim()) action.mutate("rename");
									}}
								>
									<Input
										className={field}
										id="pipeline-rename"
										value={rename}
										onChange={(event) => {
											setRename(event.target.value);
										}}
									/>
									<Button
										className="h-10 rounded-lg"
										type="submit"
										variant="outline"
										disabled={action.isPending}
									>
										Save name
									</Button>
								</form>
							</Row>
							{detail.data.source.type === "push" ? (
								<Row
									label={<Label htmlFor="pipeline-push">Push email addresses</Label>}
									layout="stacked"
								>
									<div className="flex w-full flex-col gap-3">
										<textarea
											id="pipeline-push"
											aria-label="Push email addresses"
											className="min-h-24 w-full rounded-lg border border-input bg-transparent px-3 py-2 text-sm dark:bg-input/30"
											value={pushRows}
											onChange={(event) => {
												setPushRows(event.target.value);
											}}
										/>
										<div className="flex justify-end">
											<Button
												className="rounded-lg"
												disabled={action.isPending || !pushRows.trim()}
												onClick={() => {
													action.mutate("push");
												}}
											>
												Push rows
											</Button>
										</div>
									</div>
								</Row>
							) : null}
						</Group>
						<Section title="Run history">
							<Group>
								{runs.data?.runs.length ? (
									runs.data.runs.map((item) => (
										<div
											className="flex flex-wrap items-center gap-x-4 gap-y-3 px-5 py-4"
											key={item.id}
										>
											<div className="min-w-0 flex-1">
												<p className="truncate text-sm text-white">Run {String(item.id)}</p>
												<p className="text-xs text-[#858585]">
													{new Date(item.created_at).toLocaleString()} · Delivery{" "}
													{item.delivery_status}
												</p>
											</div>
											<Pill>{item.status}</Pill>
											<Button
												size="sm"
												variant="secondary"
												onClick={() => {
													setSelectedRun(item.id);
												}}
											>
												Inspect
											</Button>
										</div>
									))
								) : (
									<Row>
										<p className="text-sm text-[#858585]">No runs yet.</p>
									</Row>
								)}
							</Group>
							<div className="flex justify-end gap-2">
								<Button
									className="rounded-lg"
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
									className="rounded-lg"
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
								<Group>
									<Row
										description={`Delivery: ${run.data.delivery_status} · Billed: ${String(run.data.billed_emails)}`}
										label={`Run ${String(run.data.id)} · ${run.data.status}`}
									>
										{run.data.job_id ? (
											<Link
												className="text-sm text-neutral-300 underline hover:text-white"
												href={`/app/jobs?job=${String(run.data.job_id)}`}
											>
												View job results
											</Link>
										) : null}
									</Row>
									{run.data.error_message ? (
										<Row>
											<p role="alert" className="text-sm text-red-300">
												{run.data.error_message}
											</p>
										</Row>
									) : null}
								</Group>
							) : null}
						</Section>
					</Section>
				) : null}
				<Section
					description="New pipelines start paused. Review the source and schedule before resuming."
					title="Create a pipeline"
				>
					<form
						className="flex flex-col gap-4"
						onSubmit={(event: SyntheticEvent<HTMLFormElement>) => {
							event.preventDefault();
							create.mutate();
						}}
					>
						<Group>
							<Row label={<Label htmlFor="pipeline-name">Name</Label>}>
								<Input
									className={field}
									id="pipeline-name"
									required
									value={name}
									onChange={(event) => {
										setName(event.target.value);
									}}
								/>
							</Row>
							<Row label={<Label htmlFor="pipeline-source">Source</Label>}>
								<select
									id="pipeline-source"
									className="h-10 w-full rounded-lg bg-white/[0.03] px-2 text-sm text-neutral-200 ring-1 ring-white/[0.08] outline-none md:w-72"
									value={sourceType}
									onChange={(event) => {
										setSourceType(event.target.value === "push" ? "push" : "list_snapshot");
									}}
								>
									<option value="list_snapshot">Saved list</option>
									<option value="push">Push API</option>
								</select>
							</Row>
							{sourceType === "list_snapshot" ? (
								<Row label={<Label htmlFor="pipeline-list">List ID</Label>}>
									<Input
										className={field}
										id="pipeline-list"
										inputMode="numeric"
										value={listId}
										onChange={(event) => {
											setListId(event.target.value);
										}}
									/>
								</Row>
							) : (
								<Row label={<Label htmlFor="pipeline-token">Push token ID</Label>}>
									<Input
										className={field}
										id="pipeline-token"
										value={tokenId}
										onChange={(event) => {
											setTokenId(event.target.value);
										}}
									/>
								</Row>
							)}
							<Row label={<Label htmlFor="pipeline-cron">Cron schedule</Label>}>
								<Input
									className={field}
									id="pipeline-cron"
									value={cron}
									onChange={(event) => {
										setCron(event.target.value);
									}}
								/>
							</Row>
							<Row label={<Label htmlFor="pipeline-timezone">Timezone</Label>}>
								<Input
									className={field}
									id="pipeline-timezone"
									value={timezone}
									onChange={(event) => {
										setTimezone(event.target.value);
									}}
								/>
							</Row>
						</Group>
						<div className="flex justify-end">
							<Button className="rounded-lg" type="submit" disabled={create.isPending}>
								Create paused pipeline
							</Button>
						</div>
					</form>
				</Section>
			</section>
		</Page>
	);
}
