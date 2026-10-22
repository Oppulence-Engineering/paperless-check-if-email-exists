"use client";

import "client-only";

import { useState, type ComponentPropsWithoutRef } from "react";
import { keepPreviousData, useQuery } from "@tanstack/react-query";
import type { z } from "zod";
import { Configuration, QueryApi } from "@oppulence/reacher-sdk";
import Link from "next/link";

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

import {
	apiMessage,
	FILTER_CLASS,
	startOfDay,
} from "@/components/features/deliverability/outcome-events/outcome-events";
import { ResultDetail } from "@/components/features/insights/result-detail/result-detail";
import { ResultFilters } from "@/components/features/insights/result-filters/result-filters";
import { Group, Pill } from "@/components/platform/page-kit";
import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";
import { parseRecommendation } from "@/lib/api/schemas/decision";
import { V1QueryResults200Response } from "@/lib/api/generated/zod/query/query";
import {
	category,
	labelFor,
	POLICY_MODE_LABELS,
	policyDecision,
	REASON_LABELS,
	recommendation,
	taskState,
} from "@/lib/verification/labels";

import { type ExplorerFilters, type ResultExplorerPropsFields } from "./result-explorer.schema";

type QueryResultRow = z.infer<typeof V1QueryResults200Response>["results"][number];

/**
 * @oppulence-gen kind=component
 * ResultExplorer is a client data component.
 * Finds verification results with every filter and opens one result in a sheet.
 *
 * It reads GET /v1/query through the generated SDK and TanStack Query. The
 * parent keeps the filters, so a saved segment can set them. The page goes
 * back to the first rows when the filters change. Owned by `result-explorer.lit.ts`.
 */
export type ResultExplorerProps = ResultExplorerPropsFields & {
	onFiltersChange: (filters: ExplorerFilters) => void;
} & ComponentPropsWithoutRef<"section">;

const queryApi = new QueryApi(new Configuration({ basePath: "/api/backend" }));
const PAGE = 20;
const FIELD_LABEL = "text-xs font-medium text-neutral-400";
const MORE_KEYS = [
	"minScore",
	"maxScore",
	"jobId",
	"since",
	"until",
	"recommendationAction",
	"policyMode",
	"policyDecision",
] as const;

/** The active filters in words, for example "Score 10 to 100". */
export function describeFilters(filters: ExplorerFilters): string[] {
	const parts = [
		filters.category && `Category: ${category.label(filters.category)}`,
		filters.safeToSend !== undefined && `Safe to send: ${filters.safeToSend ? "Yes" : "No"}`,
		(filters.minScore !== undefined || filters.maxScore !== undefined) &&
			`Score ${String(filters.minScore ?? 0)} to ${String(filters.maxScore ?? 100)}`,
		filters.jobId !== undefined && `Job ${String(filters.jobId)}`,
		filters.since && `Checked from ${filters.since}`,
		filters.until && `Checked to ${filters.until}`,
		filters.recommendationAction &&
			`Recommendation: ${recommendation.label(filters.recommendationAction)}`,
		filters.policyMode && `Policy mode: ${labelFor(POLICY_MODE_LABELS, filters.policyMode)}`,
		filters.policyDecision && `Policy decision: ${policyDecision.label(filters.policyDecision)}`,
	];
	return parts.filter((part): part is string => typeof part === "string");
}

/** The reasons for the recommendation, else the score reason codes. */
function reasonsOf(row: QueryResultRow): string[] {
	const decision = parseRecommendation(row.recommendation);
	if (decision?.reasons.length) return decision.reasons.map((reason) => reason.message);
	return (row.reason_codes ?? []).map((code) => labelFor(REASON_LABELS, code));
}

