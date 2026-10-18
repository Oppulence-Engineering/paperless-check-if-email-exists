"use client";

import "client-only";

import { useState, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";
import { useQuery } from "@tanstack/react-query";
import { Configuration, OutcomesApi } from "@oppulence/reacher-sdk";

import { Button } from "@oppulence/ui/components/button";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { cn } from "@oppulence/ui/lib/utils";

import { Group, Page, Row, Section } from "@/components/platform/page-kit";
import { useV1EmailHistory } from "@/hooks/queries/use-v1-email-history";
import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";

import { type HistoryPanelPropsFields } from "./history-panel.schema";

export type HistoryPanelProps = HistoryPanelPropsFields & ComponentPropsWithoutRef<"section">;

const outcomesApi = new OutcomesApi(new Configuration({ basePath: "/api/backend" }));

/** @oppulence-gen kind=component; owned by `history-panel.lit.ts`. */
export function HistoryPanel({ className, ...props }: HistoryPanelProps) {
	const [input, setInput] = useState("");
	const [email, setEmail] = useState("");
	const history = useV1EmailHistory(email, { limit: 50 });
	// Delivery events from the email platform, next to the check results.
	const events = useQuery({
		queryKey: workflowQueryKey("history-outcomes", email),
		enabled: Boolean(email),
		queryFn: async () => (await outcomesApi.v1ListOutcomes({ email, limit: 50 })).data,
	});

	function submit(event: SyntheticEvent<HTMLFormElement>) {
		event.preventDefault();
		setEmail(input.trim().toLowerCase());
	}

	return (
		<Page
			description="Review earlier results for an address in this workspace."
			title="Verification history"
			width="md"
		>
			<section
				className={cn("flex flex-col gap-10", className)}
				data-slot="history-panel"
				{...props}
			>
				<form onSubmit={submit}>
					<Group>
						<Row>
							<h3 className="text-sm font-medium text-white">Find an address</h3>
							<div className="flex flex-col gap-3 sm:flex-row sm:items-end">
								<div className="flex flex-1 flex-col gap-1.5">
									<Label className="text-xs font-medium text-neutral-400" htmlFor="history-email">
										Email address
									</Label>
									<Input
										className="h-10 rounded-lg"
										id="history-email"
										onChange={(event) => {
											setInput(event.target.value);
										}}
										required
										type="email"
										value={input}
									/>
								</div>
								<Button className="h-10 rounded-lg" type="submit">
									Search
								</Button>
							</div>
						</Row>
					</Group>
				</form>
				{email ? (
					<Section title={email}>
						<Group>
							{history.isPending ? (
								<Row>
									<p className="text-sm text-[#858585]" role="status">
										Loading history…
									</p>
								</Row>
							) : null}
							{history.isError ? (
								<Row>
									<p className="text-sm text-red-300" role="alert">
										{history.error.message}
									</p>
								</Row>
							) : null}
							{history.data ? (
								history.data.history.length === 0 ? (
									<Row>
										<p className="text-sm text-[#858585]">No earlier results for this address.</p>
									</Row>
								) : (
									history.data.history.map((entry, index) => (
										<div
											className="flex flex-wrap items-center gap-x-4 gap-y-3 px-5 py-4"
											key={`${String(entry.job_id ?? index)}-${String(entry.completed_at ?? index)}`}
										>
											<div className="min-w-0 flex-1">
												<p className="truncate text-sm text-white capitalize">
													{entry.is_reachable ?? entry.category ?? "Unknown result"}
												</p>
												<p className="text-xs text-[#858585]">
													{entry.reason_codes?.join(", ") ||
														entry.sub_reason ||
														"No reason recorded"}
												</p>
											</div>
											<div className="text-xs text-[#858585] sm:text-right">
												<p>{entry.score == null ? "No score" : `Score ${String(entry.score)}`}</p>
												<p>
													{entry.completed_at
														? new Date(entry.completed_at).toLocaleString()
														: "Pending timestamp"}
												</p>
											</div>
										</div>
									))
								)
							) : null}
						</Group>
						<Section title="Delivery events">
							<Group>
								{events.isPending ? (
									<Row>
										<p className="text-sm text-[#858585]" role="status">
											Loading delivery events…
										</p>
									</Row>
								) : null}
								{events.isError ? (
									<Row>
										<p className="text-sm text-[#858585]">
											Delivery events are visible to owners and admins.
										</p>
									</Row>
								) : null}
								{events.data ? (
									events.data.outcomes.length === 0 ? (
										<Row>
											<p className="text-sm text-[#858585]">No delivery events for this address.</p>
										</Row>
									) : (
										events.data.outcomes.map((event) => (
											<div
												className="flex flex-wrap items-center justify-between gap-x-4 gap-y-1 px-5 py-4 text-sm"
												key={`${event.event_type}-${event.occurred_at}-${event.provider}`}
											>
												<span className="text-white capitalize">
													{event.event_type} · {event.provider}
												</span>
												<span className="text-xs text-[#858585]">
													{new Date(event.occurred_at).toLocaleString()}
												</span>
											</div>
										))
									)
								) : null}
							</Group>
						</Section>
					</Section>
				) : null}
			</section>
		</Page>
	);
}
