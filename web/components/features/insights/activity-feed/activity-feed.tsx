"use client";

import "client-only";

import { useState, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";
import { keepPreviousData, useQuery } from "@tanstack/react-query";
import { Configuration, EventsApi } from "@oppulence/reacher-sdk";
import Link from "next/link";

import { Button } from "@oppulence/ui/components/button";
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

import {
	apiMessage,
	FILTER_CLASS,
	startOfDay,
} from "@/components/features/deliverability/outcome-events/outcome-events";
import { Group, Pill } from "@/components/platform/page-kit";
import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";
import { V1ListEvents200Response } from "@/lib/api/generated/zod/events/events";
import { humanize, jobEventType } from "@/lib/verification/labels";

import { type ActivityFeedPropsFields } from "./activity-feed.schema";

/**
 * @oppulence-gen kind=component
 * ActivityFeed is a client data component.
 * Shows job events with filters and pages.
 *
 * It reads GET /v1/events through the generated SDK and TanStack Query. The
 * filters and the page go to the API as query parameters.
 * Owned by `activity-feed.lit.ts`.
 */
export type ActivityFeedProps = ActivityFeedPropsFields & ComponentPropsWithoutRef<"section">;

const api = new EventsApi(new Configuration({ basePath: "/api/backend" }));
const PAGE = 20;
const FIELD_LABEL = "text-xs font-medium text-neutral-400";

type Filters = {
	eventType?: string;
	actor?: string;
	jobId?: number;
	since?: string;
	until?: string;
};

/** The event data as readable pairs, for example "Retry count: 1". */
function dataEntries(data: unknown): [string, string][] {
	if (!data || typeof data !== "object") return [];
	return Object.entries(data).map(([key, value]) => [
		humanize(key),
		typeof value === "object" ? JSON.stringify(value) : String(value),
	]);
}

export function ActivityFeed({ className, ...props }: ActivityFeedProps) {
	const [offset, setOffset] = useState(0);
	const [filters, setFilters] = useState<Filters>({});
	const [error, setError] = useState("");
	const events = useQuery({
		queryKey: workflowQueryKey("activity-events", offset, JSON.stringify(filters)),
		placeholderData: keepPreviousData,
		queryFn: async () =>
			V1ListEvents200Response.parse(
				(
					await api.v1ListEvents({
						limit: PAGE,
						offset,
						...filters,
						since: startOfDay(filters.since ?? ""),
						until: filters.until
							? new Date(`${filters.until}T23:59:59.999`).toISOString()
							: undefined,
					})
				).data,
			),
	});
	const rows = events.data?.events ?? [];
	const total = events.data?.total ?? 0;

	function apply(event: SyntheticEvent<HTMLFormElement>) {
		event.preventDefault();
		const form = new FormData(event.currentTarget);
		// fromSelect turns the "Any" select item back into an empty value.
		const text = (key: string) => fromSelect(String(form.get(key) ?? "").trim()) || undefined;
		const job = text("job_id");
		const jobId = job === undefined ? undefined : Number(job);
		if (jobId !== undefined && !(Number.isSafeInteger(jobId) && jobId > 0)) {
			setError("Use a job ID that is a positive whole number.");
			return;
		}
		setError("");
		setOffset(0);
		setFilters({
			eventType: text("event_type"),
			actor: text("actor"),
			jobId,
			since: text("since"),
			until: text("until"),
		});
	}

	return (
		<section data-slot="activity-feed" className={cn("flex flex-col gap-4", className)} {...props}>
			<form
				className="grid gap-3 sm:grid-cols-2 lg:grid-cols-[12rem_1fr_9rem_10rem_10rem_auto] lg:items-end"
				noValidate
				onSubmit={apply}
			>
				<div className="flex flex-col gap-1.5">
					<Label className={FIELD_LABEL} htmlFor="activity-event-type">
						Event type
					</Label>
					<Select defaultValue={ALL} name="event_type">
						<SelectTrigger className={FILTER_CLASS} id="activity-event-type">
							<SelectValue />
						</SelectTrigger>
						<SelectContent>
							<SelectItem value={ALL}>All events</SelectItem>
							{jobEventType.values.map((value) => (
								<SelectItem key={value} value={value}>
									{jobEventType.label(value)}
								</SelectItem>
							))}
						</SelectContent>
					</Select>
				</div>
				<div className="flex flex-col gap-1.5">
					<Label className={FIELD_LABEL} htmlFor="activity-actor">
						Actor
					</Label>
					<Input
						className="h-10 rounded-lg"
						id="activity-actor"
						name="actor"
						placeholder="For example api"
					/>
				</div>
				<div className="flex flex-col gap-1.5">
					<Label className={FIELD_LABEL} htmlFor="activity-job">
						Job ID
					</Label>
					<Input
						className="h-10 rounded-lg"
						id="activity-job"
						inputMode="numeric"
						name="job_id"
						placeholder="Filter by job ID"
					/>
				</div>
				<div className="flex flex-col gap-1.5">
					<Label className={FIELD_LABEL} htmlFor="activity-since">
						From
					</Label>
					<Input className="h-10 rounded-lg" id="activity-since" name="since" type="date" />
				</div>
				<div className="flex flex-col gap-1.5">
					<Label className={FIELD_LABEL} htmlFor="activity-until">
						To
					</Label>
					<Input className="h-10 rounded-lg" id="activity-until" name="until" type="date" />
				</div>
				<Button className="h-10 rounded-lg" type="submit" variant="outline">
					Filter
				</Button>
			</form>
			{error || events.isError ? (
				<p
					className="rounded-lg bg-red-500/10 px-4 py-3 text-sm text-red-300 ring-1 ring-red-500/20"
					role="alert"
				>
					{error || (events.error ? apiMessage(events.error) : "")}
				</p>
			) : null}
			<Group>
				{rows.length ? (
					rows.map((event) => {
						const data = dataEntries(event.event_data);
						return (
							<article className="flex flex-col gap-1.5 px-5 py-4" key={event.id}>
								<div className="flex flex-wrap items-center gap-x-3 gap-y-1 text-sm">
									<Pill tone={jobEventType.tone(event.event_type)}>
										{jobEventType.label(event.event_type)}
									</Pill>
									<Link
										className="text-white underline underline-offset-4 hover:text-neutral-200"
										href={`/app/jobs?job=${String(event.job_id)}`}
									>
										Job {event.job_id}
									</Link>
									{event.task_id ? (
										<span className="text-[#858585]">Row {event.task_id}</span>
									) : null}
								</div>
								<p className="text-xs text-[#858585]">
									{new Date(event.created_at).toLocaleString()} · {event.actor ?? "System"}
								</p>
								{data.length ? (
									<dl className="flex flex-wrap gap-x-4 gap-y-1 text-xs break-all">
										{data.map(([key, value]) => (
											<div className="flex gap-1" key={key}>
												<dt className="text-[#858585]">{key}:</dt>
												<dd className="text-neutral-300">{value}</dd>
											</div>
										))}
									</dl>
								) : null}
							</article>
						);
					})
				) : (
					<p className="p-5 text-sm text-[#858585]">
						{events.isPending ? "Loading activity…" : "No activity found."}
					</p>
				)}
			</Group>
			<div className="flex flex-wrap items-center justify-end gap-2">
				{rows.length && !events.isPlaceholderData ? (
					<span className="mr-auto text-xs text-[#858585]">
						Events {offset + 1}–{offset + rows.length} of {total}
					</span>
				) : null}
				<Button
					className="rounded-lg"
					disabled={offset === 0 || events.isPlaceholderData}
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
					disabled={offset + PAGE >= total || events.isPlaceholderData}
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
