"use client";

import "client-only";

import { useState, type ComponentPropsWithoutRef } from "react";

import { Button } from "@oppulence/ui/components/button";
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetHeader,
	SheetTitle,
	SheetTrigger,
} from "@oppulence/ui/components/sheet";
import { cn } from "@oppulence/ui/lib/utils";
import { PipelineDetail } from "@/components/features/automation/pipeline-detail/pipeline-detail";
import { PipelineForm } from "@/components/features/automation/pipeline-form/pipeline-form";
import { PipelineList } from "@/components/features/automation/pipeline-list/pipeline-list";
import { PipelineRunDetail } from "@/components/features/automation/pipeline-run-detail/pipeline-run-detail";
import { PipelineRuns } from "@/components/features/automation/pipeline-runs/pipeline-runs";
import { Page, Section } from "@/components/platform/page-kit";

import { type PipelinesPanelPropsFields } from "./pipelines-panel.schema";

/** @oppulence-gen kind=component Owned by `pipelines-panel.lit.ts`. */
export type PipelinesPanelProps = PipelinesPanelPropsFields & ComponentPropsWithoutRef<"section">;

/**
 * The results come first: the pipelines, then the selected pipeline, its runs,
 * and the selected run. The create form opens in a sheet.
 */
export function PipelinesPanel({ className, ...props }: PipelinesPanelProps) {
	const [selectedId, setSelectedId] = useState<number>();
	const [runId, setRunId] = useState<number>();
	const [creating, setCreating] = useState(false);
	const [notice, setNotice] = useState("");
	function select(pipelineId?: number) {
		setSelectedId(pipelineId);
		setRunId(undefined);
	}

	return (
		<Page
			actions={
				<Sheet onOpenChange={setCreating} open={creating}>
					<SheetTrigger asChild>
						<Button className="rounded-lg" size="sm" type="button">
							New pipeline
						</Button>
					</SheetTrigger>
					<SheetContent className="flex w-full flex-col gap-6 overflow-y-auto pb-8 sm:max-w-xl">
						<SheetHeader>
							<SheetTitle>Create a pipeline</SheetTitle>
							<SheetDescription>
								New pipelines start paused. Review the source and the schedule before you resume.
							</SheetDescription>
						</SheetHeader>
						<PipelineForm
							className="px-4"
							onSaved={(pipeline) => {
								setCreating(false);
								select(pipeline.id);
								setNotice("Pipeline created in paused state. Review before resuming.");
							}}
						/>
					</SheetContent>
				</Sheet>
			}
			description="Schedule verification, run it on demand, and inspect each delivery."
			title="Pipelines"
		>
			<section
				data-slot="pipelines-panel"
				className={cn("flex flex-col gap-10", className)}
				{...props}
			>
				{notice ? (
					<p
						role="status"
						className="rounded-lg bg-emerald-500/10 px-4 py-3 text-sm text-emerald-300 ring-1 ring-emerald-500/20"
					>
						{notice}
					</p>
				) : null}
				<Section title="Your pipelines">
					<PipelineList
						onOpen={(pipelineId) => {
							select(pipelineId);
							setNotice("");
						}}
						selectedId={selectedId}
					/>
				</Section>
				{selectedId ? (
					<>
						<PipelineDetail
							key={selectedId}
							onDeleted={() => {
								select(undefined);
								setNotice("Pipeline deleted.");
							}}
							onRun={setRunId}
							pipelineId={selectedId}
						/>
						<Section title="Run history">
							<PipelineRuns
								key={selectedId}
								onInspect={setRunId}
								pipelineId={selectedId}
								selectedRunId={runId}
							/>
							{runId ? <PipelineRunDetail pipelineId={selectedId} runId={runId} /> : null}
						</Section>
					</>
				) : null}
			</section>
		</Page>
	);
}
