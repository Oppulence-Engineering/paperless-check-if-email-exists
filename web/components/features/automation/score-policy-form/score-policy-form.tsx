"use client";

import "client-only";

import type { z } from "zod";

import { useState, type ComponentPropsWithoutRef, type SyntheticEvent } from "react";
import { useMutation, useQueryClient } from "@tanstack/react-query";
import { Configuration, TenantApi } from "@oppulence/reacher-sdk";

import { Button } from "@oppulence/ui/components/button";
import { Checkbox } from "@oppulence/ui/components/checkbox";
import { Input } from "@oppulence/ui/components/input";
import { Label } from "@oppulence/ui/components/label";
import { cn } from "@oppulence/ui/lib/utils";

import {
	apiMessage,
	FILTER_CLASS,
} from "@/components/features/deliverability/outcome-events/outcome-events";
import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";
import { V1GetScorePolicy200Response } from "@/lib/api/generated/zod/tenant/tenant";
import { category, labelFor, policyDecision, REASON_LABELS } from "@/lib/verification/labels";

import { type ScorePolicyFormPropsFields } from "./score-policy-form.schema";

type ScorePolicyRules = NonNullable<z.infer<typeof V1GetScorePolicy200Response>["rules"]>;
type ScorePolicyCondition = NonNullable<ScorePolicyRules["send"]>;

/**
 * @oppulence-gen kind=component
 * ScorePolicyForm is a client data component.
 * Creates a score policy, or changes one. It sets one condition for each
 * decision. Owned by `score-policy-form.lit.ts`.
 */
export type ScorePolicyFormProps = ScorePolicyFormPropsFields & {
	onSaved?: () => void;
} & ComponentPropsWithoutRef<"section">;

const api = new TenantApi(new Configuration({ basePath: "/api/backend" }));
const FIELD_LABEL = "text-xs font-medium text-neutral-400";

/** The decisions in the order that the backend tries them (`evaluate_policy`). */
export const DECISIONS = ["suppress", "send", "review"] as const;

type Decision = (typeof DECISIONS)[number];

/** What the backend decides when no condition matches (`default_policy_decision`). */
export const BUILT_IN_RULE =
	"Send when the address is safe to send and the score is 80 or more. Suppress when the address is not safe to send, is invalid, or has a score of 30 or less. Review all other addresses.";

/** The rules of a policy, or null when they have a shape that this page does not know. */
export function parseRules(value: unknown): ScorePolicyRules | null {
	const parsed = V1GetScorePolicy200Response.shape.rules.safeParse(value ?? {});
	return parsed.success ? parsed.data : null;
}

/** One condition in plain words. */
export function conditionText(condition: ScorePolicyCondition | null | undefined): string {
	if (!condition) return "No rule.";
	const parts: string[] = [];
	const { score_min: min, score_max: max } = condition;
	if (min != null && max != null) parts.push(`the score is ${String(min)} to ${String(max)}`);
	else if (min != null) parts.push(`the score is ${String(min)} or more`);
	else if (max != null) parts.push(`the score is ${String(max)} or less`);
	if (condition.category?.length)
		parts.push(
			`the category is ${condition.category.map((value) => category.label(value).toLowerCase()).join(" or ")}`,
		);
	if (condition.safe_to_send != null)
		parts.push(`the address is ${condition.safe_to_send ? "" : "not "}safe to send`);
	if (condition.reason_codes_any?.length)
		parts.push(
			`the result has one of these reasons: ${condition.reason_codes_any.map((code) => labelFor(REASON_LABELS, code)).join("; ")}`,
		);
	return parts.length ? `When ${parts.join(" and ")}.` : "All addresses.";
}

/** The form fields of one condition, as text. */
type Draft = { min: string; max: string; categories: string[]; safe: string; reasons: string };

function toDraft(condition: ScorePolicyCondition | null | undefined): Draft {
	return {
		min: condition?.score_min?.toString() ?? "",
		max: condition?.score_max?.toString() ?? "",
		categories: condition?.category ?? [],
		safe: condition?.safe_to_send == null ? "" : String(condition.safe_to_send),
		reasons: condition?.reason_codes_any?.join(", ") ?? "",
	};
}

/** Only the fields that are set. Undefined when none is set, because an empty condition matches all addresses. */
function fromDraft(draft: Draft): ScorePolicyCondition | undefined {
	const reasons = draft.reasons
		.split(",")
		.map((code) => code.trim())
		.filter(Boolean);
	const condition = {
		...(draft.min ? { score_min: Number(draft.min) } : {}),
		...(draft.max ? { score_max: Number(draft.max) } : {}),
		...(draft.categories.length ? { category: draft.categories } : {}),
		...(draft.safe ? { safe_to_send: draft.safe === "true" } : {}),
		...(reasons.length ? { reason_codes_any: reasons } : {}),
	};
	return Object.keys(condition).length ? condition : undefined;
}

