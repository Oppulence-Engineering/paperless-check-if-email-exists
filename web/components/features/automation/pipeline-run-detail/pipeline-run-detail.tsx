"use client";

import "client-only";

import Link from "next/link";
import type { ComponentPropsWithoutRef, ReactNode } from "react";
import { useQuery } from "@tanstack/react-query";
import { Configuration, PipelinesApi } from "@oppulence/reacher-sdk";

import { cn } from "@oppulence/ui/lib/utils";

import { apiMessage } from "@/components/features/deliverability/outcome-events/outcome-events";
import { Group, Pill, Row } from "@/components/platform/page-kit";
import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";
import { V1GetPipelineRun200Response } from "@/lib/api/generated/zod/pipelines/pipelines";
import {
	labelFor,
	PIPELINE_TRIGGER_LABELS,
	pipelineDeliveryStatus,
	pipelineRunStatus,
} from "@/lib/verification/labels";

import {
	FINAL_RUN_STATES,
	type PipelineRun,
	type PipelineRunDetailPropsFields,
} from "./pipeline-run-detail.schema";

/**
 * @oppulence-gen kind=component
 * PipelineRunDetail is a client data component.
 * Shows the stats, the webhook delivery, the errors, the links, and the times
 * of one pipeline run. It polls while the run is active and stops when the
 * run ends. Owned by `pipeline-run-detail.lit.ts`.
 */
export type PipelineRunDetailProps = PipelineRunDetailPropsFields &
	ComponentPropsWithoutRef<"section">;

const api = new PipelinesApi(new Configuration({ basePath: "/api/backend" }));

/** True while the run can still change. */
export function isRunActive(status: string): boolean {
	return !FINAL_RUN_STATES.includes(status);
}

/** The query for one run. The list and the detail share it, so they share the cache. */
export function pipelineRunQuery(pipelineId: number, runId: number) {
	return {
		queryKey: workflowQueryKey("pipelines", pipelineId, "runs", "run", runId),
		queryFn: async ({ signal }: { signal: AbortSignal }) =>
			V1GetPipelineRun200Response.parse(
				(await api.v1GetPipelineRun({ pipelineId, runId }, { signal })).data,
			),
		refetchInterval: (query: { state: { data?: PipelineRun } }) =>
			query.state.data && isRunActive(query.state.data.status) ? 5_000 : false,
	};
}

/** A local date and time, or undefined when the value is empty. */
export function when(value: string | null | undefined): string | undefined {
	return value ? new Date(value).toLocaleString() : undefined;
}

/** The result counts of a run, for example "2 checked · 1 risky · 1 invalid". */
export function runCounts(stats: PipelineRun["stats"]): string {
	const parts: [string, number | null | undefined][] = [
		["accepted", stats.accepted_rows],
		["checked", stats.total_processed],
		["valid", stats.total_valid],
		["risky", stats.total_risky],
		["unknown", stats.total_unknown],
		["invalid", stats.total_invalid],
		["failed", stats.total_failed],
	];
	return parts
		.filter(([label, value]) => value != null && (value > 0 || label === "checked"))
		.map(([label, value]) => `${String(value)} ${label}`)
		.join(" · ");
}

/** Label and value pairs. Pairs without a value do not show. */
export function Facts({ items }: { items: [string, ReactNode][] }) {
	return (
		<dl className="grid w-full grid-cols-2 gap-x-6 gap-y-4 lg:grid-cols-3">
			{items
				.filter(([, value]) => value != null && value !== "")
				.map(([label, value]) => (
					<div className="min-w-0" key={label}>
						<dt className="text-xs text-[#858585]">{label}</dt>
						<dd className="mt-0.5 text-sm break-words text-white">{value}</dd>
					</div>
				))}
		</dl>
	);
}

const LINK = "text-sm text-neutral-300 underline underline-offset-4 hover:text-white";

