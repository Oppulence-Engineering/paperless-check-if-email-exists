"use client";

import "client-only";

import { useState, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";

import { Button } from "@oppulence/ui/components/button";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { NativeSelect, NativeSelectOption } from "@oppulence/ui/components/native-select";
import { cn } from "@oppulence/ui/lib/utils";

import { FILTER_CLASS } from "@/components/features/deliverability/outcome-events/outcome-events";
import {
	ExplorerFiltersSchema,
	type ExplorerFilters,
} from "@/components/features/insights/result-explorer/result-explorer.schema";
import { POLICY_MODE_LABELS, policyDecision, recommendation } from "@/lib/verification/labels";

import { type ResultFiltersPropsFields } from "./result-filters.schema";

/**
 * @oppulence-gen kind=component
 * ResultFilters is a client presentation component.
 * Edits the score, job, date, recommendation, and policy filters of the result explorer.
 *
 * The form keeps the category and safe-to-send filters as they are. It does
 * not fetch. Owned by `result-filters.lit.ts`.
 */
export type ResultFiltersProps = ResultFiltersPropsFields & {
	onApply: (filters: ExplorerFilters) => void;
} & ComponentPropsWithoutRef<"form">;

/** The actions of the decision engine (`RecommendedAction` in backend/src/decision/types.rs). */
const DECISION_ACTIONS = [
	"send",
	"send_with_caution",
	"fix_then_send",
	"review",
	"suppress",
	"drop",
];
const FIELD_LABEL = "text-xs font-medium text-neutral-400";

export function ResultFilters({ className, filters, onApply, ...props }: ResultFiltersProps) {
	const [error, setError] = useState("");

	function apply(event: SyntheticEvent<HTMLFormElement>) {
		event.preventDefault();
		const form = new FormData(event.currentTarget);
		const text = (key: string) => String(form.get(key) ?? "").trim() || undefined;
		const number = (key: string) => {
			const value = text(key);
			return value === undefined ? undefined : Number(value);
		};
		const parsed = ExplorerFiltersSchema.safeParse({
			...filters,
			minScore: number("min_score"),
			maxScore: number("max_score"),
			jobId: number("job_id"),
			since: text("since"),
			until: text("until"),
			recommendationAction: text("recommendation_action"),
			policyMode: text("policy_mode"),
			policyDecision: text("policy_decision"),
		});
		if (!parsed.success) {
			setError("Use a score from 0 to 100 and a job ID that is a positive whole number.");
			return;
		}
		setError("");
		onApply(parsed.data);
	}

	const select = (
		name: string,
		label: string,
		value: string | undefined,
		options: [string, string][],
	) => (
		<div className="flex flex-col gap-1.5">
			<Label className={FIELD_LABEL} htmlFor={`result-filter-${name}`}>
				{label}
			</Label>
			<NativeSelect
				className={FILTER_CLASS}
				defaultValue={value ?? ""}
				id={`result-filter-${name}`}
				name={name}
			>
				<NativeSelectOption value="">Any</NativeSelectOption>
				{options.map(([key, text]) => (
					<NativeSelectOption key={key} value={key}>
						{text}
					</NativeSelectOption>
				))}
			</NativeSelect>
		</div>
	);

	const input = (
		name: string,
		label: string,
		value: string | number | undefined,
		extra: ComponentPropsWithoutRef<"input">,
	) => (
		<div className="flex flex-col gap-1.5">
			<Label className={FIELD_LABEL} htmlFor={`result-filter-${name}`}>
				{label}
			</Label>
			<Input
				className="h-10 rounded-lg"
				defaultValue={value ?? ""}
				id={`result-filter-${name}`}
				name={name}
				{...extra}
			/>
		</div>
	);

	return (
		<form
			data-slot="result-filters"
			className={cn("flex flex-col gap-5", className)}
			{...props}
			noValidate
			onSubmit={apply}
		>
			<div className="grid gap-4 sm:grid-cols-2">
				{input("min_score", "Lowest score", filters.minScore, {
					inputMode: "numeric",
					placeholder: "0",
				})}
				{input("max_score", "Highest score", filters.maxScore, {
					inputMode: "numeric",
					placeholder: "100",
				})}
				{input("since", "Checked from", filters.since, { type: "date" })}
				{input("until", "Checked to", filters.until, { type: "date" })}
				{input("job_id", "Job ID", filters.jobId, { inputMode: "numeric" })}
				{select(
					"recommendation_action",
					"Recommendation",
					filters.recommendationAction,
					DECISION_ACTIONS.map((value) => [value, recommendation.label(value)]),
				)}
				{select(
					"policy_mode",
					"Policy mode",
					filters.policyMode,
					Object.entries(POLICY_MODE_LABELS),
				)}
				{select(
					"policy_decision",
					"Policy decision",
					filters.policyDecision,
					policyDecision.values.map((value) => [value, policyDecision.label(value)]),
				)}
			</div>
			{error ? (
				<p className="text-sm text-red-300" role="alert">
					{error}
				</p>
			) : null}
			<Button className="h-10 self-start rounded-lg" type="submit">
				Apply filters
			</Button>
		</form>
	);
}