export function ScorePolicyForm({ className, onSaved, policy, ...props }: ScorePolicyFormProps) {
	const client = useQueryClient();
	const [name, setName] = useState(policy?.name ?? "");
	const [isDefault, setIsDefault] = useState(policy?.is_default ?? false);
	const [drafts, setDrafts] = useState<Record<Decision, Draft>>(() => {
		const rules = parseRules(policy?.rules);
		return Object.fromEntries(
			DECISIONS.map((decision) => [decision, toDraft(rules?.[decision])]),
		) as Record<Decision, Draft>;
	});
	const save = useMutation({
		mutationFn: async () => {
			const rules = Object.fromEntries(
				DECISIONS.flatMap((decision) => {
					const condition = fromDraft(drafts[decision]);
					return condition ? [[decision, condition]] : [];
				}),
			);
			const body = { name: name.trim(), is_default: isDefault, rules };
			const response = policy
				? await api.v1UpdateScorePolicy({ policyId: policy.id, updateScorePolicyRequest: body })
				: await api.v1CreateScorePolicy({ createScorePolicyRequest: body });
			return V1GetScorePolicy200Response.parse(response.data);
		},
		onSuccess: async () => {
			await client.invalidateQueries({ queryKey: workflowQueryKey("score-policies") });
			onSaved?.();
		},
	});

	function setDraft(decision: Decision, change: Partial<Draft>) {
		setDrafts((current) => ({ ...current, [decision]: { ...current[decision], ...change } }));
	}

	function submit(event: SyntheticEvent<HTMLFormElement>) {
		event.preventDefault();
		save.mutate();
	}

	return (
		<section
			data-slot="score-policy-form"
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
					<Label className={FIELD_LABEL} htmlFor="score-policy-name">
						Name
					</Label>
					<Input
						className="h-10 rounded-lg"
						id="score-policy-name"
						onChange={(event) => {
							setName(event.target.value);
						}}
						required
						value={name}
					/>
				</div>
				<Label
					className="flex items-center gap-2 text-sm text-neutral-200"
					htmlFor="score-policy-default"
				>
					<Checkbox
						checked={isDefault}
						id="score-policy-default"
						onCheckedChange={(checked) => {
							setIsDefault(checked === true);
						}}
					/>
					Use as the default policy
				</Label>
				<p className="text-xs text-[#858585]">
					The backend tries Suppress, then Send, then Review. The first rule that matches decides.
					Leave all fields of a rule empty to remove it.
				</p>
				{DECISIONS.map((decision) => {
					const draft = drafts[decision];
					const label = policyDecision.label(decision);
					const id = `score-policy-${decision}`;
					const prefix = <span className="sr-only">{label}: </span>;
					return (
						<fieldset
							className="flex flex-col gap-3 rounded-xl p-4 ring-1 ring-white/[0.08]"
							key={decision}
						>
							<legend className="px-1 text-sm font-medium text-white">{label}</legend>
							<div className="grid grid-cols-2 gap-3">
								<div className="flex flex-col gap-1.5">
									<Label className={FIELD_LABEL} htmlFor={`${id}-min`}>
										{prefix}Lowest score
									</Label>
									<Input
										className="h-10 rounded-lg"
										id={`${id}-min`}
										max={100}
										min={0}
										onChange={(event) => {
											setDraft(decision, { min: event.target.value });
										}}
										step={1}
										type="number"
										value={draft.min}
									/>
								</div>
								<div className="flex flex-col gap-1.5">
									<Label className={FIELD_LABEL} htmlFor={`${id}-max`}>
										{prefix}Highest score
									</Label>
									<Input
										className="h-10 rounded-lg"
										id={`${id}-max`}
										max={100}
										min={0}
										onChange={(event) => {
											setDraft(decision, { max: event.target.value });
										}}
										step={1}
										type="number"
										value={draft.max}
									/>
								</div>
							</div>
							<fieldset className="flex flex-col gap-2">
								<legend className={`mb-2 ${FIELD_LABEL}`}>{prefix}Categories</legend>
								<div className="flex flex-wrap gap-x-4 gap-y-2">
									{category.values.map((value) => (
										<Label
											className="flex items-center gap-2 text-xs text-neutral-300"
											htmlFor={`${id}-category-${value}`}
											key={value}
										>
											<Checkbox
												checked={draft.categories.includes(value)}
												id={`${id}-category-${value}`}
												onCheckedChange={(checked) => {
													setDraft(decision, {
														categories:
															checked === true
																? [...draft.categories, value]
																: draft.categories.filter((item) => item !== value),
													});
												}}
											/>
											{prefix}
											{category.label(value)}
										</Label>
									))}
								</div>
							</fieldset>
							<div className="flex flex-col gap-1.5">
								<Label className={FIELD_LABEL} htmlFor={`${id}-safe`}>
									{prefix}Safe to send
								</Label>
								<select
									className={FILTER_CLASS}
									id={`${id}-safe`}
									onChange={(event) => {
										setDraft(decision, { safe: event.target.value });
									}}
									value={draft.safe}
								>
									<option value="">Any</option>
									<option value="true">Safe to send</option>
									<option value="false">Not safe to send</option>
								</select>
							</div>
							<div className="flex flex-col gap-1.5">
								<Label className={FIELD_LABEL} htmlFor={`${id}-reasons`}>
									{prefix}Reason codes
								</Label>
								<Input
									className="h-10 rounded-lg font-mono text-xs"
									id={`${id}-reasons`}
									onChange={(event) => {
										setDraft(decision, { reasons: event.target.value });
									}}
									placeholder="invalid_recipient, disposable"
									value={draft.reasons}
								/>
								<p className="text-xs text-[#858585]">
									Put a comma between codes. The rule matches a result with one of them.
								</p>
							</div>
							<p className="text-xs text-neutral-300">{conditionText(fromDraft(draft))}</p>
						</fieldset>
					);
				})}
				<p className="text-xs text-[#858585]">Other addresses: {BUILT_IN_RULE}</p>
				<div className="flex justify-end">
					<Button className="rounded-lg" disabled={save.isPending} type="submit">
						{policy ? "Save policy" : "Create policy"}
					</Button>
				</div>
			</form>
		</section>
	);
}
