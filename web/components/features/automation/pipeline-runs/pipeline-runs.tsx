"use client";

import "client-only";

import { useState, type ComponentPropsWithoutRef } from "react";
import { useQuery } from "@tanstack/react-query";
import { Configuration, PipelinesApi } from "@oppulence/reacher-sdk";

import { Button } from "@oppulence/ui/components/button";
import { cn } from "@oppulence/ui/lib/utils";

import { apiMessage } from "@/components/features/deliverability/outcome-events/outcome-events";
import {
	isRunActive,
	runCounts,
	when,
} from "@/components/features/automation/pipeline-run-detail/pipeline-run-detail";
import { V1ListPipelineRuns200Response } from "@/lib/api/generated/zod/pipelines/pipelines";
import { Group, Pill, Row } from "@/components/platform/page-kit";
import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";
import {
	labelFor,
	PIPELINE_TRIGGER_LABELS,
	pipelineDeliveryStatus,
	pipelineRunStatus,
} from "@/lib/verification/labels";

import { type PipelineRunsPropsFields } from "./pipeline-runs.schema";

/**
 * @oppulence-gen kind=component
 * PipelineRuns is a client data component.
 * Lists the runs of one pipeline, newest first. It polls while a run on the
 * page is active. Owned by `pipeline-runs.lit.ts`.
 */
export type PipelineRunsProps = PipelineRunsPropsFields & {
	onInspect?: (runId: number) => void;
} & ComponentPropsWithoutRef<"section">;

const api = new PipelinesApi(new Configuration({ basePath: "/api/backend" }));
const PAGE = 20;

export function PipelineRuns({
	className,
	pipelineId,
	selectedRunId,
	onInspect,
	...props
}: PipelineRunsProps) {
	const [offset, setOffset] = useState(0);
	const runs = useQuery({
		queryKey: workflowQueryKey("pipelines", pipelineId, "runs", "page", offset),
		queryFn: async () =>
			V1ListPipelineRuns200Response.parse(
				(await api.v1ListPipelineRuns({ pipelineId, limit: PAGE, offset })).data,
			),
		refetchInterval: (query) =>
			query.state.data?.runs.some((run) => isRunActive(run.status)) ? 3_000 : false,
	});
	const total = runs.data?.total ?? 0;

	return (
		<section data-slot="pipeline-runs" className={cn("flex flex-col gap-4", className)} {...props}>
			{runs.isError ? (
				<p
					className="rounded-lg bg-red-500/10 px-4 py-3 text-sm text-red-300 ring-1 ring-red-500/20"
					role="alert"
				>
					{apiMessage(runs.error)}
				</p>
			) : null}
			<Group>
				{runs.data?.runs.length ? (
					runs.data.runs.map((run) => (
						<div
							className={cn(
								"flex flex-wrap items-center gap-x-4 gap-y-3 px-5 py-4",
								run.id === selectedRunId && "bg-white/[0.03]",
							)}
							key={run.id}
						>
							<div className="min-w-0 flex-1">
								<p className="truncate text-sm text-white">
									Run {String(run.id)} · {labelFor(PIPELINE_TRIGGER_LABELS, run.trigger_type)}
								</p>
								<p className="text-xs break-words text-[#858585]">
									{when(run.created_at)} · {pipelineDeliveryStatus.label(run.delivery_status)}
									{runCounts(run.stats) ? ` · ${runCounts(run.stats)}` : ""}
								</p>
							</div>
							<Pill tone={pipelineRunStatus.tone(run.status)}>
								{pipelineRunStatus.label(run.status)}
							</Pill>
							<Button
								aria-pressed={run.id === selectedRunId}
								onClick={() => {
									onInspect?.(run.id);
								}}
								size="sm"
								type="button"
								variant="secondary"
							>
								Inspect
							</Button>
						</div>
					))
				) : (
					<Row>
						<p className="text-sm text-[#858585]">
							{runs.isPending ? "Loading runs…" : "No runs yet."}
						</p>
					</Row>
				)}
			</Group>
			{total > PAGE ? (
				<div className="flex justify-end gap-2">
					<Button
						aria-label="Previous runs"
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
						aria-label="Next runs"
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
