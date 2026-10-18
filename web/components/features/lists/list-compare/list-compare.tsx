"use client";

import "client-only";

import { useState, type ComponentPropsWithoutRef } from "react";
import { useQuery } from "@tanstack/react-query";
import { Configuration, ListsApi } from "@oppulence/reacher-sdk";

import { Label } from "@oppulence/ui/components/label";
import { cn } from "@oppulence/ui/lib/utils";

import { Group, Pill, Row } from "@/components/platform/page-kit";
import { useV1ListLists } from "@/hooks/queries/use-v1-list-lists";
import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";
import { V1DiffLists200Response } from "@/lib/api/generated/zod/lists/lists";
import { category } from "@/lib/verification/labels";

import { type ListComparePropsFields } from "./list-compare.schema";

/**
 * @oppulence-gen kind=component
 * ListCompare is a client data component.
 * Compares one list with an earlier list and shows what changed.
 *
 * It reads the lists and the list diff through the generated SDK and TanStack
 * Query. The earlier list is the base and this list is the compare side.
 * Owned by `list-compare.lit.ts`.
 */
export type ListCompareProps = ListComparePropsFields & ComponentPropsWithoutRef<"section">;

const listsApi = new ListsApi(new Configuration({ basePath: "/api/backend" }));

/** The diff groups in the order the page shows them (backend/src/http/v1/lists/diff.rs). */
const GROUPS = [
	{ key: "newly_invalid", label: "Newly invalid", tone: "red" },
	{ key: "newly_risky", label: "Newly risky", tone: "yellow" },
	{ key: "degraded", label: "Worse", tone: "yellow" },
	{ key: "newly_safe", label: "Newly safe", tone: "green" },
	{ key: "improved", label: "Better", tone: "green" },
	{ key: "added", label: "Added", tone: "blue" },
	{ key: "removed", label: "Removed", tone: "gray" },
	{ key: "unchanged", label: "Unchanged", tone: "gray" },
] as const;

function side(score: number | null | undefined, value: string | null | undefined): string {
	return value ? `${category.label(value)} · ${String(score ?? "—")}` : "—";
}

export function ListCompare({ className, listId, ...props }: ListCompareProps) {
	const [baseListId, setBaseListId] = useState("");
	const lists = useV1ListLists({ limit: 100 });
	const earlierLists =
		lists.data?.lists.filter((list) => list.id !== listId && list.status === "completed") ?? [];
	const diff = useQuery({
		queryKey: workflowQueryKey("list-detail", listId, "diff", baseListId),
		enabled: !!baseListId,
		queryFn: async () =>
			V1DiffLists200Response.parse(
				(
					await listsApi.v1DiffLists({
						baseListId: Number(baseListId),
						compareListId: listId,
						limit: 50,
					})
				).data,
			),
	});

	return (
		<section data-slot="list-compare" className={cn("flex flex-col gap-4", className)} {...props}>
			<Group>
				<Row
					description="Both lists must be complete. Each group shows up to 50 addresses."
					label={<Label htmlFor={`compare-base-${String(listId)}`}>Compare with</Label>}
				>
					<select
						className="h-10 w-full rounded-lg bg-white/[0.03] px-2 text-sm text-neutral-200 ring-1 ring-white/[0.08] outline-none md:w-72"
						id={`compare-base-${String(listId)}`}
						onChange={(event) => {
							setBaseListId(event.target.value);
						}}
						value={baseListId}
					>
						<option value="">Choose an earlier list</option>
						{earlierLists.map((list) => (
							<option key={list.id} value={list.id}>
								{list.name}
							</option>
						))}
					</select>
				</Row>
			</Group>
			{diff.isError ? (
				<p
					className="rounded-lg bg-red-500/10 px-4 py-3 text-sm text-red-300 ring-1 ring-red-500/20"
					role="alert"
				>
					{diff.error.message}
				</p>
			) : null}
			{diff.data ? (
				<Group>
					{GROUPS.map(({ key, label, tone }) => {
						const group = diff.data[key];
						return (
							<details className="group/diff" key={key}>
								<summary className="flex cursor-pointer items-center justify-between gap-4 px-5 py-4 text-sm text-white">
									{label}
									<Pill tone={group.count ? tone : "gray"}>{group.count}</Pill>
								</summary>
								{group.rows.length ? (
									<div
										aria-label={`${label} addresses`}
										className="overflow-x-auto px-5 pb-5"
										role="region"
										// eslint-disable-next-line jsx-a11y/no-noninteractive-tabindex -- axe scrollable-region-focusable requires focus on scrolling regions.
										tabIndex={0}
									>
										<table className="w-full min-w-[560px] text-left text-sm">
											<thead>
												<tr className="border-b border-border">
													<th className="py-2 pr-4 font-medium text-foreground">Email</th>
													<th className="py-2 pr-4 font-medium text-foreground">Before</th>
													<th className="py-2 font-medium text-foreground">Now</th>
												</tr>
											</thead>
											<tbody>
												{group.rows.map((row) => (
													<tr className="border-b border-border/60" key={row.canonical_email}>
														<td className="py-2 pr-4 break-all text-white">
															{row.canonical_email}
														</td>
														<td className="py-2 pr-4 text-[#858585]">
															{side(row.base_score, row.base_category)}
														</td>
														<td className="py-2 text-[#858585]">
															{side(row.compare_score, row.compare_category)}
														</td>
													</tr>
												))}
											</tbody>
										</table>
									</div>
								) : (
									<p className="px-5 pb-5 text-sm text-[#858585]">No addresses in this group.</p>
								)}
							</details>
						);
					})}
				</Group>
			) : null}
		</section>
	);
}
