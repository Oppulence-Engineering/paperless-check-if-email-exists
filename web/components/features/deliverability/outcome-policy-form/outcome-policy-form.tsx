"use client";

import "client-only";

import { useState, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import type { z } from "zod";
import { Configuration, TenantApi } from "@oppulence/reacher-sdk";

import { Button } from "@oppulence/ui/components/button";
import { Checkbox } from "@oppulence/ui/components/checkbox";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { NativeSelect, NativeSelectOption } from "@oppulence/ui/components/native-select";
import { cn } from "@oppulence/ui/lib/utils";

import {
	apiMessage,
	FILTER_CLASS,
} from "@/components/features/deliverability/outcome-events/outcome-events";
import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";
import { V1GetOutcomePolicy200Response } from "@/lib/api/generated/zod/tenant/tenant";
import { outcomeType, ruleAction } from "@/lib/verification/labels";

import { type OutcomePolicyFormPropsFields } from "./outcome-policy-form.schema";

/**
 * @oppulence-gen kind=component
 * OutcomePolicyForm is a client data component.
 * Creates an outcome policy, or changes one. It sets one rule for each
 * campaign event and the age of the events that later checks use.
 * Owned by `outcome-policy-form.lit.ts`.
 */
export type OutcomePolicyFormProps = OutcomePolicyFormPropsFields & {
	onSaved?: () => void;
} & ComponentPropsWithoutRef<"section">;

const api = new TenantApi(new Configuration({ basePath: "/api/backend" }));
const FIELD_LABEL = "text-xs font-medium text-neutral-400";

/** The events that a policy has a rule for, in the order of `OutcomePolicyRules`. */
export const RULE_EVENTS = [
	"hard_bounce",
	"soft_bounce",
	"complaint",
	"unsubscribe",
	"delivered",
	"open",
	"click",
] as const;

export type RuleEvent = (typeof RULE_EVENTS)[number];

/** One rule of a policy (`OutcomeRuleAction` in the contract). */
export type OutcomeRule = NonNullable<
	NonNullable<z.infer<typeof V1GetOutcomePolicy200Response>["rules"]>[RuleEvent]
>;

/** A copy of `default_outcome_policy_rules` in backend/src/outcomes.rs. */
const DEFAULT_RULES: Record<RuleEvent, OutcomeRule> = {
	hard_bounce: { action: "suppress", score_override: "invalid" },
	soft_bounce: { action: "suppress_after", threshold_count: 3, threshold_window_days: 30 },
	complaint: { action: "suppress_and_unsubscribe", score_override: "invalid" },
	unsubscribe: { action: "suppress" },
	delivered: { action: "score_boost", boost: 5 },
	open: { action: "score_boost", boost: 3 },
	click: { action: "score_boost", boost: 8 },
};

/** One rule in plain words. */
export function ruleText(rule: OutcomeRule | undefined): string {
	if (!rule) return "Uses the default rule.";
	switch (rule.action) {
		case "suppress":
			return "Add the address to the suppression list.";
		case "suppress_and_unsubscribe":
			return "Add the address to the suppression list and mark it as unsubscribed.";
		case "suppress_after":
			return `Add the address to the suppression list after ${String(rule.threshold_count)} of these events in ${String(rule.threshold_window_days)} days.`;
		case "score_boost":
			return "Raise the score of the address in later checks.";
		case "informational":
			return "Record the event only.";
		default:
			return ruleAction.label(rule.action);
	}
}

/** Only the fields that the action uses. The backend stores the rules as sent. */
function cleanRule(rule: OutcomeRule, fallback: OutcomeRule): OutcomeRule {
	const { action } = rule;
	if (action === "suppress_after")
		return {
			action,
			threshold_count: rule.threshold_count ?? 3,
			threshold_window_days: rule.threshold_window_days ?? 30,
		};
	if (action === "score_boost") return { action, boost: rule.boost ?? fallback.boost ?? 5 };
	if ((action === "suppress" || action === "suppress_and_unsubscribe") && rule.score_override)
		return { action, score_override: rule.score_override };
	return { action };
}

export function OutcomePolicyForm({
	className,
	onSaved,
	policy,
	...props
}: OutcomePolicyFormProps) {
	const client = useQueryClient();
	const [name, setName] = useState(policy?.name ?? "");
	const [isDefault, setIsDefault] = useState(policy?.is_default ?? false);
	const [rules, setRules] = useState<Record<RuleEvent, OutcomeRule>>(() => {
		const current = policy?.rules ?? {};
		return Object.fromEntries(
			RULE_EVENTS.map((event) => [event, current[event] ?? DEFAULT_RULES[event]]),
		) as Record<RuleEvent, OutcomeRule>;
	});
	const [ttlDays, setTtlDays] = useState(policy?.rules.outcome_ttl_days ?? 90);
	const save = useMutation({
		mutationFn: async () => {
			const body = {
				name: name.trim(),
				is_default: isDefault,
				rules: {
					...Object.fromEntries(
						RULE_EVENTS.map((event) => [event, cleanRule(rules[event], DEFAULT_RULES[event])]),
					),
					outcome_ttl_days: ttlDays,
				},
			};
			const response = policy
				? await api.v1UpdateOutcomePolicy({
						policyId: policy.id,
						updateOutcomePolicyRequest: body,
					})
				: await api.v1CreateOutcomePolicy({ createOutcomePolicyRequest: body });
			return V1GetOutcomePolicy200Response.parse(response.data);
		},
		onSuccess: async () => {
			await client.invalidateQueries({ queryKey: workflowQueryKey("outcome-policies") });
			onSaved?.();
		},
	});

	function setRule(event: RuleEvent, change: Partial<OutcomeRule>) {
		setRules((current) => ({ ...current, [event]: { ...current[event], ...change } }));
	}

	function submit(event: SyntheticEvent<HTMLFormElement>) {
		event.preventDefault();
		save.mutate();
	}

	return (
		<section
			data-slot="outcome-policy-form"
			className={cn("flex flex-col gap-4", className)}
			{...props}
		>
			{save.isError ? (
				<p
					className="rounded-lg bg-red-500/10 px-4 py-3 text-sm text-red-300 ring-1 ring-red-500/20"
					role="alert"
				>
					{apiMessage(save.error)}
				</p>
			) : null}
			<form className="flex flex-col gap-5" onSubmit={submit}>
				<div className="flex flex-col gap-1.5">
					<Label className={FIELD_LABEL} htmlFor="policy-name">
						Name
					</Label>
					<Input
						className="h-10 rounded-lg"
						id="policy-name"
						onChange={(event) => {
							setName(event.target.value);
						}}
						required
						value={name}
					/>
				</div>
				<Label
					className="flex items-center gap-2 text-sm text-neutral-200"
					htmlFor="policy-default"
				>
					<Checkbox
						checked={isDefault}
						id="policy-default"
						onCheckedChange={(checked) => {
							setIsDefault(checked === true);
						}}
					/>
					Use as the default policy
				</Label>
				<p className="text-xs text-[#858585]">
					Only the default policy applies. It applies to campaign results.
				</p>
				{RULE_EVENTS.map((event) => {
					const rule = rules[event];
					const label = outcomeType.label(event);
					return (
						<div className="flex flex-col gap-1.5" key={event}>
							<Label className={FIELD_LABEL} htmlFor={`rule-${event}`}>
								{label}
							</Label>
							<NativeSelect
								className={FILTER_CLASS}
								id={`rule-${event}`}
								onChange={(change) => {
									// The options come from the ruleAction labels.
									setRule(event, { action: change.target.value as OutcomeRule["action"] });
								}}
								value={rule.action}
							>
								{ruleAction.values.map((value) => (
									<NativeSelectOption key={value} value={value}>
										{ruleAction.label(value)}
									</NativeSelectOption>
								))}
							</NativeSelect>
							{rule.action === "suppress_after" ? (
								<div className="flex flex-wrap items-center gap-2">
									<Input
										className="h-9 w-20 rounded-lg"
										id={`rule-${event}-count`}
										min={1}
										onChange={(change) => {
											setRule(event, { threshold_count: Number(change.target.value) });
										}}
										required
										type="number"
										value={rule.threshold_count ?? 3}
									/>
									<Label className="text-xs text-[#858585]" htmlFor={`rule-${event}-count`}>
										<span className="sr-only">{label} </span>events in
									</Label>
									<Input
										className="h-9 w-20 rounded-lg"
										id={`rule-${event}-days`}
										min={1}
										onChange={(change) => {
											setRule(event, { threshold_window_days: Number(change.target.value) });
										}}
										required
										type="number"
										value={rule.threshold_window_days ?? 30}
									/>
									<Label className="text-xs text-[#858585]" htmlFor={`rule-${event}-days`}>
										<span className="sr-only">{label} </span>days
									</Label>
								</div>
							) : null}
							<p className="text-xs text-[#858585]">{ruleText(rule)}</p>
						</div>
					);
				})}
				<div className="flex flex-col gap-1.5">
					<Label className={FIELD_LABEL} htmlFor="policy-ttl">
						Event age in days
					</Label>
					<Input
						className="h-10 w-28 rounded-lg"
						id="policy-ttl"
						min={1}
						onChange={(event) => {
							setTtlDays(Number(event.target.value));
						}}
						required
						type="number"
						value={ttlDays}
					/>
					<p className="text-xs text-[#858585]">
						Later checks use the events of the last {ttlDays} days.
					</p>
				</div>
				<div className="flex justify-end">
					<Button className="rounded-lg" disabled={save.isPending} type="submit">
						{policy ? "Save policy" : "Create policy"}
					</Button>
				</div>
			</form>
		</section>
	);
}
