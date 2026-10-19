"use client";

import "client-only";

import { useState, type ComponentPropsWithoutRef } from "react";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { Configuration, PipelinesApi } from "@oppulence/reacher-sdk";

import { Button } from "@oppulence/ui/components/button";
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetHeader,
	SheetTitle,
} from "@oppulence/ui/components/sheet";
import { Textarea } from "@oppulence/ui/components/textarea";
import { cn } from "@oppulence/ui/lib/utils";

import { apiMessage } from "@/components/features/deliverability/outcome-events/outcome-events";
import { PipelineForm } from "@/components/features/automation/pipeline-form/pipeline-form";
import { scheduleText } from "@/components/features/automation/pipeline-list/pipeline-list";
import {
	Facts,
	when,
} from "@/components/features/automation/pipeline-run-detail/pipeline-run-detail";
import { Group, Pill, Row, Section } from "@/components/platform/page-kit";
import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";
import {
	V1GetPipeline200Response,
	V1PushPipeline202Response,
	V1TriggerPipeline202Response,
} from "@/lib/api/generated/zod/pipelines/pipelines";
import {
	labelFor,
	PIPELINE_SOURCE_LABELS,
	pipelineRunStatus,
	pipelineStatus,
} from "@/lib/verification/labels";

import { type PipelineDetailPropsFields } from "./pipeline-detail.schema";

/**
 * @oppulence-gen kind=component
 * PipelineDetail is a client data component.
 * Shows the settings of one pipeline and its actions: pause, resume, run,
 * edit, and delete. A push pipeline also gets a form that pushes rows.
 * Owned by `pipeline-detail.lit.ts`.
 */
export type PipelineDetailProps = PipelineDetailPropsFields & {
	/** Gets the ID of the run that "Run now" or a push starts. */
	onRun?: (runId: number) => void;
	onDeleted?: () => void;
} & ComponentPropsWithoutRef<"section">;

const api = new PipelinesApi(new Configuration({ basePath: "/api/backend" }));

type Action = "pause" | "resume" | "trigger" | "delete";

