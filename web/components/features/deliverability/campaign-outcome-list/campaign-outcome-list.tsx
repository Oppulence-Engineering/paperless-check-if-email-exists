"use client";

import "client-only";

import { useState, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";
import { useQuery } from "@tanstack/react-query";
import { Configuration, OutcomesApi } from "@oppulence/reacher-sdk";

import { Button } from "@oppulence/ui/components/button";
import { DatePicker } from "@oppulence/ui/components/date-picker";
import { Input } from "@oppulence/ui/components/input";
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

import { Group, Pill } from "@/components/platform/page-kit";
import {
	apiMessage,
	FILTER_CLASS,
	startOfDay,
} from "@/components/features/deliverability/outcome-events/outcome-events";
import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";
import {
	V1ListCampaignOutcomes200Response,
	V1PostCampaignOutcomesBody,
} from "@/lib/api/generated/zod/outcomes/outcomes";
import { outcomeType, policyAction } from "@/lib/verification/labels";

import { type CampaignOutcomeListPropsFields } from "./campaign-outcome-list.schema";

/**
 * @oppulence-gen kind=component
 * CampaignOutcomeList is a client data component.
 * Shows the campaign results with filters and pages of 20 rows.
 *
 * It reads GET /v1/campaign-outcomes through the generated SDK and TanStack
 * Query. The filters and the page go to the API. Owned by `campaign-outcome-list.lit.ts`.
 */
export type CampaignOutcomeListProps = CampaignOutcomeListPropsFields &
	ComponentPropsWithoutRef<"section">;

const api = new OutcomesApi(new Configuration({ basePath: "/api/backend" }));
const PAGE = 20;
const FIELD_LABEL = "text-xs font-medium text-neutral-400";
/** The `OutcomeType` enum in the contract. */
export const CAMPAIGN_TYPES = V1PostCampaignOutcomesBody.shape.outcomes.element.shape.type.options;

type Filters = { email: string; type: string; source: string; since: string };

export function CampaignOutcomeList({ className, ...props }: CampaignOutcomeListProps) {
	const [offset, setOffset] = useState(0);
	const [filters, setFilters] = useState<Filters>({ email: "", type: "", source: "", since: "" });
	const outcomes = useQuery({
		queryKey: workflowQueryKey(
			"campaign-outcomes",
			offset,
			filters.email,
			filters.type,
			filters.source,
			filters.since,
		),
		queryFn: async ({ signal }) =>
			V1ListCampaignOutcomes200Response.parse(
				(
					await api.v1ListCampaignOutcomes(
						{
							limit: PAGE,
							offset,
							email: filters.email || undefined,
							type: filters.type || undefined,
							source: filters.source || undefined,
							since: startOfDay(filters.since),
						},
						{ signal },
					)
				).data,
			),
	});
	const rows = outcomes.data?.outcomes ?? [];
	const total = outcomes.data?.total ?? 0;

	function apply(event: SyntheticEvent<HTMLFormElement>) {
		event.preventDefault();
		const form = new FormData(event.currentTarget);
		const value = (name: string) => String(form.get(name) ?? "").trim();
		setOffset(0);
		setFilters({
			email: value("email"),
			type: fromSelect(value("type")),
			source: value("source"),
			since: value("since"),
		});
	}

	return (
		<section
			data-slot="campaign-outcome-list"
			className={cn("flex flex-col gap-4", className)}
			{...props}
		>
			<form
				className="grid gap-3 sm:grid-cols-2 lg:grid-cols-[1fr_11rem_11rem_11rem_auto] lg:items-end"
				onSubmit={apply}
			>
				<div className="flex flex-col gap-1.5">
					<Label className={FIELD_LABEL} htmlFor="campaign-filter-email">
						Result email
					</Label>
					<Input className="h-10 rounded-lg" id="campaign-filter-email" name="email" type="email" />
				</div>
				<div className="flex flex-col gap-1.5">
					<Label className={FIELD_LABEL} htmlFor="campaign-filter-type">
						Result type
					</Label>
					<Select defaultValue={ALL} name="type">
						<SelectTrigger className={FILTER_CLASS} id="campaign-filter-type">
							<SelectValue />
						</SelectTrigger>
						<SelectContent>
							<SelectItem value={ALL}>All results</SelectItem>
							{CAMPAIGN_TYPES.map((value) => (
								<SelectItem key={value} value={value}>
									{outcomeType.label(value)}
								</SelectItem>
							))}
						</SelectContent>
					</Select>
				</div>
				<div className="flex flex-col gap-1.5">
					<Label className={FIELD_LABEL} htmlFor="campaign-filter-source">
						Result source
					</Label>
					<Input className="h-10 rounded-lg" id="campaign-filter-source" name="source" />
				</div>
				<div className="flex flex-col gap-1.5">
					<Label className={FIELD_LABEL} htmlFor="campaign-filter-since">
						Results since
					</Label>
					<DatePicker className="h-10 rounded-lg" id="campaign-filter-since" name="since" />
				</div>
				<Button className="h-10 rounded-lg" type="submit" variant="outline">
					Search results
				</Button>
			</form>
			{outcomes.isError ? (
				<p
					className="rounded-lg bg-red-500/10 px-4 py-3 text-sm text-red-300 ring-1 ring-red-500/20"
					role="alert"
				>
					{apiMessage(outcomes.error)}
				</p>
			) : null}
			<Group>
				{rows.length ? (
					<div
						aria-label="Campaign results"
						className="overflow-x-auto p-5"
						role="region"
						// eslint-disable-next-line jsx-a11y/no-noninteractive-tabindex -- axe scrollable-region-focusable requires focus on scrolling regions.
						tabIndex={0}
					>
						<table className="w-full min-w-[760px] text-left text-sm">
							<thead>
								<tr className="border-b border-border">
									<th className="py-2 pr-4 font-medium text-foreground">Address</th>
									<th className="py-2 pr-4 font-medium text-foreground">Result</th>
									<th className="py-2 pr-4 font-medium text-foreground">Policy action</th>
									<th className="py-2 pr-4 font-medium text-foreground">Source</th>
									<th className="py-2 pr-4 font-medium text-foreground">Campaign</th>
									<th className="py-2 font-medium text-foreground">When</th>
								</tr>
							</thead>
							<tbody>
								{rows.map((item) => (
									<tr className="border-b border-border/60" key={item.id}>
										<td className="py-2 pr-4 break-all text-white">{item.email}</td>
										<td className="py-2 pr-4">
											<Pill tone={outcomeType.tone(item.type)}>{outcomeType.label(item.type)}</Pill>
										</td>
										<td className="py-2 pr-4">
											{item.policy_action ? (
												<Pill tone={policyAction.tone(item.policy_action)}>
													{policyAction.label(item.policy_action)}
												</Pill>
											) : (
												<span className="text-[#858585]">—</span>
											)}
										</td>
										<td className="py-2 pr-4 text-[#858585]">{item.source ?? "—"}</td>
										<td className="py-2 pr-4 text-[#858585]">{item.campaign_id ?? "—"}</td>
										<td className="py-2 whitespace-nowrap text-[#858585]">
											{new Date(item.occurred_at).toLocaleString()}
										</td>
									</tr>
								))}
							</tbody>
						</table>
					</div>
				) : (
					<p className="p-5 text-sm text-[#858585]">
						{outcomes.isPending ? "Loading results…" : "No campaign results found."}
					</p>
				)}
			</Group>
			<div className="flex flex-wrap items-center justify-end gap-2">
				{rows.length ? (
					<span className="mr-auto text-xs text-[#858585]">
						Rows {offset + 1}–{offset + rows.length} of {total}
					</span>
				) : null}
				<Button
					aria-label="Previous results"
					className="rounded-lg"
					disabled={offset === 0}
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
					aria-label="Next results"
					className="rounded-lg"
					disabled={offset + PAGE >= total}
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
		</section>
	);
}
