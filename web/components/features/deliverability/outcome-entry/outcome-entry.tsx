"use client";

import "client-only";

import { useState, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { Configuration, OutcomesApi } from "@oppulence/reacher-sdk";

import { Button } from "@oppulence/ui/components/button";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { cn } from "@oppulence/ui/lib/utils";

import {
	apiMessage,
	FILTER_CLASS,
	PROVIDER_EVENT_TYPES,
} from "@/components/features/deliverability/outcome-events/outcome-events";
import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";
import { V1IngestOutcomes200Response } from "@/lib/api/generated/zod/outcomes/outcomes";
import { outcomeType } from "@/lib/verification/labels";

import { type OutcomeEntryPropsFields } from "./outcome-entry.schema";

/**
 * @oppulence-gen kind=component
 * OutcomeEntry is a client data component.
 * Records one delivery event by hand through POST /v1/outcomes.
 *
 * A bounce, a complaint, or an unsubscribe adds the address to the
 * suppression list. Owned by `outcome-entry.lit.ts`.
 */
export type OutcomeEntryProps = OutcomeEntryPropsFields & ComponentPropsWithoutRef<"section">;

const api = new OutcomesApi(new Configuration({ basePath: "/api/backend" }));
const FIELD_LABEL = "text-xs font-medium text-neutral-400";

function text(form: FormData, name: string): string {
	return String(form.get(name) ?? "").trim();
}

export function OutcomeEntry({ className, ...props }: OutcomeEntryProps) {
	const client = useQueryClient();
	const [notice, setNotice] = useState("");
	const ingest = useMutation({
		mutationFn: async (form: FormData) => {
			const occurredAt = text(form, "occurred_at");
			return V1IngestOutcomes200Response.parse(
				(
					await api.v1IngestOutcomes({
						outcomeIngestRequest: {
							provider: text(form, "provider"),
							outcomes: [
								{
									email: text(form, "email"),
									event_type: text(form, "event_type"),
									occurred_at: occurredAt ? new Date(occurredAt).toISOString() : undefined,
									source_key: text(form, "source_key") || undefined,
									campaign_id: text(form, "campaign_id") || undefined,
								},
							],
						},
					})
				).data,
			);
		},
		onSuccess: async (data) => {
			setNotice(
				`Recorded ${String(data.ingested)} outcome(s); ${String(data.auto_suppressed)} auto-suppressed; ${String(data.ignored)} skipped.`,
			);
			await client.invalidateQueries({ queryKey: workflowQueryKey("outcomes") });
		},
	});

	function submit(event: SyntheticEvent<HTMLFormElement>) {
		event.preventDefault();
		const formElement = event.currentTarget;
		if (
			!window.confirm(
				"Record this delivery outcome? Bounce or complaint events may suppress the address.",
			)
		)
			return;
		setNotice("");
		ingest.mutate(new FormData(formElement), {
			onSuccess: () => {
				formElement.reset();
			},
		});
	}

	return (
		<section data-slot="outcome-entry" className={cn("flex flex-col gap-4", className)} {...props}>
			{ingest.isError ? (
				<p
					className="rounded-lg bg-red-500/10 px-4 py-3 text-sm text-red-300 ring-1 ring-red-500/20"
					role="alert"
				>
					{apiMessage(ingest.error)}
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
			<form className="flex flex-col gap-4" onSubmit={submit}>
				<div className="flex flex-col gap-1.5">
					<Label className={FIELD_LABEL} htmlFor="event-email">
						Email
					</Label>
					<Input className="h-10 rounded-lg" id="event-email" name="email" required type="email" />
				</div>
				<div className="grid gap-4 sm:grid-cols-2">
					<div className="flex flex-col gap-1.5">
						<Label className={FIELD_LABEL} htmlFor="event-type">
							Event
						</Label>
						<select
							className={FILTER_CLASS}
							defaultValue="delivered"
							id="event-type"
							name="event_type"
						>
							{PROVIDER_EVENT_TYPES.map((value) => (
								<option key={value} value={value}>
									{outcomeType.label(value)}
								</option>
							))}
						</select>
					</div>
					<div className="flex flex-col gap-1.5">
						<Label className={FIELD_LABEL} htmlFor="event-occurred-at">
							Occurred at
						</Label>
						<Input
							className="h-10 rounded-lg"
							id="event-occurred-at"
							name="occurred_at"
							type="datetime-local"
						/>
					</div>
					<div className="flex flex-col gap-1.5">
						<Label className={FIELD_LABEL} htmlFor="event-provider">
							Source
						</Label>
						<Input
							className="h-10 rounded-lg"
							defaultValue="manual"
							id="event-provider"
							name="provider"
							required
						/>
					</div>
					<div className="flex flex-col gap-1.5">
						<Label className={FIELD_LABEL} htmlFor="event-source-key">
							Source key
						</Label>
						<Input
							className="h-10 rounded-lg"
							id="event-source-key"
							name="source_key"
							placeholder="newsletter"
						/>
					</div>
				</div>
				<div className="flex flex-col gap-1.5">
					<Label className={FIELD_LABEL} htmlFor="event-campaign">
						Campaign
					</Label>
					<Input className="h-10 rounded-lg" id="event-campaign" name="campaign_id" />
				</div>
				<p className="text-xs text-[#858585]">
					Leave the time empty to use the current time. The source key and the campaign are
					optional.
				</p>
				<div className="flex justify-end">
					<Button className="rounded-lg" disabled={ingest.isPending} type="submit">
						Record outcome
					</Button>
				</div>
			</form>
		</section>
	);
}