export function PipelineDetail({
	className,
	pipelineId,
	onRun,
	onDeleted,
	...props
}: PipelineDetailProps) {
	const client = useQueryClient();
	const [notice, setNotice] = useState("");
	const [editing, setEditing] = useState(false);
	const [rows, setRows] = useState("");
	const detail = useQuery({
		queryKey: workflowQueryKey("pipelines", pipelineId),
		queryFn: async () =>
			V1GetPipeline200Response.parse((await api.v1GetPipeline({ pipelineId })).data),
	});
	const action = useMutation({
		mutationFn: async (kind: Action) => {
			if (kind === "pause") await api.v1PausePipeline({ pipelineId });
			if (kind === "resume") await api.v1ResumePipeline({ pipelineId });
			if (kind === "delete") await api.v1DeletePipeline({ pipelineId });
			if (kind !== "trigger") return { kind };
			const run = V1TriggerPipeline202Response.parse(
				(
					await api.v1TriggerPipeline({
						pipelineId,
						triggerPipelineInput: { reason: "Manual run from workspace" },
					})
				).data,
			);
			return { kind, runId: run.run_id };
		},
		onSuccess: async ({ kind, runId }) => {
			if (kind === "delete") {
				onDeleted?.();
				await client.invalidateQueries({ queryKey: workflowQueryKey("pipelines", "list") });
				return;
			}
			setNotice(
				kind === "pause"
					? "Pipeline paused. Scheduled runs stop."
					: kind === "resume"
						? "Pipeline resumed."
						: `Run ${String(runId)} started.`,
			);
			if (runId) onRun?.(runId);
			await client.invalidateQueries({ queryKey: workflowQueryKey("pipelines") });
		},
	});
	const push = useMutation({
		mutationFn: async () => {
			const emails = rows
				.split(/[\s,;]+/)
				.map((value) => value.trim())
				.filter(Boolean);
			if (!emails.length) throw new Error("Enter at least one address to push.");
			return V1PushPipeline202Response.parse(
				(
					await api.v1PushPipeline({
						pipelineId,
						idempotencyKey: crypto.randomUUID(),
						pushPipelineInput: { rows: emails.map((email) => ({ email })), email_column: "email" },
					})
				).data,
			);
		},
		onSuccess: async (data) => {
			setRows("");
			onRun?.(data.run_id);
			await client.invalidateQueries({ queryKey: workflowQueryKey("pipelines") });
		},
	});
	const error = [detail, action].find((request) => request.isError)?.error;
	const pipeline = detail.data;
	if (!pipeline)
		return error ? (
			<p className="text-sm text-red-300" role="alert">
				{apiMessage(error)}
			</p>
		) : null;
	const isPush = pipeline.source.type === "push";
	const webhook = pipeline.delivery.webhook;

	return (
		<section
			data-slot="pipeline-detail"
			className={cn("flex flex-col gap-4", className)}
			{...props}
		>
			{error ? (
				<p
					className="rounded-lg bg-red-500/10 px-4 py-3 text-sm text-red-300 ring-1 ring-red-500/20"
					role="alert"
				>
					{apiMessage(error)}
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
			<Section
				description={`${labelFor(PIPELINE_SOURCE_LABELS, pipeline.source.type)} · ${scheduleText(pipeline)}`}
				title={pipeline.name}
			>
				<div className="flex flex-wrap items-center gap-2">
					<Pill tone={pipelineStatus.tone(pipeline.status)}>
						{pipelineStatus.label(pipeline.status)}
					</Pill>
					<Button
						className="rounded-lg"
						disabled={action.isPending || pipeline.status === "paused"}
						onClick={() => {
							action.mutate("pause");
						}}
						size="sm"
						variant="outline"
					>
						Pause
					</Button>
					<Button
						className="rounded-lg"
						disabled={action.isPending || pipeline.status === "active"}
						onClick={() => {
							action.mutate("resume");
						}}
						size="sm"
						variant="outline"
					>
						Resume
					</Button>
					{isPush ? null : (
						<Button
							className="rounded-lg"
							disabled={action.isPending}
							onClick={() => {
								if (window.confirm("Run this pipeline now?")) action.mutate("trigger");
							}}
							size="sm"
						>
							Run now
						</Button>
					)}
					<Button
						className="rounded-lg"
						onClick={() => {
							setEditing(true);
						}}
						size="sm"
						variant="outline"
					>
						Edit
					</Button>
					<Button
						className="rounded-lg"
						disabled={action.isPending}
						onClick={() => {
							if (window.confirm("Delete this pipeline and stop future runs?"))
								action.mutate("delete");
						}}
						size="sm"
						variant="destructive"
					>
						Delete
					</Button>
				</div>
				<Group>
					<Row>
						<Facts
							items={[
								[
									"Source",
									pipeline.source.type === "list_snapshot"
										? `List ${String(pipeline.source.list_id)}`
										: pipeline.source.type === "push"
											? `Push token ${pipeline.source.token_id}`
											: labelFor(PIPELINE_SOURCE_LABELS, pipeline.source.type),
								],
								["Next run", isPush ? undefined : (when(pipeline.next_run_at) ?? "Not scheduled")],
								["Last scheduled run", when(pipeline.last_scheduled_at)],
								[
									"Missed-run window",
									isPush
										? undefined
										: `${String(pipeline.policy.missed_run_window_hours ?? 24)} hours`,
								],
								[
									"Verification",
									pipeline.verification.delta_mode
										? "Only new or changed addresses"
										: "Every address in each run",
								],
								[
									"Check again after",
									pipeline.verification.delta_mode && pipeline.verification.freshness_days
										? `${String(pipeline.verification.freshness_days)} days`
										: undefined,
								],
								["Webhook", webhook?.url ?? "Not set"],
								[
									"Webhook headers",
									webhook?.headers && Object.keys(webhook.headers).length
										? Object.keys(webhook.headers).join(", ")
										: undefined,
								],
								[
									"Delivery attempts",
									webhook
										? `${String(pipeline.delivery.max_attempts ?? 5)}, ${String(pipeline.delivery.retry_backoff_seconds ?? 300)} seconds apart`
										: undefined,
								],
								["Created", when(pipeline.created_at)],
								["Updated", when(pipeline.updated_at)],
							]}
						/>
					</Row>
					{isPush ? (
						<Row
							description={
								pipeline.status === "active"
									? "Enter email addresses, one on each line or with commas between them."
									: "Resume the pipeline before you push rows."
							}
							label="Push rows"
							layout="stacked"
						>
							<div className="flex w-full flex-col gap-3">
								<Textarea
									aria-label="Push email addresses"
									className="min-h-24 rounded-lg"
									onChange={(event) => {
										setRows(event.target.value);
									}}
									value={rows}
								/>
								{push.isError ? (
									<p className="text-sm text-red-300" role="alert">
										{apiMessage(push.error)}
									</p>
								) : null}
								{push.data ? (
									<p
										className="flex flex-wrap items-center gap-2 text-sm text-emerald-300"
										role="status"
									>
										Run {String(push.data.run_id)} accepted {String(push.data.accepted_rows)} rows
										{push.data.replayed ? " (the API returned the earlier batch)" : ""}.
										<Pill tone={pipelineRunStatus.tone(push.data.status)}>
											{pipelineRunStatus.label(push.data.status)}
										</Pill>
									</p>
								) : null}
								<div className="flex justify-end">
									<Button
										className="rounded-lg"
										disabled={push.isPending || !rows.trim() || pipeline.status !== "active"}
										onClick={() => {
											push.mutate();
										}}
									>
										Push rows
									</Button>
								</div>
							</div>
						</Row>
					) : null}
				</Group>
			</Section>
			<Sheet onOpenChange={setEditing} open={editing}>
				<SheetContent className="flex w-full flex-col gap-6 overflow-y-auto pb-8 sm:max-w-xl">
					<SheetHeader>
						<SheetTitle>Edit {pipeline.name}</SheetTitle>
						<SheetDescription>
							Change the name, the source, the schedule, and the delivery.
						</SheetDescription>
					</SheetHeader>
					<PipelineForm
						className="px-4"
						onSaved={() => {
							setEditing(false);
							setNotice("Pipeline saved.");
						}}
						pipeline={pipeline}
					/>
				</SheetContent>
			</Sheet>
		</section>
	);
}
