import type { ComponentPropsWithoutRef } from "react";

import { cn } from "@oppulence/ui/lib/utils";

import { Group, Pill, Row } from "@/components/platform/page-kit";
import { Check, Minus, X } from "@/lib/icons";
import { catchAllSeverity, SIGNALS } from "@/lib/verification/labels";

import { type SignalChecklistPropsFields } from "./signal-checklist.schema";

/**
 * @oppulence-gen kind=component
 * SignalChecklist is a server presentation component.
 * Shows each scoring signal as passed, failed, or neutral.
 *
 * Data and callbacks arrive through validated props. This module does not fetch,
 * persist, or read cookies. Owned by `signal-checklist.lit.ts`.
 */
export type SignalChecklistProps = SignalChecklistPropsFields & ComponentPropsWithoutRef<"section">;

type State = "pass" | "fail" | "neutral";

const STATE_STYLE: Record<State, { icon: typeof Check; className: string; label: string }> = {
	pass: { icon: Check, className: "text-emerald-300", label: "Passed" },
	fail: { icon: X, className: "text-red-300", label: "Failed" },
	neutral: { icon: Minus, className: "text-[#858585]", label: "Neutral" },
};

/** A signal passes when its value is the one that lowers bounce risk. */
export function signalState(value: boolean, good: boolean | null): State {
	if (good === null) return "neutral";
	return value === good ? "pass" : "fail";
}

export function SignalChecklist({ className, result, ...props }: SignalChecklistProps) {
	const signals = result.score.signals as Record<string, unknown>;
	const severity = result.score.catch_all?.severity ?? result.score.catch_all_severity;
	const catchAllFactors = result.score.catch_all?.factors ?? [];

	return (
		<section data-slot="signal-checklist" className={cn(className)} {...props}>
			<Group>
				<Row>
					<ul className="grid gap-x-6 gap-y-2.5 sm:grid-cols-2">
						{SIGNALS.map((signal) => {
							const value = signals[signal.key];
							if (typeof value !== "boolean") return null;
							const state = signalState(value, signal.good);
							const { icon: Icon, className: tone, label } = STATE_STYLE[state];
							return (
								<li className="flex items-center gap-2.5 text-sm" key={signal.key}>
									<Icon className={cn("size-4", tone)} title={label} />
									<span className={state === "fail" ? "text-white" : "text-neutral-300"}>
										{signal.label}
										{signal.good === null ? `: ${value ? "yes" : "no"}` : null}
									</span>
								</li>
							);
						})}
					</ul>
				</Row>
				{result.score.signals.smtp_is_catch_all && severity ? (
					<Row
						description={
							result.score.catch_all
								? `Confidence ${result.score.catch_all.confidence}/100`
								: "The domain accepts mail for any address, so the mailbox cannot be confirmed."
						}
						label="Catch-all risk"
						layout="inline"
					>
						<Pill tone={catchAllSeverity.tone(severity)}>{catchAllSeverity.label(severity)}</Pill>
					</Row>
				) : null}
				{catchAllFactors.length > 0 ? (
					<Row label="Catch-all factors" layout="stacked">
						<ul className="list-disc space-y-1 pl-5 text-sm text-neutral-300">
							{catchAllFactors.map((factor) => (
								<li key={factor}>{factor}</li>
							))}
						</ul>
					</Row>
				) : null}
			</Group>
		</section>
	);
}
