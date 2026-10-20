"use client";

import "client-only";

import { useState, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";

import { Button } from "@oppulence/ui/components/button";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { cn } from "@oppulence/ui/lib/utils";

import { AddressTimeline } from "@/components/features/verification/address-timeline/address-timeline";
import { Group, Page, Row, Section } from "@/components/platform/page-kit";

import { type HistoryPanelPropsFields } from "./history-panel.schema";

export type HistoryPanelProps = HistoryPanelPropsFields & ComponentPropsWithoutRef<"section">;

/** @oppulence-gen kind=component; owned by `history-panel.lit.ts`. */
export function HistoryPanel({ className, ...props }: HistoryPanelProps) {
	const [input, setInput] = useState("");
	const [email, setEmail] = useState("");

	function submit(event: SyntheticEvent<HTMLFormElement>) {
		event.preventDefault();
		setEmail(input.trim().toLowerCase());
	}

	return (
		<Page
			description="Review earlier results for an address in this workspace."
			title="Verification history"
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
						<AddressTimeline email={email} key={email} />
					</Section>
				) : null}
			</section>
		</Page>
	);
}