export function PipelineRunDetail({
	className,
	pipelineId,
	runId,
	...props
}: PipelineRunDetailProps) {
	const run = useQuery(pipelineRunQuery(pipelineId, runId));
	const data = run.data;
	const download = data?.result_location?.download_url;

	return (
		<section
			data-slot="pipeline-run-detail"
			className={cn("flex flex-col gap-4", className)}
			{...props}
		>
			{run.isError ? (
				<p
					className="rounded-lg bg-red-500/10 px-4 py-3 text-sm text-red-300 ring-1 ring-red-500/20"
					role="alert"
				>
					{apiMessage(run.error)}
				</p>
			) : null}
			{data ? (
				<Group>
					<Row
						description={
							isRunActive(data.status)
								? "This run is active. The page updates every few seconds."
								: `Started by ${labelFor(PIPELINE_TRIGGER_LABELS, data.trigger_type).toLowerCase()}.`
						}
						label={`Run ${String(data.id)}`}
						layout="inline"
					>
						<span role="status">
							<Pill tone={pipelineRunStatus.tone(data.status)}>
								{pipelineRunStatus.label(data.status)}
							</Pill>
						</span>
					</Row>
					{data.error_message ? (
						<Row>
							<p className="text-sm text-red-300" role="alert">
								{data.error_code ? `${data.error_code}: ` : ""}
								{data.error_message}
							</p>
						</Row>
					) : null}
					<Row label="Times" layout="stacked">
						<Facts
							items={[
								["Trigger", labelFor(PIPELINE_TRIGGER_LABELS, data.trigger_type)],
								["Reason", data.stats.trigger_reason],
								["Created", when(data.created_at)],
								["Scheduled for", when(data.scheduled_for)],
								["Started", when(data.started_at)],
								["Completed", when(data.completed_at)],
								["Updated", when(data.updated_at)],
							]}
						/>
					</Row>
					<Row label="Stats" layout="stacked">
						<Facts
							items={[
								["Source", data.stats.source_name],
								["Accepted rows", data.stats.accepted_rows],
								["Rows in source", data.stats.source_row_count],
								["Unique addresses", data.stats.source_unique_emails],
								["Selected for checking", data.stats.selected_unique_emails],
								["Skipped as unchanged", data.stats.skipped_unchanged],
								["Checked", data.stats.total_processed],
								["Valid", data.stats.total_valid],
								["Risky", data.stats.total_risky],
								["Unknown", data.stats.total_unknown],
								["Invalid", data.stats.total_invalid],
								["Failed checks", data.stats.total_failed],
								["Billed addresses", data.billed_emails],
							]}
						/>
					</Row>
					<Row label="Webhook delivery" layout="stacked">
						<Facts
							items={[
								[
									"Status",
									<Pill key="delivery" tone={pipelineDeliveryStatus.tone(data.delivery_status)}>
										{pipelineDeliveryStatus.label(data.delivery_status)}
									</Pill>,
								],
								["Attempts", data.delivery_attempts],
								["Last attempt", when(data.last_delivery_attempt_at)],
								["Next attempt", when(data.next_delivery_attempt_at)],
								[
									"Last error",
									data.delivery_error ? (
										<span className="text-red-300" role="alert">
											{data.delivery_error}
										</span>
									) : undefined,
								],
							]}
						/>
					</Row>
					{download?.startsWith("/") || data.list_id || data.job_id ? (
						<Row>
							<div className="flex flex-wrap gap-x-6 gap-y-2">
								{download?.startsWith("/") ? (
									<a className={LINK} download href={`/api/backend${download}`}>
										Download results
									</a>
								) : null}
								{data.list_id ? (
									<Link className={LINK} href={`/app/lists/detail?list=${String(data.list_id)}`}>
										Open result list
									</Link>
								) : null}
								{data.job_id ? (
									<Link className={LINK} href={`/app/jobs?job=${String(data.job_id)}`}>
										View job results
									</Link>
								) : null}
							</div>
						</Row>
					) : null}
				</Group>
			) : run.isPending ? (
				<p className="text-sm text-[#858585]">Loading the run…</p>
			) : null}
		</section>
	);
}
