"use client";

import "client-only";

import { useState, type ComponentPropsWithoutRef } from "react";
import { useQuery } from "@tanstack/react-query";
import { Configuration, PipelinesApi } from "@oppulence/reacher-sdk";

import { Button } from "@oppulence/ui/components/button";
import { cn } from "@oppulence/ui/lib/utils";

import {
	apiMessage,
	FILTER_CLASS,
} from "@/components/features/deliverability/outcome-events/outcome-events";
import { type Pipeline } from "@/components/features/automation/pipeline-form/pipeline-form.schema";
import {
	pipelineRunQuery,
	runCounts,
	when,
} from "@/components/features/automation/pipeline-run-detail/pipeline-run-detail";
import { EmptyState, Group, Pill, Row } from "@/components/platform/page-kit";
import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";
import { V1ListPipelines200Response } from "@/lib/api/generated/zod/pipelines/pipelines";
import { FlowArrow } from "@/lib/icons";
import {
	labelFor,
	PIPELINE_SOURCE_LABELS,
	pipelineRunStatus,
	pipelineStatus,
} from "@/lib/verification/labels";

import { type PipelineListPropsFields } from "./pipeline-list.schema";

/**
 * @oppulence-gen kind=component
 * PipelineList is a client data component.
 * Lists the pipelines with the status, the schedule, the next run, and the
 * last run with its counts. A status filter and pages limit the list.
 * Owned by `pipeline-list.lit.ts`.
 */
export type PipelineListProps = PipelineListPropsFields & {
	onOpen?: (pipelineId: number) => void;
} & ComponentPropsWithoutRef<"section">;

const api = new PipelinesApi(new Configuration({ basePath: "/api/backend" }));
const PAGE = 20;

/** The schedule in words, for example "0 9 * * 1 (UTC) · Next run 20/10/2026, 09:00:00". */
export function scheduleText(pipeline: Pipeline): string {
	if (pipeline.source.type === "push") return "Runs when your systems push rows";
	const next = when(pipeline.next_run_at);
	return `${pipeline.schedule.cron} (${pipeline.schedule.timezone}) · ${next ? `Next run ${next}` : "No next run"}`;
}

/** The status and the counts of the last run. The query polls while that run is active. */
function LastRun({ pipelineId, runId }: { pipelineId: number; runId: number }) {
	const run = useQuery(pipelineRunQuery(pipelineId, runId));
	if (!run.data) return null;
	const counts = runCounts(run.data.stats);
	return (
		<p className="mt-1 flex flex-wrap items-center gap-2 text-xs text-[#858585]">
			Last run
			<Pill tone={pipelineRunStatus.tone(run.data.status)}>
				{pipelineRunStatus.label(run.data.status)}
			</Pill>
			{when(run.data.completed_at ?? run.data.created_at)}
			{counts ? ` · ${counts}` : ""}
		</p>
	);
}

export function PipelineList({ className, selectedId, onOpen, ...props }: PipelineListProps) {
	const [status, setStatus] = useState<"" | "active" | "paused">("");
	const [offset, setOffset] = useState(0);
	const list = useQuery({
		queryKey: workflowQueryKey("pipelines", "list", status, offset),
		queryFn: async () =>
			V1ListPipelines200Response.parse(
				(await api.v1ListPipelines({ status: status || undefined, limit: PAGE, offset })).data,
			),
	});
	const total = list.data?.total ?? 0;
	const empty = !!list.data && !list.data.pipelines.length && !status && !offset;

	return (
		<section data-slot="pipeline-list" className={cn("flex flex-col gap-4", className)} {...props}>
			{list.isError ? (
				<p
					className="rounded-lg bg-red-500/10 px-4 py-3 text-sm text-red-300 ring-1 ring-red-500/20"
					role="alert"
				>
					{apiMessage(list.error)}
				</p>
			) : null}
			{empty ? null : (
				<select
					aria-label="Pipeline status"
					className={cn(FILTER_CLASS, "sm:w-48")}
					onChange={(event) => {
						const value = event.target.value;
						setStatus(value === "active" || value === "paused" ? value : "");
						setOffset(0);
					}}
					value={status}
				>
					<option value="">All statuses</option>
					<option value="active">Active</option>
					<option value="paused">Paused</option>
				</select>
			)}
			{empty ? (
				<EmptyState
					description="Create a pipeline to check a saved list on a schedule, or to check the rows that your systems push."
					icon={FlowArrow}
					title="No pipelines yet."
				/>
			) : (
				<Group>
					{list.data?.pipelines.length ? (
						list.data.pipelines.map((pipeline) => (
							<div
								className={cn(
									"flex flex-wrap items-center gap-x-4 gap-y-3 px-5 py-4",
									pipeline.id === selectedId && "bg-white/[0.03]",
								)}
								key={pipeline.id}
							>
								<div className="min-w-0 flex-1">
									<p className="truncate text-sm text-white">{pipeline.name}</p>
									<p className="text-xs break-words text-[#858585]">
										{labelFor(PIPELINE_SOURCE_LABELS, pipeline.source.type)} ·{" "}
										{scheduleText(pipeline)}
									</p>
									{pipeline.last_run_id ? (
										<LastRun pipelineId={pipeline.id} runId={pipeline.last_run_id} />
									) : (
										<p className="mt-1 text-xs text-[#858585]">No runs yet</p>
									)}
								</div>
								<Pill tone={pipelineStatus.tone(pipeline.status)}>
									{pipelineStatus.label(pipeline.status)}
								</Pill>
								<Button
									aria-label={`Open ${pipeline.name}`}
									aria-pressed={pipeline.id === selectedId}
									onClick={() => {
										onOpen?.(pipeline.id);
									}}
									size="sm"
									type="button"
									variant="secondary"
								>
									Open
								</Button>
							</div>
						))
					) : (
						<Row>
							<p className="text-sm text-[#858585]">
								{list.isPending ? "Loading pipelines…" : "No pipelines match this status."}
							</p>
						</Row>
					)}
				</Group>
			)}
			{total > PAGE ? (
				<div className="flex justify-end gap-2">
					<Button
						aria-label="Previous pipelines"
						className="rounded-lg"
						disabled={offset === 0}
						onClick={() => {
							setOffset(Math.max(0, offset - PAGE));
						}}
						size="sm"
						variant="outline"
					>
						Previous
					</Button>
					<Button
						aria-label="Next pipelines"
						className="rounded-lg"
						disabled={offset + PAGE >= total}
						onClick={() => {
							setOffset(offset + PAGE);
						}}
						size="sm"
						variant="outline"
					>
						Next
					</Button>
				</div>
			) : null}
		</section>
	);
}
