"use client";

import "client-only";

import { useState, type ComponentPropsWithoutRef } from "react";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { Configuration, ListsApi, V1Api } from "@oppulence/reacher-sdk";

import { Button } from "@oppulence/ui/components/button";
import { Checkbox } from "@oppulence/ui/components/checkbox";
import { Label } from "@oppulence/ui/components/label";
import {
	Select,
	SelectContent,
	SelectItem,
	SelectTrigger,
	SelectValue,
} from "@oppulence/ui/components/select";
import { ALL, fromSelect } from "@/lib/select";
import { cn } from "@oppulence/ui/lib/utils";

import { Group, Pill, Row, Section } from "@/components/platform/page-kit";
import { useV1ListLists } from "@/hooks/queries/use-v1-list-lists";
import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";
import { saveFile } from "@/lib/downloads";
import { RemediationExportSchema, RemediationPlanSchema } from "@/lib/api/schemas/lists";
import { category, humanize, remediationClass } from "@/lib/verification/labels";

import { type ListExportPropsFields } from "./list-export.schema";

/**
 * @oppulence-gen kind=component
 * ListExport is a client data component.
 * Downloads a cleaned list, builds the remediation plan, and exports its rows.
 *
 * It reads and writes through the generated SDK and TanStack Query. The API
 * has no endpoint that lists earlier exports, so each export download makes a
 * new export from the latest plan and downloads it at once. No export ID stays
 * in component state. Owned by `list-export.lit.ts`.
 */
export type ListExportProps = ListExportPropsFields & ComponentPropsWithoutRef<"section">;

const config = new Configuration({ basePath: "/api/backend" });
const listsApi = new ListsApi(config);
const v1Api = new V1Api(config);
const SELECT_CLASS = "h-10 w-full rounded-lg md:w-72";

/** Remediation export files the backend can build (backend/src/http/v1/lists/remediation.rs). */
const PARTITIONS = {
	safe_to_send: "Send-ready (safe and fixed)",
	safe: "Safe",
	fixed: "Fixed",
	review: "Review",
	drop: "Drop",
	changed: "Changed addresses",
	all: "All rows",
} as const;

/** Plan options and their defaults (`PlanOptions` in remediation.rs). */
const DEFAULT_PLAN_OPTIONS = {
	normalize_emails: true,
	apply_domain_typos: true,
	deduplicate: true,
	drop_suppressed: true,
	allow_partial: false,
};

type PlanOption = keyof typeof DEFAULT_PLAN_OPTIONS;

const PLAN_OPTION_LABELS: Record<PlanOption, string> = {
	normalize_emails: "Fix the address format",
	apply_domain_typos: "Fix domain typos",
	deduplicate: "Drop duplicate addresses",
	drop_suppressed: "Drop suppressed addresses",
	allow_partial: "Plan before all rows are checked",
};