export function ResultExplorer({
	className,
	filters,
	onFiltersChange,
	...props
}: ResultExplorerProps) {
	const [offset, setOffset] = useState(0);
	const [shown, setShown] = useState(filters);
	const [moreOpen, setMoreOpen] = useState(false);
	const [open, setOpen] = useState<QueryResultRow>();
	if (shown !== filters) {
		setShown(filters);
		setOffset(0);
	}
	const results = useQuery({
		queryKey: workflowQueryKey("query-results", offset, filters),
		placeholderData: keepPreviousData,
		queryFn: async ({ signal }) =>
			V1QueryResults200Response.parse(
				(
					await queryApi.v1QueryResults(
						{
							limit: PAGE,
							offset,
							...filters,
							since: startOfDay(filters.since ?? ""),
							until: filters.until
								? new Date(`${filters.until}T23:59:59.999`).toISOString()
								: undefined,
						},
						{ signal },
					)
				).data,
			),
	});
	const rows = results.data?.results ?? [];
	const total = results.data?.total ?? 0;
	const moreCount = MORE_KEYS.filter((key) => filters[key] !== undefined).length;
	const active = describeFilters(filters);
	const set = (next: Partial<ExplorerFilters>) => {
		onFiltersChange({ ...filters, ...next });
	};

	return (
		<section
			data-slot="result-explorer"
			className={cn("flex flex-col gap-4", className)}
			{...props}
		>
			<div className="flex flex-wrap items-end gap-3">
				<div className="flex w-full flex-col gap-1.5 sm:w-44">
					<Label className={FIELD_LABEL} htmlFor="explorer-category">
						Category
					</Label>
					<Select
						onValueChange={(value) => {
							set({ category: fromSelect(value) || undefined });
						}}
						value={filters.category ?? ALL}
					>
						<SelectTrigger className={FILTER_CLASS} id="explorer-category">
							<SelectValue />
						</SelectTrigger>
						<SelectContent>
							<SelectItem value={ALL}>All</SelectItem>
							{category.values.map((value) => (
								<SelectItem key={value} value={value}>
									{category.label(value)}
								</SelectItem>
							))}
						</SelectContent>
					</Select>
				</div>
				<div className="flex w-full flex-col gap-1.5 sm:w-36">
					<Label className={FIELD_LABEL} htmlFor="explorer-safe">
						Safe to send
					</Label>
					<Select
						onValueChange={(value) => {
							set({ safeToSend: value === ALL ? undefined : value === "yes" });
						}}
						value={filters.safeToSend === undefined ? ALL : filters.safeToSend ? "yes" : "no"}
					>
						<SelectTrigger className={FILTER_CLASS} id="explorer-safe">
							<SelectValue />
						</SelectTrigger>
						<SelectContent>
							<SelectItem value={ALL}>Any</SelectItem>
							<SelectItem value="yes">Yes</SelectItem>
							<SelectItem value="no">No</SelectItem>
						</SelectContent>
					</Select>
				</div>
				<Button
					className="h-10 rounded-lg"
					onClick={() => {
						setMoreOpen(true);
					}}
					type="button"
					variant="outline"
				>
					{moreCount ? `More filters (${String(moreCount)})` : "More filters"}
				</Button>
				{active.length ? (
					<Button
						className="h-10 rounded-lg"
						onClick={() => {
							onFiltersChange({});
						}}
						type="button"
						variant="ghost"
					>
						Clear filters
					</Button>
				) : null}
			</div>
			{active.length ? (
				<ul aria-label="Active filters" className="flex flex-wrap gap-2">
					{active.map((text) => (
						<li key={text}>
							<Pill>{text}</Pill>
						</li>
					))}
				</ul>
			) : null}
			{results.isError ? (
				<p
					className="rounded-lg bg-red-500/10 px-4 py-3 text-sm text-red-300 ring-1 ring-red-500/20"
					role="alert"
				>
					{apiMessage(results.error)}
				</p>
			) : null}
			<Group>
				{rows.length ? (
					<div
						aria-label="Verification results"
						className="overflow-x-auto p-5"
						role="region"
						// eslint-disable-next-line jsx-a11y/no-noninteractive-tabindex -- axe scrollable-region-focusable requires focus on scrolling regions.
						tabIndex={0}
					>
						<table className="w-full min-w-[1180px] text-left text-sm">
							<thead>
								<tr className="border-b border-border">
									<th className="py-2 pr-4 font-medium text-foreground">Address</th>
									<th className="py-2 pr-4 font-medium text-foreground">Score</th>
									<th className="py-2 pr-4 font-medium text-foreground">Category</th>
									<th className="py-2 pr-4 font-medium text-foreground">Safe to send</th>
									<th className="py-2 pr-4 font-medium text-foreground">Recommendation</th>
									<th className="py-2 pr-4 font-medium text-foreground">Policy</th>
									<th className="py-2 pr-4 font-medium text-foreground">Reasons</th>
									<th className="py-2 pr-4 font-medium text-foreground">Job</th>
									<th className="py-2 font-medium text-foreground">Checked</th>
								</tr>
							</thead>
							<tbody>
								{rows.map((row) => {
									const reasons = reasonsOf(row);
									return (
										<tr className="border-b border-border/60 align-top" key={row.id}>
											<td className="py-2 pr-4">
												<Button
													className="h-auto justify-start p-0 text-left break-all whitespace-normal text-white"
													onClick={() => {
														setOpen(row);
													}}
													type="button"
													variant="link"
												>
													{row.email || `Result ${String(row.id)}`}
												</Button>
											</td>
											<td className="py-2 pr-4 text-white">{row.score ?? "—"}</td>
											<td className="py-2 pr-4">
												{row.category ? (
													<Pill tone={category.tone(row.category)}>
														{category.label(row.category)}
													</Pill>
												) : (
													<Pill tone={taskState.tone(row.task_state)}>
														{taskState.label(row.task_state)}
													</Pill>
												)}
											</td>
											<td className="py-2 pr-4 text-[#858585]">
												{typeof row.safe_to_send === "boolean"
													? row.safe_to_send
														? "Yes"
														: "No"
													: "—"}
											</td>
											<td className="py-2 pr-4">
												{row.recommendation_action ? (
													<Pill tone={recommendation.tone(row.recommendation_action)}>
														{recommendation.label(row.recommendation_action)}
													</Pill>
												) : (
													<span className="text-[#858585]">—</span>
												)}
											</td>
											<td className="py-2 pr-4">
												{row.policy_decision ? (
													<Pill tone={policyDecision.tone(row.policy_decision)}>
														{policyDecision.label(row.policy_decision)}
													</Pill>
												) : (
													<span className="text-[#858585]">—</span>
												)}
												{row.policy_mode ? (
													<span className="mt-1 block text-xs text-[#858585]">
														{labelFor(POLICY_MODE_LABELS, row.policy_mode)}
													</span>
												) : null}
											</td>
											<td className="min-w-72 py-2 pr-4 text-[#858585]">
												{reasons.length ? (
													<ul className="flex flex-col gap-1">
														{reasons.map((reason) => (
															<li key={reason}>{reason}</li>
														))}
													</ul>
												) : (
													"—"
												)}
											</td>
											<td className="py-2 pr-4 text-[#858585]">
												{row.job_id ? (
													<Link
														className="underline hover:text-neutral-200"
														href={`/app/jobs?job=${String(row.job_id)}`}
													>
														{row.job_id}
													</Link>
												) : (
													"Single check"
												)}
											</td>
											<td className="py-2 whitespace-nowrap text-[#858585]">
												{row.completed_at ? new Date(row.completed_at).toLocaleString() : "—"}
											</td>
										</tr>
									);
								})}
							</tbody>
						</table>
					</div>
				) : (
					<p className="p-5 text-sm text-[#858585]">
						{results.isPending ? "Loading results…" : "No results match these filters."}
					</p>
				)}
			</Group>
			<div className="flex flex-wrap items-center justify-end gap-2">
				{rows.length && !results.isPlaceholderData ? (
					<span className="mr-auto text-xs text-[#858585]">
						Rows {offset + 1}–{offset + rows.length} of {total}
					</span>
				) : null}
				<Button
					className="rounded-lg"
					disabled={offset === 0 || results.isPlaceholderData}
					onClick={() => {
						setOffset(Math.max(0, offset - PAGE));
					}}
					size="sm"
					type="button"
					variant="outline"
				>
					Previous
				</Button>
				<Button
					className="rounded-lg"
					disabled={offset + PAGE >= total || results.isPlaceholderData}
					onClick={() => {
						setOffset(offset + PAGE);
					}}
					size="sm"
					type="button"
					variant="outline"
				>
					Next
				</Button>
			</div>
			<Sheet onOpenChange={setMoreOpen} open={moreOpen}>
				<SheetContent className="flex w-full flex-col gap-6 overflow-y-auto pb-8 sm:max-w-xl">
					<SheetHeader>
						<SheetTitle>More filters</SheetTitle>
						<SheetDescription>
							Find results by score, job, date, recommendation, and policy.
						</SheetDescription>
					</SheetHeader>
					<ResultFilters
						className="px-4"
						filters={filters}
						onApply={(next) => {
							onFiltersChange(next);
							setMoreOpen(false);
						}}
					/>
				</SheetContent>
			</Sheet>
			<Sheet
				onOpenChange={(next) => {
					if (!next) setOpen(undefined);
				}}
				open={!!open}
			>
				<SheetContent className="flex w-full flex-col gap-6 overflow-y-auto pb-8 sm:max-w-2xl">
					{open ? (
						<>
							<SheetHeader>
								<SheetTitle className="break-all">
									{open.email || `Result ${String(open.id)}`}
								</SheetTitle>
								<SheetDescription>
									{open.job_id ? `Job ${String(open.job_id)}` : "Single check"} · Checked{" "}
									{open.completed_at ? new Date(open.completed_at).toLocaleString() : "—"}
								</SheetDescription>
							</SheetHeader>
							<ResultDetail className="px-4" row={open} />
						</>
					) : null}
				</SheetContent>
			</Sheet>
		</section>
	);
}
