"use client";

import "client-only";
import { errorMessage } from "@/lib/api/error-message";

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
import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";
import { V1ListOutcomes200Response } from "@/lib/api/generated/zod/outcomes/outcomes";
import { correlationStatus, eventFamily, outcomeType } from "@/lib/verification/labels";

import { type OutcomeEventsPropsFields } from "./outcome-events.schema";

/**
 * @oppulence-gen kind=component
 * OutcomeEvents is a client data component.
 * Shows the provider events with filters and pages of 20 rows.
 *
 * It reads GET /v1/outcomes through the generated SDK and TanStack Query.
 * The filters and the page go to the API as query parameters.
 * Owned by `outcome-events.lit.ts`.
 */
export type OutcomeEventsProps = OutcomeEventsPropsFields & ComponentPropsWithoutRef<"section">;

const api = new OutcomesApi(new Configuration({ basePath: "/api/backend" }));
const PAGE = 20;

/** The event types that POST /v1/outcomes accepts (`validate_event_type` in outcomes.rs). */
export const PROVIDER_EVENT_TYPES = [
	"delivered",
	"bounced",
	"complained",
	"opened",
	"clicked",
	"unsubscribed",
];

/**
 * The backend sets `event_family` only for provider webhook events. This copy
 * of `event_family()` in provider_outcomes.rs gives manual events a family too.
 */
const FAMILY_OF_EVENT: Record<string, string> = {
	delivered: "delivery",
	bounced: "delivery",
	opened: "engagement",
	clicked: "engagement",
	complained: "negative_feedback",
	unsubscribed: "negative_feedback",
};

export const FILTER_CLASS = "h-10 w-full rounded-lg";
const FIELD_LABEL = "text-xs font-medium text-neutral-400";

type Filters = { email: string; eventType: string; since: string };

/** A local date (YYYY-MM-DD) as the start of that day in ISO 8601, or undefined. */
export function startOfDay(date: string): string | undefined {
	return date ? new Date(`${date}T00:00`).toISOString() : undefined;
}

/** The error text from the backend, for example "allowed_ips contains an invalid IP". */
export function apiMessage(error: Error): string {
	return errorMessage(error);
}

