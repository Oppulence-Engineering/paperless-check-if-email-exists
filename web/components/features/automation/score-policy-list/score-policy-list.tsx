"use client";

import "client-only";

import { useState, type ComponentPropsWithoutRef } from "react";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { Configuration, TenantApi } from "@oppulence/reacher-sdk";

import { Button } from "@oppulence/ui/components/button";
import {
	Sheet,
	SheetContent,
	SheetDescription,
	SheetHeader,
	SheetTitle,
} from "@oppulence/ui/components/sheet";
import { cn } from "@oppulence/ui/lib/utils";

import {
	BUILT_IN_RULE,
	conditionText,
	DECISIONS,
	parseRules,
	ScorePolicyForm,
} from "@/components/features/automation/score-policy-form/score-policy-form";
import { apiMessage } from "@/components/features/deliverability/outcome-events/outcome-events";
import { Group, Pill, Row } from "@/components/platform/page-kit";
import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";
import {
	V1GetScorePolicy200Response,
	V1ListScorePolicies200Response,
} from "@/lib/api/generated/zod/tenant/tenant";
import { V1DeleteScorePolicy200Response } from "@/lib/api/generated/zod/tenant/tenant";
import { policyDecision } from "@/lib/verification/labels";

import { type ScorePolicyListPropsFields } from "./score-policy-list.schema";

/**
 * @oppulence-gen kind=component
 * ScorePolicyList is a client data component.
 * Lists the score policies and shows each rule in plain words. A sheet
 * creates a policy or edits one. Each policy can be deleted.
 * Owned by `score-policy-list.lit.ts`.
 */
export type ScorePolicyListProps = ScorePolicyListPropsFields & ComponentPropsWithoutRef<"section">;

const api = new TenantApi(new Configuration({ basePath: "/api/backend" }));

export function ScorePolicyList({ className, ...props }: ScorePolicyListProps) {
	const client = useQueryClient();
	const [notice, setNotice] = useState("");
	// "new" opens the create form. A number opens the edit form for that policy.
	const [open, setOpen] = useState<number | "new">();
	const policies = useQuery({
		queryKey: workflowQueryKey("score-policies"),
		queryFn: async ({ signal }) =>
			V1ListScorePolicies200Response.parse(
				(await api.v1ListScorePolicies({ limit: 200 }, { signal })).data,
			),
	});
	const editing = useQuery({
		queryKey: workflowQueryKey("score-policies", typeof open === "number" ? open : null),
		enabled: typeof open === "number",
		queryFn: async ({ signal }) =>
			V1GetScorePolicy200Response.parse(
				(await api.v1GetScorePolicy({ policyId: Number(open) }, { signal })).data,
			),
	});
	const remove = useMutation({
		mutationFn: async (policyId: number) =>
			V1DeleteScorePolicy200Response.parse((await api.v1DeleteScorePolicy({ policyId })).data),
		onSuccess: async () => {
			setNotice("Score policy deleted.");
			await client.invalidateQueries({ queryKey: workflowQueryKey("score-policies") });
		},
	});
	const error = [policies, remove].find((request) => request.isError)?.error;

	function close(message: string) {
		setOpen(undefined);
		setNotice(message);
	}

	return (
		<section
			data-slot="score-policy-list"
			className={cn("flex flex-col gap-4", className)}
			{...props}
		>
			<div className="flex justify-end">
				<Button
					className="rounded-lg"
					onClick={() => {
						setOpen("new");
					}}
					size="sm"
					type="button"
					variant="outline"
				>
					New score policy
				</Button>
			</div>
			{error ? (
				<p
					className="rounded-lg bg-red-500/10 px-4 py-3 text-sm text-red-300 ring-1 ring-red-500/20"
					role="alert"
				>
					{apiMessage(error)}
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
			{policies.data?.policies.length ? (
				policies.data.policies.map((policy) => {
					const rules = parseRules(policy.rules);
					return (
						<Group key={policy.id}>
							<Row
								description={`Updated ${new Date(policy.updated_at).toLocaleString()}`}
								label={
									<span className="flex flex-wrap items-center gap-2">
										{policy.name}
										{policy.is_default ? <Pill tone="green">Default</Pill> : null}
									</span>
								}
							>
								<div className="flex flex-wrap gap-2 md:justify-end">
									<Button
										aria-label={`Edit ${policy.name}`}
										onClick={() => {
											setOpen(policy.id);
										}}
										size="sm"
										type="button"
										variant="secondary"
									>
										Edit
									</Button>
									<Button
										aria-label={`Delete ${policy.name}`}
										className="text-red-400 hover:text-red-300"
										disabled={remove.isPending}
										onClick={() => {
											if (window.confirm(`Delete the score policy ${policy.name}?`))
												remove.mutate(policy.id);
										}}
										size="sm"
										type="button"
										variant="ghost"
									>
										Delete
									</Button>
								</div>
							</Row>
							<ul className="flex flex-col gap-2 p-5 text-sm">
								{rules ? (
									DECISIONS.map((decision) => (
										<li className="flex flex-col gap-0.5 sm:flex-row sm:gap-2" key={decision}>
											<span className="shrink-0 text-white sm:w-32">
												{policyDecision.label(decision)}
											</span>
											<span className="text-[#858585]">{conditionText(rules[decision])}</span>
										</li>
									))
								) : (
									<li className="text-[#858585]">
										This page cannot show the rules of this policy.
									</li>
								)}
								<li className="flex flex-col gap-0.5 sm:flex-row sm:gap-2">
									<span className="shrink-0 text-white sm:w-32">Other addresses</span>
									<span className="text-[#858585]">{BUILT_IN_RULE}</span>
								</li>
							</ul>
						</Group>
					);
				})
			) : (
				<Group>
					<Row>
						<p className="text-sm text-[#858585]">
							{policies.isPending
								? "Loading policies…"
								: `No score policies yet. Without a policy, the built-in rule decides. ${BUILT_IN_RULE}`}
						</p>
					</Row>
				</Group>
			)}
			<Sheet
				onOpenChange={(next) => {
					if (!next) setOpen(undefined);
				}}
				open={open !== undefined}
			>
				<SheetContent className="flex w-full flex-col gap-6 overflow-y-auto pb-8 sm:max-w-xl">
					<SheetHeader>
						<SheetTitle>
							{open === "new" ? "New score policy" : `Edit ${editing.data?.name ?? "score policy"}`}
						</SheetTitle>
						<SheetDescription>
							Set a condition for each decision. The built-in rule decides for addresses that no
							condition matches.
						</SheetDescription>
					</SheetHeader>
					{open === "new" ? (
						<ScorePolicyForm
							className="px-4"
							onSaved={() => {
								close("Score policy created.");
							}}
						/>
					) : editing.data ? (
						<ScorePolicyForm
							className="px-4"
							key={editing.data.updated_at}
							onSaved={() => {
								close("Score policy saved.");
							}}
							policy={editing.data}
						/>
					) : (
						<p className="px-4 text-sm text-[#858585]" role={editing.isError ? "alert" : "status"}>
							{editing.isError ? apiMessage(editing.error) : "Loading the policy…"}
						</p>
					)}
				</SheetContent>
			</Sheet>
		</section>
	);
}
