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

import { Group, Pill, Row } from "@/components/platform/page-kit";
import {
	OutcomePolicyForm,
	RULE_EVENTS,
	ruleText,
} from "@/components/features/deliverability/outcome-policy-form/outcome-policy-form";
import { apiMessage } from "@/components/features/deliverability/outcome-events/outcome-events";
import { workflowQueryKey } from "@/hooks/queries/utils/workflow-query-key";
import type { z } from "zod";

import {
	V1DeleteOutcomePolicy200Response,
	V1GetOutcomePolicy200Response,
	V1ListOutcomePolicies200Response,
} from "@/lib/api/generated/zod/tenant/tenant";
import { outcomeType } from "@/lib/verification/labels";

import { type OutcomePolicyListPropsFields } from "./outcome-policy-list.schema";

/**
 * @oppulence-gen kind=component
 * OutcomePolicyList is a client data component.
 * Lists the outcome policies and shows each rule in plain words. Each policy
 * can become the default, open in the editor, or be deleted.
 * Owned by `outcome-policy-list.lit.ts`.
 */
export type OutcomePolicyListProps = OutcomePolicyListPropsFields &
	ComponentPropsWithoutRef<"section">;

const api = new TenantApi(new Configuration({ basePath: "/api/backend" }));

export function OutcomePolicyList({ className, ...props }: OutcomePolicyListProps) {
	const client = useQueryClient();
	const [notice, setNotice] = useState("");
	const [editing, setEditing] = useState<z.infer<typeof V1GetOutcomePolicy200Response>>();
	const policies = useQuery({
		queryKey: workflowQueryKey("outcome-policies"),
		queryFn: async () =>
			V1ListOutcomePolicies200Response.parse((await api.v1ListOutcomePolicies({ limit: 50 })).data),
	});
	const makeDefault = useMutation({
		mutationFn: async (policyId: number) =>
			V1GetOutcomePolicy200Response.parse(
				(
					await api.v1UpdateOutcomePolicy({
						policyId,
						updateOutcomePolicyRequest: { is_default: true },
					})
				).data,
			),
		onSuccess: async (data) => {
			setNotice(`${data.name} is now the default policy.`);
			await client.invalidateQueries({ queryKey: workflowQueryKey("outcome-policies") });
		},
	});
	const remove = useMutation({
		mutationFn: async (policyId: number) =>
			V1DeleteOutcomePolicy200Response.parse((await api.v1DeleteOutcomePolicy({ policyId })).data),
		onSuccess: async () => {
			setNotice("Outcome policy deleted.");
			await client.invalidateQueries({ queryKey: workflowQueryKey("outcome-policies") });
		},
	});
	const error = [policies, makeDefault, remove].find((request) => request.isError)?.error;

	return (
		<section
			data-slot="outcome-policy-list"
			className={cn("flex flex-col gap-4", className)}
			{...props}
		>
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
				policies.data.policies.map((policy) => (
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
								{policy.is_default ? null : (
									<Button
										aria-label={`Make ${policy.name} the default`}
										disabled={makeDefault.isPending}
										onClick={() => {
											makeDefault.mutate(policy.id);
										}}
										size="sm"
										type="button"
										variant="secondary"
									>
										Make default
									</Button>
								)}
								<Button
									aria-label={`Edit ${policy.name}`}
									onClick={() => {
										setEditing(policy);
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
										if (window.confirm(`Delete the outcome policy ${policy.name}?`))
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
							{RULE_EVENTS.map((event) => (
								<li className="flex flex-col gap-0.5 sm:flex-row sm:gap-2" key={event}>
									<span className="shrink-0 text-white sm:w-32">{outcomeType.label(event)}</span>
									<span className="text-[#858585]">{ruleText(policy.rules[event])}</span>
								</li>
							))}
							<li className="flex flex-col gap-0.5 sm:flex-row sm:gap-2">
								<span className="shrink-0 text-white sm:w-32">Event age</span>
								<span className="text-[#858585]">
									Later checks use the events of the last {policy.rules.outcome_ttl_days ?? 90}{" "}
									days.
								</span>
							</li>
						</ul>
					</Group>
				))
			) : (
				<Group>
					<Row>
						<p className="text-sm text-[#858585]">
							{policies.isPending
								? "Loading policies…"
								: "No outcome policies yet. The first campaign upload creates the default policy."}
						</p>
					</Row>
				</Group>
			)}
			<Sheet
				onOpenChange={(next) => {
					if (!next) setEditing(undefined);
				}}
				open={!!editing}
			>
				<SheetContent className="flex w-full flex-col gap-6 overflow-y-auto pb-8 sm:max-w-xl">
					<SheetHeader>
						<SheetTitle>Edit {editing?.name}</SheetTitle>
						<SheetDescription>Change the rules for each campaign event.</SheetDescription>
					</SheetHeader>
					{editing ? (
						<OutcomePolicyForm
							className="px-4"
							onSaved={() => {
								setEditing(undefined);
								setNotice("Outcome policy saved.");
							}}
							policy={editing}
						/>
					) : null}
				</SheetContent>
			</Sheet>
		</section>
	);
}