export function OutcomeEvents({ className, ...props }: OutcomeEventsProps) {
	const [offset, setOffset] = useState(0);
	const [filters, setFilters] = useState<Filters>({ email: "", eventType: "", since: "" });
	const outcomes = useQuery({
		queryKey: workflowQueryKey("outcomes", offset, filters.email, filters.eventType, filters.since),
		queryFn: async ({ signal }) =>
			V1ListOutcomes200Response.parse(
				(
					await api.v1ListOutcomes(
						{
							limit: PAGE,
							offset,
							email: filters.email || undefined,
							eventType: filters.eventType || undefined,
							since: startOfDay(filters.since),
						},
						{ signal },
					)
				).data,
			),
	});
	const rows = outcomes.data?.outcomes ?? [];

	function apply(event: SyntheticEvent<HTMLFormElement>) {
		event.preventDefault();
		const form = new FormData(event.currentTarget);
		setOffset(0);
		setFilters({
			email: String(form.get("email") ?? "").trim(),
			eventType: fromSelect(String(form.get("event_type") ?? "")),
			since: String(form.get("since") ?? ""),
		});
	}

	return (
		<section data-slot="outcome-events" className={cn("flex flex-col gap-4", className)} {...props}>
			<form
				className="grid gap-3 sm:grid-cols-[1fr_12rem_11rem_auto] sm:items-end"
				onSubmit={apply}
			>
				<div className="flex flex-col gap-1.5">
					<Label className={FIELD_LABEL} htmlFor="outcome-filter">
						Filter by email
					</Label>
					<Input
						className="h-10 rounded-lg"
						id="outcome-filter"
						name="email"
						placeholder="Filter by email"
						type="email"
					/>
				</div>
				<div className="flex flex-col gap-1.5">
					<Label className={FIELD_LABEL} htmlFor="outcome-event-type">
						Event type
					</Label>
					<Select defaultValue={ALL} name="event_type">
						<SelectTrigger className={FILTER_CLASS} id="outcome-event-type">
							<SelectValue />
						</SelectTrigger>
						<SelectContent>
							<SelectItem value={ALL}>All events</SelectItem>
							{PROVIDER_EVENT_TYPES.map((value) => (
								<SelectItem key={value} value={value}>
									{outcomeType.label(value)}
								</SelectItem>
							))}
						</SelectContent>
					</Select>
				</div>
				<div className="flex flex-col gap-1.5">
					<Label className={FIELD_LABEL} htmlFor="outcome-since">
						Since
					</Label>
					<DatePicker className="h-10 rounded-lg" id="outcome-since" name="since" />
				</div>
				<Button className="h-10 rounded-lg" type="submit">
					Search
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
						aria-label="Provider events"
						className="overflow-x-auto p-5"
						role="region"
						// eslint-disable-next-line jsx-a11y/no-noninteractive-tabindex -- axe scrollable-region-focusable requires focus on scrolling regions.
						tabIndex={0}
					>
						<table className="w-full min-w-[1040px] text-left text-sm">
							<thead>
								<tr className="border-b border-border">
									<th className="py-2 pr-4 font-medium text-foreground">Address</th>
									<th className="py-2 pr-4 font-medium text-foreground">Event</th>
									<th className="py-2 pr-4 font-medium text-foreground">Family</th>
									<th className="py-2 pr-4 font-medium text-foreground">Provider</th>
									<th className="py-2 pr-4 font-medium text-foreground">Campaign</th>
									<th className="py-2 pr-4 font-medium text-foreground">Send match</th>
									<th className="py-2 pr-4 font-medium text-foreground">Provider IDs</th>
									<th className="py-2 font-medium text-foreground">When</th>
								</tr>
							</thead>
							<tbody>
								{rows.map((item) => {
									const family = item.event_family ?? FAMILY_OF_EVENT[item.event_type];
									const ids = [
										item.provider_message_id ? `Message ${item.provider_message_id}` : null,
										item.provider_event_id ? `Event ${item.provider_event_id}` : null,
									].filter(Boolean);
									return (
										<tr className="border-b border-border/60 align-top" key={item.id}>
											<td className="py-2 pr-4 break-all text-white">{item.email}</td>
											<td className="py-2 pr-4">
												<Pill tone={outcomeType.tone(item.event_type)}>
													{outcomeType.label(item.event_type)}
												</Pill>
											</td>
											<td className="py-2 pr-4">
												{family ? (
													<Pill tone={eventFamily.tone(family)}>{eventFamily.label(family)}</Pill>
												) : (
													<span className="text-[#858585]">—</span>
												)}
											</td>
											<td className="py-2 pr-4 text-neutral-300">
												{item.provider}
												{item.source_key ? (
													<span className="block text-xs text-[#858585]">
														Source {item.source_key}
													</span>
												) : null}
											</td>
											<td className="py-2 pr-4 text-[#858585]">{item.campaign_id ?? "—"}</td>
											<td className="py-2 pr-4">
												<Pill tone={correlationStatus.tone(item.correlation_status)}>
													{correlationStatus.label(item.correlation_status)}
												</Pill>
											</td>
											<td className="max-w-64 py-2 pr-4 text-xs break-all text-[#858585]">
												{ids.length ? ids.join(" · ") : "—"}
											</td>
											<td className="py-2 whitespace-nowrap text-[#858585]">
												{new Date(item.occurred_at).toLocaleString()}
											</td>
										</tr>
									);
								})}
							</tbody>
						</table>
					</div>
				) : (
					<p className="p-5 text-sm text-[#858585]">
						{outcomes.isPending ? "Loading events…" : "No outcomes found."}
					</p>
				)}
			</Group>
			<div className="flex flex-wrap items-center justify-end gap-2">
				{rows.length ? (
					<span className="mr-auto text-xs text-[#858585]">
						Rows {offset + 1}–{offset + rows.length}
					</span>
				) : null}
				<Button
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
					className="rounded-lg"
					disabled={rows.length < PAGE}
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
