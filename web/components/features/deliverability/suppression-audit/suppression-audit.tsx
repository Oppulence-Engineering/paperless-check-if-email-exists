"use client";

import "client-only";

import type { ComponentPropsWithoutRef } from "react";
import { useQuery } from "@tanstack/react-query";
import { Configuration, V1Api } from "@oppulence/reacher-sdk";

import { cn } from "@oppulence/ui/lib/utils";

import { Pill } from "@/components/platform/page-kit";
import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";
import { V1ListSuppressionEvents200Response } from "@/lib/api/generated/zod/v1/v1";
import {
	labelFor,
	SUPPRESSION_ACTOR_LABELS,
	suppressionEvent,
	suppressionReason,
	suppressionSource,
	suppressionStatus,
} from "@/lib/verification/labels";

import { type SuppressionAuditPropsFields } from "./suppression-audit.schema";

/**
 * @oppulence-gen kind=component
 * SuppressionAudit is a client data component.
 * Shows the audit timeline of one suppression entry.
 *
 * It reads GET /v1/suppressions/{id}/events through the generated SDK. The API
 * sends the newest 50 events first. Owned by `suppression-audit.lit.ts`.
 */
export type SuppressionAuditProps = SuppressionAuditPropsFields &
	ComponentPropsWithoutRef<"section">;

const api = new V1Api(new Configuration({ basePath: "/api/backend" }));

function joined(...parts: (string | null | undefined)[]): string {
	return parts.filter(Boolean).join(" · ") || "Not recorded";
}

export function SuppressionAudit({ className, entryId, ...props }: SuppressionAuditProps) {
	const events = useQuery({
		queryKey: workflowQueryKey("suppression-events", entryId),
		queryFn: async () =>
			V1ListSuppressionEvents200Response.parse(
				(await api.v1ListSuppressionEvents({ id: entryId })).data,
			),
	});

	return (
		<section
			data-slot="suppression-audit"
			className={cn("flex flex-col gap-4", className)}
			{...props}
		>
			{events.isError ? (
				<p
					className="rounded-lg bg-red-500/10 px-4 py-3 text-sm text-red-300 ring-1 ring-red-500/20"
					role="alert"
				>
					{events.error.message}
				</p>
			) : null}
			{events.isPending ? (
				<p className="text-sm text-[#858585]" role="status">
					Loading the audit timeline…
				</p>
			) : null}
			{events.data && !events.data.events.length ? (
				<p className="text-sm text-[#858585]">No changes are recorded for this entry.</p>
			) : null}
			{events.data?.events.length ? (
				<ol className="flex flex-col">
					{events.data.events.map((event) => (
						<li className="relative border-l border-white/[0.1] pb-6 pl-5 last:pb-0" key={event.id}>
							<span
								aria-hidden
								className="absolute top-1.5 -left-[5px] size-2.5 rounded-full bg-neutral-500"
							/>
							<div className="flex flex-wrap items-center gap-2">
								<Pill tone={suppressionEvent.tone(event.event_type)}>
									{suppressionEvent.label(event.event_type)}
								</Pill>
								<time className="text-xs text-[#858585]" dateTime={event.created_at}>
									{new Date(event.created_at).toLocaleString()}
								</time>
							</div>
							<dl className="mt-3 grid grid-cols-[6rem_1fr] gap-x-3 gap-y-1.5 text-sm">
								<dt className="text-[#858585]">Status</dt>
								<dd className="text-white">
									{event.from_status
										? `From ${suppressionStatus.label(event.from_status)} to ${suppressionStatus.label(event.to_status)}`
										: `Set to ${suppressionStatus.label(event.to_status)}`}
								</dd>
								<dt className="text-[#858585]">Reason</dt>
								<dd className="break-words text-white">
									{joined(
										event.reason_code ? suppressionReason.label(event.reason_code) : null,
										event.reason_detail,
									)}
								</dd>
								<dt className="text-[#858585]">Source</dt>
								<dd className="break-words text-white">
									{joined(
										event.source_type ? suppressionSource.label(event.source_type) : null,
										event.source_ref,
									)}
								</dd>
								<dt className="text-[#858585]">Changed by</dt>
								<dd className="break-words text-white">
									{joined(
										labelFor(SUPPRESSION_ACTOR_LABELS, event.actor_type),
										event.actor_id === event.actor_type ? null : event.actor_id,
									)}
								</dd>
							</dl>
						</li>
					))}
				</ol>
			) : null}
			{events.data && events.data.total > events.data.events.length ? (
				<p className="text-xs text-[#858585]">
					The timeline shows the newest {events.data.events.length} of {events.data.total} changes.
				</p>
			) : null}
		</section>
	);
}