export function ListExport({ className, listId, ...props }: ListExportProps) {
	const queryClient = useQueryClient();
	const [filter, setFilter] = useState("");
	const [baseListId, setBaseListId] = useState("");
	const [partition, setPartition] = useState<keyof typeof PARTITIONS>("safe_to_send");
	const [options, setOptions] = useState(DEFAULT_PLAN_OPTIONS);
	const lists = useV1ListLists({ limit: 100 });
	const earlierLists =
		lists.data?.lists.filter((list) => list.id !== listId && list.status === "completed") ?? [];
	const planKey = workflowQueryKey("list-detail", listId, "plan");
	const plan = useQuery({
		queryKey: planKey,
		retry: false,
		queryFn: async () =>
			RemediationPlanSchema.parse((await listsApi.v1GetRemediationPlan({ listId })).data),
	});
	const createPlan = useMutation({
		mutationFn: async () =>
			RemediationPlanSchema.parse(
				(await listsApi.v1CreateRemediationPlan({ listId, requestBody: options })).data,
			),
		onSuccess: (data) => {
			queryClient.setQueryData(planKey, data);
		},
	});
	const cleaned = useMutation({
		mutationFn: async () => {
			const response = await v1Api.v1DownloadList(
				{
					listId,
					format: "csv",
					filter: filter || undefined,
					changedSinceListId: filter === "newly_invalid" ? Number(baseListId) : undefined,
				},
				{ responseType: "blob" },
			);
			saveFile(response.data, `list-${String(listId)}-${filter || "all"}.csv`);
		},
	});
	const exportRows = useMutation({
		mutationFn: async () => {
			const created = RemediationExportSchema.parse(
				(
					await listsApi.v1CreateRemediationExport({
						listId,
						requestBody: { plan_id: plan.data?.id, partitions: [partition], format: "csv" },
					})
				).data,
			);
			const response = await listsApi.v1DownloadRemediationExport(
				{ listId, exportId: created.id },
				{ responseType: "blob" },
			);
			saveFile(response.data, `list-${String(listId)}-${partition}.csv`);
		},
	});
	const error = [createPlan, cleaned, exportRows].find((request) => request.isError)?.error;

	return (
		<section data-slot="list-export" className={cn("flex flex-col gap-10", className)} {...props}>
			{error ? (
				<p
					className="rounded-lg bg-red-500/10 px-4 py-3 text-sm text-red-300 ring-1 ring-red-500/20"
					role="alert"
				>
					{error.message}
				</p>
			) : null}
			<Section
				description="Download the list with its check results. Choose the rows to include."
				title="Cleaned download"
			>
				<Group>
					<Row label={<Label htmlFor={`clean-filter-${String(listId)}`}>Rows to include</Label>}>
						<Select
							onValueChange={(value) => {
								setFilter(fromSelect(value));
							}}
							value={filter || ALL}
						>
							<SelectTrigger className={SELECT_CLASS} id={`clean-filter-${String(listId)}`}>
								<SelectValue />
							</SelectTrigger>
							<SelectContent>
								<SelectItem value={ALL}>All rows</SelectItem>
								{category.values.map((value) => (
									<SelectItem key={value} value={value}>
										{category.label(value)} only
									</SelectItem>
								))}
								<SelectItem value="newly_invalid">Newly invalid since an earlier list</SelectItem>
							</SelectContent>
						</Select>
					</Row>
					{filter === "newly_invalid" ? (
						<Row
							description="Only completed lists can be the baseline."
							label={<Label htmlFor={`clean-base-${String(listId)}`}>Earlier list</Label>}
						>
							<Select
								onValueChange={(value) => {
									setBaseListId(value);
								}}
								value={baseListId}
							>
								<SelectTrigger className={SELECT_CLASS} id={`clean-base-${String(listId)}`}>
									<SelectValue placeholder="Choose a list" />
								</SelectTrigger>
								<SelectContent>
									{earlierLists.map((list) => (
										<SelectItem key={list.id} value={String(list.id)}>
											{list.name}
										</SelectItem>
									))}
								</SelectContent>
							</Select>
						</Row>
					) : null}
					<Row>
						<div className="flex justify-end">
							<Button
								className="rounded-lg"
								disabled={cleaned.isPending || (filter === "newly_invalid" && !baseListId)}
								onClick={() => {
									cleaned.mutate();
								}}
								type="button"
							>
								Download cleaned CSV
							</Button>
						</div>
					</Row>
				</Group>
			</Section>
			<Section
				description="The plan fixes address formats and typos, and drops duplicates and suppressed addresses. Review it before you export."
				title="Remediation plan"
			>
				<Group>
					<Row>
						<div className="grid gap-3 sm:grid-cols-2">
							{Object.entries(PLAN_OPTION_LABELS).map(([key, label]) => (
								<Label
									className="flex items-center gap-2 text-sm text-neutral-300"
									htmlFor={`plan-${key}-${String(listId)}`}
									key={key}
								>
									<Checkbox
										checked={options[key as PlanOption]}
										id={`plan-${key}-${String(listId)}`}
										onCheckedChange={(checked) => {
											setOptions((current) => ({ ...current, [key]: checked === true }));
										}}
									/>
									{label}
								</Label>
							))}
						</div>
						<div className="flex justify-end">
							<Button
								className="rounded-lg"
								disabled={createPlan.isPending}
								onClick={() => {
									createPlan.mutate();
								}}
								type="button"
								variant={plan.data ? "outline" : "default"}
							>
								Create remediation plan
							</Button>
						</div>
					</Row>
					{plan.data ? (
						<>
							<Row
								description={`Created ${new Date(plan.data.created_at).toLocaleString()}. The table shows up to 100 rows: fixed rows first, then review and drop.`}
								label={`Plan ${String(plan.data.id)}`}
							>
								<div className="flex flex-wrap gap-2 md:justify-end">
									{Object.entries(plan.data.summary_counts).map(([key, count]) => (
										<Pill key={key} tone={remediationClass.tone(key)}>
											{remediationClass.label(key)}: {count}
										</Pill>
									))}
								</div>
							</Row>
							{plan.data.preview_rows.length ? (
								<div
									aria-label="Plan rows"
									className="max-h-96 overflow-auto p-5"
									role="region"
									// eslint-disable-next-line jsx-a11y/no-noninteractive-tabindex -- axe scrollable-region-focusable requires focus on scrolling regions.
									tabIndex={0}
								>
									<table className="w-full min-w-[960px] text-left text-sm">
										<thead>
											<tr className="border-b border-border">
												<th className="py-2 pr-4 font-medium text-foreground">Row</th>
												<th className="py-2 pr-4 font-medium text-foreground">Original</th>
												<th className="py-2 pr-4 font-medium text-foreground">After</th>
												<th className="py-2 pr-4 font-medium text-foreground">Decision</th>
												<th className="py-2 pr-4 font-medium text-foreground">Rule</th>
												<th className="py-2 pr-4 font-medium text-foreground">Confidence</th>
												<th className="py-2 font-medium text-foreground">Reasons</th>
											</tr>
										</thead>
										<tbody>
											{plan.data.preview_rows.map((row) => (
												<tr className="border-b border-border/60 align-top" key={row.id}>
													<td className="py-2 pr-4 text-[#858585]">{row.row_index + 1}</td>
													<td className="py-2 pr-4 whitespace-nowrap text-white">
														{row.original_email || "—"}
													</td>
													<td className="py-2 pr-4 whitespace-nowrap text-[#858585]">
														{row.effective_email || "—"}
													</td>
													<td className="py-2 pr-4">
														<Pill tone={remediationClass.tone(row.classification)}>
															{remediationClass.label(row.classification)}
														</Pill>
													</td>
													<td className="py-2 pr-4 text-[#858585]">{humanize(row.rule_id)}</td>
													<td className="py-2 pr-4 text-[#858585]">{humanize(row.confidence)}</td>
													<td className="min-w-64 py-2 text-[#858585]">
														{row.reasons
															.map((reason) => reason.message ?? humanize(reason.code))
															.join(" ") || "—"}
													</td>
												</tr>
											))}
										</tbody>
									</table>
								</div>
							) : null}
						</>
					) : null}
				</Group>
			</Section>
			{plan.data ? (
				<Section description="Each download builds the file from the latest plan." title="Exports">
					<Group>
						<Row
							label={<Label htmlFor={`export-partition-${String(listId)}`}>Rows to export</Label>}
						>
							<Select
								onValueChange={(value) => {
									setPartition(value as keyof typeof PARTITIONS);
								}}
								value={partition}
							>
								<SelectTrigger className={SELECT_CLASS} id={`export-partition-${String(listId)}`}>
									<SelectValue />
								</SelectTrigger>
								<SelectContent>
									{Object.entries(PARTITIONS).map(([value, label]) => (
										<SelectItem key={value} value={value}>
											{label}
										</SelectItem>
									))}
								</SelectContent>
							</Select>
						</Row>
						<Row>
							<div className="flex justify-end">
								<Button
									className="rounded-lg"
									disabled={exportRows.isPending}
									onClick={() => {
										exportRows.mutate();
									}}
									type="button"
								>
									Download export
								</Button>
							</div>
						</Row>
					</Group>
				</Section>
			) : null}
		</section>
	);
}
