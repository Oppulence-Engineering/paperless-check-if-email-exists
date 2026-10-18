"use client";

import "client-only";

import Link from "next/link";
import type { ComponentPropsWithoutRef } from "react";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { Configuration, OutcomesApi, V1Api } from "@oppulence/reacher-sdk";

import { Button } from "@oppulence/ui/components/button";
import { cn } from "@oppulence/ui/lib/utils";

import { Group, Pill, Row } from "@/components/platform/page-kit";
import { useV1EmailHistory } from "@/hooks/queries/use-v1-email-history";
import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";
import { V1ListOutcomes200Response } from "@/lib/api/generated/zod/outcomes/outcomes";
import {
	V1AddSuppressions200Response,
	V1CheckSuppression200Response,
} from "@/lib/api/generated/zod/v1/v1";
import {
	category,
	labelFor,
	outcomeType,
	REASON_LABELS,
	recommendation,
	suppressionReason,
} from "@/lib/verification/labels";

import { type AddressTimelinePropsFields } from "./address-timeline.schema";

/**
 * @oppulence-gen kind=component
 * AddressTimeline is a client data component.
 * Shows earlier results, delivery events, and suppression status for one
 * address, and lets the user suppress it. It reads through the generated SDK
 * and TanStack Query. Owned by `address-timeline.lit.ts`.
 */
export type AddressTimelineProps = AddressTimelinePropsFields & ComponentPropsWithoutRef<"section">;

const configuration = new Configuration({ basePath: "/api/backend" });
const v1Api = new V1Api(configuration);
const outcomesApi = new OutcomesApi(configuration);

function when(value: string | null | undefined): string {
	return value ? new Date(value).toLocaleString() : "Time not recorded";
}

export function AddressTimeline({ className, email, ...props }: AddressTimelineProps) {
	const queryClient = useQueryClient();
	const suppressionKey = workflowQueryKey("address-timeline-suppression", email);
	const history = useV1EmailHistory(email, { limit: 50 });
	const events = useQuery({
		queryKey: workflowQueryKey("address-timeline-outcomes", email),
		enabled: Boolean(email),
		queryFn: async () =>
			V1ListOutcomes200Response.parse(
				(await outcomesApi.v1ListOutcomes({ email, limit: 50 })).data,
			),
	});
	const suppression = useQuery({
		queryKey: suppressionKey,
		enabled: Boolean(email),
		queryFn: async () =>
			V1CheckSuppression200Response.parse((await v1Api.v1CheckSuppression({ email })).data),
	});
	const suppress = useMutation({
		mutationFn: async () =>
			V1AddSuppressions200Response.parse(
				(
					await v1Api.v1AddSuppressions({
						addSuppressionsRequest: { emails: [email], reason: "manual" },
					})
				).data,
			),
		onSuccess: () => queryClient.invalidateQueries({ queryKey: suppressionKey }),
	});

	const items = [
		...(history.data?.history ?? []).map((entry) => ({
			kind: "check" as const,
			at: entry.completed_at ?? "",
			entry,
		})),
		...(events.data?.outcomes ?? []).map((event) => ({
			kind: "event" as const,
			at: event.occurred_at ?? event.created_at ?? "",
			event,
		})),
	].sort((a, b) => b.at.localeCompare(a.at));
	const loading = history.isPending || events.isPending;
	const status = suppression.data;

	return (
		<section data-slot="address-timeline" className={cn(className)} {...props}>
			<Group>
				<Row
					description={
						status?.suppressed
							? `${suppressionReason.label(status.reason)}${status.source ? ` · ${status.source}` : ""} · since ${when(status.created_at)}`
							: "Sends to this address are allowed."
					}
					label="Suppression"
					layout="inline"
				>
					{status?.suppressed ? (
						<Pill tone="red">Suppressed</Pill>
					) : (
						<Button
							disabled={!status || suppress.isPending}
							onClick={() => suppress.mutate()}
							size="sm"
							type="button"
							variant="outline"
						>
							{suppress.isPending ? "Adding…" : "Add to suppression list"}
						</Button>
					)}
				</Row>
				{suppress.isError ? (
					<Row>
						<p className="text-sm text-red-300" role="alert">
							We could not add this address to the suppression list. Try again.
						</p>
					</Row>
				) : null}
				{loading ? (
					<Row>
						<p className="text-sm text-[#858585]">Loading the history of this address…</p>
					</Row>
				) : items.length === 0 ? (
					<Row>
						<p className="text-sm text-[#858585]">
							No earlier checks or delivery events for this address.
						</p>
					</Row>
				) : (
					items.map((item) =>
						item.kind === "check" ? (
							<Row key={`check-${item.entry.task_id}`} layout="stacked">
								<div className="flex flex-wrap items-center gap-2 text-sm">
									<span className="text-[#858585]">{when(item.entry.completed_at)}</span>
									<span className="text-white">Check</span>
									<Pill tone={category.tone(item.entry.category)}>
										{category.label(item.entry.category)}
									</Pill>
									{item.entry.score != null ? (
										<span className="text-neutral-300">{item.entry.score}/100</span>
									) : null}
									{item.entry.recommendation_action ? (
										<Pill tone={recommendation.tone(item.entry.recommendation_action)}>
											{recommendation.label(item.entry.recommendation_action)}
										</Pill>
									) : null}
								</div>
								{item.entry.previous_category &&
								item.entry.previous_category !== item.entry.category ? (
									<p className="text-sm text-neutral-300">
										Changed from {category.label(item.entry.previous_category)} to{" "}
										{category.label(item.entry.category)}
									</p>
								) : null}
								{item.entry.reason_codes?.length ? (
									<p className="text-xs text-[#858585]">
										{item.entry.reason_codes
											.map((code) => labelFor(REASON_LABELS, code))
											.join(" · ")}
									</p>
								) : null}
								<div className="flex gap-4 text-xs">
									{item.entry.list_id ? (
										<Link
											className="text-neutral-300 underline underline-offset-4 hover:text-white"
											href={`/app/lists/detail?list=${item.entry.list_id}`}
										>
											Open the list
										</Link>
									) : null}
									{item.entry.job_id ? (
										<Link
											className="text-neutral-300 underline underline-offset-4 hover:text-white"
											href={`/app/jobs?job=${item.entry.job_id}`}
										>
											Open job {item.entry.job_id}
										</Link>
									) : null}
								</div>
							</Row>
						) : (
							<Row key={`event-${item.event.id}`} layout="stacked">
								<div className="flex flex-wrap items-center gap-2 text-sm">
									<span className="text-[#858585]">{when(item.at)}</span>
									<span className="text-white">Delivery event</span>
									<Pill tone={outcomeType.tone(item.event.type)}>
										{outcomeType.label(item.event.type)}
									</Pill>
									{item.event.source ? (
										<span className="text-neutral-300">{item.event.source}</span>
									) : null}
								</div>
								{item.event.campaign_id || item.event.policy_action ? (
									<p className="text-xs text-[#858585]">
										{[
											item.event.campaign_id ? `Campaign ${item.event.campaign_id}` : null,
											item.event.policy_action ? `Action: ${item.event.policy_action}` : null,
										]
											.filter(Boolean)
											.join(" · ")}
									</p>
								) : null}
							</Row>
						),
					)
				)}
			</Group>
		</section>
	);
}
