"use client";

import "client-only";

import { useState, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";

import { Button } from "@oppulence/ui/components/button";
import { Card, CardContent, CardHeader, CardTitle } from "@oppulence/ui/components/card";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { cn } from "@oppulence/ui/lib/utils";

import { useV1EmailHistory } from "@/hooks/queries/use-v1-email-history";

import { type HistoryPanelPropsFields } from "./history-panel.schema";

export type HistoryPanelProps = HistoryPanelPropsFields & ComponentPropsWithoutRef<"section">;

/** @oppulence-gen kind=component; owned by `history-panel.lit.ts`. */
export function HistoryPanel({ className, ...props }: HistoryPanelProps) {
	const [input, setInput] = useState("");
	const [email, setEmail] = useState("");
	const history = useV1EmailHistory(email, { limit: 50 });

	function submit(event: SyntheticEvent<HTMLFormElement>) {
		event.preventDefault();
		setEmail(input.trim().toLowerCase());
	}

	return (
		<section className={cn("space-y-8", className)} data-slot="history-panel" {...props}>
			<div>
				<h1 className="text-3xl font-semibold">Verification history</h1>
				<p className="mt-2 text-muted-foreground">
					Review earlier results for an address in this workspace.
				</p>
			</div>
			<Card>
				<CardHeader>
					<CardTitle>Find an address</CardTitle>
				</CardHeader>
				<CardContent>
					<form className="flex flex-col gap-4 sm:flex-row sm:items-end" onSubmit={submit}>
						<div className="flex-1 space-y-2">
							<Label htmlFor="history-email">Email address</Label>
							<Input
								id="history-email"
								onChange={(event) => {
									setInput(event.target.value);
								}}
								required
								type="email"
								value={input}
							/>
						</div>
						<Button type="submit">Search</Button>
					</form>
				</CardContent>
			</Card>
			{email ? (
				<Card>
					<CardHeader>
						<CardTitle>{email}</CardTitle>
					</CardHeader>
					<CardContent>
						{history.isPending ? <p role="status">Loading history…</p> : null}
						{history.isError ? (
							<p className="text-destructive" role="alert">
								{history.error.message}
							</p>
						) : null}
						{history.data ? (
							history.data.history.length === 0 ? (
								<p className="text-muted-foreground">No earlier results for this address.</p>
							) : (
								<ol className="divide-y divide-border">
									{history.data.history.map((entry, index) => (
										<li
											className="grid gap-2 py-4 sm:grid-cols-[1fr_auto]"
											key={`${String(entry.job_id ?? index)}-${String(entry.completed_at ?? index)}`}
										>
											<div>
												<p className="font-medium capitalize">
													{entry.is_reachable ?? entry.category ?? "Unknown result"}
												</p>
												<p className="text-sm text-muted-foreground">
													{entry.reason_codes?.join(", ") ||
														entry.sub_reason ||
														"No reason recorded"}
												</p>
											</div>
											<div className="text-sm text-muted-foreground sm:text-right">
												<p>{entry.score == null ? "No score" : `Score ${String(entry.score)}`}</p>
												<p>
													{entry.completed_at
														? new Date(entry.completed_at).toLocaleString()
														: "Pending timestamp"}
												</p>
											</div>
										</li>
									))}
								</ol>
							)
						) : null}
					</CardContent>
				</Card>
			) : null}
		</section>
	);
}
