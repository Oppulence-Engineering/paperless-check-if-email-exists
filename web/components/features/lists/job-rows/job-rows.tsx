"use client";

import "client-only";
import { errorMessage } from "@/lib/api/error-message";

import { useState, type ComponentPropsWithoutRef } from "react";
import { useInfiniteQuery } from "@tanstack/react-query";
import { Configuration, JobsApi } from "@oppulence/reacher-sdk";

import { Button } from "@oppulence/ui/components/button";
import { Label } from "@oppulence/ui/components/label";
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetHeader,
	SheetTitle,
} from "@oppulence/ui/components/sheet";
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from "@oppulence/ui/components/select";
import { ALL, fromSelect } from "@/lib/select";
import { cn } from "@oppulence/ui/lib/utils";

import { Group, Pill } from "@/components/platform/page-kit";
import { AddressTimeline } from "@/components/features/verification/address-timeline/address-timeline";
import { ProviderDetails } from "@/components/features/verification/provider-details/provider-details";
import { RiskFactors } from "@/components/features/verification/risk-factors/risk-factors";
import { SignalChecklist } from "@/components/features/verification/signal-checklist/signal-checklist";
import { TechnicalDetails } from "@/components/features/verification/technical-details/technical-details";
import { VerdictSummary } from "@/components/features/verification/verdict-summary/verdict-summary";
import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";
import { V1GetJobResults200Response } from "@/lib/api/generated/zod/jobs/jobs";
import { type JobResultRow } from "@/lib/api/schemas/jobs";
import { primaryAction } from "@/lib/verification/check-result";
import {
	category,
	labelFor,
	REASON_LABELS,
	recommendation,
	taskState,
} from "@/lib/verification/labels";

import { type JobRowsPropsFields } from "./job-rows.schema";

/**
 * @oppulence-gen kind=component
 * JobRows is a client data component.
 * Shows the result of each row in one bulk job and opens one row in a sheet.
 *
 * It reads result pages through the generated SDK and TanStack Query. The
 * state filter goes to the API. The category filter applies to the loaded rows,
 * because the results endpoint has no category filter. Owned by `job-rows.lit.ts`.
 */
export type JobRowsProps = JobRowsPropsFields & ComponentPropsWithoutRef<"section">;

const jobsApi = new JobsApi(new Configuration({ basePath: "/api/backend" }));
const SELECT_CLASS = "h-10 w-full rounded-lg sm:w-48";

export function JobRows({ className, jobId, ...props }: JobRowsProps) {
	const [state, setState] = useState("completed");
	const [filter, setFilter] = useState("");
	const [open, setOpen] = useState<JobResultRow>();
	const pages = useInfiniteQuery({
		queryKey: workflowQueryKey("jobs", jobId, "results", state),
		initialPageParam: undefined as number | undefined,
		queryFn: async ({ pageParam }) =>
			V1GetJobResults200Response.parse(
				(
					await jobsApi.v1GetJobResults({
						jobId,
						cursor: pageParam,
						limit: 50,
						state,
					})
				).data,
			),
		getNextPageParam: (last) => (last.has_more ? (last.next_cursor ?? undefined) : undefined),
	});
	const allRows = pages.data?.pages.flatMap((page) => page.results) ?? [];
	const rowNumbers = new Map(allRows.map((row) => [row.id, row.row_number]));
	const rows = allRows.filter((row) => !filter || row.result?.score.category === filter);

	return (
		<section data-slot="job-rows" className={cn("flex flex-col gap-4", className)} {...props}>
			<div className="flex flex-col gap-3 sm:flex-row sm:items-end">
				<div className="flex flex-col gap-1.5">
					<Label
						className="text-xs font-medium text-neutral-400"
						htmlFor={`rows-state-${String(jobId)}`}
					>
						Row state
					</Label>
					<Select
						onValueChange={(value) => {
							setState(value);
						}}
						value={state}
					>
						<SelectTrigger className={SELECT_CLASS} id={`rows-state-${String(jobId)}`}>
							<SelectValue />
						</SelectTrigger>
						<SelectContent>
							{taskState.values.map((value) => (
								<SelectItem key={value} value={value}>
									{taskState.label(value)}
								</SelectItem>
							))}
						</SelectContent>
					</Select>
				</div>
				<div className="flex flex-col gap-1.5">
					<Label
						className="text-xs font-medium text-neutral-400"
						htmlFor={`rows-category-${String(jobId)}`}
					>
						Category
					</Label>
					<Select
						onValueChange={(value) => {
							setFilter(fromSelect(value));
						}}
						value={filter || ALL}
					>
						<SelectTrigger className={SELECT_CLASS} id={`rows-category-${String(jobId)}`}>
							<SelectValue />
						</SelectTrigger>
						<SelectContent>
							<SelectItem value={ALL}>All categories</SelectItem>
							{category.values.map((value) => (
								<SelectItem key={value} value={value}>
									{category.label(value)}
								</SelectItem>
							))}
						</SelectContent>
					</Select>
				</div>
			</div>
			{pages.isError ? (
				<p
					className="rounded-lg bg-red-500/10 px-4 py-3 text-sm text-red-300 ring-1 ring-red-500/20"
					role="alert"
				>
					{errorMessage(pages.error)}
				</p>
			) : null}
			<Group>
				{rows.length ? (
					<div
						aria-label="Row results"
						className="overflow-x-auto p-5"
						role="region"
						// eslint-disable-next-line jsx-a11y/no-noninteractive-tabindex -- axe scrollable-region-focusable requires focus on scrolling regions.
						tabIndex={0}
					>
						<table className="w-full min-w-[960px] text-left text-sm">
							<thead>
								<tr className="border-b border-border">
									<th className="py-2 pr-4 font-medium text-foreground">Email</th>
									<th className="py-2 pr-4 font-medium text-foreground">Score</th>
									<th className="py-2 pr-4 font-medium text-foreground">Category</th>
									<th className="py-2 pr-4 font-medium text-foreground">Safe to send</th>
									<th className="py-2 pr-4 font-medium text-foreground">Recommendation</th>
									<th className="py-2 font-medium text-foreground">Reasons</th>
								</tr>
							</thead>
							<tbody>
								{rows.map((row) => (
									<tr className="border-b border-border/60" key={row.id}>
										<td className="py-2 pr-4">
											{row.result ? (
												<Button
													className="h-auto justify-start p-0 text-left text-white"
													onClick={() => {
														setOpen(row);
													}}
													type="button"
													variant="link"
												>
													{row.original_email ||
														row.result.input ||
														`Row ${String(row.row_number ?? "unknown")} (blank)`}
												</Button>
											) : (
												<span className="text-white">
													Row {row.row_number ?? "unknown"}: {row.original_email || "Blank address"}
												</span>
											)}
											{row.row_number != null ? (
												<p className="mt-1 text-xs text-[#858585]">
													Row {row.row_number}
													{row.is_duplicate &&
													row.canonical_task_id != null &&
													rowNumbers.get(row.canonical_task_id) != null
														? ` · Duplicate of row ${String(rowNumbers.get(row.canonical_task_id))}`
														: ""}
												</p>
											) : null}
										</td>
										<td className="py-2 pr-4 text-white">{row.result?.score.score ?? "—"}</td>
										<td className="py-2 pr-4">
											{row.result ? (
												<Pill tone={category.tone(row.result.score.category)}>
													{category.label(row.result.score.category)}
												</Pill>
											) : (
												<Pill tone={taskState.tone(row.task_state)}>
													{taskState.label(row.task_state)}
												</Pill>
											)}
										</td>
										<td className="py-2 pr-4 text-[#858585]">
											{row.result
												? !row.is_duplicate && row.result.score.safe_to_send
													? "Yes"
													: "No"
												: "—"}
										</td>
										<td className="py-2 pr-4">
											{row.result ? (
												<Pill
													tone={
														row.is_duplicate
															? "gray"
															: recommendation.tone(primaryAction(row.result))
													}
												>
													{row.is_duplicate
														? "Duplicate — excluded"
														: recommendation.label(primaryAction(row.result))}
												</Pill>
											) : (
												"—"
											)}
										</td>
										<td className="min-w-64 py-2 text-[#858585]">
											{row.result
												? row.result.score.reason_codes
														.map((code) => labelFor(REASON_LABELS, code))
														.join(", ") || "—"
												: (row.error ?? "—")}
										</td>
									</tr>
								))}
							</tbody>
						</table>
					</div>
				) : (
					<p className="p-5 text-sm text-[#858585]">
						{pages.isPending ? "Loading rows…" : "No rows match these filters."}
					</p>
				)}
			</Group>
			{pages.hasNextPage ? (
				<div className="flex justify-end">
					<Button
						className="rounded-lg"
						disabled={pages.isFetchingNextPage}
						onClick={() => {
							void pages.fetchNextPage();
						}}
						size="sm"
						type="button"
						variant="outline"
					>
						Load more rows
					</Button>
				</div>
			) : null}
			<Sheet
				onOpenChange={(next) => {
					if (!next) setOpen(undefined);
				}}
				open={!!open?.result}
			>
				<SheetContent className="flex w-full flex-col gap-6 overflow-y-auto pb-8 sm:max-w-2xl">
					{open?.result ? (
						<>
							<SheetHeader>
								<SheetTitle className="break-all">
									{open.original_email ||
										open.result.input ||
										`Row ${String(open.row_number ?? open.id)}`}
								</SheetTitle>
								<SheetDescription>
									{taskState.label(open.task_state)} · Checked {open.retry_count + 1}{" "}
									{open.retry_count ? "times" : "time"}
								</SheetDescription>
							</SheetHeader>
							<div className="flex flex-col gap-6 px-4">
								<VerdictSummary result={open.result} />
								<RiskFactors result={open.result} />
								<SignalChecklist result={open.result} />
								<ProviderDetails result={open.result} />
								<TechnicalDetails result={open.result} />
								{open.result.input ? <AddressTimeline email={open.result.input} /> : null}
							</div>
						</>
					) : null}
				</SheetContent>
			</Sheet>
		</section>
	);
}
